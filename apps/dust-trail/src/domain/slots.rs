//! Multiple named save slots.
//!
//! [`SaveSlots`] is a pure, serialisable collection of [`SaveData`] snapshots
//! keyed by a user-chosen slot name, ordered by name so serialised output is
//! deterministic. It performs no I/O and is fully testable headlessly.
//!
//! [`SaveStore`] is the thin file-system adapter that persists one such
//! collection as a single JSON document. Keeping every slot in one document
//! means listing, overwriting and deleting a slot never needs to scan a
//! directory or reason about several files at once.
//!
//! The legacy single-file save (`savegame.json`) is still readable: see
//! [`SaveStore::import_legacy`].

use std::collections::BTreeMap;
use std::fmt;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::domain::save::SaveData;

/// On-disk format version written by this build.
pub const SAVE_SLOTS_FORMAT_VERSION: u32 = 1;

/// Maximum accepted length of a save slot name, in characters.
pub const MAX_SLOT_NAME_LEN: usize = 40;

/// File name of the slot container inside the save directory.
pub const SAVE_SLOTS_FILE_NAME: &str = "saves.json";

/// File name of the legacy single-save document used before slots existed.
pub const LEGACY_SAVE_FILE_NAME: &str = "savegame.json";

/// Reasons a save slot operation can be refused.
///
/// Returned by the pure collection API; the file-system adapter in
/// [`SaveStore`] reports the same conditions as [`String`] messages so callers
/// keep one error style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveSlotError {
    /// The name was empty or only whitespace.
    EmptyName,
    /// The name was longer than [`MAX_SLOT_NAME_LEN`] characters.
    NameTooLong {
        /// Length that was supplied, in characters.
        len: usize,
        /// Maximum length that is accepted.
        max: usize,
    },
    /// The name contained a character that is not allowed in a slot name.
    InvalidCharacter {
        /// The first offending character.
        ch: char,
    },
    /// No slot with that name exists.
    NotFound(String),
}

impl fmt::Display for SaveSlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "save slot name must not be empty"),
            Self::NameTooLong { len, max } => {
                write!(
                    f,
                    "save slot name has {len} characters; the maximum is {max}"
                )
            }
            Self::InvalidCharacter { ch } => {
                write!(f, "save slot name has an unsupported character {ch:?}")
            }
            Self::NotFound(name) => write!(f, "save slot {name:?} does not exist"),
        }
    }
}

impl std::error::Error for SaveSlotError {}

/// Whether `ch` may appear in a save slot name.
///
/// Letters, digits, spaces and a small set of punctuation are accepted. Path
/// separators, control characters and shell metacharacters are not, so a slot
/// name can never be mistaken for a path or a command-line option.
fn is_allowed_slot_char(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, ' ' | '-' | '_' | '.' | '\'' | '(' | ')')
}

/// Trim and validate a user-supplied slot name.
///
/// Returns the canonical (trimmed) name to store as the slot key.
pub fn normalise_slot_name(name: &str) -> Result<String, SaveSlotError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(SaveSlotError::EmptyName);
    }
    let len = trimmed.chars().count();
    if len > MAX_SLOT_NAME_LEN {
        return Err(SaveSlotError::NameTooLong {
            len,
            max: MAX_SLOT_NAME_LEN,
        });
    }
    if let Some(ch) = trimmed.chars().find(|c| !is_allowed_slot_char(*c)) {
        return Err(SaveSlotError::InvalidCharacter { ch });
    }
    Ok(trimmed.to_owned())
}

/// Default for the container's `format_version` field.
fn current_format_version() -> u32 {
    SAVE_SLOTS_FORMAT_VERSION
}

/// A summary of one save slot, for save/load menus.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SaveSlotMeta {
    /// User-chosen slot name.
    pub name: String,
    /// Application version that wrote the slot.
    pub version: String,
    /// Turn counter stored in the slot.
    pub turn: u64,
    /// Player character name stored in the slot.
    pub player_name: String,
    /// Player score stored in the slot.
    pub score: u32,
}

