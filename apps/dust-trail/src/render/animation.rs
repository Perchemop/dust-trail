//! Animation system: frame-timer, keyframe interpolation, per-entity animation state.
//! Clean-room: original types and timing; no copied game-engine code.

use crate::render::primitive::{Primitive, PrimitiveKind, Transform};

/// A single animation frame with a transform delta applied to a primitive.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimFrame {
    /// Frame index within the animation sequence.
    pub frame_index: u32,
    /// Transform delta applied to the primitive for this frame.
    pub transform_delta: Transform,
    /// Whether this is the last frame (for Loop/Once/PingPong logic).
    pub is_last: bool,
}

/// Key type for looking up an animation by entity kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnimKey {
    Walk,
    Idle,
    Attack,
    Death,
}

/// Playback mode for the animation controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopMode {
    /// Loop continuously; wraps around at the end.
    Loop,
    /// Play once and stop on the last frame.
    Once,
    /// Play forward then reverse back to start (1 frame = static reversal).
    PingPong,
}

/// A time-step count, used for fixed-point frame advancement
/// (integer ticks per frame; no floating-point math).
type FrameTick = u32;

/// Holds the current animation state for a single entity.
#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct Animation {
    /// The key that identifies which animation this is.
    pub key: AnimKey,
    /// Current frame index (0-based).
    pub current_frame: u32,
    /// Loop mode controlling playback behaviour.
    pub mode: LoopMode,
    /// Number of ticks elapsed since the last frame advance.
    pub tick_accumulator: FrameTick,
    /// Fixed ticks required to advance one frame.
    pub frame_duration: FrameTick,
    /// Total number of frames in this animation (0 = static / no animation).
    pub total_frames: u32,
}

impl Animation {
    /// Create a new animation with the given key and configuration.
    pub fn new(key: AnimKey, frame_duration: FrameTick, total_frames: u32, mode: LoopMode) -> Self {
        Self {
            key,
            current_frame: 0,
            mode,
            tick_accumulator: 0,
            frame_duration,
            total_frames,
        }
    }
}

/// Advances the animation state by the given number of ticks.
/// Uses pure integer arithmetic; no floating-point.
pub fn tick(anim: Animation, ticks: FrameTick) -> Animation {
    let new_tick = anim.tick_accumulator.saturating_add(ticks);
    let frame_advances = new_tick / anim.frame_duration;
    let remaining_tick = new_tick % anim.frame_duration;

    let new_frame = anim.current_frame + frame_advances;
    let (new_mode, new_current_frame) = match anim.mode {
        LoopMode::Loop => {
            let wrapped = new_frame % anim.total_frames.max(1);
            (LoopMode::Loop, wrapped)
        }
        LoopMode::Once => {
            let capped = new_frame.min(anim.total_frames.saturating_sub(1));
            (LoopMode::Once, capped)
        }
        LoopMode::PingPong => {
            // Ping-pong: forward then reverse. Full cycle = 2 * (N-1) steps for N>1.
            let period = if anim.total_frames > 1 {
                2 * (anim.total_frames - 1)
            } else {
                1
            };
            let pegged = new_frame % period;
            let frame = if pegged < anim.total_frames - 1 {
                pegged
            } else {
                (period - 1) - pegged
            };
            (LoopMode::PingPong, frame)
        }
    };

    Animation {
        key: anim.key,
        current_frame: new_current_frame,
        mode: new_mode,
        tick_accumulator: remaining_tick,
        frame_duration: anim.frame_duration,
        total_frames: anim.total_frames,
    }
}

/// Set the animation to play, resetting to the first frame.
pub fn set_animation(mut anim: Animation, key: AnimKey, frame_duration: FrameTick, total_frames: u32, mode: LoopMode) -> Animation {
    anim.key = key;
    anim.current_frame = 0;
    anim.mode = mode;
    anim.tick_accumulator = 0;
    anim.frame_duration = frame_duration;
    anim.total_frames = total_frames;
    anim
}

/// Returns the current frame's primitive transforms for the given controller.
/// If the animation key is unknown / no animation, returns base (identity) transforms.
pub fn get_primitive_transforms(
    anim: &Animation,
    controller: &AnimationController,
    entity_transform: &Transform,
) -> Vec<Primitive> {
    // If there's no effective animation, return the entity's base transform.
    if anim.current_frame == 0 && anim.mode != LoopMode::PingPong {
        return vec![Primitive {
            kind: PrimitiveKind::Box,
            transform: *entity_transform,
            material: crate::render::primitive::Material::default(),
        }];
    }

    // Delegate to the controller for per-frame transform computation.
    controller.get_primitive_transforms(anim, entity_transform)
}

/// -------------------------------------------------------------------
// AnimationController: controls playback of an Animation for one entity.
// -------------------------------------------------------------------

