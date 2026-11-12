use std::{
    f32::consts::{FRAC_PI_2, TAU},
    ops::RangeInclusive,
};

use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, FontId, InnerResponse, Layout, Margin, Rect,
    Response, Sense, Shape, Stroke, StrokeKind, TextStyle, Ui, Vec2, Widget, pos2, vec2,
};

use super::{FluentIcon, FluentPalette, FluentTheme, card_frame, icon_font_id, palette};

const TEXT_Y_NUDGE: f32 = -0.5;

fn ui_palette(ui: &Ui) -> FluentPalette {
    let mut p = palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    });
    let accent = ui.visuals().hyperlink_color;
    if accent != p.accent {
        p.accent = accent;
        p.accent_hover = mix(accent, Color32::WHITE, 0.10);
        p.accent_pressed = mix(accent, Color32::BLACK, 0.12);
        p.selection = Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 42);
    }
    p
}

fn mix(a: Color32, b: Color32, amount: f32) -> Color32 {
    let amount = amount.clamp(0.0, 1.0);
    let [ar, ag, ab, aa] = a.to_srgba_unmultiplied();
    let [br, bg, bb, ba] = b.to_srgba_unmultiplied();
    let c = |x: u8, y: u8| ((x as f32 * (1.0 - amount)) + (y as f32 * amount)) as u8;
    Color32::from_rgba_unmultiplied(c(ar, br), c(ag, bg), c(ab, bb), c(aa, ba))
}

fn animated_fill(ui: &Ui, id: egui::Id, from: Color32, to: Color32, active: bool) -> Color32 {
    let t = ui.ctx().animate_bool_responsive(id, active);
    mix(from, to, t)
}

fn settings_hover_fill(ui: &Ui, p: FluentPalette) -> Color32 {
    if ui.visuals().dark_mode {
        mix(p.card_bg, Color32::WHITE, 0.035)
    } else {
        mix(p.card_bg, Color32::BLACK, 0.025)
    }
}

fn settings_separator(ui: &Ui) -> Color32 {
    if ui.visuals().dark_mode {
        Color32::from_rgba_unmultiplied(0, 0, 0, 110)
    } else {
        Color32::from_rgba_unmultiplied(0, 0, 0, 24)
    }
}

/// A WinUI-aligned single-line [`egui::TextEdit`].
///
/// It centers the text/caret vertically and uses WinUI's horizontal content padding.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentTextEdit<'a> {
    text: &'a mut dyn egui::TextBuffer,
    hint_text: String,
    password: bool,
    char_limit: usize,
}

impl<'a> FluentTextEdit<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: &'a mut dyn egui::TextBuffer) -> Self {
        Self {
            text,
            hint_text: String::new(),
            password: false,
            char_limit: usize::MAX,
        }
    }

    /// Configures or reads the `hint_text` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn hint_text(mut self, hint_text: impl Into<String>) -> Self {
        self.hint_text = hint_text.into();
        self
    }

    /// Configures or reads the `password` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    /// Configures or reads the `char_limit` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn char_limit(mut self, char_limit: usize) -> Self {
        self.char_limit = char_limit;
        self
    }
}

impl Widget for FluentTextEdit<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_empty = self.text.as_str().is_empty();
        let response = egui::TextEdit::singleline(self.text)
            .vertical_align(Align::Center)
            .margin(Margin {
                left: 10,
                right: 10,
                top: 4,
                bottom: 6,
            })
            .password(self.password)
            .char_limit(self.char_limit)
            .ui(ui);

        if is_empty && !self.hint_text.is_empty() {
            let clip = response.rect.shrink2(vec2(10.0, 2.0));
            ui.painter().with_clip_rect(clip).text(
                pos2(response.rect.left() + 11.0, response.rect.center().y + 0.5),
                Align2::LEFT_CENTER,
                self.hint_text,
                TextStyle::Body.resolve(ui.style()),
                ui.visuals().weak_text_color(),
            );
        }

        response
    }
}

/// Runs the `fluent_text_edit` Fluent helper.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn fluent_text_edit(text: &mut dyn egui::TextBuffer) -> FluentTextEdit<'_> {
    FluentTextEdit::new(text)
}

/// Visual variants for [`FluentButton`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonKind {
    #[default]
    /// The `Standard` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Standard,
    /// The `Accent` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Accent,
    /// The `Subtle` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Subtle,
}

/// A WinUI-style push button with optional Segoe Fluent icon.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentButton {
    text: String,
    icon: Option<FluentIcon>,
    kind: ButtonKind,
    min_width: f32,
}

impl FluentButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            kind: ButtonKind::Standard,
            min_width: 0.0,
        }
    }

    /// Creates the `accent` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn accent(text: impl Into<String>) -> Self {
        Self::new(text).kind(ButtonKind::Accent)
    }

    /// Creates the `subtle` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn subtle(text: impl Into<String>) -> Self {
        Self::new(text).kind(ButtonKind::Subtle)
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures or reads the `kind` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }

    /// Configures or reads the `min_width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }
}

impl Widget for FluentButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let font_id = TextStyle::Button.resolve(ui.style());
        let base_text_color = if self.kind == ButtonKind::Accent {
            p.on_accent
        } else {
            p.text_primary
        };
        let galley = ui
            .painter()
            .layout_no_wrap(self.text.clone(), font_id, base_text_color);
        let icon_width = if self.icon.is_some() { 24.0 } else { 0.0 };
        let desired = vec2(
            self.min_width.max(galley.size().x + icon_width + 24.0),
            32.0,
        );
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();
            let pressed = response.is_pointer_button_down_on();
            let (base, hover, pressed_fill, border, text_color) = match self.kind {
                ButtonKind::Standard => (
                    p.control_bg,
                    p.control_hover,
                    p.control_pressed,
                    p.control_border,
                    p.text_primary,
                ),
                ButtonKind::Accent => (
                    p.accent,
                    p.accent_hover,
                    p.accent_pressed,
                    Color32::TRANSPARENT,
                    p.on_accent,
                ),
                ButtonKind::Subtle => (
                    Color32::TRANSPARENT,
                    p.subtle_hover,
                    p.control_pressed,
                    Color32::TRANSPARENT,
                    p.text_primary,
                ),
            };
            let hover_fill = animated_fill(ui, response.id.with("hover"), base, hover, hovered);
            let fill = animated_fill(
                ui,
                response.id.with("press"),
                hover_fill,
                pressed_fill,
                pressed,
            );
            ui.painter().rect(
                rect,
                CornerRadius::ZERO,
                fill,
                Stroke::new(1.0, border),
                StrokeKind::Inside,
            );

            let content_width = galley.size().x + icon_width;
            let mut x = rect.center().x - content_width * 0.5;
            if let Some(icon) = self.icon {
                ui.painter().text(
                    pos2(x + 8.0, rect.center().y + TEXT_Y_NUDGE),
                    Align2::CENTER_CENTER,
                    icon.glyph(),
                    icon_font_id(14.0),
                    text_color,
                );
                x += icon_width;
            }
            ui.painter().galley(
                pos2(x, rect.center().y - galley.size().y * 0.5 + TEXT_Y_NUDGE),
                galley,
                text_color,
            );
        }
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &self.text)
        });
        response
    }
}

/// A WinUI Button whose content is supplied by an egui closure.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentContentButton {
    size: Vec2,
    kind: ButtonKind,
    accessible_name: String,
}

