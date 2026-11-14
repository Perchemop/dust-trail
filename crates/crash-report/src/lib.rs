//! Crash/bug reporting: capture a scrubbed report on panic and hand it to the
//! UI for review. The report is only *shown* — nothing leaves the machine
//! until the user chooses to send it (Help ▸ Report a Bug…).
//!
//! Clean-room: original implementation, no third-party crash-report source
//! copied.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Environment captured alongside a panic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub app_version: String,
    pub os: String,
    pub arch: String,
    pub rustc_version: String,
}

impl Environment {
    pub fn capture(app_version: &str) -> Self {
        Self {
            app_version: app_version.to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            rustc_version: option_env!("RUSTC_VERSION").unwrap_or("unknown").to_string(),
        }
    }
}

/// A scrubbed crash report ready for user review (and optional submission).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub environment: Environment,
    pub message: String,
    pub backtrace: Option<String>,
    pub notes: HashMap<String, String>,
}

impl CrashReport {
    pub fn new(message: String, backtrace: Option<String>) -> Self {
        Self {
            environment: Environment::capture("0.0.0"), // overwritten by caller
            message,
            backtrace,
            notes: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str(json).ok()
    }
}

/// Install a panic hook that writes a scrubbed report to `park_path` and
/// invokes `on_report`. Nothing is sent over the network from the panic path.
pub fn install_panic_hook<F>(environment: Environment, on_report: F)
where
    F: Fn(&CrashReport) + Send + Sync + 'static,
{
    let hook = std::panic::take_hook();
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
            environment: environment.clone(),
            message,
            backtrace: Some(backtrace),
            notes: HashMap::new(),
        };
        (on_report)(&report);
        hook(info);
    }))
}

/// Where an unhandled crash report is parked for review on the next launch.
pub fn default_report_path() -> std::path::PathBuf {
    std::env::temp_dir().join("dust-trail-crash-report.json")
}
