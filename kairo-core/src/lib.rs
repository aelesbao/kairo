mod error;
pub mod exec;
mod handler;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use error::Error;
pub use handler::UrlHandlerApp;
pub use url::Url;
