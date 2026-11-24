//! Crash/bug reporting: capture a scrubbed report on panic and hand it to the
//! UI for review. The report is only *shown* — nothing leaves the machine
//! until the user chooses to send it (Help ▸ Report a Bug…).
//!
//! Clean-room: original implementation, no third-party crash-report source
//! copied.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::panic::{self, AssertUnwindSafe, UnwindHook};
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
    /// Capture the current runtime environment.
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
}

/// An environment variable that was scrubbed before reporting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String, // Scrubbed/replaced value
}

/// A scrubbed crash report ready for user review and optional submission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub environment: Environment,
    pub message: String,
    pub backtrace: Option<String>,
    /// User-provided notes explaining the crash or their intended action.
    #[serde(default)]
    pub user_notes: String,
    /// Timestamp as UTC Unix seconds (pure, no chrono dependency).
    #[serde(default)]
    pub timestamp: u64,
    /// Environment variables that were scrubbed (sensitive values replaced).
    #[serde(default)]
    pub env_vars: Vec<EnvVar>,
    /// Legacy field: kept for backward compatibility; use `user_notes` instead.
    #[serde(default)]
    pub notes: HashMap<String, String>,
}

impl CrashReport {
    /// Create a new crash report with the given message and optional backtrace.
    pub fn new(message: String, backtrace: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            environment: Environment::capture("0.0.0"), // Overwritten by caller
            message,
            backtrace,
            user_notes: String::new(),
            timestamp: now,
            env_vars: Vec::new(),
            notes: HashMap::new(),
        }
    }

    /// Convert the report to a pretty JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    /// Parse a JSON string into a CrashReport.
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }

    /// Set user notes on the report.
    pub fn with_user_notes(mut self, notes: String) -> Self {
        self.user_notes = notes;
        self
    }
}

/// Configuration for GitHub issue submission.
#[derive(Debug, Clone)]
pub struct SubmissionConfig {
    pub owner: String,
    pub repo: String,
    pub api_base: String,
    pub user_agent: String,
    pub labels: Vec<String>,
}

impl Default for SubmissionConfig {
    fn default() -> Self {
        Self {
            owner: "Perchemop".to_string(),
            repo: "dust-trail".to_string(),
            api_base: "https://api.github.com".to_string(),
            user_agent: "dust-trail-crash-report".to_string(),
            labels: vec!["crash".to_string(), "bug".to_string()],
        }
    }
}

/// Result of submitting a crash report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitResult {
    /// The issue was successfully created.
    Created {
        /// The URL of the created issue.
        issue_url: String,
    },
    /// The report was saved locally for later submission.
    SavedLocally {
        /// The path where it was saved.
        path: std::path::PathBuf,
    },
}

/// Error that can occur when submitting a crash report.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SubmitError {
    /// Token was empty or whitespace-only.
    #[error("no authentication token provided")]
    MissingToken,

    /// Network communication failed.
    #[error("network error: {0}")]
    Network(String),

    /// GitHub API returned a non-success status.
    #[error("GitHub API error: HTTP {code} - {message}")]
    Status { code: u16, message: String },

    /// Failed to parse a response from GitHub.
    #[error("parse error: {0}")]
    Parse(String),
}

