mod app;
mod error;
mod exec;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use app::App;
pub use error::Error;
