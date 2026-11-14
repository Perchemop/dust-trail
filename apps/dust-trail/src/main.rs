//! Binary entry point. Defaults to the TUI; pass `--gui` (requires the `gui`
//! feature) to launch the egui/eframe desktop window.
//!
//! Wires the shared suite crates: crash-report panic hook and update-check.

use std::error::Error;

#[cfg(feature = "gui")]
use dust_trail::gui::Gui;
use dust_trail::tui::Tui;

fn run_tui() -> Result<(), Box<dyn Error>> {
    let tui = Tui::new().map_err(|e| format!("tui init failed: {e}"))?;
    tui.run().map_err(|e| format!("tui run failed: {e}").into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let want_gui = args.contains(&"--gui".to_string());

    // Crash reporting (guide §7.1): capture a scrubbed report on panic and park
    // it for review on the next launch. Nothing is sent over the network
    // without the user's consent.
    let env = crash_report::Environment::capture(env!("CARGO_PKG_VERSION"));
    let report_path = crash_report::default_report_path();
    let hook_path = report_path.clone();
    crash_report::install_panic_hook(env, move |report| {
        let _ = std::fs::write(&hook_path, report.to_json());
    });

    // Pick up a report parked by a previous crash so it can be reviewed (stub
    // for v0.1.0; the review-and-send dialog lands in a later slice).
    if let Ok(json) = std::fs::read_to_string(&report_path) {
        if let Some(report) = crash_report::CrashReport::from_json(&json) {
            eprintln!(
                "[crash report parked from previous run]: {} v{} — {}",
                report.environment.os, report.environment.app_version, report.message
            );
            let _ = std::fs::remove_file(&report_path);
        }
    }

    // Update-check stub (guide §7.2): perform a quiet, fail-soft check once per
    // run. The first tick() schedules the check; the second performs it
    // (synchronous, 10s timeout, fail-soft — never blocks startup meaningfully).
    let mut updater = update_check::Updater::new(update_check::Config::default());
    updater.tick();
    updater.tick();
    if let update_check::Status::UpdateAvailable {
        current,
        latest,
        url,
    } = updater.status()
    {
        eprintln!("[update available] Dust Trail {current} → {latest} — {url}");
    }

    #[cfg(feature = "gui")]
    if want_gui {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("Dust Trail — ASCII Western Adventure")
                .with_inner_size([940.0, 720.0])
                .with_min_inner_size([700.0, 500.0]),
            ..Default::default()
        };
        eframe::run_native(
            "Dust Trail",
            options,
            Box::new(|cc| Ok(Box::new(Gui::new(cc)))),
        )
        .map_err(|e: eframe::Error| format!("eframe failed: {e}").into())?;
        return Ok(());
    }

    #[cfg(not(feature = "gui"))]
    if want_gui {
        eprintln!(
            "The `gui` feature is not enabled. Build with: cargo run --features gui -- --gui"
        );
        std::process::exit(2);
    }

    // Default: terminal TUI.
    run_tui()
}