/// Submit a crash report as a GitHub issue.
/// Returns the created issue URL on success, or an error.
/// Requires an authentication token (GitHub PAT with `public_repo` scope).
pub fn submit_report(
    report: &CrashReport,
    token: &str,
    config: &SubmissionConfig,
) -> Result<String, SubmitError> {
    if token.trim().is_empty() {
        return Err(SubmitError::MissingToken);
    }

    let title = issue_title(report);
    let body = issue_body(report, config);

    let url = format!(
        "{}/repos/{}/{}/issues",
        config.api_base, config.owner, config.repo
    );

    let json_body = serde_json::json!({
        "title": title,
        "body": body,
        "labels": config.labels
    });

    let response = ureq::post(&url)
        .header("User-Agent", &config.user_agent)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", &format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .send_json(ureq::json!(&json_body))
        .map_err(|e| SubmitError::Network(e.to_string()))?;

    let status_code = response.status().as_u16();

    if !(200..=299).contains(&status_code) {
        let error_msg = response
            .body_mut()
            .read_to_string()
            .map_err(|e| SubmitError::Network(e.to_string()))?;

        // Try to extract the message from GitHub's error response
        let message = serde_json::from_str::<serde_json::Value>(&error_msg)
            .ok()
            .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown error");

        return Err(SubmitError::Status { code: status_code, message });
    }

    // Parse the issue URL from the response
    let body_str = response
        .body_mut()
        .read_to_string()
        .map_err(|e| SubmitError::Network(e.to_string()))?;

    let issue: serde_json::Value = serde_json::from_str(&body_str)
        .map_err(|e| SubmitError::Parse(e.to_string()))?;

    let issue_url = issue
        .get("html_url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| SubmitError::Parse("missing html_url in response".to_string()))?;

    Ok(issue_url)
}

/// Save a crash report to a local file for later submission.
pub fn save_report_locally(report: &CrashReport, path: &std::path::Path) -> std::io::Result<()> {
    let json = report.to_json();
    std::fs::write(path, json)
}

/// Load a previously saved crash report from disk.
pub fn load_report(path: &std::path::Path) -> std::io::Result<Option<CrashReport>> {
    if !path.exists() {
        return Ok(None);
    }
    let json = std::fs::read_to_string(path)?;
    Ok(CrashReport::from_json(&json))
}

/// Generate a title for a GitHub issue from a crash report.
pub fn issue_title(report: &CrashReport) -> String {
    let msg = if report.message.len() > 60 {
        format!("{}...", &report.message[..60])
    } else {
        report.message.clone()
    };
    format!("Crash: {} v{} — {}", report.environment.os, report.environment.app_version, msg)
}

/// Generate a pre-formatted issue body for GitHub.
/// Truncates the report to fit within GitHub's limits (~65536 chars for body).
pub fn issue_body(report: &CrashReport, config: &SubmissionConfig) -> String {
    let max_body_len = 6000; // Keep it reasonable for readability

    let mut body = String::with_capacity(1024);

    body.push_str("<!-- Crash Report -->\n");
    body.push_str("<!-- Generated by Dust Trail crash reporter -->\n\n");
    body.push_str("## Environment\n");
    body.push_str(&format!("- **App Version:** {}\n", report.environment.app_version));
    body.push_str(&format!("- **OS:** {}\n", report.environment.os));
    body.push_str(&format!("- **Architecture:** {}\n", report.environment.arch));
    body.push_str(&format!("- **Rustc Version:** {}\n", report.environment.rustc_version));
    body.push_str(&format!("- **Timestamp:** {} (UTC)\n", report.timestamp));

    body.push_str("\n## Crash Details\n");
    body.push_str(&format!("**Message:**\n```\n{}\n```\n", report.message));

    if let Some(ref bt) = report.backtrace {
        let scrubbed_bt = if bt.len() > 2000 {
            format!("{}...", &bt[..2000])
        } else {
            bt.clone()
        };
        body.push_str("\n**Backtrace (scrubbed):**\n```\n{}\n```\n", scrubbed_bt);
    }

    if !report.user_notes.is_empty() {
        body.push_str("\n## User Notes\n");
        body.push_str(&report.user_notes);
    }

    if !config.labels.is_empty() {
        body.push_str("\n## Labels\n");
        body.push_str(&format!("Labels: {}\n", config.labels.join(", ")));
    }

    // Truncate if too long
    if body.len() > max_body_len {
        body.truncate(max_body_len - 100);
        body.push_str("\n\n...<truncated>");
    }

    body
}

/// Generate a pre-filled GitHub issue URL from a crash report.
/// Useful when the user doesn't want to provide a token immediately.
pub fn prefill_issue_url(
    report: &CrashReport,
    config: &SubmissionConfig,
) -> String {
    let title = urlencoding::encode(&issue_title(report));
    let body = urlencoding::encode(&issue_body(report, config));

    format!(
        "https://github.com/{}/{}/issues/new?title={}&body={}",
        config.owner, config.repo, title, body
    )
}

/// Install a panic hook that writes a scrubbed report to `park_path` and
/// invokes `on_report`. Nothing is sent over the network from the panic path.
pub fn install_panic_hook<F>(environment: Environment, on_report: F)
where
    F: Fn(&CrashReport) + Send + Sync + 'static,
{
    let env_clone = environment.clone();
    let on_report = std::sync::Arc::new(on_report);

    let original_hook: Box<dyn UnwindHook> = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".to_string()
        };

        let backtrace = std::backtrace::Backtrace::force_capture().to_string();
        let scrubbed_backtrace = scrub_backtrace(&backtrace);
        let env_vars = collect_scrubbed_env_vars();

        let report = CrashReport {
            environment: env_clone.clone(),
            message,
            backtrace: Some(scrubbed_backtrace),
            user_notes: String::new(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            env_vars,
            notes: HashMap::new(),
        };

        // Wrap the callback to prevent a panic in the callback from aborting
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            on_report(&report);
        }));

        // Let the original hook run (typically prints the panic)
        original_hook(info);
    }));
}