impl FluentContentButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(size: Vec2, accessible_name: impl Into<String>) -> Self {
        Self {
            size,
            kind: ButtonKind::Standard,
            accessible_name: accessible_name.into(),
        }
    }

    /// Configures or reads the `kind` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
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
    pub fn show<R>(self, ui: &mut Ui, add_content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let p = ui_palette(ui);
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::click());
        let (base, hover, pressed, border) = match self.kind {
            ButtonKind::Standard => (
                p.control_bg,
                p.control_hover,
                p.control_pressed,
                p.control_border,
            ),
            ButtonKind::Accent => (
                p.accent,
                p.accent_hover,
                p.accent_pressed,
                Color32::TRANSPARENT,
            ),
            ButtonKind::Subtle => (
                Color32::TRANSPARENT,
                p.subtle_hover,
                p.control_pressed,
                Color32::TRANSPARENT,
            ),
        };
        let hover_fill = animated_fill(
            ui,
            response.id.with("hover"),
            base,
            hover,
            response.hovered(),
        );
        let fill = animated_fill(
            ui,
            response.id.with("press"),
            hover_fill,
            pressed,
            response.is_pointer_button_down_on(),
        );
        ui.painter().rect(
            rect,
            CornerRadius::ZERO,
            fill,
            Stroke::new(1.0, border),
            StrokeKind::Inside,
        );
        let inner = ui
            .scope_builder(
                egui::UiBuilder::new()
                    .id_salt(response.id.with("content"))
                    .max_rect(rect.shrink(6.0))
                    .layout(Layout::centered_and_justified(egui::Direction::LeftToRight)),
                add_content,
            )
            .inner;
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                &self.accessible_name,
            )
        });
        InnerResponse { inner, response }
    }
}

/// Convenience wrapper for an accent-colored [`FluentButton`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AccentButton(FluentButton);

impl AccentButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self(FluentButton::accent(text))
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn icon(mut self, icon: FluentIcon) -> Self {
        self.0 = self.0.icon(icon);
        self
    }
}

impl Widget for AccentButton {
    fn ui(self, ui: &mut Ui) -> Response {
        self.0.ui(ui)
    }
}

/// A compact square command button. Attach a tooltip with `Response::on_hover_text`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct IconButton {
    icon: FluentIcon,
    selected: bool,
}

impl IconButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(icon: FluentIcon) -> Self {
        Self {
            icon,
            selected: false,
        }
    }

    /// Configures or reads the `selected` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::click());
        let base = if self.selected {
            p.selection
        } else {
            Color32::TRANSPARENT
        };
        let fill = animated_fill(
            ui,
            response.id.with("hover"),
            base,
            p.subtle_hover,
            response.hovered(),
        );
        ui.painter().rect_filled(rect, 0, fill);
        ui.painter().text(
            rect.center() + vec2(0.0, 1.0),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(15.0),
            p.text_primary,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), self.icon.name())
        });
        response
    }
}

/// A WinUI ToggleSwitch that directly edits a boolean.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ToggleSwitch<'a> {
    value: &'a mut bool,
    label: Option<String>,
}

impl<'a> ToggleSwitch<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut bool) -> Self {
        Self { value, label: None }
    }

    /// Configures or reads the `label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Widget for ToggleSwitch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let label_galley = self.label.as_ref().map(|label| {
            ui.painter().layout_no_wrap(
                label.clone(),
                TextStyle::Button.resolve(ui.style()),
                p.text_primary,
            )
        });
        let label_width = label_galley.as_ref().map_or(0.0, |g| g.size().x + 10.0);
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(40.0 + label_width, 24.0), Sense::click());
        if response.clicked() {
            *self.value = !*self.value;
            response.mark_changed();
        }

        let t = ui
            .ctx()
            .animate_bool_responsive(response.id.with("toggle"), *self.value);
        let track = egui::Rect::from_min_size(rect.min + vec2(label_width, 2.0), vec2(40.0, 20.0));
        let off_fill = if response.hovered() {
            p.control_hover
        } else {
            p.control_bg
        };
        let track_fill = mix(off_fill, p.accent, t);
        let border = mix(p.control_border, p.accent, t);
        ui.painter().rect(
            track,
            10,
            track_fill,
            Stroke::new(1.0, border),
            StrokeKind::Inside,
        );
        let knob_x = egui::lerp((track.left() + 10.0)..=(track.right() - 10.0), t);
        let knob_color = mix(p.text_secondary, p.on_accent, t);
        let radius = if response.is_pointer_button_down_on() {
            7.0
        } else {
            6.0
        };
        ui.painter()
            .circle_filled(pos2(knob_x, track.center().y), radius, knob_color);
        if let Some(galley) = label_galley {
            ui.painter().galley(
                pos2(
                    rect.left(),
                    rect.center().y - galley.size().y * 0.5 + TEXT_Y_NUDGE,
                ),
                galley,
                p.text_primary,
            );
        }
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Checkbox,
                ui.is_enabled(),
                *self.value,
                self.label.as_deref().unwrap_or("Toggle switch"),
            )
        });
        response
    }
}

/// Animated WinUI-style segmented selector.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SegmentedControl<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    width: f32,
}

impl<'a> SegmentedControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        selected: &'a mut usize,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            selected,
            labels: labels.into_iter().map(Into::into).collect(),
            width: 280.0,
        }
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Widget for SegmentedControl<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let count = self.labels.len().max(1);
        *self.selected = (*self.selected).min(count - 1);
        let (rect, mut response) = ui.allocate_exact_size(vec2(self.width, 32.0), Sense::hover());
        ui.painter().rect(
            rect,
            6,
            p.control_bg,
            Stroke::new(1.0, p.control_border),
            StrokeKind::Inside,
        );
        let segment_width = rect.width() / count as f32;
        let animated_index = ui.ctx().animate_value_with_time(
            response.id.with("selection"),
            *self.selected as f32,
            ui.style().animation_time,
        );
        let selected_rect = egui::Rect::from_min_size(
            pos2(
                rect.left() + animated_index * segment_width + 3.0,
                rect.top() + 3.0,
            ),
            vec2(segment_width - 6.0, rect.height() - 6.0),
        );
        ui.painter().rect(
            selected_rect,
            4,
            p.card_bg,
            Stroke::new(1.0, p.card_border),
            StrokeKind::Inside,
        );

        for (index, label) in self.labels.iter().enumerate() {
            let segment = egui::Rect::from_min_size(
                pos2(rect.left() + index as f32 * segment_width, rect.top()),
                vec2(segment_width, rect.height()),
            );
            let item_response = ui.interact(segment, response.id.with(index), Sense::click());
            if item_response.clicked() && *self.selected != index {
                *self.selected = index;
                response.mark_changed();
            }
            response |= item_response;
            ui.painter().text(
                segment.center() + vec2(0.0, TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                label,
                TextStyle::Button.resolve(ui.style()),
                p.text_primary,
            );
        }
        response
    }
}

/// A compact WinUI slider with a 4-point track and animated concentric thumb.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentSlider<'a> {
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    width: f32,
    show_value: bool,
    step: Option<f32>,
    vertical_height: Option<f32>,
}

impl<'a> FluentSlider<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut f32, range: RangeInclusive<f32>) -> Self {
        Self {
            value,
            range,
            width: 220.0,
            show_value: false,
            step: None,
            vertical_height: None,
        }
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Configures or reads the `show_value` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    /// Configures or reads the `step_by` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn step_by(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    /// Configures or reads the `vertical` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn vertical(mut self, height: f32) -> Self {
        self.vertical_height = Some(height);
        self
    }
}

