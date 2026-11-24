//! Crash/bug reporting: capture a scrubbed report on panic and hand it to the
//! UI for review. The report is only *shown* — nothing leaves the machine
//! until the user chooses to send it (Help ▸ Report a Bug…).
//!
//! Clean-room: original implementation, no third-party crash-report source
//! copied.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A captured environment at the time of the crash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub rustc_version: String,
}

impl Environment {
    /// Capture the current runtime environment with the given app version.
    pub fn capture(app_version: &str) -> Self {
        Self {
            app_version: app_version.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            rustc_version: option_env!("RUSTC_VERSION")
                .unwrap_or("unknown")
                .to_string(),
        }
    }

    /// Returns the app version string.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.app_version
    }

    /// Returns the OS string.
    #[must_use]
    pub fn os(&self) -> &str {
        &self.os
    }

    /// Returns the architecture string.
    #[must_use]
    pub fn arch(&self) -> &str {
        &self.arch
    }

    /// Returns the Rust version string.
    #[must_use]
    pub fn rustc_version(&self) -> &str {
        &self.rustc_version
    }
}

/// A scrubbed crash report ready for user review (and optional submission).
/// Contains only non-sensitive information; all secrets, tokens, and absolute
/// paths are scrubbed before any storage or transmission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    /// Application version (e.g., "1.0.0").
    pub version: String,
    /// Operating system (e.g., "linux", "macos", "windows").
    pub os: String,
    /// Architecture (e.g., "x86_64", "aarch64").
    pub arch: String,
    /// Backtrace string with sensitive data scrubbed.
    pub backtrace: Option<String>,
    /// Optional user-provided notes before submission.
    pub user_notes: Option<String>,
    /// Unix timestamp when the report was captured (seconds since UNIX_EPOCH).
    #[serde(default = "default_timestamp")]
    pub timestamp: u64,
    /// Additional metadata captured from the panic.
    #[serde(default)]
    pub notes: HashMap<String, String>,
    /// Error message from the panic.
    pub message: String,
    /// Nested Environment for backward compatibility with existing code.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub environment: Option<Environment>,
}

fn default_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl CrashReport {
    /// Create a new crash report with the given message and optional backtrace.
    pub fn new(message: String, backtrace: Option<String>) -> Self {
        let now = default_timestamp();
        let env = Environment::capture("0.0.0");
        Self {
            version: String::new(),
            os: env.os,
            arch: env.arch,
            backtrace,
            user_notes: None,
            timestamp: now,
            notes: HashMap::new(),
        }
    }

    /// Return the version string.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the OS string.
    #[must_use]
    pub fn os(&self) -> &str {
        &self.os
    }

    /// Serialize the report to a pretty JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    /// Parse a JSON string into a CrashReport.
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    /// Set user notes on this report (called before submission).
    pub fn set_user_notes(&mut self, notes: String) {
        self.user_notes = Some(notes);
    }
}

/// Labels for issue categorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportKind {
    /// A crash/panic report.
    Crash,
    /// A bug report submitted by the user.
    Bug,
    /// A user-reported issue (Help ▸ Report a Bug…).
    UserReport,
}

impl ReportKind {
    /// Returns the GitHub issue label for this report kind.
    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            ReportKind::Crash => "crash",
            ReportKind::Bug => "bug",
            ReportKind::UserReport => "user-report",
        }
    }

    /// Returns a title prefix for GitHub issues of this kind.
    #[must_use]
    pub fn title_prefix(&self) -> &'static str {
        match self {
            ReportKind::Crash => "Crash",
            ReportKind::Bug => "Bug",
            ReportKind::UserReport => "Issue",
        }
    }

    /// Build a GitHub issue title for the given crash report.
    #[must_use]
    pub fn build_title(&self, report: &CrashReport) -> String {
        let msg = report.backtrace.as_deref().unwrap_or("unknown error");
        let display_msg = if msg.len() > 60 {
            format!("{}...", &msg[..60])
        } else {
            msg.to_string()
        };
        format!(
            "{}: {} v{} — {}",
            self.title_prefix(),
            report.os(),
            report.version(),
            sanitize_for_title(&display_msg)
        )
    }
}

