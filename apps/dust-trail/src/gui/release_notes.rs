//! In-app release-notes popup (guide §9, §12.1).
//!
//! The changelog (`CHANGELOG.md`) and the CI-managed token ledger
//! (`docs/tokens.md`) are embedded at build time so the popup works in a
//! packaged binary, not only in a source checkout.

use crate::gui::Gui;

use std::collections::HashMap;

use egui::{Align, Layout};

/// Changelog embedded at build time.
const EMBEDDED_CHANGELOG: &str = include_str!("../../../../CHANGELOG.md");

/// CI-managed token ledger embedded at build time.
const EMBEDDED_TOKENS: &str = include_str!("../../../../docs/tokens.md");

/// A single parsed release entry from the changelog.
#[derive(Clone, Debug)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub sections: Vec<(String, Vec<String>)>,
}

/// Normalises a release version string so both `## v1.5.0` and `## [1.0.0]`
/// headings produce the same form as the ledger's version column: starts with
/// `v`, surrounding `[` `]` and whitespace stripped.
pub fn normalise_version(version: &str) -> String {
    let trimmed = version
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim();
    if trimmed.starts_with('v') || trimmed.starts_with('V') {
        trimmed.to_string()
    } else {
        format!("v{trimmed}")
    }
}

/// Parsed token ledger from the embedded `docs/tokens.md` table.
pub struct TokenLedger {
    /// Cumulative tokens as of the newest row in the table.
    pub total: u64,
    /// Per-release token counts, keyed by normalised version.
    pub by_version: HashMap<String, u64>,
    /// Per-release cumulative token counts, keyed by normalised version.
    pub cumulative_by_version: HashMap<String, u64>,
}

/// Parse a Markdown token-ledger table (one row per release, newest last).
///
/// Only lines starting with `|` are considered. Cells are split on `|` and
/// trimmed; the leading/trailing empty cells from the outer pipes are dropped.
/// The header row (first cell `Version`) and the separator row (only `-` and
/// `:` characters) are skipped. A data row needs at least 4 cells: version,
/// date, tokens this release, tokens cumulative. Cells 2 and 3 are parsed as
/// `u64` with underscores accepted as digit separators; rows that fail to parse
/// are skipped rather than failing.
pub fn parse_token_ledger(markdown: &str) -> TokenLedger {
    let mut by_version = HashMap::new();
    let mut cumulative_by_version = HashMap::new();
    let mut total = 0u64;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }

        let cells: Vec<&str> = trimmed.split('|').map(str::trim).collect();
        // Drop the empty first and last cells produced by the outer pipes.
        if cells.len() < 2 {
            continue;
        }
        let inner = &cells[1..cells.len() - 1];

        // Skip header row.
        if inner
            .first()
            .is_some_and(|c| c.eq_ignore_ascii_case("Version"))
        {
            continue;
        }
        // Skip separator row: only `-` and `:` characters.
        if !inner
            && inner
                .iter()
                .all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
        {
            continue;
        }

        if inner.len() < 4 {
            continue;
        }
        let version = normalise_version(inner[0]);
        let tokens_this: Option<u64> = inner[2].replace('_', "").parse().ok();
        let tokens_cumulative: Option<u64> = inner[3].replace('_', "").parse().ok();
        let Some(tokens_this) = tokens_this else {
            continue;
        };
        let tokens_cumulative = tokens_cumulative.unwrap_or(tokens_this);

        by_version.insert(version.clone(), tokens_this);
        cumulative_by_version.insert(version.clone(), tokens_cumulative);
        // File order is oldest-first, so the last successfully parsed row wins.
        total = tokens_cumulative;
    }

    TokenLedger {
        total,
        by_version,
        cumulative_by_version,
    }
}

/// Tokens used in a single release, looked up by normalised version.
pub fn release_tokens(ledger: &TokenLedger, version: &str) -> Option<u64> {
    ledger.by_version.get(&normalise_version(version)).copied()
}

