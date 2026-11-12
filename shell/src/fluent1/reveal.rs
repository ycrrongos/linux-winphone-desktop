//! Reveal highlight — simplified from Win10UI `reveal.css` / `reveal.js`.

use eframe::egui::{self, Color32};

use super::tokens::Palette;

#[derive(Debug, Default, Clone)]
pub struct RevealState {
    pub pointer: Option<egui::Pos2>,
}

impl RevealState {
    pub fn track(&mut self, ui: &egui::Ui) {
        self.pointer = ui.ctx().pointer_latest_pos();
    }

    /// Soft proximity fill inside `rect` when pointer is near / inside.
    pub fn paint_on(&self, ui: &egui::Ui, rect: egui::Rect, pal: &Palette) {
        let Some(pos) = self.pointer else {
            return;
        };
        let near = 80.0;
        let d = if rect.contains(pos) {
            0.0
        } else {
            rect.distance_to_pos(pos)
        };
        if d > near {
            return;
        }
        let strength = 1.0 - (d / near);
        let a = (pal.reveal_strong.a() as f32 * strength) as u8;
        let fill = Color32::from_rgba_unmultiplied(
            pal.reveal_strong.r(),
            pal.reveal_strong.g(),
            pal.reveal_strong.b(),
            a.max(1),
        );
        // Radial-ish: brighter near pointer, clipped to rect.
        let r = if rect.contains(pos) { 48.0 } else { 28.0 };
        let glow = egui::Rect::from_center_size(pos, egui::vec2(r * 2.0, r * 2.0));
        let clip = glow.intersect(rect);
        if clip.is_positive() {
            ui.painter().rect_filled(clip, 0.0, fill);
        }
    }
}
