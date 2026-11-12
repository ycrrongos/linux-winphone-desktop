//! Compile-time coverage for target-gated public startup APIs.

#[cfg(all(
    any(feature = "glow", feature = "wgpu"),
    not(target_arch = "wasm32"),
    not(target_os = "ios")
))]
#[test]
fn native_shell_api_remains_public() {
    use fluent_egui::winui::{RenderBackend, native_options};

    let options = native_options("Fluent test");
    #[cfg(feature = "glow")]
    assert!(matches!(options.renderer, eframe::Renderer::Glow));
    #[cfg(all(not(feature = "glow"), feature = "wgpu"))]
    assert!(matches!(options.renderer, eframe::Renderer::Wgpu));
    assert!(!RenderBackend::default().name().is_empty());
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[test]
fn web_options_select_glow_and_webgl2() {
    use eframe::{Renderer, WebGlContextOption};
    use fluent_egui::winui::web_options;

    let options = web_options();
    assert!(matches!(options.renderer, Renderer::Glow));
    assert!(matches!(
        options.webgl_context_option,
        WebGlContextOption::WebGl2
    ));
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[allow(dead_code)]
async fn web_start_api_is_callable(
    canvas: web_sys::HtmlCanvasElement,
) -> Result<eframe::WebRunner, wasm_bindgen::JsValue> {
    use fluent_egui::winui::{self, FluentTheme, ThemeConfig};

    #[derive(Default)]
    struct App;

    impl eframe::App for App {
        fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
            ui.label("Web API compile test");
        }
    }

    winui::start_web(canvas, ThemeConfig::new(FluentTheme::Light), |_| App).await
}

#[cfg(all(
    target_os = "android",
    any(feature = "android-native-activity", feature = "android-game-activity")
))]
#[allow(dead_code)]
fn android_start_api_is_callable(
    app: winit::platform::android::activity::AndroidApp,
) -> eframe::Result {
    use fluent_egui::winui::{self, FluentTheme, ThemeConfig};

    #[derive(Default)]
    struct App;

    impl eframe::App for App {
        fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
            ui.label("Android API compile test");
        }
    }

    let options = winui::android_options("Fluent test", app.clone());
    assert!(options.android_app.is_some());
    winui::run_android(
        app,
        "Fluent test",
        ThemeConfig::new(FluentTheme::Light),
        |_| App,
    )
}
