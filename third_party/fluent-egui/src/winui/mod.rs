//! A small WinUI 3 inspired presentation layer built on public egui APIs.

mod backdrop;
mod collections;
mod commands;
mod date_time;
mod design;
mod easy;
mod extended;
mod fundamentals;
mod gallery;
mod gallery_settings;
mod guidance;
mod icon_catalog;
mod icons;
mod layout;
mod motion;
mod platform;
#[cfg(all(
    any(feature = "glow", feature = "wgpu"),
    not(target_arch = "wasm32"),
    not(target_os = "ios")
))]
mod shell;
mod text;
mod theme;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
mod web;
mod widgets;

pub use backdrop::{BackdropKind, BackdropStatus, sync_window_backdrop};
pub use collections::{
    AnnotatedScrollBar, AnnotatedScrollBarLabel, CollectionItem, FlipView, GridSelectionMode,
    GridView, ItemsRepeater, ItemsRepeaterLayout, ItemsView, ListBox, ListView, Pivot,
    PullToRefresh, PullToRefreshResponse, ScrollView, ScrollViewer, SelectorBar, SemanticZoom,
    SwipeControl, SwipeControlResponse, TreeNode, TreeSelectionMode, TreeView,
};
pub use commands::{
    AppBarButton, AppBarSeparator, AppBarToggleButton, CommandBar, CommandBarFlyout, CommandList,
    CommandListResponse, FluentPopup, MenuBar, MenuDefinition, MenuFlyout, MenuFlyoutResponse,
    MenuItem, NavigationView, NavigationViewItem, SplitView, SplitViewDisplayMode,
    SplitViewPanePlacement, SplitViewResponse, StandardUICommand, StandardUICommandKind, TitleBar,
    ToggleSplitButton, XamlUICommand,
};
pub use date_time::{
    CalendarDatePicker, CalendarView, DatePicker, FluentDate, FluentTime, TimePicker,
    TimePickerClock,
};
pub use design::{CornerRadiusTable, ResourceScopePreview, SpacingRamp, TypographyRamp};
pub use easy::{FluentUi, WinUiExt};
pub use extended::{
    BreadcrumbBar, ColorSplitButton, ColorSplitButtonResponse, DropDownButton, HyperlinkButton,
    InfoBadge, InfoBadgeKind, NumberBox, NumberBoxSpinButtonPlacement, PersonPicture, PipsPager,
    RatingControl, RepeatButton, SplitButton, SplitButtonResponse, TabView,
    TabViewCloseButtonOverlayMode, TabViewResponse, TabViewWidthMode, TeachingTip,
    TeachingTipResponse, ToggleButton, evaluate_number_expression,
};
pub use fundamentals::{
    CounterControl, CounterMode, TemperatureConverterControl, TemperatureConverterResponse,
    ValidatedPasswordBox, ValidatedPasswordResponse, validated_submit_button,
};
pub use gallery::{
    GalleryAnnotatedImage, GalleryCatalogGrid, GalleryCatalogItem, GalleryExample,
    GalleryImageHotspot, GalleryPageHeader, GalleryPageHeaderResponse, GallerySourceExpander,
    GalleryTopicCard, gallery_example_spacing,
};
pub use gallery_settings::{GallerySettings, GallerySettingsResponse};
pub use guidance::ColorGuidance;
pub use icon_catalog::{FluentIconCatalog, FluentIconEntry, fluent_icon_catalog};
pub use icons::{FluentIcon, icon_font_id};
pub use layout::{
    AcrylicBrush, Canvas, FluentBorder, FluentImage, GridPanel, IconElement, LineElement,
    Orientation, RadialGradientBrush, RelativePanel, RelativePanelUi, ShapeElement, ShapeKind,
    StackPanel, VariableSizedWrapGrid, Viewbox, compact_sizing,
};
pub use motion::{
    AnimatedIcon, AnimatedVisualPlayer, CompositionInteropStatus, ConnectedAnimation,
    EasingFunction, ImplicitTransition, PageTransition, ParallaxView, ThemeShadow, ThemeTransition,
    XamlCompInterop, transition_color,
};
pub use platform::{
    AppNotification, AppWindow, AppWindowTitleBar, BadgeNotificationManager, CaptureElement,
    ClipboardControl, ColorPicker, ContentIsland, ImagePlaceholder, JumpList, MapControl,
    MediaPlayerElement, MultipleWindows, PlatformCapability, PlatformStatus, PlaybackStatus,
    SoundControl, StoragePickerKind, StoragePickerLocation, StoragePickerRequest,
    StoragePickerViewMode, StoragePickers, WebView2, detect_platform_capability,
};
#[cfg(all(
    any(feature = "glow", feature = "wgpu"),
    not(target_arch = "wasm32"),
    not(target_os = "ios")
))]
pub use shell::{
    RenderBackend, apply_runtime_theme, native_options, native_options_with_backend, run_native,
    run_native_with_backend, run_native_with_options,
};
#[cfg(all(
    target_os = "android",
    any(feature = "android-native-activity", feature = "android-game-activity")
))]
pub use shell::{android_options, run_android};
pub use text::{
    AutoSuggestBox, AutoSuggestResponse, PasswordBox, RichEditBox, RichTextBlock, text_box,
};
pub use theme::{
    FluentPalette, FluentTheme, ThemeConfig, accent_presets, apply_theme, card_frame,
    card_frame_with_config, content_frame, content_frame_with_config, default_accent, install,
    palette, palette_with_config, surface_frame, surface_frame_with_config,
};
#[cfg(all(target_arch = "wasm32", feature = "web"))]
pub use web::{start_web, web_options};
pub use widgets::{
    AccentButton, ButtonKind, ContentDialog, ContentDialogAction, ContentDialogResponse,
    FluentButton, FluentContentButton, FluentProgressBar, FluentSlider, FluentTextEdit,
    FlyoutButton, FlyoutOption, IconButton, InfoBar, InfoBarKind, NavigationGroup,
    NavigationGroupResponse, ProgressRing, SegmentedControl, SettingsChoiceResponse,
    SettingsChoiceRow, SettingsExpander, SettingsExpanderResponse, SettingsNavigationRow, Toast,
    ToggleSwitch, TriStateCheckBox, animated_page, card_columns, combo_option, fluent_combo_box,
    fluent_text_edit, navigation_branch_item, navigation_child_item, navigation_item, section_card,
    setting_row, settings_footer, settings_group,
};

/// Imports the high-level composition API and the most commonly customized builders.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub mod prelude {
    pub use super::{
        ButtonKind, FluentButton, FluentIcon, FluentSlider, FluentTextEdit, FluentTheme, FluentUi,
        WinUiExt,
    };
}
