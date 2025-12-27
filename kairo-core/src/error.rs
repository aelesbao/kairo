#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no handlers found for URL scheme: {0}")]
    NoHandlersFound(String),

    #[error("failed to execute application {0}: {1}")]
    OpenUrl(String, std::process::ExitStatus),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to convert scheme to MIME type: {0}")]
    MimeFromStr(#[from] mime::FromStrError),

    #[error("failed to open desktop entry: {0}")]
    DesktopEntryDecode(#[from] freedesktop_desktop_entry::DecodeError),

    #[error("failed to parse Exec command: {0}")]
    ParseExecArgs(#[from] crate::exec::ExecParseError),

    #[error("failed to parse arguments: {0}")]
    ExecArgsShellParse(#[from] shell_words::ParseError),
}
