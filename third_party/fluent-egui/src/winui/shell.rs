use std::time::Duration;

use eframe::{App, CreationContext, Frame, NativeOptions, Renderer, egui};

use super::{
    BackdropKind, BackdropStatus, ThemeConfig, apply_theme, install, sync_window_backdrop,
};

/// Rendering backend used by the native eframe shell.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderBackend {
    /// OpenGL renderer provided by eframe and Glow.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    #[cfg(feature = "glow")]
    #[default]
    Glow,
    /// WGPU renderer provided by eframe.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    #[cfg(feature = "wgpu")]
    #[cfg_attr(not(feature = "glow"), default)]
    Wgpu,
}

impl RenderBackend {
    const fn renderer(self) -> Renderer {
        match self {
            #[cfg(feature = "glow")]
            Self::Glow => Renderer::Glow,
            #[cfg(feature = "wgpu")]
            Self::Wgpu => Renderer::Wgpu,
        }
    }

    /// Human-readable backend name used by diagnostics and About pages.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn name(self) -> &'static str {
        match self {
            #[cfg(feature = "glow")]
            Self::Glow => "Glow / OpenGL",
            #[cfg(feature = "wgpu")]
            Self::Wgpu => "WGPU",
        }
    }
}

/// Native defaults for the transparent viewport used by the Fluent shell.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn native_options(title: impl Into<String>) -> NativeOptions {
    native_options_with_backend(title, RenderBackend::default())
}

/// Native defaults for a transparent Fluent viewport using the selected backend.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn native_options_with_backend(
    title: impl Into<String>,
    backend: RenderBackend,
) -> NativeOptions {
    NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_app_id("fluent-egui")
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([760.0, 560.0])
            .with_transparent(true)
            .with_decorations(true)
            .with_resizable(true)
            .with_minimize_button(true)
            .with_maximize_button(true),
        renderer: backend.renderer(),
        centered: true,
        multisampling: 4,
        ..Default::default()
    }
}

/// Starts an eframe application inside the transparent Fluent shell.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn run_native<A, F>(title: &str, config: ThemeConfig, creator: F) -> eframe::Result
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    run_native_with_backend(title, config, RenderBackend::default(), creator)
}

/// Starts an eframe application with an explicitly selected rendering backend.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn run_native_with_backend<A, F>(
    title: &str,
    config: ThemeConfig,
    backend: RenderBackend,
    creator: F,
) -> eframe::Result
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    let options = native_options_with_backend(title, backend);
    run_native_with_options(title, config, options, creator)
}

/// Starts an eframe application with fully custom [`NativeOptions`] (viewport, etc.).
pub fn run_native_with_options<A, F>(
    title: &str,
    config: ThemeConfig,
    options: NativeOptions,
    creator: F,
) -> eframe::Result
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    run_with_options(title, config, options, creator)
}

#[cfg(all(
    target_os = "android",
    any(feature = "android-native-activity", feature = "android-game-activity")
))]
/// Creates native options carrying the Android activity required by winit.
///
/// # Examples
///
/// ```rust,no_run
/// # #[cfg(target_os = "android")]
/// # fn options(app: winit::platform::android::activity::AndroidApp) {
/// let options = fluent_egui::winui::android_options("Fluent Android", app);
/// assert!(options.android_app.is_some());
/// # }
/// ```
pub fn android_options(
    title: impl Into<String>,
    android_app: winit::platform::android::activity::AndroidApp,
) -> NativeOptions {
    let mut options = native_options(title);
    options.android_app = Some(android_app);
    options
}

#[cfg(all(
    target_os = "android",
    any(feature = "android-native-activity", feature = "android-game-activity")
))]
/// Starts an eframe application using the activity supplied by the Android host.
///
/// # Examples
///
/// ```rust,no_run
/// # #[cfg(target_os = "android")]
/// # fn android_main(app: winit::platform::android::activity::AndroidApp) -> eframe::Result {
/// use fluent_egui::winui::{self, FluentTheme, ThemeConfig};
///
/// #[derive(Default)]
/// struct App;
///
/// impl eframe::App for App {
///     fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
///         ui.label("Fluent Android");
///     }
/// }
///
/// winui::run_android(
///     app,
///     "Fluent Android",
///     ThemeConfig::new(FluentTheme::Light),
///     |_| App,
/// )
/// # }
/// ```
pub fn run_android<A, F>(
    android_app: winit::platform::android::activity::AndroidApp,
    title: &str,
    config: ThemeConfig,
    creator: F,
) -> eframe::Result
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    let options = android_options(title, android_app);
    run_with_options(title, config, options, creator)
}

