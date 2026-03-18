//! Scan management commands.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Result, bail};
use console::style;
use indicatif::ProgressBar;
use uuid::Uuid;

use crate::client::AnalyzerClient;
use crate::client::models::{
    AnalysisStatus, AnalysisStatusEntry, AnalysisType, CapabilityFinding, ComplianceReport,
    ComplianceType, CryptoFinding, CveFinding, HardeningFinding, IdfSymbolFinding, IdfTaskFinding,
    KernelFinding, MalwareFinding, PasswordFinding, ResultsQuery, SbomComponent, ScanTypeRequest,
};
use crate::i18n::{self, Text};
use crate::output::{self, Format, format_score, format_status};

/// Resolve a scan ID from either an explicit --scan or an --object flag.
/// When --object is used, fetches the object and returns its last scan ID.
pub async fn resolve_scan_id(
    client: &AnalyzerClient,
    scan_id: Option<Uuid>,
    object_id: Option<Uuid>,
) -> Result<Uuid> {
    if let Some(sid) = scan_id {
        return Ok(sid);
    }
    if let Some(oid) = object_id {
        let object = client.get_object(oid).await?;
        let scan = object
            .last_scan
            .ok_or_else(|| anyhow::anyhow!("object {oid} has no scans yet"))?;
        return Ok(scan.status.id);
    }
    bail!("either --scan or --object must be provided")
}

/// Create a new scan.
#[allow(clippy::too_many_arguments)]
pub async fn run_new(
    client: &AnalyzerClient,
    object_id: Uuid,
    file: PathBuf,
    scan_type: String,
    analyses: Vec<String>,
    format: Format,
    wait: bool,
    interval: Duration,
    timeout: Duration,
) -> Result<()> {
    // If no analyses specified, fetch all available for this scan type.
    let analyses = if analyses.is_empty() {
        let types = client.get_scan_types().await?;
        let matching = types.iter().find(|t| t.image_type == scan_type);
        match matching {
            Some(t) => t.analyses.iter().map(|a| a.analysis_type.clone()).collect(),
            None => bail!(
                "unknown scan type '{scan_type}'. Run `analyzer scan types` to see available types."
            ),
        }
    } else {
        analyses
    };

    let req = ScanTypeRequest {
        scan_type: scan_type.clone(),
        analyses: analyses.clone(),
    };

    let resp = client.create_scan(object_id, &file, &req).await?;

    match format {
        Format::Json if !wait => {
            println!("{}", serde_json::json!({ "id": resp.id }));
        }
        _ if !wait => {
            output::success(&i18n::scan_created(resp.id));
            eprintln!();
            output::command_hint(
                &format!("scan status --object {object_id}"),
                &i18n::check_status_command(object_id),
            );
        }
        _ => {}
    }

    if wait {
        let status = wait_for_completion(client, resp.id, interval, timeout).await?;
        print_status(resp.id, &status, format)?;
    }

    Ok(())
}

/// Delete a scan.
pub async fn run_delete(client: &AnalyzerClient, id: Uuid) -> Result<()> {
    client.delete_scan(id).await?;
    output::success(&i18n::deleted_scan(id));
    Ok(())
}

/// Cancel a running scan.
pub async fn run_cancel(client: &AnalyzerClient, id: Uuid) -> Result<()> {
    client.cancel_scan(id).await?;
    output::success(&i18n::cancelled_scan(id));
    Ok(())
}

/// Show scan status.
pub async fn run_status(client: &AnalyzerClient, scan_id: Uuid, format: Format) -> Result<()> {
    let status = client.get_scan_status(scan_id).await?;
    print_status(scan_id, &status, format)
}

/// Download the PDF report.
pub async fn run_report(
    client: &AnalyzerClient,
    scan_id: Uuid,
    output_path: PathBuf,
    wait: bool,
    interval: Duration,
    timeout: Duration,
) -> Result<()> {
    if wait {
        wait_for_completion(client, scan_id, interval, timeout).await?;
    }
    output::status("", i18n::downloading_pdf_report());
    let bytes = client.download_report(scan_id).await?;
    tokio::fs::write(&output_path, &bytes).await?;
    output::success(&i18n::report_saved(output_path.display()));
    Ok(())
}