/// Cumulative tokens as of a release, looked up by normalised version.
pub fn release_cumulative_tokens(ledger: &TokenLedger, version: &str) -> Option<u64> {
    ledger
        .cumulative_by_version
        .get(&normalise_version(version))
        .copied()
}

/// Parse a Keep-a-Changelog style document into release entries.
pub fn parse_releases(changelog: &str) -> Vec<Release> {
    let mut releases: Vec<Release> = Vec::new();
    let mut current: Option<Release> = None;
    let mut current_section: Option<String> = None;

    for raw in changelog.lines() {
        let line = raw.trim_end();
        let trimmed = line.trim_start();
        if trimmed.starts_with("## ") {
            if !is_release_heading(trimmed) {
                // A prose `##` heading (for example `## Known limitations`)
                // is not a release. Leave the current release open so the
                // bullets underneath it still attach to that release.
                continue;
            }
            if let Some(r) = current.take() {
                releases.push(r);
            }
            let (version, date) = parse_header(trimmed);
            current = Some(Release {
                version,
                date,
                sections: Vec::new(),
            });
            current_section = None;
            continue;
        }
        let Some(ref mut rel) = current else { continue };

        if trimmed.starts_with("### ") {
            let heading = trimmed.trim_start_matches("### ").trim().to_string();
            current_section = Some(heading.clone());
            if !rel.sections.iter().any(|(h, _)| h == &heading) {
                rel.sections.push((heading, Vec::new()));
            }
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let bullet = trimmed.trim_start_matches(['-', '*']).trim().to_string();
            if bullet.is_empty() {
                continue;
            }
            if let Some(sec) = &current_section {
                if let Some(s) = rel.sections.iter_mut().find(|(h, _)| h == sec) {
                    s.1.push(bullet);
                }
            } else {
                if !rel.sections.iter().any(|(h, _)| h == "Notes") {
                    rel.sections.push(("Notes".to_string(), Vec::new()));
                }
                if let Some(s) = rel.sections.iter_mut().find(|(h, _)| h == "Notes") {
                    s.1.push(bullet);
                }
            }
        }
    }
    if let Some(r) = current {
        releases.push(r);
    }
    releases
}

/// Whether a `## ` heading names a release version rather than a prose
/// section.
///
/// Only headings whose first token looks like a semantic version (`v1.5.0`,
/// `[1.0.0]`, `1.5.0`, `v1.5.0-rc.1`) start a release entry. Anything else
/// (`## Notes`, `## Known limitations`) is a prose heading and must not be
/// parsed as a release with a garbage version.
fn is_release_heading(heading: &str) -> bool {
    let text = heading.trim_start_matches('#').trim();
    let first = text.split_whitespace().next().unwrap_or("");
    let token = first.trim_start_matches('[').trim_end_matches(']');
    let token = token
        .strip_prefix('v')
        .or_else(|| token.strip_prefix('V'))
        .unwrap_or(token);

    let Some(first_char) = token.chars().next() else {
        return false;
    };
    if !first_char.is_ascii_digit() {
        return false;
    }
    token
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '+')
}

fn parse_header(header: &str) -> (String, String) {
    let inner = header.trim_start_matches('#').trim();

    let (version, rest) = if inner.starts_with('[') {
        // `## [1.0.0] - 2026-11-16`
        if let Some(bracket_end) = inner.find(']') {
            let version = inner[1..bracket_end].trim().to_string();
            let rest = inner[bracket_end + 1..].trim();
            (version, rest)
        } else {
            (inner.to_string(), "")
        }
    } else {
        // `## v1.5.0 - 2026-11-18` — version is the leading token.
        let mut version_end = inner.len();
        for (i, ch) in inner.char_indices() {
            if ch.is_whitespace() || ch == '-' {
                version_end = i;
                break;
            }
        }
        let version = inner[..version_end].trim().to_string();
        let rest = inner[version_end..].trim();
        (version, rest)
    };

    let date = rest.trim_start_matches('-').trim().to_string();
    let version = normalise_version(&version);
    (version, date)
}

