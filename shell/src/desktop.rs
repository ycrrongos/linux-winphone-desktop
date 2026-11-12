//! Desktop personalization (wallpaper + Win10 accent palette + wallpaper theme colors).
//!
//! Accent-from-wallpaper uses Rust [`color-thief`](https://github.com/RazrFalcon/color-thief-rs),
//! a port of [lokesh/color-thief](https://github.com/lokesh/color-thief) (~13k★ JS).

use std::path::Path;

use eframe::egui::{Color32, ColorImage};
use fluent_egui::winui::FluentIcon;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperKind {
    Gradient,
    Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WallpaperPreset {
    pub id: &'static str,
    pub name: &'static str,
    pub kind: WallpaperKind,
    /// Fallback / swatch colors (also used for gradients).
    pub top: Color32,
    pub bottom: Color32,
    /// Absolute path for [`WallpaperKind::Image`].
    pub path: Option<&'static str>,
}

impl WallpaperPreset {
    pub const fn solid(id: &'static str, name: &'static str, c: Color32) -> Self {
        Self {
            id,
            name,
            kind: WallpaperKind::Gradient,
            top: c,
            bottom: c,
            path: None,
        }
    }

    pub const fn gradient(
        id: &'static str,
        name: &'static str,
        top: Color32,
        bottom: Color32,
    ) -> Self {
        Self {
            id,
            name,
            kind: WallpaperKind::Gradient,
            top,
            bottom,
            path: None,
        }
    }

    pub const fn image(
        id: &'static str,
        name: &'static str,
        path: &'static str,
        preview: Color32,
    ) -> Self {
        Self {
            id,
            name,
            kind: WallpaperKind::Image,
            top: preview,
            bottom: preview,
            path: Some(path),
        }
    }
}

/// Downscaled CPU bitmap for wallpaper painting and Color Thief.
#[derive(Clone)]
pub struct WallpaperBitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color32>,
}

impl WallpaperBitmap {
    pub fn load_image(path: &Path, max_edge: u32) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
        let img = img.thumbnail(max_edge, max_edge);
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let pixels = rgba
            .pixels()
            .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        Ok(Self {
            width: w as usize,
            height: h as usize,
            pixels,
        })
    }

    pub fn from_gradient(top: Color32, bottom: Color32, w: usize, h: usize) -> Self {
        let mut pixels = Vec::with_capacity(w * h);
        for y in 0..h {
            let t = y as f32 / (h.saturating_sub(1).max(1)) as f32;
            let c = lerp(top, bottom, t);
            for _ in 0..w {
                pixels.push(c);
            }
        }
        Self {
            width: w,
            height: h,
            pixels,
        }
    }

    pub fn color_image(&self) -> ColorImage {
        ColorImage::new([self.width, self.height], self.pixels.clone())
    }

    pub fn rgb_bytes(&self) -> Vec<u8> {
        let mut rgb = Vec::with_capacity(self.pixels.len() * 3);
        for c in &self.pixels {
            rgb.push(c.r());
            rgb.push(c.g());
            rgb.push(c.b());
        }
        rgb
    }
}

pub const WIN10UI_WALLPAPER_PATH: &str = "/home/rong/Win10UI/assets/wallpaper.jpg";

pub const WALLPAPERS: &[WallpaperPreset] = &[
    WallpaperPreset::solid("10x-sky", "10X sky", Color32::from_rgb(0x5a, 0x7e, 0xa0)),
    WallpaperPreset::image(
        "win10ui",
        "Win10UI wallpaper",
        WIN10UI_WALLPAPER_PATH,
        Color32::from_rgb(0x1a, 0x4a, 0x7a),
    ),
    WallpaperPreset::gradient(
        "win10-blue",
        "Bloom blue",
        Color32::from_rgb(0x00, 0x3b, 0x6f),
        Color32::from_rgb(0x00, 0x78, 0xd4),
    ),
    WallpaperPreset::gradient(
        "dusk",
        "Dusk",
        Color32::from_rgb(0x1a, 0x14, 0x3a),
        Color32::from_rgb(0x6b, 0x3f, 0xa0),
    ),
    WallpaperPreset::gradient(
        "forest",
        "Forest",
        Color32::from_rgb(0x0d, 0x2b, 0x1a),
        Color32::from_rgb(0x2e, 0x7d, 0x4f),
    ),
    WallpaperPreset::solid("graphite", "Graphite", Color32::from_rgb(0x2c, 0x2c, 0x2c)),
    WallpaperPreset::solid(
        "light-gray",
        "Light gray",
        Color32::from_rgb(0xe5, 0xe5, 0xe5),
    ),
    WallpaperPreset::gradient(
        "sunrise",
        "Sunrise",
        Color32::from_rgb(0xff, 0x8c, 0x42),
        Color32::from_rgb(0xff, 0xc8, 0x57),
    ),
];

