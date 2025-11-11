use std::sync::Arc;

use cosmic_mime_apps::App;
use mime::Mime;

pub mod error;

pub type KiroResult<T> = Result<T, error::KiroError>;

pub fn handlers_for_scheme(scheme: &str) -> KiroResult<Vec<Arc<App>>> {
    let mime = format!("x-scheme-handler/{}", scheme)
        .as_str()
        .parse::<Mime>()?;

    // TODO: find workaround for testing
    let assocs = cosmic_mime_apps::associations::by_app();
    let apps = assocs
        .into_values()
        .filter(move |app| app.mime_types.contains(&mime))
        .collect();

    Ok(apps)
}
