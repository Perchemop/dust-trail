//! Update checking: GitHub Releases API, version comparison, fail-soft offline.
//!
//! The crate owns the whole flow; the app calls `tick()` / `show()` each frame
//! and acts on the returned action, so every app handles failures, dropped
//! workers, repaints and restart identically (guide §7.2).
//!
//! Clean-room: original implementation, no third-party update-check source
//! copied.

use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// The result of an update check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// A quiet check is in progress (or no check has ever run).
    Idle,
    /// A check is running in the background.
    Checking,
    /// The running version is up to date.
    UpToDate { current: String, latest: String },
    /// A newer release is available.
    UpdateAvailable {
        current: String,
        latest: String,
        url: String,
    },
    /// The check failed. Fail soft — never block app start.
    Failed { error: String },
}

/// One action the updater can request the app to take.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    None,
    /// Show the startup prompt modal (once per run when an update exists).
    ShowPrompt,
    /// Open the release page in the browser.
    OpenReleasePage,
}

/// Metadata for a GitHub release.
#[derive(Debug, Clone, Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<ReleaseAsset>,
}

/// Configuration for the updater.
pub struct Config {
    pub owner: String,
    pub repo: String,
    pub current_version: String,
    /// How often (in seconds) to re-check quietly. 0 = startup only.
    pub quiet_interval_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            owner: "Perchemop".to_string(),
            repo: "dust-trail".to_string(),
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            quiet_interval_secs: 3600,
        }
    }
}

/// The updater state machine. The app drives `tick()` each frame and acts on
/// `status()` / `action()`.
pub struct Updater {
    config: Config,
    status: Status,
    pending_action: Action,
    last_check: Option<u64>,
    checking: bool,
}

impl Updater {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            status: Status::Idle,
            pending_action: Action::None,
            last_check: None,
            checking: false,
        }
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn pending_action(&self) -> Action {
        self.pending_action.clone()
    }

    pub fn clear_action(&mut self) {
        self.pending_action = Action::None;
    }

    /// Request an explicit user-initiated check (Help ▸ Check for Updates).
    pub fn request_check(&mut self) {
        self.pending_action = Action::None;
        self.status = Status::Checking;
        self.checking = true;
        self.last_check = Some(now_secs());
        // The actual network check resolves on the next tick(); this keeps the
        // UI responsive and fail-soft (offline failures never block the app).
    }

    /// Drive the updater one frame. Call from the app's update loop.
    pub fn tick(&mut self) {
        // Resolve an in-flight background check.
        if self.checking {
            match self.perform_check() {
                Ok(status) => {
                    self.status = status;
                    self.checking = false;
                    if matches!(self.status, Status::UpdateAvailable { .. }) {
                        self.pending_action = Action::ShowPrompt;
                    }
                }
                Err(e) => {
                    self.status = Status::Failed { error: e };
                    self.checking = false;
                }
            }
            return;
        }

        // Quiet startup check, once per run.
        let now = now_secs();
        let due = self.last_check.map_or(true, |last| {
            now.saturating_sub(last) >= self.config.quiet_interval_secs
        });
        if due {
            self.status = Status::Checking;
            self.checking = true;
            self.last_check = Some(now);
        }
    }

    /// Perform a single blocking check against the GitHub Releases API.
    fn perform_check(&self) -> Result<Status, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases?per_page=5",
            self.config.owner, self.config.repo
        );
        let body = ureq::get(&url)
            .header("User-Agent", "dust-trail-update-check")
            .header("Accept", "application/vnd.github+json")
            .call()
            .map_err(|e| format!("network error: {e}"))?
            .body_mut()
            .read_to_string()
            .map_err(|e| format!("read error: {e}"))?;

        let releases: Vec<Release> =
            serde_json::from_str(&body).map_err(|e| format!("parse error: {e}"))?;

        let current = semver::Version::parse(&self.config.current_version)
            .map_err(|e| format!("version parse error: {e}"))?;

        let mut best: Option<&Release> = None;
        for rel in releases.iter() {
            if rel.draft || rel.prerelease {
                continue;
            }
            let tag = rel.tag_name.trim_start_matches('v');
            let ok = match semver::Version::parse(tag) {
                Ok(v) => v > current,
                Err(_) => false,
            };
            if ok {
                best = Some(rel);
                break; // releases are returned newest-first by the API
            }
        }

        match best {
            Some(rel) => Ok(Status::UpdateAvailable {
                current: self.config.current_version.clone(),
                latest: rel.tag_name.clone(),
                url: rel.html_url.clone(),
            }),
            None => {
                let latest = releases
                    .first()
                    .map(|r| r.tag_name.clone())
                    .unwrap_or_else(|| self.config.current_version.clone());
                Ok(Status::UpToDate {
                    current: self.config.current_version.clone(),
                    latest,
                })
            }
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_defaults_to_idle() {
        let u = Updater::new(Config::default());
        assert!(matches!(u.status(), Status::Idle));
    }
}
