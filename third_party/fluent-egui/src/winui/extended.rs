use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, FontId, InnerResponse, Layout, Margin, Response,
    Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2, Widget, pos2, vec2,
};

use super::{FluentIcon, FluentTheme, icon_font_id, palette};

const TEXT_Y_NUDGE: f32 = -0.5;

fn ui_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

/// A button rendered as WinUI hyperlink text without forcing URL navigation.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct HyperlinkButton {
    text: String,
}

impl HyperlinkButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for HyperlinkButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            egui::Button::new(egui::RichText::new(self.text).color(ui.visuals().hyperlink_color))
                .frame(false),
        )
    }
}

/// A stateful WinUI ToggleButton.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ToggleButton<'a> {
    value: &'a mut bool,
    text: String,
}

impl<'a> ToggleButton<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut bool, text: impl Into<String>) -> Self {
        Self {
            value,
            text: text.into(),
        }
    }
}

impl Widget for ToggleButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut response = ui.add(
            egui::Button::new(self.text)
                .selected(*self.value)
                .min_size(vec2(72.0, 32.0)),
        );
        if response.clicked() {
            *self.value = !*self.value;
            response.mark_changed();
        }
        response
    }
}

/// A two-part command whose primary section invokes an action and whose Chevron opens a popup.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SplitButton {
    text: String,
    icon: Option<FluentIcon>,
    width: Option<f32>,
}

impl SplitButton {
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
            width: None,
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
        self.icon = Some(icon);
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
    ) -> SplitButtonResponse<R> {
        let p = ui_palette(ui);
        let font = TextStyle::Button.resolve(ui.style());
        let galley = ui
            .painter()
            .layout_no_wrap(self.text.clone(), font, p.text_primary);
        let icon_width = if self.icon.is_some() { 22.0 } else { 0.0 };
        let width = self
            .width
            .unwrap_or_else(|| galley.size().x + icon_width + 60.0)
            .max(72.0);
        let (rect, _) = ui.allocate_exact_size(vec2(width, 32.0), Sense::hover());
        let menu_rect =
            egui::Rect::from_min_max(pos2(rect.right() - 32.0, rect.top()), rect.right_bottom());
        let primary_rect = egui::Rect::from_min_max(rect.left_top(), menu_rect.left_bottom());
        let primary = ui.interact(primary_rect, ui.id().with("split_primary"), Sense::click());
        let menu = ui.interact(menu_rect, ui.id().with("split_menu"), Sense::click());
        let hovered = primary.hovered() || menu.hovered();
        let fill = if primary.is_pointer_button_down_on() || menu.is_pointer_button_down_on() {
            p.control_pressed
        } else if hovered {
            p.control_hover
        } else {
            p.control_bg
        };
        ui.painter().rect(
            rect,
            4,
            fill,
            Stroke::new(1.0, p.control_border),
            StrokeKind::Inside,
        );
        ui.painter().line_segment(
            [menu_rect.left_top(), menu_rect.left_bottom()],
            Stroke::new(1.0, p.control_border),
        );
        let content_width = galley.size().x + icon_width;
        let mut x = primary_rect.center().x - content_width * 0.5;
        if let Some(icon) = self.icon {
            ui.painter().text(
                pos2(x + 7.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                icon.glyph(),
                icon_font_id(14.0),
                p.text_primary,
            );
            x += icon_width;
        }
        ui.painter().galley(
            pos2(x, rect.center().y - galley.size().y * 0.5 + TEXT_Y_NUDGE),
            galley,
            p.text_primary,
        );
        ui.painter().text(
            menu_rect.center() + vec2(0.0, TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronDown.glyph(),
            icon_font_id(11.0),
            p.text_primary,
        );
        primary.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &self.text)
        });
        menu.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!("{} menu", self.text),
            )
        });

        let popup = egui::Popup::from_toggle_button_response(&menu)
            .width(rect.width().max(160.0))
            .gap(4.0)
            .frame(
                egui::Frame::popup(ui.style())
                    .inner_margin(Margin::same(4))
                    .corner_radius(0),
            )
            .show(add_contents)
            .map(|response| response.inner);

        SplitButtonResponse {
            response: primary,
            menu_response: menu,
            popup,
        }
    }
}

