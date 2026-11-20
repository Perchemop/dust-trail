//! Depth-aware compositing: sprite → Frame with z-compare.

use crate::domain::frame::Frame;
use crate::render::sprite::Sprite;
use crate::domain::Cell;
use crate::render::model::ModelKey;

/// Composite a sprite into a frame with depth-aware occlusion.
/// Draws back-to-front: caller must ensure sprites are passed in far-to-near order.
/// Uses Frame::set_at_depth for per-cell z-comparison.
pub fn composite_sprite(frame: &mut Frame, sprite: &Sprite, screen_x: i32, screen_y: i32) {
    if sprite.is_empty() {
        return;
    }

    let sw = sprite.width as i32;
    let sh = sprite.height as i32;

    // Sprite anchor is bottom-center of the sprite's footprint
    // Screen position is where the anchor should be placed
    let start_x = screen_x - sprite.anchor.0;
    let start_y = screen_y - sprite.anchor.1;

    for (sy, row) in sprite.cells.iter().enumerate() {
        for (sx, (ch, fg, bg)) in row.iter().enumerate() {
            let fx = start_x + sx as i32;
            let fy = start_y + (sh - 1 - sy as i32); // flip Y: sprite origin is bottom

            // Only write if in bounds
            if fx >= 0 && fy >= 0 {
                let ux = fx as usize;
                let uy = fy as usize;
                if ux < frame.width && uy < frame.height {
                    let depth = sprite.depth[sy][sx];
                    let cell = Cell::new(*ch, *fg, *bg);
                    frame.set_at_depth(ux, uy, cell, depth);
                }
            }
        }
    }
}