impl Widget for FluentSlider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        if let Some(height) = self.vertical_height {
            let (rect, mut response) =
                ui.allocate_exact_size(vec2(42.0, height.max(72.0)), Sense::click_and_drag());
            let start = *self.range.start();
            let end = *self.range.end();
            let track = Rect::from_min_max(
                pos2(rect.center().x - 2.0, rect.top() + 12.0),
                pos2(rect.center().x + 2.0, rect.bottom() - 12.0),
            );
            if (response.dragged() || response.clicked())
                && let Some(pointer) = response.interact_pointer_pos()
            {
                let t = 1.0 - ((pointer.y - track.top()) / track.height()).clamp(0.0, 1.0);
                let mut value = egui::lerp(start..=end, t);
                if let Some(step) = self.step.filter(|step| *step > 0.0) {
                    value = start + ((value - start) / step).round() * step;
                }
                *self.value = value.clamp(start, end);
                response.mark_changed();
            }
            let t = if (end - start).abs() <= f32::EPSILON {
                0.0
            } else {
                ((*self.value - start) / (end - start)).clamp(0.0, 1.0)
            };
            let thumb = pos2(
                track.center().x,
                egui::lerp(track.bottom()..=track.top(), t),
            );
            ui.painter().rect_filled(track, 2.0, p.progress_track);
            ui.painter().rect_filled(
                Rect::from_min_max(pos2(track.left(), thumb.y), track.right_bottom()),
                2.0,
                p.accent,
            );
            let hover = ui.ctx().animate_bool_with_time_and_easing(
                response.id.with("thumb_hover"),
                response.hovered(),
                0.24,
                egui::emath::easing::cubic_out,
            );
            ui.painter().circle(
                thumb,
                10.0,
                p.slider_thumb,
                Stroke::new(1.0, p.slider_thumb_border),
            );
            ui.painter()
                .circle_filled(thumb, egui::lerp(5.5..=7.0, hover), p.accent);
            if self.show_value {
                ui.painter().text(
                    pos2(rect.left(), rect.top()),
                    Align2::LEFT_TOP,
                    format!("{:.0}", *self.value),
                    TextStyle::Button.resolve(ui.style()),
                    p.text_primary,
                );
            }
            return response;
        }
        let label_width = if self.show_value { 40.0 } else { 0.0 };
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(self.width, 32.0), Sense::click_and_drag());
        let start = *self.range.start();
        let end = *self.range.end();
        let track = egui::Rect::from_min_max(
            pos2(rect.left() + label_width + 8.0, rect.center().y - 2.0),
            pos2(rect.right() - 8.0, rect.center().y + 2.0),
        );

        if (response.dragged() || response.clicked())
            && let Some(pointer) = response.interact_pointer_pos()
        {
            let t = ((pointer.x - track.left()) / track.width()).clamp(0.0, 1.0);
            let mut new_value = egui::lerp(start..=end, t);
            if let Some(step) = self.step.filter(|step| *step > 0.0) {
                new_value = start + ((new_value - start) / step).round() * step;
            }
            if (*self.value - new_value).abs() > f32::EPSILON {
                *self.value = new_value;
                response.mark_changed();
            }
        }

        let t = if (end - start).abs() <= f32::EPSILON {
            0.0
        } else {
            ((*self.value - start) / (end - start)).clamp(0.0, 1.0)
        };
        ui.painter().rect_filled(track, 2, p.progress_track);
        let active = egui::Rect::from_min_max(
            track.min,
            pos2(egui::lerp(track.left()..=track.right(), t), track.bottom()),
        );
        if active.width() > 0.0 {
            ui.painter().rect_filled(active, 2, p.accent);
        }

        let thumb_center = pos2(
            egui::lerp(track.left()..=track.right(), t),
            track.center().y,
        );
        let hover_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("thumb_hover"),
            response.hovered(),
            0.24,
            egui::emath::easing::cubic_out,
        );
        let pressed_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("thumb_pressed"),
            response.is_pointer_button_down_on(),
            0.12,
            egui::emath::easing::cubic_out,
        );
        let inner_radius = egui::lerp(5.5..=7.0, hover_t) * (1.0 - pressed_t * 0.14);
        ui.painter().circle(
            thumb_center,
            10.0,
            p.slider_thumb,
            Stroke::new(1.0, p.slider_thumb_border),
        );
        ui.painter()
            .circle_filled(thumb_center, inner_radius, p.accent);

        if self.show_value {
            ui.painter().text(
                pos2(rect.left() + 18.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                format!("{:.0}", *self.value),
                TextStyle::Button.resolve(ui.style()),
                p.text_primary,
            );
        }
        response.widget_info(|| {
            egui::WidgetInfo::slider(ui.is_enabled(), *self.value as f64, "Slider")
        });
        response
    }
}

/// A 4-point WinUI progress track with an optional percentage placed beside the rail.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentProgressBar {
    progress: f32,
    width: f32,
    show_percentage: bool,
}

impl FluentProgressBar {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(progress: f32) -> Self {
        Self {
            progress,
            width: 180.0,
            show_percentage: false,
        }
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Configures or reads the `show_percentage` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn show_percentage(mut self, show_percentage: bool) -> Self {
        self.show_percentage = show_percentage;
        self
    }
}

impl Widget for FluentProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let label_width = if self.show_percentage { 46.0 } else { 0.0 };
        let (rect, response) = ui.allocate_exact_size(vec2(self.width, 28.0), Sense::hover());
        let track = egui::Rect::from_center_size(
            pos2(
                rect.left() + (rect.width() - label_width) * 0.5,
                rect.center().y,
            ),
            vec2((rect.width() - label_width).max(24.0), 4.0),
        );
        let progress = self.progress.clamp(0.0, 1.0);
        ui.painter().rect_filled(track, 2, p.progress_track);
        let filled = egui::Rect::from_min_max(
            track.min,
            pos2(
                egui::lerp(track.left()..=track.right(), progress),
                track.bottom(),
            ),
        );
        if filled.width() > 0.0 {
            ui.painter().rect_filled(filled, 2, p.accent);
        }
        if self.show_percentage {
            ui.painter().text(
                pos2(rect.right() - 20.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                format!("{}%", (progress * 100.0).round() as u8),
                TextStyle::Button.resolve(ui.style()),
                p.text_secondary,
            );
        }
        response
    }
}

/// Returns a ComboBox with WinUI's outlined chevron instead of egui's filled triangle.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn fluent_combo_box(
    id_source: impl std::hash::Hash + std::fmt::Debug,
    selected_text: impl Into<egui::WidgetText>,
) -> egui::ComboBox {
    egui::ComboBox::from_id_salt(id_source)
        .selected_text(selected_text)
        .truncate()
        .icon(|ui, rect, visuals, open| {
            let center = rect.center() + vec2(0.0, TEXT_Y_NUDGE);
            let direction = if open { -1.0 } else { 1.0 };
            let stroke = Stroke::new(1.25, visuals.fg_stroke.color);
            ui.painter().line_segment(
                [
                    center + vec2(-4.0, -2.0 * direction),
                    center + vec2(0.0, 2.0 * direction),
                ],
                stroke,
            );
            ui.painter().line_segment(
                [
                    center + vec2(0.0, 2.0 * direction),
                    center + vec2(4.0, -2.0 * direction),
                ],
                stroke,
            );
        })
}

/// A fixed-height WinUI dropdown option whose text never moves between states.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn combo_option(ui: &mut Ui, label: impl Into<String>, selected: bool) -> Response {
    let label = label.into();
    let p = ui_palette(ui);
    let galley = ui.painter().layout_no_wrap(
        label.clone(),
        TextStyle::Button.resolve(ui.style()),
        p.text_primary,
    );
    let width = ui.available_width().max(galley.size().x + 28.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, 32.0), Sense::click());
    let fill = if selected {
        settings_hover_fill(ui, p)
    } else if response.hovered() {
        p.subtle_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect.shrink2(vec2(1.0, 1.0)), 4, fill);
    if selected {
        let bar =
            egui::Rect::from_center_size(pos2(rect.left() + 4.0, rect.center().y), vec2(3.0, 16.0));
        ui.painter().rect_filled(bar, 1.5, p.accent);
    }
    ui.painter().galley(
        pos2(
            rect.left() + 14.0,
            rect.center().y - galley.size().y * 0.5 + TEXT_Y_NUDGE,
        ),
        galley,
        p.text_primary,
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &label,
        )
    });
    response
}