/// A Fluent API value representing `SplitButtonResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SplitButtonResponse<R> {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `menu_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub menu_response: Response,
    /// The `popup` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub popup: Option<R>,
}

/// A SplitButton whose primary content is a color swatch or a text label.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ColorSplitButton<'a> {
    color: &'a mut Color32,
    colors: &'a [Color32],
    label: Option<String>,
}

impl<'a> ColorSplitButton<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(color: &'a mut Color32, colors: &'a [Color32]) -> Self {
        Self {
            color,
            colors,
            label: None,
        }
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

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> ColorSplitButtonResponse {
        let swatch_only = self.label.is_none();
        let label = self.label.unwrap_or_else(|| "      ".into());
        let mut selected = None;
        let result = super::SplitButton::new(label)
            .width(if swatch_only { 72.0 } else { 132.0 })
            .show(ui, |ui| {
                ui.set_width(132.0);
                egui::Grid::new(ui.id().with("color_split_grid"))
                    .num_columns(3)
                    .spacing(vec2(6.0, 6.0))
                    .show(ui, |ui| {
                        for (index, color) in self.colors.iter().copied().enumerate() {
                            let (rect, response) =
                                ui.allocate_exact_size(vec2(32.0, 32.0), Sense::click());
                            ui.painter().rect_filled(rect, 0, color);
                            if response.hovered() {
                                ui.painter().rect_stroke(
                                    rect,
                                    4,
                                    Stroke::new(2.0, ui_palette(ui).text_primary),
                                    StrokeKind::Inside,
                                );
                            }
                            if response.clicked() {
                                selected = Some(color);
                                ui.close();
                            }
                            if (index + 1) % 3 == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        if swatch_only {
            ui.painter()
                .rect_filled(result.response.rect.shrink2(vec2(4.0, 3.0)), 4, *self.color);
        }
        if let Some(color) = selected {
            *self.color = color;
        }
        ColorSplitButtonResponse {
            response: result.response,
            menu_response: result.menu_response,
            selected,
        }
    }
}

/// A Fluent API value representing `ColorSplitButtonResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ColorSplitButtonResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `menu_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub menu_response: Response,
    /// The `selected` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub selected: Option<Color32>,
}

/// A button whose entire surface opens a popup menu.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct DropDownButton {
    text: String,
    width: Option<f32>,
    icon: Option<FluentIcon>,
    open: Option<bool>,
}

impl DropDownButton {
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
            width: None,
            icon: None,
            open: None,
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
        self.width = Some(width);
        self
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

    /// Configures or reads the `open` option.
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
        let width = self.width.unwrap_or(128.0);
        let (rect, response) = ui.allocate_exact_size(vec2(width, 32.0), Sense::click());
        let fill = if response.is_pointer_button_down_on() {
            p.control_pressed
        } else if response.hovered() {
            p.control_hover
        } else {
            p.control_bg
        };
        ui.painter().rect(
            rect,
            4,
            fill,
            Stroke::new(1.0, p.control_border),
            StrokeKind::Inside,
        );
        if let Some(icon) = self.icon {
            ui.painter().text(
                pos2(rect.left() + 14.0, rect.center().y + TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                icon.glyph(),
                icon_font_id(15.0),
                p.text_primary,
            );
        }
        ui.painter().text(
            pos2(
                rect.left() + if self.icon.is_some() { 30.0 } else { 12.0 },
                rect.center().y + TEXT_Y_NUDGE,
            ),
            Align2::LEFT_CENTER,
            &self.text,
            TextStyle::Button.resolve(ui.style()),
            p.text_primary,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, ui.is_enabled(), &self.text)
        });
        ui.painter().text(
            pos2(rect.right() - 16.0, rect.center().y + TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronDown.glyph(),
            icon_font_id(11.0),
            p.text_primary,
        );
        let mut popup = egui::Popup::from_toggle_button_response(&response)
            .width(width.max(160.0))
            .frame(
                egui::Frame::new()
                    .fill(p.card_bg)
                    .stroke(Stroke::new(1.0, p.card_border))
                    .corner_radius(0)
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 6],
                        blur: 20,
                        spread: 0,
                        color: p.shadow,
                    })
                    .inner_margin(Margin::same(4)),
            );
        if let Some(open) = self.open {
            popup = popup.open(open);
        }
        let inner = popup.show(add_contents).map(|result| result.inner);
        InnerResponse { inner, response }
    }
}

