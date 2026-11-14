//! Isometric screen-space conversion for ASCII rendering.
//!
//! World coordinates (wx, wy) map to screen coordinates (sx, sy):
//!   sx = (wx - wy) * 2
//!   sy = (wx + wy)
//!
//! Moving +x (east) goes right 2 and down 1; moving +y (south) goes left 2
//! and down 1. This produces a dimetric diamond grid — the classic isometric
//! ASCII look — without any commercial art assets.

/// Convert a world tile position to its top-left screen position.
pub fn world_to_screen(wx: i32, wy: i32) -> (i32, i32) {
    ((wx - wy) * 2, wx + wy)
}

/// The screen "depth" of a tile, used for painter's-algorithm draw order.
/// Larger (wx + wy) is lower/front on screen, so draw ascending for
/// back-to-front (far tiles first, near tiles overlap them).
pub fn screen_depth(wx: i32, wy: i32) -> i32 {
    wx + wy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn east_moves_right_and_down() {
        let (sx0, sy0) = world_to_screen(0, 0);
        let (sx1, sy1) = world_to_screen(1, 0);
        assert_eq!(sx1, sx0 + 2);
        assert_eq!(sy1, sy0 + 1);
    }

    #[test]
    fn south_moves_left_and_down() {
        let (sx0, sy0) = world_to_screen(0, 0);
        let (sx1, sy1) = world_to_screen(0, 1);
        assert_eq!(sx1, sx0 - 2);
        assert_eq!(sy1, sy0 + 1);
    }

    #[test]
    fn draw_order_is_back_to_front() {
        // Back tiles have smaller screen depth.
        assert!(screen_depth(0, 0) < screen_depth(1, 1));
    }
}