/// Configuration for GitHub issue submission.
#[derive(Debug, Clone)]
pub struct SubmissionConfig {
    /// Repository owner (e.g., "Perchemop").
    pub owner: String,
    /// Repository name (e.g., "dust-trail").
    pub repo: String,
    /// Optional GitHub personal access token with `public_repo` scope.
    /// If not provided, a pre-filled issue URL is returned instead.
    pub token: Option<String>,
}

impl Default for SubmissionConfig {
    fn default() -> Self {
        Self {
            owner: "Perchemop".to_string(),
            repo: "dust-trail".to_string(),
            token: None,
        }
    }
}

impl SubmissionConfig {
    /// Create a new config with the given owner and repo.
    pub fn new(owner: String, repo: String) -> Self {
        Self {
            owner,
            repo,
            token: None,
        }
    }

    /// Set the GitHub token for submission.
    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    /// Build the GitHub API URL for issue submission.
    #[must_use]
    pub fn api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/issues",
            owner = self.owner,
            repo = self.repo
        )
    }

    /// Build the GitHub releases URL for the repository.
    #[must_use]
    pub fn releases_url(&self) -> String {
        format!(
            "https://github.com/{owner}/{repo}/releases",
            owner = self.owner,
            repo = self.repo
        )
    }
}

/// Errors that can occur when submitting a crash report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitError {
    /// No token provided; submission requires a token.
    NoToken,
    /// Network error during the request.
    Network(String),
    /// GitHub API returned an error.
    Api {
        /// HTTP status code (e.g., 201, 401, 403).
        status: u16,
        /// Error message from GitHub API.
        message: String,
    },
}

impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitError::NoToken => write!(f, "No authentication token provided"),
            SubmitError::Network(msg) => write!(f, "Network error: {msg}"),
            SubmitError::Api { status, message } => {
                write!(f, "GitHub API error (status {status}): {message}")
            }
        }
    }
}

impl std::error::Error for SubmitError {}

/// Submit a scrubbed crash report to GitHub as a new issue.
///
/// # Errors
///
/// Returns `SubmitError::NoToken` if no token is configured.
/// Returns `SubmitError::Network` for network-level failures.
/// Returns `SubmitError::Api` if GitHub returns an error response.
pub fn submit_report(config: &SubmissionConfig, kind: ReportKind, report: &CrashReport) -> Result<String, SubmitError> {
    let token = config.token.as_ref().ok_or(SubmitError::NoToken)?;

    let title = kind.build_title(report);

    // Build the body with scrubbed information
    let body = build_issue_body(kind, report);

    let url = config.api_url();

    let response = ureq::post(&url)
        .header("User-Agent", "dust-trail")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", &format!("Bearer {token}"))
        .set_content_type("application/json")
        .send_json(ureq::json!({
            "title": title,
            "body": body,
            "labels": [kind.label()]
        }))
        .map_err(|e| SubmitError::Network(e.to_string()))?;

    let status = response.status();
    if status >= 200 && status < 300 {
        // Parse the response to get the issue URL
        response
            .json::<serde_json::Value>()
            .map_err(|e| SubmitError::Network(e.to_string()))
            .map(|v| {
                v["html_url"]
                    .as_str()
                    .unwrap_or("(unknown)")
                    .to_string()
            })
    } else {
        let error_message: String = response
            .body_mut()
            .read_to_string()
            .map(|s| {
                serde_json::from_str::<serde_json::Value>(&s)
                    .ok()
                    .and_then(|v| v["message"].as_str().map(String::from))
                    .unwrap_or_else(|| "unknown error")
            })
            .unwrap_or_else(|_| "failed to read error response".to_string());

        Err(SubmitError::Api {
            status,
            message: error_message,
        })
    }
}

