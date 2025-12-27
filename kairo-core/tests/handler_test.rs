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
    let de =
        fde::DesktopEntry::from_path(entries_path.join("firefox.desktop"), Some(&locales)).unwrap();
    let app = UrlHandlerApp::from_desktop_entry(de.clone(), &locales);

    assert_eq!(app.appid.as_str(), "firefox");
    assert_eq!(app.name.as_str(), "Firefox");
    assert_eq!(app.comment.unwrap().as_str(), "Browse the World Wide Web");
    assert!(matches!(
        app.icon,
        fde::IconSource::Name(name) if name == app.appid
    ));
    assert_eq!(app.path.to_path_buf(), de.path);

    // Loads localized name and comment
    let locales = ["de"];
    let de = utils::black_hole_de(Some(&locales));
    let app = UrlHandlerApp::from_desktop_entry(de, &locales);

    assert_eq!(app.name.as_str(), "Schwarzes Loch-Browser");
    assert_eq!(
        app.comment.unwrap().as_str(),
        "Ein minimalistischer Browser"
    );
    assert_eq!(app.icon, fde::IconSource::default());
}

#[test]
fn test_open_url() {
    let locales: [String; 0] = [];
    let de = utils::black_hole_de(Some(&locales));
    let app = UrlHandlerApp::from_desktop_entry(de, &locales);

    let result = app.open_url("https://github.com".parse().unwrap());
    assert!(result.is_ok());

    let de = utils::fail_exec_de(Some(&locales));
    let app = UrlHandlerApp::from_desktop_entry(de, &locales);

    let result = app.open_url("exit://1".parse().unwrap());
    assert!(dbg!(&result).is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        kairo_core::Error::OpenUrl(appid, status) if appid == app.appid && dbg!(status.code()).is_some_and(|code| code == 127)
    ));
}