/// A large two-line button that opens a Fluent flyout below itself.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FlyoutButton {
    title: String,
    subtitle: String,
    icon: FluentIcon,
    width: Option<f32>,
    open: Option<bool>,
}

impl FlyoutButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            icon: FluentIcon::Device,
            width: None,
            open: None,
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the popup's open state. Without this, clicking the button toggles it.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
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
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<Option<R>> {
        let p = ui_palette(ui);
        let width = self.width.unwrap_or_else(|| ui.available_width());
        let (rect, response) = ui.allocate_exact_size(vec2(width, 64.0), Sense::click());
        let fill = animated_fill(
            ui,
            response.id.with("flyout_hover"),
            p.control_bg,
            p.control_hover,
            response.hovered(),
        );
        ui.painter().rect(
            rect,
            4,
            fill,
            Stroke::new(1.0, p.control_border),
            StrokeKind::Inside,
        );
        ui.painter().text(
            pos2(rect.left() + 26.0, rect.center().y + TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(18.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rect.left() + 52.0, rect.center().y - 10.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rect.left() + 52.0, rect.center().y + 11.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.subtitle,
            FontId::proportional(12.0),
            p.text_secondary,
        );
        ui.painter().text(
            pos2(rect.right() - 22.0, rect.center().y + TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronDown.glyph(),
            icon_font_id(12.0),
            p.text_secondary,
        );

        let popup = if let Some(open) = self.open {
            egui::Popup::from_response(&response).open(open)
        } else {
            egui::Popup::from_toggle_button_response(&response)
        };
        let inner = popup
            .kind(egui::PopupKind::Popup)
            .width(rect.width())
            .gap(4.0)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .frame(
                egui::Frame::popup(ui.style())
                    .corner_radius(0)
                    .inner_margin(Margin::same(8)),
            )
            .show(|ui| {
                ui.set_min_width((rect.width() - 16.0).max(240.0));
                add_contents(ui)
            })
            .map(|result| result.inner);

        InnerResponse { inner, response }
    }
}

/// A tiled option for [`FlyoutButton`], with either a selection dot or a navigation chevron.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FlyoutOption {
    title: String,
    subtitle: String,
    icon: FluentIcon,
    selected: bool,
    navigates: bool,
}

impl FlyoutOption {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            icon: FluentIcon::Device,
            selected: false,
            navigates: false,
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Configures or reads the `selected` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Configures or reads the `navigates` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn navigates(mut self, navigates: bool) -> Self {
        self.navigates = navigates;
        self
    }
}

impl Widget for FlyoutOption {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), 60.0), Sense::click());
        let base = if self.selected {
            p.selection
        } else {
            p.control_bg
        };
        let fill = animated_fill(
            ui,
            response.id.with("tile_hover"),
            base,
            p.control_hover,
            response.hovered(),
        );
        ui.painter().rect(
            rect,
            4,
            fill,
            Stroke::new(1.0, p.control_border),
            StrokeKind::Inside,
        );
        ui.painter().text(
            pos2(rect.left() + 26.0, rect.center().y + TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(17.0),
            if self.selected {
                p.accent
            } else {
                p.text_primary
            },
        );
        ui.painter().text(
            pos2(rect.left() + 52.0, rect.center().y - 9.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rect.left() + 52.0, rect.center().y + 11.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.subtitle,
            FontId::proportional(12.0),
            p.text_secondary,
        );

        let trailing = pos2(rect.right() - 24.0, rect.center().y);
        if self.navigates {
            ui.painter().text(
                trailing + vec2(0.0, TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                FluentIcon::ChevronRight.glyph(),
                icon_font_id(12.0),
                p.text_secondary,
            );
        } else {
            ui.painter().circle(
                trailing,
                8.0,
                p.control_bg,
                Stroke::new(1.0, p.control_border),
            );
            if self.selected {
                ui.painter().circle_filled(trailing, 4.5, p.accent);
            }
        }
        response
    }
}

/// A full-width Settings row with an icon, two lines of text and a navigation chevron.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SettingsNavigationRow {
    title: String,
    description: String,
    icon: FluentIcon,
    trailing_width: f32,
    navigates: bool,
}

impl SettingsNavigationRow {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, description: impl Into<String>, icon: FluentIcon) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            icon,
            trailing_width: 0.0,
            navigates: true,
        }
    }

    /// Configures or reads the `trailing_width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn trailing_width(mut self, width: f32) -> Self {
        self.trailing_width = width;
        self
    }

    /// Configures or reads the `navigates` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn navigates(mut self, navigates: bool) -> Self {
        self.navigates = navigates;
        self
    }

    /// Shows a row without a trailing control.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        self.show_with_trailing(ui, |_| ()).response
    }

    /// Shows a row and reserves a fixed-width area for a toggle, combo box or command.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_with_trailing<R>(
        self,
        ui: &mut Ui,
        add_trailing: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let p = ui_palette(ui);
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), 68.0), Sense::click());
        let fill = animated_fill(
            ui,
            response.id.with("settings_row_hover"),
            p.card_bg,
            settings_hover_fill(ui, p),
            response.hovered(),
        );
        ui.painter().rect(
            rect,
            4,
            fill,
            Stroke::new(1.0, p.card_border),
            StrokeKind::Inside,
        );
        ui.painter().text(
            pos2(rect.left() + 28.0, rect.center().y + TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(18.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rect.left() + 58.0, rect.center().y - 9.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rect.left() + 58.0, rect.center().y + 11.0 + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.description,
            FontId::proportional(12.0),
            p.text_secondary,
        );

        let chevron_width = if self.navigates { 40.0 } else { 16.0 };
        if self.navigates {
            ui.painter().text(
                pos2(rect.right() - 22.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                FluentIcon::ChevronRight.glyph(),
                icon_font_id(12.0),
                p.text_secondary,
            );
        }
        let control_rect = egui::Rect::from_min_max(
            pos2(
                (rect.right() - chevron_width - self.trailing_width).max(rect.left() + 210.0),
                rect.top(),
            ),
            pos2(rect.right() - chevron_width, rect.bottom()),
        );
        let mut trailing_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt(response.id.with("trailing"))
                .max_rect(control_rect)
                .layout(Layout::right_to_left(Align::Center)),
        );
        trailing_ui.set_clip_rect(control_rect.intersect(ui.clip_rect()));
        let inner = add_trailing(&mut trailing_ui);
        InnerResponse { inner, response }
    }
}

/// A selectable device row used inside [`SettingsExpander`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SettingsChoiceRow {
    title: String,
    subtitle: String,
    status: Option<String>,
    selected: bool,
    navigates: bool,
    separator: bool,
}

