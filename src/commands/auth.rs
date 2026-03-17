//! Authentication commands: login, whoami.

use anyhow::Result;
use console::style;

use crate::client::AnalyzerClient;
use crate::config::ConfigFile;
use crate::i18n::{self, Text};
use crate::output;

/// Run the `login` command — prompt for API key, validate, save.
pub async fn run_login(url: Option<&str>, profile_name: Option<&str>) -> Result<()> {
    let profile_name = profile_name.unwrap_or("default");

    eprintln!();
    output::hero(
        i18n::analyzer_cli(),
        &i18n::configuring_profile(profile_name),
    );

    let api_key = prompt_api_key()?;

    // Resolve URL from arg, existing config, or default
    let mut config = ConfigFile::load().unwrap_or_default();
    let existing_url = config.profile(Some(profile_name)).url.clone();
    let url = url
        .map(String::from)
        .or(existing_url)
        .unwrap_or_else(|| "https://analyzer.exein.io/api/".to_string());

    // Validate
    eprintln!();
    output::status("", i18n::validating_api_key());
    let parsed_url: url::Url = url.parse()?;
    let client = AnalyzerClient::new(parsed_url, &api_key)?;

    match client.health().await {
        Ok(_) => output::success(i18n::key_accepted()),
        Err(e) => output::warning(&i18n::could_not_validate(e)),
    }

    // Save
    let profile = config.profile_mut(profile_name);
    profile.api_key = Some(api_key);
    profile.url = Some(url);
    config.save()?;

    let path = ConfigFile::path()?;
    output::success(&i18n::config_saved(path.display()));
    eprintln!();
    eprintln!("  {} {}", style("🎯").green().bold(), i18n::ready_to_hunt());
    output::command_hint("object list", i18n::list_your_objects());
    output::command_hint("scan types", i18n::available_scan_types());
    output::command_hint("scan new -h", i18n::start_a_scan());
    eprintln!();

    Ok(())
}

/// Show current identity / configuration.
pub fn run_whoami(api_key: Option<&str>, url: Option<&str>, profile: Option<&str>) -> Result<()> {
    let config = ConfigFile::load().unwrap_or_default();
    let profile_name = profile
        .map(String::from)
        .or_else(|| std::env::var("ANALYZER_PROFILE").ok())
        .unwrap_or_else(|| config.default_profile.clone());

    let p = config.profile(Some(&profile_name));

    let resolved_url = url
        .map(String::from)
        .or_else(|| std::env::var("ANALYZER_URL").ok())
        .or_else(|| p.url.clone())
        .unwrap_or_else(|| "https://analyzer.exein.io/api/".to_string());

    let resolved_key = api_key
        .map(String::from)
        .or_else(|| std::env::var("ANALYZER_API_KEY").ok())
        .or_else(|| p.api_key.clone());

    let masked_key = match &resolved_key {
        Some(key) if key.len() > 8 => format!("{}...{}", &key[..4], &key[key.len() - 4..]),
        Some(key) => format!("{}...", &key[..key.len().min(4)]),
        None => i18n::not_set_value().to_string(),
    };

    output::hero(i18n::analyzer_cli(), i18n::tagline());
    output::key_value(i18n::text(Text::Profile), profile_name);
    output::key_value(i18n::text(Text::Url), resolved_url);
    output::key_value(i18n::text(Text::ApiKey), masked_key);

    if let Ok(path) = ConfigFile::path() {
        output::key_value(i18n::text(Text::Config), path.display());
    }

    Ok(())
}

fn prompt_api_key() -> Result<String> {
    eprint!("  🔑 {} ", i18n::enter_api_key());
    let mut key = String::new();
    std::io::stdin().read_line(&mut key)?;
    let key = key.trim().to_string();
    if key.is_empty() {
        anyhow::bail!(i18n::api_key_cannot_be_empty());
    }
    Ok(key)
}
