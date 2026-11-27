//! Settings panel for Dust Trail GUI (T-018).
//!
//! Provides sound, theme, and accessibility options. Settings persist to the
//! config directory and apply immediately.

use serde::{Deserialize, Serialize};

use crate::theme::{Theme, Variant};

/// Theme mode preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

/// Target size for UI elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TargetSize {
    Compact,
    #[default]
    Comfortable,
}

/// Application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Sound
    pub sound_enabled: bool,
    pub volume: f32,
    // Theme
    pub theme_mode: ThemeMode,
    pub font_size: f32,
    // Accessibility
    pub reduced_motion: bool,
    pub high_contrast: bool,
    pub target_size: TargetSize,
    // Debug camera
    pub debug_camera: DebugCameraState,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            volume: 0.5,
            theme_mode: ThemeMode::System,
            font_size: 13.0,
            reduced_motion: false,
            high_contrast: false,
            target_size: TargetSize::Comfortable,
        }
    }
}

/// Resolve the settings config path.
fn settings_path() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "dust-trail")
        .map(|p| p.config_dir().join("settings.json"))
        .unwrap_or_else(|| std::env::temp_dir().join("dust-trail-settings.json"))
}

/// Load settings from config file, falling back to defaults.
pub fn load_settings() -> Settings {
    let path = settings_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}

/// Save settings to config file.
pub fn save_settings(settings: &Settings) {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(data) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(&path, data);
    }
}

/// Resolve the effective theme variant from settings.
pub fn effective_variant(settings: &Settings) -> Variant {
    if settings.high_contrast {
        return Variant::HighContrast;
    }
    match settings.theme_mode {
        ThemeMode::Light => Variant::Light,
        ThemeMode::Dark => Variant::Dark,
        ThemeMode::System => {
            // Default to dark for simplicity; a real implementation would
            // query the OS preference.
            Variant::Dark
        }
    }
}

/// Apply settings to the egui context (theme visuals).
pub fn apply_settings(ctx: &egui::Context, settings: &Settings) {
    let variant = effective_variant(settings);
    let egui_theme = match variant {
        Variant::Light => egui::Theme::Light,
        Variant::Dark => egui::Theme::Dark,
        Variant::HighContrast => egui::Theme::Dark,
    };
    let theme = match variant {
        Variant::Light => Theme::light(),
        Variant::Dark => Theme::dark(),
        Variant::HighContrast => Theme::high_contrast(),
    };
    ctx.set_visuals(theme.visuals());
    // Font size scaling: egui uses the style's text_styles. We set a global
    // override by adjusting the base font size via style.
    // In egui 0.37+, ctx.set_style() is removed; use set_style_of().
    let mut style = egui::Style {
        visuals: theme.visuals(),
        ..Default::default()
    };
    for font_id in style.text_styles.values_mut() {
        font_id.size = settings.font_size;
    }
    ctx.set_style_of(egui_theme, style);
}

/// State for the settings panel window.
pub struct SettingsPanelState {
    pub open: bool,
    pub settings: Settings,
    pub dirty: bool,
}

impl Default for SettingsPanelState {
    fn default() -> Self {
        Self {
            open: false,
            settings: load_settings(),
            dirty: false,
        }
    }
}

impl SettingsPanelState {
    pub fn open(&mut self) {
        self.settings = load_settings();
        self.open = true;
        self.dirty = false;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// Apply and persist if dirty.
    pub fn commit(&mut self, ctx: &egui::Context) {
        if self.dirty {
            apply_settings(ctx, &self.settings);
            save_settings(&self.settings);
            self.dirty = false;
        }
    }
}

/// Draw the settings panel window. Returns true if settings changed.
pub fn draw_settings_panel(
    state: &mut SettingsPanelState,
    ctx: &egui::Context,
) -> bool {
    if !state.open {
        return false;
    }

    let mut changed = false;
    let mut window_open = state.open;

    egui::Window::new("Settings")
        .open(&mut window_open)
        .resizable(true)
        .default_width(420.0)
        .show(ctx, |ui| {
            ui.heading("Sound");
            ui.separator();
            if ui
                .checkbox(&mut state.settings.sound_enabled, "Enable sound")
                .changed()
            {
                changed = true;
                state.dirty = true;
            }
            let mut vol = state.settings.volume;
            if ui
                .add(egui::Slider::new(&mut vol, 0.0..=1.0).text("Volume"))
                .changed()
            {
                state.settings.volume = vol;
                changed = true;
                state.dirty = true;
            }

            ui.add_space(12.0);
            ui.heading("Theme");
            ui.separator();
            let mut theme_idx = match state.settings.theme_mode {
                ThemeMode::Light => 0,
                ThemeMode::Dark => 1,
                ThemeMode::System => 2,
            };
            let r0 = ui.radio_value(&mut theme_idx, 0, "Light");
            let r1 = ui.radio_value(&mut theme_idx, 1, "Dark");
            let r2 = ui.radio_value(&mut theme_idx, 2, "System");
            if r0.changed() || r1.changed() || r2.changed() {
                state.settings.theme_mode = match theme_idx {
                    0 => ThemeMode::Light,
                    1 => ThemeMode::Dark,
                    _ => ThemeMode::System,
                };
                changed = true;
                state.dirty = true;
            }
            let mut fs = state.settings.font_size;
            if ui
                .add(egui::Slider::new(&mut fs, 10.0..=24.0).text("Font size"))
                .changed()
            {
                state.settings.font_size = fs;
                changed = true;
                state.dirty = true;
            }

            ui.add_space(12.0);
            ui.heading("Accessibility");
            ui.separator();
            if ui
                .checkbox(&mut state.settings.reduced_motion, "Reduced motion")
                .changed()
            {
                changed = true;
                state.dirty = true;
            }
            if ui
                .checkbox(&mut state.settings.high_contrast, "High contrast")
                .changed()
            {
                changed = true;
                state.dirty = true;
            }
            let mut ts_idx = match state.settings.target_size {
                TargetSize::Compact => 0,
                TargetSize::Comfortable => 1,
            };
            let t0 = ui.radio_value(&mut ts_idx, 0, "Compact");
            let t1 = ui.radio_value(&mut ts_idx, 1, "Comfortable");
            if t0.changed() || t1.changed() {
                state.settings.target_size = if ts_idx == 0 {
                    TargetSize::Compact
                } else {
                    TargetSize::Comfortable
                };
                changed = true;
                state.dirty = true;
            }

            ui.add_space(12.0);
            if ui.button("Apply").clicked() {
                state.dirty = true;
                changed = true;
            }
            if ui.button("Close").clicked() {
                state.open = false;
                // window_open will be updated from state.open on next frame
            }
        });

    state.open = window_open;
    if changed {
        state.commit(ctx);
    }
    changed
}

/// State for the debug camera, persisted in GUI settings.
/// Serializes: enabled, yaw, pitch, distance, projection, toggle_key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DebugCameraState {
    /// Whether the debug camera is enabled.
    pub enabled: bool,
    /// Yaw angle in radians.
    pub yaw: f32,
    /// Pitch angle in radians.
    pub pitch: f32,
    /// Distance from the target point.
    pub distance: f32,
    /// Projection mode: true = perspective, false = isometric.
    pub isometric: bool,
    /// Toggle key (egui key code).
    pub toggle_key: u8,
}
    changed
}
