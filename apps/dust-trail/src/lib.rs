//! Dust Trail — original Western-themed ASCII adventure game.
//!
//! Domain-first: `domain` is pure Rust, headless and testable. `tui` and
//! `gui` are frontends that render the same `Frame` produced by the domain.
//!
//! Clean-room: original mechanics, art style, and assets.

pub mod domain;
pub mod tui;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "gui")]
pub mod theme;

pub use domain::{Game, TileType, World};

#[cfg(feature = "gui")]
pub use theme::{Role, Theme, Variant};
