# fluent-egui 更新日志

本文件记录该项目所有值得关注的变更。

本文档格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
项目遵循[语义化版本](https://semver.org/lang/zh-CN/spec/v2.0.0.html)。

**其他语言：** [English](CHANGELOG.md) | [中文](CHANGELOG_zh.md)

## [未发布]

## [0.1.0] - 2026-07-20

### 新增

- 为 egui/eframe 提供受 WinUI 3 启发的浅色和深色主题。
- 可复用的 Fluent 控件、布局、Gallery 页面组合及动效 API。
- 高层级的 `FluentUi` 和 `WinUiExt` 页面组合 API。
- 默认使用 Glow 的演示程序后端，并支持可选的 WGPU 后端及不依赖后端的库构建。
- 集成 Mica、Acrylic、Tabbed 和 Aero 背景材质，并支持运行时回退。
- 可选的 `windows-platform` 平台能力适配器。
- 完整的 Gallery 示例，涵盖 105 个控件和 125 个可见页面。
- 内置 Segoe Fluent 图标目录和设计指导组件。
- 公共 API 文档，包含经过编译测试的 `# Examples` 示例章节。
- 面向 WebAssembly 的 WebGL 2 启动辅助函数和浏览器版 Gallery 入口。
- Android NativeActivity 与 GameActivity 启动辅助函数及示例入口。
- Linux X11/Wayland、iOS WGPU/Metal 和旧版 Windows 编译配置。
- 为非 Windows 平台内置采用 MIT 许可证的 Fluent System Icons 后备字体。
- 覆盖 Windows、Linux、Web、Android 和 iOS 的跨平台 CI 编译矩阵。
- 通过手动触发 GitHub Actions 构建发行版，提供可下载的桌面程序、WebAssembly、
  Android NativeActivity 库和未签名 iOS 构件。
- 针对原生平台、Web 和 Android 启动 API 的目标平台条件编译测试。
- 每次手动构建 Demo 构件时发布稳定的 GitHub Release。
- 在手动发布工作流中提供可直接安装、使用调试密钥签名的 Android 7+ NativeActivity APK，
  以及可直接在浏览器中部署的 WebAssembly ZIP 包。
