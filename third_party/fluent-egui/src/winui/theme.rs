use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

#[cfg(windows)]
use eframe::egui::FontTweak;
use eframe::egui::style::WidgetVisuals;
use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin,
    Shadow, Stroke, Style, TextStyle, Vec2, Visuals,
};

use super::icons::ICON_FONT_FAMILY;

/// Light and dark color modes matching WinUI's two application themes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum FluentTheme {
    #[default]
    /// The `Light` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Light,
    /// The `Dark` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Dark,
}

impl FluentTheme {
    /// Reports the current `is_dark` state.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

/// Theme inputs. `accent` is the system-blue default when omitted.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThemeConfig {
    /// The `theme` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub theme: FluentTheme,
    /// The `accent` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub accent: Option<Color32>,
    /// Opacity applied to the main material layer. Keep below 1.0 to reveal Mica.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub material_opacity: f32,
    /// The `animations` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub animations: bool,
}

impl ThemeConfig {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(theme: FluentTheme) -> Self {
        Self {
            theme,
            accent: None,
            material_opacity: 0.70,
            animations: true,
        }
    }

    /// Configures or reads the `with_accent` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn with_accent(mut self, accent: Color32) -> Self {
        self.accent = Some(accent);
        self
    }

    /// Configures or reads the `with_animations` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn with_animations(mut self, animations: bool) -> Self {
        self.animations = animations;
        self
    }

    /// Configures or reads the `with_material_opacity` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn with_material_opacity(mut self, material_opacity: f32) -> Self {
        self.material_opacity = material_opacity;
        self
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self::new(FluentTheme::Light)
    }
}

/// Resolved colors used by both the global egui style and custom Fluent widgets.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct FluentPalette {
    /// The `text_primary` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub text_primary: Color32,
    /// The `text_secondary` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub text_secondary: Color32,
    /// The `text_disabled` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub text_disabled: Color32,
    /// The `accent` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub accent: Color32,
    /// The `accent_hover` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub accent_hover: Color32,
    /// The `accent_pressed` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub accent_pressed: Color32,
    /// The `on_accent` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub on_accent: Color32,
    /// The `app_bg` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub app_bg: Color32,
    /// The `nav_bg` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub nav_bg: Color32,
    /// The `card_bg` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub card_bg: Color32,
    /// The `card_border` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub card_border: Color32,
    /// The `control_bg` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub control_bg: Color32,
    /// The `control_hover` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub control_hover: Color32,
    /// The `control_pressed` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub control_pressed: Color32,
    /// The `control_border` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub control_border: Color32,
    /// The `slider_thumb` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub slider_thumb: Color32,
    /// The `slider_thumb_border` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub slider_thumb_border: Color32,
    /// The `progress_track` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub progress_track: Color32,
    /// The `subtle_hover` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub subtle_hover: Color32,
    /// The `selection` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub selection: Color32,
    /// The `info_bg` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub info_bg: Color32,
    /// The `info_border` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub info_border: Color32,
    /// The `success` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub success: Color32,
    /// The `warning` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub warning: Color32,
    /// The `danger` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub danger: Color32,
    /// The `shadow` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub shadow: Color32,
}

/// Returns the default WinUI palette for a light or dark theme.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn palette(theme: FluentTheme) -> FluentPalette {
    resolved_palette(ThemeConfig::new(theme))
}

/// Resolves a palette from the complete theme configuration.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn palette_with_config(config: ThemeConfig) -> FluentPalette {
    resolved_palette(config)
}

/// Default accent for each WinUI application theme.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub const fn default_accent(theme: FluentTheme) -> Color32 {
    if theme.is_dark() {
        Color32::from_rgb(96, 205, 255)
    } else {
        Color32::from_rgb(0, 95, 184)
    }
}

