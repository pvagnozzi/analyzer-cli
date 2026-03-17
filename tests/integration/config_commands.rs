use analyzer_cli::commands::config::{run_get, run_set};
use analyzer_cli::config::ConfigFile;

use crate::common::apply_config_env;

#[test]
fn config_set_and_get_round_trip() {
    let temp = tempfile::tempdir().expect("tempdir");
    let _env = apply_config_env(temp.path());

    run_set(
        "url",
        "https://integration.example/api/",
        Some("integration"),
    )
    .expect("set config url");

    let config = ConfigFile::load().expect("load config");
    let profile = config.profile(Some("integration"));
    assert_eq!(
        profile.url.as_deref(),
        Some("https://integration.example/api/")
    );

    run_get("url", Some("integration")).expect("get config url");
}

#[test]
fn config_set_rejects_invalid_url() {
    let temp = tempfile::tempdir().expect("tempdir");
    let _env = apply_config_env(temp.path());

    let error =
        run_set("url", "not-a-url", Some("integration")).expect_err("invalid url should fail");
    assert!(format!("{error:#}").contains("invalid URL: not-a-url"));
}
