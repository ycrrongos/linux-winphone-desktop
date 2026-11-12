# fluent-egui

`fluent-egui` is a WinUI 3 inspired theme and widget library for
`egui`/`eframe`, using the `winit` window integration with Glow by default and
an optional WGPU backend.

It provides light and dark Fluent themes, automatic layouts, animated controls,
Gallery-style page composition, Segoe Fluent icons, and optional Windows
platform integrations. It is not an official Microsoft or WinUI project.

For complete instructions on installation, controls, themes, cross-platform launching, and compatibility, please refer to [PROJECT_GUIDE](PROJECT_GUIDE.md).

**Read this in other languages:** [English](README.md) | [中文](README_zh.md)

## Installation

```toml
[dependencies]
fluent-egui = "0.1"
```

Enable optional Windows Runtime capability detection when the target systems
support it:

```toml
fluent-egui = { version = "0.1", features = ["windows-platform"] }
```

## Quick start

```rust,no_run
use eframe::egui;
use fluent_egui::winui::{self, FluentTheme, ThemeConfig};
use fluent_egui::winui::prelude::*;

#[derive(Default)]
struct App {
    name: String,
    remember_me: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.winui(|ui| {
            ui.card("Account", "Sign-in preferences", |ui| {
                ui.text_box_hint(&mut self.name, "User name");
                ui.toggle(&mut self.remember_me, "Remember me");
                ui.wrap(|ui| {
                    ui.accent_button("Save");
                    ui.button("Cancel");
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    winui::run_native(
        "Fluent app",
        ThemeConfig::new(FluentTheme::Light),
        |_| App::default(),
    )
}
```

The high-level API handles control sizing and layout. For detailed
customization, use the lower-level builders without leaving the same scope:

```rust,ignore
ui.button_with("Install", |button| {
    button.icon(FluentIcon::Download).min_width(160.0)
});
ui.slider_with(&mut volume, 0.0..=100.0, |slider| {
    slider.width(320.0).step_by(5.0).show_value(true)
});
ui.control(RatingControl::new(&mut rating).max(10));
```

## Features

- WinUI 3 and Fluent 2 light/dark palettes, typography, spacing, borders and motion.
- High-level `FluentUi` API with automatic row, column, wrapping, card, example
  and Settings layouts.
- Buttons, text input, selection, date/time, navigation, collections, dialogs,
  menus, media surfaces and other reusable Fluent widgets.
- Mica, Acrylic, Tabbed and Aero backdrop requests with runtime fallback.
- Embedded 1,533-entry Segoe Fluent icon catalog.
- AccessKit semantics through eframe for supported controls.
- No XAML, WinAppSDK or WinRT requirement for the default cross-platform build.

Cargo features:

| Feature | Default | Purpose |
|---|---:|---|
| `glow` | yes | eframe Glow/OpenGL renderer and screenshot support |
| `wgpu` | no | eframe WGPU renderer and screenshot support |
| `x11` | yes | Linux X11 window integration |
| `wayland` | yes | Linux Wayland window integration |
| `accesskit` | yes | Native accessibility integration |
| `web` | no | WebAssembly entry points using Glow/WebGL 2 |
| `android-native-activity` | no | Android NativeActivity host using Glow |
| `android-game-activity` | no | Android GameActivity host using Glow |
| `ios` | no | iOS-compatible WGPU/Metal dependency set |
| `win7` | no | Legacy Windows build profile without WinRT adapters |
| `windows-platform` | no | Optional windows-rs capability adapters |

The theme and widget modules do not require either renderer. Applications that
provide their own eframe shell can depend on the backend-free library surface:

```toml
fluent-egui = { version = "0.1", default-features = false }
```

When both `glow` and `wgpu` are enabled, `run_native` continues to select Glow
for compatibility. Use `run_native_with_backend` to select one explicitly.

## Demo

Run the complete control Gallery from the repository:

```bash
cargo run --release --example demo
```

To include optional Windows platform adapters:

```bash
cargo run --release --example demo --features windows-platform
```

Glow is enabled by default. To build the same Demo with WGPU instead:

```bash
cargo run --release --example demo --no-default-features --features wgpu,x11,wayland
```

## Platform behavior

The default feature set keeps platform-specific Windows Runtime adapters
disabled. Native DWM materials are requested only where supported and fall back
to readable Fluent surfaces when unavailable. Mica and Tabbed materials require
modern Windows versions; legacy Aero also depends on desktop composition and
system settings.