/// Six Fluent accent swatches tuned for the selected application theme.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub const fn accent_presets(theme: FluentTheme) -> [Color32; 6] {
    if theme.is_dark() {
        [
            Color32::from_rgb(96, 205, 255),
            Color32::from_rgb(76, 194, 255),
            Color32::from_rgb(108, 203, 95),
            Color32::from_rgb(194, 57, 179),
            Color32::from_rgb(255, 153, 164),
            Color32::from_rgb(252, 225, 0),
        ]
    } else {
        [
            Color32::from_rgb(0, 95, 184),
            Color32::from_rgb(0, 120, 212),
            Color32::from_rgb(16, 124, 16),
            Color32::from_rgb(136, 23, 152),
            Color32::from_rgb(194, 57, 52),
            Color32::from_rgb(202, 80, 16),
        ]
    }
}

fn resolved_palette(config: ThemeConfig) -> FluentPalette {
    let opacity = (config.material_opacity.clamp(0.0, 1.0) * 255.0) as u8;
    if config.theme.is_dark() {
        let accent = config
            .accent
            .unwrap_or_else(|| default_accent(config.theme));
        FluentPalette {
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_rgb(200, 200, 200),
            text_disabled: Color32::from_rgb(126, 126, 126),
            accent,
            accent_hover: mix(accent, Color32::WHITE, 0.10),
            accent_pressed: mix(accent, Color32::BLACK, 0.14),
            on_accent: Color32::from_rgb(0, 0, 0),
            app_bg: Color32::from_rgba_unmultiplied(32, 32, 32, opacity),
            nav_bg: Color32::from_rgba_unmultiplied(24, 24, 24, opacity.saturating_sub(72)),
            card_bg: Color32::from_rgba_unmultiplied(43, 43, 43, 248),
            card_border: Color32::from_rgba_unmultiplied(255, 255, 255, 18),
            control_bg: Color32::from_rgba_unmultiplied(58, 58, 58, 238),
            control_hover: Color32::from_rgba_unmultiplied(66, 66, 66, 242),
            control_pressed: Color32::from_rgba_unmultiplied(50, 50, 50, 242),
            control_border: Color32::from_rgba_unmultiplied(255, 255, 255, 25),
            slider_thumb: Color32::from_rgb(69, 69, 69),
            slider_thumb_border: Color32::from_rgba_unmultiplied(255, 255, 255, 45),
            progress_track: Color32::from_rgba_unmultiplied(255, 255, 255, 24),
            subtle_hover: Color32::from_rgba_unmultiplied(255, 255, 255, 15),
            selection: Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 48),
            info_bg: Color32::from_rgb(45, 45, 45),
            info_border: Color32::from_rgba_unmultiplied(255, 255, 255, 26),
            success: Color32::from_rgb(108, 203, 95),
            warning: Color32::from_rgb(252, 225, 0),
            danger: Color32::from_rgb(255, 153, 164),
            shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 76),
        }
    } else {
        let accent = config
            .accent
            .unwrap_or_else(|| default_accent(config.theme));
        FluentPalette {
            text_primary: Color32::from_rgb(27, 27, 27),
            text_secondary: Color32::from_rgb(92, 92, 92),
            text_disabled: Color32::from_rgb(160, 160, 160),
            accent,
            accent_hover: mix(accent, Color32::WHITE, 0.08),
            accent_pressed: mix(accent, Color32::BLACK, 0.10),
            on_accent: Color32::WHITE,
            app_bg: Color32::from_rgba_unmultiplied(243, 243, 243, opacity),
            nav_bg: Color32::from_rgba_unmultiplied(249, 249, 249, opacity.saturating_sub(72)),
            card_bg: Color32::from_rgba_unmultiplied(255, 255, 255, 248),
            card_border: Color32::from_rgba_unmultiplied(0, 0, 0, 16),
            control_bg: Color32::from_rgba_unmultiplied(255, 255, 255, 242),
            control_hover: Color32::from_rgba_unmultiplied(249, 249, 249, 228),
            control_pressed: Color32::from_rgba_unmultiplied(245, 245, 245, 218),
            control_border: Color32::from_rgba_unmultiplied(0, 0, 0, 38),
            slider_thumb: Color32::WHITE,
            slider_thumb_border: Color32::from_rgba_unmultiplied(0, 0, 0, 32),
            progress_track: Color32::from_rgba_unmultiplied(0, 0, 0, 28),
            subtle_hover: Color32::from_rgba_unmultiplied(0, 0, 0, 9),
            selection: Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 30),
            info_bg: Color32::WHITE,
            info_border: Color32::from_rgba_unmultiplied(0, 0, 0, 24),
            success: Color32::from_rgb(15, 123, 15),
            warning: Color32::from_rgb(157, 93, 0),
            danger: Color32::from_rgb(196, 43, 28),
            shadow: Color32::from_rgba_unmultiplied(0, 0, 0, 38),
        }
    }
}

