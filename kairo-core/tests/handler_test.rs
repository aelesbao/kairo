mod utils;

use freedesktop_desktop_entry as fde;
use kairo_core::UrlHandlerApp;

#[test]
fn test_handlers_for_scheme() {
    let search_paths = vec![utils::entries_path()];

    let apps =
        UrlHandlerApp::handlers_for_scheme("http", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 3);

    let apps =
        UrlHandlerApp::handlers_for_scheme("ipfs", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 1);

    let err =
        UrlHandlerApp::handlers_for_scheme("file", None, Some(search_paths.clone())).unwrap_err();
    assert!(matches!(err, kairo_core::Error::NoHandlersFound(_)));
}

#[test]
fn test_from_desktop_entry() {
    let entries_path = utils::entries_path();

    let locales: [String; 0] = [];
    let firefox_de =
        fde::DesktopEntry::from_path(entries_path.join("firefox.desktop"), Some(&locales)).unwrap();
    let firefox_app = UrlHandlerApp::from_desktop_entry(firefox_de.clone(), &locales);

    assert_eq!(firefox_app.appid.as_str(), "firefox");
    assert_eq!(firefox_app.name.as_str(), "Firefox");
    assert_eq!(
        firefox_app.comment.unwrap().as_str(),
        "Browse the World Wide Web"
    );
    assert!(matches!(
        firefox_app.icon,
        fde::IconSource::Name(name) if name == firefox_app.appid
    ));
    assert_eq!(firefox_app.path.to_path_buf(), firefox_de.path);

    // Loads localized name and comment
    let locales = ["de"];
    let black_hole_de =
        fde::DesktopEntry::from_path(entries_path.join("black-hole.desktop"), Some(&locales))
            .unwrap();
    let black_hole_app = UrlHandlerApp::from_desktop_entry(black_hole_de, &locales);

    assert_eq!(black_hole_app.name.as_str(), "Schwarzes Loch-Browser");
    assert_eq!(
        black_hole_app.comment.unwrap().as_str(),
        "Ein minimalistischer Browser"
    );
    assert_eq!(black_hole_app.icon, fde::IconSource::default());
}

#[test]
fn test_open_url() {
    let locales: [String; 0] = [];
    let de = utils::black_hole_de(Some(&locales));
    let app = UrlHandlerApp::from_desktop_entry(de, &locales);

    let url = "http://github.com".parse().unwrap();
    let result = app.open_url(url);

    assert!(result.is_ok());
    assert_ne!(result.unwrap(), 0);
}
