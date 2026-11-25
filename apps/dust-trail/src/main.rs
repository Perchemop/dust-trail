//! Binary entry point.
//!
//! Dust Trail is GUI-only. The egui/eframe desktop window is the
//! sole frontend:
//!   - `--gui` forces the desktop window explicitly.
//!   - `--tui` is rejected; the terminal frontend was removed in v2.0.0.
//!   - With no flag the window opens when no terminal is attached, so the
//!     macOS `.app` bundle (and any desktop launcher without `Terminal=true`)
//!     opens a real window.
//!
//! Wires the shared suite crates: crash-report panic hook and update-check.

#[cfg(feature = "gui")]
use dust_trail::gui::Gui;
use std::error::Error;

/// Whether this process is attached to an interactive terminal.
#[cfg(feature = "gui")]
fn terminal_attached() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal() && std::io::stdin().is_terminal()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    // The terminal frontend was removed in v2.0.0 (spec FR-008): reject the
    // retired flag with a clear message instead of silently ignoring it.
    if args.iter().any(|arg| arg == "--tui") {
        eprintln!(
            "The TUI was removed in v2.0.0. Dust Trail is GUI-only; \
             run with --gui to open the desktop window."
        );
        std::process::exit(2);
    }

    let force_gui = args.iter().any(|arg| arg == "--gui");

    // Crash reporting (guide §7.1): capture a scrubbed report on panic and park
    // it for review on the next launch. Nothing is sent over the network
    // without the user's consent.
    let env = crash_report::Environment::capture(env!("CARGO_PKG_VERSION"));
    let report_path = crash_report::default_report_path();
    let hook_path = report_path.clone();
    crash_report::install_panic_hook(&env, move |report| {
        let _ = std::fs::write(&hook_path, report.to_json());
    });

    // Pick up a report parked by a previous crash so it can be reviewed (stub
    // for v0.1.0; the review-and-send dialog lands in a later slice).
    if let Ok(json) = std::fs::read_to_string(&report_path) {
        if let Some(report) = crash_report::CrashReport::from_json(&json) {
            eprintln!(
                "[crash report parked from previous run]: {} v{} — {}",
                report.os, report.version, report.backtrace.as_deref().unwrap_or("unknown error")
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

    // Frontend selection: the GUI is the only frontend. An explicit --gui
    // always wins; otherwise the window opens when no terminal is attached,
    // so launching the installed app from a desktop environment works.
    #[cfg(feature = "gui")]
    if force_gui || !terminal_attached() {
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
        .map_err(|e: eframe::Error| -> Box<dyn Error> { format!("eframe failed: {e}").into() })?;
    } else {
        eprintln!("Dust Trail is GUI-only. Run `dust-trail --gui` to open the desktop window.");
        std::process::exit(2);
    }

    // No GUI in this build: there is no terminal fallback any more, so exit
    // with a clear message instead of starting nothing (spec AC-003).
    #[cfg(not(feature = "gui"))]
    {
        let _ = force_gui;
        eprintln!(
            "This build has no GUI feature and Dust Trail no longer has a \
             terminal frontend. Rebuild with the `gui` feature (the default) \
             to get the desktop window."
        );
        std::process::exit(2);
    }

    Ok(())
}