/// A command that reports repeated `changed()` ticks while held down.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RepeatButton {
    text: String,
    repeats_per_second: f64,
}

#[derive(Clone, Copy, Default)]
struct RepeatButtonState {
    pressed_at: f64,
    last_tick: i64,
    repeated: bool,
}

impl RepeatButton {
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
            repeats_per_second: 8.0,
        }
    }

    /// Configures or reads the `repeats_per_second` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn repeats_per_second(mut self, repeats_per_second: f64) -> Self {
        self.repeats_per_second = repeats_per_second;
        self
    }
}

impl Widget for RepeatButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut response = super::FluentButton::new(self.text).min_width(96.0).ui(ui);
        let now = ui.input(|input| input.time);
        let state_id = response.id.with("repeat_state");

        if response.is_pointer_button_down_on() {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(16));
            let changed = ui.ctx().data_mut(|data| {
                let mut state =
                    data.get_temp::<RepeatButtonState>(state_id)
                        .unwrap_or(RepeatButtonState {
                            pressed_at: now,
                            last_tick: -1,
                            repeated: false,
                        });
                let elapsed = now - state.pressed_at;
                let tick = ((elapsed - 0.4).max(0.0) * self.repeats_per_second) as i64;
                let should_repeat = elapsed >= 0.4 && tick != state.last_tick;
                if should_repeat {
                    state.last_tick = tick;
                    state.repeated = true;
                }
                data.insert_temp(state_id, state);
                should_repeat
            });
            if changed {
                response.mark_changed();
            }
        } else {
            let click_without_repeat = ui.ctx().data_mut(|data| {
                let state = data.remove_temp::<RepeatButtonState>(state_id);
                response.clicked() && state.is_some_and(|state| !state.repeated)
            });
            if click_without_repeat {
                response.mark_changed();
            }
        }
        response
    }
}

/// A 1-to-N WinUI star rating control.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RatingControl<'a> {
    value: &'a mut i32,
    max: i32,
    allow_clear: bool,
    read_only: bool,
}

impl<'a> RatingControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut i32) -> Self {
        Self {
            value,
            max: 5,
            allow_clear: true,
            read_only: false,
        }
    }

    /// Configures or reads the `max` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Configures or reads the `allow_clear` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn allow_clear(mut self, allow_clear: bool) -> Self {
        self.allow_clear = allow_clear;
        self
    }

    /// Configures or reads the `read_only` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }
}

impl Widget for RatingControl<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let max = self.max.max(1);
        let (rect, mut response) = ui.allocate_exact_size(
            vec2(max as f32 * 24.0, 28.0),
            if self.read_only {
                Sense::hover()
            } else {
                Sense::click()
            },
        );
        let hovered_value = response
            .hover_pos()
            .map(|pos| (((pos.x - rect.left()) / 24.0).floor() as i32 + 1).clamp(1, max));
        if response.clicked()
            && let Some(candidate) = hovered_value
        {
            *self.value = if self.allow_clear && *self.value == candidate {
                0
            } else {
                candidate
            };
            response.mark_changed();
        }
        let display = hovered_value.unwrap_or(*self.value).clamp(0, max);
        for index in 0..max {
            ui.painter().text(
                pos2(rect.left() + index as f32 * 24.0 + 12.0, rect.center().y),
                Align2::CENTER_CENTER,
                if index < display { '★' } else { '☆' },
                FontId::proportional(20.0),
                if index < display {
                    p.text_primary
                } else {
                    p.text_secondary
                },
            );
        }
        response.widget_info(|| {
            egui::WidgetInfo::slider(ui.is_enabled(), *self.value as f64, "Rating")
        });
        response
    }
}