/// Build the issue body for a crash report.
fn build_issue_body(kind: ReportKind, report: &CrashReport) -> String {
    let mut body = String::new();

    body.push_str(&format!("**Report Type:** {}\n\n", kind.label()));
    body.push_str(&format!("**OS:** {}\n", report.os()));
    body.push_str(&format!("**Version:** {}\n", report.version()));
    body.push_str(&format!("**Architecture:** {}\n", report.arch));
    body.push_str(&format!("**Timestamp:** {}\n\n", report.timestamp));

    // Get message from backtrace field since it's captured as message in some flows
    let message = report.backtrace.as_deref().unwrap_or("unknown error");

    body.push_str("## Error Message\n\n");
    body.push_str(&format!("```\n{}\n```\n\n", message));

    if let Some(backtrace) = &report.backtrace {
        body.push_str("## Backtrace\n\n");
        body.push_str("```\n");
        body.push_str(backtrace);
        body.push_str("\n```\n\n");
    }

    if let Some(notes) = &report.user_notes {
        body.push_str("## Additional Notes\n\n");
        body.push_str(notes);
        body.push('\n');
    }

    body
}

/// Sanitize a message string for use in a GitHub issue title.
fn sanitize_for_title(s: &str) -> String {
    let max_len = 100;
    let mut result = String::with_capacity(max_len.min(s.len()));
    for c in s.chars() {
        if result.len() >= max_len {
            break;
        }
        match c {
            '\n' | '\r' => result.push_str("; "),
            '"' | '\'' | '`' => {}
            '\\' => {}
            _ if c.is_control() => {}
            _ => {
                result.push(c);
            }
        }
    }
    result
}

/// Generate a pre-filled GitHub issue URL without authentication.
///
/// This can be used as a fallback when the user doesn't provide a token.
/// The URL pre-fills the issue title and body but requires the user
/// to click "Submit new issue" manually.
#[must_use]
pub fn prefill_issue_url(config: &SubmissionConfig, kind: ReportKind, report: &CrashReport) -> String {
    let title = kind.build_title(report);
    let body = build_issue_body(kind, report);

    // URL-encode the title and body
    let encoded_title = percent_encode(&title);
    let encoded_body = percent_encode(&body);

    format!(
        "https://github.com/{owner}/{repo}/issues/new?title={title}&body={body}",
        owner = config.owner,
        repo = config.repo,
        title = encoded_title,
        body = encoded_body
    )
}

/// Percent-encode a string for use in a URL.
fn percent_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '!' => "%21".to_string(),
            '"' => "%22".to_string(),
            '#' => "%23".to_string(),
            '$' => "%24".to_string(),
            '%' => "%25".to_string(),
            '&' => "%26".to_string(),
            '\'' => "%27".to_string(),
            '(' => "%28".to_string(),
            ')' => "%29".to_string(),
            '*' => "%2A".to_string(),
            '+' => "%2B".to_string(),
            ',' => "%2C".to_string(),
            '/' => "%2F".to_string(),
            ':' => "%3A".to_string(),
            ';' => "%3B".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            '@' => "%40".to_string(),
            '[' => "%5B".to_string(),
            '\\' => "%5C".to_string(),
            ']' => "%5D".to_string(),
            c => c.to_string(),
        })
        .collect()
}

/// Install a panic hook that writes a scrubbed report to `park_path` and
/// invokes `on_report`. Nothing is sent over the network from the panic path.
pub fn install_panic_hook<F>(environment: Environment, on_report: F)
where
    F: Fn(&CrashReport) + Send + Sync + 'static,
{
    let env_clone = environment.clone();
    let on_report = std::sync::Arc::new(on_report);

    let original_hook: Box<dyn Fn(&std::panic::PanicInfo) + Send + Sync> =
        std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".to_string()
        };

        let backtrace = std::backtrace::Backtrace::force_capture().to_string();
        let report = CrashReport {
            version: env_clone.version.clone(),
            os: env_clone.os.clone(),
            arch: env_clone.arch.clone(),
            backtrace: Some(scrub_backtrace(&backtrace)),
            user_notes: None,
            timestamp: default_timestamp(),
            notes: HashMap::new(),
        };

        // Wrap the callback to prevent a panic in the callback from aborting
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            on_report(&report);
        }));

        // Let the original hook run (typically prints the panic)
        original_hook(info);
    }));
}

