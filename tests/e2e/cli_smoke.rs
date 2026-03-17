use assert_cmd::Command;
use predicates::prelude::*;

use crate::common::config_env;

#[test]
fn top_level_help_lists_key_commands() {
    Command::cargo_bin("analyzer")
        .expect("binary")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("login"))
        .stdout(predicate::str::contains("scan"))
        .stdout(predicate::str::contains("object"))
        .stdout(predicate::str::contains("config"));
}

#[test]
fn completions_bash_mentions_binary_name() {
    Command::cargo_bin("analyzer")
        .expect("binary")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_analyzer"));
}

#[test]
fn whoami_masks_api_key_from_environment() {
    let temp = tempfile::tempdir().expect("tempdir");
    let envs = config_env(temp.path());

    let mut cmd = Command::cargo_bin("analyzer").expect("binary");
    for (key, value) in &envs {
        cmd.env(key, value);
    }
    cmd.env("ANALYZER_API_KEY", "super-secret-key")
        .env("ANALYZER_URL", "https://whoami.example/api/")
        .args(["whoami"])
        .assert()
        .success()
        .stderr(predicate::str::contains("supe...-key"))
        .stderr(predicate::str::contains("https://whoami.example/api/"));
}

#[test]
fn whoami_supports_localized_human_output() {
    let temp = tempfile::tempdir().expect("tempdir");
    let envs = config_env(temp.path());

    let mut cmd = Command::cargo_bin("analyzer").expect("binary");
    for (key, value) in &envs {
        cmd.env(key, value);
    }

    cmd.env("ANALYZER_API_KEY", "localized-key")
        .args(["--lang", "fr", "whoami"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Profil:"))
        .stderr(predicate::str::contains("Cle API:"));
}
