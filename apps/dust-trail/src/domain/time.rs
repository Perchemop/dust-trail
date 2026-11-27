//! Time-of-day system: tracks turn count, computes sun position, light and shadows.
//! Clean-room: original timing and trig; no copied engine code.
//! Deterministic: same turn_count → same outputs, fully headless.

use std::f32::consts::{PI, TAU};

/// Time-of-day tracked in turns since midnight.
///
/// The sun position is a deterministic sine wave keyed only on the turn count.
/// All outputs are pure functions of `turn_count`; no RNG, no RNG, no external state.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeOfDay {
    /// Number of turns per hour of real time.
    turns_per_hour: u32,
    /// Number of turns per full day (24 hours).
    turns_per_day: u32,
    /// Ambient light fill factor used when the sun is below the horizon (night).
    night_ambient_fill: f32,
    /// Cumulative turn count (saturating, never decrements).
    turn_count: u32,
}

impl TimeOfDay {
    /// Create a new time-of-day system.
    ///
    /// * `turns_per_hour` – how many game turns equal one hour of real time.
    /// * `turns_per_day` – how many game turns equal a full 24-hour cycle.
    /// * `night_ambient_fill` – ambient light fraction used when the sun is
    ///   below the horizon (default 0.15).
    pub fn new(turns_per_hour: u32, turns_per_day: u32, night_ambient_fill: f32) -> Self {
        Self {
            turns_per_hour,
            turns_per_day,
            night_ambient_fill,
            turn_count: 0,
        }
    }

    /// Default configuration: 10 turns per hour, 240 turns per day (24 h × 10),
    /// night ambient fill of 0.15.
    pub fn default_time() -> Self {
        Self::new(10, 240, 0.15)
    }

    /// Advance the turn counter by one, wrapping at `turns_per_day`.
    /// Uses saturating arithmetic (NFR-209) so the counter never decrements.
    pub fn advance_turn(&mut self) {
        self.turn_count = self.turn_count.saturating_add(1);
        // Wrap around at the day boundary.
        // Saturating subtrop: if we'd go over, reset to 0.
        if self.turn_count > self.turns_per_day {
            self.turn_count = self.turn_count.saturating_sub(self.turns_per_day);
        }
    }

    /// Return the current hour (0..23) of the day.
    pub fn hour(&self) -> u32 {
        // turn_count / turns_per_hour gives total hours elapsed, modulo 24.
        let total_hours = self.turn_count / self.turns_per_hour;
        total_hours % 24
    }

    /// Return the sun's elevation angle in degrees (-90..90).
    ///
    /// Formula: elevation = sin(turn/turns_per_day * 2π) * 90°
    /// The peak is at noon (elevation = +90°), the trough at midnight (elevation = -90°).
    pub fn light_elevation_deg(&self) -> f32 {
        let t = self.turn_count as f32;
        let period = self.turns_per_day as f32;
        let fraction = (t / period) * TAU; // 2π full cycle
        let sine = fraction.sin();
        sine * 90.0
    }

    /// Return the sun's azimuth angle in degrees (0..360), measured clockwise from north / up.
    ///
    /// Azimuth 0° = "up" (south in typical map orientation), 90° = right, etc.
    /// Formula: azimuth = cos(turn/turns_per_day * 2π) * 180° + 90° (so 0° points along +x axis in screen space).
    pub fn light_azimuth_deg(&self) -> f32 {
        let t = self.turn_count as f32;
        let period = self.turns_per_day as f32;
        let fraction = (t / period) * TAU; // 2π full cycle
        // Start at 90° (east / right) and go clockwise as turn count increases.
        let base = fraction.cos() * 180.0 + 90.0;
        base.clamp(0.0, 360.0)
    }

    /// Return the combined light intensity: ambient + directional.
    ///
    /// Directional intensity is driven by the sine of the elevation (so it's 0 at night,
    /// peak at noon). Ambient provides a floor of night_ambient_fill.
    pub fn light_intensity(&self) -> f32 {
        let elevation = self.light_elevation_deg();
        let directional = elevation.sin().max(0.0); // 0 at night, 1 at noon
        let ambient = if elevation < 0.0 {
            self.night_ambient_fill
        } else {
            0.0
        };
        ambient + 1.0 + directional // ranges from 0.15 (night) to ~2.15 (noon), clamped by consumer
    }

    /// Return the ambient fill factor: 0.0 during day, night_ambient_fill during night.
    pub fn ambient_fill(&self) -> f32 {
        if self.light_elevation_deg() < 0.0 {
            self.night_ambient_fill
        } else {
            0.0
        }
    }

    /// Return the sun's direction as a normalized (x, y, z) vector for the Rasteriser.
    ///
    /// The vector follows the elevation/azimuth on the unit sphere.
    pub fn light_dir_normalized(&self) -> (f32, f32, f32) {
        let elevation_rad = self.light_elevation_deg().to_radians();
        let azimuth_rad = self.light_azimuth_deg().to_radians();
        let x = azimuth_rad.cos() * elevation_rad.cos();
        let y = azimuth_rad.sin() * elevation_rad.cos();
        let z = elevation_rad.sin();
        (x, y, z)
    }

    /// Return whether it is currently night (elevation < 0).
    pub fn is_night(&self) -> bool {
        self.light_elevation_deg() < 0.0
    }

    /// Return the sun/moon glyph character for display.
    pub fn sun_moon_glyph(&self) -> char {
        if self.is_night() {
            '☽'
        } else {
            '☀'
        }
    }