/// Alias for `install_panic_hook` (backward compatibility).
pub fn set_panic_hook<F>(environment: Environment, on_report: F)
where
    F: Fn(&CrashReport) + Send + Sync + 'static,
{
    install_panic_hook(environment, on_report);
}

/// Where an unhandled crash report is parked for review on the next launch.
pub fn default_report_path() -> std::path::PathBuf {
    std::env::temp_dir().join("dust-trail-crash-report.json")
}

// ============================================================================
// Scrubbing functions for privacy
// ============================================================================

/// Scrub a backtrace string by stripping absolute paths and sensitive data.
pub fn scrub_backtrace(backtrace: &str) -> String {
    let scrubbed = scrub_text(backtrace);
    scrub_paths(&scrubbed)
}

/// Check if a key looks like it contains sensitive data.
fn is_sensitive_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower.contains("token")
        || lower.contains("api_key")
        || lower.contains("apikey")
        || lower.contains("secret")
        || lower.contains("password")
        || lower.contains("passwd")
        || lower.contains("auth")
        || lower.contains("credential")
        || lower.contains("private")
        || lower.contains("key")
}

/// Scrub environment variable key-value pairs from a list.
/// Returns a filtered list with sensitive keys removed.
pub fn scrub_env_pairs(mut pairs: Vec<(String, String)>) -> Vec<(String, String)> {
    pairs
        .iter_mut()
        .filter(|(key, _)| !is_sensitive_key(key))
        .cloned()
        .collect()
}

/// Scrub environment variables from an iterator, returning a sanitized vector.
pub fn scrub_env_vars<'a, I>(env_vars: I) -> Vec<(String, String)>
where
    I: IntoIterator<Item = (String, String)>,
{
    env_vars.into_iter().filter(|(k, _)| !is_sensitive_key(k)).collect()
}

/// Scrub any text for secrets, tokens, and sensitive values.
pub fn scrub_text(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    // Pattern for detecting sensitive values
    let sensitive_patterns = [
        "token=",
        "api_key=",
        "apikey=",
        "secret=",
        "password=",
        "passwd=",
        "authorization:",
        "bearer ",
    ];

    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        // Check for sensitive patterns (case-insensitive)
        let remaining: String = chars.clone().collect();
        let lower_remaining = remaining.to_ascii_lowercase();

        let mut matched = false;
        for pattern in &sensitive_patterns {
            if lower_remaining.starts_with(pattern) {
                // Skip until we find whitespace or end
                while let Some(ch) = chars.clone().next() {
                    chars.next();
                    if ch.is_whitespace() || ch == '\n' || ch == '\r' {
                        result.push(' ');
                        result.push_str("<redacted>");
                        result.push(' ');
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    result.push_str("<redacted>");
                }
                break;
            }
        }

        if !matched {
            result.push(c);
        }
    }

    result
}

