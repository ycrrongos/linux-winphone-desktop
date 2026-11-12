# fluent-egui 用户指南与 API 实用手册

`fluent-egui` 是一个受 WinUI 3 和 Fluent 2 启发、面向 `egui`/`eframe` 的主题与控件库。它无需 XAML，即可提供浅色和深色主题、自动布局、动画控件、Fluent 图标、Gallery 页面组合辅助工具，以及可选的 Windows 原生背景材质。

本文面向使用该 crate 开发应用的开发者，介绍状态如何在公共 API 中流转、各类公共 API 的调用方式、应检查哪些响应，以及需要启用哪些平台功能。每个字段和方法的精确定义仍以生成的 rustdoc 为准。

除非代码片段显式给出导入语句，否则均假定已经使用 `use fluent_egui::winui::{self, *};`、可以访问 `egui::Ui`，并且应用拥有示例中同名变量对应的状态。`save()`、`draw_page()` 等简短辅助函数代表应用自身的代码。

`fluent-egui` 并非 Microsoft 或 WinUI 官方项目。

## 目录

- [安装与功能](#安装与功能)
- [应用模型](#应用模型)
- [桌面端启动](#桌面端启动)
- [主题与原生背景](#主题与原生背景)
- [高层级页面组合 API](#高层级页面组合-api)
- [核心控件](#核心控件)
- [扩展输入与导航控件](#扩展输入与导航控件)
- [文本控件](#文本控件)
- [日期和时间控件](#日期和时间控件)
- [集合与滚动控件](#集合与滚动控件)
- [命令菜单与导航](#命令菜单与导航)
- [布局绘制与画刷](#布局绘制与画刷)
- [动效与过渡](#动效与过渡)
- [Gallery 与设计辅助工具](#gallery-与设计辅助工具)
- [图标](#图标)
- [平台界面与请求](#平台界面与请求)
- [Web、Android、Linux、Windows 7 与 iOS](#跨平台启动)
- [Demo 发布构件](#demo-发布构件)

## 安装与功能

将该 crate 添加到应用：

```toml
[dependencies]
fluent-egui = "0.1"
```

默认功能集使用 Glow，同时启用 X11 和 Wayland 支持：

```toml
fluent-egui = {
    version = "0.1",
    features = ["glow", "x11", "wayland", "accesskit"]
}
```

如果应用自行管理 eframe 外壳，可使用不依赖后端的主题和控件接口：

```toml
fluent-egui = { version = "0.1", default-features = false }
```

| 功能 | 默认启用 | 用途 |
|---|---:|---|
| `glow` | 是 | Glow/OpenGL 渲染器及截图支持 |
| `wgpu` | 否 | WGPU 渲染器及截图支持 |
| `x11` | 是 | Linux X11 集成 |
| `wayland` | 是 | Linux Wayland 集成 |
| `accesskit` | 是 | 原生无障碍语义 |
| `web` | 否 | 使用 Glow/WebGL 2 启动 WebAssembly |
| `android-native-activity` | 否 | Android NativeActivity 入口 |
| `android-game-activity` | 否 | Android GameActivity 入口 |
| `ios` | 否 | iOS WGPU/Metal 依赖集 |
| `win7` | 否 | 不含现代 WinRT 适配器的旧版 Windows 构建配置 |
| `windows-platform` | 否 | 可选的 windows-rs 能力检测 |

## 应用模型

该 crate 遵循 egui 的立即模式状态模型：

- 应用负责持有 `bool`、`String`、`usize`、日期、选中行和对话框可见性等值；
- 控件在每一帧临时借用这些状态；
- 控件通常返回 `egui::Response`，包含多种操作时则返回特定类型的响应；
- 渲染由 `eframe::App::ui` 驱动；
- 视觉默认值和动画状态由库持有，并存储在 egui 上下文中。

普通控件使用标准 `Response` 方法：

```rust
let response = ui.add(FluentButton::accent("保存"));

if response.clicked() {
    save();
}
if response.hovered() {
    // 可选的应用行为。
}
```

类型化响应会区分不同操作。例如，`SettingsChoiceResponse` 可区分选中项目与进入项目，`ContentDialogResponse` 则能指出关闭对话框的是哪个按钮。

## 桌面端启动

最小的完整桌面应用如下：

```rust
use eframe::egui;
use fluent_egui::winui::{self, FluentTheme, ThemeConfig};
use fluent_egui::winui::prelude::*;

#[derive(Default)]
struct App {
    name: String,
    notifications: bool,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.winui(|ui| {
            ui.page_header("账户", "管理你的偏好设置");
            ui.card("个人资料", "个人信息", |ui| {
                ui.text_box_hint(&mut self.name, "用户名");
                ui.toggle(&mut self.notifications, "通知");
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

### 原生外壳函数

| API | 调用方式 | 返回值 |
|---|---|---|
| `native_options(title)` | 创建默认透明窗口选项 | `eframe::NativeOptions` |
| `native_options_with_backend(title, backend)` | 选择 Glow 或 WGPU | `eframe::NativeOptions` |
| `run_native(title, config, creator)` | 使用默认后端启动 | `eframe::Result` |
| `run_native_with_backend(...)` | 使用 `RenderBackend::Glow` 或 `Wgpu` 启动 | `eframe::Result` |
| `apply_runtime_theme(ctx, frame, config)` | 更新 egui 视觉样式和原生背景 | `BackdropStatus` |

同时启用两种渲染器功能时，`RenderBackend::default()` 为 Glow。可显式选择 WGPU：

```rust
winui::run_native_with_backend(
    "WGPU 应用",
    ThemeConfig::new(FluentTheme::Dark),
    RenderBackend::Wgpu,
    |_| App::default(),
)
```

## 主题与原生背景

### 主题配置

`FluentTheme` 包含 `Light` 和 `Dark`。`ThemeConfig` 用于控制主题、强调色、材质不透明度和动画：

```rust
let config = ThemeConfig::new(FluentTheme::Dark)
    .with_accent(egui::Color32::from_rgb(96, 205, 255))
    .with_material_opacity(0.72)
    .with_animations(true);
```

主题 API 的用法如下：

```rust
// 初次安装字体和样式。
winui::install(&cc.egui_ctx, config);

// 运行时只更改 egui 视觉样式。
winui::apply_theme(ctx, config);

// 获取用于自定义绘制的颜色。
let colors: FluentPalette = winui::palette_with_config(config);

// 绘制标准材质表面。
winui::content_frame_with_config(config).show(ui, |ui| { /* 页面 */ });
winui::surface_frame_with_config(config).show(ui, |ui| { /* 工具栏 */ });
winui::card_frame_with_config(config).show(ui, |ui| { /* 卡片 */ });
```

其他主题辅助函数包括 `default_accent`、`accent_presets`、`palette`、`content_frame`、`surface_frame` 和 `card_frame`。

### 背景类型

`BackdropKind` 支持 `Auto`、`Mica`、`Acrylic`、`Tabbed`、`Aero` 和 `None`：

```rust
if let Some(window) = frame.winit_window() {
    let status = winui::sync_window_backdrop(
        window,
        FluentTheme::Dark,
        BackdropKind::Acrylic,
    );

    match status {
        BackdropStatus::Applied => {}
        BackdropStatus::Unsupported => {}
        BackdropStatus::Failed(code) => eprintln!("DWM 错误：{code}"),
    }
}
```

Mica、系统 Acrylic 和 Tabbed/Alt Mica 属于 Windows 11 能力。Windows 10 和 Windows 7 会在条件允许时回退至旧式模糊或 Aero。非 Windows 目标返回 `Unsupported`；没有系统背景时，Fluent 表面仍会保持清晰可读。

## 高层级页面组合 API

导入 prelude 并进入 `WinUiExt` 作用域：

```rust
use fluent_egui::winui::prelude::*;

ui.winui(|ui| {
    ui.button("默认");
    ui.accent_button("主要操作");
});
```

`winui()` 会根据已安装的 egui 视觉样式推断主题。独立预览可使用 `winui_with_theme(theme, ...)`。

### FluentUi 方法参考

| 方法 | 状态/输入 | 返回值 |
|---|---|---|
| `button(text)` | 命令标签 | `Response` |
| `accent_button(text)` | 主要命令标签 | `Response` |
| `subtle_button(text)` | 低强调命令 | `Response` |
| `icon_button(icon)` | `FluentIcon` | `Response` |
| `button_with(text, configure)` | 完整 `FluentButton` 构建器 | `Response` |
| `text_box(text)` | `&mut TextBuffer` | `Response` |
| `text_box_hint(text, hint)` | 文本与占位提示 | `Response` |
| `text_box_with(text, configure)` | 完整 `FluentTextEdit` 构建器 | `Response` |
| `password_box(text)` | `&mut String` | `Response` |
| `toggle(value, label)` | `&mut bool` | `Response` |
| `slider(value, range)` | `&mut f32` | `Response` |
| `slider_with(value, range, configure)` | 完整滑块构建器 | `Response` |
| `progress(value)` | 归一化 `f32` | `Response` |
| `segmented(selected, labels)` | `&mut usize` | `Response` |
| `select(id, selected, values)` | 选择状态和字符串值 | `Response` |
| `row`、`column`、`wrap` | 布局闭包 | 闭包返回值 |
| `card(title, subtitle, body)` | 卡片内容闭包 | `InnerResponse` |
| `example(title, body)` | Gallery 示例块 | `InnerResponse` |
| `setting(...)` | 设置页样式的行 | `Response` |
| `page_header(title, description)` | 页面标题 | `GalleryPageHeaderResponse` |
| `control(widget)` | 任意 `egui::Widget` | `Response` |

`FluentUi` 实现了 `Deref<Target = egui::Ui>`，因此作用域内仍可使用标签、分隔线、滚动、网格和所有 egui 原生方法。

无需离开简洁 API 即可定制控件：

```rust
ui.winui(|ui| {
    ui.button_with("安装", |button| {
        button.icon(FluentIcon::Download).min_width(160.0)
    });
    ui.text_box_with(&mut query, |edit| {
        edit.hint_text("搜索").char_limit(128)
    });
    ui.slider_with(&mut volume, 0.0..=100.0, |slider| {
        slider.width(320.0).step_by(5.0).show_value(true)
    });
    ui.control(RatingControl::new(&mut rating).max(10));
});
```

## 核心控件

### 按钮

```rust
ui.add(FluentButton::new("取消"));
ui.add(FluentButton::accent("保存").min_width(100.0));
ui.add(FluentButton::subtle("添加").icon(FluentIcon::Add));
ui.add(AccentButton::new("保存").icon(FluentIcon::Check));
ui.add(IconButton::new(FluentIcon::Download).selected(false))
    .on_hover_text("下载");
```

`FluentButton` 的构建方法有 `icon`、`kind` 和 `min_width`。`ButtonKind` 可取 `Standard`、`Accent` 或 `Subtle`。`AccentButton` 是便捷包装器；`IconButton` 使用固定的命令点击区域并支持 `selected(bool)`。

按钮内容不是纯文本时，使用 `FluentContentButton`。其 `show` 闭包可组合任意内容，返回响应的 `response.clicked()` 用于判断点击。

### 文本输入

```rust
ui.add(
    FluentTextEdit::new(&mut text)
        .hint_text("搜索")
        .password(false)
        .char_limit(128),
);
ui.add(fluent_text_edit(&mut text).hint_text("搜索"));
```

`FluentTextEdit` 不持有文本，而是借用 egui 的 `TextBuffer`。它会根据 Fluent 度量对齐文本、占位文字和插入光标，并返回标准 `Response` 中的变化与焦点信息。

### 开关、三态复选框、分段控件与滑块

```rust
ui.add(ToggleSwitch::new(&mut enabled).label("开启"));
let mut tri_state = Some(true);
ui.add(TriStateCheckBox::new(&mut tri_state, "可选功能"));
ui.add(SegmentedControl::new(&mut selected, ["预览", "代码", "详情"]).width(300.0));
ui.add(FluentSlider::new(&mut volume, 0.0..=100.0).width(240.0).step_by(5.0).show_value(true));
ui.add(FluentSlider::new(&mut gain, 0.0..=1.0).vertical(180.0));
```

各控件都会直接修改借用的状态，并通过 `Response::changed()` 报告变化。

### 进度

```rust
ui.add(FluentProgressBar::new(0.68).width(220.0).show_percentage(true));
ui.add(ProgressRing::new(Some(0.68)).size(28.0));
ui.add(ProgressRing::new(None).size(32.0));
```

`Some(value)` 创建确定进度环，`None` 创建动画形式的不确定进度环。

### 组合框辅助函数

`fluent_combo_box` 返回带 Fluent 尺寸和下拉箭头的 egui `ComboBox` 构建器；`combo_option` 绘制高度稳定、左侧带选中指示器的菜单行。

### 浮出菜单与设置行

`FlyoutButton` 组合标题、副标题、图标和浮出内容。`FlyoutOption` 支持 `selected` 和 `navigates` 状态。原地展开的设置卡片使用 `SettingsExpander`，其返回值分别通过 `header_response` 和 `body` 暴露标题与内容响应。

`SettingsChoiceRow` 可通过 `navigation_clicked()` 区分导航操作与普通选择；`SettingsNavigationRow::show` 返回单个导航响应，带开关或选择器的尾部区域则使用 `show_with_trailing`。

### 对话框、InfoBar 与 Toast

```rust
let dialog = ContentDialog::new("save-dialog", "确认操作", "要保存当前偏好设置吗？")
    .primary_text("保存")
    .secondary_text("不保存")
    .close_text("取消")
    .width(380.0)
    .show(ctx, theme);

match dialog.action {
    Some(ContentDialogAction::Primary) => save(),
    Some(ContentDialogAction::Secondary) => discard(),
    Some(ContentDialogAction::Close) | None => {}
}

let close = InfoBar::new("已保存", "偏好设置已更新")
    .kind(InfoBarKind::Success).icon_visible(true).closable(true).show(ui);

Toast::new("已保存", "你的更改已准备就绪")
    .icon(FluentIcon::Check).show(ctx, theme, toast_visible);
```

### 页面组合辅助函数

`settings_group` 与 `setting_row` 用于设置组；`section_card` 创建带标题和说明的区块卡片；`card_columns` 提供多列卡片布局；`animated_page` 根据页面 ID 和修订号添加页面动效。其他导航辅助函数有 `navigation_item`、`navigation_child_item`、`navigation_branch_item` 和 `NavigationGroup`。

## 扩展输入与导航控件

### 命令型控件

```rust
ui.add(HyperlinkButton::new("打开文档"));
ui.add(ToggleButton::new(&mut pinned, "已固定"));
ui.add(RepeatButton::new("增加").repeats_per_second(8.0));
```

`RepeatButton` 在短按时触发一次，只有超过按住延迟后才开始连续触发。

### 拆分按钮与下拉按钮

`SplitButton` 分别通过 `response` 和 `menu_response` 暴露主操作及菜单操作。`DropDownButton` 只提供菜单入口。`ColorSplitButton` 绑定选中的 `Color32` 和调色板，返回 `ColorSplitButtonResponse`；`ToggleSplitButton` 则将可变布尔状态与拆分菜单组合起来。

### 评分、NumberBox、徽章与分页器

```rust
ui.add(RatingControl::new(&mut rating).max(10).allow_clear(true).read_only(false));
ui.add(NumberBox::new(&mut amount).range(0.0..=100.0).step(0.5).width(160.0)
    .spin_button_placement(NumberBoxSpinButtonPlacement::Compact));
let result = evaluate_number_expression("(12 + 3) * 2")?;
ui.add(InfoBadge::dot());
ui.add(InfoBadge::value(12).kind(InfoBadgeKind::Attention));
ui.add(PipsPager::new(&mut page, 8).arrows(true));
```

`evaluate_number_expression` 支持括号、正负号、`+`、`-`、`*`、`/` 和乘方，返回 `Result<f64, String>`。

### 面包屑、标签页、用户头像与教学提示

`BreadcrumbBar::show` 返回被点击层级的索引。`TabViewResponse` 提供选择、关闭、新增和重新排序请求。`PersonPicture` 显示用户头像或占位符。`TeachingTipResponse` 区分主要操作与关闭操作。

## 文本控件

```rust
let suggestion = AutoSuggestBox::new(&mut query, ["TextBox", "PasswordBox"])
    .hint_text("查找控件").width(260.0).show(ui);
if let Some(chosen) = suggestion.chosen { open_control(chosen); }
ui.add(PasswordBox::new(&mut password).reveal(show_password));
ui.add(RichEditBox::new(&mut body).desired_rows(5).width(360.0));
ui.add(RichTextBlock::new("只读内容").width(360.0));
let response = text_box(ui, &mut simple_text, 240.0);
```

`RichEditBox` 是 egui 多行编辑器的 Fluent 视觉包装器，并不实现 XAML 的 RTF/文档对象模型。`RichTextBlock` 只读。`AutoSuggestResponse::chosen` 包含当前帧选中的建议。

## 日期和时间控件

```rust
let mut date = FluentDate::new(2026, 7, 20);
let mut time = FluentTime::new(20, 30);
CalendarDatePicker::new(&mut date).width(220.0).show(ui);
ui.add(CalendarView::new(&mut date));
ui.add(DatePicker::new(&mut date).width(300.0));
ui.add(TimePicker::new(&mut time).minute_increment(5)
    .clock(TimePickerClock::TwentyFourHour).width(220.0));
```

`FluentDate` 和 `FluentTime` 是平台无关的值类型。`TimePickerClock` 支持 12 小时制和 24 小时制。日期与时间控件会直接修改借用的值。

## 集合与滚动控件

### 列表、网格与项目视图

`ListBox`、`ListView`、`GridView` 和 `ItemsView` 使用 `CollectionItem` 数据。`ItemsView` 与 `GridView` 具有相同的主要选择和磁贴配置。多选或扩展选择需通过 `selected_indices` 传入可变索引向量，并选择相应的 `GridSelectionMode`。

### 重复器与树

`ItemsRepeater::show` 使用默认垂直布局；`show_with_layout` 接受 `VerticalStack`、`HorizontalStack` 或 `UniformGrid`。`TreeView` 使用 `TreeNode::branch` 和 `TreeNode::leaf` 构造层级，并通过 `selected_paths` 管理选择路径。

### 翻页、滚动、缩放、选择与滑动操作

`FlipView` 显示按索引切换的页面；`ScrollViewer`/`ScrollView` 提供滚动区域；`PullToRefreshResponse::refresh_requested` 表示刷新请求；`SemanticZoom` 在分组和详情之间切换；`SelectorBar` 与 `Pivot` 提供区段选择；`SwipeControlResponse::action_clicked` 表示滑动操作被执行。

`AnnotatedScrollBar` 绑定归一化的 `0.0..=1.0` 值，可通过 `AnnotatedScrollBarLabel`、标记位置和 `detail_label` 格式化闭包添加注释。

## 命令、菜单与导航

### 标准命令与自定义命令

`StandardUICommand` 使用 `StandardUICommandKind` 中的标准命令目录。`XamlUICommand` 保存应用自定义的标签、说明、图标和命令元数据，但不依赖 XAML。`CommandListResponse::invoked` 返回被调用项目的索引。

### 菜单定义

`MenuDefinition` 和 `MenuItem` 定义菜单结构；`MenuBar::show` 返回调用的项目。`MenuFlyoutResponse::selected` 返回浮出菜单的选择。可使用 `CommandBar`、`CommandBarFlyout`、`AppBarButton`、`AppBarToggleButton` 和 `AppBarSeparator` 组合命令界面；`FluentPopup` 是这些控件使用的底层弹出容器。

### NavigationView 与 SplitView

`NavigationView` 绑定选中页面与 `NavigationViewItem` 集合。`SplitView` 可配置窗格位置和显示模式，其响应分别通过 `pane` 与 `content` 暴露窗格和内容结果。`TitleBar` 绘制应用标题栏内容；各响应结构会分别暴露窗格、内容和命令操作。

## 布局、绘制与画刷

### 面板与容器

`FluentBorder` 支持填充、描边、圆角和内边距。`StackPanel` 按指定 `Orientation`、间距与对齐方式排列内容。`GridPanel` 创建固定列数的网格。`Canvas` 提供指定尺寸和可选裁剪的绘制区；`RelativePanel` 按坐标放置控件；`VariableSizedWrapGrid` 自动换行；`Viewbox` 在固定范围内显示预览。

`compact_sizing(ui, body)` 会在闭包执行期间临时启用紧凑间距，适合信息密集的工具界面。

### 绘制元素

`ShapeElement`、`LineElement`、`FluentImage` 和 `IconElement` 分别用于形状、线条、图像和图标绘制。`RadialGradientBrush` 与 `AcrylicBrush` 是基于 egui painter 的内容表面近似效果，与 Windows DWM 系统背景并不是同一种机制。

## 动效与过渡

`EasingFunction` 提供缓动采样；`AnimatedIcon` 和 `AnimatedVisualPlayer` 驱动图标及自定义帧动画。页面与状态过渡包括 `PageTransition`、`ImplicitTransition`、`ConnectedAnimation` 和 `ParallaxView`。

`ThemeShadow` 包裹具有层级感的内容。`ThemeTransition` 返回主题变化的插值值；`transition_color(light, dark, t)` 在不重复预乘 Alpha 的前提下插值颜色。

`XamlCompInterop` 和 `CompositionInteropStatus` 会明确报告外部 Composition visual 是否已托管，或当前是否使用 egui 回退；该 crate 不声称提供 XAML Composition 运行时。

## Gallery 与设计辅助工具

`GalleryPageHeader` 显示页面标题并返回复制链接响应；`GalleryExample` 展示带说明的控件示例；`GallerySourceExpander` 显示可展开的源代码。

其他 Gallery 辅助组件：

- `GalleryTopicCard`：全宽导航入口；
- `GalleryCatalogItem` 和 `GalleryCatalogGrid`：响应式控件目录；
- `GalleryImageHotspot` 和 `GalleryAnnotatedImage`：带标注的参考图像；
- `GallerySettings` 和 `GallerySettingsResponse`：完整的 Gallery 设置界面；
- `gallery_example_spacing`：示例之间的标准间距。

`GallerySettings` 是适合 Gallery 风格应用的现成设置页。可检查 `theme_changed`、`clear_recents` 和 `remove_favorites` 等响应字段。设计参考控件可直接调用：`ResourceScopePreview::show`、`CornerRadiusTable`、`SpacingRamp::show`、`TypographyRamp::show` 和 `ColorGuidance`。

## 基础示例控件

这些控件演示如何将应用级组合封装为可复用控件：`CounterControl`、`ValidatedPasswordBox`、`validated_submit_button` 和 `TemperatureConverterControl`。`ValidatedPasswordResponse` 包含输入响应和有效性；`TemperatureConverterResponse` 报告输入及转换命令状态。

## 图标

常用的强类型图标使用 `FluentIcon`，完整的内置目录可通过 `fluent_icon_catalog()` 搜索。每个 `FluentIconEntry` 提供 `name`、`code`、`tags`、`glyph` 和 `is_segoe_fluent_only` 字段，以及 `matches`、`code_glyph` 和 `text_glyph` 辅助方法。

Windows 10/11 优先使用已安装的 Segoe UI、Microsoft YaHei、Segoe MDL2 Assets 和 Segoe Fluent Icons 字体；其他平台使用内置的跨平台 Fluent System Icons 字体。

## 平台界面与请求

### 能力检测

```rust
let status = detect_platform_capability(PlatformCapability::WebView2);
if status.is_available() {
    start_webview_service();
} else {
    ui.label(status.label());
}
```

`PlatformStatus` 区分 `Available`、`Fallback`、`Unsupported` 和 `Failed(message)`。现代 Windows 上启用 `windows-platform` 可使用 windows-rs 类型检测；Windows 7 构建中不要启用该功能。

### 平台 UI 界面

即使宿主服务不可用，以下控件仍会渲染完整的 Fluent UI 表面：

- `WebView2` 与 `CaptureElement`：通过 `show_surface(ui, body)` 承载调用方提供的后备内容；
- `MapControl`：地图占位/表面 API；
- `MediaPlayerElement` 和 `SoundControl`：媒体播放与音量界面；
- `StoragePickers`、`AppNotification`、`BadgeNotificationManager` 和 `JumpList`；
- `AppWindow`、`AppWindowTitleBar`、`ClipboardControl` 和 `ColorPicker`；
- `ImagePlaceholder`、`PlaybackStatus`、`MultipleWindows` 和 `ContentIsland`。

该 crate 不会在后台静默执行特权系统操作。权限、异步任务、媒体解码器、Web 引擎、文件对话框、通知和操作系统服务集成都由宿主管理。

### 存储选择器请求对象

`StoragePickerRequest::open_file()`、`save_file()`、`open_files()` 和 `pick_folder()` 用于创建平台中立请求，再交给宿主适配器。可设置文件筛选器、提交按钮文字、起始位置、视图模式、默认扩展名、建议文件名和目录。

`StoragePickerKind` 描述操作类型。`StoragePickerRequest` 只传递配置，本身绝不会阻塞 egui 或打开原生对话框。

## 跨平台启动

### Web：Chromium 80+

启用 `web` 并针对 wasm 构建：

```bash
rustup target add wasm32-unknown-unknown
cargo build --release \
  --target wasm32-unknown-unknown \
  --no-default-features \
  --features web
```

在 `HtmlCanvasElement` 上调用 `start_web` 并长期保留返回的 runner。`web_options()` 返回 Glow/WebGL 2 选项。示例见 `examples/web.rs` 和 `examples/web-app/index.html`。应通过 HTTP 提供生成文件，不要使用 `file://` 直接打开。

### Android 7+

启用 `android-native-activity`，并从 `android_main` 调用 `winui::run_android`。宿主需要定制启动前的 `NativeOptions` 时可使用 `android_options(title, app)`。将 `minSdkVersion` 设为 24。`examples/android.rs` 导出 Rust 入口，`examples/android-app` 提供最小 Gradle NativeActivity 宿主。

发行 APK 面向 arm64，使用调试密钥签名：

```bash
adb install fluent-egui-demo-android7-arm64.apk
```

### Linux 桌面环境

```bash
cargo run --release --example demo \
  --no-default-features \
  --features glow,x11,wayland,accesskit
```

该配置面向当前 GNOME 和 KDE 的 X11/Wayland 会话，以及常见的 Budgie、XFCE 和 MATE X11 会话。目标系统仍需合适的图形驱动和窗口系统运行库。

### Windows 7

```bash
cargo build --release --example demo \
  --no-default-features \
  --features glow,win7
```

不要启用 `windows-platform`。常规 MSVC 编译成功并不保证 Rust 标准库本身支持 Windows 7；发布构件需使用兼容 Win7 的目标/工具链。Mica 和现代 Acrylic 不可用，会回退至 Aero 或不透明 Fluent 表面。

### iOS 12+

```bash
rustup target add aarch64-apple-ios
cargo check \
  --target aarch64-apple-ios \
  --no-default-features \
  --features ios
```

控件和主题库可通过 WGPU/Metal 编译。生命周期集成、Metal 表面、`.app` 包、签名和配置文件均由 iOS 宿主管理。通用的未签名目标不是可安装的 IPA。

## Demo 发布构件

本地运行完整 Gallery：

```bash
cargo run --release --example demo
```

手动触发 `CI and cross-platform demos` 工作流后，所有质量和平台任务成功时会发布稳定 GitHub Release。

| 构件 | 用途 |
|---|---|
| `fluent-egui-demo-windows-x64.exe` | Windows 10/11 Gallery |
| `fluent-egui-demo-windows7-profile-x64.exe` | Windows 7 功能配置 Gallery |
| `fluent-egui-demo-linux-x64` 和 `.tar.gz` | Linux x64 Gallery |
| `fluent-egui-demo-web.zip` | 可直接用于浏览器的 wasm-bindgen 包 |
| `fluent-egui-demo-web-raw.wasm` | 未打包的原始 wasm 目标 |
| `fluent-egui-demo-android7-arm64.apk` | 可安装的 Android 7+ arm64 调试 APK |
| `fluent-egui-demo-android7-arm64-release.so` | 优化后的 Android NativeActivity 库 |
| `fluent-egui-demo-ios12-arm64-unsigned` 和 `.tar.gz` | 未签名 iOS 目标 |

## 故障排查

### 看不到 Mica 或 Acrylic

系统背景取决于 Windows 版本、透明效果设置、DWM 合成、远程桌面状态和图形驱动。该 crate 可以请求原生材质，但无法在不支持的系统上模拟真正的 DWM Mica。

### Windows 之外的平台中文字或图标不同

Windows 优先使用已安装的 Segoe 字体。其他平台使用内置 Fluent System Icons 和 egui 的后备文本字体。图标含义保持不变，但轮廓和字体度量可能略有差异。

### 混用 fluent-egui 与 egui 原生控件

这种方式受到支持。安装主题会修改全局 egui `Style` 和 `Visuals`，因此原生 egui 控件也会继承调色板。需要精确的 Fluent 尺寸、基线对齐、动画或交互语义时，请使用 fluent-egui 构建器。

### 双击无法运行 Web 文件

浏览器通常会阻止从 `file://` 加载模块和 WebAssembly。解压 Web ZIP 后通过 HTTP 提供文件：

```bash
python -m http.server 8080
```

### Android APK 无法覆盖旧版本

GitHub runner 会生成临时调试签名密钥。签名不一致时，请先卸载旧包，再安装新 APK。

### Android APK 可以直接发布到应用商店吗？

不可以。它只是测试用 Gallery。生产应用需要自己的包 ID、图标、权限、版本信息、发布构建策略和长期使用的签名密钥。

## 完整公共 API 一览

该 crate 在 `fluent_egui::winui` 下重新导出面向使用者的 API：

- 主题：`FluentTheme`、`ThemeConfig`、`FluentPalette`、`install`、`apply_theme` 及调色板/Frame 辅助函数；
- 外壳/背景：`RenderBackend`、原生和 Android 启动函数、`BackdropKind`、`BackdropStatus`；
- 简洁页面组合：`FluentUi`、`WinUiExt`；
- 核心控件：按钮、文本编辑、开关、三态复选框、分段控件、滑块、进度、浮出菜单、设置、对话框、InfoBar 和 Toast API；
- 扩展控件：超链接、切换/拆分/下拉/重复按钮、评分、数字、徽章、分页器、面包屑、标签页、用户头像和教学提示 API；
- 文本及日期/时间：对应专门章节中介绍的全部类型；
- 集合：`CollectionItem`、列表/网格/项目/树/翻页/滚动/缩放/选择/透视/滑动 API；
- 命令：标准/自定义命令、菜单、命令栏、应用栏、导航、拆分视图、标题栏和弹出 API；
- 布局：边框、画布、网格、堆叠、相对面板、换行网格、Viewbox、形状、线条、图像、图标和画刷 API；
- 动效：缓动、动画视觉、页面/隐式/连接过渡、视差、阴影、主题过渡和 Composition 互操作；
- Gallery/设计：页面、示例、源代码、目录、标注、设置、图标目录、颜色、资源、圆角、间距和字体排印辅助工具；
- 平台：能力检测、UI 表面、选择器请求、通知、剪贴板、颜色、图像和播放状态。

每个公共项目均有 rustdoc 文档和 `# Examples` 章节。精确签名与字段级细节请查阅 docs.rs；应用层调用方式和平台边界请以本指南为参考。

## 许可证

`fluent-egui` 采用 MIT 许可证。内置的 Microsoft Fluent System Icons 字体依据 `src/assets/FLUENT_SYSTEM_ICONS_LICENSE.txt` 中附带的 MIT 许可证分发。

WinUI、Fluent、Windows 及相关名称归各自所有者所有。本项目不是 Microsoft 官方产品。
