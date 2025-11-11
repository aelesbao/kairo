use std::{fmt::Display, path::Path, process::Command, sync::Arc};

use freedesktop_desktop_entry::DesktopEntry;
use mime::Mime;

pub mod error;

pub type KiroResult<T> = Result<T, error::KiroError>;

#[derive(Clone, Debug)]
pub struct App {
    pub appid: Box<str>,
    pub name: Box<str>,
    pub icon: Box<str>,
    pub path: Box<Path>,
}

impl App {
    pub fn into_desktop_entry(&self) -> KiroResult<DesktopEntry> {
        let entry = DesktopEntry::from_path::<&str>(self.path.clone(), None)?;
        Ok(entry)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl From<Arc<cosmic_mime_apps::App>> for App {
    fn from(value: Arc<cosmic_mime_apps::App>) -> Self {
        Self {
            appid: value.appid.clone(),
            name: value.name.clone(),
            icon: value.icon.clone(),
            path: value.path.clone(),
        }
    }
}

pub fn handlers_for_scheme(scheme: &str) -> KiroResult<Vec<Arc<App>>> {
    let mime = format!("x-scheme-handler/{}", scheme)
        .as_str()
        .parse::<Mime>()?;

    // TODO: find workaround for testing
    let assocs = cosmic_mime_apps::associations::by_app();
    let apps = assocs
        .values()
        .filter(move |app| app.mime_types.contains(&mime))
        .map(|app| Arc::new(App::from(app.clone())))
        .collect();

    Ok(apps)
}

pub fn open_with_app(app: Arc<App>, url: url::Url) -> KiroResult<u32> {
    let entry = app.into_desktop_entry()?;

    let exec_args = entry.parse_exec_with_uris::<&str>(&[url.as_str()], &[])?;
    let [cmd, args @ ..] = exec_args.as_slice() else {
        return Err(error::KiroError::ParseExecArgsFailed {
            path: app.path.clone(),
        });
    };

    println!("Executing: {cmd} {}", args.join(" "));
    let program = Command::new(cmd).args(args).spawn()?;

    Ok(program.id())
}