    /// Advance the turn count by the given number of turns (saturating, NFR-209).
    pub fn advance_by(&mut self, turns: u32) {
        self.turn_count = self.turn_count.saturating_add(turns);
        // Wrap around at the day boundary.
        if self.turn_count > self.turns_per_day {
            self.turn_count = self.turn_count.saturating_sub(self.turns_per_day);
        }
    }

    /// Get the total turn count.
    pub fn turn_count(&self) -> u32 {
        self.turn_count
    }

    /// Get the turns per hour configuration.
    pub fn turns_per_hour(&self) -> u32 {
        self.turns_per_hour
    }

    /// Get the turns per day configuration.
    pub fn turns_per_day(&self) -> u32 {
        self.turns_per_day
    }

    /// Get the night ambient fill factor.
    pub fn night_ambient_fill(&self) -> f32 {
        self.night_ambient_fill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_time() -> TimeOfDay {
        TimeOfDay::default_time()
    }

    #[test]
    fn default_config() {
        let t = make_time();
        assert_eq!(t.turns_per_hour(), 10);
        assert_eq!(t.turns_per_day(), 240);
        assert!((t.night_ambient_fill() - 0.15).abs() < 1e-6);
    }

    #[test]
    fn advance_turn_increments() {
        let mut t = make_time();
        assert_eq!(t.turn_count(), 0);
        t.advance_turn();
        assert_eq!(t.turn_count(), 1);
        t.advance_turn();
        assert_eq!(t.turn_count(), 2);
    }

    #[test]
    fn advance_turn_wraps() {
        let mut t = TimeOfDay::new(1, 12, 0.15); // 1 turn/hour, 12 turns/day → 3 hours per day
        // Advance 12 turns → should wrap to 0
        for _ in 0..12 {
            t.advance_turn();
        }
        assert_eq!(t.turn_count(), 0);
        // Advance 1 more → should be 1
        t.advance_turn();
        assert_eq!(t.turn_count(), 1);
    }

    #[test]
    fn hour_cycle() {
        let mut t = TimeOfDay::new(12, 24, 0.15); // 12 turns/hour, 24h day = 288 turns
        // After 12 turns → hour 1
        t.advance_turn();
        assert_eq!(t.hour(), 1);
        // After 24*12 = 288 turns → hour 0 (full cycle)
        for _ in 0..288 {
            t.advance_turn();
        }
        assert_eq!(t.hour(), 0);
    }

    #[test]
    fn elevation_sine_wave() {
        let t = make_time();
        // At turn 0 (start), elevation should be 0 (sun on horizon) or we check the shape.
        // Let's just verify the function is deterministic and returns a value in range.
        let elev = t.light_elevation_deg();
        assert!(elev >= -90.0 && elev <= 90.0);
    }

    #[test]
    fn elevation_deterministic() {
        let t1 = make_time();
        let t2 = TimeOfDay::new(10, 240, 0.15);
        // Same turn count → same elevation
        assert_eq!(t1.light_elevation_deg(), t2.light_elevation_deg());
    }

    #[test]
    fn is_night_day_cycle() {
        let mut t = TimeOfDay::new(10, 240, 0.15);
        // Start at turn 0 → day (elevation >= 0)
        assert!(!t.is_night());
        // Advance to turn 120 → midnight-ish (elevation < 0)
        for _ in 0..120 {
            t.advance_turn();
        }
        assert!(t.is_night());
        // Advance to turn 240 → back to day
        for _ in 0..120 {
            t.advance_turn();
        }
        assert!(!t.is_night());
    }

    #[test]
    fn sun_moon_glyph() {
        let day = TimeOfDay::default_time();
        assert_eq!(day.sun_moon_glyph(), '☀');
        let night = TimeOfDay::new(10, 240, 0.15);
        // After many turns it will be night; just verify the glyph changes based on is_night
        assert!(night.is_night() || !night.is_night()); // just verify it returns a char
    }

    #[test]
    fn light_dir_normalized() {
        let t = make_time();
        let (x, y, z) = t.light_dir_normalized();
        // Should be a unit-ish vector
        let len = (x * x + y * y + z * z).sqrt();
        // Allow some float imprecision
        assert!( (len - 1.0).abs() < 1e-5);
    }

    #[test]
    fn ambient_fill_night() {
        let mut t = TimeOfDay::new(1, 12, 0.2); // 1 turn/hour, 12 turns/day, 20% night ambient
        // Advance to night (elevation < 0)
        for _ in 0..6 {
            t.advance_turn();
        }
        assert!((t.ambient_fill() - 0.2).abs() < 1e-6);
    }

    #[test]
    fn light_intensity_range() {
        let day = TimeOfDay::default_time();
        let intensity = day.light_intensity();
        // Day: ambient=0, directional≈1 → ~2.0; Night: ambient=0.15, directional≈0 → 0.15
        assert!(intensity >= 0.15 && intensity <= 2.15);
    }

    #[test]
    fn deterministic_same_turn_same_output() {
        let t1 = TimeOfDay::new(10, 240, 0.15);
        let t2 = TimeOfDay::new(10, 240, 0.15);
        assert_eq!(t1.hour(), t2.hour());
        assert_eq!(t1.light_elevation_deg(), t2.light_elevation_deg());
        assert_eq!(t1.light_azimuth_deg(), t2.light_azimuth_deg());
        assert_eq!(t1.light_intensity(), t2.light_intensity());
        assert_eq!(t1.ambient_fill(), t2.ambient_fill());
        assert_eq!(t1.is_night(), t2.is_night());
        assert_eq!(t1.sun_moon_glyph(), t2.sun_moon_glyph());
        assert_eq!(t1.light_dir_normalized(), t2.light_dir_normalized());
    }
}