/// Download the SBOM.
pub async fn run_sbom(
    client: &AnalyzerClient,
    scan_id: Uuid,
    output_path: PathBuf,
    wait: bool,
    interval: Duration,
    timeout: Duration,
) -> Result<()> {
    if wait {
        wait_for_completion(client, scan_id, interval, timeout).await?;
    }
    output::status("", i18n::downloading_sbom());
    let bytes = client.download_sbom(scan_id).await?;
    tokio::fs::write(&output_path, &bytes).await?;
    output::success(&i18n::sbom_saved(output_path.display()));
    Ok(())
}

/// Download a compliance report.
pub async fn run_compliance_report(
    client: &AnalyzerClient,
    scan_id: Uuid,
    ct: ComplianceType,
    output_path: PathBuf,
    wait: bool,
    interval: Duration,
    timeout: Duration,
) -> Result<()> {
    if wait {
        wait_for_completion(client, scan_id, interval, timeout).await?;
    }
    output::status("", &i18n::downloading_compliance_report(ct.display_name()));
    let bytes = client.download_compliance_report(scan_id, ct).await?;
    tokio::fs::write(&output_path, &bytes).await?;
    output::success(&i18n::compliance_report_saved(
        ct.display_name(),
        output_path.display(),
    ));
    Ok(())
}

/// Show the security score for a scan.
pub async fn run_score(client: &AnalyzerClient, scan_id: Uuid, format: Format) -> Result<()> {
    let score = client.get_scan_score(scan_id).await?;

    match format {
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::to_value(&score)?)?
            );
        }
        Format::Human | Format::Table => {
            eprintln!(
                "\n  {} {}",
                style(format!("{}:", i18n::text(Text::OverallScore))).bold(),
                format_score(score.score)
            );
            if !score.scores.is_empty() {
                eprintln!();
                eprintln!(
                    "  {:<20}  {}",
                    style(i18n::text(Text::Analysis)).underlined(),
                    style(i18n::text(Text::Score)).underlined(),
                );
                for s in &score.scores {
                    eprintln!("  {:<20}  {}", s.analysis_type, format_score(Some(s.score)));
                }
            }
            eprintln!();
        }
    }
    Ok(())
}