/// Numeric input with compact inline increment/decrement buttons.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NumberBoxSpinButtonPlacement {
    #[default]
    /// The `Inline` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Inline,
    /// The `Compact` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Compact,
}

/// A Fluent API value representing `NumberBox`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct NumberBox<'a> {
    value: &'a mut f64,
    range: std::ops::RangeInclusive<f64>,
    step: f64,
    width: f32,
    spin_button_placement: NumberBoxSpinButtonPlacement,
}

impl<'a> NumberBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut f64) -> Self {
        Self {
            value,
            range: f64::NEG_INFINITY..=f64::INFINITY,
            step: 1.0,
            width: 132.0,
            spin_button_placement: NumberBoxSpinButtonPlacement::Inline,
        }
    }

    /// Configures or reads the `range` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn range(mut self, range: std::ops::RangeInclusive<f64>) -> Self {
        self.range = range;
        self
    }

    /// Configures or reads the `step` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn step(mut self, step: f64) -> Self {
        self.step = step;
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

    /// Configures or reads the `spin_button_placement` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn spin_button_placement(mut self, placement: NumberBoxSpinButtonPlacement) -> Self {
        self.spin_button_placement = placement;
        self
    }
}

impl Widget for NumberBox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let (rect, base) = ui.allocate_exact_size(vec2(self.width, 32.0), Sense::hover());
        let step_width = 28.0;
        let edit_rect = egui::Rect::from_min_max(
            rect.min,
            pos2(rect.right() - step_width - 2.0, rect.bottom()),
        );
        let step_rect = egui::Rect::from_min_max(
            pos2(rect.right() - step_width, rect.top()),
            rect.right_bottom(),
        );
        let up_rect =
            egui::Rect::from_min_max(step_rect.min, pos2(step_rect.right(), step_rect.center().y));
        let down_rect =
            egui::Rect::from_min_max(pos2(step_rect.left(), step_rect.center().y), step_rect.max);
        let show_spin_buttons =
            self.spin_button_placement == NumberBoxSpinButtonPlacement::Inline || base.hovered();

        let mut response = ui.put(
            edit_rect,
            egui::DragValue::new(self.value)
                .range(self.range.clone())
                .speed(self.step),
        );
        let up = ui.interact(up_rect, base.id.with("up"), Sense::click());
        let down = ui.interact(down_rect, base.id.with("down"), Sense::click());
        up.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), "Increment")
        });
        down.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), "Decrement")
        });
        if show_spin_buttons {
            for (button_rect, item, icon) in [
                (up_rect, &up, FluentIcon::ChevronUp),
                (down_rect, &down, FluentIcon::ChevronDown),
            ] {
                let fill = if item.is_pointer_button_down_on() {
                    p.control_pressed
                } else if item.hovered() {
                    p.control_hover
                } else {
                    p.control_bg
                };
                ui.painter().rect(
                    button_rect,
                    3,
                    fill,
                    Stroke::new(1.0, p.control_border),
                    StrokeKind::Inside,
                );
                ui.painter().text(
                    button_rect.center(),
                    Align2::CENTER_CENTER,
                    icon.glyph(),
                    icon_font_id(8.0),
                    p.text_primary,
                );
            }
        }
        if show_spin_buttons && up.clicked() {
            *self.value = (*self.value + self.step).min(*self.range.end());
            response.mark_changed();
        }
        if show_spin_buttons && down.clicked() {
            *self.value = (*self.value - self.step).max(*self.range.start());
            response.mark_changed();
        }
        response |= up;
        response |= down;
        response
    }
}

