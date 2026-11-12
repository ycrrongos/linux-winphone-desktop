use std::time::Duration;

use eframe::egui::{
    self, Align, Align2, Color32, InnerResponse, Layout, Margin, Response, Sense, Stroke, Ui,
    UiBuilder, Vec2, Widget, vec2,
};

use super::{FluentIcon, FluentTheme, icon_font_id, palette};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Available `EasingFunction` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum EasingFunction {
    /// The `Linear` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Linear,
    /// The `CubicIn` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CubicIn,
    #[default]
    /// The `CubicOut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CubicOut,
    /// The `CubicInOut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CubicInOut,
    /// The `SineInOut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    SineInOut,
    /// The `BackOut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    BackOut,
}

impl EasingFunction {
    /// Configures or reads the `sample` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn sample(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::CubicIn => t.powi(3),
            Self::CubicOut => 1.0 - (1.0 - t).powi(3),
            Self::CubicInOut if t < 0.5 => 4.0 * t.powi(3),
            Self::CubicInOut => 1.0 - (-2.0 * t + 2.0).powi(3) * 0.5,
            Self::SineInOut => -(std::f32::consts::PI * t).cos() * 0.5 + 0.5,
            Self::BackOut => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
        }
    }
}

/// A state-driven Fluent icon animation.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AnimatedIcon {
    icon: FluentIcon,
    active: bool,
    size: f32,
}

impl AnimatedIcon {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(icon: FluentIcon, active: bool) -> Self {
        Self {
            icon,
            active,
            size: 24.0,
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

impl Widget for AnimatedIcon {
    fn ui(self, ui: &mut Ui) -> Response {
        let side = self.size + 12.0;
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::hover());
        let phase = if self.active {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
            ui.input(|input| input.time as f32 * 4.0)
        } else {
            0.0
        };
        let scale = 1.0 + phase.sin() * if self.active { 0.08 } else { 0.0 };
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(self.size * scale),
            ui.visuals().text_color(),
        );
        response
    }
}

/// Playback state for an application-provided animated visual.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AnimatedVisualPlayer<'a> {
    progress: &'a mut f32,
    playing: &'a mut bool,
    duration: f32,
    looping: bool,
    size: Vec2,
}

impl<'a> AnimatedVisualPlayer<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(progress: &'a mut f32, playing: &'a mut bool) -> Self {
        Self {
            progress,
            playing,
            duration: 1.0,
            looping: true,
            size: vec2(280.0, 150.0),
        }
    }

    /// Configures or reads the `duration` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn duration(mut self, seconds: f32) -> Self {
        self.duration = seconds;
        self
    }

    /// Configures or reads the `looping` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Configures or reads the `size` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn size(mut self, size: Vec2) -> Self {
        self.size = size;
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
        draw_frame: impl FnOnce(&egui::Painter, egui::Rect, f32) -> R,
    ) -> InnerResponse<R> {
        if *self.playing {
            let dt = ui.input(|input| input.stable_dt).min(0.1);
            *self.progress += dt / self.duration.max(0.05);
            if *self.progress >= 1.0 {
                if self.looping {
                    *self.progress = self.progress.fract();
                } else {
                    *self.progress = 1.0;
                    *self.playing = false;
                }
            }
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let inner = draw_frame(ui.painter(), rect, self.progress.clamp(0.0, 1.0));
        InnerResponse { inner, response }
    }
}

/// A keyed offset/fade transition corresponding to PageTransition.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PageTransition {
    id: egui::Id,
    revision: u64,
    distance: f32,
    easing: EasingFunction,
}

impl PageTransition {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, revision: u64) -> Self {
        Self {
            id: egui::Id::new(id_source),
            revision,
            distance: 12.0,
            easing: EasingFunction::CubicOut,
        }
    }

    /// Configures or reads the `distance` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    /// Configures or reads the `easing` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
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
        let state_id = self.id.with("revision");
        let changed = ui.ctx().data_mut(|data| {
            let old = data.get_temp::<u64>(state_id);
            data.insert_temp(state_id, self.revision);
            old != Some(self.revision)
        });
        let animation_id = self.id.with(self.revision);
        let raw = ui
            .ctx()
            .animate_bool_with_time(animation_id, !changed, 0.22);
        let t = self.easing.sample(raw);
        if t < 1.0 {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
        let offset = vec2(0.0, (1.0 - t) * self.distance);
        let max_rect = ui.available_rect_before_wrap().translate(offset);
        ui.scope_builder(UiBuilder::new().max_rect(max_rect), add_contents)
    }
}

/// A boolean reveal/collapse transition corresponding to ImplicitTransition.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ImplicitTransition {
    id: egui::Id,
    visible: bool,
    distance: f32,
}