/// List available scan types.
pub async fn run_types(client: &AnalyzerClient, format: Format) -> Result<()> {
    let types = client.get_scan_types().await?;

    match format {
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::to_value(&types)?)?
            );
        }
        Format::Human | Format::Table => {
            for st in &types {
                eprintln!("\n  {}", style(&st.image_type).bold().underlined());
                for a in &st.analyses {
                    let marker = if a.default {
                        style(format!(" ({})", i18n::text(Text::Default)))
                            .dim()
                            .to_string()
                    } else {
                        String::new()
                    };
                    eprintln!("    - {}{marker}", a.analysis_type);
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn print_status(
    scan_id: Uuid,
    status: &crate::client::models::ScanStatus,
    format: Format,
) -> Result<()> {
    match format {
        Format::Json => {
            let mut map = serde_json::Map::new();
            map.insert("id".into(), serde_json::to_value(scan_id)?);
            map.insert(
                "status".into(),
                serde_json::to_value(status.status.to_string())?,
            );
            for (key, val) in &status.analyses {
                if let Ok(entry) = serde_json::from_value::<AnalysisStatusEntry>(val.clone()) {
                    let mut m = serde_json::Map::new();
                    m.insert("id".into(), serde_json::to_value(entry.id)?);
                    m.insert(
                        "status".into(),
                        serde_json::to_value(entry.status.to_string())?,
                    );
                    map.insert(key.clone(), serde_json::Value::Object(m));
                }
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::Value::Object(map))?
            );
        }
        Format::Human | Format::Table => {
            eprintln!(
                "\n  {} {} ({})",
                style(i18n::text(Text::Scan)).bold(),
                scan_id,
                format_status(&status.status.to_string()),
            );

            let entries: Vec<_> = status
                .analyses
                .iter()
                .filter_map(|(key, val)| {
                    serde_json::from_value::<AnalysisStatusEntry>(val.clone())
                        .ok()
                        .map(|e| (key.clone(), e))
                })
                .collect();

            if !entries.is_empty() {
                eprintln!();
                eprintln!(
                    "  {:<20}  {}",
                    style(i18n::text(Text::Analysis)).underlined(),
                    style(i18n::text(Text::Status)).underlined(),
                );
                for (key, entry) in &entries {
                    eprintln!(
                        "  {:<20}  {}",
                        key,
                        format_status(&entry.status.to_string()),
                    );
                }
            }
            eprintln!();
        }
    }
    Ok(())
}

/// Poll scan status until completion, error, or timeout.
async fn wait_for_completion(
    client: &AnalyzerClient,
    scan_id: Uuid,
    interval: Duration,
    timeout: Duration,
) -> Result<crate::client::models::ScanStatus> {
    let deadline = tokio::time::Instant::now() + timeout;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        indicatif::ProgressStyle::with_template("  {spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["   ", ".  ", ".. ", "...", " ..", "  .", "   "]),
    );
    spinner.enable_steady_tick(Duration::from_millis(120));
    spinner.set_message(i18n::waiting_for_scan());

    loop {
        let status = client.get_scan_status(scan_id).await?;

        match status.status {
            AnalysisStatus::Success => {
                spinner.finish_and_clear();
                output::success(i18n::scan_completed_successfully());
                return Ok(status);
            }
            AnalysisStatus::Error => {
                spinner.finish_and_clear();
                bail!(i18n::scan_failed_with_error_status());
            }
            AnalysisStatus::Canceled => {
                spinner.finish_and_clear();
                bail!(i18n::scan_was_cancelled());
            }
            _ => {
                let mut parts = Vec::new();
                for (key, val) in &status.analyses {
                    if let Ok(entry) = serde_json::from_value::<AnalysisStatusEntry>(val.clone()) {
                        let icon = i18n::progress_word(&entry.status.to_string());
                        parts.push(format!("{key}: {icon}"));
                    }
                }
                spinner.set_message(i18n::analyzing(&parts.join(", ")));
            }
        }

        if tokio::time::Instant::now() >= deadline {
            spinner.finish_and_clear();
            bail!(i18n::timed_out_waiting_for_scan(timeout.as_secs()));
        }

        tokio::time::sleep(interval).await;
    }
}

// ===========================================================================
// Overview
// ===========================================================================

/// Show scan overview.
pub async fn run_overview(client: &AnalyzerClient, scan_id: Uuid, format: Format) -> Result<()> {
    let overview = client.get_scan_overview(scan_id).await?;

    match format {
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&overview)?);
        }
        Format::Human | Format::Table => {
            eprintln!("\n  {} {}\n", style(i18n::text(Text::Scan)).bold(), scan_id);

            if let Some(cve) = &overview.cve {
                let c = &cve.counts;
                eprintln!(
                    "  {} ({})",
                    style(i18n::text(Text::CveVulnerabilities)).bold(),
                    cve.total
                );
                eprintln!(
                    "    Critical: {}  High: {}  Medium: {}  Low: {}  Unknown: {}",
                    style(c.critical).red(),
                    style(c.high).red(),
                    style(c.medium).yellow(),
                    style(c.low).green(),
                    style(c.unknown).dim(),
                );
            }
            if let Some(m) = &overview.malware {
                eprintln!(
                    "  {}: {}",
                    style(i18n::text(Text::MalwareDetections)).bold(),
                    m.count
                );
            }
            if let Some(p) = &overview.password_hash {
                eprintln!(
                    "  {}: {}",
                    style(i18n::text(Text::PasswordIssues)).bold(),
                    p.count
                );
            }
            if let Some(h) = &overview.hardening {
                let c = &h.counts;
                eprintln!(
                    "  {} ({})",
                    style(i18n::text(Text::HardeningIssues)).bold(),
                    h.total
                );
                eprintln!(
                    "    High: {}  Medium: {}  Low: {}",
                    style(c.high).red(),
                    style(c.medium).yellow(),
                    style(c.low).green(),
                );
            }
            if let Some(cap) = &overview.capabilities {
                eprintln!(
                    "  {} ({} executables)",
                    style(i18n::text(Text::Capabilities)).bold(),
                    cap.executable_count
                );
                let c = &cap.counts;
                eprintln!(
                    "    Critical: {}  High: {}  Medium: {}  Low: {}",
                    style(c.critical).red(),
                    style(c.high).red(),
                    style(c.medium).yellow(),
                    style(c.low).green(),
                );
            }
            if let Some(cr) = &overview.crypto {
                eprintln!(
                    "  {}: {} certs, {} public keys, {} private keys",
                    style(i18n::text(Text::Crypto)).bold(),
                    cr.certificates,
                    cr.public_keys,
                    cr.private_keys,
                );
            }
            if let Some(sbom) = &overview.software_bom {
                eprintln!(
                    "  {}: {} components",
                    style(i18n::text(Text::SoftwareBom)).bold(),
                    sbom.count
                );
            }
            if let Some(k) = &overview.kernel {
                eprintln!(
                    "  {}: {} configs",
                    style(i18n::text(Text::Kernel)).bold(),
                    k.count
                );
            }
            if let Some(s) = &overview.symbols {
                eprintln!("  {}: {}", style(i18n::text(Text::Symbols)).bold(), s.count);
            }
            if let Some(t) = &overview.tasks {
                eprintln!("  {}: {}", style(i18n::text(Text::Tasks)).bold(), t.count);
            }
            if let Some(so) = &overview.stack_overflow {
                if let Some(method) = &so.method {
                    eprintln!(
                        "  {}: {}",
                        style(i18n::text(Text::StackOverflow)).bold(),
                        method
                    );
                }
            }
            eprintln!();
        }
    }
    Ok(())
}

