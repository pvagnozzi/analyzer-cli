use analyzer_cli::client::models::AnalysisStatus;
use analyzer_cli::output::{format_score, format_status};

#[test]
fn format_score_handles_none_and_thresholds() {
    let empty = format_score(None);
    let high = format_score(Some(95));
    let medium = format_score(Some(60));
    let low = format_score(Some(15));

    assert!(empty.contains("--"));
    assert!(high.contains("95"));
    assert!(medium.contains("60"));
    assert!(low.contains("15"));
}

#[test]
fn format_status_handles_known_and_unknown_values() {
    assert!(format_status("success").contains("success"));
    assert!(format_status("pending").contains("pending"));
    assert_eq!(format_status("mystery"), "mystery");
}

#[test]
fn analysis_status_display_uses_expected_api_strings() {
    assert_eq!(AnalysisStatus::Success.to_string(), "success");
    assert_eq!(AnalysisStatus::Pending.to_string(), "pending");
    assert_eq!(AnalysisStatus::InProgress.to_string(), "in-progress");
    assert_eq!(AnalysisStatus::Canceled.to_string(), "canceled");
    assert_eq!(AnalysisStatus::Error.to_string(), "error");
}