/// Composite multiple sprites in back-to-front order (far first, near last).
/// Sprites should be sorted by screen depth (far = small, near = large).
pub fn composite_sprites_back_to_front(frame: &mut Frame, sprites: &[(Sprite, i32, i32)]) {
    // Sort by depth: far first (smaller screen_depth), near last
    let mut sorted: Vec<_> = sprites.iter().collect();
    sorted.sort_by(|a, b| {
        // Use the sprite's average depth for ordering
        let avg_depth_a = a.0.depth.iter()
            .flatten()
            .copied()
            .fold(0.0f32, |acc, d| acc + d) / (a.0.width * a.0.height) as f32;
        let avg_depth_b = b.0.depth.iter()
            .flatten()
            .copied()
            .fold(0.0f32, |acc, d| acc + d) / (b.0.width * b.0.height) as f32;
        avg_depth_a.partial_cmp(&avg_depth_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    for (sprite, sx, sy) in sorted {
        composite_sprite(frame, sprite, *sx, *sy);
    }
}

/// Placeholder sprite for unknown models.
pub fn placeholder_sprite(key: ModelKey) -> Sprite {
    Sprite::placeholder(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::frame::Frame;
    use crate::domain::Cell;
    use crate::domain::world::Rgb;
    use crate::render::sprite::Sprite;

    #[test]
    fn composite_sprite_overwrites_nearer_depth() {
        let mut frame = Frame::new(10, 10);
        frame.clear_depth();

        // Far sprite at depth 10.0
        let mut far_sprite = Sprite::placeholder("far");
        far_sprite.cells = vec![vec![('A', Rgb(255, 0, 0), Rgb(0, 0, 0))]];
        far_sprite.depth = vec![vec![10.0]];
        far_sprite.width = 1;
        far_sprite.height = 1;
        far_sprite.anchor = (0, 0);

        // Near sprite at depth 5.0
        let mut near_sprite = Sprite::placeholder("near");
        near_sprite.cells = vec![vec![('B', Rgb(0, 255, 0), Rgb(0, 0, 0))]];
        near_sprite.depth = vec![vec![5.0]];
        near_sprite.width = 1;
        near_sprite.height = 1;
        near_sprite.anchor = (0, 0);

        // Draw far first, then near (back-to-front)
        composite_sprite(&mut frame, &far_sprite, 5, 5);
        composite_sprite(&mut frame, &near_sprite, 5, 5);

        // Near should win (depth 5.0 < 10.0)
        let cell = frame.cell(5, 5).unwrap();
        assert_eq!(cell.ch, 'B');

        // Farther sprite at same position should not overwrite
        composite_sprite(&mut frame, &far_sprite, 5, 5);
        let cell = frame.cell(5, 5).unwrap();
        assert_eq!(cell.ch, 'B'); // still near
    }

    #[test]
    fn composite_sprite_farther_depth_skipped() {
        let mut frame = Frame::new(10, 10);
        frame.clear_depth();

        // Near sprite first
        let mut near_sprite = Sprite::placeholder("near");
        near_sprite.cells = vec![vec![('A', Rgb(255, 0, 0), Rgb(0, 0, 0))]];
        near_sprite.depth = vec![vec![5.0]];
        near_sprite.width = 1;
        near_sprite.height = 1;
        near_sprite.anchor = (0, 0);

        // Farther sprite later
        let mut far_sprite = Sprite::placeholder("far");
        far_sprite.cells = vec![vec![('B', Rgb(0, 255, 0), Rgb(0, 0, 0))]];
        far_sprite.depth = vec![vec![10.0]];
        far_sprite.width = 1;
        far_sprite.height = 1;
        far_sprite.anchor = (0, 0);

        composite_sprite(&mut frame, &near_sprite, 5, 5);
        composite_sprite(&mut frame, &far_sprite, 5, 5);

        // Near should still be there
        let cell = frame.cell(5, 5).unwrap();
        assert_eq!(cell.ch, 'A');
    }

    #[test]
    fn composite_empty_sprite_no_op() {
        let mut frame = Frame::new(10, 10);
        frame.clear_depth();
        let empty = Sprite::empty("empty");
        composite_sprite(&mut frame, &empty, 5, 5);
        // No change
        assert!(frame.cell(5, 5).is_none() || frame.cell(5, 5).unwrap().ch == ' ');
    }

    #[test]
    fn composite_out_of_bounds_no_panic() {
        let mut frame = Frame::new(10, 10);
        frame.clear_depth();
        let mut sprite = Sprite::placeholder("test");
        sprite.width = 1;
        sprite.height = 1;
        sprite.anchor = (0, 0);
        // Write at negative position
        composite_sprite(&mut frame, &sprite, -5, -5);
        // Write beyond bounds
        composite_sprite(&mut frame, &sprite, 100, 100);
        // Should not panic
    }

    #[test]
    fn composite_clear_depth_resets() {
        let mut frame = Frame::new(10, 10);
        frame.clear_depth();
        assert_eq!(frame.depth_at(5, 5), Some(f32::INFINITY));

        let mut sprite = Sprite::placeholder("test");
        sprite.width = 1;
        sprite.height = 1;
        sprite.anchor = (0, 0);
        composite_sprite(&mut frame, &sprite, 5, 5);
        assert!(frame.depth_at(5, 5).unwrap() < f32::INFINITY);

        frame.clear_depth();
        assert_eq!(frame.depth_at(5, 5), Some(f32::INFINITY));
    }

    #[test]
    fn composite_sprites_back_to_front_ordering() {
        let mut frame = Frame::new(20, 20);
        frame.clear_depth();

        let mut near = Sprite::placeholder("near");
        near.cells = vec![vec![('N', Rgb(255, 255, 255), Rgb(0, 0, 0))]];
        near.depth = vec![vec![2.0]];
        near.width = 1;
        near.height = 1;
        near.anchor = (0, 0);

        let mut far = Sprite::placeholder("far");
        far.cells = vec![vec![('F', Rgb(255, 255, 255), Rgb(0, 0, 0))]];
        far.depth = vec![vec![10.0]];
        far.width = 1;
        far.height = 1;
        far.anchor = (0, 0);

        // Pass in wrong order (near first) — function should sort
        composite_sprites_back_to_front(&mut frame, &[(near, 10, 10), (far, 10, 10)]);

        // Far should be drawn first, then near on top
        let cell = frame.cell(10, 10).unwrap();
        assert_eq!(cell.ch, 'N');
    }
}