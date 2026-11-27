//! Render module: software 3D primitive rasteriser for ASCII viewport.
//!
//! This module is compiled only when the `gui` feature is enabled.
//! It provides a headless, deterministic renderer that converts 3D primitive
//! models into depth-aware ASCII sprites for the egui/eframe GUI frontend.

pub mod primitive;
pub mod model;
pub mod raster;
pub mod glyph;
pub mod sprite;
pub mod composite;
pub mod cache;
pub mod camera;
pub mod animation; // T-201

// Re-export key types for convenience
pub use primitive::{Primitive, PrimitiveKind, Transform, Material};
pub use model::{ObjectModel, ModelKey, BoundingBox, ModelRegistry, EntityKind, DrawKey};
pub use raster::Rasteriser;
pub use glyph::luminance_to_glyph;
pub use sprite::Sprite;
pub use composite::composite_sprite;
pub use cache::{SpriteCache, CacheKey, LightState, Orientation};
pub use camera::Camera;
pub use animation::{Animation, AnimFrame, AnimationController, AnimKey, LoopMode, PrimitiveDelta};