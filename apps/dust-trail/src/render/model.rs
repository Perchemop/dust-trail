//! ObjectModel and ModelRegistry: maps game entity kinds to 3D models.

use crate::render::primitive::{Primitive, Transform, Material};
use std::collections::HashMap;

/// Stable identifier for a model (e.g., "tree_oak", "player").
pub type ModelKey = &'static str;

/// Draw ordering hint for composite (lower = drawn first/farther).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DrawKey(pub i32);

/// Bounding box in object space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: (f32, f32, f32),
    pub max: (f32, f32, f32),
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: (f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: (f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }
}

/// A named set of primitives forming one renderable object.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectModel {
    pub key: ModelKey,
    pub name: String,
    pub primitives: Vec<Primitive>,
    pub bounding_box: BoundingBox,
    pub draw_key: DrawKey,
}

/// Game entity kinds that need models.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Tree,
    Shrub,
    Cactus,
    Rock,
    Water,
    Barn,
    Player,
    Sheriff,
    Deputy,
    Outlaw,
    Loot,
    Horse,
}

impl EntityKind {
    /// Get all entity kinds that need models.
    pub fn all() -> &'static [EntityKind] {
        &[
            EntityKind::Tree,
            EntityKind::Shrub,
            EntityKind::Cactus,
            EntityKind::Rock,
            EntityKind::Water,
            EntityKind::Barn,
            EntityKind::Player,
            EntityKind::Sheriff,
            EntityKind::Deputy,
            EntityKind::Outlaw,
            EntityKind::Loot,
            EntityKind::Horse,
        ]
    }
}

/// Registry mapping game entity kinds to models.
#[derive(Debug)]
pub struct ModelRegistry {
    models: HashMap<EntityKind, ModelKey>,
    definitions: HashMap<ModelKey, ObjectModel>,
    placeholder: ObjectModel,
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelRegistry {
    pub fn new() -> Self {
        let placeholder = ObjectModel {
            key: "placeholder",
            name: "Placeholder".to_string(),
            primitives: Vec::new(),
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(0),
        };
        Self {
            models: HashMap::new(),
            definitions: HashMap::new(),
            placeholder,
        }
    }

    /// Register a model under an entity kind. Returns error if key already registered.
    pub fn register(&mut self, kind: EntityKind, model: ObjectModel) -> Result<(), String> {
        if self.models.contains_key(&kind) {
            return Err(format!("Model already registered for {:?}", kind));
        }
        let key = model.key;
        self.definitions.insert(key, model);
        self.models.insert(kind, key);
        Ok(())
    }

    /// Look up the model for an entity kind. Returns the placeholder if unknown — never panics.
    pub fn model_for(&self, kind: EntityKind) -> &ObjectModel {
        if let Some(key) = self.models.get(&kind) {
            if let Some(model) = self.definitions.get(key) {
                return model;
            }
        }
        // Unknown kind → placeholder (logs at debug level in practice)
        &self.placeholder
    }

    /// Get the placeholder model.
    pub fn placeholder(&self) -> &ObjectModel {
        &self.placeholder
    }

    /// Check if a kind is registered.
    pub fn contains(&self, kind: EntityKind) -> bool {
        self.models.contains_key(&kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_returns_placeholder_for_unknown_kind() {
        let reg = ModelRegistry::new();
        let model = reg.model_for(EntityKind::Tree); // Not registered
        assert_eq!(model.key, "placeholder");
        assert_eq!(model.primitives.len(), 0);
    }

    #[test]
    fn registry_deterministic_lookup() {
        let mut reg = ModelRegistry::new();
        let model = ObjectModel {
            key: "test_model",
            name: "Test".to_string(),
            primitives: vec![],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(0),
        };
        reg.register(EntityKind::Player, model).unwrap();

        let m1 = reg.model_for(EntityKind::Player);
        let m2 = reg.model_for(EntityKind::Player);
        assert_eq!(m1.key, m2.key);
        assert_eq!(m1.key, "test_model");
    }

    #[test]
    fn registry_registration() {
        let mut reg = ModelRegistry::new();
        let model = ObjectModel {
            key: "tree_oak",
            name: "Oak Tree".to_string(),
            primitives: vec![],
            bounding_box: BoundingBox::default(),
            draw_key: DrawKey(10),
        };
        assert!(reg.register(EntityKind::Tree, model).is_ok());
        assert!(reg.register(EntityKind::Tree, model.clone()).is_err()); // duplicate
    }
}