impl SettingsChoiceRow {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            status: None,
            selected: false,
            navigates: false,
            separator: true,
        }
    }

    /// Configures or reads the `status` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Configures or reads the `selected` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Configures or reads the `navigates` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn navigates(mut self, navigates: bool) -> Self {
        self.navigates = navigates;
        self
    }

    /// Configures or reads the `separator` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn separator(mut self, separator: bool) -> Self {
        self.separator = separator;
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
    pub fn show(self, ui: &mut Ui) -> SettingsChoiceResponse {
        let p = ui_palette(ui);
        let (rect, row_response) =
            ui.allocate_exact_size(vec2(ui.available_width(), 70.0), Sense::hover());
        let navigation_width = if self.navigates { 96.0 } else { 0.0 };
        let selection_rect = egui::Rect::from_min_max(
            rect.left_top(),
            pos2(rect.right() - navigation_width, rect.bottom()),
        );
        let response = ui.interact(
            selection_rect,
            row_response.id.with("select"),
            Sense::click(),
        );
        let navigation_response = if self.navigates {
            let nav_rect =
                egui::Rect::from_min_max(selection_rect.right_top(), rect.right_bottom());
            Some(ui.interact(nav_rect, row_response.id.with("navigate"), Sense::click()))
        } else {
            None
        };

        ui.painter().rect_filled(rect, 0, p.card_bg);
        let selection_fill = animated_fill(
            ui,
            response.id.with("choice_hover"),
            Color32::TRANSPARENT,
            settings_hover_fill(ui, p),
            response.hovered(),
        );
        ui.painter().rect_filled(selection_rect, 0, selection_fill);
        if let Some(navigation_response) = &navigation_response {
            let navigation_fill = animated_fill(
                ui,
                navigation_response.id.with("choice_navigation_hover"),
                Color32::TRANSPARENT,
                settings_hover_fill(ui, p),
                navigation_response.hovered(),
            );
            ui.painter()
                .rect_filled(navigation_response.rect, 0, navigation_fill);
        }
        if self.separator {
            let y = rect.bottom() - 1.0;
            ui.painter().line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                Stroke::new(1.0, settings_separator(ui)),
            );
        }

        let radio_center = pos2(rect.left() + 68.0, rect.center().y);
        let hover_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("radio_hover"),
            response.hovered(),
            0.16,
            egui::emath::easing::cubic_out,
        );
        let pressed_t = ui.ctx().animate_bool_with_time_and_easing(
            response.id.with("radio_pressed"),
            response.is_pointer_button_down_on(),
            0.10,
            egui::emath::easing::cubic_out,
        );
        let inner_color = if ui.visuals().dark_mode {
            Color32::from_rgb(24, 24, 24)
        } else {
            Color32::from_rgb(250, 250, 250)
        };
        if self.selected {
            ui.painter().circle_filled(radio_center, 9.0, p.accent);
            let inner_radius = egui::lerp(3.0..=4.15, hover_t) * (1.0 - pressed_t * 0.18);
            ui.painter()
                .circle_filled(radio_center, inner_radius, inner_color);
        } else {
            ui.painter().circle(
                radio_center,
                9.0,
                p.card_bg,
                Stroke::new(1.0, p.text_secondary),
            );
            if hover_t > 0.0 {
                ui.painter()
                    .circle_filled(radio_center, 2.6 * hover_t, inner_color);
            }
        }

        let title_y = if self.status.is_some() { -17.0 } else { -9.0 };
        ui.painter().text(
            pos2(rect.left() + 88.0, rect.center().y + title_y + TEXT_Y_NUDGE),
            Align2::LEFT_CENTER,
            self.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(
                rect.left() + 88.0,
                rect.center().y + if self.status.is_some() { 0.0 } else { 11.0 } + TEXT_Y_NUDGE,
            ),
            Align2::LEFT_CENTER,
            self.subtitle,
            FontId::proportional(12.0),
            p.text_secondary,
        );
        if let Some(status) = self.status {
            ui.painter().text(
                pos2(rect.left() + 88.0, rect.center().y + 16.0 + TEXT_Y_NUDGE),
                Align2::LEFT_CENTER,
                status,
                FontId::proportional(12.0),
                p.text_secondary,
            );
        }

        if self.navigates {
            ui.painter().text(
                pos2(rect.right() - 22.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                FluentIcon::ChevronRight.glyph(),
                icon_font_id(12.0),
                p.text_primary,
            );
        }

        SettingsChoiceResponse {
            response,
            navigation_response,
        }
    }
}

/// Responses for the selectable region and the optional navigation chevron.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SettingsChoiceResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `navigation_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub navigation_response: Option<Response>,
}

impl SettingsChoiceResponse {
    /// Configures or reads the `navigation_clicked` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn navigation_clicked(&self) -> bool {
        self.navigation_response
            .as_ref()
            .is_some_and(Response::clicked)
    }
}

/// An inline Settings card whose body expands below its header.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SettingsExpander<'a> {
    id: egui::Id,
    open: &'a mut bool,
    title: String,
    description: String,
    trailing: String,
    icon: FluentIcon,
}

impl<'a> SettingsExpander<'a> {
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
        open: &'a mut bool,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            open,
            title: title.into(),
            description: description.into(),
            trailing: String::new(),
            icon: FluentIcon::Settings,
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Configures or reads the `trailing` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn trailing(mut self, trailing: impl Into<String>) -> Self {
        self.trailing = trailing.into();
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
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> SettingsExpanderResponse<R> {
        let p = ui_palette(ui);
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            self.id,
            *self.open,
        );
        state.set_open(*self.open);

        let framed = egui::Frame::new()
            .fill(p.card_bg)
            .stroke(Stroke::new(1.0, p.card_border))
            .corner_radius(0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let (rect, header_response) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 68.0), Sense::click());
                if header_response.clicked() {
                    *self.open = !*self.open;
                    state.set_open(*self.open);
                }
                let header_fill = animated_fill(
                    ui,
                    header_response.id.with("expander_hover"),
                    p.card_bg,
                    settings_hover_fill(ui, p),
                    header_response.hovered(),
                );
                ui.painter().rect_filled(rect, 0, header_fill);
                if *self.open {
                    let y = rect.bottom() - 1.0;
                    ui.painter().line_segment(
                        [pos2(rect.left(), y), pos2(rect.right(), y)],
                        Stroke::new(1.0, settings_separator(ui)),
                    );
                }
                ui.painter().text(
                    pos2(rect.left() + 28.0, rect.center().y + TEXT_Y_NUDGE),
                    Align2::CENTER_CENTER,
                    self.icon.glyph(),
                    icon_font_id(18.0),
                    p.text_primary,
                );
                ui.painter().text(
                    pos2(rect.left() + 58.0, rect.center().y - 9.0 + TEXT_Y_NUDGE),
                    Align2::LEFT_CENTER,
                    self.title,
                    FontId::proportional(14.0),
                    p.text_primary,
                );
                ui.painter().text(
                    pos2(rect.left() + 58.0, rect.center().y + 11.0 + TEXT_Y_NUDGE),
                    Align2::LEFT_CENTER,
                    self.description,
                    FontId::proportional(12.0),
                    p.text_secondary,
                );
                ui.painter().text(
                    pos2(rect.right() - 48.0, rect.center().y + TEXT_Y_NUDGE),
                    Align2::RIGHT_CENTER,
                    self.trailing,
                    FontId::proportional(13.0),
                    p.text_secondary,
                );
                ui.painter().text(
                    pos2(rect.right() - 20.0, rect.center().y + TEXT_Y_NUDGE),
                    Align2::CENTER_CENTER,
                    if *self.open {
                        FluentIcon::ChevronUp.glyph()
                    } else {
                        FluentIcon::ChevronDown.glyph()
                    },
                    icon_font_id(12.0),
                    p.text_primary,
                );

                let body = state
                    .show_body_unindented(ui, |ui| {
                        ui.set_min_width(rect.width());
                        add_contents(ui)
                    })
                    .map(|response| response.inner);
                (header_response, body)
            });

        SettingsExpanderResponse {
            response: framed.response,
            header_response: framed.inner.0,
            body: framed.inner.1,
        }
    }
}

/// Return value of [`SettingsExpander::show`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SettingsExpanderResponse<R> {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `header_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub header_response: Response,
    /// The `body` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub body: Option<R>,
}

