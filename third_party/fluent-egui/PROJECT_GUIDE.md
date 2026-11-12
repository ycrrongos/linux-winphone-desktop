# fluent-egui User Guide and API Cookbook

`fluent-egui` is a WinUI 3 and Fluent 2 inspired theme and widget library for `egui`/`eframe`. It provides light and dark themes, automatic layouts, animated controls, Fluent icons, Gallery composition helpers, and optional native Windows backdrops without requiring XAML.

This document is written for application developers consuming the crate. It explains how state flows through the public API, how each public API family is called, which responses to inspect, and which platform features to enable. The generated rustdoc remains the exact reference for every field and method.

Unless a snippet shows imports explicitly, it assumes `use fluent_egui::winui::{self, *};`, access to an `egui::Ui`, and application-owned state with the displayed variable names. Short helper functions such as `save()` or `draw_page()` stand for application code.

`fluent-egui` is not an official Microsoft or WinUI project.

## Contents

- [Installation and features](#installation-and-features)
- [Application model](#application-model)
- [Desktop startup](#desktop-startup)
- [Themes and native backdrops](#themes-and-native-backdrops)
- [High-level composition API](#high-level-composition-api)
- [Core widgets](#core-widgets)
- [Extended input and navigation controls](#extended-input-and-navigation-controls)
- [Text controls](#text-controls)
- [Date and time controls](#date-and-time-controls)
- [Collection and scrolling controls](#collection-and-scrolling-controls)
- [Commands, menus, and navigation](#commands-menus-and-navigation)
- [Layout, drawing, and brushes](#layout-drawing-and-brushes)
- [Motion and transitions](#motion-and-transitions)
- [Gallery and design helpers](#gallery-and-design-helpers)
- [Icons](#icons)
- [Platform surfaces and requests](#platform-surfaces-and-requests)
- [Web, Android, Linux, Windows 7, and iOS](#cross-platform-startup)
- [Demo releases](#demo-releases)

## Installation and features

Add the crate to your application:

```toml
[dependencies]
fluent-egui = "0.1"
```

The default feature set uses Glow and enables both X11 and Wayland support:

```toml
fluent-egui = {
    version = "0.1",
    features = ["glow", "x11", "wayland", "accesskit"]
}
```

Use the backend-independent theme and widget surface when your application owns the eframe shell:

```toml
fluent-egui = { version = "0.1", default-features = false }
```

| Feature | Default | Purpose |
|---|---:|---|
| `glow` | yes | Glow/OpenGL renderer and screenshot support |
| `wgpu` | no | WGPU renderer and screenshot support |
| `x11` | yes | Linux X11 integration |
| `wayland` | yes | Linux Wayland integration |
| `accesskit` | yes | Native accessibility semantics |
| `web` | no | WebAssembly startup with Glow/WebGL 2 |
| `android-native-activity` | no | Android NativeActivity entry point |
| `android-game-activity` | no | Android GameActivity entry point |
| `ios` | no | iOS WGPU/Metal dependency set |
| `win7` | no | Legacy Windows build profile without modern WinRT adapters |
| `windows-platform` | no | Optional windows-rs capability detection |

## Application model

The crate follows egui's immediate-mode state model:

- your application owns values such as `bool`, `String`, `usize`, dates, selected rows, and dialog visibility;
- widgets borrow that state for one frame;
- a widget returns `egui::Response` or a typed response when it has more than one action;
- rendering is driven by `eframe::App::ui`;
- the library owns visual defaults and animation state stored in the egui context.

Use standard `Response` methods for ordinary controls:

```rust
let response = ui.add(FluentButton::accent("Save"));

if response.clicked() {
    save();
}
if response.hovered() {
    // Optional application behavior.
}
```

Typed responses separate distinct actions. For example, `SettingsChoiceResponse` distinguishes selecting an item from navigating into it, and `ContentDialogResponse` identifies the button that closed the dialog.

## Desktop startup

The smallest complete desktop application is:

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
            ui.page_header("Account", "Manage your preferences");
            ui.card("Profile", "Personal information", |ui| {
                ui.text_box_hint(&mut self.name, "User name");
                ui.toggle(&mut self.notifications, "Notifications");
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

### Native shell functions

| API | Call pattern | Result |
|---|---|---|
| `native_options(title)` | build default transparent options | `eframe::NativeOptions` |
| `native_options_with_backend(title, backend)` | choose Glow or WGPU | `eframe::NativeOptions` |
| `run_native(title, config, creator)` | start with the default backend | `eframe::Result` |
| `run_native_with_backend(...)` | start with `RenderBackend::Glow` or `Wgpu` | `eframe::Result` |
| `apply_runtime_theme(ctx, frame, config)` | update egui visuals and the native backdrop | `BackdropStatus` |

When both renderer features are enabled, `RenderBackend::default()` is Glow. Select WGPU explicitly:

```rust
winui::run_native_with_backend(
    "WGPU app",
    ThemeConfig::new(FluentTheme::Dark),
    RenderBackend::Wgpu,
    |_| App::default(),
)
```

## Themes and native backdrops

### Theme configuration

`FluentTheme` contains `Light` and `Dark`. `ThemeConfig` controls the theme, accent color, material opacity, and animations:

```rust
let config = ThemeConfig::new(FluentTheme::Dark)
    .with_accent(egui::Color32::from_rgb(96, 205, 255))
    .with_material_opacity(0.72)
    .with_animations(true);
```

Use the theme APIs as follows:

```rust
// Initial font and style installation.
winui::install(&cc.egui_ctx, config);

// Change only egui visuals at runtime.
winui::apply_theme(ctx, config);

// Obtain colors for custom painting.
let colors: FluentPalette = winui::palette_with_config(config);

// Draw standard material surfaces.
winui::content_frame_with_config(config).show(ui, |ui| { /* page */ });
winui::surface_frame_with_config(config).show(ui, |ui| { /* toolbar */ });
winui::card_frame_with_config(config).show(ui, |ui| { /* card */ });
```

Other theme helpers are `default_accent`, `accent_presets`, `palette`, `content_frame`, `surface_frame`, and `card_frame`.

### Backdrop kinds

`BackdropKind` supports `Auto`, `Mica`, `Acrylic`, `Tabbed`, `Aero`, and `None`:

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
        BackdropStatus::Failed(code) => eprintln!("DWM error: {code}"),
    }
}
```

Mica, system Acrylic, and Tabbed/Alt Mica are Windows 11 capabilities. Windows 10 and Windows 7 fall back to legacy blur or Aero when available. Non-Windows targets return `Unsupported`; Fluent surfaces remain readable without a system backdrop.

## High-level composition API

Import the prelude and enter a `WinUiExt` scope:

```rust
use fluent_egui::winui::prelude::*;

ui.winui(|ui| {
    ui.button("Default");
    ui.accent_button("Primary");
});
```

`winui()` infers the theme from installed egui visuals. Use `winui_with_theme(theme, ...)` for an isolated preview.

### FluentUi method reference

| Method | State/input | Return |
|---|---|---|
| `button(text)` | command label | `Response` |
| `accent_button(text)` | primary command label | `Response` |
| `subtle_button(text)` | low-emphasis command | `Response` |
| `icon_button(icon)` | `FluentIcon` | `Response` |
| `button_with(text, configure)` | full `FluentButton` builder | `Response` |
| `text_box(text)` | `&mut TextBuffer` | `Response` |
| `text_box_hint(text, hint)` | text and placeholder | `Response` |
| `text_box_with(text, configure)` | full `FluentTextEdit` builder | `Response` |
| `password_box(text)` | `&mut String` | `Response` |
| `toggle(value, label)` | `&mut bool` | `Response` |
| `slider(value, range)` | `&mut f32` | `Response` |
| `slider_with(value, range, configure)` | full slider builder | `Response` |
| `progress(value)` | normalized `f32` | `Response` |
| `segmented(selected, labels)` | `&mut usize` | `Response` |
| `select(id, selected, values)` | selection and string values | `Response` |
| `row`, `column`, `wrap` | layout closure | closure result |
| `card(title, subtitle, body)` | card content closure | `InnerResponse` |
| `example(title, body)` | Gallery example block | `InnerResponse` |
| `setting(...)` | Settings-style row | `Response` |
| `page_header(title, description)` | page heading | `GalleryPageHeaderResponse` |
| `control(widget)` | any `egui::Widget` | `Response` |

`FluentUi` implements `Deref<Target = egui::Ui>`, so labels, separators, scrolling, grids, and all native egui methods remain available inside the scope.

Customize without abandoning the concise API:

```rust
ui.winui(|ui| {
    ui.button_with("Install", |button| {
        button.icon(FluentIcon::Download).min_width(160.0)
    });

    ui.text_box_with(&mut query, |edit| {
        edit.hint_text("Search").char_limit(128)
    });

    ui.slider_with(&mut volume, 0.0..=100.0, |slider| {
        slider.width(320.0).step_by(5.0).show_value(true)
    });

    ui.control(RatingControl::new(&mut rating).max(10));
});
```

## Core widgets

### Buttons

```rust
ui.add(FluentButton::new("Cancel"));
ui.add(FluentButton::accent("Save").min_width(100.0));
ui.add(FluentButton::subtle("Add").icon(FluentIcon::Add));
ui.add(AccentButton::new("Save").icon(FluentIcon::Check));
ui.add(IconButton::new(FluentIcon::Download).selected(false))
    .on_hover_text("Download");
```

`FluentButton` builder methods are `icon`, `kind`, and `min_width`. `ButtonKind` is `Standard`, `Accent`, or `Subtle`. `AccentButton` is a convenience wrapper. `IconButton` uses a fixed command target and supports `selected(bool)`.

Use `FluentContentButton` when the button body is not plain text:

```rust
let response = FluentContentButton::new(egui::vec2(180.0, 64.0), "Open profile")
    .kind(ButtonKind::Standard)
    .show(ui, |ui| {
        ui.vertical(|ui| {
            ui.strong("Profile");
            ui.label("Account settings");
        });
    });

if response.response.clicked() {
    open_profile();
}
```

### Text entry

```rust
ui.add(
    FluentTextEdit::new(&mut text)
        .hint_text("Search")
        .password(false)
        .char_limit(128),
);

ui.add(fluent_text_edit(&mut text).hint_text("Search"));
```

`FluentTextEdit` owns no text; it borrows an egui `TextBuffer`. It aligns text, placeholder, and caret to Fluent metrics and returns standard `Response` change/focus information.

### Toggle, tri-state, segment, and slider

```rust
ui.add(ToggleSwitch::new(&mut enabled).label("On"));

let mut tri_state = Some(true);
ui.add(TriStateCheckBox::new(&mut tri_state, "Optional feature"));

ui.add(
    SegmentedControl::new(&mut selected, ["Preview", "Code", "Details"])
        .width(300.0),
);

ui.add(
    FluentSlider::new(&mut volume, 0.0..=100.0)
        .width(240.0)
        .step_by(5.0)
        .show_value(true),
);

ui.add(FluentSlider::new(&mut gain, 0.0..=1.0).vertical(180.0));
```

Each control mutates borrowed state and reports changes through `Response::changed()`.

### Progress

```rust
ui.add(
    FluentProgressBar::new(0.68)
        .width(220.0)
        .show_percentage(true),
);

ui.add(ProgressRing::new(Some(0.68)).size(28.0));
ui.add(ProgressRing::new(None).size(32.0));
```

`ProgressRing::new(Some(value))` is determinate. `None` creates an animated indeterminate ring.

### Combo box helpers

```rust
fluent_combo_box("quality", qualities[selected])
    .width(180.0)
    .show_ui(ui, |ui| {
        for (index, quality) in qualities.iter().enumerate() {
            if combo_option(ui, *quality, selected == index).clicked() {
                selected = index;
                ui.close();
            }
        }
    });
```

`fluent_combo_box` returns egui's `ComboBox` builder with Fluent sizing and chevron. `combo_option` draws a stable-height menu row and left selection indicator.

### Flyouts and Settings rows

```rust
FlyoutButton::new("Speakers", "USB Audio Device")
    .icon(FluentIcon::Volume)
    .width(ui.available_width())
    .open(flyout_open)
    .show(ui, |ui| {
        ui.add(
            FlyoutOption::new("Speakers", "USB Audio Device")
                .icon(FluentIcon::Volume)
                .selected(true),
        );
        ui.add(
            FlyoutOption::new("Device properties", "Format and spatial audio")
                .icon(FluentIcon::Settings)
                .navigates(true),
        );
    });
```

For an in-place expanding Settings card:

```rust
let result = SettingsExpander::new(
    "output-devices",
    &mut expanded,
    "Choose where to play sound",
    "Apps may have their own settings",
)
.icon(FluentIcon::Volume)
.trailing("USB Audio")
.show(ui, |ui| {
    let row = SettingsChoiceRow::new("Speakers", "USB Audio Device")
        .status("Enabled")
        .selected(selected_device == 0)
        .navigates(true)
        .separator(true)
        .show(ui);

    if row.navigation_clicked() {
        open_device_properties();
    } else if row.response.clicked() {
        selected_device = 0;
    }

    settings_footer(ui, "Pair a new device", |ui| {
        ui.add(AccentButton::new("Add device"));
    });
});

let _header = result.header_response;
let _body = result.body;
```

`SettingsNavigationRow::show` returns a single navigation response. Use `show_with_trailing` for a switch or selector:

```rust
SettingsNavigationRow::new(
    "Night light",
    "Use warmer colors",
    FluentIcon::Sun,
)
.trailing_width(76.0)
.navigates(false)
.show_with_trailing(ui, |ui| {
    ui.add(ToggleSwitch::new(&mut night_light));
});
```

### Dialogs, InfoBar, and Toast

```rust
let dialog = ContentDialog::new(
    "save-dialog",
    "Confirm action",
    "Save the current preferences?",
)
.primary_text("Save")
.secondary_text("Don't save")
.close_text("Cancel")
.width(380.0)
.show(ctx, theme);

match dialog.action {
    Some(ContentDialogAction::Primary) => save(),
    Some(ContentDialogAction::Secondary) => discard(),
    Some(ContentDialogAction::Close) | None => {}
}

let close = InfoBar::new("Saved", "Preferences were updated")
    .kind(InfoBarKind::Success)
    .icon_visible(true)
    .closable(true)
    .show(ui);

if close.clicked() {
    show_info = false;
}

Toast::new("Saved", "Your changes are ready")
    .icon(FluentIcon::Check)
    .show(ctx, theme, toast_visible);
```

### Composition helper functions

```rust
settings_group(ui, |ui| {
    setting_row(ui, theme, "Notifications", "Show app alerts", |ui| {
        ui.add(ToggleSwitch::new(&mut notifications));
    });
});

section_card(ui, theme, "Input", "Text and values", |ui| {
    ui.add(FluentTextEdit::new(&mut text));
});

card_columns(ui, |columns| {
    section_card(&mut columns[0], theme, "Left", "", |_| {});
    section_card(&mut columns[1], theme, "Right", "", |_| {});
});

animated_page(ui, "page", revision, |ui| {
    ui.heading("Animated content");
});
```

Other navigation helpers are `navigation_item`, `navigation_child_item`, `navigation_branch_item`, and `NavigationGroup`.

## Extended input and navigation controls

### Command-like controls

```rust
ui.add(HyperlinkButton::new("Open documentation"));
ui.add(ToggleButton::new(&mut pinned, "Pinned"));
ui.add(RepeatButton::new("Increase").repeats_per_second(8.0));
```

`RepeatButton` emits once for a short click and begins repeating only after its hold delay.

### Split and drop-down buttons

```rust
let split = SplitButton::new("Save")
    .icon(FluentIcon::Save)
    .width(140.0)
    .show(ui, |ui| {
        if combo_option(ui, "Save as", false).clicked() {
            ui.close();
        }
    });

if split.response.clicked() {
    save();
}
if split.menu_response.clicked() {
    // Popup state changed.
}

DropDownButton::new("Export")
    .icon(FluentIcon::Upload)
    .width(140.0)
    .open(menu_open)
    .show(ui, |ui| {
        ui.button("PNG");
        ui.button("PDF");
    });
```

`ColorSplitButton` binds a selected `Color32` and a palette, returning a `ColorSplitButtonResponse` for primary and color-menu actions. `ToggleSplitButton` combines a mutable toggle value with a split menu.

### Rating, NumberBox, badges, and pager

```rust
ui.add(
    RatingControl::new(&mut rating)
        .max(10)
        .allow_clear(true)
        .read_only(false),
);

ui.add(
    NumberBox::new(&mut amount)
        .range(0.0..=100.0)
        .step(0.5)
        .width(160.0)
        .spin_button_placement(NumberBoxSpinButtonPlacement::Compact),
);

let result = evaluate_number_expression("(12 + 3) * 2")?;

ui.add(InfoBadge::dot());
ui.add(InfoBadge::value(12).kind(InfoBadgeKind::Attention));
ui.add(PipsPager::new(&mut page, 8).arrows(true));
```

`evaluate_number_expression` supports parentheses, unary signs, `+`, `-`, `*`, `/`, and exponentiation, returning `Result<f64, String>`.

### Breadcrumbs, tabs, people, and teaching tips

```rust
if let Some(index) = BreadcrumbBar::new(["Home", "Settings", "Display"]).show(ui) {
    navigate_to(index);
}

let tabs = vec!["Home".to_owned(), "Output".to_owned()];
let response = TabView::new(&mut selected_tab, &tabs)
    .closable(true)
    .add_button(true)
    .width_mode(TabViewWidthMode::Equal)
    .close_button_overlay_mode(TabViewCloseButtonOverlayMode::OnHover)
    .reorderable(true)
    .show(ui);

if let Some(index) = response.close_requested {
    close_tab(index);
}
if response.add_clicked {
    add_tab();
}

ui.add(PersonPicture::new("Ada Lovelace").size(48.0).profile_placeholder(true));

let tip = TeachingTip::new("tip", "Keyboard shortcuts", "Use Ctrl+S to save")
    .primary_text("Got it")
    .close_text("Close")
    .show(ctx, theme, tip_open);
```

Inspect `TabViewResponse` for selection, close, add, and reorder requests. Inspect `TeachingTipResponse` for primary and close actions.

## Text controls

```rust
let suggestion = AutoSuggestBox::new(&mut query, ["TextBox", "PasswordBox"])
    .hint_text("Find a control")
    .width(260.0)
    .show(ui);

if let Some(chosen) = suggestion.chosen {
    open_control(chosen);
}

ui.add(PasswordBox::new(&mut password).reveal(show_password));
ui.add(RichEditBox::new(&mut body).desired_rows(5).width(360.0));
ui.add(RichTextBlock::new("Read-only content").width(360.0));

let response = text_box(ui, &mut simple_text, 240.0);
```

`RichEditBox` is a Fluent visual wrapper around egui multiline editing. It does not implement XAML's RTF/document object model. `RichTextBlock` is read-only. `AutoSuggestResponse::chosen` contains the selected suggestion for the current frame.

## Date and time controls

```rust
let mut date = FluentDate::new(2026, 7, 20);
let mut time = FluentTime::new(20, 30);

CalendarDatePicker::new(&mut date)
    .width(220.0)
    .show(ui);

ui.add(CalendarView::new(&mut date));
ui.add(DatePicker::new(&mut date).width(300.0));
ui.add(
    TimePicker::new(&mut time)
        .minute_increment(5)
        .clock(TimePickerClock::TwentyFourHour)
        .width(220.0),
);
```

`FluentDate` and `FluentTime` are platform-independent value types. `TimePickerClock` supports 12-hour and 24-hour modes. The date and time controls mutate the borrowed values directly.

## Collection and scrolling controls

### List, grid, and items views

```rust
let items = vec![
    CollectionItem::new("Keyboard").subtitle("Connected"),
    CollectionItem::new("Mouse").subtitle("Connected"),
];

ListBox::new(&mut selected, &items).width(320.0).show(ui);
ListView::new(&mut selected, &items).max_height(280.0).show(ui);

GridView::new(&mut selected, &items)
    .tile_size(egui::vec2(180.0, 88.0))
    .selection_mode(GridSelectionMode::Single)
    .spacing(egui::vec2(8.0, 8.0))
    .max_columns(4)
    .show(ui);
```

`ItemsView` has the same main selection and tile configuration as `GridView`. For multiple or extended selection, pass a mutable index vector through `selected_indices` and select the matching `GridSelectionMode`.

### Repeater and tree

```rust
ItemsRepeater::show_with_layout(
    ui,
    &items,
    ItemsRepeaterLayout::UniformGrid { columns: 3 },
    |ui, item| {
        ui.label(&item.title);
    },
);

let nodes = vec![
    TreeNode::branch(
        "Devices",
        vec![TreeNode::leaf("Keyboard"), TreeNode::leaf("Mouse")],
    ),
];

TreeView::new(&nodes)
    .selection_mode(TreeSelectionMode::Single)
    .selected_paths(&mut selected_paths)
    .template_icons(true)
    .show(ui);
```

`ItemsRepeater::show` uses a default vertical layout. `show_with_layout` accepts `VerticalStack`, `HorizontalStack`, or `UniformGrid`.

### Flip, scroll, zoom, selector, and swipe

```rust
FlipView::new(&mut slide, slide_count)
    .size(egui::vec2(480.0, 270.0))
    .show(ui, |ui, index| draw_slide(ui, index));

ScrollViewer::vertical(360.0).show(ui, |ui| {
    draw_long_content(ui);
});

let refresh = PullToRefresh::show(ui, |ui| draw_feed(ui));
if refresh.refresh_requested {
    reload();
}

ui.add(SemanticZoom::new(&mut zoomed_out));
if zoomed_out {
    draw_groups(ui);
} else {
    draw_detail(ui);
}

SelectorBar::new(&mut section, ["Overview", "Details"]).width(320.0).show(ui);
Pivot::new(&mut section, ["Overview", "Details"]).width(320.0).show(ui);

let swipe = SwipeControl::new("Archive message", "Archive")
    .reveal(reveal)
    .show(ui);
if swipe.action_clicked {
    archive();
}
```

Other collection APIs are `AnnotatedScrollBar`, `AnnotatedScrollBarLabel`, `ScrollView` (an alias of `ScrollViewer`), and `ItemsRepeaterLayout`.

An annotated scrollbar binds a normalized `0.0..=1.0` value:

```rust
let markers = [0.10, 0.35, 0.75];
let labels = [
    AnnotatedScrollBarLabel::new("A", 0.10),
    AnnotatedScrollBarLabel::new("B", 0.35),
    AnnotatedScrollBarLabel::new("C", 0.75),
];
let format_value = |value: f32| format!("{:.0}%", value * 100.0);

ui.add(
    AnnotatedScrollBar::new(&mut scroll_value, &markers)
        .height(240.0)
        .labels(&labels)
        .detail_label(&format_value),
);
```

## Commands, menus, and navigation

### Standard and custom commands

```rust
ui.add(StandardUICommand::new(StandardUICommandKind::Copy));
let custom = XamlUICommand::new("Export")
    .icon(FluentIcon::Upload)
    .description("Export the current document");
ui.add(custom);

let items = vec!["First item".to_owned(), "Second item".to_owned()];
let response = CommandList::new(
    &items,
    &mut selected_item,
    StandardUICommandKind::Delete,
)
.show(ui);
if let Some(index) = response.invoked {
    delete_item(index);
}
```

`StandardUICommandKind` covers the standard command catalogue. `XamlUICommand` stores application-defined label, description, icon, and command metadata without depending on XAML.

### Menu definitions

```rust
let file = MenuDefinition::new("File", vec![
    MenuItem::new("New").icon(FluentIcon::Add),
    MenuItem::new("Open")
        .icon(FluentIcon::OpenFile)
        .shortcut("Ctrl+O")
        .separator_after(true),
    MenuItem::new("Exit").icon(FluentIcon::Close),
]);

if let Some(invoked) = MenuBar::new(&[file]).show(ui) {
    handle_menu(invoked);
}

let flyout = MenuFlyout::new("Actions", menu_items).show(ui);
if let Some(item) = flyout.selected {
    handle_menu(item);
}
```

Use `CommandBar`, `CommandBarFlyout`, `AppBarButton`, `AppBarToggleButton`, and `AppBarSeparator` to compose command surfaces. `FluentPopup` is the lower-level popup container used by these controls.

### NavigationView and SplitView

```rust
let nav_items = vec![
    NavigationViewItem::new("Home", FluentIcon::Home),
    NavigationViewItem::new("Settings", FluentIcon::Settings),
];

NavigationView::new(&mut selected_page, &nav_items)
    .width(220.0)
    .show(ui);
draw_page(ui, selected_page);

let split = SplitView::new(pane_open)
    .placement(SplitViewPanePlacement::Left)
    .display_mode(SplitViewDisplayMode::CompactOverlay)
    .show(ui, |pane| draw_navigation(pane), |content| draw_content(content));

let _pane_result = split.pane;
let _content_result = split.content;
```

`TitleBar` draws application title-bar content. `ToggleSplitButton` combines a boolean command with a menu. Response structures expose pane, content, and command actions separately.

## Layout, drawing, and brushes

### Panels and containers

```rust
FluentBorder::new()
    .fill(colors.card_bg)
    .stroke(egui::Stroke::new(1.0, colors.card_border))
    .corner_radius(4)
    .margin(egui::Margin::same(12))
    .show(ui, |ui| ui.label("Border content"));

StackPanel::new(Orientation::Horizontal)
    .spacing(8.0)
    .align(egui::Align::Center)
    .show(ui, |ui| {
        ui.button("One");
        ui.button("Two");
    });

GridPanel::new("properties", 2)
    .spacing(egui::vec2(16.0, 8.0))
    .striped(false)
    .show(ui, |ui| draw_properties(ui));
```

Additional panels:

```rust
Canvas::new(egui::vec2(320.0, 180.0))
    .clip(true)
    .show(ui, |painter, rect| {
        painter.circle_filled(rect.center(), 24.0, colors.accent);
    });

RelativePanel::new(egui::vec2(320.0, 160.0)).show(ui, |panel| {
    panel.put(
        egui::vec2(16.0, 16.0),
        egui::vec2(120.0, 32.0),
        FluentButton::new("Positioned"),
    );
});

VariableSizedWrapGrid::new(egui::vec2(120.0, 72.0))
    .spacing(egui::vec2(8.0, 8.0))
    .show(ui, &items, |ui, item| ui.label(item));

Viewbox::new(egui::vec2(320.0, 180.0)).show(ui, draw_preview);
```

`compact_sizing(ui, body)` temporarily installs compact spacing for dense tool surfaces.

### Drawing elements

```rust
ui.add(
    ShapeElement::new(
        ShapeKind::Ellipse,
        egui::vec2(64.0, 64.0),
        colors.accent,
    )
    .stroke(egui::Stroke::new(1.0, colors.card_border)),
);

ui.add(LineElement::new(
    egui::vec2(160.0, 24.0),
    egui::Stroke::new(2.0, colors.accent),
));

ui.add(FluentImage::new(texture_id, egui::vec2(240.0, 160.0)).corner_radius(4));
ui.add(IconElement::new(FluentIcon::Settings).size(20.0).color(colors.text_primary));
```

`RadialGradientBrush` and `AcrylicBrush` are painter-backed approximations for content surfaces:

```rust
ui.add(
    RadialGradientBrush::new(
        egui::vec2(240.0, 160.0),
        colors.accent,
        colors.app_bg,
    )
    .rings(24),
);

AcrylicBrush::new(colors.accent)
    .opacity(0.24)
    .corner_radius(8)
    .show(ui, |ui| ui.label("Acrylic-like surface"));
```

These brushes are egui-rendered surfaces. They are distinct from Windows DWM system backdrops.

## Motion and transitions

```rust
let eased = EasingFunction::CubicOut.sample(progress);

ui.add(AnimatedIcon::new(FluentIcon::Refresh, playing));

AnimatedVisualPlayer::new(&mut progress, &mut playing)
    .duration(2.0)
    .looping(true)
    .show(ui, |painter, rect, progress| {
        draw_animation_frame(painter, rect, progress);
    });
```

Page and state transitions:

```rust
PageTransition::new("settings-page", revision).show(ui, draw_settings);

ImplicitTransition::new("details", details_visible)
    .show(ui, |ui, amount| draw_details(ui, amount));

ConnectedAnimation::new("selected-card", revision).show(ui, draw_card);

ParallaxView::new(scroll_offset)
    .ratio(0.12)
    .show(ui, draw_header);
```

`ThemeShadow` wraps elevated content. `ThemeTransition` returns an interpolation value for theme changes. `transition_color(light, dark, t)` interpolates colors without double-premultiplying alpha.

`XamlCompInterop` and `CompositionInteropStatus` explicitly report whether an external Composition visual is hosted or the egui fallback is active; the crate does not claim to provide a XAML Composition runtime.

## Gallery and design helpers

```rust
let header = GalleryPageHeader::new("Button", "Triggers an action")
    .show(ui);

if header.copy_link.as_ref().is_some_and(|response| response.clicked()) {
    copy_page_link();
}

GalleryExample::new("Default button")
    .description("A standard command")
    .show(ui, |ui| {
        ui.add(FluentButton::new("Button"));
    });

GallerySourceExpander::new("button-source", source)
    .language("rust")
    .show(ui);
```

Other Gallery helpers:

- `GalleryTopicCard`: full-width navigation entry;
- `GalleryCatalogItem` and `GalleryCatalogGrid`: responsive control catalogue;
- `GalleryImageHotspot` and `GalleryAnnotatedImage`: annotated reference images;
- `GallerySettings` and `GallerySettingsResponse`: complete Gallery settings surface;
- `gallery_example_spacing`: standard spacing between examples.

`GallerySettings` is a ready-made settings page for Gallery-style applications:

```rust
let response = GallerySettings::new(
    &mut theme,
    &mut navigation_style,
    &mut sound_expanded,
    sound_available,
    &mut spatial_audio,
    &mut about_expanded,
)
.status_message(status_message.as_str())
.show(ui);

if response.theme_changed {
    winui::apply_theme(ui.ctx(), ThemeConfig::new(theme));
}
if response.clear_recents {
    recent_samples.clear();
}
if response.remove_favorites {
    favorite_samples.clear();
}
```

Design reference widgets are called directly:

```rust
ResourceScopePreview::show(ui);
CornerRadiusTable::new(theme).show(ui);
SpacingRamp::show(ui);
TypographyRamp::show(ui);

let mut color_section = 0;
ColorGuidance::new(&mut color_section, theme).show(ui);
```

## Fundamentals sample controls

These controls demonstrate how to package application-level composites as reusable widgets:

```rust
CounterControl::new(&mut count, CounterMode::Increment).show(ui);

let validation = ValidatedPasswordBox::new(&mut password)
    .min_length(8)
    .width(240.0)
    .reveal(reveal_password)
    .show(ui);

validated_submit_button(ui, validation.is_valid, "Submit");

let conversion = TemperatureConverterControl::new(
    &mut celsius_text,
    &mut fahrenheit_result,
)
.width(200.0)
.show(ui);

if conversion.convert_response.clicked() {
    persist_result();
}
```

`ValidatedPasswordResponse` contains the input response and validity. `TemperatureConverterResponse` reports the input and conversion command state.

## Icons

Use `FluentIcon` for the common strongly typed set:

```rust
ui.add(IconButton::new(FluentIcon::Settings));

ui.label(
    egui::RichText::new(FluentIcon::Globe.glyph())
        .font(icon_font_id(20.0)),
);
```

Search the complete embedded catalogue:

```rust
for icon in fluent_icon_catalog()
    .iter()
    .filter(|entry| entry.matches("wifi"))
{
    ui.label(
        egui::RichText::new(icon.glyph.to_string())
            .font(icon_font_id(20.0)),
    );
}
```

Each `FluentIconEntry` exposes `name`, `code`, `tags`, `glyph`, and `is_segoe_fluent_only`, plus `matches`, `code_glyph`, and `text_glyph` helpers.

Windows 10/11 prefer installed Segoe UI, Microsoft YaHei, Segoe MDL2 Assets, and Segoe Fluent Icons fonts. Other targets use the embedded cross-platform Fluent System Icons font.

## Platform surfaces and requests

### Capability detection

```rust
let status = detect_platform_capability(PlatformCapability::WebView2);

if status.is_available() {
    start_webview_service();
} else {
    ui.label(status.label());
}
```

`PlatformStatus` distinguishes `Available`, `Fallback`, `Unsupported`, and `Failed(message)`. Enable `windows-platform` for windows-rs type detection on modern Windows. Do not enable it in a Windows 7 build.

### Platform UI surfaces

The following controls render a complete Fluent UI surface even when the host service is unavailable:

- `WebView2` and `CaptureElement`: `show_surface(ui, body)` hosts caller-provided fallback content;
- `MapControl`: map placeholder/surface API;
- `MediaPlayerElement::new(&mut playing, &mut position).duration_label(...).show(ui)`;
- `SoundControl::new(&mut volume, &mut muted).show(ui)`;
- `StoragePickers::new().show(ui)`;
- `AppNotification::new(title, message).show(ui)`;
- `BadgeNotificationManager::new(value)`;
- `JumpList::new(&items).show(ui)` returns the selected item index;
- `AppWindow::new().show(ui)` and `AppWindowTitleBar`;
- `ClipboardControl::new(&mut text).show(ui)`;
- `ColorPicker::new(&mut color).alpha(true)`;
- `ImagePlaceholder::new(label, size)`;
- `PlaybackStatus::new(progress)`;
- `MultipleWindows` and `ContentIsland` surfaces.

The crate does not silently perform privileged system operations. The host owns permissions, async tasks, media decoders, web engines, file dialogs, notifications, and OS service integration.

### Storage picker request objects

Build a platform-neutral request and pass it to your host adapter:

```rust
let open = StoragePickerRequest::open_file()
    .file_filter("Images (*.png;*.jpg)")
    .commit_button_text("Open")
    .suggested_start_location(StoragePickerLocation::PicturesLibrary)
    .view_mode(StoragePickerViewMode::Thumbnail);

let save = StoragePickerRequest::save_file()
    .default_extension("png")
    .suggested_file_name("capture")
    .suggested_folder("Pictures");

let many = StoragePickerRequest::open_files();
let folder = StoragePickerRequest::pick_folder();
```

`StoragePickerKind` describes the operation. `StoragePickerRequest` only transports configuration; it never blocks egui or opens a native dialog by itself.

## Cross-platform startup

### Web: Chromium 80+

Enable `web` and build for wasm:

```bash
rustup target add wasm32-unknown-unknown
cargo build --release \
  --target wasm32-unknown-unknown \
  --no-default-features \
  --features web
```

Start on an `HtmlCanvasElement` and retain the runner:

```rust
let runner = winui::start_web(
    canvas,
    ThemeConfig::new(FluentTheme::Light),
    |_| App::default(),
).await?;

// Store runner in a long-lived application handle.
```

`web_options()` returns Glow/WebGL 2 options. See `examples/web.rs` and `examples/web-app/index.html`. Serve generated files over HTTP rather than opening them with `file://`.

### Android 7+

Enable NativeActivity:

```toml
fluent-egui = {
    version = "0.1",
    default-features = false,
    features = ["android-native-activity"]
}
```

```rust
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    winui::run_android(
        app,
        "Fluent Android",
        ThemeConfig::new(FluentTheme::Light),
        |_| App::default(),
    )
    .expect("failed to start fluent-egui");
}
```

`android_options(title, app)` is available when the host needs to customize `NativeOptions` before launch. Set `minSdkVersion` to 24. `examples/android.rs` exports the Rust entry point and `examples/android-app` is a minimal Gradle NativeActivity host.

The release APK is arm64 and debug-key signed:

```bash
adb install fluent-egui-demo-android7-arm64.apk
```

### Linux desktop environments

```bash
cargo run --release --example demo \
  --no-default-features \
  --features glow,x11,wayland,accesskit
```

This configuration targets current GNOME and KDE X11/Wayland sessions and common Budgie, XFCE, and MATE X11 sessions. The target still needs suitable graphics drivers and window-system runtime libraries.

### Windows 7

```bash
cargo build --release --example demo \
  --no-default-features \
  --features glow,win7
```

Do not enable `windows-platform`. A successful normal MSVC compile does not guarantee that the Rust standard library itself targets Windows 7; use a Win7-capable target/toolchain for the shipping artifact. Mica and modern Acrylic are unavailable and fall back to Aero or an opaque Fluent surface.

### iOS 12+

```bash
rustup target add aarch64-apple-ios
cargo check \
  --target aarch64-apple-ios \
  --no-default-features \
  --features ios
```

The widget and theme library compiles with WGPU/Metal. The iOS host owns lifecycle integration, the Metal surface, the `.app` bundle, signing, and provisioning. A generic unsigned target is not an installable IPA.

## Demo releases

Run the complete Gallery locally:

```bash
cargo run --release --example demo
```

The manually triggered `CI and cross-platform demos` workflow publishes a stable GitHub Release after all quality and platform jobs succeed.

| Asset | Purpose |
|---|---|
| `fluent-egui-demo-windows-x64.exe` | Windows 10/11 Gallery |
| `fluent-egui-demo-windows7-profile-x64.exe` | Windows 7 feature-profile Gallery |
| `fluent-egui-demo-linux-x64` and `.tar.gz` | Linux x64 Gallery |
| `fluent-egui-demo-web.zip` | Browser-ready wasm-bindgen bundle |
| `fluent-egui-demo-web-raw.wasm` | Raw, unpackaged wasm target |
| `fluent-egui-demo-android7-arm64.apk` | Installable Android 7+ arm64 debug APK |
| `fluent-egui-demo-android7-arm64-release.so` | Optimized Android NativeActivity library |
| `fluent-egui-demo-ios12-arm64-unsigned` and `.tar.gz` | Unsigned iOS target |

## Troubleshooting

### Mica or Acrylic is not visible

System backdrops depend on the Windows version, transparency settings, DWM composition, remote desktop state, and graphics driver. The crate requests the native material but cannot emulate real DWM Mica on unsupported systems.

### Text or icons differ outside Windows

Windows prefers installed Segoe fonts. Other targets use embedded Fluent System Icons and egui's text fallback fonts. Icon meaning is preserved, but outlines and font metrics can differ slightly.

### Mixing fluent-egui and native egui widgets

This is supported. Installing the theme changes global egui `Style` and `Visuals`, so native egui widgets inherit the palette. Use fluent-egui builders where exact Fluent sizing, baseline alignment, animation, or interaction semantics matter.

### Web files do not run when double-clicked

Browsers commonly block module and WebAssembly loading from `file://`. Extract the Web ZIP and serve it:

```bash
python -m http.server 8080
```

### Android APK cannot replace an older build

GitHub runners generate temporary debug signing keys. If signatures differ, uninstall the previous package before installing the new APK.

### Is the Android APK ready for an app store?

No. It is a test Gallery. A production app needs its own package ID, icons, permissions, versioning, release build policy, and long-lived signing key.

## Complete public API map

The crate re-exports its consumer-facing API under `fluent_egui::winui`:

- theme: `FluentTheme`, `ThemeConfig`, `FluentPalette`, `install`, `apply_theme`, palette/frame helpers;
- shell/backdrop: `RenderBackend`, native and Android startup functions, `BackdropKind`, `BackdropStatus`;
- concise composition: `FluentUi`, `WinUiExt`;
- core widgets: `FluentButton`, `AccentButton`, `FluentContentButton`, `IconButton`, `FluentTextEdit`, `ToggleSwitch`, `TriStateCheckBox`, `SegmentedControl`, `FluentSlider`, `FluentProgressBar`, `ProgressRing`, flyout/settings/dialog/info/toast APIs;
- extended widgets: hyperlink, toggle/split/drop-down/repeat buttons, rating, number, badge, pager, breadcrumb, tab, person, and teaching-tip APIs;
- text and date/time: all types described in their dedicated sections;
- collections: `CollectionItem`, list/grid/items/tree/flip/scroll/zoom/selector/pivot/swipe APIs;
- commands: standard/custom commands, menus, command bars, app bars, navigation, split view, title bar, and popup APIs;
- layout: border, canvas, grid, stack, relative, wrap grid, viewbox, shape, line, image, icon, and brush APIs;
- motion: easing, animated visual, page/implicit/connected transitions, parallax, shadow, theme transition, and composition interop;
- Gallery/design: page, example, source, catalogue, annotation, settings, icon catalogue, color, resource, radius, spacing, and typography helpers;
- platform: capability detection, UI surfaces, picker requests, notifications, clipboard, color, images, and playback status.

Every public item has rustdoc documentation and a `# Examples` section. Use docs.rs for exact signatures and field-level details; use this guide for application-level invocation patterns and platform boundaries.

## License

`fluent-egui` is licensed under the MIT License. The embedded Microsoft Fluent System Icons font is distributed under its included MIT license at `src/assets/FLUENT_SYSTEM_ICONS_LICENSE.txt`.

WinUI, Fluent, Windows, and related names belong to their respective owners. This project is not an official Microsoft product.
