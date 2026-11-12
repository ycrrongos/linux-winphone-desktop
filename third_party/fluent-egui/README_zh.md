# fluent-egui

`fluent-egui` 是一个受 WinUI 3 启发、面向 `egui`/`eframe` 的主题与控件库。默认通过 `winit` 集成窗口并使用 Glow 渲染，也可选择 WGPU 后端。

它提供 Fluent 浅色与深色主题、自动布局、动画控件、Gallery 风格的页面组合、Segoe Fluent 图标，以及可选的 Windows 平台集成。本项目并非 Microsoft 或 WinUI 官方项目。

有关安装、控件、主题、跨平台启动和兼容性的完整说明，请参阅[项目指南](PROJECT_GUIDE_zh.md)。

**其他语言：** [English](README.md) | [中文](README_zh.md)

## 安装

```toml
[dependencies]
fluent-egui = "0.1"
```

如果目标系统支持 Windows Runtime 能力检测，可启用相应的可选功能：

```toml
fluent-egui = { version = "0.1", features = ["windows-platform"] }
```

## 快速开始

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
            ui.card("账户", "登录偏好", |ui| {
                ui.text_box_hint(&mut self.name, "用户名");
                ui.toggle(&mut self.remember_me, "记住我");
                ui.wrap(|ui| {
                    ui.accent_button("保存");
                    ui.button("取消");
                });
            });
        });
    }
}