/// Installs system fonts and the complete Fluent style into an egui context.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn install(ctx: &egui::Context, config: ThemeConfig) {
    install_system_fonts(ctx);
    let opposite = ThemeConfig {
        theme: if config.theme.is_dark() {
            FluentTheme::Light
        } else {
            FluentTheme::Dark
        },
        accent: None,
        ..config
    };
    let _ = cached_style(opposite);
    apply_theme(ctx, config);
}

/// Replaces egui's global style with the selected Fluent theme.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn apply_theme(ctx: &egui::Context, config: ThemeConfig) {
    let theme = if config.theme.is_dark() {
        egui::Theme::Dark
    } else {
        egui::Theme::Light
    };
    ctx.set_style_of(theme, cached_style(config));
    ctx.set_theme(theme);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct StyleCacheKey {
    theme: FluentTheme,
    accent: [u8; 4],
    material_opacity: u32,
    animations: bool,
}

fn cached_style(config: ThemeConfig) -> Arc<Style> {
    static CACHE: OnceLock<Mutex<HashMap<StyleCacheKey, Arc<Style>>>> = OnceLock::new();
    let key = StyleCacheKey {
        theme: config.theme,
        accent: config
            .accent
            .unwrap_or_else(|| default_accent(config.theme))
            .to_srgba_unmultiplied(),
        material_opacity: config.material_opacity.to_bits(),
        animations: config.animations,
    };
    let mut cache = CACHE
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    Arc::clone(cache.entry(key).or_insert_with(|| Arc::new(style(config))))
}

