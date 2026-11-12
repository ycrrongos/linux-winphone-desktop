//! Opaque Fluent 1 shell surfaces.
//!
//! These painters intentionally contain no backdrop sampling, blur shader,
//! translucency, or noise layer. They are the neutral baseline for the next
//! material implementation.

use eframe::egui::{self, Color32, CornerRadius, Stroke, StrokeKind, Ui};

use super::tokens::Palette;

pub fn paint_surface(ui: &mut Ui, rect: egui::Rect, pal: &Palette) {
    paint_flat(ui, rect, pal.surface, pal);
}

pub fn paint_strong_surface(ui: &mut Ui, rect: egui::Rect, pal: &Palette, accent: Option<Color32>) {
    paint_flat(ui, rect, surface_fill(pal.surface_strong, accent), pal);
}

pub fn paint_taskbar(ui: &mut Ui, rect: egui::Rect, pal: &Palette, accent: Option<Color32>) {
    paint_flat(ui, rect, surface_fill(pal.surface_taskbar, accent), pal);
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, with_alpha(pal.stroke_card, 180)),
    );
}

fn surface_fill(base: Color32, accent: Option<Color32>) -> Color32 {
    let Some(accent) = accent else {
        return Color32::from_rgb(base.r(), base.g(), base.b());
    };
    mix_opaque(base, accent, 0.22)
}

fn paint_flat(ui: &mut Ui, rect: egui::Rect, fill: Color32, pal: &Palette) {
    ui.painter().rect(
        rect,
        CornerRadius::ZERO,
        Color32::from_rgb(fill.r(), fill.g(), fill.b()),
        Stroke::new(1.0, with_alpha(pal.stroke_card, 180)),
        StrokeKind::Inside,
    );
}

fn mix_opaque(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| ((x as f32) + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

fn with_alpha(c: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), alpha)
}
