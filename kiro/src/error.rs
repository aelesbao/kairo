#[derive(thiserror::Error, Debug)]
pub enum KiroError {
    #[error("failed ")]
    MimeFromStrError(#[from] mime::FromStrError),
}