pub fn draw_release_notes(app: &mut Gui, ctx: &egui::Context) {
    if !app.show_release_notes {
        return;
    }

    // Esc closes the popup (keyboard accessibility).
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.show_release_notes = false;
        return;
    }

    let mut open = true;
    egui::Window::new("Release Notes")
        .open(&mut open)
        .resizable(true)
        .default_width(720.0)
        .default_height(560.0)
        .show(ctx, |ui| {
            ui.heading("Dust Trail Release Notes");
            ui.add_space(8.0);
            ui.separator();

            let current_version = env!("CARGO_PKG_VERSION");
            let ledger = parse_token_ledger(EMBEDDED_TOKENS);
            let total = ledger.total;

            ui.horizontal(|ui| {
                ui.label(format!("Current version: {current_version}"));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.small(format!("Cumulative tokens used: {total}"));
                });
            });
            ui.separator();
            ui.add_space(8.0);

            let releases = parse_releases(EMBEDDED_CHANGELOG);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for release in &releases {
                    ui.add_space(8.0);
                    let title = if release.date.is_empty() {
                        release.version.clone()
                    } else {
                        format!("{}  —  {}", release.version, release.date)
                    };
                    ui.strong(title);
                    if let Some(t) = release_tokens(&ledger, &release.version) {
                        ui.small(format!("Tokens this release: {t}"));
                    }
                    ui.add_space(4.0);

                    for (heading, bullets) in &release.sections {
                        ui.label(egui::RichText::new(heading).strong());
                        for b in bullets {
                            ui.label(format!("• {b}"));
                        }
                        ui.add_space(4.0);
                    }
                    ui.separator();
                }
                if releases.is_empty() {
                    ui.label("No release notes found.");
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("View on GitHub").clicked() {
                    let _ = open_url("https://github.com/Perchemop/dust-trail/releases");
                }
                if ui.button("Close").clicked() {
                    app.show_release_notes = false;
                }
            });
        });

    if !open {
        app.show_release_notes = false;
    }
}