/// A collection of named save slots.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveSlots {
    /// Format version of the container document.
    #[serde(default = "current_format_version")]
    pub format_version: u32,
    /// Slot name to save data, ordered by name for deterministic output.
    #[serde(default)]
    pub slots: BTreeMap<String, SaveData>,
}

impl SaveSlots {
    /// Create an empty collection at the current format version.
    pub fn new() -> Self {
        Self {
            format_version: SAVE_SLOTS_FORMAT_VERSION,
            slots: BTreeMap::new(),
        }
    }

    /// Number of slots held.
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Whether no slots are held.
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Slot names, ordered alphabetically.
    pub fn names(&self) -> Vec<String> {
        self.slots.keys().cloned().collect()
    }

    /// Whether a slot with this name exists (the name is normalised first).
    pub fn contains(&self, name: &str) -> bool {
        normalise_slot_name(name).is_ok_and(|key| self.slots.contains_key(&key))
    }

    /// Borrow the save data stored in `name`.
    pub fn get(&self, name: &str) -> Result<&SaveData, SaveSlotError> {
        let key = normalise_slot_name(name)?;
        match self.slots.get(&key) {
            Some(data) => Ok(data),
            None => Err(SaveSlotError::NotFound(key)),
        }
    }

    /// Insert `data` under `name`, replacing any existing slot of that name.
    ///
    /// The canonical slot name is stamped into the [`SaveData`] and returned,
    /// so callers can report the name that was actually stored.
    pub fn insert(&mut self, name: &str, mut data: SaveData) -> Result<String, SaveSlotError> {
        let key = normalise_slot_name(name)?;
        data.slot = key.clone();
        self.slots.insert(key.clone(), data);
        Ok(key)
    }

    /// Remove `name` and return its data.
    pub fn remove(&mut self, name: &str) -> Result<SaveData, SaveSlotError> {
        let key = normalise_slot_name(name)?;
        match self.slots.remove(&key) {
            Some(data) => Ok(data),
            None => Err(SaveSlotError::NotFound(key)),
        }
    }

    /// Per-slot summaries, ordered by slot name.
    pub fn metadata(&self) -> Vec<SaveSlotMeta> {
        self.slots
            .iter()
            .map(|(name, data)| SaveSlotMeta {
                name: name.clone(),
                version: data.version.clone(),
                turn: data.turn,
                player_name: data.player.name.clone(),
                score: data.player.score,
            })
            .collect()
    }
}

impl Default for SaveSlots {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve the directory where saves live, matching the location the GUI has
/// always used so existing saves keep working.
///
/// Path resolution only; no file-system access happens here.
pub fn default_save_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "dust-trail")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .unwrap_or_else(|| std::env::temp_dir().join("dust-trail"))
}

/// A [`SaveSlots`] collection persisted as one JSON document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveStore {
    path: PathBuf,
}

impl SaveStore {
    /// Create a store that reads and writes the document at `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Path of the default container document (see [`default_save_dir`]).
    pub fn default_path() -> PathBuf {
        default_save_dir().join(SAVE_SLOTS_FILE_NAME)
    }

    /// Path of the backing document.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Path of the temporary file used to write atomically.
    fn tmp_path(&self) -> PathBuf {
        let mut raw = self.path.clone().into_os_string();
        raw.push(".tmp");
        PathBuf::from(raw)
    }