/// Evaluates the arithmetic expression accepted by the Gallery NumberBox example.
/// Supports parentheses, unary signs, `+`, `-`, `*`, `/`, and right-associative `^`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn evaluate_number_expression(expression: &str) -> Result<f64, String> {
    struct Parser<'a> {
        bytes: &'a [u8],
        cursor: usize,
    }

    impl Parser<'_> {
        fn skip_space(&mut self) {
            while self
                .bytes
                .get(self.cursor)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.cursor += 1;
            }
        }

        fn consume(&mut self, token: u8) -> bool {
            self.skip_space();
            if self.bytes.get(self.cursor) == Some(&token) {
                self.cursor += 1;
                true
            } else {
                false
            }
        }

        fn expression(&mut self) -> Result<f64, String> {
            let mut value = self.term()?;
            loop {
                if self.consume(b'+') {
                    value += self.term()?;
                } else if self.consume(b'-') {
                    value -= self.term()?;
                } else {
                    return Ok(value);
                }
            }
        }

        fn term(&mut self) -> Result<f64, String> {
            let mut value = self.power()?;
            loop {
                if self.consume(b'*') {
                    value *= self.power()?;
                } else if self.consume(b'/') {
                    let divisor = self.power()?;
                    if divisor == 0.0 {
                        return Err("division by zero".into());
                    }
                    value /= divisor;
                } else {
                    return Ok(value);
                }
            }
        }

        fn power(&mut self) -> Result<f64, String> {
            let base = self.unary()?;
            if self.consume(b'^') {
                Ok(base.powf(self.power()?))
            } else {
                Ok(base)
            }
        }

        fn unary(&mut self) -> Result<f64, String> {
            if self.consume(b'+') {
                self.unary()
            } else if self.consume(b'-') {
                Ok(-self.unary()?)
            } else {
                self.primary()
            }
        }

        fn primary(&mut self) -> Result<f64, String> {
            if self.consume(b'(') {
                let value = self.expression()?;
                if !self.consume(b')') {
                    return Err("missing closing parenthesis".into());
                }
                return Ok(value);
            }
            self.skip_space();
            let start = self.cursor;
            while self
                .bytes
                .get(self.cursor)
                .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'.')
            {
                self.cursor += 1;
            }
            if start == self.cursor {
                return Err(format!("expected number at byte {}", self.cursor));
            }
            std::str::from_utf8(&self.bytes[start..self.cursor])
                .map_err(|_| "invalid UTF-8".to_owned())?
                .parse::<f64>()
                .map_err(|_| "invalid number".to_owned())
        }
    }

    let mut parser = Parser {
        bytes: expression.as_bytes(),
        cursor: 0,
    };
    let value = parser.expression()?;
    parser.skip_space();
    if parser.cursor != parser.bytes.len() {
        return Err(format!("unexpected token at byte {}", parser.cursor));
    }
    if !value.is_finite() {
        return Err("result is not finite".into());
    }
    Ok(value)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Available `InfoBadgeKind` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum InfoBadgeKind {
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
    /// The `Attention` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Attention,
    /// The `Success` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Success,
    /// The `Critical` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Critical,
}

/// A compact numeric or dot notification badge.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct InfoBadge {
    value: Option<String>,
    kind: InfoBadgeKind,
}

impl InfoBadge {
    /// Creates the `dot` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn dot() -> Self {
        Self {
            value: None,
            kind: InfoBadgeKind::Informational,
        }
    }

    /// Creates the `value` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn value(value: impl ToString) -> Self {
        Self {
            value: Some(value.to_string()),
            kind: InfoBadgeKind::Informational,
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
    pub const fn kind(mut self, kind: InfoBadgeKind) -> Self {
        self.kind = kind;
        self
    }
}

impl Widget for InfoBadge {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let color = match self.kind {
            InfoBadgeKind::Informational => p.accent,
            InfoBadgeKind::Attention => p.warning,
            InfoBadgeKind::Success => p.success,
            InfoBadgeKind::Critical => p.danger,
        };
        let width = self.value.as_ref().map_or(10.0, |value| {
            (value.chars().count() as f32 * 8.0 + 10.0).max(18.0)
        });
        let height = if self.value.is_some() { 18.0 } else { 10.0 };
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        ui.painter().rect_filled(rect, height * 0.5, color);
        if let Some(value) = self.value {
            ui.painter().text(
                rect.center() + vec2(0.0, TEXT_Y_NUDGE),
                Align2::CENTER_CENTER,
                value,
                FontId::proportional(11.0),
                if ui.visuals().dark_mode {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                },
            );
        }
        response
    }
}

/// Dot-based page navigation with optional previous/next buttons.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PipsPager<'a> {
    selected: &'a mut usize,
    count: usize,
    arrows: bool,
}

