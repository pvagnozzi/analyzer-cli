//! Configuration management commands.

use anyhow::Result;

use crate::config::ConfigFile;
use crate::i18n::{self, Text};
use crate::output;

/// Show the current configuration.
pub fn run_show() -> Result<()> {
    let config = ConfigFile::load().unwrap_or_default();

    output::hero(i18n::analyzer_cli_configuration(), i18n::tagline());
    output::key_value(i18n::text(Text::DefaultProfile), &config.default_profile);

    if let Ok(path) = ConfigFile::path() {
        output::key_value(i18n::text(Text::ConfigFile), path.display());
    }

    if config.profiles.is_empty() {
        eprintln!("\n  {}", i18n::no_profiles_configured());
    } else {
        eprintln!("\n  📚 {}", i18n::text(Text::Profiles));
        for (name, profile) in &config.profiles {
            let url = profile.url.as_deref().unwrap_or(i18n::default_value());
            let key_status = if profile.api_key.is_some() {
                i18n::value_set()
            } else {
                i18n::value_not_set()
            };
            eprintln!(
                "    {} -- URL: {}, API key: {}",
                console::style(name).cyan(),
                url,
                key_status
            );
        }
    }
    Ok(())
}

/// Set a configuration value.
pub fn run_set(key: &str, value: &str, profile: Option<&str>) -> Result<()> {
    let mut config = ConfigFile::load().unwrap_or_default();
    let profile_name = profile.unwrap_or("default");
    let p = config.profile_mut(profile_name);

    match key {
        "url" => {
            let _: url::Url = value
                .parse()
                .map_err(|_| anyhow::anyhow!("invalid URL: {value}"))?;
            p.url = Some(value.to_string());
        }
        "api-key" | "api_key" => {
            p.api_key = Some(value.to_string());
        }
        "default-profile" | "default_profile" => {
            config.default_profile = value.to_string();
        }
        other => {
            anyhow::bail!(i18n::unknown_config_key(other));
        }
    }

    config.save()?;
    output::success(&i18n::set_config_value(key, value, profile_name));
    Ok(())
}

/// Get a configuration value.
pub fn run_get(key: &str, profile: Option<&str>) -> Result<()> {
    let config = ConfigFile::load().unwrap_or_default();
    let profile_name = profile.unwrap_or(&config.default_profile);
    let p = config.profile(Some(profile_name));

    let value = match key {
        "url" => p.url.as_deref().unwrap_or(i18n::not_set_value()),
        "api-key" | "api_key" => {
            if p.api_key.is_some() {
                i18n::value_set()
            } else {
                i18n::not_set_value()
            }
        }
        "default-profile" | "default_profile" => &config.default_profile,
        other => {
            anyhow::bail!(i18n::unknown_config_key(other));
        }
    };

    println!("{value}");
    Ok(())
}
