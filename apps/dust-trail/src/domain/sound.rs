//! Sound management for Dust Trail.
//!
//! Provides optional sound effects with volume control and mute support.
//! Feature-gated behind the `sound` Cargo feature.

#[cfg(feature = "sound")]
use rodio::{Decoder, MixerDeviceSink, Player, Source, DeviceSinkBuilder};
use serde::{Deserialize, Serialize};
#[cfg(feature = "sound")]
use std::sync::{Arc, Mutex};

/// Sound effect categories for volume control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundCategory {
    /// Master volume affects all sounds
    Master,
    /// Player movement sounds (footsteps)
    Movement,
    /// Item collection (loot, horse)
    Collection,
    /// Combat and hit sounds
    Combat,
    /// UI sounds (save, load, menu)
    Ui,
}

/// Sound effect identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundEffect {
    // Movement
    Footstep,
    // Collection
    LootPickup,
    HorseMount,
    // Combat
    Hit,
    EnemyDeath,
    // UI
    Save,
    Load,
    GameOver,
    Victory,
    // Ambient
    Ambient,
}

/// Sound settings persisted with save data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoundSettings {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
    /// Whether sound is muted
    pub muted: bool,
    /// Per-category volumes (0.0 to 1.0) - stored as Vec of tuples for serde compatibility
    #[serde(default)]
    pub category_volumes: Vec<(SoundCategory, f32)>,
}

impl SoundSettings {
    pub fn new() -> Self {
        Self {
            master_volume: 0.5,
            muted: false,
            category_volumes: vec![
                (SoundCategory::Master, 1.0),
                (SoundCategory::Movement, 0.5),
                (SoundCategory::Collection, 0.7),
                (SoundCategory::Combat, 0.8),
                (SoundCategory::Ui, 0.6),
            ],
        }
    }

    #[cfg(not(feature = "sound"))]
    pub fn new_stub() -> Self {
        Self {
            master_volume: 0.5,
            muted: true,
            category_volumes: vec![
                (SoundCategory::Master, 1.0),
                (SoundCategory::Movement, 0.5),
                (SoundCategory::Collection, 0.7),
                (SoundCategory::Combat, 0.8),
                (SoundCategory::Ui, 0.6),
            ],
        }
    }

    /// Get effective volume for a category (master * category * (1 if not muted else 0))
    pub fn effective_volume(&self, category: SoundCategory) -> f32 {
        if self.muted {
            return 0.0;
        }
        let master = self.master_volume;
        let category_vol = self
            .category_volumes
            .iter()
            .find(|(c, _)| *c == category)
            .map(|(_, v)| *v)
            .unwrap_or(1.0);
        (master * category_vol).clamp(0.0, 1.0)
    }
}

/// Manages sound playback for the game.
#[cfg(feature = "sound")]
pub struct SoundManager {
    handle: Option<MixerDeviceSink>,
    player: Option<Arc<Mutex<Player>>>,
    settings: SoundSettings,
    sound_files: std::collections::HashMap<SoundEffect, Vec<u8>>,
}

// Manual Debug implementation since rodio::Player doesn't implement Debug
#[cfg(feature = "sound")]
impl std::fmt::Debug for SoundManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoundManager")
            .field("handle", &self.handle.as_ref().map(|_| "MixerDeviceSink"))
            .field("player", &self.player.as_ref().map(|_| "Player"))
            .field("settings", &self.settings)
            .field("sound_files", &format!("{} sounds", self.sound_files.len()))
            .finish()
    }
}

#[cfg(feature = "sound")]
impl SoundManager {
    /// Create a new sound manager with embedded sound files.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let handle = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(handle.mixer());
        player.set_volume(0.5);

        let sound_files = Self::load_sound_files()?;

