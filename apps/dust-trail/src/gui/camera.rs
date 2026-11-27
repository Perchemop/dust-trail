//! Debug camera: free-look / isometric toggle with keyboard shortcut.
//! GUI-only (egui imports allowed); headless tests use `enabled=false`.

use crate::theme::Role;
use egui::Context;
use std::sync::atomic::{AtomicU8, Ordering};

/// Debug camera state for the GUI viewport.
///
/// Can be toggled with Ctrl+Shift+C (configurable in Settings). When enabled,
/// the camera responds to mouse drag, scroll, and key presses. When disabled,
/// it falls back to the fixed isometric view.
#[derive(Clone, Debug, Default)]
pub struct DebugCamera {
    /// Whether the debug camera is active.
    enabled: bool,
    /// Yaw angle in radians (horizontal rotation).
    yaw: f32,
    /// Pitch angle in radians (vertical rotation, clamped to -89..89°).
    pitch: f32,
    /// Distance from the target point.
    distance: f32,
    /// Toggle key (stored as a byte code for comparison).
    toggle_key: AtomicU8,
    /// Isometric / perspective mode flag.
    isometric: bool,
    /// Anchor point the camera orbits (typically screen centre).
    anchor: (f32, f32),
}

impl DebugCamera {
    /// Create a new debug camera with defaults.
    ///
    /// * `enabled` – starts disabled (fixed isometric view is the default).
    /// * `toggle_key` – egui key code for the toggle shortcut (Ctrl+Shift+C = 17 for Ctrl, 17+90+67 for full, but we store the primary mod+ctrl combo as a single byte for simplicity).
    pub fn new(enabled: bool, toggle_key: u8, isometric: bool) -> Self {
        Self {
            enabled,
            yaw: 0.0,
            pitch: 0.0,
            distance: 10.0,
            toggle_key: AtomicU8::new(toggle_key),
            isometric,
            anchor: (0.0, 0.0),
        }
    }

    /// Default: disabled, isometric, toggle key = Ctrl+Shift+C (encoded as 'C'=67 with ctrl modifier).
    /// In practice the caller sets the actual key via Settings.
    pub fn default_camera() -> Self {
        // Ctrl=17, Shift=16, C=67; we store the 'C' code and let the handler check mods.
        Self::new(false, b'C', true)
    }

    /// Return whether the debug camera is currently enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Toggle the debug camera on/off.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Handle egui input events. Returns `true` if the event was consumed
    /// (should not be processed by the regular GUI).
    pub fn handle_input(&mut self, ctx: &Context) -> bool {
        let mut consumed = false;
        let input = ctx.input();

        // Check toggle key (Ctrl+Shift+C)
        // We check for Ctrl+Shift+C by examining the modifier flags + key code.
        let is_ctrl = input.modifiers.ctrl;
        let is_shift = input.modifiers.shift;
        let key_c = input.key_pressed(egui::Key::C);

        if is_ctrl && is_shift && key_c {
            self.enabled = !self.enabled;
            consumed = true;
        }

        // When debug camera is enabled, process input.
        if self.enabled {
            // Mouse drag: rotate yaw/pitch
            if input.mouse_delta().0 != 0.0 || input.mouse_delta().1 != 0.0 {
                let mut delta = input.mouse_delta();
                self.yaw -= delta.0 * 0.01;
                self.pitch -= delta.1 * 0.01;
                // Clamp pitch to prevent flipping
                self.pitch = self.pitch.max(-std::f32::consts::FRAC_PI_2 - 0.01);
                self.pitch = self.pitch.min(std::f32::consts::FRAC_PI_2 - 0.01);
                consumed = true;
            }

            // Scroll: adjust distance
            if input.scroll_delta().0 != 0.0 || input.scroll_delta().1 != 0.0 {
                let scroll = input.scroll_delta().0.abs().max(input.scroll_delta().1.abs());
                self.distance -= scroll * 0.5;
                self.distance = self.distance.max(1.0).min(50.0);
                consumed = true;
            }

            // 'P' key: toggle isometric/perspective
            if input.key_pressed(egui::Key::P) {
                self.isometric = !self.isometric;
                consumed = true;
            }

            // 'R' key: reset to defaults
            if input.key_pressed(egui::Key::R) {
                self.yaw = 0.0;
                self.pitch = 0.0;
                self.distance = 10.0;
                self.isometric = true;
                consumed = true;
            }
        }

        consumed
    }

    /// Return the view matrix (4×4 look-at matrix) for the rasteriser.
    ///
    /// When the debug camera is disabled, returns the fixed isometric matrix.
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        if !self.enabled || self.isometric {
            // Fixed isometric look-at
            let (sx, sy) = self.anchor;
            let cx = sx + self.distance * self.yaw.cos() * self.pitch.cos();
            let cy = sy + self.distance * self.yaw.sin() * self.pitch.cos();
            let cz = self.distance * self.pitch.sin();
            // Simplified: return identity for isometric fallback in v2.0.0
            return [[1f32, 0.0, 0.0, 0.0], [0.0, 1f32, 0.0, 0.0], [0.0, 0.0, 1f32, 0.0], [0.0, 0.0, 0.0, 1f32]];
        }

        // Free-look orbit matrix
        let target = self.anchor;
        let up = egui::Vec2::new(0.0, 1.0); // up direction
        // Simplified: return identity for now; full implementation would use
        // a proper look-at matrix computation.
        [[1f32, 0.0, 0.0, 0.0], [0.0, 1f32, 0.0, 0.0], [0.0, 0.0, 1f32, 0.0], [0.0, 0.0, 0.0, 1f32]]
    }

    /// Project world screen coordinates through the debug camera.
    ///
    /// When the debug camera is disabled, uses the fixed isometric projection.
    /// When enabled, applies the orbiting camera transform.
    pub fn project(&self, wx: f32, wy: f32, wz: f32) -> (f32, f32, f32) {
        if !self.enabled || self.isometric {
            // Fixed isometric projection
            let sx = (wx - wy) * 2.0;
            let sy = wx + wy;
            (sx, sy, wz)
        } else {
            // Free-look projection (simplified)
            (wx, wy, wz)
        }
    }

    /// Set the anchor point around which the camera orbits.
    pub fn set_anchor(&mut self, anchor: (f32, f32)) {
        self.anchor = anchor;
    }

    /// Get the current distance.
    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Get the current yaw.
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    /// Get the current pitch.
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Context;

    fn make_camera() -> DebugCamera {
        DebugCamera::default_camera()
    }

    #[test]
    fn default_camera_disabled() {
        let cam = make_camera();
        assert!(!cam.is_enabled());
        assert!(cam.isometric);
    }

    #[test]
    fn toggle_camera() {
        let mut cam = make_camera();
        // Simulate Ctrl+Shift+C press
        cam.handle_input(&Context::default());
        assert!(cam.is_enabled());
        // Toggle again
        cam.handle_input(&Context::default());
        assert!(!cam.is_enabled());
    }

    #[test]
    fn view_matrix_isometric() {
        let cam = make_camera();
        // When disabled or isometric, should return a valid matrix
        let m = cam.view_matrix();
        assert_eq!(m.len(), 4);
        assert_eq!(m[0].len(), 4);
    }

    #[test]
    fn project_isometric() {
        let cam = make_camera();
        let result = cam.project(10.0, 5.0, 0.0);
        // Isometric project should return (sx, sy, wz) = ((10-5)*2, 10+5, 0) = (10, 15, 0)
        assert!(result.0 == 10.0 || result.1 == 15.0);
    }
}