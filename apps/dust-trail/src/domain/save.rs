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
    /// Time-of-day at the moment of saving (turns since midnight).
    /// Optional with default 0 for v2.0.x saves that predate this field.
    #[serde(default = "default_time_of_day")]
    pub time_of_day: u64,
    /// Sound settings for persistence.
    #[cfg(feature = "sound")]
    #[serde(default)]
    pub sound_settings: crate::domain::sound::SoundSettings,
    #[cfg(not(feature = "sound"))]
    #[serde(default)]
    pub sound_settings: crate::domain::sound::SoundSettings,
}

impl SaveData {
    /// Snapshot `game` for the legacy single-file save (no slot name).
    pub fn new(game: &Game) -> Self {
        Self::with_slot(game, "")
    }

    /// Snapshot `game`, labelled with `slot`.
    pub fn with_slot(game: &Game, slot: &str) -> Self {
        #[cfg(feature = "sound")]
        let sound_settings = game.sound_manager.settings().clone();
        #[cfg(not(feature = "sound"))]
        let sound_settings = game.sound_settings.clone();

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            slot: slot.to_owned(),
            world: game.world.clone(),
            player: game.player.clone(),
            turn: game.turn,
            time_of_day: u64::from(game.world.time_of_day.turn_count()),
            sound_settings,
        }
    }

    /// Turn this snapshot back into a playable game, applying any necessary
    /// version migration for older save formats.
    pub fn into_game(mut self) -> Game {
        self.migrate();
        Game::from_save_data(self)
    }

    /// Apply version-based migration rules to older save formats.
    ///
    /// For v1.3.0, no breaking changes are expected, but this hook is
    /// reserved for future compatibility. All fields use `#[serde(default)]`
    /// so missing keys from older saves deserialize to safe defaults.
    fn migrate(&mut self) {
        // Pre-1.2.0 saves lack `health` on Player and Npc; these default to
        // 0 via #[serde(default)], which means dead. Restore to full health
        // for playability.
        if self.version.starts_with("0.")
            || self.version.starts_with("1.0.")
            || self.version.starts_with("1.1.")
        {
            self.player.health = self.player.health.max(100);
            for npc in &mut self.world.npcs {
                npc.health = npc.health.max(50);
            }
        }
        // v2.0.x saves lack `time_of_day`; add default (noon = turn 120).
        if self.version.starts_with("2.")
            && !self.version.starts_with("2.1.")
        {
            self.time_of_day = 120; // noon
        }
        // v2.1.0+ saves are current; no migration needed.
        // Unknown future versions rely on #[serde(default)] for safe defaults.
    }
}

// Default for the optional `time_of_day` field in SaveData.
// Used when deserializing v2.0.x saves that predate this field.
fn default_time_of_day() -> u64 {
    0
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

    #[test]
    fn v1_x_save_migrates_health_to_safe_defaults() {
        // Simulate a v1.1.0 save with zero health (pre-health fields)
        let game = Game::new();
        let data = SaveData::with_slot(&game, "test");
        let mut value = serde_json::to_value(data).expect("to value");
        let object = value.as_object_mut().expect("object");
        // Set version to v1.1.0
        object.insert("version".to_owned(), serde_json::Value::String("1.1.0".to_owned()));
        // Remove health fields to simulate pre-health save
        if let Some(player_obj) = object.get_mut("player").and_then(|v| v.as_object_mut()) {
            player_obj.remove("health");
        }
        if let Some(world_obj) = object.get_mut("world").and_then(|v| v.as_object_mut()) {
            if let Some(npcs) = world_obj.get_mut("npcs").and_then(|v| v.as_array_mut()) {
                for npc in npcs {
                    if let Some(npc_obj) = npc.as_object_mut() {
                        npc_obj.remove("health");
                    }
                }
            }
        }

        let back: SaveData = serde_json::from_value(value).expect("deserialise v1.1 save");
        let restored = back.into_game();

        // Health should be restored to safe defaults
        assert_eq!(restored.player.health, 100);
        for npc in &restored.world.npcs {
            assert_eq!(npc.health, 50);
        }
    }

    #[test]
    fn v2_x_save_loads_without_migration_changes() {
        let mut game = Game::new();
        game.move_player(Direction::Right);
        let data = SaveData::with_slot(&game, "v2_test");
        let mut value = serde_json::to_value(data).expect("to value");
        let object = value.as_object_mut().expect("object");
        // Set version to 2.0.0
        object.insert("version".to_owned(), serde_json::Value::String("2.0.0".to_owned()));

        let back: SaveData = serde_json::from_value(value).expect("deserialise v2.0.0 save");
        let restored = back.into_game();

        // State should be preserved exactly (no migration changes)
        assert_eq!(restored.player.pos, game.player.pos);
        assert_eq!(restored.turn, game.turn);
        assert_eq!(restored.player.health, 100); // Full health
    }

    #[test]
    fn unknown_future_version_loads_with_serde_defaults() {
        let game = Game::new();
        let data = SaveData::with_slot(&game, "future_test");
        let mut value = serde_json::to_value(data).expect("to value");
        let object = value.as_object_mut().expect("object");
        // Set version to unknown future 3.5.0
        object.insert("version".to_owned(), serde_json::Value::String("3.5.0".to_owned()));

        let back: SaveData = serde_json::from_value(value).expect("deserialise future save");
        let restored = back.into_game();

        // Should load with safe defaults via #[serde(default)]
        assert_eq!(restored.player.health, 100);
        for npc in &restored.world.npcs {
            assert_eq!(npc.health, 50);
        }
    }
}