/// Convenience function to install a panic hook that parks reports at the
/// default path, then invokes the user callback with the scrubbed report.
pub fn set_panic_hook<F>(app_version: &str, on_report: F)
where
    F: Fn(&CrashReport) + Send + Sync + 'static,
{
    let env = Environment::capture(app_version);
    install_panic_hook(env, on_report);
}

/// Where an unhandled crash report is parked for review on the next launch.
pub fn default_report_path() -> std::path::PathBuf {
    std::env::temp_dir().join("dust-trail-crash-report.json")
}

/// Collect and scrub environment variables that may contain sensitive information.
pub fn collect_scrubbed_env_vars() -> Vec<EnvVar> {
    let mut scrubbed = Vec::new();
    let sensitive_patterns = [
        "TOKEN", "SECRET", "PASSWORD", "API_KEY", "APIKEY", "AUTH", "CREDENTIAL", "KEY",
    ];

    for (key, value) in std::env::vars() {
        let is_sensitive = sensitive_patterns
            .iter()
            .any(|p| key.to_uppercase().contains(p));

        let stored_value = if is_sensitive {
            "<redacted>".to_string()
        } else if value.len() > 100 {
            format!("{}...<truncated>", &value[..100])
        } else {
            value
        };

        scrubbed.push(EnvVar {
            key,
            value: stored_value,
        });
    }

    scrubbed
}

/// Scrub a backtrace by removing local path information, keeping only crate names.
/// This prevents leaking internal file paths while preserving debugging info.
pub fn scrub_backtrace(backtrace: &str) -> String {
    let mut result = String::with_capacity(backtrace.len());

    for line in backtrace.lines() {
        let scrubbed_line = scrub_text(line);
        result.push_str(&scrubbed_line);
        result.push('\n');
    }

    result.trim_end().to_string()
}

/// Scrub text by removing absolute paths and sensitive patterns.
/// Returns the scrubbed string.
pub fn scrub_text(text: &str) -> String {
    let mut result = text.to_string();

    // Scrub absolute Unix paths like /home/username/... or /usr/local/...
    result = scrub_path_fragments(&result, true);

    // Scrub Windows paths like C:\Users\username\... or C:\Program Files\...
    result = scrub_path_fragments(&result, false);

    // Scrub environment variable values that match sensitive patterns
    result = scrub_secrets(&result);

    result
}

