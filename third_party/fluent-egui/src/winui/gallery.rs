use eframe::egui::{
    self, Align, InnerResponse, Layout, Margin, Rect, Response, Stroke, Ui, UiBuilder, pos2, vec2,
};

use super::{
    DropDownButton, FluentBorder, FluentIcon, FluentTheme, IconButton, TeachingTip, combo_option,
    icon_font_id, palette,
};

fn current_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

/// Header used by WinUI Gallery-style control pages.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryPageHeader {
    title: String,
    description: String,
}

impl GalleryPageHeader {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> GalleryPageHeaderResponse {
        let p = current_palette(ui);
        ui.label(egui::RichText::new(self.title).size(28.0).strong());
        ui.add_space(10.0);

        let mut response = GalleryPageHeaderResponse::default();
        ui.horizontal(|ui| {
            let forced_menu = std::env::var("WINUI_EGUI_HEADER_MENU").ok();
            let mut documentation = DropDownButton::new("Documentation")
                .icon(FluentIcon::OpenFile)
                .width(140.0);
            if forced_menu.as_deref() == Some("documentation") {
                documentation = documentation.open(true);
            }
            documentation.show(ui, |ui| {
                    if combo_option(ui, "Microsoft Learn", false).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://learn.microsoft.com/windows/apps/winui/winui3/",
                        ));
                        ui.close();
                    }
                    if combo_option(ui, "API reference", false).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://learn.microsoft.com/windows/windows-app-sdk/api/winrt/",
                        ));
                        ui.close();
                    }
                });
            let mut source = DropDownButton::new("Source code")
                .icon(FluentIcon::Globe)
                .width(136.0);
            if forced_menu.as_deref() == Some("source") {
                source = source.open(true);
            }
            source.show(ui, |ui| {
                    if combo_option(ui, "XAML", false).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://github.com/microsoft/WinUI-Gallery/tree/main/WinUIGallery/Samples/ControlPages",
                        ));
                        ui.close();
                    }
                    if combo_option(ui, "C#", false).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://github.com/microsoft/WinUI-Gallery/tree/main/WinUIGallery/Samples/ControlPages",
                        ));
                        ui.close();
                    }
                });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                response.favorite = Some(
                    ui.add(IconButton::new(FluentIcon::Favorite))
                        .on_hover_text("Favorite sample"),
                );
                response.feedback = Some(
                    ui.add(IconButton::new(FluentIcon::Mail))
                        .on_hover_text("Send feedback"),
                );
                response.copy_link = Some(
                    ui.add(IconButton::new(FluentIcon::Copy))
                        .on_hover_text("Copy link"),
                );
                response.theme = Some(
                    ui.add(IconButton::new(if ui.visuals().dark_mode {
                        FluentIcon::Sun
                    } else {
                        FluentIcon::Moon
                    }))
                    .on_hover_text("Toggle theme"),
                );
            });
        });
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(self.description)
                .size(14.0)
                .color(p.text_secondary),
        );
        ui.add_space(22.0);
        response
    }
}

/// Responses for the command buttons in [`GalleryPageHeader`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Default)]
pub struct GalleryPageHeaderResponse {
    /// The `theme` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub theme: Option<Response>,
    /// The `copy_link` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub copy_link: Option<Response>,
    /// The `feedback` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub feedback: Option<Response>,
    /// The `favorite` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub favorite: Option<Response>,
}

/// A full-width, clickable navigation card used by Gallery landing pages.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryTopicCard {
    icon: FluentIcon,
    title: String,
    subtitle: String,
}

impl GalleryTopicCard {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(icon: FluentIcon, title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            icon,
            title: title.into(),
            subtitle: subtitle.into(),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        FluentBorder::new()
            .show(ui, |ui| {
                let row_width = ui.available_width();
                ui.allocate_ui_with_layout(
                    vec2(row_width, 64.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.add(IconButton::new(self.icon));
                        let text_width = (ui.available_width() - 28.0).max(0.0);
                        let (text_rect, _) =
                            ui.allocate_exact_size(vec2(text_width, 44.0), egui::Sense::hover());
                        ui.painter().text(
                            pos2(text_rect.left(), text_rect.center().y - 10.0),
                            egui::Align2::LEFT_CENTER,
                            self.title,
                            egui::FontId::proportional(16.0),
                            ui.visuals().text_color(),
                        );
                        ui.painter().text(
                            pos2(text_rect.left(), text_rect.center().y + 11.0),
                            egui::Align2::LEFT_CENTER,
                            self.subtitle,
                            egui::FontId::proportional(14.0),
                            ui.visuals().weak_text_color(),
                        );
                        ui.add_sized(
                            [20.0, 32.0],
                            egui::Label::new(
                                egui::RichText::new(FluentIcon::ChevronRight.glyph())
                                    .font(icon_font_id(12.0)),
                            ),
                        );
                    },
                );
            })
            .response
            .interact(egui::Sense::click())
    }
}

#[derive(Clone, Copy)]
/// A Fluent API value representing `GalleryCatalogItem`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryCatalogItem<'a> {
    /// The `title` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub title: &'a str,
    /// The `subtitle` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub subtitle: &'a str,
    /// The `icon` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub icon: FluentIcon,
}