/// Windows 10 Settings → Personalization → Colors style grid (~48 swatches).
pub const WIN10_WINDOWS_COLORS: &[Color32] = &[
    Color32::from_rgb(0xFF, 0xB9, 0x00),
    Color32::from_rgb(0xFF, 0x8C, 0x00),
    Color32::from_rgb(0xF7, 0x63, 0x0C),
    Color32::from_rgb(0xCA, 0x50, 0x10),
    Color32::from_rgb(0xDA, 0x3B, 0x01),
    Color32::from_rgb(0xEF, 0x69, 0x50),
    Color32::from_rgb(0xD1, 0x34, 0x38),
    Color32::from_rgb(0xFF, 0x43, 0x43),
    Color32::from_rgb(0xE7, 0x48, 0x56),
    Color32::from_rgb(0xE8, 0x11, 0x23),
    Color32::from_rgb(0xEA, 0x00, 0x5E),
    Color32::from_rgb(0xC3, 0x00, 0x52),
    Color32::from_rgb(0xE3, 0x00, 0x8C),
    Color32::from_rgb(0xBF, 0x00, 0x77),
    Color32::from_rgb(0xC2, 0x39, 0xB3),
    Color32::from_rgb(0x9A, 0x00, 0x89),
    Color32::from_rgb(0x00, 0x78, 0xD7),
    Color32::from_rgb(0x00, 0x63, 0xB1),
    Color32::from_rgb(0x8E, 0x8C, 0xD8),
    Color32::from_rgb(0x6B, 0x69, 0xD6),
    Color32::from_rgb(0x87, 0x64, 0xB8),
    Color32::from_rgb(0x74, 0x4D, 0xA9),
    Color32::from_rgb(0xB1, 0x46, 0xC2),
    Color32::from_rgb(0x88, 0x17, 0x98),
    Color32::from_rgb(0x00, 0x99, 0xBC),
    Color32::from_rgb(0x2D, 0x7D, 0x9A),
    Color32::from_rgb(0x00, 0xB7, 0xC3),
    Color32::from_rgb(0x03, 0x83, 0x87),
    Color32::from_rgb(0x00, 0xB2, 0x94),
    Color32::from_rgb(0x01, 0x85, 0x74),
    Color32::from_rgb(0x00, 0xCC, 0x6A),
    Color32::from_rgb(0x10, 0x89, 0x3E),
    Color32::from_rgb(0x7A, 0x75, 0x74),
    Color32::from_rgb(0x5D, 0x5A, 0x58),
    Color32::from_rgb(0x68, 0x76, 0x8A),
    Color32::from_rgb(0x51, 0x5C, 0x6B),
    Color32::from_rgb(0x56, 0x7C, 0x73),
    Color32::from_rgb(0x48, 0x68, 0x60),
    Color32::from_rgb(0x49, 0x82, 0x05),
    Color32::from_rgb(0x10, 0x7C, 0x10),
    Color32::from_rgb(0x76, 0x76, 0x76),
    Color32::from_rgb(0x4C, 0x4A, 0x48),
    Color32::from_rgb(0x69, 0x79, 0x7E),
    Color32::from_rgb(0x4A, 0x54, 0x59),
    Color32::from_rgb(0x64, 0x7C, 0x64),
    Color32::from_rgb(0x52, 0x5E, 0x54),
    Color32::from_rgb(0x84, 0x75, 0x45),
    Color32::from_rgb(0x7E, 0x73, 0x5F),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContrastTheme {
    pub name: &'static str,
    pub bg: Color32,
    pub text: Color32,
    pub hyperlink: Color32,
    pub disabled: Color32,
    pub selected: Color32,
    pub button: Color32,
}

pub const CONTRAST_THEMES: &[ContrastTheme] = &[
    ContrastTheme {
        name: "None (preview)",
        bg: Color32::from_rgb(0x20, 0x20, 0x20),
        text: Color32::WHITE,
        hyperlink: Color32::from_rgb(0x60, 0xcd, 0xff),
        disabled: Color32::from_rgb(0xa0, 0xa0, 0xa0),
        selected: Color32::from_rgb(0x00, 0x78, 0xd4),
        button: Color32::from_rgb(0x2c, 0x2c, 0x2c),
    },
    ContrastTheme {
        name: "Aquatic",
        bg: Color32::from_rgb(0x20, 0x20, 0x00),
        text: Color32::WHITE,
        hyperlink: Color32::from_rgb(0x75, 0xE6, 0xFF),
        disabled: Color32::from_rgb(0xA6, 0xA6, 0xA6),
        selected: Color32::from_rgb(0x00, 0x8A, 0xE2),
        button: Color32::from_rgb(0x00, 0x00, 0x00),
    },
    ContrastTheme {
        name: "Desert",
        bg: Color32::from_rgb(0xFF, 0xDE, 0xB3),
        text: Color32::from_rgb(0x3D, 0x3D, 0x3D),
        hyperlink: Color32::from_rgb(0x00, 0x6A, 0xB7),
        disabled: Color32::from_rgb(0x67, 0x67, 0x67),
        selected: Color32::from_rgb(0x90, 0x3F, 0x00),
        button: Color32::from_rgb(0xFF, 0xFC, 0xEF),
    },
    ContrastTheme {
        name: "Dusk",
        bg: Color32::from_rgb(0x2D, 0x32, 0x36),
        text: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        hyperlink: Color32::from_rgb(0x70, 0xEB, 0xDE),
        disabled: Color32::from_rgb(0xA6, 0xA6, 0xA6),
        selected: Color32::from_rgb(0xA4, 0x5A, 0x00),
        button: Color32::from_rgb(0x1A, 0x1A, 0x1A),
    },
    ContrastTheme {
        name: "Night sky",
        bg: Color32::BLACK,
        text: Color32::WHITE,
        hyperlink: Color32::from_rgb(0x80, 0x80, 0xFF),
        disabled: Color32::from_rgb(0xA6, 0xA6, 0xA6),
        selected: Color32::from_rgb(0xFF, 0xD7, 0x00),
        button: Color32::from_rgb(0xFF, 0xFF, 0xFF),
    },
];

pub fn wallpaper_by_id(id: &str) -> WallpaperPreset {
    WALLPAPERS
        .iter()
        .copied()
        .find(|w| w.id == id)
        .unwrap_or(WALLPAPERS[0])
}

pub fn resolve_bitmap(wp: WallpaperPreset) -> WallpaperBitmap {
    match wp.kind {
        WallpaperKind::Image => {
            if let Some(path) = wp.path {
                match WallpaperBitmap::load_image(Path::new(path), 1920) {
                    Ok(bmp) => return bmp,
                    Err(e) => tracing::warn!(error = %e, "wallpaper image load failed"),
                }
            }
            WallpaperBitmap::from_gradient(wp.top, wp.bottom, 64, 64)
        }
        WallpaperKind::Gradient => WallpaperBitmap::from_gradient(wp.top, wp.bottom, 64, 96),
    }
}

pub fn palette_from_bitmap(bmp: &WallpaperBitmap, max_colors: u8) -> Vec<Color32> {
    let max_colors = max_colors.clamp(2, 16);
    let rgb = bmp.rgb_bytes();
    match color_thief::get_palette(&rgb, color_thief::ColorFormat::Rgb, 8, max_colors) {
        Ok(colors) => colors
            .into_iter()
            .map(|c| Color32::from_rgb(c.r, c.g, c.b))
            .collect(),
        Err(_) => vec![Color32::from_rgb(0x00, 0x78, 0xd4)],
    }
}

pub fn accent_from_bitmap(bmp: &WallpaperBitmap) -> Color32 {
    let palette = palette_from_bitmap(bmp, 8);
    pick_accent_candidate(&palette).unwrap_or(Color32::from_rgb(0x00, 0x78, 0xd4))
}

fn pick_accent_candidate(colors: &[Color32]) -> Option<Color32> {
    let mut best: Option<(f32, Color32)> = None;
    for &c in colors {
        let r = c.r() as f32;
        let g = c.g() as f32;
        let b = c.b() as f32;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let sat = if max <= 1.0 { 0.0 } else { (max - min) / max };
        let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        if sat < 0.18 || !(28.0..=230.0).contains(&lum) {
            continue;
        }
        let score = sat * 2.0 + (1.0 - ((lum - 120.0).abs() / 120.0));
        if best.map(|(s, _)| score > s).unwrap_or(true) {
            best = Some((score, c));
        }
    }
    best.map(|(_, c)| c).or_else(|| colors.first().copied())
}

pub fn contrast_ratio(fg: Color32, bg: Color32) -> f32 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);
    let (hi, lo) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (hi + 0.05) / (lo + 0.05)
}

