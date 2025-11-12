use std::{path::Path, process::Command};

use freedesktop_desktop_entry as fde;
use mime::Mime;
use url::Url;

pub mod error;

pub type KiroResult<T> = Result<T, error::KiroError>;

#[derive(Clone, Debug)]
pub struct App {
    pub appid: Box<str>,
    pub name: Box<str>,
    pub icon: Option<Box<str>>,
    pub path: Box<Path>,
}

impl App {
    pub fn open_url(&self, url: Url) -> KiroResult<u32> {
        let entry = fde::DesktopEntry::from_path::<&str>(self.path.clone(), None)?;
        let locales = fde::get_languages_from_env();
        let exec_args = entry.parse_exec_with_uris(&[url.as_str()], &locales)?;
        let [cmd, args @ ..] = exec_args.as_slice() else {
            return Err(error::KiroError::ParseExecArgsFailed {
                path: self.path.clone(),
            });
        };

        let program = Command::new(cmd).args(args).spawn()?;

        Ok(program.id())
    }

    pub fn handlers_for_scheme(scheme: &str) -> KiroResult<Vec<Self>> {
        let scheme_handler_mime = format!("x-scheme-handler/{}", scheme)
            .as_str()
            .parse::<Mime>()?;

        // TODO: find workaround for testing
        let locales = fde::get_languages_from_env();
        let entries = fde::Iter::new(fde::default_paths()).entries(Some(&locales));

        let apps = entries
            .filter(|entry| {
                entry
                    .mime_type()
                    .is_some_and(|mime| mime.contains(&scheme_handler_mime.essence_str()))
            })
            .map(|entry| Self::from_desktop_entry(entry, &locales))
            .collect();

        Ok(apps)
    }

    fn from_desktop_entry<L: AsRef<str>>(entry: fde::DesktopEntry, locales: &[L]) -> Self {
        let appid: Box<str> = entry.appid.clone().into();
        let name = entry
            .name(locales)
            .map(|name| name.into())
            .unwrap_or_else(|| appid.clone());

        Self {
            appid,
            name,
            icon: entry.icon().map(|icon| icon.into()),
            path: entry.path.into(),
        }
    }
}