// ===========================================================================
// Results
// ===========================================================================

/// Resolve an analysis type name to its UUID by fetching the scan metadata.
async fn resolve_analysis_id(
    client: &AnalyzerClient,
    scan_id: Uuid,
    analysis_type: &AnalysisType,
) -> Result<Uuid> {
    let scan = client.get_scan(scan_id).await?;
    let api_name = analysis_type.api_name();

    for entry in &scan.analysis {
        if entry.entry_type.analyses.iter().any(|a| a == api_name) {
            return Ok(entry.id);
        }
    }

    let available: Vec<_> = scan
        .analysis
        .iter()
        .flat_map(|e| e.entry_type.analyses.iter())
        .collect();
    bail!(
        "analysis type '{}' not found in scan. Available: {}",
        api_name,
        available
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// Browse analysis results.
pub async fn run_results(
    client: &AnalyzerClient,
    scan_id: Uuid,
    analysis_type: AnalysisType,
    page: Option<u32>,
    per_page: Option<u32>,
    search: Option<String>,
    format: Format,
) -> Result<()> {
    let analysis_id = resolve_analysis_id(client, scan_id, &analysis_type).await?;

    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(25);
    let query = ResultsQuery {
        page,
        per_page,
        sort_by: analysis_type.default_sort_by().to_string(),
        sort_ord: "asc".to_string(),
        search,
    };

    let results = client
        .get_analysis_results(scan_id, analysis_id, &query)
        .await?;

    match format {
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Format::Human | Format::Table => {
            if results.findings.is_empty() {
                eprintln!("\n  {}\n", i18n::no_findings());
                return Ok(());
            }

            match analysis_type {
                AnalysisType::Cve => render_cve_table(&results.findings)?,
                AnalysisType::PasswordHash => render_password_table(&results.findings)?,
                AnalysisType::Malware => render_malware_table(&results.findings)?,
                AnalysisType::Hardening => render_hardening_table(&results.findings)?,
                AnalysisType::Capabilities => render_capabilities_table(&results.findings)?,
                AnalysisType::Crypto => render_crypto_table(&results.findings)?,
                AnalysisType::SoftwareBom => render_sbom_table(&results.findings)?,
                AnalysisType::Kernel => render_kernel_table(&results.findings)?,
                AnalysisType::Symbols => render_symbols_table(&results.findings)?,
                AnalysisType::Tasks => render_tasks_table(&results.findings)?,
                AnalysisType::Info => render_info(&results.findings)?,
                AnalysisType::StackOverflow => render_info(&results.findings)?,
            }

            let total_pages = results.total_findings.div_ceil(per_page as u64);
            eprintln!(
                "\n  {}\n",
                i18n::page_navigation(page, total_pages, results.total_findings)
            );
        }
    }
    Ok(())
}

fn render_cve_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<8}  {:<15}  {:<5}  {:<14}  {:<20}  {}",
        style(i18n::text(Text::Severity)).underlined(),
        style("CVE ID").underlined(),
        style(i18n::text(Text::Score)).underlined(),
        style(i18n::text(Text::Vendor)).underlined(),
        style(i18n::text(Text::Product)).underlined(),
        style(i18n::text(Text::Summary)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<CveFinding>(val.clone()) {
            let score_str = f
                .cvss
                .as_ref()
                .and_then(|c| c.v3.as_ref().or(c.v2.as_ref()))
                .and_then(|d| d.base_score)
                .map(|s| format!("{s:.1}"))
                .unwrap_or_default();
            let sev = format_severity(f.severity.as_deref().unwrap_or("unknown"), 8);
            let product = f
                .products
                .first()
                .and_then(|p| p.product.as_deref())
                .unwrap_or("-");
            eprintln!(
                "  {}  {:<15}  {:<5}  {:<14}  {:<20}  {}",
                sev,
                f.cveid.as_deref().unwrap_or("-"),
                score_str,
                truncate_str(f.vendor.as_deref().unwrap_or("-"), 14),
                truncate_str(product, 20),
                truncate_str(f.summary.as_deref().unwrap_or(""), 40),
            );
        }
    }
    Ok(())
}