impl<'a> GalleryCatalogItem<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(title: &'a str, subtitle: &'a str, icon: FluentIcon) -> Self {
        Self {
            title,
            subtitle,
            icon,
        }
    }
}

/// Responsive equal-width card grid used by the complete control catalog.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryCatalogGrid<'a> {
    id: egui::Id,
    items: &'a [GalleryCatalogItem<'a>],
    min_card_width: f32,
    max_columns: usize,
}

impl<'a> GalleryCatalogGrid<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        id_source: impl std::hash::Hash + std::fmt::Debug,
        items: &'a [GalleryCatalogItem<'a>],
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            items,
            min_card_width: 220.0,
            max_columns: 4,
        }
    }

    /// Configures or reads the `min_card_width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn min_card_width(mut self, width: f32) -> Self {
        self.min_card_width = width;
        self
    }

    /// Configures or reads the `max_columns` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn max_columns(mut self, columns: usize) -> Self {
        self.max_columns = columns;
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
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        let gap = 8.0;
        let column_count = (((ui.available_width() + gap) / (self.min_card_width + gap)).floor()
            as usize)
            .clamp(1, self.max_columns.max(1));
        let card_width = (ui.available_width() - gap * (column_count.saturating_sub(1) as f32))
            / column_count as f32;
        let mut clicked = None;
        egui::Grid::new(self.id)
            .num_columns(column_count)
            .spacing(vec2(gap, 8.0))
            .show(ui, |ui| {
                for (index, item) in self.items.iter().enumerate() {
                    let response = ui
                        .allocate_ui(vec2(card_width, 82.0), |ui| {
                            ui.set_width(card_width);
                            FluentBorder::new()
                                .show(ui, |ui| {
                                    ui.set_min_width((card_width - 24.0).max(0.0));
                                    ui.set_min_height(64.0);
                                    ui.horizontal(|ui| {
                                        ui.add(IconButton::new(item.icon));
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(item.title).strong());
                                            ui.label(
                                                egui::RichText::new(item.subtitle)
                                                    .size(11.0)
                                                    .color(ui.visuals().weak_text_color()),
                                            );
                                        });
                                    });
                                })
                                .response
                                .interact(egui::Sense::click())
                        })
                        .inner;
                    if response.clicked() {
                        clicked = Some(index);
                    }
                    if (index + 1) % column_count == 0 {
                        ui.end_row();
                    }
                }
            });
        clicked
    }
}

#[derive(Clone, Copy, Debug)]
/// A Fluent API value representing `GalleryImageHotspot`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryImageHotspot<'a> {
    /// The `position` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub position: egui::Vec2,
    /// The `title` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub title: &'a str,
    /// The `message` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub message: &'a str,
}

impl<'a> GalleryImageHotspot<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(position: egui::Vec2, title: &'a str, message: &'a str) -> Self {
        Self {
            position,
            title,
            message,
        }
    }
}

/// Theme image with clickable WinUI information hotspots and TeachingTips.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryAnnotatedImage<'a> {
    id: egui::Id,
    texture: &'a egui::TextureHandle,
    source_size: egui::Vec2,
    hotspots: &'a [GalleryImageHotspot<'a>],
    selected: &'a mut Option<usize>,
}

impl<'a> GalleryAnnotatedImage<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        id_source: impl std::hash::Hash + std::fmt::Debug,
        texture: &'a egui::TextureHandle,
        source_size: egui::Vec2,
        hotspots: &'a [GalleryImageHotspot<'a>],
        selected: &'a mut Option<usize>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            texture,
            source_size,
            hotspots,
            selected,
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui, theme: FluentTheme) -> Response {
        let scale = (ui.available_width() / self.source_size.x).min(1.0);
        let display_size = self.source_size * scale;
        let (image_rect, mut response) = ui.allocate_exact_size(display_size, egui::Sense::hover());
        ui.painter().image(
            self.texture.id(),
            image_rect,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );

        let p = palette(theme);
        for (index, hotspot) in self.hotspots.iter().enumerate() {
            let rect =
                Rect::from_min_size(image_rect.min + hotspot.position * scale, vec2(30.0, 30.0));
            let hotspot_response = ui.interact(rect, self.id.with(index), egui::Sense::click());
            ui.painter().rect(
                rect,
                4,
                if hotspot_response.hovered() {
                    p.control_hover
                } else {
                    p.control_bg
                },
                Stroke::new(1.0, p.control_border),
                egui::StrokeKind::Inside,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                FluentIcon::Info.glyph(),
                icon_font_id(16.0),
                p.text_primary,
            );
            if hotspot_response.clicked() {
                *self.selected = Some(index);
            }
            response |= hotspot_response;
        }

        if let Some(index) = *self.selected
            && let Some(hotspot) = self.hotspots.get(index)
            && TeachingTip::new((self.id, index), hotspot.title, hotspot.message)
                .show(ui.ctx(), theme, true)
                .close_clicked
        {
            *self.selected = None;
        }
        response
    }
}

