//! Theme module — single source of truth for colours, spacing and text styles.
//! Widget code reads semantic roles; no scattered `Color32::from_*` literals
//! appear outside this module (guide II.30.6, enforced by a CI grep gate).

use eframe::egui;

/// Semantic colour role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    // Surfaces
    SurfaceBase,
    SurfaceRaised,
    SurfaceSunken,
    // Text
    TextPrimary,
    TextSecondary,
    TextMuted,
    TextDisabled,
    // Accent
    Accent,
    AccentHover,
    AccentOn,
    // State
    Danger,
    DangerOn,
    Warning,
    Success,
    // Borders
    BorderSubtle,
    BorderStrong,
    // Focus
    FocusRing,
    // Overlay
    OverlayScrim,
    // Selection
    SelectionBg,
    SelectionText,
}

/// A theme variant: light, dark, or high contrast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Light,
    Dark,
    HighContrast,
}

/// The theme: maps semantic roles to egui colours for a variant.
#[derive(Debug, Clone)]
pub struct Theme {
    pub variant: Variant,
}

impl Theme {
    pub fn dark() -> Self {
        Self { variant: Variant::Dark }
    }

    pub fn light() -> Self {
        Self { variant: Variant::Light }
    }

    pub fn high_contrast() -> Self {
        Self {
            variant: Variant::HighContrast,
        }
    }

    /// Resolve a role to an egui colour for the current variant.
    pub fn color32(&self, role: Role) -> egui::Color32 {
        egui::Color32::from_rgb(self.rgb(role).r, self.rgb(role).g, self.rgb(role).b)
    }

    /// Resolve a role to a domain RGB (shared with the TUI).
    pub fn rgb(&self, role: Role) -> crate::domain::color::Rgb {
        match self.variant {
            Variant::Dark => self.dark_rgb(role),
            Variant::Light => self.light_rgb(role),
            Variant::HighContrast => self.high_contrast_rgb(role),
        }
    }

    fn dark_rgb(&self, role: Role) -> crate::domain::color::Rgb {
        use crate::domain::color::Rgb;
        match role {
            Role::SurfaceBase => Rgb::new(28, 28, 30),
            Role::SurfaceRaised => Rgb::new(42, 42, 46),
            Role::SurfaceSunken => Rgb::new(20, 20, 22),
            Role::TextPrimary => Rgb::WHITE,
            Role::TextSecondary => Rgb::new(190, 190, 200),
            Role::TextMuted => Rgb::new(120, 120, 128),
            Role::TextDisabled => Rgb::new(90, 90, 96),
            Role::Accent => Rgb::new(90, 160, 250),
            Role::AccentHover => Rgb::new(110, 180, 255),
            Role::AccentOn => Rgb::WHITE,
            Role::Danger => Rgb::new(220, 60, 60),
            Role::DangerOn => Rgb::WHITE,
            Role::Warning => Rgb::new(230, 170, 60),
            Role::Success => Rgb::new(80, 200, 120),
            Role::BorderSubtle => Rgb::new(70, 70, 80),
            Role::BorderStrong => Rgb::new(110, 110, 120),
            Role::FocusRing => Rgb::new(90, 160, 250),
            Role::OverlayScrim => Rgb::new(0, 0, 0),
            Role::SelectionBg => Rgb::new(90, 160, 250),
            Role::SelectionText => Rgb::WHITE,
        }
    }

    fn light_rgb(&self, role: Role) -> crate::domain::color::Rgb {
        use crate::domain::color::Rgb;
        match role {
            Role::SurfaceBase => Rgb::new(245, 245, 247),
            Role::SurfaceRaised => Rgb::WHITE,
            Role::SurfaceSunken => Rgb::new(230, 230, 232),
            Role::TextPrimary => Rgb::new(30, 30, 34),
            Role::TextSecondary => Rgb::new(90, 90, 100),
            Role::TextMuted => Rgb::new(140, 140, 150),
            Role::TextDisabled => Rgb::new(170, 170, 180),
            Role::Accent => Rgb::new(20, 110, 220),
            Role::AccentHover => Rgb::new(40, 130, 240),
            Role::AccentOn => Rgb::WHITE,
            Role::Danger => Rgb::new(190, 40, 40),
            Role::DangerOn => Rgb::WHITE,
            Role::Warning => Rgb::new(200, 130, 30),
            Role::Success => Rgb::new(40, 160, 90),
            Role::BorderSubtle => Rgb::new(210, 210, 218),
            Role::BorderStrong => Rgb::new(170, 170, 180),
            Role::FocusRing => Rgb::new(20, 110, 220),
            Role::OverlayScrim => Rgb::new(0, 0, 0),
            Role::SelectionBg => Rgb::new(20, 110, 220),
            Role::SelectionText => Rgb::WHITE,
        }
    }

    fn high_contrast_rgb(&self, role: Role) -> crate::domain::color::Rgb {
        use crate::domain::color::Rgb;
        match role {
            Role::SurfaceBase => Rgb::BLACK,
            Role::SurfaceRaised => Rgb::BLACK,
            Role::SurfaceSunken => Rgb::BLACK,
            Role::TextPrimary => Rgb::WHITE,
            Role::TextSecondary => Rgb::WHITE,
            Role::TextMuted => Rgb::WHITE,
            Role::TextDisabled => Rgb::new(120, 120, 120),
            Role::Accent => Rgb::new(0, 120, 255),
            Role::AccentHover => Rgb::new(40, 150, 255),
            Role::AccentOn => Rgb::WHITE,
            Role::Danger => Rgb::new(255, 0, 0),
            Role::DangerOn => Rgb::WHITE,
            Role::Warning => Rgb::new(255, 255, 0),
            Role::Success => Rgb::new(0, 255, 0),
            Role::BorderSubtle => Rgb::WHITE,
            Role::BorderStrong => Rgb::WHITE,
            Role::FocusRing => Rgb::new(255, 255, 0),
            Role::OverlayScrim => Rgb::BLACK,
            Role::SelectionBg => Rgb::new(0, 120, 255),
            Role::SelectionText => Rgb::WHITE,
        }
    }