fn open_url(url: &str) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
        Ok(())
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        let _ = url;
        Err("unsupported platform".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VBRACKET_CHANGELOG: &str = "# Changelog - Dust Trail\n\n## [1.0.0] - 2026-11-16\n\n### Added\n- First stable release\n\n## [1.1.0] - 2026-11-18\n\n### Added\n- Multi-save slots\n";

    const V_PLAIN_CHANGELOG: &str = "# Changelog - Dust Trail\n\n## v1.5.0 - 2026-11-18\n\n### Added\n- Smarter outlaw AI\n- Deputy AI\n";

    const TOKENS_LEDGER: &str = "| Version | Date       | Tokens this release | Tokens cumulative | Notes                  |\n|---------|------------|---------------------|-------------------|------------------------|\n| v1.0.0  | 2026-11-15 | 160734746           | 160734746         | First functional build |\n| v1.5.1  | 2026-11-20 | 2_511_784_324       | 2_511_784_324     | Release infrastructure |\n";

    #[test]
    fn parse_releases_plain_v_heading() {
        let releases = parse_releases(V_PLAIN_CHANGELOG);
        assert_eq!(releases.len(), 1);
        let r = &releases[0];
        assert_eq!(r.version, "v1.5.0");
        assert_eq!(r.date, "2026-11-18");
        assert_eq!(r.sections.len(), 1);
        assert_eq!(r.sections[0].0, "Added");
        assert_eq!(r.sections[0].1, vec!["Smarter outlaw AI", "Deputy AI"]);
    }

    #[test]
    fn parse_releases_bracket_heading_normalises() {
        let releases = parse_releases(VBRACKET_CHANGELOG);
        assert_eq!(releases.len(), 2);
        assert_eq!(releases[0].version, "v1.0.0");
        assert_eq!(releases[0].date, "2026-11-16");
        assert_eq!(releases[1].version, "v1.1.0");
        assert_eq!(releases[1].date, "2026-11-18");

        // Both forms normalise to the same shape.
        assert_eq!(normalise_version("1.0.0"), "v1.0.0");
        assert_eq!(normalise_version("[1.0.0]"), "v1.0.0");
        assert_eq!(normalise_version("v1.0.0"), "v1.0.0");
    }

    #[test]
    fn parse_token_ledger_basic() {
        let ledger = parse_token_ledger(TOKENS_LEDGER);
        assert_eq!(ledger.total, 2_511_784_324);
        assert_eq!(ledger.by_version.get("v1.0.0"), Some(&160734746));
        assert_eq!(ledger.by_version.get("v1.5.1"), Some(&2_511_784_324));
        assert_eq!(ledger.cumulative_by_version.get("v1.0.0"), Some(&160734746));
    }

    #[test]
    fn parse_releases_ignores_prose_headings() {
        // A `##` heading that is not a version must not become a release.
        let changelog = "# Changelog\n\n## Notes\n- Not a release\n\n## Known limitations\n- Slow on old machines\n\n## v2.0.0 - 2026-11-20\n\n### Added\n- Real release\n";
        let releases = parse_releases(changelog);
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].version, "v2.0.0");
        assert_eq!(releases[0].date, "2026-11-20");
        assert_eq!(releases[0].sections[0].0, "Added");
        assert_eq!(releases[0].sections[0].1, vec!["Real release"]);
    }

    #[test]
    fn parse_releases_prose_heading_after_release_does_not_split_it() {
        let changelog = "## v2.0.0 - 2026-11-20\n\n### Added\n- Real release\n\n## Known limitations\n- Slow on old machines\n";
        let releases = parse_releases(changelog);
        assert_eq!(releases.len(), 1);
        assert_eq!(releases[0].version, "v2.0.0");
    }

    #[test]
    fn parse_token_ledger_extra_columns_and_mixed_separators() {
        // Rows may carry extra columns after the notes column, and the
        // numeric columns may mix underscore and plain digit separators.
        let table = "| Version | Date | Tokens this release | Tokens cumulative | Notes | Extra |\n|---|---|---|---|---|---|\n| v1.0.0 | 2026-11-15 | 1_000 | 1_000 | First | x |\n| v1.1.0 | 2026-11-16 | 2000 | 3_000 | Second | y |\n";
        let ledger = parse_token_ledger(table);
        assert_eq!(ledger.by_version.get("v1.0.0"), Some(&1_000));
        assert_eq!(ledger.by_version.get("v1.1.0"), Some(&2_000));
        assert_eq!(ledger.cumulative_by_version.get("v1.1.0"), Some(&3_000));
        assert_eq!(ledger.total, 3_000);
    }

    #[test]
    fn parse_token_ledger_garbage_and_empty() {
        let empty = parse_token_ledger("");
        assert_eq!(empty.total, 0);
        assert!(empty.by_version.is_empty());

        let garbage = parse_token_ledger("not a table at all\njust random text\n");
        assert_eq!(garbage.total, 0);
        assert!(garbage.by_version.is_empty());

        let bad_rows = "| Version | Date | Tokens this release | Tokens cumulative |\n|---|---|---|---|\n| v1.0.0 | 2026-11-15 | not-a-number | 100 |\n";
        let ledger = parse_token_ledger(bad_rows);
        assert_eq!(ledger.total, 0);
        assert!(ledger.by_version.is_empty());
    }

    #[test]
    fn release_tokens_lookup() {
        let ledger = parse_token_ledger(TOKENS_LEDGER);
        assert_eq!(release_tokens(&ledger, "v1.0.0"), Some(160734746));
        assert_eq!(release_tokens(&ledger, "[1.0.0]"), Some(160734746));
        assert_eq!(release_tokens(&ledger, "1.0.0"), Some(160734746));
        assert_eq!(release_tokens(&ledger, "v9.9.9"), None);
    }
}