fn style(config: ThemeConfig) -> Style {
    let p = resolved_palette(config);
    let mut style = Style {
        text_styles: BTreeMap::from([
            (TextStyle::Small, FontId::proportional(12.0)),
            (TextStyle::Body, FontId::proportional(14.0)),
            (TextStyle::Button, FontId::proportional(14.0)),
            (TextStyle::Heading, FontId::proportional(20.0)),
            (TextStyle::Monospace, FontId::monospace(13.0)),
            (
                TextStyle::Name("Caption".into()),
                FontId::proportional(12.0),
            ),
            (
                TextStyle::Name("Subtitle".into()),
                FontId::proportional(16.0),
            ),
            (TextStyle::Name("Title".into()), FontId::proportional(28.0)),
        ]),
        override_text_valign: Some(egui::Align::Center),
        animation_time: if config.animations { 0.167 } else { 0.0 },
        ..Default::default()
    };
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.window_margin = Margin::same(16);
    style.spacing.menu_margin = Margin::same(4);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    style.spacing.interact_size = Vec2::new(40.0, 32.0);
    style.spacing.icon_width = 18.0;
    style.spacing.icon_width_inner = 11.0;
    style.spacing.icon_spacing = 8.0;
    style.spacing.slider_width = 180.0;
    style.spacing.slider_rail_height = 4.0;
    style.spacing.scroll = egui::style::ScrollStyle::floating();
    style.spacing.scroll.bar_width = 5.0;
    style.spacing.scroll.floating_width = 2.0;
    style.spacing.scroll.floating_allocated_width = 3.0;
    style.spacing.scroll.handle_min_length = 24.0;
    style.spacing.scroll.dormant_handle_opacity = 0.35;
    style.spacing.scroll.active_handle_opacity = 0.60;
    style.spacing.scroll.interact_handle_opacity = 0.85;
    style.interaction.tooltip_delay = 0.45;
    style.interaction.tooltip_grace_time = 0.15;
    style.interaction.show_tooltips_only_when_still = true;
    style.scroll_animation = egui::style::ScrollAnimation::duration(0.22);

    let mut visuals = if config.theme.is_dark() {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    visuals.override_text_color = Some(p.text_primary);
    visuals.weak_text_color = Some(p.text_secondary);
    visuals.faint_bg_color = p.subtle_hover;
    visuals.extreme_bg_color = p.control_bg;
    visuals.text_edit_bg_color = Some(p.control_bg);
    visuals.code_bg_color = p.control_pressed;
    visuals.hyperlink_color = p.accent;
    visuals.warn_fg_color = p.warning;
    visuals.error_fg_color = p.danger;
    visuals.window_corner_radius = CornerRadius::ZERO;
    visuals.menu_corner_radius = CornerRadius::ZERO;
    visuals.window_fill = p.info_bg;
    visuals.window_stroke = Stroke::new(1.0, p.info_border);
    visuals.panel_fill = Color32::TRANSPARENT;
    visuals.window_shadow = Shadow {
        offset: [0, 4],
        blur: 24,
        spread: 0,
        color: p.shadow,
    };
    visuals.popup_shadow = Shadow {
        offset: [0, 8],
        blur: 32,
        spread: 0,
        color: p.shadow,
    };
    visuals.selection.bg_fill = p.selection;
    visuals.selection.stroke = Stroke::new(1.0, p.accent);
    visuals.button_frame = true;
    visuals.collapsing_header_frame = false;
    visuals.indent_has_left_vline = false;
    visuals.slider_trailing_fill = true;
    visuals.handle_shape = egui::style::HandleShape::Circle;
    visuals.interact_cursor = None;
    visuals.disabled_alpha = 0.42;

    visuals.widgets.noninteractive = widget_visuals(
        Color32::TRANSPARENT,
        Color32::TRANSPARENT,
        Color32::TRANSPARENT,
        p.text_primary,
    );
    visuals.widgets.inactive =
        widget_visuals(p.control_bg, p.control_bg, p.control_border, p.text_primary);
    visuals.widgets.hovered = widget_visuals(
        p.control_hover,
        p.control_hover,
        p.control_border,
        p.text_primary,
    );
    visuals.widgets.active = widget_visuals(
        p.control_pressed,
        p.control_pressed,
        p.control_border,
        p.text_primary,
    );
    visuals.widgets.open = visuals.widgets.active;
    visuals.text_cursor.stroke = Stroke::new(1.5, p.accent);
    style.visuals = visuals;
    style
}

fn widget_visuals(
    bg_fill: Color32,
    weak_bg_fill: Color32,
    border: Color32,
    foreground: Color32,
) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill,
        weak_bg_fill,
        bg_stroke: Stroke::new(1.0, border),
        corner_radius: CornerRadius::ZERO,
        fg_stroke: Stroke::new(1.0, foreground),
        expansion: 0.0,
    }
}

fn install_system_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    #[cfg(windows)]
    let system_icon_font = {
        let windows_fonts = std::env::var_os("WINDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Windows"))
            .join("Fonts");

        if let Some(bytes) = read_first(&[
            windows_fonts.join("segoeui.ttf"),
            windows_fonts.join("SegUIVar.ttf"),
        ]) {
            fonts.font_data.insert(
                "Segoe UI".into(),
                Arc::new(FontData::from_owned(bytes).tweak(FontTweak {
                    scale: 1.0,
                    y_offset: -0.75,
                    ..Default::default()
                })),
            );
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "Segoe UI".into());
        }

        if let Some(bytes) = read_first(&[windows_fonts.join("msyh.ttc")]) {
            fonts.font_data.insert(
                "Microsoft YaHei".into(),
                Arc::new(FontData::from_owned(bytes).tweak(FontTweak {
                    y_offset: -0.75,
                    ..Default::default()
                })),
            );
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .push("Microsoft YaHei".into());
        }

        read_first(&[
            windows_fonts.join("SegoeIcons.ttf"),
            windows_fonts.join("segmdl2.ttf"),
        ])
    };

    #[cfg(not(windows))]
    let system_icon_font: Option<Vec<u8>> = None;

    // CJK fallback for Linux/macOS — without this, Chinese UI strings render as □□□□.
    #[cfg(not(windows))]
    install_cjk_fallback(&mut fonts);

    let icon_bytes = system_icon_font.unwrap_or_else(|| {
        include_bytes!("../assets/FluentSystemIcons-CrossPlatform.ttf").to_vec()
    });
    fonts.font_data.insert(
        ICON_FONT_FAMILY.into(),
        Arc::new(FontData::from_owned(icon_bytes)),
    );
    fonts.families.insert(
        FontFamily::Name(ICON_FONT_FAMILY.into()),
        vec![ICON_FONT_FAMILY.into()],
    );

    ctx.set_fonts(fonts);
}

