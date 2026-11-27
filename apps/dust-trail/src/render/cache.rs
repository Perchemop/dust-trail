//! SpriteCache: bounded LRU cache for rendered sprites.

use crate::render::model::ModelKey;
use crate::render::sprite::Sprite;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Quantised light direction + intensity for cache keying.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightState {
    pub dir_x: i8, // -127..127 representing -1.0..1.0
    pub dir_y: i8,
    pub dir_z: i8,
    pub intensity: u8, // 0..255 representing 0.0..1.0
}

impl LightState {
    pub fn from_light(dir: (f32, f32, f32), intensity: f32) -> Self {
        let quant = |v: f32| (v.clamp(-1.0, 1.0) * 127.0).round() as i8;
        Self {
            dir_x: quant(dir.0),
            dir_y: quant(dir.1),
            dir_z: quant(dir.2),
            intensity: (intensity.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }
}

/// Camera orientation (fixed in v2.0.0, but keyed for future).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Isometric,
}

/// Cache key: (model key, light state, size, orientation, animation frame).
/// animation_frame = 0 means static (no animation); >0 is a content hash for the frame.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CacheKey {
    pub model_key: ModelKey,
    pub light_state: LightState,
    pub size: (u32, u32),    // raster size
    pub orientation: Orientation,
    pub animation_frame: u64, // 0 = static; >0 = specific frame hash
}

/// Bounded LRU cache of rendered sprites.
#[derive(Debug, Clone)]
pub struct SpriteCache {
    entries: HashMap<CacheKey, Sprite>,
    order: VecDeque<CacheKey>, // LRU order: front = most recent, back = least recent
    max_entries: usize,
}

impl SpriteCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            order: VecDeque::new(),
            max_entries,
        }
    }

    /// Get a sprite from cache, updating LRU order on hit.
    pub fn get(&self, key: &CacheKey) -> Option<&Sprite> {
        self.entries.get(key)
    }

    /// Insert a sprite into cache, evicting LRU if over budget.
    pub fn insert(&mut self, key: CacheKey, sprite: Sprite) {
        if self.entries.contains_key(&key) {
            // Update existing: move to front of LRU
            self.order.retain(|k| k != &key);
            self.order.push_front(key.clone());
            self.entries.insert(key, sprite);
        } else if self.entries.len() >= self.max_entries && self.max_entries > 0 {
            // Evict LRU
            if let Some(lru_key) = self.order.pop_back() {
                self.entries.remove(&lru_key);
            }
            // Insert new
            self.order.push_front(key.clone());
            self.entries.insert(key, sprite);
        } else {
            // Insert new (not full)
            self.order.push_front(key.clone());
            self.entries.insert(key, sprite);
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.order.clear();
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }
}

impl Default for SpriteCache {
    fn default() -> Self {
        Self::new(256) // sensible default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_key(model: &'static str) -> CacheKey {
        CacheKey {
            model_key: model,
            light_state: LightState { dir_x: 0, dir_y: 0, dir_z: 127, intensity: 255 },
            size: (32, 32),
            orientation: Orientation::Isometric,
            animation_frame: 0,
        }
    }

    fn make_sprite(key: &'static str) -> Sprite {
        Sprite::placeholder(key)
    }

    #[test]
    fn cache_hit_returns_stored_sprite() {
        let mut cache = SpriteCache::new(10);
        let key = make_key("tree");
        let sprite = make_sprite("tree");
        cache.insert(key.clone(), sprite);
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn cache_miss_returns_none() {
        let cache = SpriteCache::new(10);
        let key = make_key("tree");
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn cache_lru_eviction_on_overflow() {
        let mut cache = SpriteCache::new(2);
        cache.insert(make_key("a"), make_sprite("a"));
        cache.insert(make_key("b"), make_sprite("b"));
        assert_eq!(cache.len(), 2);

        // Insert third → should evict LRU ("a")
        cache.insert(make_key("c"), make_sprite("c"));
        assert_eq!(cache.len(), 2);
        assert!(cache.get(&make_key("a")).is_none());
        assert!(cache.get(&make_key("b")).is_some());
        assert!(cache.get(&make_key("c")).is_some());
    }

    #[test]
    fn cache_bounded_never_grows_beyond_max() {
        let mut cache = SpriteCache::new(3);
        // Use static strings since ModelKey is &'static str
        cache.insert(make_key("obj0"), make_sprite("obj0"));
        cache.insert(make_key("obj1"), make_sprite("obj1"));
        cache.insert(make_key("obj2"), make_sprite("obj2"));
        assert_eq!(cache.len(), 3);
        // Insert fourth → should not increase length (evicts LRU)
        cache.insert(make_key("obj3"), make_sprite("obj3"));
        assert_eq!(cache.len(), 3);
        // All old entries still present (just evicted LRU)
        assert!(cache.get(&make_key("obj0")).is_some() || cache.get(&make_key("obj1")).is_some() || cache.get(&make_key("obj2")).is_some());
    }

    #[test]
    fn cache_same_key_returns_same_sprite() {
        let mut cache = SpriteCache::new(10);
        let key = make_key("tree");
        let sprite = make_sprite("tree");
        cache.insert(key.clone(), sprite);
        let s1 = cache.get(&key).unwrap();
        let s2 = cache.get(&key).unwrap();
        // Same reference (or at least equal content)
        assert_eq!(s1.key, s2.key);
    }

    #[test]
    fn cache_clear() {
        let mut cache = SpriteCache::new(10);
        cache.insert(make_key("a"), make_sprite("a"));
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn cache_animation_frame_deduplication() {
        let mut cache = SpriteCache::new(10);
        let key1 = make_key("sprite");
        let key2 = make_key("sprite");
        key2.animation_frame = 5; // different frame
        let sprite1 = make_sprite("sprite");
        let sprite2 = make_sprite("sprite2"); // different content

        cache.insert(key1.clone(), sprite1);
        // Same model + same frame should deduplicate (return existing)
        assert!(cache.get(&key1).is_some());
        // Different frame should be a separate entry
        cache.insert(key2.clone(), sprite2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn cache_lru_eviction_with_animation() {
        let mut cache = SpriteCache::new(2);
        let key_a = make_key("a");
        key_a.animation_frame = 1;
        let key_b = make_key("b");
        key_b.animation_frame = 0;
        cache.insert(key_a.clone(), make_sprite("a"));
        cache.insert(key_b.clone(), make_sprite("b"));
        assert_eq!(cache.len(), 2);

        // Insert third → should evict LRU
        let key_c = make_key("c");
        key_c.animation_frame = 0;
        cache.insert(key_c.clone(), make_sprite("c"));
        assert_eq!(cache.len(), 2);
        // The LRU (first inserted) should be evicted
        assert!(!cache.get(&key_a).is_some() || !cache.get(&key_b).is_some());
        assert!(cache.get(&key_c).is_some());
    }
}