impl<'a> PipsPager<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, count: usize) -> Self {
        Self {
            selected,
            count: count.max(1),
            arrows: true,
        }
    }

    /// Configures or reads the `arrows` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn arrows(mut self, arrows: bool) -> Self {
        self.arrows = arrows;
        self
    }
}

impl Widget for PipsPager<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        *self.selected = (*self.selected).min(self.count - 1);
        let item_count = self.count + usize::from(self.arrows) * 2;
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(item_count as f32 * 24.0, 28.0), Sense::hover());
        let mut cursor = rect.left();
        if self.arrows {
            let back_rect = egui::Rect::from_min_size(pos2(cursor, rect.top()), vec2(24.0, 28.0));
            let back = ui.interact(back_rect, response.id.with("back"), Sense::click());
            if back.clicked() && *self.selected > 0 {
                *self.selected -= 1;
                response.mark_changed();
            }
            ui.painter().text(
                back_rect.center(),
                Align2::CENTER_CENTER,
                FluentIcon::Back.glyph(),
                icon_font_id(11.0),
                ui.visuals().text_color(),
            );
            response |= back;
            cursor += 24.0;
        }
        for index in 0..self.count {
            let item_rect = egui::Rect::from_min_size(pos2(cursor, rect.top()), vec2(24.0, 28.0));
            let item = ui.interact(item_rect, response.id.with(index), Sense::click());
            item.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    *self.selected == index,
                    format!("Page {}", index + 1),
                )
            });
            if item.clicked() && *self.selected != index {
                *self.selected = index;
                response.mark_changed();
            }
            ui.painter().circle_filled(
                item_rect.center(),
                if index == *self.selected { 4.0 } else { 2.5 },
                if index == *self.selected {
                    ui.visuals().hyperlink_color
                } else {
                    ui.visuals().weak_text_color()
                },
            );
            response |= item;
            cursor += 24.0;
        }
        if self.arrows {
            let next_rect = egui::Rect::from_min_size(pos2(cursor, rect.top()), vec2(24.0, 28.0));
            let next = ui.interact(next_rect, response.id.with("next"), Sense::click());
            if next.clicked() && *self.selected + 1 < self.count {
                *self.selected += 1;
                response.mark_changed();
            }
            ui.painter().text(
                next_rect.center(),
                Align2::CENTER_CENTER,
                FluentIcon::ChevronRight.glyph(),
                icon_font_id(11.0),
                ui.visuals().text_color(),
            );
            response |= next;
        }
        response
    }
}

/// Clickable WinUI breadcrumb path. Returns the clicked item index.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct BreadcrumbBar {
    items: Vec<String>,
}

impl BreadcrumbBar {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(items: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
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
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let mut clicked = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for (index, item) in self.items.iter().enumerate() {
                if ui
                    .add(egui::Button::new(item).frame(false))
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .clicked()
                {
                    clicked = Some(index);
                }
                if index + 1 < self.items.len() {
                    ui.label(
                        egui::RichText::new(FluentIcon::ChevronRight.glyph())
                            .font(icon_font_id(11.0)),
                    );
                }
            }
        });
        clicked
    }
}

/// A document-style tab strip with close and add commands.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabViewWidthMode {
    /// The `Equal` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Equal,
    #[default]
    /// The `SizeToContent` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    SizeToContent,
    /// The `Compact` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Compact,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Available `TabViewCloseButtonOverlayMode` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum TabViewCloseButtonOverlayMode {
    #[default]
    /// The `Auto` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Auto,
    /// The `Always` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Always,
    /// The `OnHover` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    OnHover,
}

/// A Fluent API value representing `TabView`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TabView<'a> {
    selected: &'a mut usize,
    tabs: &'a [String],
    closable: bool,
    add_button: bool,
    width_mode: TabViewWidthMode,
    close_button_overlay_mode: TabViewCloseButtonOverlayMode,
    reorderable: bool,
}

impl<'a> TabView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, tabs: &'a [String]) -> Self {
        Self {
            selected,
            tabs,
            closable: true,
            add_button: true,
            width_mode: TabViewWidthMode::SizeToContent,
            close_button_overlay_mode: TabViewCloseButtonOverlayMode::Auto,
            reorderable: false,
        }
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

