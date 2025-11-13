use std::{
    path::{Path, PathBuf},
    process::Command,
};

use freedesktop_desktop_entry as fde;
use mime::Mime;
use url::Url;

use crate::{Error, Result};

/// Represents an application that can handle specific URL schemes.
#[derive(Clone, Debug)]
pub struct App {
    pub appid: Box<str>,
    pub name: Box<str>,
    pub comment: Option<Box<str>>,
    pub icon: Option<Box<str>>,
    pub path: Box<Path>,
}

impl App {
    /// Opens the given URL with this application.
    pub fn open_url(&self, url: Url) -> Result<u32> {
        let entry = fde::DesktopEntry::from_path::<&str>(self.path.clone(), None)?;
        let locales = fde::get_languages_from_env();
        let exec_args = entry.parse_exec_with_uris(&[url.as_str()], &locales)?;
        let [cmd, args @ ..] = exec_args.as_slice() else {
            return Err(Error::ParseExecArgsFailed {
                path: self.path.clone(),
            });
        };

        let program = Command::new(cmd).args(args).spawn()?;

        Ok(program.id())
    }

    /// Retrieves all applications that can handle the specified URL scheme.
    ///
    /// # Arguments
    ///
    /// * `scheme` - The URL scheme to query (e.g., "http", "mailto").
    /// * `locales` - Optional list of locales for localization. If `None`, it fetches the system's default locales.
    /// * `search_paths` - Optional list of paths to search for desktop entries. If `None`, it uses the default XDG paths.
    pub fn handlers_for_scheme(
        scheme: &str,
        locales: Option<Vec<String>>,
        search_paths: Option<Vec<PathBuf>>,
    ) -> Result<Vec<Self>> {
        let locales = locales.unwrap_or_else(fde::get_languages_from_env);
        let search_paths = search_paths.unwrap_or_else(|| fde::default_paths().collect());
        let entries = fde::Iter::new(search_paths.into_iter()).entries(Some(&locales));

        let scheme_handler_mime = format!("x-scheme-handler/{}", scheme)
            .as_str()
            .parse::<Mime>()?;

        let apps = entries
            .filter(|de| {
                de.mime_type()
                    .is_some_and(|mime| mime.contains(&scheme_handler_mime.essence_str()))
            })
            .map(|entry| Self::from_desktop_entry(entry, &locales))
            .collect();

        Ok(apps)
    }

    /// Creates an [App] instance from a [freedesktop_desktop_entry::DesktopEntry].
    ///
    /// # Arguments
    ///
    /// * `de` - The desktop entry to convert.
    /// * `locales` - Used for localizing the app's name and comment.
    pub fn from_desktop_entry<L: AsRef<str>>(de: fde::DesktopEntry, locales: &[L]) -> Self {
        let appid: Box<str> = de.appid.clone().into();
        let name = de
            .name(locales)
            .map(|name| name.into())
            .unwrap_or_else(|| appid.clone());

        Self {
            appid,
            name,
            comment: de.comment(locales).map(|comment| comment.into()),
            icon: de.icon().map(|icon| icon.into()),
            path: de.path.into(),
        }
    }
}
