//! Output formatting: human (colored), JSON, and table modes.

use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};

use console::style;
use owo_colors::OwoColorize;

use crate::i18n::{self, Text};

/// Output format selected by the user.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum Format {
    /// Colored, human-friendly output (default).
    #[default]
    Human,
    /// JSON output for scripting.
    Json,
    /// ASCII table output.
    Table,
}

const EXEIN_ASCII: [&str; 6] = [
    r"  ███████╗██╗  ██╗███████╗██╗███╗   ██╗",
    r"  ██╔════╝╚██╗██╔╝██╔════╝██║████╗  ██║",
    r"  █████╗   ╚███╔╝ █████╗  ██║██╔██╗ ██║",
    r"  ██╔══╝   ██╔██╗ ██╔══╝  ██║██║╚██╗██║",
    r"  ███████╗██╔╝ ██╗███████╗██║██║ ╚████║",
    r"  ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝",
];

pub fn print_welcome() {
    static PRINTED: AtomicBool = AtomicBool::new(false);

    if PRINTED.swap(true, Ordering::AcqRel) {
        return;
    }

    eprintln!();
    for (idx, line) in EXEIN_ASCII.iter().enumerate() {
        let colored = match idx % 3 {
            0 => line.bright_cyan().bold().to_string(),
            1 => line.bright_blue().bold().to_string(),
            _ => line.bright_magenta().bold().to_string(),
        };
        eprintln!("{colored}");
    }
    eprintln!(
        "  {} {}",
        "🛡️".bright_cyan(),
        style(i18n::tagline()).bold().white()
    );
    eprintln!(
        "  {} {}  {} v{}  {} {}",
        "✨".bright_magenta(),
        style(i18n::subtitle()).dim(),
        "🚀".bright_green(),
        env!("CARGO_PKG_VERSION"),
        "🌍".bright_yellow(),
        style(i18n::language_name()).bold()
    );
    eprintln!();
}

pub fn hero(title: &str, subtitle: &str) {
    eprintln!("  {} {}", "◆".bright_cyan(), style(title).bold().white());
    eprintln!("  {} {}", "•".bright_magenta(), style(subtitle).dim());
    eprintln!();
}

pub fn key_value(label: &str, value: impl Display) {
    eprintln!("  {:>16}  {}", style(format!("{label}:")).bold(), value);
}

pub fn command_hint(command: &str, description: &str) {
    eprintln!(
        "  {} {} {}",
        "👉".bright_cyan(),
        style("analyzer").bold(),
        style(command).cyan()
    );
    eprintln!("     {}", style(description).dim());
}

/// Print a success message to stderr.
pub fn success(msg: &str) {
    eprintln!(
        "  {} {msg}",
        style(format!("✅ {}", i18n::text(Text::Ok))).green().bold()
    );
}

/// Print a warning message to stderr.
pub fn warning(msg: &str) {
    eprintln!(
        "  {} {msg}",
        style(format!("⚠️  {}", i18n::text(Text::Warning)))
            .yellow()
            .bold()
    );
}

/// Print an error message to stderr.
pub fn error(msg: &str) {
    eprintln!(
        "  {} {msg}",
        style(format!("❌ {}", i18n::text(Text::Error)))
            .red()
            .bold()
    );
}

/// Print a labelled status line to stderr.
pub fn status(label: &str, msg: &str) {
    if label.is_empty() {
        eprintln!("  {} {msg}", style("⏳").cyan().bold());
    } else {
        eprintln!(
            "  {} {}",
            style(format!("{label:>12}")).cyan().bold(),
            style(msg).white()
        );
    }
}

/// Format a score with colour coding.
pub fn format_score(score: Option<u8>) -> String {
    match score {
        Some(s) if s >= 80 => format!("{}", s.to_string().green()),
        Some(s) if s >= 50 => format!("{}", s.to_string().yellow()),
        Some(s) => format!("{}", s.to_string().red()),
        None => style("--").dim().to_string(),
    }
}

/// Format an analysis status string with colour.
pub fn format_status(status: &str) -> String {
    let display = i18n::status_display(status);
    match status {
        "success" => style(display.as_ref()).green().to_string(),
        "pending" => style(display.as_ref()).dim().to_string(),
        "in-progress" => style(display.as_ref()).cyan().to_string(),
        "canceled" => style(display.as_ref()).yellow().to_string(),
        "error" => style(display.as_ref()).red().to_string(),
        _ => display.into_owned(),
    }
}
