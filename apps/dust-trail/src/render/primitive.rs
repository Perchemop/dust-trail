//! Primitive types: 3D primitives with transform and material.

use crate::domain::color::Rgb;

/// A single 3D primitive with a local transform and material.
#[derive(Clone, Debug, PartialEq)]
pub struct Primitive {
    pub kind: PrimitiveKind,
    pub transform: Transform,
    pub material: Material,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimitiveKind {
    Cone,     // trunk / tapered shape
    Ellipsoid,// canopy / rounded mass
    Box,      // torso / building block
    Cylinder, // hat / pillar
    Capsule,  // limb / rounded bar
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: (f32, f32, f32), // (x, y, z) in local object space
    pub rotation: (f32, f32, f32),    // yaw, pitch, roll in radians
    pub scale: (f32, f32, f32),
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub albedo: f32,   // base luminance 0.0..=1.0
    pub shade_side: bool, // whether Lambertian shading applies per face
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: 0.5,
            shade_side: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_construction_no_panic() {
        // Valid construction
        let p = Primitive {
            kind: PrimitiveKind::Cone,
            transform: Transform::default(),
            material: Material::default(),
        };
        assert_eq!(p.kind, PrimitiveKind::Cone);

        // Invalid values stored as-is; rasteriser handles degeneracy
        let p2 = Primitive {
            kind: PrimitiveKind::Box,
            transform: Transform {
                translation: (f32::NAN, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
                scale: (0.0, 0.0, 0.0),
            },
            material: Material {
                albedo: -1.0,
                shade_side: false,
            },
        };
        assert_eq!(p2.kind, PrimitiveKind::Box);
    }
}