/// A compact footer row for an expanded Settings card.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn settings_footer<R>(
    ui: &mut Ui,
    label: impl Into<String>,
    add_action: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let p = ui_palette(ui);
    let label = label.into();
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 50.0), Sense::hover());
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        Stroke::new(1.0, settings_separator(ui)),
    );
    ui.painter().text(
        pos2(rect.left() + 58.0, rect.center().y + TEXT_Y_NUDGE),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(13.0),
        p.text_primary,
    );
    let action_rect = egui::Rect::from_min_max(
        pos2(rect.right() - 180.0, rect.top()),
        pos2(rect.right() - 16.0, rect.bottom()),
    );
    let mut action_ui = ui.new_child(
        egui::UiBuilder::new()
            .id_salt(response.id.with("action"))
            .max_rect(action_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    let inner = add_action(&mut action_ui);
    InnerResponse { inner, response }
}

/// The command chosen in a [`ContentDialog`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentDialogAction {
    /// The `Primary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Primary,
    /// The `Secondary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Secondary,
    /// The `Close` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Close,
}

/// A compact, centered WinUI ContentDialog built on egui's modal container.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ContentDialog {
    id: egui::Id,
    title: String,
    content: String,
    primary_text: String,
    secondary_text: Option<String>,
    close_text: String,
    width: f32,
}

impl ContentDialog {
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
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            title: title.into(),
            content: content.into(),
            primary_text: "OK".into(),
            secondary_text: None,
            close_text: "Cancel".into(),
            width: 380.0,
        }
    }

    /// Configures or reads the `primary_text` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn primary_text(mut self, text: impl Into<String>) -> Self {
        self.primary_text = text.into();
        self
    }

    /// Configures or reads the `secondary_text` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn secondary_text(mut self, text: impl Into<String>) -> Self {
        self.secondary_text = Some(text.into());
        self
    }

    /// Configures or reads the `close_text` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn close_text(mut self, text: impl Into<String>) -> Self {
        self.close_text = text.into();
        self
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
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
    pub fn show(self, ctx: &egui::Context, theme: FluentTheme) -> ContentDialogResponse {
        let p = palette(theme);
        let width = self
            .width
            .min((ctx.content_rect().width() - 32.0).max(280.0));
        let mut action = None;
        let frame = egui::Frame::new()
            .fill(p.card_bg)
            .stroke(Stroke::new(1.0, p.card_border))
            .corner_radius(0)
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 32,
                spread: 0,
                color: p.shadow,
            });
        let modal = egui::Modal::new(self.id)
            .backdrop_color(if theme.is_dark() {
                Color32::from_black_alpha(105)
            } else {
                Color32::from_black_alpha(58)
            })
            .frame(frame)
            .show(ctx, |ui| {
                ui.set_width(width);
                let (header_rect, _) = ui.allocate_exact_size(vec2(width, 54.0), Sense::hover());
                ui.painter().text(
                    pos2(
                        header_rect.left() + 20.0,
                        header_rect.center().y + TEXT_Y_NUDGE,
                    ),
                    Align2::LEFT_CENTER,
                    self.title,
                    FontId::proportional(20.0),
                    p.text_primary,
                );
                let close_rect = egui::Rect::from_center_size(
                    pos2(header_rect.right() - 22.0, header_rect.center().y),
                    Vec2::splat(32.0),
                );
                let close_response =
                    ui.interact(close_rect, self.id.with("close_icon"), Sense::click());
                if close_response.clicked() {
                    action = Some(ContentDialogAction::Close);
                }
                ui.painter().rect_filled(
                    close_rect,
                    0,
                    if close_response.hovered() {
                        p.subtle_hover
                    } else {
                        Color32::TRANSPARENT
                    },
                );
                ui.painter().text(
                    close_rect.center(),
                    Align2::CENTER_CENTER,
                    FluentIcon::Close.glyph(),
                    icon_font_id(14.0),
                    p.text_primary,
                );

                egui::Frame::new()
                    .inner_margin(Margin::symmetric(20, 12))
                    .show(ui, |ui| {
                        ui.set_min_width(width - 40.0);
                        ui.label(egui::RichText::new(self.content).size(14.0));
                        ui.add_space(12.0);
                    });
                ui.painter().line_segment(
                    [
                        pos2(ui.min_rect().left(), ui.cursor().top()),
                        pos2(ui.min_rect().right(), ui.cursor().top()),
                    ],
                    Stroke::new(1.0, p.card_border),
                );
                egui::Frame::new()
                    .inner_margin(Margin::symmetric(16, 14))
                    .show(ui, |ui| {
                        ui.set_min_width(width - 32.0);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.add(AccentButton::new(self.primary_text)).clicked() {
                                action = Some(ContentDialogAction::Primary);
                            }
                            if ui.add(FluentButton::new(self.close_text)).clicked() {
                                action = Some(ContentDialogAction::Close);
                            }
                            if let Some(secondary_text) = self.secondary_text
                                && ui.add(FluentButton::new(secondary_text)).clicked()
                            {
                                action = Some(ContentDialogAction::Secondary);
                            }
                        });
                    });
            });

        let escape = modal.is_top_modal
            && !modal.any_popup_open
            && ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        ContentDialogResponse {
            action,
            should_close: action.is_some() || escape,
        }
    }
}

/// Result returned by [`ContentDialog::show`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ContentDialogResponse {
    /// The `action` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub action: Option<ContentDialogAction>,
    /// The `should_close` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub should_close: bool,
}

/// Determinate (`Some`) or indeterminate (`None`) circular progress indicator.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ProgressRing {
    progress: Option<f32>,
    size: f32,
}

impl ProgressRing {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(progress: Option<f32>) -> Self {
        Self {
            progress,
            size: 28.0,
        }
    }

    /// Configures or reads the `size` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Widget for ProgressRing {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());
        let (start, sweep) = if let Some(progress) = self.progress {
            (-FRAC_PI_2, TAU * progress.clamp(0.0, 1.0))
        } else {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(16));
            let time = ui.input(|i| i.time as f32);
            (time * TAU * 0.72, TAU * 0.72)
        };
        let radius = self.size * 0.5 - 3.0;
        ui.painter()
            .circle_stroke(rect.center(), radius, Stroke::new(2.0, p.control_border));
        let points = (0..=28)
            .map(|i| {
                let angle = start + sweep * (i as f32 / 28.0);
                rect.center() + vec2(angle.cos(), angle.sin()) * radius
            })
            .collect();
        ui.painter()
            .add(Shape::line(points, Stroke::new(2.4, p.accent)));
        response
    }
}

/// NavigationView row with animated hover and selection indicator.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn navigation_item(
    ui: &mut Ui,
    icon: FluentIcon,
    label: impl Into<String>,
    selected: bool,
) -> Response {
    let label = label.into();
    let p = ui_palette(ui);
    let desired = vec2(ui.available_width(), 36.0);
    let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
    let selected_fill = if ui.visuals().dark_mode {
        Color32::from_rgba_unmultiplied(52, 52, 52, 236)
    } else {
        Color32::from_rgba_unmultiplied(225, 225, 225, 238)
    };
    let hover_fill = if ui.visuals().dark_mode {
        Color32::from_rgba_unmultiplied(45, 45, 45, 220)
    } else {
        Color32::from_rgba_unmultiplied(235, 235, 235, 230)
    };
    let fill = if selected {
        selected_fill
    } else if response.hovered() {
        hover_fill
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect.shrink2(vec2(4.0, 2.0)), 4, fill);
    let selection_t = ui
        .ctx()
        .animate_bool_responsive(response.id.with("selected"), selected);
    if selection_t > 0.0 {
        let bar = egui::Rect::from_center_size(
            pos2(rect.left() + 4.0, rect.center().y),
            vec2(3.0, 14.0 * selection_t),
        );
        ui.painter().rect_filled(bar, 2, p.accent);
    }
    ui.painter().text(
        pos2(rect.left() + 28.0, rect.center().y + 0.5),
        Align2::CENTER_CENTER,
        icon.glyph(),
        icon_font_id(15.0),
        p.text_primary,
    );
    let text_clip = Rect::from_min_max(
        pos2(rect.left() + 52.0, rect.top()),
        pos2(rect.right() - 14.0, rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        pos2(rect.left() + 52.0, rect.center().y + TEXT_Y_NUDGE),
        Align2::LEFT_CENTER,
        &label,
        TextStyle::Button.resolve(ui.style()),
        p.text_primary,
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            &label,
        )
    });
    response
}

