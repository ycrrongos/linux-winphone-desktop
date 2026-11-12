//! Apply Win10UI Fluent 1 palette onto egui `Visuals` (overrides fluent-egui Fluent 2 look).

use std::collections::BTreeMap;
use std::sync::Arc;

use eframe::egui::{self, CornerRadius, FontFamily, FontId, Stroke, Style, TextStyle, Visuals};

use super::tokens::{type_size, Fluent1Theme, Palette};

pub fn apply_to_ctx(ctx: &egui::Context, theme: Fluent1Theme, pal: &Palette) {
    apply_to_ctx_ex(ctx, theme, pal, false);
}

/// `nested`: keep unused shell pixels clear so compositor-managed apps remain visible.
pub fn apply_to_ctx_ex(ctx: &egui::Context, theme: Fluent1Theme, pal: &Palette, nested: bool) {
    let egui_theme = match theme {
        Fluent1Theme::Dark => egui::Theme::Dark,
        Fluent1Theme::Light => egui::Theme::Light,
    };
    let mut style: Style = (*ctx.style_of(egui_theme)).clone();
    style.visuals = visuals_from_palette(pal, theme, nested);
    style.text_styles = text_styles();
    // Win10: square control corners
    style.visuals.window_corner_radius = CornerRadius::same(pal.radius_chrome);
    style.visuals.menu_corner_radius = CornerRadius::same(pal.radius_chrome);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    ctx.set_style_of(egui_theme, Arc::new(style));
}

fn text_styles() -> BTreeMap<TextStyle, FontId> {
    [
        (
            TextStyle::Small,
            FontId::new(type_size::CAPTION, FontFamily::Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(type_size::BODY, FontFamily::Proportional),
        ),
        (
            TextStyle::Button,
            FontId::new(type_size::BODY, FontFamily::Proportional),
        ),
        (
            TextStyle::Heading,
            FontId::new(type_size::SUBTITLE, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(type_size::BODY, FontFamily::Monospace),
        ),
        (
            TextStyle::Name("Title".into()),
            FontId::new(type_size::TITLE, FontFamily::Proportional),
        ),
    ]
    .into()
}

fn visuals_from_palette(pal: &Palette, theme: Fluent1Theme, nested: bool) -> Visuals {
    let mut v = match theme {
        Fluent1Theme::Dark => Visuals::dark(),
        Fluent1Theme::Light => Visuals::light(),
    };
    v.dark_mode = matches!(theme, Fluent1Theme::Dark);
    v.override_text_color = Some(pal.text);
    v.widgets.noninteractive.bg_fill = pal.control;
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, pal.text);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, pal.stroke_card);

    v.widgets.inactive.bg_fill = pal.control_secondary;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, pal.text);
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, pal.stroke);
    v.widgets.inactive.corner_radius = CornerRadius::same(pal.radius_chrome);

    v.widgets.hovered.bg_fill = pal.control_tertiary;
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, pal.text);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, pal.stroke);
    v.widgets.hovered.corner_radius = CornerRadius::same(pal.radius_chrome);

    v.widgets.active.bg_fill = pal.accent;
    v.widgets.active.fg_stroke = Stroke::new(1.0, pal.accent_text);
    v.widgets.active.bg_stroke = Stroke::new(1.0, pal.accent_dark);
    v.widgets.active.corner_radius = CornerRadius::same(pal.radius_chrome);

    v.widgets.open.bg_fill = pal.control_tertiary;
    v.widgets.open.corner_radius = CornerRadius::same(pal.radius_chrome);

    v.selection.bg_fill =
        Color32::from_rgba_unmultiplied(pal.accent.r(), pal.accent.g(), pal.accent.b(), 48);
    v.selection.stroke = Stroke::new(1.0, pal.accent_light);

    v.hyperlink_color = pal.accent;
    v.faint_bg_color = pal.bg_alt;
    v.extreme_bg_color = pal.bg_smoke;
    v.code_bg_color = pal.bg_alt;
    v.warn_fg_color = pal.warning;
    v.error_fg_color = pal.critical;

    if nested {
        v.window_fill = Color32::TRANSPARENT;
        v.panel_fill = Color32::TRANSPARENT;
        v.window_stroke = Stroke::NONE;
        v.window_shadow.color = Color32::TRANSPARENT;
        v.popup_shadow.color = Color32::TRANSPARENT;
    } else {
        v.window_fill = pal.surface;
        v.window_stroke = Stroke::new(1.0, pal.stroke_card);
        v.panel_fill = pal.bg;
        v.popup_shadow.color = pal.shadow;
        v.window_shadow.color = pal.shadow;
    }

    v
}

use eframe::egui::Color32;