/// The controller owns the animation state and provides tick / query operations.
#[derive(Clone, Debug)]
pub struct AnimationController {
    /// Per-entity animation state.
    animation: Animation,
}

impl AnimationController {
    /// Create a new controller with the given animation.
    pub fn new(animation: Animation) -> Self {
        Self { animation }
    }

    /// Set a new animation, resetting playback.
    pub fn set_animation(&mut self, key: AnimKey, frame_duration: u32, total_frames: u32, mode: LoopMode) {
        self.animation = crate::render::animation::set_animation(
            self.animation.clone(),
            key,
            frame_duration,
            total_frames,
            mode,
        );
    }

    /// Advance the animation by the given tick count and return the updated controller.
    pub fn tick(&mut self, ticks: u32) {
        self.animation = crate::render::animation::tick(self.animation.clone(), ticks);
    }

    /// Returns the current frame index (0-based).
    pub fn current_frame(&self) -> u32 {
        self.animation.current_frame
    }

    /// Returns whether an animation is currently playing (non-static).
    pub fn is_animating(&self) -> bool {
        // Animate if total_frames > 1 and we are not in a static state.
        self.animation.total_frames > 1 && self.animation.current_frame > 0
    }

    /// Returns the primitive transforms for the current frame.
    /// Unknown `AnimKey` falls back to base transforms (no animation).
    pub fn get_primitive_transforms(
        &self,
        entity_transform: &crate::render::primitive::Transform,
    ) -> Vec<crate::render::primitive::Primitive> {
        crate::render::animation::get_primitive_transforms(
            &self.animation,
            self,
            entity_transform,
        )
    }
}

// -------------------------------------------------------------------
// PrimitiveDelta: a minimal transform diff used when keying animated
// cache entries or computing per-frame geometry.
// -------------------------------------------------------------------

/// A transform delta applied to a primitive during one animation frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PrimitiveDelta {
    /// Translation offset (x, y, z) in local object space.
    pub translation: (i16, i16, i16),
    /// Rotation offset (yaw, pitch, roll) in half-ticks (1/64 rad).
    pub rotation: (i16, i16, i16),
    /// Scale factor applied to each axis.
    pub scale: (u8, u8, u8),
}

impl Default for PrimitiveDelta {
    fn default() -> Self {
        Self {
            translation: (0, 0, 0),
            rotation: (0, 0, 0),
            scale: (1, 1, 1),
        }
    }
}

// -------------------------------------------------------------------
// Tests (headless, no GUI dependency)
// -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animation_tick_loop() {
        let mut anim = Animation::new(AnimKey::Walk, 4, 4, LoopMode::Loop);
        assert_eq!(anim.current_frame, 0);
        assert_eq!(anim.tick_accumulator, 0);

        // Advance 4 ticks → exactly 1 frame
        anim = tick(anim, 4);
        assert_eq!(anim.current_frame, 1);
        assert_eq!(anim.tick_accumulator, 0);

        // Advance 2 more ticks → half a frame stays in accumulator
        anim = tick(anim, 2);
        assert_eq!(anim.current_frame, 1);
        assert_eq!(anim.tick_accumulator, 2);
    }

    #[test]
    fn animation_tick_once_stops() {
        let mut anim = Animation::new(AnimKey::Attack, 2, 3, LoopMode::Once);
        // 6 ticks → exactly 3 frames, then stops
        anim = tick(anim, 6);
        assert_eq!(anim.current_frame, 2);
        assert_eq!(anim.mode, LoopMode::Once);
    }

    #[test]
    fn animation_tick_pingpong() {
        let mut anim = Animation::new(AnimKey::Death, 2, 4, LoopMode::PingPong);
        // 6 ticks in a 4-frame ping-pong: period=6, frame=0,1,2,1
        anim = tick(anim, 6);
        assert_eq!(anim.current_frame, 1); // Ping-pong: 0→1→2→1→0...
    }

    #[test]
    fn unknown_anim_key_returns_base() {
        let anim = Animation::new(AnimKey::Walk, 1, 4, LoopMode::Loop);
        let ctrl = AnimationController::new(anim);
        let transforms = ctrl.get_primitive_transforms(
            &Transform::default(),
            &ctrl,
        );
        // Unknown/should return base; at minimum one primitive.
        assert!(!transforms.is_empty());
        assert_eq!(transforms[0].kind, PrimitiveKind::Box);
    }

    #[test]
    fn deterministic_same_turn_same_frame() {
        // Two controllers with same tick counts should produce identical frames.
        let mut a = AnimationController::new(Animation::new(AnimKey::Walk, 5, 4, LoopMode::Loop));
        let mut b = AnimationController::new(Animation::new(AnimKey::Walk, 5, 4, LoopMode::Loop));

        a.tick(10);
        b.tick(10);

        assert_eq!(a.current_frame(), b.current_frame());
        assert_eq!(a.is_animating(), b.is_animating());
    }
}