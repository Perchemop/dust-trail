//! Fixed isometric camera: projection maths with height term.

/// Fixed isometric camera with no rotation or zoom in v2.0.0.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub scale: f32,            // fixed isometric scale
    pub anchor: (i32, i32),    // world position mapped to screen origin
}

impl Camera {
    /// Create the fixed isometric camera used in v2.0.0.
    pub fn fixed_isometric() -> Self {
        // Scale matches the existing world_to_screen maths:
        // sx = (wx - wy) * 2, sy = (wx + wy) * 1
        // scale of 2.0 gives the classic isometric ratio
        Self {
            scale: 2.0,
            anchor: (0, 0),
        }
    }

    /// World (wx, wy, wz) → screen (sx, sy, sz).
    /// Fixed projection; no rotation/zoom.
    /// Reproduces existing world_to_screen for wz=0, adds height term for wz>0.
    pub fn project(&self, wx: i32, wy: i32, wz: i32) -> (i32, i32, f32) {
        let sx = ((wx - wy) as f32 * self.scale) as i32 + self.anchor.0;
        let sy = ((wx + wy) as f32 * self.scale * 0.5) as i32 + self.anchor.1;
        // Height term: each unit of wz shifts screen y upward by scale
        let sz = wz as f32 * self.scale;
        let sy_with_height = sy - sz as i32;
        (sx, sy_with_height, sz)
    }

    /// Screen depth for draw ordering (far = small, near = large).
    /// Used for back-to-front sorting.
    pub fn screen_depth(&self, wx: i32, wy: i32, wz: i32) -> f32 {
        // Base depth from world position + height term
        // wx + wy gives depth in isometric; wz adds height
        (wx + wy) as f32 + wz as f32 * self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_reproduces_world_to_screen_for_wz_zero() {
        let cam = Camera::fixed_isometric();
        // At wz=0, should match classic isometric: sx=(wx-wy)*2, sy=(wx+wy)*1
        let (sx, sy, sz) = cam.project(0, 0, 0);
        assert_eq!(sx, 0);
        assert_eq!(sy, 0);
        assert_eq!(sz, 0.0);

        let (sx, sy, sz) = cam.project(1, 0, 0);
        assert_eq!(sx, 2); // (1-0)*2
        assert_eq!(sy, 1); // (1+0)*1
        assert_eq!(sz, 0.0);

        let (sx, sy, sz) = cam.project(0, 1, 0);
        assert_eq!(sx, -2); // (0-1)*2
        assert_eq!(sy, 1);  // (0+1)*1
        assert_eq!(sz, 0.0);
    }

    #[test]
    fn camera_adds_height_term_for_wz_positive() {
        let cam = Camera::fixed_isometric();
        let (sx, sy, sz) = cam.project(0, 0, 1);
        assert_eq!(sx, 0);
        assert_eq!(sz, 2.0); // wz * scale
        assert_eq!(sy, -2);  // sy shifted up by height
    }

    #[test]
    fn camera_fixed_scale() {
        let cam = Camera::fixed_isometric();
        assert_eq!(cam.scale, 2.0);
        // No public method to rotate, orbit, or zoom
    }

    #[test]
    fn camera_screen_depth_monotonic() {
        let cam = Camera::fixed_isometric();
        let d1 = cam.screen_depth(0, 0, 0);
        let d2 = cam.screen_depth(1, 1, 0);
        let d3 = cam.screen_depth(0, 0, 1);
        // d2 (farther in world) > d1 (origin) > d3 (height adds depth but less than world distance?)
        // Actually screen_depth = wx + wy + wz*scale
        assert!(d2 > d1);
        assert!(d3 > d1);
    }
}