    /// Read the collection, treating an absent or empty document as no slots.
    pub fn read(&self) -> Result<SaveSlots, String> {
        let text = match std::fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(SaveSlots::new()),
            Err(err) => return Err(format!("read error: {err}")),
        };
        if text.trim().is_empty() {
            return Ok(SaveSlots::new());
        }
        let slots: SaveSlots =
            serde_json::from_str(&text).map_err(|err| format!("parse error: {err}"))?;
        let found = slots.format_version;
        let limit = SAVE_SLOTS_FORMAT_VERSION;
        if found > limit {
            return Err(format!(
                "save format {found} is newer than supported ({limit})"
            ));
        }
        Ok(slots)
    }

    /// Write the collection atomically (temporary file, then rename).
    pub fn write(&self, slots: &SaveSlots) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|err| format!("mkdir error: {err}"))?;
            }
        }
        let mut out = slots.clone();
        out.format_version = SAVE_SLOTS_FORMAT_VERSION;
        let json =
            serde_json::to_string_pretty(&out).map_err(|err| format!("serialise error: {err}"))?;
        let tmp = self.tmp_path();
        std::fs::write(&tmp, json).map_err(|err| format!("write error: {err}"))?;
        std::fs::rename(&tmp, &self.path).map_err(|err| format!("rename error: {err}"))?;
        Ok(())
    }

    /// Save `data` under `slot`, returning the canonical slot name written.
    pub fn save_data(&self, slot: &str, data: SaveData) -> Result<String, String> {
        let mut slots = self.read()?;
        let saved_as = slots.insert(slot, data).map_err(|err| err.to_string())?;
        self.write(&slots)?;
        Ok(saved_as)
    }

    /// Load the save data stored in `slot`.
    pub fn load_data(&self, slot: &str) -> Result<SaveData, String> {
        let slots = self.read()?;
        slots.get(slot).cloned().map_err(|err| err.to_string())
    }

    /// Slot names, ordered alphabetically.
    pub fn list_saves(&self) -> Result<Vec<String>, String> {
        Ok(self.read()?.names())
    }

    /// Slot names, or an empty list when the document cannot be read.
    ///
    /// Intended for menus that must still render when the document is corrupt;
    /// use [`SaveStore::list_saves`] when the error matters.
    pub fn list_saves_or_empty(&self) -> Vec<String> {
        match self.read() {
            Ok(slots) => slots.names(),
            Err(_) => Vec::new(),
        }
    }

    /// Per-slot summaries, ordered by slot name.
    pub fn list_metadata(&self) -> Result<Vec<SaveSlotMeta>, String> {
        Ok(self.read()?.metadata())
    }

    /// Delete `slot`.
    pub fn delete_save(&self, slot: &str) -> Result<(), String> {
        let mut slots = self.read()?;
        slots.remove(slot).map_err(|err| err.to_string())?;
        self.write(&slots)
    }

    /// Whether `slot` exists. A document that cannot be read reports `false`.
    pub fn save_exists(&self, slot: &str) -> bool {
        self.read().is_ok_and(|slots| slots.contains(slot))
    }

    /// Import a legacy single-file save into `slot`.
    ///
    /// Returns `Ok(false)` when there is no legacy file to import, so callers
    /// can run this unconditionally on startup without turning a fresh install
    /// into an error.
    pub fn import_legacy(&self, legacy: &Path, slot: &str) -> Result<bool, String> {
        let text = match std::fs::read_to_string(legacy) {
            Ok(text) => text,
            Err(err) if err.kind() == ErrorKind::NotFound => return Ok(false),
            Err(err) => return Err(format!("read error: {err}")),
        };
        let data: SaveData =
            serde_json::from_str(&text).map_err(|err| format!("parse error: {err}"))?;
        self.save_data(slot, data)?;
        Ok(true)
    }

    /// Import a legacy save only while `slot` is still free.
    ///
    /// Returns `Ok(true)` when the legacy file was imported, and `Ok(false)`
    /// when there was nothing to import or the slot already holds a save. Use
    /// this for a startup migration: it can run on every launch without ever
    /// overwriting a newer slot with the old single-file save.
    pub fn import_legacy_once(&self, legacy: &Path, slot: &str) -> Result<bool, String> {
        if self.save_exists(slot) {
            return Ok(false);
        }
        self.import_legacy(legacy, slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::game::{Direction, Game};

    /// A unique, empty scratch directory for one headless test.
    fn scratch(tag: &str) -> PathBuf {
        let name = format!("dust-trail-slots-{}-{tag}", std::process::id());
        let dir = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::create_dir_all(&dir);
        dir
    }

    /// One snapshot of a fresh game, labelled with `slot`.
    fn sample(slot: &str) -> SaveData {
        let game = Game::new();
        SaveData::with_slot(&game, slot)
    }

    #[test]
    fn slot_names_are_validated_and_trimmed() {
        let trimmed = normalise_slot_name("  Keep  ");
        assert_eq!(trimmed, Ok("Keep".to_string()));

        let spaced = normalise_slot_name("Slot (2)");
        assert_eq!(spaced, Ok("Slot (2)".to_string()));

        let blank = normalise_slot_name("   ");
        assert_eq!(blank, Err(SaveSlotError::EmptyName));

        let empty = normalise_slot_name("");
        assert_eq!(empty, Err(SaveSlotError::EmptyName));

        let max = "x".repeat(MAX_SLOT_NAME_LEN);
        assert_eq!(normalise_slot_name(&max), Ok(max));

        let over = "x".repeat(MAX_SLOT_NAME_LEN + 1);
        let too_long = normalise_slot_name(&over);
        assert!(matches!(too_long, Err(SaveSlotError::NameTooLong { .. })));

        let separator = normalise_slot_name("a/b");
        let err = separator.expect_err("path separators are not allowed");
        assert_eq!(err, SaveSlotError::InvalidCharacter { ch: '/' });

        // A backslash must not be mistaken for a Windows path separator.
        let escaped = normalise_slot_name("a\\b");
        let err = escaped.expect_err("backslashes are not allowed");
        assert_eq!(err, SaveSlotError::InvalidCharacter { ch: '\\' });

        // Display messages are user-facing, so they must stay meaningful.
        assert!(SaveSlotError::EmptyName.to_string().contains("empty"));
        let missing = SaveSlotError::NotFound("gone".to_string());
        assert!(missing.to_string().contains("gone"));
    }

    #[test]
    fn slots_can_be_inserted_overwritten_and_removed() {
        let mut slots = SaveSlots::new();
        assert!(slots.is_empty());

        let first = slots.insert("alpha", sample("alpha"));
        assert!(first.is_ok());
        let again = slots.insert("  alpha  ", sample("alpha"));
        assert!(again.is_ok(), "a trimmed duplicate must overwrite");
        assert_eq!(slots.len(), 1);

        // A rejected name must leave the collection untouched.
        let bad = slots.insert("bad/name", sample("bad"));
        assert!(bad.is_err());
        assert_eq!(slots.len(), 1, "a rejected insert must not add a slot");

        assert!(slots.contains(" alpha "));
        let stored = slots.get("alpha").expect("get");
        assert_eq!(stored.slot, "alpha");

        let second = slots.insert("beta", sample("beta"));
        assert!(second.is_ok());
        let names = slots.names();
        assert_eq!(names, ["alpha", "beta"]);

        let removed = slots.remove("beta").expect("remove");
        assert_eq!(removed.slot, "beta");
        let names = slots.names();
        assert_eq!(names, ["alpha"]);

        let gone = slots.get("beta");
        let err = gone.expect_err("removed slot must not resolve");
        assert_eq!(err, SaveSlotError::NotFound("beta".to_string()));
        assert!(slots.remove("beta").is_err());
    }

    #[test]
    fn insert_stamps_the_canonical_slot_name() {
        let mut slots = SaveSlots::new();
        let data = sample("ignored");
        let saved_as = slots.insert("  Real Name  ", data).expect("insert");
        assert_eq!(saved_as, "Real Name");

        let stored = slots.get("Real Name").expect("get");
        assert_eq!(stored.slot, "Real Name");
    }

    #[test]
    fn slot_metadata_summarises_each_save() {
        let mut slots = SaveSlots::new();
        let added = slots.insert("first", sample("first"));
        assert!(added.is_ok());

        let meta = slots.metadata();
        assert_eq!(meta.len(), 1);
        assert_eq!(meta[0].name, "first");
        assert_eq!(meta[0].turn, 0);
        assert_eq!(meta[0].player_name, "Dust");
        assert_eq!(meta[0].score, 0);
        assert!(!meta[0].version.is_empty());
    }

    #[test]
    fn save_slots_serde_roundtrip() {
        let mut slots = SaveSlots::new();
        let one = slots.insert("one", sample("one"));
        assert!(one.is_ok());
        let two = slots.insert("two", sample("two"));
        assert!(two.is_ok());

        let json = serde_json::to_string(&slots).expect("serialise");
        let back: SaveSlots = serde_json::from_str(&json).expect("deserialise");

        assert_eq!(back.format_version, SAVE_SLOTS_FORMAT_VERSION);
        assert_eq!(back.names(), slots.names());

        let stored = back.get("two").expect("get");
        let original = slots.get("two").expect("get");
        assert_eq!(stored.slot, original.slot);
        assert_eq!(stored.turn, original.turn);
        assert_eq!(stored.player.pos, original.player.pos);
    }

    #[test]
    fn slot_meta_serde_roundtrip() {
        let meta = SaveSlotMeta {
            name: "camp".to_string(),
            version: "0.2.0".to_string(),
            turn: 12,
            player_name: "Dust".to_string(),
            score: 30,
        };
        let json = serde_json::to_string(&meta).expect("serialise");
        let back: SaveSlotMeta = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(back, meta);
    }

    #[test]
    fn container_without_fields_deserialises_to_empty_current_version() {
        let slots: SaveSlots = serde_json::from_str("{}").expect("empty object");
        assert!(slots.is_empty());
        assert_eq!(slots.format_version, SAVE_SLOTS_FORMAT_VERSION);
    }

    #[test]
    fn store_roundtrips_two_slots_through_disk() {
        let dir = scratch("roundtrip");
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        // A missing document is an empty collection, not an error.
        let missing = store.read().expect("read missing");
        assert!(missing.is_empty());
        assert!(store.list_saves().expect("list missing").is_empty());
        assert!(!store.save_exists("nope"));

        let lucky = sample("lucky");
        let dusty = sample("dusty");
        store.save_data("lucky", lucky).expect("save lucky");
        store.save_data("dusty", dusty).expect("save dusty");

        let names = store.list_saves().expect("list");
        assert_eq!(names, ["dusty", "lucky"]);

        let loaded = store.load_data("lucky").expect("load");
        assert_eq!(loaded.slot, "lucky");
        assert!(store.save_exists(" dusty "));

        let meta = store.list_metadata().expect("metadata");
        assert_eq!(meta.len(), 2);
        assert!(store.path().exists(), "document must be written");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_deletes_and_reports_missing_slots() {
        let dir = scratch("delete");
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        let kept = sample("keep");
        let doomed = sample("drop");
        store.save_data("keep", kept).expect("save keep");
        store.save_data("drop", doomed).expect("save drop");

        store.delete_save("drop").expect("delete drop");
        let names = store.list_saves().expect("list");
        assert_eq!(names, ["keep"]);
        assert!(!store.save_exists("drop"));

        let again = store.delete_save("drop");
        let err = again.expect_err("the slot is already gone");
        assert!(err.contains("drop"), "unexpected message: {err}");

        let gone = store.load_data("drop");
        let err = gone.expect_err("the slot is already gone");
        assert!(err.contains("drop"), "unexpected message: {err}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_rejects_invalid_slot_names() {
        let dir = scratch("invalid");
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        let blank = sample("blank");
        assert!(store.save_data("  ", blank).is_err());

        let slash = sample("slash");
        assert!(store.save_data("bad/name", slash).is_err());

        let names = store.list_saves().expect("list");
        assert!(names.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_reports_corrupt_document_and_keeps_list_usable() {
        let dir = scratch("corrupt");
        let path = dir.join(SAVE_SLOTS_FILE_NAME);
        assert!(std::fs::write(&path, "{ not json").is_ok());
        let store = SaveStore::new(&path);

        let read = store.read();
        let err = read.expect_err("a corrupt document must fail");
        assert!(err.contains("parse error"), "unexpected message: {err}");

        assert!(store.list_saves_or_empty().is_empty());
        assert!(!store.save_exists("anything"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_treats_blank_document_as_empty() {
        let dir = scratch("blank");
        let path = dir.join(SAVE_SLOTS_FILE_NAME);
        assert!(std::fs::write(&path, "   \n").is_ok());
        let store = SaveStore::new(&path);

        let slots = store.read().expect("blank is empty");
        assert!(slots.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_rejects_a_newer_format_version() {
        let dir = scratch("newer");
        let path = dir.join(SAVE_SLOTS_FILE_NAME);
        let version = SAVE_SLOTS_FORMAT_VERSION + 1;
        let newer = format!("{{\"format_version\":{version},\"slots\":{{}}}}");
        assert!(std::fs::write(&path, newer).is_ok());
        let store = SaveStore::new(&path);

        let read = store.read();
        let err = read.expect_err("a newer format must fail");
        assert!(err.contains("format"), "unexpected message: {err}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_imports_a_legacy_single_file_save() {
        let dir = scratch("legacy");
        let legacy = dir.join(LEGACY_SAVE_FILE_NAME);
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        // Nothing to import yet.
        let imported = store.import_legacy(&legacy, "old").expect("no file");
        assert!(!imported);

        let game = Game::new();
        let json = serde_json::to_string(&SaveData::new(&game)).expect("serialise");
        assert!(std::fs::write(&legacy, json).is_ok());

        let imported = store.import_legacy(&legacy, "old").expect("import");
        assert!(imported);
        assert!(store.save_exists("old"));

        let loaded = store.load_data("old").expect("load");
        assert_eq!(loaded.slot, "old");
        assert_eq!(loaded.turn, game.turn);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn store_import_leaves_other_slots_alone() {
        let dir = scratch("legacy-merge");
        let legacy = dir.join(LEGACY_SAVE_FILE_NAME);
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        let kept = sample("keep");
        store.save_data("keep", kept).expect("save keep");

        let game = Game::new();
        let json = serde_json::to_string(&SaveData::new(&game)).expect("serialise");
        assert!(std::fs::write(&legacy, json).is_ok());
        assert!(store.import_legacy(&legacy, "old").expect("import"));

        let names = store.list_saves().expect("list");
        assert_eq!(names, ["keep", "old"]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn import_once_never_overwrites_an_existing_slot() {
        let dir = scratch("legacy-once");
        let legacy = dir.join(LEGACY_SAVE_FILE_NAME);
        let store = SaveStore::new(dir.join(SAVE_SLOTS_FILE_NAME));

        let game = Game::new();
        let json = serde_json::to_string(&SaveData::new(&game)).expect("serialise");
        assert!(std::fs::write(&legacy, json).is_ok());

        let first = store.import_legacy_once(&legacy, "old");
        assert!(first.expect("first import"));
        assert!(store.save_exists("old"));

        // A newer save in the same slot must survive a second startup import.
        let mut newer = Game::new();
        newer.move_player(Direction::Right);
        let moved = SaveData::with_slot(&newer, "old");
        store.save_data("old", moved).expect("save old");

        let second = store.import_legacy_once(&legacy, "old");
        assert!(!second.expect("second import is a no-op"));

        let kept = store.load_data("old").expect("load");
        assert_eq!(kept.player.pos, newer.player.pos);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn default_path_ends_with_the_container_file_name() {
        let path = SaveStore::default_path();
        let name = path.file_name();
        assert_eq!(name, Some(std::ffi::OsStr::new(SAVE_SLOTS_FILE_NAME)));
        assert!(path.is_absolute(), "default save path must be absolute");
    }
}
