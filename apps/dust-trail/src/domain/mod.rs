//! Domain layer — pure Rust game logic, no UI dependencies, fully serialisable
//! and testable headlessly (guide AGENTS.md: domain-first).
//!
//! Clean-room: original game mechanics and ASCII tile design.

pub mod cell;
pub mod color;
pub mod frame;
pub mod game;
pub mod save;
pub mod tile;
pub mod world;

pub use cell::Cell;
pub use color::Rgb;
pub use frame::Frame;
pub use game::Game;
pub use save::SaveData;
pub use tile::TileType;
pub use world::World;
