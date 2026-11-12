# fluent-egui changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Read this in other languages:** [English](CHANGELOG.md) | [中文](CHANGELOG_zh.md)

## [released]

## [0.1.0] - 2026-07-20

### Added

- WinUI 3 inspired light and dark themes for egui/eframe.
- Reusable Fluent controls, layouts, Gallery composition and motion APIs.
- High-level `FluentUi` and `WinUiExt` composition API.
- Default Glow Demo backend plus optional WGPU and backend-free library builds.
- Runtime-fallback Mica, Acrylic, Tabbed and Aero backdrop integration.
- Optional `windows-platform` capability adapters.
- Complete Gallery example covering 105 controls and 125 visible pages.
- Embedded Segoe Fluent icon catalog and design guidance components.
- Public API documentation with compile-tested `# Examples` sections.
- WebGL 2 startup helpers and browser Gallery entry points for WebAssembly.
- Android NativeActivity and GameActivity startup helpers and example entry point.
- Linux X11/Wayland, iOS WGPU/Metal and legacy Windows compile profiles.
- Embedded MIT-licensed Fluent System Icons fallback for non-Windows platforms.
- Cross-platform CI compile matrix for Windows, Linux, Web, Android and iOS.
- Manual GitHub Actions release builds with downloadable desktop, WebAssembly,
  Android NativeActivity library and unsigned iOS artifacts.
- Target-gated compile tests for the native, Web and Android startup APIs.
- Stable GitHub Release publishing for every manually built Demo artifact.
- An installable, debug-signed Android 7+ NativeActivity APK and a browser-ready
  WebAssembly ZIP in the manual release workflow.
