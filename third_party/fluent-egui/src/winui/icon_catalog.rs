use std::sync::OnceLock;

use eframe::egui::{self, RichText, Ui, pos2, vec2};
use serde::Deserialize;

use super::{FluentBorder, FluentTheme, icon_font_id, palette};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RawIconEntry {
    name: String,
    code: String,
    tags: Vec<String>,
    #[serde(default)]
    is_segoe_fluent_only: bool,
}

/// One entry from the complete WinUI Gallery Segoe Fluent icon catalog.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Debug)]
pub struct FluentIconEntry {
    /// The `name` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub name: String,
    /// The `code` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub code: String,
    /// The `tags` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub tags: Vec<String>,
    /// The `is_segoe_fluent_only` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub is_segoe_fluent_only: bool,
    /// The `glyph` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub glyph: char,
}

impl FluentIconEntry {
    /// Configures or reads the `matches` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn matches(&self, query: &str) -> bool {
        let query = query.trim().to_ascii_lowercase();
        query.is_empty()
            || self.name.to_ascii_lowercase().contains(&query)
            || self.code.to_ascii_lowercase().contains(&query)
            || self
                .tags
                .iter()
                .any(|tag| tag.to_ascii_lowercase().contains(&query))
    }

    /// Configures or reads the `code_glyph` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn code_glyph(&self) -> String {
        format!("\\u{{{}}}", self.code)
    }

    /// Configures or reads the `text_glyph` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn text_glyph(&self) -> String {
        format!("&#x{};", self.code)
    }
}

/// Returns all 1,533 icon records used by WinUI Gallery 2.8.0.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn fluent_icon_catalog() -> &'static [FluentIconEntry] {
    static CATALOG: OnceLock<Vec<FluentIconEntry>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str::<Vec<RawIconEntry>>(include_str!("../assets/IconsData.json"))
            .expect("embedded WinUI icon catalog must be valid JSON")
            .into_iter()
            .filter_map(|entry| {
                let value = u32::from_str_radix(&entry.code, 16).ok()?;
                let glyph = char::from_u32(value)?;
                Some(FluentIconEntry {
                    name: entry.name,
                    code: entry.code,
                    tags: entry.tags,
                    is_segoe_fluent_only: entry.is_segoe_fluent_only,
                    glyph,
                })
            })
            .collect()
    })
}

/// Searchable WinUI icon grid with a selected-icon detail pane.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentIconCatalog<'a> {
    query: &'a str,
    selected: &'a mut usize,
    max_height: f32,
}

impl<'a> FluentIconCatalog<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(query: &'a str, selected: &'a mut usize) -> Self {
        Self {
            query,
            selected,
            max_height: 620.0,
        }
    }

    /// Configures or reads the `max_height` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui, theme: FluentTheme) {
        let catalog = fluent_icon_catalog();
        *self.selected = (*self.selected).min(catalog.len().saturating_sub(1));
        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("Segoe Fluent icons").size(16.0).strong());
                egui::ScrollArea::vertical()
                    .id_salt("complete_icon_catalog")
                    .max_height(self.max_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for (index, icon) in catalog.iter().enumerate() {
                                if !icon.matches(self.query) {
                                    continue;
                                }
                                let (rect, response) =
                                    ui.allocate_exact_size(vec2(96.0, 96.0), egui::Sense::click());
                                let p = palette(theme);
                                ui.painter().rect(
                                    rect,
                                    4,
                                    if response.hovered() {
                                        p.control_hover
                                    } else {
                                        p.card_bg
                                    },
                                    egui::Stroke::new(1.0, p.card_border),
                                    egui::StrokeKind::Inside,
                                );
                                ui.painter().text(
                                    pos2(rect.center().x, rect.top() + 34.0),
                                    egui::Align2::CENTER_CENTER,
                                    icon.glyph,
                                    icon_font_id(28.0),
                                    p.text_primary,
                                );
                                let label_clip = egui::Rect::from_min_max(
                                    pos2(rect.left() + 6.0, rect.bottom() - 30.0),
                                    pos2(rect.right() - 6.0, rect.bottom() - 6.0),
                                );
                                ui.painter().with_clip_rect(label_clip).text(
                                    label_clip.center(),
                                    egui::Align2::CENTER_CENTER,
                                    &icon.name,
                                    egui::FontId::proportional(11.0),
                                    p.text_secondary,
                                );
                                if response.clicked() {
                                    *self.selected = index;
                                }
                            }
                        });
                    });
            });
            columns[1].vertical(|ui| {
                let selected = &catalog[*self.selected];
                ui.label(RichText::new("Selected icon").size(16.0).strong());
                FluentBorder::new().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new(selected.glyph.to_string()).font(icon_font_id(54.0)),
                        );
                        ui.label(RichText::new(&selected.name).size(18.0).strong());
                        if selected.is_segoe_fluent_only {
                            ui.label(
                                RichText::new("Only supported in Segoe Fluent Icons")
                                    .color(palette(theme).warning),
                            );
                        }
                    });
                    ui.separator();
                    ui.label("Icon name");
                    ui.label(RichText::new(&selected.name).monospace());
                    ui.label("Text glyph");
                    ui.label(RichText::new(selected.text_glyph()).monospace());
                    ui.label("Code glyph");
                    ui.label(RichText::new(selected.code_glyph()).monospace());
                    ui.label("Tags");
                    ui.label(selected.tags.join(", "));
                });
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_catalog_contains_all_icons() {
        let catalog = fluent_icon_catalog();
        assert_eq!(catalog.len(), 1_533);
        assert_eq!(
            catalog.first().map(|icon| icon.name.as_str()),
            Some("GlobalNavButton")
        );
        assert!(catalog.iter().any(|icon| icon.name == "Wifi"));
    }
}
