mod app;
mod error;
pub mod exec;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use app::App;
pub use error::Error;
