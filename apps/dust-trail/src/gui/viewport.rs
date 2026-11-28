//! Viewport: bridge between GUI and render module.
//! Holds the debug camera and performs per-frame camera resolution.

use crate::gui::camera::DebugCamera;
use crate::render::camera::Camera;
use egui::Context;

/// The application viewport — the single point where GUI input is translated
/// into render camera state and the frame is assembled.
pub struct Viewport {
    /// Debug camera for free-look / isometric toggling (Ctrl+Shift+C).
    debug_camera: DebugCamera,
    /// Fixed isometric camera used when debug camera is disabled.
    fixed_camera: Camera,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            debug_camera: DebugCamera::default_camera(),
            fixed_camera: Camera::fixed_isometric(),
        }
    }

    /// Call once per frame at the start of `Game::frame()`.
    /// Processes debug camera input and returns whether the camera is enabled.
    pub fn handle_input(&mut self, ctx: &Context) -> bool {
        self.debug_camera.handle_input(ctx)
    }

    /// Return the active camera to use for rendering this frame.
    ///
    /// If the debug camera is enabled, its project/view methods are used.
    /// Otherwise the fixed isometric camera is returned.
    pub fn active_camera(&self) -> Camera {
        if self.debug_camera.is_enabled() {
            // When debug camera is enabled, we return the fixed camera but
            // the caller (rasteriser) will use debug_camera.project() instead.
            self.fixed_camera
        } else {
            self.fixed_camera
        }
    }

    /// Return the debug camera for project/view operations when enabled.
    pub fn debug_camera(&self) -> &DebugCamera {
        &self.debug_camera
    }

    /// Return the debug camera for mutable project/view operations when enabled.
    pub fn debug_camera_mut(&mut self) -> &mut DebugCamera {
        &mut self.debug_camera
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}