fn read_first(paths: &[PathBuf]) -> Option<Vec<u8>> {
    paths.iter().find_map(|path| fs::read(path).ok())
}

/// Append a system CJK face to Proportional/Monospace so Han glyphs resolve.
#[cfg(not(windows))]
fn install_cjk_fallback(fonts: &mut FontDefinitions) {
    // Noto Sans CJK Regular.ttc face order: JP=0, KR=1, SC=2, TC=3, HK=4.
    const CANDIDATES: &[(&str, u32, &str)] = &[
        (
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            2,
            "Noto Sans CJK SC",
        ),
        (
            "/usr/share/fonts/noto-cjk/NotoSansCJKsc-Regular.otf",
            0,
            "Noto Sans CJK SC",
        ),
        (
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            2,
            "Noto Sans CJK SC",
        ),
        (
            "/usr/share/fonts/TTF/IBMPlexSansTC-Regular.ttf",
            0,
            "IBM Plex Sans TC",
        ),
        ("/System/Library/Fonts/PingFang.ttc", 0, "PingFang"),
    ];

    for &(path, index, name) in CANDIDATES {
        let Ok(bytes) = fs::read(path) else {
            continue;
        };
        let mut data = FontData::from_owned(bytes);
        data.index = index;
        fonts.font_data.insert(name.into(), Arc::new(data));
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(name.into());
        }
        return;
    }
}

fn mix(a: Color32, b: Color32, amount: f32) -> Color32 {
    let amount = amount.clamp(0.0, 1.0);
    let [ar, ag, ab, aa] = a.to_srgba_unmultiplied();
    let [br, bg, bb, ba] = b.to_srgba_unmultiplied();
    let channel = |x: u8, y: u8| ((x as f32 * (1.0 - amount)) + (y as f32 * amount)) as u8;
    Color32::from_rgba_unmultiplied(
        channel(ar, br),
        channel(ag, bg),
        channel(ab, bb),
        channel(aa, ba),
    )
}

/// Main content material layer.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn content_frame(theme: FluentTheme) -> Frame {
    Frame::new().fill(palette(theme).app_bg)
}

/// Main content material layer using a complete [`ThemeConfig`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn content_frame_with_config(config: ThemeConfig) -> Frame {
    Frame::new().fill(resolved_palette(config).app_bg)
}

/// Navigation or toolbar material layer.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn surface_frame(theme: FluentTheme) -> Frame {
    Frame::new().fill(palette(theme).nav_bg)
}

/// Navigation or toolbar material layer using a complete [`ThemeConfig`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn surface_frame_with_config(config: ThemeConfig) -> Frame {
    Frame::new().fill(resolved_palette(config).nav_bg)
}

/// Standard WinUI card frame.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn card_frame(theme: FluentTheme) -> Frame {
    let p = palette(theme);
    Frame::new()
        .fill(p.card_bg)
        .stroke(Stroke::new(1.0, p.card_border))
        .corner_radius(0)
        .inner_margin(Margin::same(16))
}

/// Standard WinUI card frame using a complete [`ThemeConfig`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn card_frame_with_config(config: ThemeConfig) -> Frame {
    let p = resolved_palette(config);
    Frame::new()
        .fill(p.card_bg)
        .stroke(Stroke::new(1.0, p.card_border))
        .corner_radius(0)
        .inner_margin(Margin::same(16))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palettes_keep_readable_contrast() {
        let light = palette(FluentTheme::Light);
        let dark = palette(FluentTheme::Dark);
        assert!(light.text_primary.r() < light.app_bg.r());
        assert!(dark.text_primary.r() > dark.app_bg.r());
        assert_ne!(light.accent, dark.accent);
    }

    #[test]
    fn material_opacity_is_clamped() {
        let p = resolved_palette(ThemeConfig {
            material_opacity: 4.0,
            ..ThemeConfig::default()
        });
        assert_eq!(p.app_bg.a(), 255);
    }
}