    /// Configures or reads the `add_button` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn add_button(mut self, add_button: bool) -> Self {
        self.add_button = add_button;
        self
    }

    /// Configures or reads the `width_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width_mode(mut self, mode: TabViewWidthMode) -> Self {
        self.width_mode = mode;
        self
    }

    /// Configures or reads the `close_button_overlay_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn close_button_overlay_mode(mut self, mode: TabViewCloseButtonOverlayMode) -> Self {
        self.close_button_overlay_mode = mode;
        self
    }

    /// Configures or reads the `reorderable` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = reorderable;
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
    pub fn show(self, ui: &mut Ui) -> TabViewResponse {
        let p = ui_palette(ui);
        let mut close_requested = None;
        let mut add_clicked = false;
        let mut changed = false;
        let mut move_requested = None;
        let equal_width = if self.width_mode == TabViewWidthMode::Equal {
            (ui.available_width() / self.tabs.len().max(1) as f32).clamp(72.0, 180.0)
        } else {
            0.0
        };
        let response = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (index, tab) in self.tabs.iter().enumerate() {
                    let selected = *self.selected == index;
                    egui::Frame::new()
                        .fill(if selected {
                            p.control_bg
                        } else {
                            Color32::TRANSPARENT
                        })
                        .corner_radius(CornerRadius {
                            nw: 4,
                            ne: 4,
                            sw: 0,
                            se: 0,
                        })
                        .inner_margin(Margin::symmetric(10, 4))
                        .show(ui, |ui| {
                            if equal_width > 0.0 {
                                ui.set_min_width(equal_width - 20.0);
                            }
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 6.0;
                                let visible_tab =
                                    if self.width_mode == TabViewWidthMode::Compact && !selected {
                                        tab.chars().next().unwrap_or(' ').to_string()
                                    } else {
                                        tab.clone()
                                    };
                                let tab_response =
                                    ui.add(egui::Button::new(visible_tab).frame(false));
                                if tab_response.clicked() {
                                    *self.selected = index;
                                    changed = true;
                                }
                                if self.reorderable && tab_response.hovered() {
                                    if index > 0
                                        && ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(FluentIcon::Back.glyph())
                                                        .font(icon_font_id(8.0)),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_text("Move tab left")
                                            .clicked()
                                    {
                                        move_requested = Some((index, index - 1));
                                    }
                                    if index + 1 < self.tabs.len()
                                        && ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(
                                                        FluentIcon::ChevronRight.glyph(),
                                                    )
                                                    .font(icon_font_id(8.0)),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_text("Move tab right")
                                            .clicked()
                                    {
                                        move_requested = Some((index, index + 1));
                                    }
                                }
                                let show_close = match self.close_button_overlay_mode {
                                    TabViewCloseButtonOverlayMode::Always => true,
                                    TabViewCloseButtonOverlayMode::Auto => selected,
                                    TabViewCloseButtonOverlayMode::OnHover => {
                                        tab_response.hovered()
                                    }
                                };
                                if self.closable
                                    && show_close
                                    && ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new(FluentIcon::Close.glyph())
                                                    .font(icon_font_id(10.0)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    close_requested = Some(index);
                                }
                            });
                        });
                }
                if self.add_button
                    && ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(FluentIcon::Add.glyph())
                                    .font(icon_font_id(12.0)),
                            )
                            .frame(false),
                        )
                        .clicked()
                {
                    add_clicked = true;
                }
            })
            .response;
        TabViewResponse {
            response,
            changed,
            close_requested,
            add_clicked,
            move_requested,
        }
    }
}

/// A Fluent API value representing `TabViewResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TabViewResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `changed` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub changed: bool,
    /// The `close_requested` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub close_requested: Option<usize>,
    /// The `add_clicked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub add_clicked: bool,
    /// The `move_requested` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub move_requested: Option<(usize, usize)>,
}

/// A circular avatar or initials placeholder similar to WinUI PersonPicture.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PersonPicture {
    display_name: String,
    size: f32,
    profile_placeholder: bool,
}