/// Bordered example section matching the structure used by WinUI Gallery pages.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GalleryExample {
    title: String,
    description: Option<String>,
}

impl GalleryExample {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
        }
    }

    /// Configures or reads the `description` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let p = current_palette(ui);
        ui.label(egui::RichText::new(self.title).size(14.0).strong());
        ui.add_space(8.0);
        let result = egui::Frame::new()
            .fill(p.card_bg)
            .stroke(Stroke::new(1.0, p.card_border))
            .corner_radius(0)
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 24.0).max(240.0));
                if let Some(description) = self.description {
                    ui.label(
                        egui::RichText::new(description)
                            .size(14.0)
                            .color(p.text_secondary),
                    );
                    ui.add_space(14.0);
                }
                add_contents(ui)
            });
        ui.add_space(12.0);
        result
    }

    /// Configures or reads the `show_with_options` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_with_options<R, O>(
        self,
        ui: &mut Ui,
        add_example: impl FnOnce(&mut Ui) -> R,
        add_options: impl FnOnce(&mut Ui) -> O,
    ) -> InnerResponse<(R, O)> {
        let p = current_palette(ui);
        ui.label(egui::RichText::new(self.title).size(14.0).strong());
        ui.add_space(8.0);
        let result = egui::Frame::new()
            .fill(p.card_bg)
            .stroke(Stroke::new(1.0, p.card_border))
            .corner_radius(0)
            .inner_margin(Margin::symmetric(12, 10))
            .show(ui, |ui| {
                ui.set_min_width((ui.available_width() - 24.0).max(240.0));
                if let Some(description) = self.description {
                    ui.label(
                        egui::RichText::new(description)
                            .size(14.0)
                            .color(p.text_secondary),
                    );
                    ui.add_space(14.0);
                }
                if ui.available_width() >= 620.0 {
                    let available_width = ui.available_width();
                    let spacing = ui.spacing().item_spacing.x;
                    let options_width = 260.0_f32.min(available_width * 0.36);
                    let example_width = (available_width - options_width - spacing).max(280.0);
                    let top_left = ui.cursor().min;
                    let bottom = ui.max_rect().bottom();
                    let mut example_ui = ui.new_child(
                        UiBuilder::new()
                            .max_rect(Rect::from_min_max(
                                top_left,
                                pos2(top_left.x + example_width, bottom),
                            ))
                            .layout(Layout::top_down(Align::Min)),
                    );
                    example_ui.set_width(example_width);
                    let example = add_example(&mut example_ui);

                    let options_left = top_left.x + example_width + spacing;
                    let mut options_ui = ui.new_child(
                        UiBuilder::new()
                            .max_rect(Rect::from_min_max(
                                pos2(options_left, top_left.y),
                                pos2(options_left + options_width, bottom),
                            ))
                            .layout(Layout::top_down(Align::Min)),
                    );
                    options_ui.set_width(options_width);
                    options_ui.label(
                        egui::RichText::new("Options")
                            .size(12.0)
                            .color(p.text_secondary),
                    );
                    options_ui.add_space(6.0);
                    let options = add_options(&mut options_ui);
                    let height = example_ui
                        .min_rect()
                        .height()
                        .max(options_ui.min_rect().height());
                    let separator_x = top_left.x + example_width + spacing * 0.5;
                    ui.painter().line_segment(
                        [
                            pos2(separator_x, top_left.y),
                            pos2(separator_x, top_left.y + height),
                        ],
                        Stroke::new(1.0, p.card_border),
                    );
                    ui.advance_cursor_after_rect(Rect::from_min_size(
                        top_left,
                        vec2(available_width, height),
                    ));
                    (example, options)
                } else {
                    let example = add_example(ui);
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Options")
                            .size(12.0)
                            .color(p.text_secondary),
                    );
                    let options = add_options(ui);
                    (example, options)
                }
            });
        ui.add_space(12.0);
        result
    }
}

/// Collapsible source-code section used below Gallery examples.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GallerySourceExpander {
    id: egui::Id,
    source: String,
    language: String,
}

impl GallerySourceExpander {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        id_source: impl std::hash::Hash + std::fmt::Debug,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            source: source.into(),
            language: "rust".into(),
        }
    }

    /// Configures or reads the `language` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
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
    pub fn show(self, ui: &mut Ui) -> Response {
        let p = current_palette(ui);
        egui::CollapsingHeader::new("Source code")
            .id_salt(self.id)
            .default_open(false)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.language)
                            .size(12.0)
                            .color(p.text_secondary),
                    );
                    ui.add_space(ui.available_width());
                });
                egui::Frame::new()
                    .fill(p.control_bg)
                    .corner_radius(0)
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        ui.add(
                            egui::Label::new(egui::RichText::new(self.source).monospace())
                                .selectable(true)
                                .wrap(),
                        );
                    });
            })
            .header_response
    }
}

/// Standard horizontal spacing between consecutive Gallery example blocks.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn gallery_example_spacing(ui: &mut Ui) {
    ui.allocate_space(vec2(0.0, 4.0));
}