fn main() -> eframe::Result {
    winui::run_native(
        "Fluent 应用",
        ThemeConfig::new(FluentTheme::Light),
        |_| App::default(),
    )
}
```

高层级 API 会自动处理控件尺寸和布局。如需精细定制，可在同一作用域内使用底层构建器：

```rust,ignore
ui.button_with("安装", |button| {
    button.icon(FluentIcon::Download).min_width(160.0)
});
ui.slider_with(&mut volume, 0.0..=100.0, |slider| {
    slider.width(320.0).step_by(5.0).show_value(true)
});
ui.control(RatingControl::new(&mut rating).max(10));
```

## 功能特性

- WinUI 3 和 Fluent 2 风格的浅色/深色调色板、字体排印、间距、边框和动效。
- 高层级 `FluentUi` API，支持自动行列布局、换行布局、卡片、示例和设置页布局。
- 按钮、文本输入、选择器、日期/时间、导航、集合、对话框、菜单、媒体界面及其他可复用 Fluent 控件。
- 支持请求 Mica、Acrylic、Tabbed 和 Aero 背景，并提供运行时回退。
- 内置包含 1,533 个条目的 Segoe Fluent 图标目录。
- 通过 eframe 为受支持控件提供 AccessKit 语义。
- 默认的跨平台构建无需 XAML、WinAppSDK 或 WinRT。

Cargo 功能：

| 功能 | 默认启用 | 用途 |
|---|---:|---|
| `glow` | 是 | eframe Glow/OpenGL 渲染器及截图支持 |
| `wgpu` | 否 | eframe WGPU 渲染器及截图支持 |
| `x11` | 是 | Linux X11 窗口集成 |
| `wayland` | 是 | Linux Wayland 窗口集成 |
| `accesskit` | 是 | 原生无障碍集成 |
| `web` | 否 | 使用 Glow/WebGL 2 的 WebAssembly 入口 |
| `android-native-activity` | 否 | 使用 Glow 的 Android NativeActivity 宿主 |
| `android-game-activity` | 否 | 使用 Glow 的 Android GameActivity 宿主 |
| `ios` | 否 | 兼容 iOS 的 WGPU/Metal 依赖集 |
| `win7` | 否 | 不含 WinRT 适配器的旧版 Windows 构建配置 |
| `windows-platform` | 否 | 可选的 windows-rs 能力适配器 |

主题和控件模块均不依赖特定渲染器。自行提供 eframe 外壳的应用可以仅依赖不含后端的库接口：

```toml
fluent-egui = { version = "0.1", default-features = false }
```

同时启用 `glow` 和 `wgpu` 时，`run_native` 仍会优先选择 Glow，以保证兼容性。可使用 `run_native_with_backend` 显式指定后端。

## 演示程序

在仓库中运行完整控件 Gallery：

```bash
cargo run --release --example demo
```

加入可选的 Windows 平台适配器：

```bash
cargo run --release --example demo --features windows-platform
```

默认启用 Glow。要改用 WGPU 构建同一 Demo：

```bash
cargo run --release --example demo --no-default-features --features wgpu,x11,wayland
```

## 平台行为

默认功能集不会启用 Windows Runtime 平台专用适配器。仅在系统支持时请求原生 DWM 材质；不可用时会回退为清晰可读的 Fluent 表面。Mica 和 Tabbed 材质要求较新的 Windows 版本；旧式 Aero 还依赖桌面合成与系统设置。

| 平台 | 构建配置 | 兼容范围 |
|---|---|---|
| Windows 10/11 | 默认功能 | Glow、系统 Segoe 字体及受支持的 DWM 材质 |
| Windows 7 | `--no-default-features --features glow,win7` | 使用常规 MSVC 目标检查编译；运行构件需使用支持 Win7 的 Rust 目标/工具链 |
| Web | `--target wasm32-unknown-unknown --no-default-features --features web` | 使用 WebGL 2 的 Glow，面向 Chromium 80+ |
| Android 7+ | `--target aarch64-linux-android --no-default-features --features android-native-activity` | 宿主应用的 `minSdkVersion` 设为 24；最终链接需要 Android NDK |
| Linux | `--no-default-features --features glow,x11,wayland,accesskit` | 当前 GNOME/KDE Wayland 与 X11 会话，以及 Budgie/XFCE/MATE X11 会话 |
| iOS 12+ | `--target aarch64-apple-ios --no-default-features --features ios` | 主题/控件可通过 WGPU/Metal 编译；应用生命周期及表面初始化由 iOS 宿主管理 |

Windows Segoe 图标字体不可用时，会使用内置的跨平台 Fluent System Icons 字体。Windows 10/11 仍优先使用系统中的 Segoe UI、Microsoft YaHei、Segoe MDL2 Assets 和 Segoe Fluent Icons 字体，因此原有文本度量和字形选择不受影响。内置字体依据 Microsoft MIT 许可证分发，许可证见 `src/assets/FLUENT_SYSTEM_ICONS_LICENSE.txt`。

Windows 7 不支持 Mica、WinUI Acrylic 或 Tabbed 背景。背景层会尝试使用经典 DWM/Aero 模糊，否则保留清晰可读的 Fluent 表面色。不要为 Windows 7 构件启用 `windows-platform`，因为这些可选 WinRT 适配器面向更新的 Windows API。

上述命令用于验证编译。运行时渲染仍取决于设备驱动、合成器、浏览器、Activity/宿主清单及操作系统设置，因此每个发布构件都应在其最低支持环境中进行实际测试。

## 跨平台示例

完整桌面 Gallery 仍是默认的 Glow + winit 示例：

```bash
cargo run --release --example demo
```

`examples/web.rs` 暴露 WebAssembly 函数 `start(HtmlCanvasElement)`，并在页面生命周期内保留 eframe runner。完整 Gallery 在 Web 构建中还会暴露 `start_gallery(HtmlCanvasElement)`。构建命令如下：

```bash
cargo check --target wasm32-unknown-unknown --no-default-features --features web --example web --example demo
```

`examples/android.rs` 展示 `android_main(AndroidApp)` 的控制权交接。可通过 NativeActivity 或 GameActivity 将其打包为 Android 原生库；后者要求配置好 Android NDK 工具链。

### 平台启动 API

兼容性功能以增量方式加入，不会重命名或改变现有主题、控件、布局或原生桌面 API 的签名：

- `web_options()` 返回适用于 Chromium 80+、使用 WebGL 2 的 `eframe::WebOptions`；`start_web(canvas, config, creator)` 安装 Fluent 主题并返回必须由宿主长期保留的 `WebRunner`。
- `android_options(title, app)` 将 winit 的 `AndroidApp` 附加到常规 Fluent 原生选项；`run_android(app, title, config, creator)` 安装与桌面应用相同的主题和外壳。
- `run_native` 和 `run_native_with_backend` 保持现有签名，并继续默认选择 Glow。

平台 API 受目标条件限制：Web 函数仅在 `wasm32` 目标启用 `web` 时可用，Android 函数仅在 Android 目标启用任一 Android Activity 功能时可用。这样可避免普通桌面构建引入不受支持的平台类型。

## GitHub Actions

`.github/workflows/ci.yml` 会为 push 和 Pull Request 执行代码质量及包检查。通过 **Actions -> CI and cross-platform demos -> Run workflow** 输入发布标签/名称，可手动构建 Windows、Windows 7 功能配置、Linux、WebAssembly、Android 和 iOS 的 Demo 构件。所有质量与平台任务成功后，工作流会创建或更新普通 GitHub Release，设置 `draft: false`、`prerelease: false` 和 `make_latest: true`，再将所有暂存的可执行构件上传至该 Release。

Android 任务会在最小 NativeActivity 宿主中打包 Rust dev profile 的 `libandroid.so`，并发布 `fluent-egui-demo-android7-arm64.apk`。该 APK 使用调试密钥签名，可直接安装测试；优化后的 `fluent-egui-demo-android7-arm64-release.so` 会单独发布：

```bash
adb install fluent-egui-demo-android7-arm64.apk
```

签名密钥由 GitHub runner 临时生成，因此后续运行产生的 APK 可能需要先卸载旧版本才能安装。Web ZIP 包含 `index.html`、wasm-bindgen JavaScript 和 WebAssembly，可由任意静态 HTTP 服务器托管。iOS 输出仍是未签名的原始目标，而非 IPA，因为可安装的 iOS 包需要应用宿主和代码签名身份。

## 致用户

这个库并非由我亲自编写，而是借助 `gpt-5.6-sol` 完成的。我不是 UI 专家，对 Rust 也谈不上精通，但仍非常欢迎各位开发者提交错误报告或建议，我会尽力处理。

## 许可证

本项目采用双重许可证，你可以任选其一：

* [Apache License, Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)

本项目的 SPDX 许可证标识符为 `MIT OR Apache-2.0`。
