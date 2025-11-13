use std::path::PathBuf;

use freedesktop_desktop_entry as fde;
use kiro::App;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn entries_path() -> PathBuf {
    PathBuf::from(format!("{}/tests/entries", CARGO_MANIFEST_DIR))
}

#[test]
fn test_handlers_for_scheme() {
    let search_paths = vec![entries_path()];

    let apps = App::handlers_for_scheme("http", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 3);

    let apps = App::handlers_for_scheme("ipfs", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 1);

    let apps = App::handlers_for_scheme("file", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 0);
}

#[test]
fn test_from_desktop_entry() {
    let entries_path = entries_path();

    let locales: [String; 0] = [];
    let firefox_de =
        fde::DesktopEntry::from_path(entries_path.join("firefox.desktop"), Some(&locales)).unwrap();
    let firefox_app = App::from_desktop_entry(firefox_de.clone(), &locales);

    assert_eq!(firefox_app.appid.as_ref(), "firefox");
    assert_eq!(firefox_app.name.as_ref(), "Firefox");
    assert_eq!(
        firefox_app.comment.unwrap().as_ref(),
        "Browse the World Wide Web"
    );
    assert_eq!(firefox_app.icon.unwrap().as_ref(), "firefox");
    assert_eq!(firefox_app.path.to_path_buf(), firefox_de.path);

    // Loads localized name and comment
    let locales = ["de"];
    let black_hole_de =
        fde::DesktopEntry::from_path(entries_path.join("black-hole.desktop"), Some(&locales))
            .unwrap();
    let black_hole_app = App::from_desktop_entry(black_hole_de, &locales);

    assert_eq!(black_hole_app.name.as_ref(), "Schwarzes Loch-Browser");
    assert_eq!(
        black_hole_app.comment.unwrap().as_ref(),
        "Ein minimalistischer Browser"
    );
    assert!(black_hole_app.icon.is_none());
}