/// Semantic style of an [`InfoBar`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InfoBarKind {
    #[default]
    /// The `Informational` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Informational,
    /// The `Success` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Success,
    /// The `Warning` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Warning,
    /// The `Error` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Error,
}

/// A WinUI-style three-state checkbox backed by `Option<bool>`.
///
/// `Some(false)` is unchecked, `Some(true)` is checked, and `None` is indeterminate.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TriStateCheckBox<'a> {
    value: &'a mut Option<bool>,
    label: String,
}

impl<'a> TriStateCheckBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut Option<bool>, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

impl Widget for TriStateCheckBox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let desired = vec2(ui.available_width().min(420.0), 32.0);
        let (rect, mut response) = ui.allocate_exact_size(desired, Sense::click());
        if response.clicked() {
            *self.value = match *self.value {
                Some(false) => Some(true),
                Some(true) => None,
                None => Some(false),
            };
            response.mark_changed();
        }
        let box_rect =
            Rect::from_center_size(pos2(rect.left() + 10.0, rect.center().y), vec2(20.0, 20.0));
        let selected = !matches!(*self.value, Some(false));
        ui.painter().rect(
            box_rect,
            3.0,
            if selected { p.accent } else { p.control_bg },
            Stroke::new(1.0, if selected { p.accent } else { p.control_border }),
            StrokeKind::Inside,
        );
        match *self.value {
            Some(true) => {
                ui.painter().text(
                    box_rect.center(),
                    Align2::CENTER_CENTER,
                    FluentIcon::Check.glyph(),
                    icon_font_id(12.0),
                    p.on_accent,
                );
            }
            None => {
                ui.painter().line_segment(
                    [
                        box_rect.left_center() + vec2(5.0, 0.0),
                        box_rect.right_center() - vec2(5.0, 0.0),
                    ],
                    Stroke::new(2.0, p.on_accent),
                );
            }
            Some(false) => {}
        }
        ui.painter().text(
            pos2(box_rect.right() + 8.0, rect.center().y),
            Align2::LEFT_CENTER,
            &self.label,
            FontId::proportional(14.0),
            p.text_primary,
        );
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::Checkbox,
                ui.is_enabled(),
                selected,
                &self.label,
            )
        });
        response
    }
}

/// WinUI InfoBar for status and contextual messages.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct InfoBar {
    title: String,
    message: String,
    kind: InfoBarKind,
    closable: bool,
    icon_visible: bool,
}

impl InfoBar {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            kind: InfoBarKind::Informational,
            closable: false,
            icon_visible: true,
        }
    }

    /// Configures or reads the `kind` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn kind(mut self, kind: InfoBarKind) -> Self {
        self.kind = kind;
        self
    }

    /// Configures or reads the `closable` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Configures or reads the `icon_visible` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon_visible(mut self, visible: bool) -> Self {
        self.icon_visible = visible;
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
        let p = ui_palette(ui);
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), 62.0), Sense::hover());
        ui.painter().rect(
            rect,
            4,
            p.info_bg,
            Stroke::new(1.0, p.info_border),
            StrokeKind::Inside,
        );
        let (icon, color) = match self.kind {
            InfoBarKind::Informational => (FluentIcon::Info, p.accent),
            InfoBarKind::Success => (FluentIcon::Check, p.success),
            InfoBarKind::Warning => (FluentIcon::Info, p.warning),
            InfoBarKind::Error => (FluentIcon::Close, p.danger),
        };
        if self.icon_visible {
            ui.painter().text(
                pos2(rect.left() + 24.0, rect.center().y),
                Align2::CENTER_CENTER,
                icon.glyph(),
                icon_font_id(16.0),
                color,
            );
        }
        let text_left = rect.left() + if self.icon_visible { 48.0 } else { 16.0 };
        ui.painter().text(
            pos2(text_left, rect.top() + 19.0),
            Align2::LEFT_CENTER,
            self.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(text_left, rect.bottom() - 18.0),
            Align2::LEFT_CENTER,
            self.message,
            FontId::proportional(13.0),
            p.text_secondary,
        );
        let mut response = response;
        if self.closable {
            let close_rect = egui::Rect::from_center_size(
                pos2(rect.right() - 22.0, rect.center().y),
                vec2(32.0, 32.0),
            );
            let close = ui.interact(close_rect, response.id.with("close"), Sense::click());
            if close.hovered() {
                ui.painter().rect_filled(close_rect, 0, p.subtle_hover);
            }
            ui.painter().text(
                close_rect.center(),
                Align2::CENTER_CENTER,
                FluentIcon::Close.glyph(),
                icon_font_id(11.0),
                p.text_primary,
            );
            response |= close;
        }
        response
    }
}

/// Creates two card columns with the compact WinUI section gap.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn card_columns<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut [Ui]) -> R) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.columns(2, add_contents)
    })
    .inner
}

/// Groups Settings cards with the native compact 5-point vertical gap.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn settings_group<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        add_contents(ui)
    })
    .inner
}

/// Draws a titled card and returns the inner closure response.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn section_card<R>(
    ui: &mut Ui,
    theme: FluentTheme,
    title: impl Into<String>,
    subtitle: impl Into<String>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let title = title.into();
    let subtitle = subtitle.into();
    card_frame(theme).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.label(egui::RichText::new(title).size(16.0).strong());
        if !subtitle.is_empty() {
            ui.label(
                egui::RichText::new(subtitle)
                    .size(13.0)
                    .color(ui_palette(ui).text_secondary),
            );
        }
        ui.add_space(8.0);
        add_contents(ui)
    })
}

/// An indented NavigationView child row without a leading icon.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn navigation_child_item(ui: &mut Ui, label: impl Into<String>, selected: bool) -> Response {
    let label = label.into();
    let p = ui_palette(ui);
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let fill = if selected {
        if ui.visuals().dark_mode {
            Color32::from_rgba_unmultiplied(52, 52, 52, 236)
        } else {
            Color32::from_rgba_unmultiplied(225, 225, 225, 238)
        }
    } else if response.hovered() {
        if ui.visuals().dark_mode {
            Color32::from_rgba_unmultiplied(45, 45, 45, 220)
        } else {
            Color32::from_rgba_unmultiplied(235, 235, 235, 230)
        }
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect.shrink2(vec2(4.0, 2.0)), 4, fill);
    let selection_t = ui
        .ctx()
        .animate_bool_responsive(response.id.with("selected"), selected);
    if selection_t > 0.0 {
        let bar = egui::Rect::from_center_size(
            pos2(rect.left() + 4.0, rect.center().y),
            vec2(3.0, 14.0 * selection_t),
        );
        ui.painter().rect_filled(bar, 2, p.accent);
    }
    let text_clip = Rect::from_min_max(
        pos2(rect.left() + 52.0, rect.top()),
        pos2(rect.right() - 14.0, rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        pos2(rect.left() + 52.0, rect.center().y + TEXT_Y_NUDGE),
        Align2::LEFT_CENTER,
        label,
        TextStyle::Button.resolve(ui.style()),
        p.text_primary,
    );
    response
}

