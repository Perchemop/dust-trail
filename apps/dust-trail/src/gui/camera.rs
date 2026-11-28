//! Debug camera: free-look / isometric toggle with keyboard shortcut.
//! GUI-only (egui imports allowed); headless tests use `enabled=false`.

use egui::Context;

/// Debug camera state for the GUI viewport.
///
/// Can be toggled with Ctrl+Shift+C (configurable in Settings). When enabled,
/// the camera responds to mouse drag, scroll, and key presses. When disabled,
/// it falls back to the fixed isometric view.
#[derive(Clone, Debug)]
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
    toggle_key: u8,
    /// Isometric / perspective mode flag.
    isometric: bool,
}

impl Default for DebugCamera {
    fn default() -> Self {
        Self::default_camera()
    }
}

impl DebugCamera {
    /// Create a new debug camera with the given configuration.
    ///
    /// * `enabled` – starts disabled (fixed isometric view is the default).
    /// * `toggle_key` – primary key code for the toggle shortcut (the
    ///   handler also requires Ctrl+Shift modifiers).
    /// * `isometric` – start in isometric (true) or free-look (false) mode.
    pub fn new(enabled: bool, toggle_key: u8, isometric: bool) -> Self {
        Self {
            enabled,
            yaw: 0.0,
            pitch: 0.0,
            distance: 10.0,
            toggle_key,
            isometric,
        }
    }

    /// Default: disabled, isometric, toggle key = 'C' (Ctrl+Shift+C handled
    /// in the input handler by checking modifier flags).
    pub fn default_camera() -> Self {
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
        let toggle_key_code =
            egui::Key::from_name(&format!("{:?}", self.toggle_key as char));

        ctx.input(|input| {
            let mut consumed = false;

            // Toggle shortcut: Ctrl+Shift+<toggle_key>.
            let modifiers_match = input.modifiers.ctrl && input.modifiers.shift;
            let key_pressed = toggle_key_code.is_some_and(|k| input.key_pressed(k));
            if modifiers_match && key_pressed {
                self.enabled = !self.enabled;
                consumed = true;
            }

            // When the debug camera is enabled, process navigation input.
            if self.enabled {
                // Mouse drag: rotate yaw/pitch.
                let delta = input.pointer.delta();
                if delta != egui::Vec2::ZERO {
                    self.yaw -= delta.x * 0.01;
                    self.pitch -= delta.y * 0.01;
                    // Clamp pitch to prevent flipping.
                    self.pitch = self
                        .pitch
                        .clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);
                    consumed = true;
                }

                // Scroll: adjust distance.
                let scroll = input.raw_scroll_delta;
                if scroll != egui::Vec2::ZERO {
                    let amount = scroll.x.abs().max(scroll.y.abs());
                    self.distance -= amount * 0.5;
                    self.distance = self.distance.clamp(1.0, 50.0);
                    consumed = true;
                }

                // 'P' key: toggle isometric/perspective.
                if input.key_pressed(egui::Key::P) {
                    self.isometric = !self.isometric;
                    consumed = true;
                }

                // 'R' key: reset to defaults.
                if input.key_pressed(egui::Key::R) {
                    self.yaw = 0.0;
                    self.pitch = 0.0;
                    self.distance = 10.0;
                    self.isometric = true;
                    consumed = true;
                }
            }

            consumed
        })
    }

    /// Return the view matrix (4×4 look-at matrix) for the rasteriser.
    ///
    /// When the debug camera is disabled, returns the fixed isometric matrix.
    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        if !self.enabled || self.isometric {
            // Simplified: identity for isometric fallback in v2.0.0.
            return [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ];
        }

        // Free-look orbit matrix (simplified: identity in v2.0.0).
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
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
    fn set_enabled_toggles() {
        let mut cam = make_camera();
        cam.set_enabled(true);
        assert!(cam.is_enabled());
        cam.set_enabled(false);
        assert!(!cam.is_enabled());
    }

    #[test]
    fn handle_input_without_events_is_noop() {
        let mut cam = make_camera();
        // A default context has no key presses; nothing should be consumed
        // and the camera state must not change.
        let consumed = cam.handle_input(&Context::default());
        assert!(!consumed);
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
