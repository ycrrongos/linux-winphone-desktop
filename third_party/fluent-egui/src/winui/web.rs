use eframe::{App, CreationContext, Renderer, WebGlContextOption, WebOptions, WebRunner};
use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;

use super::{ThemeConfig, install};

/// Returns WebGL 2 options compatible with Chromium 80 and newer.
///
/// # Examples
///
/// ```rust
/// # #[cfg(target_arch = "wasm32")]
/// # {
/// let options = fluent_egui::winui::web_options();
/// assert!(matches!(options.renderer, eframe::Renderer::Glow));
/// # }
/// ```
pub fn web_options() -> WebOptions {
    WebOptions {
        renderer: Renderer::Glow,
        webgl_context_option: WebGlContextOption::WebGl2,
        ..Default::default()
    }
}

/// Installs the Fluent theme and starts an eframe application on an HTML canvas.
///
/// The returned [`WebRunner`] must be kept alive by the JavaScript-facing application handle.
///
/// # Examples
///
/// ```rust,no_run
/// # #[cfg(target_arch = "wasm32")]
/// # async fn launch(canvas: web_sys::HtmlCanvasElement) -> Result<(), wasm_bindgen::JsValue> {
/// use fluent_egui::winui::{self, FluentTheme, ThemeConfig};
///
/// #[derive(Default)]
/// struct App;
///
/// impl eframe::App for App {
///     fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
///         ui.label("Fluent Web");
///     }
/// }
///
/// let runner = winui::start_web(
///     canvas,
///     ThemeConfig::new(FluentTheme::Light),
///     |_| App,
/// )
/// .await?;
/// // Keep `runner` alive for as long as the page is running.
/// std::mem::forget(runner);
/// # Ok(())
/// # }
/// ```
pub async fn start_web<A, F>(
    canvas: HtmlCanvasElement,
    config: ThemeConfig,
    creator: F,
) -> Result<WebRunner, JsValue>
where
    A: App + 'static,
    F: FnOnce(&CreationContext<'_>) -> A + 'static,
{
    let runner = WebRunner::new();
    runner
        .start(
            canvas,
            web_options(),
            Box::new(move |cc| {
                install(&cc.egui_ctx, config);
                Ok(Box::new(creator(cc)))
            }),
        )
        .await?;
    Ok(runner)
}