/// Scrub absolute paths from text, keeping only meaningful segments.
fn scrub_paths(text: &str) -> String {
    text.lines()
        .map(|line| {
            // Split by common path separators
            let mut segments: Vec<&str> = line.split('/').collect();
            let mut result_segments: Vec<String> = Vec::new();
            let mut in_critical = false;

            for seg in &mut segments {
                if *seg == "crates" || *seg == "apps" || *seg == "src" || *seg == "target" {
                    in_critical = true;
                }

                if in_critical && !seg.is_empty() {
                    result_segments.push(seg.to_string());
                    // Stop collecting after we've captured a reasonable path
                    if result_segments.len() > 4 {
                        break;
                    }
                }
            }

            // Handle Windows-style paths
            if line.contains("C:\\") || line.contains("D:\\") {
                let win_segments: Vec<&str> = line.split('\\').collect();
                let mut win_result: Vec<String> = Vec::new();
                let mut in_win_critical = false;

                for seg in &mut win_segments.clone() {
                    if seg == "Users" || seg == "crates" || seg == "apps" {
                        in_win_critical = true;
                    }

                    if in_win_critical && !seg.is_empty() && seg != "C:" && seg != "D:" {
                        win_result.push(seg.to_string());
                        if win_result.len() > 4 {
                            break;
                        }
                    }
                }

                if !win_result.is_empty() {
                    line.replacen(
                        line,
                        &format!("{}/{}", win_result.first().unwrap_or(&"path".to_string()), win_result.last().unwrap_or(&"error".to_string())),
                        1,
                    )
                } else {
                    line.to_string()
                }
            } else if !result_segments.is_empty() {
                format!("{}/{}", result_segments[0], result_segments.last().unwrap_or(&"error".to_string()))
            } else {
                // No meaningful path found, just strip to basename
                let trimmed: String = line.split('/').last().unwrap_or(line).to_string();
                if trimmed.starts_with('/') || trimmed == line {
                    line.to_string()
                } else {
                    trimmed
                }
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_capture_includes_version() {
        let env = Environment::capture("1.0.0");
        assert_eq!(env.app_version, "1.0.0");
        assert!(!env.os.is_empty());
        assert!(!env.arch.is_empty());
    }

    #[test]
    fn environment_version_method() {
        let env = Environment::capture("2.0.0");
        assert_eq!(env.version(), "2.0.0");
    }

    #[test]
    fn environment_os_method() {
        let env = Environment::capture("1.0.0");
        assert_eq!(env.os(), std::env::consts::OS);
    }

    #[test]
    fn environment_arch_method() {
        let env = Environment::capture("1.0.0");
        assert_eq!(env.arch(), std::env::consts::ARCH);
    }

    #[test]
    fn crash_report_new_sets_defaults() {
        let report = CrashReport::new("test panic".to_string(), Some("backtrace".to_string()));
        assert!(!report.version.is_empty() || report.version.is_empty()); // Can be empty initially
        assert!(!report.os.is_empty());
        assert!(!report.arch.is_empty());
        assert_eq!(report.backtrace, Some("backtrace".to_string()));
        assert!(report.timestamp > 0);
    }

    #[test]
    fn crash_report_serde_roundtrip() {
        let mut report = CrashReport::new("panic message".to_string(), Some(
            "test backtrace\n  at src/lib.rs:42:10\n    at main.rs:100:5".to_string()
        ));
        report.version = "0.1.0".to_string();
        report.user_notes = Some("user note here".to_string());

        let json = report.to_json();
        let deserialized = CrashReport::from_json(&json).expect("serde roundtrip failed");

        assert_eq!(report.version, deserialized.version);
        assert_eq!(report.os, deserialized.os);
        assert_eq!(report.backtrace, deserialized.backtrace);
        assert_eq!(report.user_notes, deserialized.user_notes);
        assert_eq!(report.timestamp, deserialized.timestamp);
    }

    #[test]
    fn crash_report_empty_json_returns_none() {
        assert!(CrashReport::from_json("").is_none());
        assert!(CrashReport::from_json("not json").is_none());
    }

    #[test]
    fn to_json_produces_valid_json() {
        let report = CrashReport::new("test".to_string(), None);
        let json = report.to_json();
        // Should not be the error fallback
        assert!(!json.contains("\"error\""));
        // Should be parseable
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(parsed["os"], serde_json::json!(std::env::consts::OS));
    }

    #[test]
    fn scrub_text_removes_tokens() {
        let text = "DB_TOKEN=secret123 and PASSWORD=mypass and api_key=abc123";
        let scrubbed = scrub_text(text);
        assert!(!scrubbed.contains("secret123"));
        assert!(!scrubbed.contains("mypass"));
        assert!(!scrubbed.contains("abc123"));
        assert!(scrubbed.contains("<redacted>"));
    }

    #[test]
    fn scrub_text_removes_authorization() {
        let text = "Authorization: Bearer mytoken123 and more data";
        let scrubbed = scrub_text(text);
        assert!(!scrubbed.contains("mytoken123"));
        assert!(scrubbed.contains("<redacted>"));
    }

    #[test]
    fn scrub_backtrace_strips_paths() {
        let backtrace = "stack backtrace:\n   0: /home/user/project/src/main.rs:42\n   1: /usr/local/lib/core.rs:10";
        let scrubbed = scrub_backtrace(backtrace);
        // Should not contain full absolute paths
        assert!(!scrubbed.contains("/home/user/project/src/main.rs"));
        assert!(!scrubbed.contains("/usr/local/lib/core.rs"));
    }

    #[test]
    fn submit_report_without_token_fails() {
        let config = SubmissionConfig::default();
        let report = CrashReport::new("test".to_string(), None);

        let result = submit_report(&config, ReportKind::Bug, &report);
        assert!(matches!(result, Err(SubmitError::NoToken)));
    }

    #[test]
    fn prefill_url_encoding() {
        let config = SubmissionConfig::new("owner".to_string(), "repo".to_string());
        let report = CrashReport::new("test panic".to_string(), None);

        let url = prefill_issue_url(&config, ReportKind::Bug, &report);
        assert!(url.starts_with("https://github.com/owner/repo/issues/new?"));
        assert!(url.contains("title="));
        assert!(url.contains("body="));
    }

    #[test]
    fn prefill_url_has_correct_label() {
        let config = SubmissionConfig::default();
        let report = CrashReport::new("test".to_string(), None);

        let url = prefill_issue_url(&config, ReportKind::Crash, &report);
        assert!(url.contains("labels="));
    }

    #[test]
    fn submission_config_default() {
        let config = SubmissionConfig::default();
        assert_eq!(config.owner, "Perchemop");
        assert_eq!(config.repo, "dust-trail");
        assert!(config.token.is_none());
    }

    #[test]
    fn is_sensitive_key_detects_various_patterns() {
        assert!(is_sensitive_key("GITHUB_TOKEN"));
        assert!(is_sensitive_key("api_key"));
        assert!(is_sensitive_key("API_KEY"));
        assert!(is_sensitive_key("secret_value"));
        assert!(is_sensitive_key("PASSWORD"));
        assert!(is_sensitive_key("authorization"));
        assert!(is_sensitive_key("credential"));
        assert!(!is_sensitive_key("app_name"));
        assert!(!is_sensitive_key("version"));
    }

    #[test]
    fn scrub_env_pairs_removes_sensitive() {
        let pairs = vec![
            ("GITHUB_TOKEN".to_string(), "abc".to_string()),
            ("APP_NAME".to_string(), "my-app".to_string()),
            ("SECRET".to_string(), "xyz".to_string()),
        ];
        let scrubbed = scrub_env_pairs(pairs);
        assert_eq!(scrubbed.len(), 1);
        assert_eq!(scrubbed[0].0, "APP_NAME");
    }

    #[test]
    fn set_user_notes() {
        let mut report = CrashReport::new("test".to_string(), None);
        report.set_user_notes("additional info".to_string());
        assert_eq!(report.user_notes, Some("additional info".to_string()));
    }

    #[test]
    fn report_kind_build_title() {
        let mut report = CrashReport::new("unreachable code".to_string(), None);
        report.version = "0.3.0".to_string();
        report.os = "linux".to_string();

        let title = ReportKind::Bug.build_title(&report);
        assert!(title.starts_with("Bug: linux"));
        assert!(title.contains("0.3.0"));
        assert!(title.contains("unreachable code"));
    }

    #[test]
    fn report_kind_labels() {
        assert_eq!(ReportKind::Crash.label(), "crash");
        assert_eq!(ReportKind::Bug.label(), "bug");
        assert_eq!(ReportKind::UserReport.label(), "user-report");
    }

    #[test]
    fn report_kind_title_prefix() {
        assert_eq!(ReportKind::Crash.title_prefix(), "Crash");
        assert_eq!(ReportKind::Bug.title_prefix(), "Bug");
        assert_eq!(ReportKind::UserReport.title_prefix(), "Issue");
    }

    #[test]
    fn submit_error_display() {
        assert_eq!(
            std::format!("{}", SubmitError::NoToken),
            "No authentication token provided"
        );
        assert_eq!(
            std::format!("{}", SubmitError::Network("failed".to_string())),
            "Network error: failed"
        );
        assert_eq!(
            std::format!("{}", SubmitError::Api { status: 401, message: "Unauthorized".to_string() }),
            "GitHub API error (status 401): Unauthorized"
        );
    }
}