    /// Build `egui::Visuals` from this theme (panel fills, selection, etc.).
    pub fn visuals(&self) -> egui::Visuals {
        let base = self.color32(Role::SurfaceBase);
        let raised = self.color32(Role::SurfaceRaised);
        let text = self.color32(Role::TextPrimary);
        let accent = self.color32(Role::Accent);
        egui::Visuals {
            panel_fill: base,
            window_fill: base,
            extreme_bg_color: base,
            faint_bg_color: raised,
            override_text_color: Some(text),
            selection_bg_fill: self.color32(Role::SelectionBg),
            selection_stroke: egui::Stroke::new(1.0, self.color32(Role::SelectionText)),
            hyperlink_color: accent,
            warn_fg_color: self.color32(Role::Warning),
            error_fg_color: self.color32(Role::Danger),
            ..Default::default()
        }
    }
}

/// Convert a domain RGB to an egui::Color32. This is the one allowed
/// `Color32::from_rgb` call site (guide II.30.6 token gate); all other widget
/// colour use goes through `Theme::color32(role)`.
pub fn rgb_to_color32(col: crate::domain::color::Rgb) -> egui::Color32 {
    egui::Color32::from_rgb(col.r, col.g, col.b)
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Relative luminance of an RGB colour (sRGB linearised), per WCAG.
fn luminance(c: crate::domain::color::Rgb) -> f64 {
    let lin = |ch: u8| -> f64 {
        let x = ch as f64 / 255.0;
        if x <= 0.03928 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b)
}

/// Contrast ratio between two colours (lighter over darker), per WCAG 2.2.
pub fn contrast(fg: crate::domain::color::Rgb, bg: crate::domain::color::Rgb) -> f64 {
    let l1 = luminance(fg);
    let l2 = luminance(bg);
    let (light, dark) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (light + 0.05) / (dark + 0.05)
}

impl Theme {
    /// The foreground role to pair with a surface role for body text.
    pub fn body_on(role: Role) -> (Role, Role) {
        match role {
            Role::SurfaceBase => (Role::TextPrimary, Role::SurfaceBase),
            Role::SurfaceRaised => (Role::TextPrimary, Role::SurfaceRaised),
            Role::SurfaceSunken => (Role::TextPrimary, Role::SurfaceSunken),
            _ => (Role::TextPrimary, Role::SurfaceBase),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_is_symmetric() {
        let a = crate::domain::color::Rgb::WHITE;
        let b = crate::domain::color::Rgb::BLACK;
        assert!((contrast(a, b) - contrast(b, a)).abs() < f64::EPSILON);
    }

    #[test]
    fn black_on_white_is_max() {
        let black = crate::domain::color::Rgb::BLACK;
        let white = crate::domain::color::Rgb::WHITE;
        assert!((contrast(black, white) - 21.0).abs() < 0.01);
    }

    #[test]
    fn rgb_to_color32_roundtrip() {
        let col = crate::domain::color::Rgb::new(10, 20, 30);
        let c = rgb_to_color32(col);
        assert_eq!((c.r(), c.g(), c.b()), (10, 20, 30));
    }

    /// WCAG AA: 4.5:1 for body text, 3:1 for UI boundaries / focus rings,
    /// across light, dark and high-contrast variants (II.30.7.1).
    #[test]
    fn contrast_passes_aa_all_variants() {
        for variant in [Variant::Light, Variant::Dark, Variant::HighContrast] {
            let theme = match variant {
                Variant::Light => Theme::light(),
                Variant::Dark => Theme::dark(),
                Variant::HighContrast => Theme::high_contrast(),
            };

            // Body text: text.primary on surface.base >= 4.5:1
            let (fg, bg) = Theme::body_on(Role::SurfaceBase);
            let t = contrast(theme.rgb(fg), theme.rgb(bg));
            assert!(t >= 4.5, "{variant:?} body text contrast {t:.2} < 4.5");

            // UI boundary: border.strong on surface.base >= 3:1
            let t = contrast(theme.rgb(Role::BorderStrong), theme.rgb(Role::SurfaceBase));
            assert!(t >= 3.0, "{variant:?} border contrast {t:.2} < 3.0");

            // Focus ring >= 3:1 against surface
            let t = contrast(theme.rgb(Role::FocusRing), theme.rgb(Role::SurfaceBase));
            assert!(t >= 3.0, "{variant:?} focus ring contrast {t:.2} < 3.0");

            // Accent text (accent.on on accent) >= 4.5:1
            let t = contrast(theme.rgb(Role::AccentOn), theme.rgb(Role::Accent));
            assert!(t >= 4.5, "{variant:?} accent-on contrast {t:.2} < 4.5");

            // Danger on surface >= 3:1 (never colour-only signal)
            let t = contrast(theme.rgb(Role::Danger), theme.rgb(Role::SurfaceBase));
            assert!(t >= 3.0, "{variant:?} danger contrast {t:.2} < 3.0");
        }
    }
}