/// An indented NavigationView branch row with a Fluent outline chevron.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn navigation_branch_item(
    ui: &mut Ui,
    label: impl Into<String>,
    selected: bool,
    open: bool,
) -> Response {
    let label = label.into();
    let p = ui_palette(ui);
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), Sense::click());
    let fill = if selected {
        if ui.visuals().dark_mode {
            Color32::from_rgba_unmultiplied(52, 52, 52, 236)
        } else {
            Color32::from_rgba_unmultiplied(225, 225, 225, 238)
        }
    } else if response.hovered() {
        if ui.visuals().dark_mode {
            Color32::from_rgba_unmultiplied(45, 45, 45, 220)
        } else {
            Color32::from_rgba_unmultiplied(235, 235, 235, 230)
        }
    } else {
        Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect.shrink2(vec2(4.0, 2.0)), 4, fill);
    let selection_t = ui
        .ctx()
        .animate_bool_responsive(response.id.with("selected"), selected);
    if selection_t > 0.0 {
        let bar = Rect::from_center_size(
            pos2(rect.left() + 4.0, rect.center().y),
            vec2(3.0, 14.0 * selection_t),
        );
        ui.painter().rect_filled(bar, 2, p.accent);
    }
    let text_clip = Rect::from_min_max(
        pos2(rect.left() + 52.0, rect.top()),
        pos2(rect.right() - 32.0, rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        pos2(rect.left() + 52.0, rect.center().y + TEXT_Y_NUDGE),
        Align2::LEFT_CENTER,
        label,
        TextStyle::Button.resolve(ui.style()),
        p.text_primary,
    );
    ui.painter().text(
        pos2(response.rect.right() - 18.0, response.rect.center().y),
        Align2::CENTER_CENTER,
        if open {
            FluentIcon::ChevronUp.glyph()
        } else {
            FluentIcon::ChevronDown.glyph()
        },
        icon_font_id(11.0),
        ui_palette(ui).text_primary,
    );
    response
}

/// Expandable NavigationView group with an animated child region.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct NavigationGroup<'a> {
    id: egui::Id,
    open: &'a mut bool,
    icon: FluentIcon,
    label: String,
    selected: bool,
}

impl<'a> NavigationGroup<'a> {
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
        open: &'a mut bool,
        icon: FluentIcon,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            open,
            icon,
            label: label.into(),
            selected: false,
        }
    }

    /// Configures or reads the `selected` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_children: impl FnOnce(&mut Ui) -> R,
    ) -> NavigationGroupResponse<R> {
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            self.id,
            *self.open,
        );
        state.set_open(*self.open);
        let header = navigation_item(ui, self.icon, self.label, self.selected);
        ui.painter().text(
            pos2(header.rect.right() - 18.0, header.rect.center().y),
            Align2::CENTER_CENTER,
            if *self.open {
                FluentIcon::ChevronUp.glyph()
            } else {
                FluentIcon::ChevronDown.glyph()
            },
            icon_font_id(11.0),
            ui_palette(ui).text_primary,
        );
        if header.clicked() {
            *self.open = !*self.open;
            state.set_open(*self.open);
        }
        let body = state
            .show_body_unindented(ui, add_children)
            .map(|response| response.inner);
        NavigationGroupResponse {
            header_response: header,
            body,
        }
    }
}

/// Result returned by [`NavigationGroup::show`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct NavigationGroupResponse<R> {
    /// The `header_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub header_response: Response,
    /// The `body` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub body: Option<R>,
}

/// Draws a Settings-style card row with a right-aligned control area.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn setting_row(
    ui: &mut Ui,
    theme: FluentTheme,
    title: impl Into<String>,
    description: impl Into<String>,
    add_control: impl FnOnce(&mut Ui),
) -> Response {
    let title = title.into();
    let description = description.into();
    let p = palette(theme);
    let mut add_control = Some(add_control);
    card_frame(theme)
        .inner_margin(Margin::symmetric(16, 0))
        .show(ui, |ui| {
            let width = ui.available_width();
            let draw_text = |ui: &mut Ui| {
                ui.label(egui::RichText::new(&title).size(14.0));
                ui.label(
                    egui::RichText::new(&description)
                        .size(12.0)
                        .color(p.text_secondary),
                );
            };
            if width >= 680.0 {
                ui.allocate_ui_with_layout(
                    vec2(width, 68.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        let (text_rect, _) =
                            ui.allocate_exact_size(vec2(width - 370.0, 68.0), Sense::hover());
                        ui.painter().text(
                            pos2(text_rect.left(), text_rect.center().y - 9.0 + TEXT_Y_NUDGE),
                            Align2::LEFT_CENTER,
                            &title,
                            FontId::proportional(14.0),
                            p.text_primary,
                        );
                        ui.painter().text(
                            pos2(text_rect.left(), text_rect.center().y + 11.0 + TEXT_Y_NUDGE),
                            Align2::LEFT_CENTER,
                            &description,
                            FontId::proportional(12.0),
                            p.text_secondary,
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if let Some(add_control) = add_control.take() {
                                add_control(ui);
                            }
                        });
                    },
                );
            } else {
                ui.add_space(10.0);
                draw_text(ui);
                ui.add_space(6.0);
                if let Some(add_control) = add_control.take() {
                    add_control(ui);
                }
                ui.add_space(10.0);
            }
        })
        .response
}

/// Applies a subtle Fluent page-change fade and vertical motion to a content region.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn animated_page<R>(
    ui: &mut Ui,
    animation_id: impl egui::AsIdSalt,
    revision: u64,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let id = ui.make_persistent_id(animation_id);
    let animated =
        ui.ctx()
            .animate_value_with_time(id, revision as f32, ui.style().animation_time.max(0.001));
    let distance = (revision as f32 - animated).abs().min(1.0);
    ui.scope(|ui| {
        ui.multiply_opacity(1.0 - distance * 0.35);
        ui.add_space(distance * 12.0);
        add_contents(ui)
    })
}

/// Animated, anchored notification surface.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct Toast {
    title: String,
    message: String,
    icon: FluentIcon,
}

impl Toast {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            icon: FluentIcon::Check,
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = icon;
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
    pub fn show(self, ctx: &egui::Context, theme: FluentTheme, visible: bool) {
        let id = egui::Id::new(("fluent_toast", &self.title));
        let t = ctx.animate_bool_with_time(id, visible, 0.20);
        if t <= 0.001 {
            return;
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        egui::Area::new(id.with("area"))
            .anchor(Align2::RIGHT_BOTTOM, vec2(-24.0, -24.0 + (1.0 - t) * 18.0))
            .order(egui::Order::Tooltip)
            .interactable(false)
            .show(ctx, |ui| {
                ui.multiply_opacity(t);
                card_frame(theme)
                    .inner_margin(Margin::symmetric(16, 12))
                    .show(ui, |ui| {
                        ui.set_width(290.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(self.icon.glyph())
                                    .font(icon_font_id(16.0))
                                    .color(ui_palette(ui).accent),
                            );
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(self.title).strong());
                                ui.label(
                                    egui::RichText::new(self.message)
                                        .size(12.0)
                                        .color(ui_palette(ui).text_secondary),
                                );
                            });
                        });
                    });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_glyphs_are_private_use_characters() {
        assert!(FluentIcon::Home.glyph() >= '\u{E000}');
    }

    #[test]
    fn button_builders_keep_requested_variant() {
        let button = FluentButton::accent("Save").icon(FluentIcon::Check);
        assert_eq!(button.kind, ButtonKind::Accent);
        assert_eq!(button.icon, Some(FluentIcon::Check));
    }

    #[test]
    fn color_animation_does_not_premultiply_twice() {
        let translucent = Color32::from_rgba_unmultiplied(255, 255, 255, 190);
        assert_eq!(mix(translucent, Color32::BLACK, 0.0), translucent);
    }
}
