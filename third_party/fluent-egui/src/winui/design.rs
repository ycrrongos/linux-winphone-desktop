use eframe::egui::{self, Margin, RichText, Ui, vec2};

use super::{FluentBorder, FluentIcon, FluentTheme, IconButton, palette};

/// A Fluent API value representing `ResourceScopePreview`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ResourceScopePreview;

impl ResourceScopePreview {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(ui: &mut Ui) {
        egui::Frame::new()
            .fill(egui::Color32::from_rgb(0, 120, 212))
            .corner_radius(0)
            .inner_margin(Margin::same(12))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("Using application-level resources")
                        .size(20.0)
                        .color(egui::Color32::WHITE),
                );
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(169, 77, 193))
                    .corner_radius(0)
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Using page-level resources")
                                .size(16.0)
                                .color(egui::Color32::WHITE),
                        );
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(226, 36, 26))
                            .corner_radius(0)
                            .inner_margin(Margin::same(10))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new("Using control-level resources")
                                        .color(egui::Color32::WHITE),
                                );
                            });
                    });
            });
    }
}

/// A Fluent API value representing `CornerRadiusTable`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CornerRadiusTable {
    theme: FluentTheme,
}

impl CornerRadiusTable {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(theme: FluentTheme) -> Self {
        Self { theme }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_sized([148.0, 20.0], egui::Label::new("Corner radius"));
            ui.add_sized([400.0, 20.0], egui::Label::new("Usage"));
            ui.label("Style");
        });
        for (radius, label, usage, resource) in [
            (
                8,
                "8 px",
                "Top-level overlays, flyouts, cards and dialogs",
                "OverlayCornerRadius",
            ),
            (
                4,
                "4 px",
                "In-page controls and list backplates",
                "ControlCornerRadius",
            ),
            (
                0,
                "0 px",
                "Straight edges that meet neighboring edges",
                "N/a",
            ),
        ] {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(20.0, 20.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, radius, palette(self.theme).accent);
                ui.add_sized(
                    [90.0, 24.0],
                    egui::Label::new(RichText::new(label).strong()),
                );
                ui.add_sized([420.0, 24.0], egui::Label::new(usage));
                ui.label(RichText::new(resource).monospace());
                if ui.add(IconButton::new(FluentIcon::Copy)).clicked() {
                    ui.ctx().copy_text(resource.to_owned());
                }
            });
            ui.separator();
        }
    }
}

/// A Fluent API value representing `SpacingRamp`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SpacingRamp;

impl SpacingRamp {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(ui: &mut Ui) {
        for (value, usage) in [
            (4.0, "Compact sizing"),
            (8.0, "Between a control and its label"),
            (12.0, "Control with header and edge text"),
            (16.0, "List and card padding"),
            (24.0, "Between content sections"),
            (36.0, "Page padding"),
            (48.0, "Between titled page sections"),
        ] {
            ui.horizontal(|ui| {
                ui.add_sized([64.0, 20.0], egui::Label::new(format!("{value:.0} epx")));
                let (rect, _) =
                    ui.allocate_exact_size(vec2(value * 4.0, 12.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, 2, ui.visuals().hyperlink_color);
                ui.label(usage);
            });
            ui.separator();
        }
    }
}

/// A Fluent API value representing `TypographyRamp`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TypographyRamp;

impl TypographyRamp {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_sized([260.0, 20.0], egui::Label::new("Example"));
            ui.add_sized([150.0, 20.0], egui::Label::new("Variable Font"));
            ui.add_sized([125.0, 20.0], egui::Label::new("Size/Line height"));
            ui.label("Style");
        });
        for (label, size, line_height, weight, resource) in [
            (
                "Caption",
                12.0_f32,
                "12/16",
                "Regular 400",
                "CaptionTextBlockStyle",
            ),
            ("Body", 14.0, "14/20", "Regular 400", "BodyTextBlockStyle"),
            (
                "Body Strong",
                14.0,
                "14/20",
                "SemiBold 600",
                "BodyStrongTextBlockStyle",
            ),
            (
                "Body Large",
                18.0,
                "18/24",
                "Regular 400",
                "BodyLargeTextBlockStyle",
            ),
            (
                "Body Large Strong",
                18.0,
                "18/24",
                "SemiBold 600",
                "BodyLargeStrongTextBlockStyle",
            ),
            (
                "Subtitle",
                20.0,
                "20/28",
                "SemiBold 600",
                "SubtitleTextBlockStyle",
            ),
            (
                "Title",
                28.0,
                "28/36",
                "SemiBold 600",
                "TitleTextBlockStyle",
            ),
            (
                "Title Large",
                40.0,
                "40/52",
                "SemiBold 600",
                "TitleLargeTextBlockStyle",
            ),
            (
                "Display",
                68.0,
                "68/92",
                "SemiBold 600",
                "DisplayTextBlockStyle",
            ),
        ] {
            let variable_font = match label {
                "Caption" => "Small, Regular",
                "Body" | "Body Large" => "Text, Regular",
                "Body Strong" | "Body Large Strong" => "Text, SemiBold",
                _ => "Display, SemiBold",
            };
            FluentBorder::new().show(ui, |ui| {
                ui.horizontal(|ui| {
                    let text = RichText::new(label).size(size);
                    let text = if weight.contains("600") {
                        text.strong()
                    } else {
                        text
                    };
                    ui.add_sized([260.0, size.max(40.0) + 12.0], egui::Label::new(text));
                    ui.add_sized([150.0, 32.0], egui::Label::new(variable_font));
                    ui.add_sized(
                        [125.0, 32.0],
                        egui::Label::new(format!("{line_height} epx")),
                    );
                    ui.vertical(|ui| {
                        ui.label(RichText::new(resource).monospace());
                        ui.label(format!("Weight {weight}"));
                    });
                });
            });
        }
    }
}
