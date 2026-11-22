//! Dust Trail — original Western-themed ASCII adventure game.
//!
//! Domain-first: `domain` is pure Rust, headless and testable. `gui` is the
//! frontend that renders the `Frame` produced by the domain. The TUI frontend
//! was removed in v2.0.0.
//!
//! Clean-room: original mechanics, art style, and assets.

pub mod domain;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "gui")]
pub mod render;

#[cfg(feature = "gui")]
pub mod theme;

pub use domain::{Game, TileType, World};

#[cfg(feature = "gui")]
pub use theme::{Role, Theme, Variant};