fn render_password_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<8}  {:<20}  {}",
        style(i18n::text(Text::Severity)).underlined(),
        style(i18n::text(Text::Username)).underlined(),
        style(i18n::text(Text::Password)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<PasswordFinding>(val.clone()) {
            let sev = format_severity(f.severity.as_deref().unwrap_or("unknown"), 8);
            eprintln!(
                "  {}  {:<20}  {}",
                sev,
                f.username.as_deref().unwrap_or("-"),
                f.password.as_deref().unwrap_or("-"),
            );
        }
    }
    Ok(())
}

fn render_malware_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<30}  {:<40}  {}",
        style(i18n::text(Text::Filename)).underlined(),
        style(i18n::text(Text::Description)).underlined(),
        style(i18n::text(Text::Engine)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<MalwareFinding>(val.clone()) {
            eprintln!(
                "  {:<30}  {:<40}  {}",
                truncate_str(f.filename.as_deref().unwrap_or("-"), 30),
                truncate_str(f.description.as_deref().unwrap_or("-"), 40),
                f.detection_engine.as_deref().unwrap_or("-"),
            );
        }
    }
    Ok(())
}

fn render_hardening_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<8}  {:<30}  {:<6}  {:<3}  {:<7}  {:<7}  {}",
        style(i18n::text(Text::Severity)).underlined(),
        style(i18n::text(Text::Filename)).underlined(),
        style(i18n::text(Text::Canary)).underlined(),
        style(i18n::text(Text::Nx)).underlined(),
        style(i18n::text(Text::Pie)).underlined(),
        style(i18n::text(Text::Relro)).underlined(),
        style(i18n::text(Text::Fortify)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<HardeningFinding>(val.clone()) {
            let sev = format_severity(f.severity.as_deref().unwrap_or("unknown"), 8);
            eprintln!(
                "  {}  {:<30}  {}  {}  {:<7}  {:<7}  {}",
                sev,
                truncate_str(f.filename.as_deref().unwrap_or("-"), 30),
                format_bool(f.canary.unwrap_or(false), 6),
                format_bool(f.nx.unwrap_or(false), 3),
                f.pie.as_deref().unwrap_or("-"),
                f.relro.as_deref().unwrap_or("-"),
                format_bool(f.fortify.unwrap_or(false), 7),
            );
        }
    }
    Ok(())
}

