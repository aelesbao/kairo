use kairo_core::{
    Error,
    exec::{ExecParseError, ExecParser},
};

mod utils;

#[test]
fn test_parse_with_uris() {
    let locales: [String; 0] = [];
    let de = utils::black_hole_de(Some(&locales));

    let url: url::Url = "http://example.com".parse().unwrap();

    let (cmd, args) = ExecParser::new(&de, &locales)
        .parse_with_uris(&[url.as_str()])
        .unwrap();

    assert_eq!(cmd, "echo");
    assert_eq!(
        args,
        vec!["--arg", "Value with spaces", "http://example.com/"]
    );
}

#[test]
fn test_parse_with_uris_missing_exec() {
    let locales: [String; 0] = [];
    let de = utils::missing_exec_de(Some(&locales));

    let err = ExecParser::new(&de, &locales)
        .parse_with_uris(&[])
        .unwrap_err();

    assert!(
        matches!(
            err,
            Error::ParseExecArgs(ExecParseError::ExecFieldNotFound { .. })
        ),
        "{:?}",
        err
    );
}

#[test]
fn test_parse_with_uris_invalid_exec_format() {
    let locales: [String; 0] = [];
    let de = utils::invalid_exec_format_de(Some(&locales));

    let err = ExecParser::new(&de, &locales)
        .parse_with_uris(&[])
        .unwrap_err();

    assert!(
        matches!(
            err,
            Error::ParseExecArgs(ExecParseError::InvalidFormat { .. })
        ),
        "{:?}",
        err
    );
}

#[test]
fn test_parse_with_uris_invalid_exec_args() {
    let locales: [String; 0] = [];
    let de = utils::invalid_exec_args_de(Some(&locales));

    let url: url::Url = "http://example.com".parse().unwrap();

    let err = ExecParser::new(&de, &locales)
        .parse_with_uris(&[url.as_str()])
        .unwrap_err();

    assert!(
        matches!(
            err,
            Error::ParseExecArgs(ExecParseError::InvalidExecArgs { .. })
        ),
        "{:?}",
        err
    );
}