fn run_with_options<A, F>(
    title: &str,
    config: ThemeConfig,
    options: NativeOptions,
    creator: F,
) -> eframe::Result
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    eframe::run_native(
        title,
        options,
        Box::new(move |cc| {
            install(&cc.egui_ctx, config);
            if let Some(window) = cc.winit_window() {
                let _ = sync_window_backdrop(window, config.theme, BackdropKind::Mica);
            }
            Ok(Box::new(FluentShell::new(creator(cc))))
        }),
    )
}

/// Applies a theme immediately and synchronizes the native title bar/backdrop.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn apply_runtime_theme(
    ctx: &egui::Context,
    frame: &Frame,
    config: ThemeConfig,
) -> BackdropStatus {
    apply_theme(ctx, config);
    frame
        .winit_window()
        .map_or(BackdropStatus::Unsupported, |window| {
            sync_window_backdrop(window, config.theme, BackdropKind::Mica)
        })
}

struct FluentShell<A> {
    app: A,
    capture: Option<CaptureState>,
}

impl<A> FluentShell<A> {
    fn new(app: A) -> Self {
        Self {
            app,
            capture: std::env::var_os("WINUI_EGUI_SCREENSHOT").map(|path| CaptureState {
                path: path.into(),
                frames: 0,
                requested: false,
                started: std::time::Instant::now(),
                delay: std::env::var("WINUI_EGUI_SCREENSHOT_DELAY_MS")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .map_or(Duration::ZERO, Duration::from_millis),
            }),
        }
    }
}

struct CaptureState {
    path: std::path::PathBuf,
    frames: u8,
    requested: bool,
    started: std::time::Instant,
    delay: Duration,
}

impl<A: App> App for FluentShell<A> {
    fn logic(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.app.logic(ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame) {
        if let Some(capture) = &self.capture {
            let screenshot = ui.input(|input| {
                input.events.iter().find_map(|event| match event {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });
            if let Some(image) = screenshot {
                let rgba = image
                    .pixels
                    .iter()
                    .flat_map(|pixel| pixel.to_array())
                    .collect::<Vec<_>>();
                if let Some(parent) = capture.path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = image::save_buffer(
                    &capture.path,
                    &rgba,
                    image.size[0] as u32,
                    image.size[1] as u32,
                    image::ColorType::Rgba8,
                );
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                self.capture = None;
            }
        }

        self.app.ui(ui, frame);

        if let Some(capture) = &mut self.capture {
            capture.frames = capture.frames.saturating_add(1);
            if capture.frames >= 2
                && capture.started.elapsed() >= capture.delay
                && !capture.requested
            {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::new(
                        "winui-egui-capture",
                    )));
                capture.requested = true;
            } else if !capture.requested {
                ui.ctx().request_repaint_after(Duration::from_millis(16));
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.app.save(storage);
    }

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        self.app.on_exit(gl);
    }

    #[cfg(not(feature = "glow"))]
    fn on_exit(&mut self) {
        self.app.on_exit();
    }

    fn auto_save_interval(&self) -> Duration {
        self.app.auto_save_interval()
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn persist_egui_memory(&self) -> bool {
        self.app.persist_egui_memory()
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.app.raw_input_hook(ctx, raw_input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_backend_preserves_glow_compatibility() {
        #[cfg(feature = "glow")]
        assert_eq!(RenderBackend::default(), RenderBackend::Glow);

        #[cfg(all(not(feature = "glow"), feature = "wgpu"))]
        assert_eq!(RenderBackend::default(), RenderBackend::Wgpu);
    }
}