fn relative_luminance(c: Color32) -> f32 {
    fn chan(u: u8) -> f32 {
        let s = u as f32 / 255.0;
        if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * chan(c.r()) + 0.7152 * chan(c.g()) + 0.0722 * chan(c.b())
}

fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 { (x as f32 + (y as f32 - x as f32) * t).round() as u8 };
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

pub fn app_icon(name: &str, id: &str) -> FluentIcon {
    let key = format!("{name} {id}").to_lowercase();
    if key.contains("term") || key.contains("alacritty") || key.contains("konsole") {
        FluentIcon::Window
    } else if key.contains("browser")
        || key.contains("brave")
        || key.contains("firefox")
        || key.contains("chrome")
        || key.contains("chromium")
    {
        FluentIcon::Globe
    } else if key.contains("file") || key.contains("nautilus") || key.contains("dolphin") {
        FluentIcon::Folder
    } else if key.contains("mail") || key.contains("thunder") {
        FluentIcon::Mail
    } else if key.contains("photo") || key.contains("image") || key.contains("gimp") {
        FluentIcon::Camera
    } else if key.contains("music") || key.contains("spotify") {
        FluentIcon::Headphones
    } else if key.contains("setting") {
        FluentIcon::Settings
    } else if key.contains("code") || key.contains("editor") || key.contains("vim") {
        FluentIcon::OpenFile
    } else {
        FluentIcon::Window
    }
}
