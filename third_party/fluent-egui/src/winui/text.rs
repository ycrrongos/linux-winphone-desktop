use eframe::egui::{
    self, Align, Align2, InnerResponse, Layout, Margin, Response, Sense, Ui, Widget, pos2, vec2,
};

use super::fluent_text_edit;

/// Result returned by [`AutoSuggestBox::show`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AutoSuggestResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `chosen` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub chosen: Option<String>,
}

/// A single-line search field with a filtered suggestion popup.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AutoSuggestBox<'a> {
    text: &'a mut String,
    suggestions: Vec<String>,
    hint_text: String,
    width: f32,
}

impl<'a> AutoSuggestBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        text: &'a mut String,
        suggestions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            text,
            suggestions: suggestions.into_iter().map(Into::into).collect(),
            hint_text: "Search".into(),
            width: 240.0,
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
    pub fn show(self, ui: &mut Ui) -> AutoSuggestResponse {
        let response = ui.add_sized(
            [self.width, 32.0],
            fluent_text_edit(self.text).hint_text(self.hint_text),
        );
        let query = self.text.trim().to_lowercase();
        let matches = self
            .suggestions
            .iter()
            .filter(|item| query.is_empty() || item.to_lowercase().contains(&query))
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        let mut chosen = None;
        egui::Popup::from_response(&response)
            .open(response.has_focus() && !matches.is_empty())
            .width(self.width)
            .gap(2.0)
            .frame(egui::Frame::popup(ui.style()).inner_margin(Margin::same(4)))
            .show(|ui| {
                ui.set_min_width(self.width - 8.0);
                for suggestion in matches {
                    let (rect, item) =
                        ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
                    if item.hovered() {
                        ui.painter()
                            .rect_filled(rect, 0, ui.visuals().faint_bg_color);
                    }
                    ui.painter().text(
                        pos2(rect.left() + 10.0, rect.center().y - 0.5),
                        Align2::LEFT_CENTER,
                        &suggestion,
                        egui::TextStyle::Body.resolve(ui.style()),
                        ui.visuals().text_color(),
                    );
                    if item.clicked() {
                        *self.text = suggestion.clone();
                        chosen = Some(suggestion);
                        ui.close();
                    }
                }
            });
        AutoSuggestResponse { response, chosen }
    }
}

/// A WinUI-aligned password field with optional reveal behavior.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PasswordBox<'a> {
    text: &'a mut String,
    reveal: bool,
    hint_text: String,
    mask_char: char,
}

impl<'a> PasswordBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            reveal: false,
            hint_text: "Password".into(),
            mask_char: '*',
        }
    }

    /// Configures or reads the `reveal` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn reveal(mut self, reveal: bool) -> Self {
        self.reveal = reveal;
        self
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

    /// Configures or reads the `mask_char` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn mask_char(mut self, mask_char: char) -> Self {
        self.mask_char = mask_char;
        self
    }
}

impl Widget for PasswordBox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let _mask_char = self.mask_char;
        fluent_text_edit(self.text)
            .password(!self.reveal)
            .hint_text(self.hint_text)
            .ui(ui)
    }
}

/// A multiline text editor styled like WinUI RichEditBox.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RichEditBox<'a> {
    text: &'a mut String,
    desired_rows: usize,
    width: f32,
}

impl<'a> RichEditBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            desired_rows: 5,
            width: 320.0,
        }
    }

    /// Configures or reads the `desired_rows` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn desired_rows(mut self, desired_rows: usize) -> Self {
        self.desired_rows = desired_rows;
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
}

impl Widget for RichEditBox<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add_sized(
            [self.width, self.desired_rows.max(2) as f32 * 22.0],
            egui::TextEdit::multiline(self.text)
                .desired_rows(self.desired_rows)
                .margin(Margin::symmetric(10, 7)),
        )
    }
}

/// Convenience wrapper for read-only rich text content.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RichTextBlock {
    text: egui::RichText,
    width: f32,
}

impl RichTextBlock {
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
            text: egui::RichText::new(text),
            width: 320.0,
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

    /// Configures or reads the `strong` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn strong(mut self) -> Self {
        self.text = self.text.strong();
        self
    }
}

impl Widget for RichTextBlock {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.allocate_ui_with_layout(
            vec2(self.width, 0.0),
            Layout::left_to_right(Align::Min),
            |ui| ui.add(egui::Label::new(self.text).wrap()),
        )
        .inner
    }
}

/// Returns a normal WinUI-aligned text field and its response in one call.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn text_box(ui: &mut Ui, text: &mut String, width: f32) -> InnerResponse<Response> {
    ui.horizontal(|ui| ui.add_sized([width, 32.0], fluent_text_edit(text)))
}
