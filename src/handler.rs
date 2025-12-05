use std::{path::PathBuf, process::Command};

use freedesktop_desktop_entry as fde;
use mime::Mime;
use url::Url;

use crate::{Result, exec::ExecParser};

/// Represents an application that can handle specific URL schemes.
#[derive(Clone, Debug)]
pub struct UrlHandlerApp {
    pub appid: String,
    pub name: String,
    pub comment: Option<String>,
    pub icon: fde::IconSource,
    pub path: PathBuf,
}

impl UrlHandlerApp {
    /// Opens the given URL with this application.
    pub fn open_url(&self, url: Url) -> Result<u32> {
        log::info!(
            "Opening URL '{}' with application '{}'",
            url,
            self.path.display()
        );

        let locales = fde::get_languages_from_env();
        let de = fde::DesktopEntry::from_path(self.path.clone(), Some(&locales))?;

        let (cmd, args) = ExecParser::new(&de, &locales).parse_with_uris(&[url.as_str()])?;
        log::debug!("Executing command: '{}' with args: {:?}", cmd, args);

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

        log::debug!(
            "Searching for applications handling scheme '{}' in paths: {:?}",
            scheme,
            search_paths
        );

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
            .collect::<Vec<_>>();

        log::info!(
            "Found {} applications with support for '{}'",
            apps.len(),
            scheme_handler_mime
        );

        Ok(apps)
    }

    /// Creates an [App] instance from a [freedesktop_desktop_entry::DesktopEntry].
    ///
    /// # Arguments
    ///
    /// * `de` - The desktop entry to convert.
    /// * `locales` - Used for localizing the app's name and comment.
    pub fn from_desktop_entry<L: AsRef<str>>(de: fde::DesktopEntry, locales: &[L]) -> Self {
        let appid = de.appid.clone();
        let name = de
            .name(locales)
            .map(|name| name.into())
            .unwrap_or_else(|| appid.clone());

        Self {
            appid,
            name,
            comment: de.comment(locales).map(|comment| comment.into()),
            icon: de
                .icon()
                .map(fde::IconSource::from_unknown)
                .unwrap_or_default(),
            path: de.path,
        }
    }

    pub fn icon_path(&self, icon_size: u16) -> Option<std::path::PathBuf> {
        log::debug!(
            "Fetching icon for appid={} icon={:?}",
            self.appid,
            self.icon
        );

        match &self.icon {
            fde::IconSource::Path(path) => Some(path.to_owned()),
            fde::IconSource::Name(name) => freedesktop_icons::lookup(name)
                .with_size(icon_size)
                .with_cache()
                .find(),
        }
    }
}
