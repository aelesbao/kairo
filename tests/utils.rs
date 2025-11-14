use std::path::PathBuf;

use freedesktop_desktop_entry as fde;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn entries_path() -> PathBuf {
    PathBuf::from(format!("{}/tests/entries", CARGO_MANIFEST_DIR))
}

pub fn black_hole_de<L>(locales: Option<&[L]>) -> fde::DesktopEntry
where
    L: AsRef<str>,
{
    let path = entries_path().join("black-hole.desktop");
    fde::DesktopEntry::from_path(path, locales).unwrap()
}

#[allow(dead_code)]
pub fn missing_exec_de<L>(locales: Option<&[L]>) -> fde::DesktopEntry
where
    L: AsRef<str>,
{
    let path = entries_path().join("invalid.desktop");
    let entry = r###"
[Desktop Entry]
Version=1.0
Type=Application
Terminal=false
MimeType=x-scheme-handler/http;x-scheme-handler/https;
Name=Black Hole Browser
"###;

    fde::DesktopEntry::from_str(path, entry, locales).unwrap()
}

#[allow(dead_code)]
pub fn invalid_exec_format_de<L>(locales: Option<&[L]>) -> fde::DesktopEntry
where
    L: AsRef<str>,
{
    let path = entries_path().join("invalid.desktop");
    let entry = r###"
[Desktop Entry]
Version=1.0
Type=Application
Terminal=false
MimeType=x-scheme-handler/http;x-scheme-handler/https;
Name=Black Hole Browser
Exec="unmatched quotes
"###;

    fde::DesktopEntry::from_str(path, entry, locales).unwrap()
}

#[allow(dead_code)]
pub fn invalid_exec_args_de<L>(locales: Option<&[L]>) -> fde::DesktopEntry
where
    L: AsRef<str>,
{
    let path = entries_path().join("invalid.desktop");
    let entry = r###"
[Desktop Entry]
Version=1.0
Type=Application
Terminal=false
MimeType=x-scheme-handler/http;x-scheme-handler/https;
Name=Black Hole Browser
Exec=
"###;

    fde::DesktopEntry::from_str(path, entry, locales).unwrap()
}
