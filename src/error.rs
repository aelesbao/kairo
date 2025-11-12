use std::path::Path;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to convert scheme to MIME type: {0}")]
    MimeFromStrError(#[from] mime::FromStrError),

    #[error("failed to open desktop entry: {0}")]
    DesktopEntryDecodeError(#[from] freedesktop_desktop_entry::DecodeError),

    #[error("failed to execute command: {0}")]
    DesktopEntryExecError(#[from] freedesktop_desktop_entry::ExecError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse Exec arguments in desktop entry: {path}")]
    ParseExecArgsFailed { path: Box<Path> },
}