impl ImplicitTransition {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, visible: bool) -> Self {
        Self {
            id: egui::Id::new(id_source),
            visible,
            distance: 8.0,
        }
    }

    /// Configures or reads the `distance` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
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
        add_contents: impl FnOnce(&mut Ui, f32) -> R,
    ) -> Option<InnerResponse<R>> {
        let t = ui.ctx().animate_bool_responsive(self.id, self.visible);
        if t <= 0.001 && !self.visible {
            return None;
        }
        let rect = ui
            .available_rect_before_wrap()
            .translate(vec2(0.0, (1.0 - t) * self.distance));
        Some(ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| add_contents(ui, t)))
    }
}

/// A source/destination keyed transition using the same timing model as PageTransition.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ConnectedAnimation(PageTransition);

impl ConnectedAnimation {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, revision: u64) -> Self {
        Self(PageTransition::new(id_source, revision).distance(18.0))
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
        self.0.show(ui, add_contents)
    }
}

/// A scroll-offset presenter corresponding to ParallaxView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ParallaxView {
    offset: f32,
    ratio: f32,
    height: f32,
}

impl ParallaxView {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(scroll_offset: f32) -> Self {
        Self {
            offset: scroll_offset,
            ratio: 0.25,
            height: 160.0,
        }
    }

    /// Configures or reads the `ratio` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio;
        self
    }

    /// Configures or reads the `height` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn height(mut self, height: f32) -> Self {
        self.height = height;
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
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), self.height), Sense::hover());
        let shifted = rect.translate(vec2(0.0, -self.offset * self.ratio));
        let inner = ui
            .scope_builder(
                UiBuilder::new()
                    .max_rect(shifted)
                    .layout(Layout::top_down(Align::Center)),
                |ui| {
                    ui.set_clip_rect(rect);
                    add_contents(ui)
                },
            )
            .inner;
        InnerResponse { inner, response }
    }
}

/// A Fluent elevation wrapper corresponding to ThemeShadow.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ThemeShadow {
    elevation: f32,
}

impl ThemeShadow {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(elevation: f32) -> Self {
        Self { elevation }
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
        let p = palette(if ui.visuals().dark_mode {
            FluentTheme::Dark
        } else {
            FluentTheme::Light
        });
        egui::Frame::new()
            .fill(p.control_bg)
            .stroke(Stroke::new(1.0, p.control_border))
            .corner_radius(0)
            .shadow(egui::epaint::Shadow {
                offset: [0, (self.elevation * 0.35).round() as i8],
                blur: (8.0 + self.elevation).round().clamp(0.0, 255.0) as u8,
                spread: 0,
                color: p.shadow,
            })
            .inner_margin(Margin::same(12))
            .show(ui, add_contents)
    }
}

/// A short theme-state transition value for custom painters.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ThemeTransition {
    id: egui::Id,
    dark: bool,
}

impl ThemeTransition {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, dark: bool) -> Self {
        Self {
            id: egui::Id::new(id_source),
            dark,
        }
    }

    /// Configures or reads the `value` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn value(self, ctx: &egui::Context) -> f32 {
        ctx.animate_bool_with_time(self.id, self.dark, 0.167)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Available `CompositionInteropStatus` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum CompositionInteropStatus {
    /// The `EguiFallback` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    EguiFallback,
    /// The `ExternalVisualAttached` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ExternalVisualAttached,
}

/// A visible host slot for an optional external Composition visual.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct XamlCompInterop {
    status: CompositionInteropStatus,
    size: Vec2,
}

impl XamlCompInterop {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(status: CompositionInteropStatus) -> Self {
        Self {
            status,
            size: vec2(280.0, 120.0),
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
    pub const fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl Widget for XamlCompInterop {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = palette(if ui.visuals().dark_mode {
            FluentTheme::Dark
        } else {
            FluentTheme::Light
        });
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        ui.painter().rect(
            rect,
            4,
            p.control_bg,
            Stroke::new(1.0, p.control_border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            rect.center() - vec2(0.0, 12.0),
            Align2::CENTER_CENTER,
            FluentIcon::Window.glyph(),
            icon_font_id(24.0),
            p.text_primary,
        );
        ui.painter().text(
            rect.center() + vec2(0.0, 18.0),
            Align2::CENTER_CENTER,
            match self.status {
                CompositionInteropStatus::EguiFallback => "egui visual fallback",
                CompositionInteropStatus::ExternalVisualAttached => "External visual attached",
            },
            egui::TextStyle::Body.resolve(ui.style()),
            p.text_secondary,
        );
        response
    }
}

/// Helper for interpolating theme-aware colors in custom transition content.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn transition_color(light: Color32, dark: Color32, t: f32) -> Color32 {
    let a = egui::Rgba::from(light);
    let b = egui::Rgba::from(dark);
    Color32::from(a.multiply(1.0 - t.clamp(0.0, 1.0)) + b.multiply(t.clamp(0.0, 1.0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_endpoints_are_stable() {
        for easing in [
            EasingFunction::Linear,
            EasingFunction::CubicOut,
            EasingFunction::CubicInOut,
            EasingFunction::SineInOut,
            EasingFunction::BackOut,
        ] {
            assert!((easing.sample(0.0) - 0.0).abs() < 0.0001);
            assert!((easing.sample(1.0) - 1.0).abs() < 0.0001);
        }
    }
}
