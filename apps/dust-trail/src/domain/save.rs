//! Serialisable game save data.

use crate::domain::game::Game;

/// A point-in-time snapshot of a game, as written to disk.
///
/// The `slot` field names the save slot this snapshot belongs to. It is empty
/// for the legacy single-file save and is stamped automatically when the
/// snapshot is inserted into a [`crate::domain::slots::SaveSlots`] collection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    /// Application version that wrote this snapshot.
    pub version: String,
    /// Save slot this snapshot belongs to (empty for the legacy save file).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub slot: String,
    /// World state at the moment of saving.
    pub world: crate::domain::world::World,
    /// Player state at the moment of saving.
    pub player: crate::domain::player::Player,
    /// Turn counter at the moment of saving.
    pub turn: u64,
}

impl SaveData {
    /// Snapshot `game` for the legacy single-file save (no slot name).
    pub fn new(game: &Game) -> Self {
        Self::with_slot(game, "")
    }

    /// Snapshot `game`, labelled with `slot`.
    pub fn with_slot(game: &Game, slot: &str) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            slot: slot.to_owned(),
            world: game.world.clone(),
            player: game.player.clone(),
            turn: game.turn,
        }
    }

    /// Turn this snapshot back into a playable game.
    pub fn into_game(self) -> Game {
        Game::from_save_data(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::game::Direction;

    #[test]
    fn save_data_records_the_requested_slot() {
        let game = Game::new();
        assert_eq!(SaveData::with_slot(&game, "camp").slot, "camp");
        assert_eq!(SaveData::new(&game).slot, "");
    }

    #[test]
    fn save_data_serde_roundtrip_keeps_geometry_and_slot() {
        let mut game = Game::new();
        game.move_player(Direction::Right);
        game.move_player(Direction::Down);
        let data = SaveData::with_slot(&game, "camp");

        let json = serde_json::to_string(&data).expect("serialise");
        let back: SaveData = serde_json::from_str(&json).expect("deserialise");

        assert_eq!(back.slot, "camp");
        assert_eq!(back.turn, game.turn);
        assert_eq!(back.player.pos, game.player.pos);
        assert_eq!(back.version, data.version);
    }

    #[test]
    fn legacy_save_without_slot_field_still_deserialises() {
        // A v0.2.0-era document has no `slot` key at all.
        let game = Game::new();
        let data = SaveData::with_slot(&game, "camp");
        let mut value = serde_json::to_value(data).expect("to value");
        let object = value.as_object_mut().expect("object");
        object.remove("slot");

        let back: SaveData = serde_json::from_value(value).expect("deserialise legacy");
        assert_eq!(back.slot, "");
        assert_eq!(back.turn, 0);
    }

    #[test]
    fn snapshot_into_game_restores_state() {
        let mut game = Game::new();
        game.move_player(Direction::Left);
        let restored = SaveData::with_slot(&game, "camp").into_game();
        assert_eq!(restored.player.pos, game.player.pos);
        assert_eq!(restored.turn, game.turn);
        assert!(!restored.game_over);
    }
}