| Platform | Build configuration | Compatibility boundary |
|---|---|---|
| Windows 10/11 | Default features | Glow, system Segoe fonts and supported DWM materials |
| Windows 7 | `--no-default-features --features glow,win7` | Compile-checked with the normal MSVC target; use a Win7-capable Rust target/toolchain for a runtime artifact |
| Web | `--target wasm32-unknown-unknown --no-default-features --features web` | Glow with WebGL 2, intended for Chromium 80+ |
| Android 7+ | `--target aarch64-linux-android --no-default-features --features android-native-activity` | Set the host application's `minSdkVersion` to 24; final linking requires the Android NDK |
| Linux | `--no-default-features --features glow,x11,wayland,accesskit` | Current GNOME/KDE Wayland and X11 sessions, plus Budgie/XFCE/MATE X11 sessions |
| iOS 12+ | `--target aarch64-apple-ios --no-default-features --features ios` | Theme/widgets compile with WGPU/Metal; the iOS host owns application lifecycle and surface setup |

The embedded cross-platform Fluent System Icons font is used when Windows
Segoe icon fonts are unavailable. Windows 10/11 continue to prefer the system
Segoe UI, Microsoft YaHei, Segoe MDL2 Assets and Segoe Fluent Icons fonts, so
their existing text metrics and glyph choices are unchanged. The embedded font
is distributed under Microsoft's MIT license in
`src/assets/FLUENT_SYSTEM_ICONS_LICENSE.txt`.

Windows 7 does not implement Mica, WinUI Acrylic or Tabbed backdrops. The
backdrop layer attempts classic DWM/Aero blur and otherwise keeps the Fluent
surface colors readable. Do not enable `windows-platform` for a Windows 7
artifact because those optional WinRT adapters target newer Windows APIs.

These commands validate compilation. Runtime rendering still depends on the
device driver, compositor, browser, activity/host manifest and operating-system
settings, so each shipping artifact should also be exercised on its minimum
supported environment.

## Cross-platform examples

The complete desktop Gallery remains the default Glow + winit example:

```bash
cargo run --release --example demo
```

`examples/web.rs` exposes a `start(HtmlCanvasElement)` WebAssembly function and
retains the eframe runner for the lifetime of the page. The complete Gallery
also exposes `start_gallery(HtmlCanvasElement)` on Web builds. Build them with:

```bash
cargo check --target wasm32-unknown-unknown --no-default-features --features web --example web --example demo
```

`examples/android.rs` shows the `android_main(AndroidApp)` handoff. Package it
as an Android native library with either NativeActivity or GameActivity; the
latter requires a configured Android NDK toolchain.

### Platform startup API

The compatibility work is additive and does not rename or change the signature
of existing theme, widget, layout or native desktop APIs:

- `web_options()` returns the WebGL 2 `eframe::WebOptions` used for Chromium
  80+; `start_web(canvas, config, creator)` installs the Fluent theme and
  returns the `WebRunner`, which the host must retain.
- `android_options(title, app)` attaches winit's `AndroidApp` to the normal
  Fluent native options; `run_android(app, title, config, creator)` installs
  the same theme and shell used by desktop applications.
- `run_native` and `run_native_with_backend` retain their existing signatures
  and continue to select Glow by default.

The platform APIs are target-gated: Web functions are available with `web` on
`wasm32`, and Android functions with either Android activity feature on an
Android target. This keeps unsupported platform types out of ordinary desktop
builds.

## GitHub Actions

`.github/workflows/ci.yml` runs code-quality and package checks for pushes and
pull requests. Use **Actions -> CI and cross-platform demos -> Run workflow**
to enter a release tag/name and manually build Demo artifacts for Windows, the
Windows 7 feature profile, Linux, WebAssembly, Android and iOS. After every
quality and platform job succeeds, the workflow creates or updates a normal
GitHub Release with `draft: false`, `prerelease: false` and `make_latest: true`,
then uploads every staged executable artifact to that release.

The Android job packages a Rust dev-profile `libandroid.so` in a minimal
NativeActivity host and publishes `fluent-egui-demo-android7-arm64.apk`. It is
debug-key signed and can be installed directly for testing; an optimized
`fluent-egui-demo-android7-arm64-release.so` is published separately:

```bash
adb install fluent-egui-demo-android7-arm64.apk
```

The signing key is generated by the GitHub runner, so an APK from a later run
may require uninstalling the previous build before installation. The Web ZIP
contains `index.html`, wasm-bindgen JavaScript and WebAssembly and can be served
by any static HTTP server. iOS output remains an unsigned raw target rather than
an IPA because an installable iOS bundle requires an application host and code
signing identity.

## To the User

I didn't write this library myself; I used `gpt-5.6-sol`. I am not a UI specialist, nor am I particularly proficient in Rust, but I would still welcome any bug reports or suggestions from fellow developers—I will do my best to address them.

## License

This project is dual-licensed under either of the following licenses, at your option:

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

The SPDX license identifier for this project is `MIT OR Apache-2.0`.