use freedesktop_desktop_entry as fde;

use crate::Result;

#[derive(thiserror::Error, Debug)]
pub enum ExecParseError {
    #[error("invalid format in {path}: {reason}")]
    InvalidFormat {
        reason: String,
        path: Box<std::path::Path>,
    },

    #[error("invalid Exec arguments in {path}")]
    InvalidExecArgs { path: Box<std::path::Path> },

    #[error("Exec key was not found in {path}")]
    ExecFieldNotFound { path: Box<std::path::Path> },
}

pub struct ExecParser<'a, L>
where
    L: AsRef<str>,
{
    de: &'a fde::DesktopEntry,
    locales: &'a [L],
}

impl<'a, L> ExecParser<'a, L>
where
    L: AsRef<str>,
{
    pub fn new(de: &'a fde::DesktopEntry, locales: &'a [L]) -> ExecParser<'a, L>
    where
        L: AsRef<str>,
    {
        ExecParser { de, locales }
    }

    pub fn parse_with_uris(&self, uris: &[&str]) -> Result<(String, Vec<String>)> {
        let exec = self.de.exec().ok_or(ExecParseError::ExecFieldNotFound {
            path: self.de.path.clone().into(),
        })?;

        let exec = if let Some(without_prefix) = exec.strip_prefix('\"') {
            without_prefix
                .strip_suffix('\"')
                .ok_or(ExecParseError::InvalidFormat {
                    reason: "unmatched quote".into(),
                    path: self.de.path.clone().into(),
                })?
        } else {
            exec
        };

        let exec_args = shell_words::split(exec)?
            .iter()
            .flat_map(|arg| self.parse_arg(arg, uris))
            .flatten()
            .collect::<Vec<_>>();

        match exec_args.as_slice() {
            [cmd, args @ ..] => Ok((cmd.to_string(), args.to_vec())),
            _ => Err(ExecParseError::InvalidExecArgs {
                path: self.de.path.clone().into(),
            })?,
        }
    }

    fn parse_arg(&self, arg: &str, uris: &[&str]) -> Option<Vec<String>> {
        match ArgOrFieldCode::try_from(arg) {
            Ok(arg) => match arg {
                ArgOrFieldCode::SingleFileName | ArgOrFieldCode::SingleUrl => {
                    uris.first().map(|uri| vec![uri.to_string()])
                }
                ArgOrFieldCode::FileList | ArgOrFieldCode::UrlList => {
                    let args = uris.iter().map(|uri| uri.to_string()).collect();
                    Some(args)
                }
                ArgOrFieldCode::IconKey => self.de.icon().map(|icon| vec![icon.to_string()]),
                ArgOrFieldCode::TranslatedName => self
                    .de
                    .name(self.locales)
                    .map(|name| vec![name.to_string()]),
                ArgOrFieldCode::DesktopFileLocation => {
                    Some(vec![self.de.path.to_string_lossy().to_string()])
                }
                ArgOrFieldCode::Arg(arg) => Some(vec![arg.to_string()]),
            },
            Err(e) => {
                log::error!("{}", e);
                None
            }
        }
    }
}

// either a command line argument or a field-code as described
// in https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html#exec-variables
enum ArgOrFieldCode<'a> {
    SingleFileName,
    FileList,
    SingleUrl,
    UrlList,
    IconKey,
    TranslatedName,
    DesktopFileLocation,
    Arg(&'a str),
}

#[derive(Debug, thiserror::Error)]
enum ExecErrorInternal<'a> {
    #[error("Unknown field code: '{0}'")]
    UnknownFieldCode(&'a str),

    #[error("Deprecated field code: '{0}'")]
    DeprecatedFieldCode(&'a str),
}

impl<'a> TryFrom<&'a str> for ArgOrFieldCode<'a> {
    type Error = ExecErrorInternal<'a>;

    // todo: handle escaping
    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        match value {
            "%f" => Ok(ArgOrFieldCode::SingleFileName),
            "%F" => Ok(ArgOrFieldCode::FileList),
            "%u" => Ok(ArgOrFieldCode::SingleUrl),
            "%U" => Ok(ArgOrFieldCode::UrlList),
            "%i" => Ok(ArgOrFieldCode::IconKey),
            "%c" => Ok(ArgOrFieldCode::TranslatedName),
            "%k" => Ok(ArgOrFieldCode::DesktopFileLocation),
            "%d" | "%D" | "%n" | "%N" | "%v" | "%m" => {
                Err(ExecErrorInternal::DeprecatedFieldCode(value))
            }
            other if other.starts_with('%') => Err(ExecErrorInternal::UnknownFieldCode(other)),
            other => Ok(ArgOrFieldCode::Arg(other)),
        }
    }
}