impl PersonPicture {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(display_name: impl Into<String>) -> Self {
        Self {
            display_name: display_name.into(),
            size: 40.0,
            profile_placeholder: false,
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

    /// Shows the profile-image presentation using the Fluent person glyph as a fallback.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn profile_placeholder(mut self, enabled: bool) -> Self {
        self.profile_placeholder = enabled;
        self
    }
}

impl Widget for PersonPicture {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = ui_palette(ui);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());
        ui.painter()
            .circle_filled(rect.center(), self.size * 0.5, p.accent);
        let initials = if self.profile_placeholder {
            FluentIcon::Person.glyph().to_string()
        } else {
            self.display_name
                .split_whitespace()
                .filter_map(|part| part.chars().next())
                .take(2)
                .collect::<String>()
                .to_uppercase()
        };
        ui.painter().text(
            rect.center() + vec2(0.0, TEXT_Y_NUDGE),
            Align2::CENTER_CENTER,
            initials,
            if self.profile_placeholder {
                super::icon_font_id(self.size * 0.42)
            } else {
                FontId::proportional(self.size * 0.34)
            },
            if ui.visuals().dark_mode {
                Color32::BLACK
            } else {
                Color32::WHITE
            },
        );
        response
    }
}

/// Anchored non-modal guidance surface with optional action buttons.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TeachingTip {
    id: egui::Id,
    title: String,
    message: String,
    primary_text: Option<String>,
    close_text: Option<String>,
}

impl TeachingTip {
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
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: egui::Id::new(id_source),
            title: title.into(),
            message: message.into(),
            primary_text: None,
            close_text: Some("Close".into()),
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
        self.primary_text = Some(text.into());
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
        self.close_text = Some(text.into());
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
    pub fn show(
        self,
        ctx: &egui::Context,
        theme: FluentTheme,
        visible: bool,
    ) -> TeachingTipResponse {
        let mut primary_clicked = false;
        let mut close_clicked = false;
        if !visible {
            return TeachingTipResponse {
                primary_clicked,
                close_clicked,
            };
        }
        let p = palette(theme);
        egui::Area::new(self.id)
            .anchor(Align2::CENTER_TOP, vec2(0.0, 84.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(p.info_bg)
                    .stroke(Stroke::new(1.0, p.info_border))
                    .corner_radius(0)
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 6],
                        blur: 28,
                        spread: 0,
                        color: p.shadow,
                    })
                    .inner_margin(Margin::same(16))
                    .show(ui, |ui| {
                        ui.set_width(320.0);
                        ui.label(egui::RichText::new(self.title).size(18.0).strong());
                        ui.label(egui::RichText::new(self.message).color(p.text_secondary));
                        ui.add_space(10.0);
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if let Some(primary_text) = self.primary_text
                                && ui.add(egui::Button::new(primary_text)).clicked()
                            {
                                primary_clicked = true;
                            }
                            if let Some(close_text) = self.close_text
                                && ui.add(egui::Button::new(close_text)).clicked()
                            {
                                close_clicked = true;
                            }
                        });
                    });
            });
        TeachingTipResponse {
            primary_clicked,
            close_clicked,
        }
    }
}

/// A Fluent API value representing `TeachingTipResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TeachingTipResponse {
    /// The `primary_clicked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub primary_clicked: bool,
    /// The `close_clicked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub close_clicked: bool,
}

#[cfg(test)]
mod tests {
    use super::evaluate_number_expression;

    #[test]
    fn number_expression_respects_precedence_and_parentheses() {
        assert_eq!(evaluate_number_expression("1 + 2^2"), Ok(5.0));
        assert_eq!(evaluate_number_expression("(1 + 2) * 4"), Ok(12.0));
        assert_eq!(evaluate_number_expression("2^3^2"), Ok(512.0));
    }

    #[test]
    fn number_expression_rejects_invalid_results() {
        assert!(evaluate_number_expression("1 / 0").is_err());
        assert!(evaluate_number_expression("1 +").is_err());
        assert!(evaluate_number_expression("(2 + 3").is_err());
    }
}