        Ok(Self {
            handle: Some(handle),
            player: Some(Arc::new(Mutex::new(player))),
            settings: SoundSettings::new(),
            sound_files,
        })
    }

    /// Create a stub sound manager that does nothing (used as fallback when audio is unavailable).
    pub fn stub() -> Self {
        Self {
            handle: None,
            player: None,
            settings: SoundSettings::new(),
            sound_files: std::collections::HashMap::new(),
        }
    }

    /// Load embedded sound files.
    fn load_sound_files(
    ) -> Result<std::collections::HashMap<SoundEffect, Vec<u8>>, Box<dyn std::error::Error>> {
        let mut sounds = std::collections::HashMap::new();

        // Embed sound files using include_bytes!
        sounds.insert(
            SoundEffect::Footstep,
            include_bytes!("../../assets/sounds/footstep.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::LootPickup,
            include_bytes!("../../assets/sounds/loot_pickup.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::HorseMount,
            include_bytes!("../../assets/sounds/horse_mount.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::Hit,
            include_bytes!("../../assets/sounds/hit.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::EnemyDeath,
            include_bytes!("../../assets/sounds/enemy_death.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::Save,
            include_bytes!("../../assets/sounds/save.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::Load,
            include_bytes!("../../assets/sounds/load.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::GameOver,
            include_bytes!("../../assets/sounds/game_over.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::Victory,
            include_bytes!("../../assets/sounds/victory.wav").to_vec(),
        );
        sounds.insert(
            SoundEffect::Ambient,
            include_bytes!("../../assets/sounds/ambient.wav").to_vec(),
        );

        Ok(sounds)
    }

    /// Play a sound effect.
    pub fn play(&self, effect: SoundEffect) {
        if self.settings.muted {
            return;
        }

        // Get the sound data
        if let Some(data) = self.sound_files.get(&effect) {
            if data.is_empty() {
                return; // No sound data loaded
            }

            // Try to decode and play
            if let Ok(source) =
                Decoder::new(std::io::BufReader::new(std::io::Cursor::new(data.clone())))
            {
                if let Some(player) = &self.player {
                    let volume = self
                        .settings
                        .effective_volume(self.category_for_effect(effect));
                    let player = player.lock().unwrap();
                    let source = source.amplify(volume);
                    player.append(source);
                }
            }
        }
    }

    /// Get the category for a sound effect.
    fn category_for_effect(&self, effect: SoundEffect) -> SoundCategory {
        match effect {
            SoundEffect::Footstep => SoundCategory::Movement,
            SoundEffect::LootPickup | SoundEffect::HorseMount => SoundCategory::Collection,
            SoundEffect::Hit | SoundEffect::EnemyDeath => SoundCategory::Combat,
            SoundEffect::Save
            | SoundEffect::Load
            | SoundEffect::GameOver
            | SoundEffect::Victory => SoundCategory::Ui,
            SoundEffect::Ambient => SoundCategory::Master,
        }
    }

    /// Set master volume (0.0 to 1.0).
    pub fn set_master_volume(&mut self, volume: f32) {
        self.settings.master_volume = volume.clamp(0.0, 1.0);
        if let Some(player) = &self.player {
            let player = player.lock().unwrap();
            player.set_volume(self.settings.master_volume);
        }
    }

    /// Toggle mute.
    pub fn toggle_mute(&mut self) {
        self.settings.muted = !self.settings.muted;
        if let Some(player) = &self.player {
            let player = player.lock().unwrap();
            if self.settings.muted {
                player.set_volume(0.0);
            } else {
                player.set_volume(self.settings.master_volume);
            }
        }
    }

    /// Set mute state.
    pub fn set_muted(&mut self, muted: bool) {
        if muted != self.settings.muted {
            self.toggle_mute();
        }
    }

    /// Check if muted.
    pub fn is_muted(&self) -> bool {
        self.settings.muted
    }

    /// Get master volume.
    pub fn master_volume(&self) -> f32 {
        self.settings.master_volume
    }

    /// Set category volume.
    pub fn set_category_volume(&mut self, category: SoundCategory, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        if let Some((_, v)) = self
            .settings
            .category_volumes
            .iter_mut()
            .find(|(c, _)| *c == category)
        {
            *v = volume;
        } else {
            self.settings.category_volumes.push((category, volume));
        }
    }

    /// Get category volume.
    pub fn category_volume(&self, category: SoundCategory) -> f32 {
        self.settings
            .category_volumes
            .iter()
            .find(|(c, _)| *c == category)
            .map(|(_, v)| *v)
            .unwrap_or(1.0)
    }

    /// Get all settings for persistence.
    pub fn settings(&self) -> &SoundSettings {
        &self.settings
    }

    /// Load settings from saved data.
    pub fn load_settings(&mut self, settings: SoundSettings) {
        self.settings = settings;
        if let Some(player) = &self.player {
            let player = player.lock().unwrap();
            player.set_volume(if self.settings.muted {
                0.0
            } else {
                self.settings.master_volume
            });
        }
    }
}

/// Stub implementation when sound feature is disabled.
#[cfg(not(feature = "sound"))]
pub struct SoundManager;

#[cfg(not(feature = "sound"))]
impl SoundManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(SoundManager)
    }

    pub fn play(&self, _effect: SoundEffect) {}
    pub fn set_master_volume(&mut self, _volume: f32) {}
    pub fn toggle_mute(&mut self) {}
    pub fn set_muted(&mut self, _muted: bool) {}
    pub fn is_muted(&self) -> bool {
        true
    }
    pub fn master_volume(&self) -> f32 {
        0.0
    }
    pub fn set_category_volume(&mut self, _category: SoundCategory, _volume: f32) {}
    pub fn category_volume(&self, _category: SoundCategory) -> f32 {
        0.0
    }
    pub fn settings(&self) -> &SoundSettings {
        static SETTINGS: std::sync::OnceLock<SoundSettings> = std::sync::OnceLock::new();
        SETTINGS.get_or_init(|| SoundSettings {
            master_volume: 0.0,
            muted: true,
            category_volumes: vec![
                (SoundCategory::Master, 1.0),
                (SoundCategory::Movement, 0.5),
                (SoundCategory::Collection, 0.7),
                (SoundCategory::Combat, 0.8),
                (SoundCategory::Ui, 0.6),
            ],
        })
    }

    pub fn load_settings(&mut self, _settings: SoundSettings) {}

    pub fn new_stub() -> Self {
        SoundManager
    }
}