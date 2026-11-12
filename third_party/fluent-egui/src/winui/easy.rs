//! High-level helpers for composing WinUI-style interfaces with minimal boilerplate.

use std::{fmt::Debug, hash::Hash, ops::RangeInclusive};

use eframe::egui::{self, InnerResponse, Response, Ui, Widget};

use super::{
    FluentButton, FluentIcon, FluentProgressBar, FluentSlider, FluentTextEdit, FluentTheme,
    GalleryExample, GalleryPageHeader, GalleryPageHeaderResponse, IconButton, PasswordBox,
    SegmentedControl, ToggleSwitch, combo_option, fluent_combo_box, section_card, setting_row,
};

/// A WinUI-oriented view over an [`egui::Ui`].
///
/// The short methods cover common application code. All regular egui methods remain available
/// through `Deref`, and advanced callers can still pass the lower-level widget builders to
/// [`Ui::add`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentUi<'a> {
    ui: &'a mut Ui,
    theme: FluentTheme,
}

impl<'a> FluentUi<'a> {
    /// Wraps an egui UI and explicitly selects the theme used by composite controls.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(ui: &'a mut Ui, theme: FluentTheme) -> Self {
        Self { ui, theme }
    }

    /// Wraps an egui UI and infers the theme from its installed visuals.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn from_ui(ui: &'a mut Ui) -> Self {
        let theme = if ui.visuals().dark_mode {
            FluentTheme::Dark
        } else {
            FluentTheme::Light
        };
        Self::new(ui, theme)
    }

    /// Configures or reads the `theme` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn theme(&self) -> FluentTheme {
        self.theme
    }

    /// Adds a standard WinUI button.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn button(&mut self, text: impl Into<String>) -> Response {
        self.ui.add(FluentButton::new(text))
    }

    /// Adds the primary/accent button for the current action group.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn accent_button(&mut self, text: impl Into<String>) -> Response {
        self.ui.add(FluentButton::accent(text))
    }

    /// Adds a low-emphasis button with no resting backplate.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn subtle_button(&mut self, text: impl Into<String>) -> Response {
        self.ui.add(FluentButton::subtle(text))
    }

    /// Adds an icon-only command button.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn icon_button(&mut self, icon: FluentIcon) -> Response {
        self.ui.add(IconButton::new(icon))
    }

    /// Adds a button while exposing the complete builder for optional customization.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn button_with(
        &mut self,
        text: impl Into<String>,
        configure: impl FnOnce(FluentButton) -> FluentButton,
    ) -> Response {
        self.ui.add(configure(FluentButton::new(text)))
    }

    /// Adds a WinUI-aligned single-line text box.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn text_box(&mut self, text: &mut dyn egui::TextBuffer) -> Response {
        self.ui.add(FluentTextEdit::new(text))
    }

    /// Adds a text box with a placeholder while retaining sensible default sizing.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn text_box_hint(
        &mut self,
        text: &mut dyn egui::TextBuffer,
        hint: impl Into<String>,
    ) -> Response {
        self.ui.add(FluentTextEdit::new(text).hint_text(hint))
    }

    /// Adds a text box while exposing its builder for password and length customization.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn text_box_with<'b>(
        &mut self,
        text: &'b mut dyn egui::TextBuffer,
        configure: impl FnOnce(FluentTextEdit<'b>) -> FluentTextEdit<'b>,
    ) -> Response {
        self.ui.add(configure(FluentTextEdit::new(text)))
    }

    /// Adds a masked password box.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn password_box(&mut self, text: &mut String) -> Response {
        self.ui.add(PasswordBox::new(text))
    }

    /// Adds a labeled toggle switch that edits `value` directly.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn toggle(&mut self, value: &mut bool, label: impl Into<String>) -> Response {
        self.ui.add(ToggleSwitch::new(value).label(label))
    }

    /// Adds a slider using the library's standard WinUI width and animation.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn slider(&mut self, value: &mut f32, range: RangeInclusive<f32>) -> Response {
        self.ui.add(FluentSlider::new(value, range))
    }

    /// Adds a slider while exposing width, step, value-label and orientation customization.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn slider_with<'b>(
        &mut self,
        value: &'b mut f32,
        range: RangeInclusive<f32>,
        configure: impl FnOnce(FluentSlider<'b>) -> FluentSlider<'b>,
    ) -> Response {
        self.ui.add(configure(FluentSlider::new(value, range)))
    }

    /// Adds a progress bar. Progress is clamped to the `0.0..=1.0` range by the control.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn progress(&mut self, progress: f32) -> Response {
        self.ui.add(FluentProgressBar::new(progress))
    }

    /// Adds a segmented selector and edits `selected` directly.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn segmented(
        &mut self,
        selected: &mut usize,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Response {
        self.ui.add(SegmentedControl::new(selected, labels))
    }

    /// Adds a WinUI ComboBox. The returned response is marked changed after a new selection.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn select<T: AsRef<str>>(
        &mut self,
        id_source: impl Hash + Debug,
        selected: &mut usize,
        options: &[T],
    ) -> Response {
        if options.is_empty() {
            return self.ui.add_enabled(false, FluentButton::new("No options"));
        }
        *selected = (*selected).min(options.len() - 1);
        let selected_text = options[*selected].as_ref().to_owned();
        let mut changed = false;
        let mut response = fluent_combo_box(id_source, selected_text)
            .show_ui(self.ui, |ui| {
                for (index, option) in options.iter().enumerate() {
                    if combo_option(ui, option.as_ref(), *selected == index).clicked() {
                        *selected = index;
                        changed = true;
                        ui.close();
                    }
                }
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }

    /// Lays out child controls horizontally using egui's automatic layout.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn row<R>(
        &mut self,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = self.theme;
        self.ui.horizontal(|ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_contents(&mut fluent)
        })
    }

    /// Lays out controls left-to-right and automatically wraps on narrow windows.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn wrap<R>(
        &mut self,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = self.theme;
        self.ui.horizontal_wrapped(|ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_contents(&mut fluent)
        })
    }

    /// Lays out child controls vertically using egui's automatic layout.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn column<R>(
        &mut self,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = self.theme;
        self.ui.vertical(|ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_contents(&mut fluent)
        })
    }

    /// Adds a responsive titled card. The caller never specifies its coordinates or width.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn card<R>(
        &mut self,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = self.theme;
        section_card(self.ui, theme, title, subtitle, |ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_contents(&mut fluent)
        })
    }

    /// Adds a Gallery-style example panel with automatic width and spacing.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn example<R>(
        &mut self,
        title: impl Into<String>,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let theme = self.theme;
        GalleryExample::new(title).show(self.ui, |ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_contents(&mut fluent)
        })
    }

    /// Adds a responsive Settings row with a right-side control area.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn setting(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
        add_control: impl FnOnce(&mut FluentUi<'_>),
    ) -> Response {
        let theme = self.theme;
        setting_row(self.ui, theme, title, description, |ui| {
            let mut fluent = FluentUi::new(ui, theme);
            add_control(&mut fluent);
        })
    }

    /// Adds the standard Gallery page heading and command row.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn page_header(
        &mut self,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> GalleryPageHeaderResponse {
        GalleryPageHeader::new(title, description).show(self.ui)
    }

    /// Adds any lower-level egui or WinUI widget without leaving the fluent scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn control<W: Widget>(&mut self, widget: W) -> Response {
        self.ui.add(widget)
    }
}

impl std::ops::Deref for FluentUi<'_> {
    type Target = Ui;

    fn deref(&self) -> &Self::Target {
        self.ui
    }
}

impl std::ops::DerefMut for FluentUi<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ui
    }
}

/// Adds the concise [`FluentUi`] composition scope to every egui [`Ui`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub trait WinUiExt {
    /// Uses the theme already installed in egui's visuals.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    fn winui<R>(&mut self, add_contents: impl FnOnce(&mut FluentUi<'_>) -> R) -> R;

    /// Uses an explicit theme for composite controls inside this scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    fn winui_with_theme<R>(
        &mut self,
        theme: FluentTheme,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> R;
}

impl WinUiExt for Ui {
    fn winui<R>(&mut self, add_contents: impl FnOnce(&mut FluentUi<'_>) -> R) -> R {
        let mut fluent = FluentUi::from_ui(self);
        add_contents(&mut fluent)
    }

    fn winui_with_theme<R>(
        &mut self,
        theme: FluentTheme,
        add_contents: impl FnOnce(&mut FluentUi<'_>) -> R,
    ) -> R {
        let mut fluent = FluentUi::new(self, theme);
        add_contents(&mut fluent)
    }
}