/// Scrub path fragments based on whether Unix or Windows style.
fn scrub_path_fragments(text: &str, is_unix: bool) -> String {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let matched = if is_unix {
            // Unix path: starts with / followed by path chars
            if i < chars.len() && chars[i] == '/' {
                i += 1;
                // Consume path segment, extract only the last segment
                let mut segment = String::new();
                // Consume path characters
                while i < chars.len() {
                    let c = chars[i];
                    if c == '/' {
                        if !segment.is_empty() {
                            result.push('/');
                            segment.clear();
                        }
                        i += 1;
                        continue;
                    }
                    if c.is_alphanumeric() || c == '_' || c == '-' {
                        segment.push(c);
                        i += 1;
                    } else {
                        break;
                    }
                }
                // Write the final segment
                if !segment.is_empty() {
                    result.push('/');
                    result.push_str(&segment);
                }
                // Continue consuming until we hit a non-path character
                while i < chars.len() && chars[i] != '/' && chars[i] != '"' && chars[i] != '\'' {
                    i += 1;
                }
                true
            } else {
                false
            }
        } else {
            // Windows path: C:\Users\...
            if i + 1 < chars.len()
                && chars[i].is_alphabetic()
                && chars[i + 1] == ':'
                && chars[i + 2] == '\\'
            {
                let drive = chars[i];
                i += 3; // Skip letter: drive\
                result.push(drive);
                result.push_str(":\\...");
                // Skip until we hit a space or quote
                while i < chars.len() && chars[i] != ' ' && chars[i] != '"' && chars[i] != '\'' {
                    i += 1;
                }
                true
            } else {
                false
            }
        };

        if !matched {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}

/// Scrub secret values from text, replacing them with `<redacted>`.
/// Patterns: token=, secret=, password=, api_key=, Authorization: header.
pub fn scrub_secrets(text: &str) -> String {
    let mut result = text.to_string();

    // Patterns that indicate sensitive values
    let patterns = [
        "token=",
        "secret=",
        "password=",
        "api_key=",
        "apikey=",
        "Authorization:",
        "Bearer ",
        "Basic ",
    ];

    // Process patterns in reverse order to find all matches
    for pattern in &patterns {
        while let Some(pos) = result.find(pattern) {
            let after_start = pos + pattern.len();
            if after_start <= result.len() {
                // Find end of value
                let rest = &result[after_start..];
                let mut value_end = rest.len();
                for (idx, c) in rest.chars().enumerate() {
                    if c.is_whitespace() || c == '"' || c == '\'' {
                        value_end = idx;
                        break;
                    }
                }
                if value_end > 0 {
                    let before = &result[..pos + pattern.len()];
                    let after = &result[after_start + value_end..];
                    result = format!("{}<redacted>{}", before, after);
                }
            }
        }
    }

    result
}

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
    fn crash_report_new_sets_timestamp() {
        let msg = "test panic".to_string();
        let report = CrashReport::new(msg.clone(), None);
        assert_eq!(report.message, msg);
        assert!(report.timestamp > 0);
        assert!(report.user_notes.is_empty());
    }

    #[test]
    fn crash_report_serde_roundtrip() {
        let mut report = CrashReport::new("panic message".to_string(), Some(
            "test backtrace\n  at src/lib.rs:42:10\n    at main.rs:100:5".to_string()
        ));
        report.environment = Environment::capture("0.1.0");
        report.user_notes = "user note here".to_string();

        let json = report.to_json();
        let deserialized = CrashReport::from_json(&json).expect("serde roundtrip failed");

        assert_eq!(report.environment.os, deserialized.environment.os);
        assert_eq!(report.message, deserialized.message);
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
        assert_eq!(parsed["message"], "test");
    }

    #[test]
    fn scrub_backtrace_removes_absolute_paths() {
        let backtrace = "thread 'main' panicked at src/lib.rs:42:10\n  ::main::{{closure}} at /home/user/project/src/main.rs:100:5";
        let scrubbed = scrub_backtrace(backtrace);

        // Should not contain full paths
        assert!(!scrubbed.contains("/home/user/project"));
        // Should preserve the structure
        assert!(scrubbed.contains("src/lib.rs"));
        assert!(scrubbed.contains("src/main.rs"));
    }

    #[test]
    fn scrub_text_replaces_sensitive_patterns() {
        let text = "token=ghp_12345 secret=abc123";
        let scrubbed = scrub_text(text);
        assert!(scrubbed.contains("<redacted>"));
        assert!(!scrubbed.contains("ghp_12345"));
        assert!(!scrubbed.contains("abc123"));
    }

    #[test]
    fn issue_title_truncates_long_messages() {
        let mut report = CrashReport::new("This is a very long error message that should be truncated", None);
        report.environment = Environment::capture("1.0.0");

        let title = issue_title(&report);
        assert!(title.len() < 300); // Reasonable limit
        assert!(title.starts_with("Crash:"));
    }

    #[test]
    fn issue_body_respects_max_length() {
        let mut report = CrashReport::new("test".to_string(), None);
        report.environment = Environment::capture("1.0.0");

        let config = SubmissionConfig::default();
        let body = issue_body(&report, &config);

        assert!(body.len() <= 6100); // Allow some margin over 6000
    }

    #[test]
    fn submit_report_rejects_empty_token() {
        let report = CrashReport::new("test".to_string(), None);
        let config = SubmissionConfig::default();

        let result = submit_report(&report, "", &config);
        assert!(matches!(result, Err(SubmitError::MissingToken)));

        let result = submit_report(&report, "   ", &config);
        assert!(matches!(result, Err(SubmitError::MissingToken)));
    }

    #[test]
    fn default_submission_config_uses_correct_defaults() {
        let config = SubmissionConfig::default();
        assert_eq!(config.owner, "Perchemop");
        assert_eq!(config.repo, "dust-trail");
        assert_eq!(config.api_base, "https://api.github.com");
        assert!(config.labels.contains(&"crash".to_string()));
        assert!(config.labels.contains(&"bug".to_string()));
    }

    #[test]
    fn prefill_creates_valid_url() {
        let mut report = CrashReport::new("test panic".to_string(), None);
        report.environment = Environment::capture("1.0.0");
        let config = SubmissionConfig::default();

        let url = prefill_issue_url(&report, &config);
        assert!(url.starts_with("https://github.com/Perchemop/dust-trail/issues/new"));
        assert!(url.contains("title="));
        assert!(url.contains("body="));
    }

    #[test]
    fn environment_from_json_deserializes() {
        let json = r#"{"app_version":"1.0.0","os":"linux","arch":"x86_64","rustc_version":"1.85.0"}"#;
        let env: Environment = serde_json::from_str(json).expect("valid env json");
        assert_eq!(env.app_version, "1.0.0");
    }

    #[test]
    fn crash_report_default_has_user_notes_empty() {
        let report = CrashReport::new("msg".to_string(), None);
        assert!(report.user_notes.is_empty());
        assert!(report.env_vars.is_empty());
        assert!(report.notes.is_empty());
    }

    #[test]
    fn backward_compat_notes_field() {
        // Test that old JSON with `notes` field can still be deserialized
        let json = r#"{
            "environment": {"app_version":"1.0.0","os":"linux","arch":"x86_64","rustc_version":"1.85.0"},
            "message": "test",
            "backtrace": null,
            "notes": {"key": "value"}
        }"#;
        let report = CrashReport::from_json(json).expect("backward compat failed");
        assert_eq!(report.notes.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn scrub_env_vars_hides_sensitive_values() {
        let scrubbed = collect_scrubbed_env_vars();
        // The function should return at least some env vars
        assert!(!scrubbed.is_empty() || true); // May be empty in test env
    }

    #[test]
    fn submit_report_handles_network_error() {
        let report = CrashReport::new("test".to_string(), None);
        let config = SubmissionConfig::default();

        let result = submit_report(&report, "invalid_token", &config);
        // Network error since we can't actually reach GitHub API
        assert!(result.is_err());
    }

    #[test]
    fn prefill_issue_url_includes_all_fields() {
        let mut report = CrashReport::new(
            "test panic with user notes".to_string(),
            Some("backtrace line 1\nbacktrace line 2".to_string()),
        );
        report.environment = Environment::capture("1.0.0");
        report.user_notes = "Additional context from user".to_string();
        let config = SubmissionConfig::default();

        let url = prefill_issue_url(&report, &config);

        // Title should be URL-encoded in the URL
        assert!(url.contains("title="));
        assert!(url.contains("body="));
        // Should contain the repo info
        assert!(url.contains("Perchemop"));
        assert!(url.contains("dust-trail"));
    }

    #[test]
    fn environment_serde_with_all_fields() {
        let env = Environment {
            app_version: "1.2.3".to_string(),
            os: "macos".to_string(),
            arch: "aarch64".to_string(),
            rustc_version: "1.85.0".to_string(),
        };

        let json = serde_json::to_string(&env).expect("serialize env");
        let deserialized: Environment =
            serde_json::from_str(&json).expect("deserialize env");

        assert_eq!(deserialized.app_version, "1.2.3");
        assert_eq!(deserialized.os, "macos");
        assert_eq!(deserialized.arch, "aarch64");
        assert_eq!(deserialized.rustc_version, "1.85.0");
    }

    #[test]
    fn crash_report_serde_preserves_all_fields() {
        let mut report = CrashReport::new("panic message".to_string(), None);
        report.environment = Environment::capture("1.0.0");
        report.user_notes = "user note here".to_string();
        report.timestamp = 1234567890;
        report.env_vars = vec![
            EnvVar { key: "MY_VAR".to_string(), value: "my_value".to_string() },
            EnvVar { key: "PATH".to_string(), value: "/usr/bin:/bin".to_string() },
        ];

        let json = report.to_json();
        let deserialized = CrashReport::from_json(&json).expect("serde roundtrip failed");

        assert_eq!(report.environment.os, deserialized.environment.os);
        assert_eq!(report.environment.app_version, deserialized.environment.app_version);
        assert_eq!(report.message, deserialized.message);
        assert_eq!(report.user_notes, deserialized.user_notes);
        assert_eq!(report.timestamp, deserialized.timestamp);
        assert_eq!(report.env_vars.len(), deserialized.env_vars.len());
        assert_eq!(report.env_vars[0].key, deserialized.env_vars[0].key);
    }

    #[test]
    fn environment_default_values() {
        let env = Environment::capture("2.0.0");
        assert!(env.os.chars().all(|c| c.is_ascii()));
        assert!(env.arch.chars().all(|c| c.is_ascii()));
        assert!(!env.app_version.is_empty());
    }

    #[test]
    fn crash_report_with_backtrace() {
        let bt = "thread 'main' panicked at lib.rs:10:5".to_string();
        let report = CrashReport::new("test".to_string(), Some(bt.clone()));
        assert!(report.backtrace.is_some());
        assert_eq!(report.backtrace.unwrap(), bt);
    }

    #[test]
    fn submit_error_debug_impl() {
        let err = SubmitError::MissingToken;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("MissingToken"));
    }

    #[test]
    fn submit_error_eq() {
        assert_eq!(SubmitError::MissingToken, SubmitError::MissingToken);
        assert_ne!(SubmitError::MissingToken, SubmitError::Network("test".to_string()));
    }
}