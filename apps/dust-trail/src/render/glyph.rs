//! Glyph ramp: luminance → character mapping.

/// Glyph ramp: 10 characters from darkest to brightest.
/// Per spec Q1: owner recommends 10-char ramp for better shading gradation.
pub const GLYPH_RAMP: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/// Convert luminance (0.0 = darkest, 1.0 = brightest) to a glyph index.
/// Clamped to valid ramp range.
pub fn luminance_to_glyph(luminance: f32) -> char {
    let clamped = luminance.clamp(0.0, 1.0);
    let index = (clamped * (GLYPH_RAMP.len() - 1) as f32).round() as usize;
    GLYPH_RAMP[index.min(GLYPH_RAMP.len() - 1)]
}

/// Convert luminance to glyph index (0..ramp_len-1).
pub fn luminance_to_index(luminance: f32) -> usize {
    let clamped = luminance.clamp(0.0, 1.0);
    let index = (clamped * (GLYPH_RAMP.len() - 1) as f32).round() as usize;
    index.min(GLYPH_RAMP.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_ramp_has_at_least_8_distinct_chars() {
        assert!(GLYPH_RAMP.len() >= 8);
        let unique: std::collections::HashSet<char> = GLYPH_RAMP.iter().copied().collect();
        assert_eq!(unique.len(), GLYPH_RAMP.len()); // all distinct
    }

    #[test]
    fn endpoints_map_to_first_and_last_char() {
        assert_eq!(luminance_to_glyph(0.0), ' ');
        assert_eq!(luminance_to_glyph(1.0), '@');
    }

    #[test]
    fn monotonic_mapping() {
        let mut prev_idx = 0;
        for i in 0..=10 {
            let lum = i as f32 / 10.0;
            let idx = luminance_to_index(lum);
            assert!(idx >= prev_idx, "luminance {} -> index {} < prev {}", lum, idx, prev_idx);
            prev_idx = idx;
        }
    }

    #[test]
    fn spans_at_least_8_distinct_chars() {
        let mut seen = std::collections::HashSet::new();
        for i in 0..=20 {
            let lum = i as f32 / 20.0;
            seen.insert(luminance_to_glyph(lum));
        }
        assert!(seen.len() >= 8);
    }

    #[test]
    fn clamp_out_of_range() {
        assert_eq!(luminance_to_glyph(-1.0), ' ');
        assert_eq!(luminance_to_glyph(2.0), '@');
    }
}