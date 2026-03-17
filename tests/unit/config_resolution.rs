use analyzer_cli::config::{ConfigFile, Profile, resolve};

use crate::common::{apply_config_env, apply_full_env};

#[test]
fn resolve_prefers_cli_values_over_env_and_profile() {
    let temp = tempfile::tempdir().expect("tempdir");
    let _env = apply_full_env(
        temp.path(),
        &[
            ("ANALYZER_API_KEY", "env-key"),
            ("ANALYZER_URL", "https://env.example/api/"),
            ("ANALYZER_PROFILE", "team"),
        ],
    );

    let mut config = ConfigFile {
        default_profile: "team".to_string(),
        ..ConfigFile::default()
    };
    config.profiles.insert(
        "team".to_string(),
        Profile {
            api_key: Some("profile-key".to_string()),
            url: Some("https://profile.example/api/".to_string()),
        },
    );
    config.save().expect("save config");

    let resolved = resolve(
        Some("cli-key"),
        Some("https://cli.example/api/"),
        Some("team"),
    )
    .expect("resolve config");

    assert_eq!(resolved.api_key, "cli-key");
    assert_eq!(resolved.url.as_str(), "https://cli.example/api/");
    assert_eq!(resolved.profile, "team");
}

#[test]
fn resolve_uses_default_url_when_missing() {
    let temp = tempfile::tempdir().expect("tempdir");
    let _env = apply_full_env(temp.path(), &[("ANALYZER_API_KEY", "env-key")]);

    let resolved = resolve(None, None, None).expect("resolve config");

    assert_eq!(resolved.api_key, "env-key");
    assert_eq!(resolved.url.as_str(), "https://analyzer.exein.io/api/");
    assert_eq!(resolved.profile, "default");
}

#[test]
fn resolve_errors_when_api_key_is_missing() {
    let temp = tempfile::tempdir().expect("tempdir");
    let _env = apply_config_env(temp.path());

    let error = resolve(None, None, None).expect_err("missing api key should fail");
    let message = format!("{error:#}");

    assert!(message.contains("no API key provided"));
    assert!(message.contains("analyzer login"));
}
