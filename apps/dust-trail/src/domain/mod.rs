//! Domain layer — pure Rust game logic, no UI dependencies, fully serialisable
//! and testable headlessly (guide AGENTS.md: domain-first).
//!
//! Clean-room: original game mechanics and ASCII tile design.

pub mod cell;
pub mod color;
pub mod frame;
pub mod game;
pub mod iso;
pub mod player;
pub mod procgen;
pub mod time;
pub mod save;
pub mod slots;
pub mod sound;
pub mod tile;
pub mod world;

pub use cell::Cell;
pub use color::Rgb;
pub use frame::Frame;
pub use game::Game;
pub use iso::world_to_screen;
pub use player::Player;
pub use save::SaveData;
pub use slots::{SaveSlotError, SaveSlotMeta, SaveSlots, SaveStore};
pub use tile::TileType;
pub use time::TimeOfDay;
pub use world::World;
