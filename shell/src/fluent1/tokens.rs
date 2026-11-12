//! Tokens mirrored from `/home/rong/Win10UI/src/tokens/*.css`.
#![allow(dead_code)] // design-system surface: not every token is wired yet

use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Fluent1Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub accent: Color32,
    pub accent_light: Color32,
    pub accent_dark: Color32,
    pub accent_text: Color32,

    pub bg: Color32,
    pub bg_alt: Color32,
    pub bg_smoke: Color32,
    pub bg_card: Color32,
    pub chrome: Color32,

    pub surface: Color32,
    pub surface_strong: Color32,
    pub surface_taskbar: Color32,

    pub text: Color32,
    pub text_secondary: Color32,
    pub text_tertiary: Color32,
    pub text_disabled: Color32,

    pub control: Color32,
    pub control_secondary: Color32,
    pub control_tertiary: Color32,

    pub stroke: Color32,
    pub stroke_card: Color32,
    pub stroke_divider: Color32,

    pub success: Color32,
    pub warning: Color32,
    pub critical: Color32,

    pub reveal: Color32,
    pub reveal_strong: Color32,
    pub overlay: Color32,
    pub shadow: Color32,

    /// Win10 / Fluent 1: rectangular chrome is square (0). Circles stay circular.
    pub radius_chrome: u8,
    pub radius_panel: u8,
}

impl Palette {
    pub fn for_theme(theme: Fluent1Theme) -> Self {
        match theme {
            Fluent1Theme::Light => Self::light(),
            Fluent1Theme::Dark => Self::dark(),
        }
    }

    /// Rebuild base theme palette then override system accent (and derived tones).
    pub fn for_theme_accent(theme: Fluent1Theme, accent: Color32) -> Self {
        let mut p = Self::for_theme(theme);
        p.apply_accent(accent);
        p
    }

    pub fn apply_accent(&mut self, accent: Color32) {
        self.accent = accent;
        self.accent_light = mix(accent, Color32::WHITE, 0.28);
        self.accent_dark = mix(accent, Color32::BLACK, 0.22);
        // Prefer white text on mid/dark accents; black on very light ones.
        let lum =
            0.2126 * accent.r() as f32 + 0.7152 * accent.g() as f32 + 0.0722 * accent.b() as f32;
        self.accent_text = if lum > 160.0 {
            Color32::BLACK
        } else {
            Color32::WHITE
        };
    }

    /// From Win10UI `[data-theme="light"]`.
    pub fn light() -> Self {
        Self {
            accent: rgb(0x00, 0x78, 0xd4),
            accent_light: rgb(0x42, 0x9c, 0xe3),
            accent_dark: rgb(0x00, 0x5a, 0x9e),
            accent_text: Color32::WHITE,

            bg: rgb(0xff, 0xff, 0xff),
            bg_alt: rgb(0xf3, 0xf3, 0xf3),
            bg_smoke: rgb(0xf9, 0xf9, 0xf9),
            bg_card: rgb(0xff, 0xff, 0xff),
            chrome: rgb(0xf3, 0xf3, 0xf3),

            surface: rgb(0xf3, 0xf3, 0xf3),
            surface_strong: rgb(0xf3, 0xf3, 0xf3),
            surface_taskbar: rgb(0xff, 0xff, 0xff),

            text: rgb(0x1a, 0x1a, 0x1a),
            text_secondary: rgb(0x5d, 0x5d, 0x5d),
            text_tertiary: rgb(0x8a, 0x8a, 0x8a),
            text_disabled: rgb(0xa1, 0x9f, 0x9d),

            control: Color32::WHITE,
            control_secondary: rgb(0xf9, 0xf9, 0xf9),
            control_tertiary: rgb(0xf3, 0xf3, 0xf3),

            stroke: rgb(0x8a, 0x8a, 0x8a),
            stroke_card: rgb(0xe5, 0xe5, 0xe5),
            stroke_divider: rgb(0xe5, 0xe5, 0xe5),

            success: rgb(0x0f, 0x7b, 0x0f),
            warning: rgb(0x9d, 0x5d, 0x00),
            critical: rgb(0xc4, 0x2b, 0x1c),

            reveal: rgba(0, 0, 0, 15),
            reveal_strong: rgba(0, 0, 0, 26),
            overlay: rgba(0, 0, 0, 77),
            shadow: rgba(0, 0, 0, 36),

            radius_chrome: 0,
            radius_panel: 0,
        }
    }

    /// From Win10UI `[data-theme="dark"]`.
    pub fn dark() -> Self {
        Self {
            accent: rgb(0x60, 0xcd, 0xff),
            accent_light: rgb(0x99, 0xeb, 0xff),
            accent_dark: rgb(0x00, 0x78, 0xd4),
            accent_text: Color32::BLACK,

            bg: rgb(0x20, 0x20, 0x20),
            bg_alt: rgb(0x2c, 0x2c, 0x2c),
            bg_smoke: rgb(0x1c, 0x1c, 0x1c),
            bg_card: rgb(0x2c, 0x2c, 0x2c),
            chrome: rgb(0x2c, 0x2c, 0x2c),

            surface: rgb(0x20, 0x20, 0x20),
            surface_strong: rgb(0x20, 0x20, 0x20),
            surface_taskbar: rgb(0x2c, 0x2c, 0x2c),

            text: Color32::WHITE,
            text_secondary: rgb(0xcf, 0xcf, 0xcf),
            text_tertiary: rgb(0x9a, 0x9a, 0x9a),
            text_disabled: rgb(0x71, 0x71, 0x71),

            control: rgb(0x2c, 0x2c, 0x2c),
            control_secondary: rgb(0x32, 0x32, 0x32),
            control_tertiary: rgb(0x3a, 0x3a, 0x3a),

            stroke: rgb(0x9a, 0x9a, 0x9a),
            stroke_card: rgb(0x3a, 0x3a, 0x3a),
            stroke_divider: rgb(0x3a, 0x3a, 0x3a),

            success: rgb(0x6c, 0xcb, 0x5f),
            warning: rgb(0xfc, 0xe1, 0x00),
            critical: rgb(0xff, 0x99, 0xa4),

            reveal: rgba(255, 255, 255, 20),
            reveal_strong: rgba(255, 255, 255, 31),
            overlay: rgba(0, 0, 0, 140),
            shadow: rgba(0, 0, 0, 102),

            radius_chrome: 0,
            radius_panel: 0,
        }
    }
}

fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 { (x as f32 + (y as f32 - x as f32) * t).round() as u8 };
    Color32::from_rgba_unmultiplied(
        lerp(a.r(), b.r()),
        lerp(a.g(), b.g()),
        lerp(a.b(), b.b()),
        lerp(a.a(), b.a()),
    )
}

/// Type sizes from Win10UI typography.css (px).
pub mod type_size {
    pub const CAPTION: f32 = 12.0;
    pub const BODY: f32 = 14.0;
    pub const SUBTITLE: f32 = 20.0;
    pub const TITLE: f32 = 28.0;
    pub const TITLE_LARGE: f32 = 40.0;
}

/// Spacing from Win10UI spacing.css.
pub mod space {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
}