fn render_capabilities_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<30}  {:<8}  {:<9}  {}",
        style(i18n::text(Text::Filename)).underlined(),
        style(i18n::text(Text::Severity)).underlined(),
        style(i18n::text(Text::Behaviors)).underlined(),
        style(i18n::text(Text::Syscalls)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<CapabilityFinding>(val.clone()) {
            let sev = format_severity(f.level.as_deref().unwrap_or("unknown"), 8);
            eprintln!(
                "  {:<30}  {}  {:<9}  {}",
                truncate_str(f.filename.as_deref().unwrap_or("-"), 30),
                sev,
                f.behaviors.len(),
                f.syscalls.len(),
            );
        }
    }
    Ok(())
}

/// Format a severity string with color and fixed-width padding.
fn format_severity(severity: &str, width: usize) -> String {
    let padded = format!("{:<width$}", severity.to_uppercase(), width = width);
    match severity.to_lowercase().as_str() {
        "critical" => style(padded).red().bold().to_string(),
        "high" => style(padded).red().to_string(),
        "medium" => style(padded).yellow().to_string(),
        "low" => style(padded).green().to_string(),
        _ => style(padded).dim().to_string(),
    }
}

/// Truncate a string to max chars, adding "..." if needed.
fn truncate_str(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        format!("{:<width$}", s, width = max)
    }
}

/// Format a boolean as colored Yes/No with fixed-width padding.
fn format_bool(val: bool, width: usize) -> String {
    if val {
        style(format!("{:<width$}", "Yes", width = width))
            .green()
            .to_string()
    } else {
        style(format!("{:<width$}", "No", width = width))
            .red()
            .to_string()
    }
}

fn render_crypto_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<14}  {:<20}  {:<20}  {:<8}  {}",
        style(i18n::text(Text::Type)).underlined(),
        style(i18n::text(Text::Filename)).underlined(),
        style("Path").underlined(),
        style(i18n::text(Text::KeySize)).underlined(),
        style(i18n::text(Text::Aux)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<CryptoFinding>(val.clone()) {
            let aux = if f.aux.is_empty() {
                "-".to_string()
            } else {
                f.aux.join(", ")
            };
            eprintln!(
                "  {:<14}  {:<20}  {:<20}  {:<8}  {}",
                truncate_str(f.crypto_type.as_deref().unwrap_or("-"), 14),
                truncate_str(f.filename.as_deref().unwrap_or("-"), 20),
                truncate_str(f.parent.as_deref().unwrap_or("-"), 20),
                f.pubsz.map(|s| s.to_string()).as_deref().unwrap_or("-"),
                truncate_str(&aux, 30),
            );
        }
    }
    Ok(())
}

fn render_sbom_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<30}  {:<14}  {:<12}  {}",
        style(i18n::text(Text::Name)).underlined(),
        style(i18n::text(Text::Version)).underlined(),
        style(i18n::text(Text::Type)).underlined(),
        style(i18n::text(Text::Licenses)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<SbomComponent>(val.clone()) {
            let licenses = f
                .licenses
                .iter()
                .filter_map(|l| {
                    l.get("license")
                        .and_then(|lic| lic.get("id").or_else(|| lic.get("name")))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .collect::<Vec<_>>()
                .join(", ");
            eprintln!(
                "  {:<30}  {:<14}  {:<12}  {}",
                truncate_str(f.name.as_deref().unwrap_or("-"), 30),
                truncate_str(f.version.as_deref().unwrap_or("-"), 14),
                f.component_type.as_deref().unwrap_or("-"),
                if licenses.is_empty() { "-" } else { &licenses },
            );
        }
    }
    Ok(())
}

fn render_kernel_table(values: &[serde_json::Value]) -> Result<()> {
    for val in values {
        if let Ok(f) = serde_json::from_value::<KernelFinding>(val.clone()) {
            if let Some(file) = &f.file {
                eprintln!(
                    "\n  {} {}",
                    style(format!("{}:", i18n::text(Text::KernelConfig))).bold(),
                    file
                );
            }
            if let Some(score_value) = f.score {
                eprintln!("  {}: {}", i18n::text(Text::Score), score_value);
            }
            eprintln!();
            eprintln!(
                "  {:<40}  {}",
                style(i18n::text(Text::Feature)).underlined(),
                style(i18n::text(Text::Status)).underlined(),
            );
            for feat in &f.features {
                eprintln!("  {:<40}  {}", feat.name, format_bool(feat.enabled, 8),);
            }
        }
    }
    Ok(())
}

fn render_symbols_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<40}  {:<12}  {}",
        style(i18n::text(Text::Name)).underlined(),
        style(i18n::text(Text::Type)).underlined(),
        style(i18n::text(Text::Bind)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<IdfSymbolFinding>(val.clone()) {
            eprintln!(
                "  {:<40}  {:<12}  {}",
                truncate_str(f.symbol_name.as_deref().unwrap_or("-"), 40),
                f.symbol_type.as_deref().unwrap_or("-"),
                f.symbol_bind.as_deref().unwrap_or("-"),
            );
        }
    }
    Ok(())
}

