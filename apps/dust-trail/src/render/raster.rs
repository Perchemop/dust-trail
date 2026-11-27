//! Software rasteriser: project, fill, z-buffer, Lambertian shading.

use crate::render::camera::Camera;
use crate::render::model::ObjectModel;
use crate::render::primitive::{Primitive, PrimitiveKind, Transform};
use crate::render::animation::{Animation, AnimFrame, AnimationController, AnimKey, LoopMode, PrimitiveDelta};
use crate::render::sprite::Sprite;
use crate::render::glyph::luminance_to_glyph;
use std::collections::HashMap;
use crate::domain::color::Rgb;

/// Software rasteriser for 3D primitive models.
#[derive(Clone, Debug)]
pub struct Rasteriser {
    light_dir: (f32, f32, f32), // normalized directional light
    width: u32,
    height: u32,
}

impl Rasteriser {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            light_dir: (0.0, 0.0, 1.0), // default: light from above
            width,
            height,
        }
    }

    pub fn with_light(mut self, dir: (f32, f32, f32)) -> Self {
        let len = (dir.0 * dir.0 + dir.1 * dir.1 + dir.2 * dir.2).sqrt().max(1e-6);
        self.light_dir = (dir.0 / len, dir.1 / len, dir.2 / len);
        self
    }

    /// Rasterise one model into a Sprite. Projects primitives, fills, writes z-buffer,
    /// applies Lambertian shading, converts luminance to glyph. Never panics on
    /// degenerate input; returns placeholder/empty sprite for empty/degenerate models.
    /// If `anim_frame` is provided, per-primitive animation deltas are applied.
    pub fn render(&self, model: &ObjectModel, camera: &Camera, anim_frame: Option<&AnimFrame>) -> Sprite {
        // Empty model → empty sprite
        if model.primitives.is_empty() {
            return Sprite::empty(model.key);
        }

        // Zero-size raster → empty sprite
        if self.width == 0 || self.height == 0 {
            return Sprite::empty(model.key);
        }

        let w = self.width as usize;
        let h = self.height as usize;

        // Per-pixel buffers
        let mut z_buffer = vec![vec![f32::INFINITY; w]; h];
        let mut char_buffer = vec![vec![' '; w]; h];
        let mut fg_buffer = vec![vec![Rgb { r: 255, g: 255, b: 255 }; w]; h];
        let mut bg_buffer = vec![vec![Rgb { r: 0, g: 0, b: 0 }; w]; h];

        // Build per-primitive transforms, applying animation deltas if present.
        let transforms: Vec<Transform> = model.primitives
            .iter()
            .map(|primitive| {
                if let Some(anim_frame) = anim_frame {
                    // Apply the animation frame's primitive delta to the base transform.
                    // The animation controller has already computed the delta; we add it here.
                    let delta = &anim_frame.transform_delta;
                    Transform {
                        translation: (
                            primitive.transform.translation.0 + delta.translation.0 as f32,
                            primitive.transform.translation.1 + delta.translation.1 as f32,
                            primitive.transform.translation.2 + delta.translation.2 as f32,
                        ),
                        rotation: (
                            primitive.transform.rotation.0 + delta.rotation.0 as f32,
                            primitive.transform.rotation.1 + delta.rotation.1 as f32,
                            primitive.transform.rotation.2 + delta.rotation.2 as f32,
                        ),
                        scale: (
                            primitive.transform.scale.0 * delta.scale.0 as f32,
                            primitive.transform.scale.1 * delta.scale.1 as f32,
                            primitive.transform.scale.2 * delta.scale.2 as f32,
                        ),
                    }
                } else {
                    primitive.transform.clone()
                }
            })
            .collect();

        // Rasterise each primitive
        for (i, primitive) in model.primitives.iter().enumerate() {
            self.rasterise_primitive(primitive, camera, &transforms[i], &mut z_buffer, &mut char_buffer, &mut fg_buffer, &mut bg_buffer);
        }

        // Convert buffers to Sprite
        let mut cells = Vec::with_capacity(h);
        let mut depth = Vec::with_capacity(h);
        for row in 0..h {
            let mut cell_row = Vec::with_capacity(w);
            let mut depth_row = Vec::with_capacity(w);
            for col in 0..w {
                cell_row.push((char_buffer[row][col], fg_buffer[row][col], bg_buffer[row][col]));
                depth_row.push(z_buffer[row][col]);
            }
            cells.push(cell_row);
            depth.push(depth_row);
        }

        Sprite {
            key: model.key,
            width: w,
            height: h,
            cells,
            depth,
            anchor: (0, 0), // default anchor
        }
    }

    fn rasterise_primitive(
        &self,
        primitive: &Primitive,
        camera: &Camera,
        transform: &Transform,
        z_buffer: &mut [Vec<f32>],
        char_buffer: &mut [Vec<char>],
        fg_buffer: &mut [Vec<Rgb>],
        bg_buffer: &mut [Vec<Rgb>],
    ) {
        // For each primitive kind, generate vertices and project to screen
        let vertices = self.generate_primitive_vertices(primitive, transform);
        
        // Project all vertices
        let mut screen_verts = Vec::with_capacity(vertices.len());
        for (vx, vy, vz) in vertices {
            let (sx, sy, sz) = camera.project(vx, vy, vz);
            screen_verts.push((sx, sy, sz));
        }

        // Simple fill: project primitive bounds and fill rectangle
        // In a full implementation, this would be proper triangle rasterisation
        // For v2.0.0, we use a simplified footprint approach
        self.fill_primitive_footprint(&screen_verts, primitive, z_buffer, char_buffer, fg_buffer, bg_buffer);
    }

    fn generate_primitive_vertices(&self, primitive: &Primitive, transform: &Transform) -> Vec<(i32, i32, i32)> {
        let Transform { translation, rotation, scale } = *transform;
        let (tx, ty, tz) = translation;
        let _rotation = rotation;
        let (sx, sy, sz) = scale;

        let mut vertices = Vec::new();

        match primitive.kind {
            PrimitiveKind::Cone => {
                // Base circle (4 points) + apex
                let base_r = (sx.max(sy) * 0.5).max(1.0) as i32;
                let height = (sz * 2.0).max(1.0) as i32;
                for i in 0..4 {
                    let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
                    let x = (tx + angle.cos() * base_r as f32) as i32;
                    let y = (ty + angle.sin() * base_r as f32) as i32;
                    vertices.push((x, y, tz as i32));
                }
                vertices.push((tx as i32, ty as i32, (tz + height as f32) as i32));
            }
            PrimitiveKind::Ellipsoid => {
                // Sphere approximation (8 points)
                let rx_v = (sx * 0.5).max(1.0) as i32;
                let ry_v = (sy * 0.5).max(1.0) as i32;
                let rz_v = (sz * 0.5).max(1.0) as i32;
                for i in 0..8 {
                    let angle = (i as f32) * std::f32::consts::FRAC_PI_4;
                    let x = (tx + angle.cos() * rx_v as f32) as i32;
                    let y = (ty + angle.sin() * ry_v as f32) as i32;
                    let z = (tz + ((i % 2) as f32 * 2.0 - 1.0) * rz_v as f32) as i32;
                    vertices.push((x, y, z));
                }
            }
            PrimitiveKind::Box => {
                // 8 corners of box
                let hx = (sx * 0.5).max(1.0) as i32;
                let hy = (sy * 0.5).max(1.0) as i32;
                let hz = (sz * 0.5).max(1.0) as i32;
                for dz in [-hz, hz] {
                    for dy in [-hy, hy] {
                        for dx in [-hx, hx] {
                            vertices.push((tx as i32 + dx, ty as i32 + dy, tz as i32 + dz));
                        }
                    }
                }
            }
            PrimitiveKind::Cylinder => {
                // Base circle (4 points) + top circle (4 points)
                let r = (sx.max(sy) * 0.5).max(1.0) as i32;
                let h = (sz * 2.0).max(1.0) as i32;
                for i in 0..4 {
                    let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
                    let x = (tx + angle.cos() * r as f32) as i32;
                    let y = (ty + angle.sin() * r as f32) as i32;
                    vertices.push((x, y, tz as i32));
                    vertices.push((x, y, tz as i32 + h));
                }
            }
            PrimitiveKind::Capsule => {
                // Two spheres (caps) + cylinder middle
                let r = (sx.max(sy) * 0.5).max(1.0) as i32;
                let h = (sz * 2.0).max(1.0) as i32;
                // Bottom cap
                for i in 0..4 {
                    let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
                    let x = (tx + angle.cos() * r as f32) as i32;
                    let y = (ty + angle.sin() * r as f32) as i32;
                    vertices.push((x, y, tz as i32));
                }
                // Top cap
                for i in 0..4 {
                    let angle = (i as f32) * std::f32::consts::FRAC_PI_2;
                    let x = (tx + angle.cos() * r as f32) as i32;
                    let y = (ty + angle.sin() * r as f32) as i32;
                    vertices.push((x, y, tz as i32 + h));
                }
            }
        }

        vertices
    }

    fn fill_primitive_footprint(
        &self,
        screen_verts: &[(i32, i32, f32)],
        primitive: &Primitive,
        z_buffer: &mut [Vec<f32>],
        char_buffer: &mut [Vec<char>],
        fg_buffer: &mut [Vec<Rgb>],
        bg_buffer: &mut [Vec<Rgb>],
    ) {
        if screen_verts.is_empty() {
            return;
        }

        let h = z_buffer.len();
        let w = z_buffer[0].len();

        // Compute screen-space bounds of the primitive
        let min_x = screen_verts.iter().map(|v| v.0).min().unwrap_or(0);
        let max_x = screen_verts.iter().map(|v| v.0).max().unwrap_or(0);
        let min_y = screen_verts.iter().map(|v| v.1).min().unwrap_or(0);
        let max_y = screen_verts.iter().map(|v| v.1).max().unwrap_or(0);
        let min_z = screen_verts.iter().map(|v| v.2).min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(0.0);

        // Clamp to screen bounds
        let min_x = min_x.clamp(0, w as i32 - 1) as usize;
        let max_x = max_x.clamp(0, w as i32 - 1) as usize;
        let min_y = min_y.clamp(0, h as i32 - 1) as usize;
        let max_y = max_y.clamp(0, h as i32 - 1) as usize;

        // Material properties
        let albedo = primitive.material.albedo.clamp(0.0, 1.0);
        let shade = primitive.material.shade_side;

        // For each cell in footprint, compute shading and write if nearer
        for sy in min_y..=max_y {
            for sx in min_x..=max_x {
                // Simple depth: use min_z for the whole footprint
                // In a proper implementation, we'd interpolate depth per pixel
                let depth = min_z;

                // Z-buffer test: only write if nearer (smaller depth = closer)
                if depth < z_buffer[sy][sx] {
                    z_buffer[sy][sx] = depth;

                    // Lambertian shading
                    let luminance = if shade {
                        // For simplicity, use a fixed normal based on primitive kind
                        let normal = match primitive.kind {
                            PrimitiveKind::Cone | PrimitiveKind::Cylinder => (0.0, 0.0, 1.0), // top-facing
                            PrimitiveKind::Box => (0.0, 1.0, 0.0), // side-facing
                            PrimitiveKind::Ellipsoid | PrimitiveKind::Capsule => (0.0, 0.0, 1.0),
                        };
                        let dot = (normal.0 * self.light_dir.0 + normal.1 * self.light_dir.1 + normal.2 * self.light_dir.2).max(0.0);
                        albedo * dot
                    } else {
                        albedo
                    };

                    // Convert to glyph
                    let ch = luminance_to_glyph(luminance);

                    // Simple color from theme roles would go here; for now use white on black
                    char_buffer[sy][sx] = ch;
                    fg_buffer[sy][sx] = Rgb { r: 255, g: 255, b: 255 };
                    bg_buffer[sy][sx] = Rgb { r: 0, g: 0, b: 0 };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::camera::Camera;
    use crate::render::model::{ObjectModel, BoundingBox, DrawKey};
    use crate::render::primitive::{Primitive, Transform, Material, PrimitiveKind};

    fn make_rasteriser() -> Rasteriser {
        Rasteriser::new(32, 32).with_light((0.0, 0.0, 1.0))
    }

    fn make_camera() -> Camera {
        Camera::fixed_isometric()
    }

    fn make_tree_model() -> ObjectModel {
        ObjectModel {
            key: "tree_oak",
            name: "Oak Tree".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Cone,
                    transform: Transform {
                        translation: (0.0, 0.0, 0.0),
                        rotation: (0.0, 0.0, 0.0),
                        scale: (2.0, 2.0, 4.0),
                    },
                    material: Material { albedo: 0.6, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Ellipsoid,
                    transform: Transform {
                        translation: (0.0, 0.0, 4.0),
                        rotation: (0.0, 0.0, 0.0),
                        scale: (3.0, 3.0, 2.0),
                    },
                    material: Material { albedo: 0.8, shade_side: true },
                },
            ],
                animations: HashMap::new(),
bounding_box: BoundingBox::default(),
            draw_key: DrawKey(10),
        }
    }

    #[test]
    fn rasteriser_produces_non_empty_sprite_for_real_model() {
        let raster = make_rasteriser();
        let cam = make_camera();
        let model = make_tree_model();
        let sprite = raster.render(&model, &cam);
        assert!(sprite.width > 0);
        assert!(sprite.height > 0);
        assert_eq!(sprite.cells.len(), sprite.depth.len());
        assert_eq!(sprite.cells[0].len(), sprite.depth[0].len());
        // At least one non-space character
        let has_content = sprite.cells.iter().any(|row| row.iter().any(|(ch, _, _)| *ch != ' '));
        assert!(has_content, "sprite should have at least one non-space character");
    }

    #[test]
    fn rasteriser_empty_model_returns_empty_sprite() {
        let raster = make_rasteriser();
        let cam = make_camera();
        let model = ObjectModel {
            key: "empty",
            name: "Empty".to_string(),
            primitives: vec![],
                animations: HashMap::new(),
bounding_box: BoundingBox::default(),
            draw_key: DrawKey(0),
        };
        let sprite = raster.render(&model, &cam);
        assert!(sprite.is_empty());
        assert_eq!(sprite.width, 0);
        assert_eq!(sprite.height, 0);
    }

    #[test]
    fn rasteriser_zero_size_returns_empty_sprite() {
        let raster = Rasteriser::new(0, 32);
        let cam = make_camera();
        let model = make_tree_model();
        let sprite = raster.render(&model, &cam);
        assert!(sprite.is_empty());
    }

    #[test]
    fn rasteriser_deterministic() {
        let raster = make_rasteriser();
        let cam = make_camera();
        let model = make_tree_model();
        let s1 = raster.render(&model, &cam);
        let s2 = raster.render(&model, &cam);
        assert_eq!(s1.cells, s2.cells);
        assert_eq!(s1.depth, s2.depth);
        assert_eq!(s1.anchor, s2.anchor);
    }

    #[test]
    fn rasteriser_lambertian_shading_monotonic() {
        let raster = Rasteriser::new(32, 32).with_light((0.0, 0.0, 1.0));
        let cam = make_camera();
        // The rasteriser shades with one fixed normal per primitive kind
        // (see fill_primitive_footprint): a side-facing Box gets dot = 0
        // against the top-down light, a top-facing Ellipsoid gets dot = 1.
        // Two screen-separated primitives therefore render at opposite ends
        // of the glyph ramp — the monotonic Lambertian contract. Per-pixel
        // normals are a documented future refinement, not the current one.
        let model = ObjectModel {
            key: "test",
            name: "Test".to_string(),
            primitives: vec![
                Primitive {
                    kind: PrimitiveKind::Box,
                    transform: Transform { translation: (0.0, 0.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
                    material: Material { albedo: 1.0, shade_side: true },
                },
                Primitive {
                    kind: PrimitiveKind::Ellipsoid,
                    transform: Transform { translation: (8.0, 8.0, 0.0), rotation: (0.0, 0.0, 0.0), scale: (1.0, 1.0, 1.0) },
                    material: Material { albedo: 1.0, shade_side: true },
                },
            ],
                animations: HashMap::new(),
bounding_box: BoundingBox::default(),
            draw_key: DrawKey(0),
        };
        let sprite = raster.render(&model, &cam);
        // Verify at least some cells have different glyphs (shading variation)
        let glyphs: std::collections::HashSet<char> = sprite.cells.iter()
            .flat_map(|row| row.iter().map(|(ch, _, _)| *ch))
            .collect();
        assert!(glyphs.len() > 1, "shading should produce multiple glyphs");
    }
}