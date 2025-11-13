use kiro::App;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn test_handlers_for_scheme() {
    let search_paths = vec![std::path::PathBuf::from(format!(
        "{}/tests/entries",
        CARGO_MANIFEST_DIR
    ))];

    let apps = App::handlers_for_scheme("http", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 3);

    let apps = App::handlers_for_scheme("ipfs", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 1);

    let apps = App::handlers_for_scheme("file", None, Some(search_paths.clone())).unwrap();
    assert_eq!(apps.len(), 0);
}