fn render_tasks_table(values: &[serde_json::Value]) -> Result<()> {
    eprintln!();
    eprintln!(
        "  {:<30}  {}",
        style(i18n::text(Text::Name)).underlined(),
        style(i18n::text(Text::Function)).underlined(),
    );
    for val in values {
        if let Ok(f) = serde_json::from_value::<IdfTaskFinding>(val.clone()) {
            eprintln!(
                "  {:<30}  {}",
                truncate_str(f.task_name.as_deref().unwrap_or("-"), 30),
                f.task_fn.as_deref().unwrap_or("-"),
            );
        }
    }
    Ok(())
}

fn render_info(values: &[serde_json::Value]) -> Result<()> {
    for val in values {
        eprintln!("\n{}", serde_json::to_string_pretty(val)?);
    }
    Ok(())
}

// ===========================================================================
// Compliance
// ===========================================================================

/// Show compliance check results.
pub async fn run_compliance(
    client: &AnalyzerClient,
    scan_id: Uuid,
    ct: ComplianceType,
    format: Format,
) -> Result<()> {
    let report = client.get_compliance(scan_id, ct).await?;

    match format {
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Format::Human | Format::Table => {
            render_compliance_human(&report, ct);
        }
    }
    Ok(())
}

fn render_compliance_human(report: &ComplianceReport, ct: ComplianceType) {
    let c = &report.checks;
    eprintln!(
        "\n  {} — {}\n",
        style(format!("{} Compliance Report", ct.display_name())).bold(),
        &report.name,
    );
    eprintln!(
        "  {} passed  {} failed  {} unknown  {} N/A  ({} total)\n",
        style(c.passed).green(),
        style(c.failed).red(),
        style(c.unknown).yellow(),
        style(c.not_applicable).dim(),
        c.total,
    );

    for section in &report.sections {
        eprintln!(
            "  {} ({})",
            style(&section.label).bold(),
            section.policy_ref
        );

        for sub in &section.sub_sections {
            eprintln!("    {}", style(&sub.label).underlined());
            eprintln!();
            eprintln!(
                "    {:<8}  {:<16}  {}",
                style("ID").underlined(),
                style("Status").underlined(),
                style("Description").underlined(),
            );
            for req in &sub.requirements {
                let effective_status = req
                    .overwritten_status
                    .as_deref()
                    .unwrap_or(&req.analyzer_status);
                let desc = if req.description.len() > 60 {
                    format!("{}...", &req.description[..57])
                } else {
                    req.description.clone()
                };
                eprintln!(
                    "    {:<8}  {}  {}",
                    req.id,
                    format_compliance_status(effective_status, 16),
                    desc,
                );
            }
            eprintln!();
        }
    }
}

/// Format a compliance status string with color and fixed-width padding.
fn format_compliance_status(status: &str, width: usize) -> String {
    let normalized = status
        .to_lowercase()
        .replace("analyzer-", "")
        .replace("analyzer_", "");
    let padded = format!("{:<width$}", status, width = width);
    match normalized.as_str() {
        "passed" => style(padded).green().to_string(),
        "failed" => style(padded).red().to_string(),
        "unknown" => style(padded).yellow().to_string(),
        "not_applicable" | "not-applicable" | "notapplicable" => style(padded).dim().to_string(),
        _ => padded,
    }
}
