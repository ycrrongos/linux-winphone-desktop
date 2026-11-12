use eframe::egui::{Response, RichText, Ui, vec2};

use super::{AccentButton, FluentButton, PasswordBox, fluent_text_edit};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Available `CounterMode` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum CounterMode {
    /// The `Increment` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Increment,
    /// The `Decrement` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Decrement,
}

/// A Fluent API value representing `CounterControl`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CounterControl<'a> {
    value: &'a mut i32,
    mode: CounterMode,
}

impl<'a> CounterControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(value: &'a mut i32, mode: CounterMode) -> Self {
        Self { value, mode }
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
        ui.allocate_ui(vec2(120.0, 72.0), |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(self.value.to_string()).size(20.0));
                let label = match self.mode {
                    CounterMode::Increment => "Increase",
                    CounterMode::Decrement => "Decrease",
                };
                let response = ui.add_sized([100.0, 32.0], FluentButton::new(label));
                if response.clicked() {
                    match self.mode {
                        CounterMode::Increment => *self.value = self.value.saturating_add(1),
                        CounterMode::Decrement => *self.value = self.value.saturating_sub(1),
                    }
                }
                response
            })
            .inner
        })
        .inner
    }
}

/// A Fluent API value representing `ValidatedPasswordBox`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ValidatedPasswordBox<'a> {
    text: &'a mut String,
    reveal: bool,
    min_length: usize,
    width: f32,
    hint_text: String,
}

impl<'a> ValidatedPasswordBox<'a> {
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
            min_length: 8,
            width: 240.0,
            hint_text: "Enter password...".into(),
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

    /// Configures or reads the `min_length` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
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

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> ValidatedPasswordResponse {
        let is_valid = self.text.len() >= self.min_length;
        let response = ui
            .vertical(|ui| {
                let response = ui.add_sized(
                    [self.width, 32.0],
                    PasswordBox::new(self.text)
                        .hint_text(self.hint_text)
                        .reveal(self.reveal),
                );
                let message = if is_valid {
                    "Password is valid".to_owned()
                } else {
                    format!("Use at least {} characters", self.min_length)
                };
                ui.label(RichText::new(message).color(if is_valid {
                    ui.visuals().hyperlink_color
                } else {
                    ui.visuals().error_fg_color
                }));
                response
            })
            .inner;
        ValidatedPasswordResponse { response, is_valid }
    }
}

/// A Fluent API value representing `ValidatedPasswordResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ValidatedPasswordResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `is_valid` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub is_valid: bool,
}

/// A Fluent API value representing `TemperatureConverterControl`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TemperatureConverterControl<'a> {
    input: &'a mut String,
    result: &'a mut Option<f64>,
    width: f32,
}

impl<'a> TemperatureConverterControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(input: &'a mut String, result: &'a mut Option<f64>) -> Self {
        Self {
            input,
            result,
            width: 200.0,
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

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> TemperatureConverterResponse {
        ui.label("Enter Temperature in Celsius");
        let input_response = ui.add_sized(
            [self.width, 32.0],
            fluent_text_edit(self.input).hint_text("Celsius"),
        );
        let parsed = self.input.trim().parse::<f64>().ok();
        let convert_response = ui
            .add_enabled_ui(parsed.is_some(), |ui| {
                ui.add_sized(
                    [self.width, 32.0],
                    FluentButton::new("Convert to Fahrenheit"),
                )
            })
            .inner;
        if convert_response.clicked() {
            *self.result = parsed.map(|celsius| celsius.mul_add(1.8, 32.0));
        }
        if let Some(result) = *self.result {
            ui.label(RichText::new(format!("{result:.1} °F")).strong());
        }
        TemperatureConverterResponse {
            input_response,
            convert_response,
            result: *self.result,
        }
    }
}

/// A Fluent API value representing `TemperatureConverterResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TemperatureConverterResponse {
    /// The `input_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub input_response: Response,
    /// The `convert_response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub convert_response: Response,
    /// The `result` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub result: Option<f64>,
}

/// Runs the `validated_submit_button` Fluent helper.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn validated_submit_button(ui: &mut Ui, enabled: bool, text: impl Into<String>) -> Response {
    ui.add_enabled_ui(enabled, |ui| {
        ui.add_sized([240.0, 32.0], AccentButton::new(text))
    })
    .inner
}
