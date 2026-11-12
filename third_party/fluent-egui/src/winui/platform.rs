use eframe::egui::{
    self, Align, Align2, Color32, InnerResponse, Layout, Margin, Response, Sense, Stroke, Ui, Vec2,
    Widget, vec2,
};

use super::{
    AccentButton, FluentButton, FluentIcon, FluentProgressBar, FluentSlider, FluentTheme,
    IconButton, InfoBadge, icon_font_id, palette,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Available `PlatformCapability` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum PlatformCapability {
    /// The `WebView2` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    WebView2,
    /// The `CameraCapture` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CameraCapture,
    /// The `Map` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Map,
    /// The `MediaPlayback` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    MediaPlayback,
    /// The `Sound` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Sound,
    /// The `StoragePicker` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    StoragePicker,
    /// The `AppNotification` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    AppNotification,
    /// The `BadgeNotification` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    BadgeNotification,
    /// The `JumpList` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    JumpList,
    /// The `AppWindow` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    AppWindow,
    /// The `MultipleWindows` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    MultipleWindows,
    /// The `ContentIsland` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ContentIsland,
    /// The `Clipboard` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Clipboard,
}

impl PlatformCapability {
    #[cfg(all(windows, feature = "windows-platform"))]
    const fn winrt_type(self) -> &'static str {
        match self {
            Self::WebView2 => "Microsoft.Web.WebView2.Core.CoreWebView2Environment",
            Self::CameraCapture => "Windows.Media.Capture.MediaCapture",
            Self::Map => "Microsoft.UI.Xaml.Controls.Maps.MapControl",
            Self::MediaPlayback | Self::Sound => "Windows.Media.Playback.MediaPlayer",
            Self::StoragePicker => "Windows.Storage.Pickers.FileOpenPicker",
            Self::AppNotification => "Microsoft.Windows.AppNotifications.AppNotificationManager",
            Self::BadgeNotification => "Windows.UI.Notifications.BadgeUpdateManager",
            Self::JumpList => "Windows.UI.StartScreen.JumpList",
            Self::AppWindow | Self::MultipleWindows => "Microsoft.UI.Windowing.AppWindow",
            Self::ContentIsland => "Microsoft.UI.Content.ContentIsland",
            Self::Clipboard => "Windows.ApplicationModel.DataTransfer.Clipboard",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Available `PlatformStatus` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum PlatformStatus {
    /// The `Available` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Available,
    /// The `Fallback` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Fallback,
    /// The `Unsupported` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Unsupported,
    /// The `Failed` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Failed(String),
}

impl PlatformStatus {
    /// Reports the current `is_available` state.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }

    /// Configures or reads the `label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Available => "Windows adapter available",
            Self::Fallback => "Using egui fallback",
            Self::Unsupported => "Unsupported on this system",
            Self::Failed(_) => "Windows adapter failed",
        }
    }
}

/// Detects whether the Windows runtime type for a platform capability is available.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn detect_platform_capability(capability: PlatformCapability) -> PlatformStatus {
    detect::capability(capability)
}

#[cfg(all(windows, feature = "windows-platform"))]
mod detect {
    use windows::{
        Foundation::Metadata::ApiInformation,
        Win32::System::WinRT::{RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED, RoInitialize},
        core::{HRESULT, HSTRING},
    };

    use super::{PlatformCapability, PlatformStatus};

    pub(super) fn capability(capability: PlatformCapability) -> PlatformStatus {
        thread_local! {
            static WINRT_READY: Result<(), String> = {
                // A pre-existing STA is also valid for ApiInformation calls.
                match unsafe { RoInitialize(RO_INIT_MULTITHREADED) } {
                    Ok(()) => Ok(()),
                    Err(error) if error.code() == HRESULT(0x80010106_u32 as i32) => {
                        unsafe { RoInitialize(RO_INIT_SINGLETHREADED) }
                            .map_err(|error| error.to_string())
                    }
                    Err(error) => Err(error.to_string()),
                }
            };
        }
        let initialization = WINRT_READY.with(Clone::clone);
        if let Err(error) = initialization {
            return PlatformStatus::Failed(error);
        }
        match ApiInformation::IsTypePresent(&HSTRING::from(capability.winrt_type())) {
            Ok(true) => PlatformStatus::Available,
            Ok(false) => PlatformStatus::Unsupported,
            Err(error) if error.code() == HRESULT(0x80073D54_u32 as i32) => {
                PlatformStatus::Unsupported
            }
            Err(error) => PlatformStatus::Failed(error.to_string()),
        }
    }
}

#[cfg(not(all(windows, feature = "windows-platform")))]
mod detect {
    use super::{PlatformCapability, PlatformStatus};

    pub(super) fn capability(_capability: PlatformCapability) -> PlatformStatus {
        PlatformStatus::Fallback
    }
}

fn current_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

fn capability_surface<R>(
    ui: &mut Ui,
    title: &str,
    description: &str,
    icon: FluentIcon,
    status: &PlatformStatus,
    size: Vec2,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let p = current_palette(ui);
    egui::Frame::new()
        .fill(p.control_bg)
        .stroke(Stroke::new(1.0, p.control_border))
        .corner_radius(0)
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_size(size);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(icon.glyph())
                        .font(icon_font_id(20.0))
                        .color(p.text_primary),
                );
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(title).strong());
                    ui.label(
                        egui::RichText::new(description)
                            .size(12.0)
                            .color(p.text_secondary),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(egui::RichText::new(status.label()).size(11.0).color(
                        if status.is_available() {
                            p.success
                        } else {
                            p.text_secondary
                        },
                    ));
                });
            });
            ui.add_space(10.0);
            add_contents(ui)
        })
}

macro_rules! capability_control {
    ($name:ident, $capability:ident, $title:literal, $description:literal, $icon:ident) => {
        /// A Fluent API value representing `pub`.
        ///
        /// # Examples
        ///
        /// ```
        /// let theme = fluent_egui::winui::FluentTheme::Light;
        /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
        /// ```
        pub struct $name {
            status: PlatformStatus,
            size: Vec2,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            /// Creates a new value with WinUI-compatible defaults.
            ///
            /// # Examples
            ///
            /// ```
            /// let theme = fluent_egui::winui::FluentTheme::Light;
            /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
            /// ```
            pub fn new() -> Self {
                Self {
                    status: detect_platform_capability(PlatformCapability::$capability),
                    size: vec2(360.0, 116.0),
                }
            }

            /// Configures or reads the `status` option.
            ///
            /// # Examples
            ///
            /// ```
            /// let theme = fluent_egui::winui::FluentTheme::Light;
            /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
            /// ```
            pub fn status(mut self, status: PlatformStatus) -> Self {
                self.status = status;
                self
            }

            /// Configures or reads the `size` option.
            ///
            /// # Examples
            ///
            /// ```
            /// let theme = fluent_egui::winui::FluentTheme::Light;
            /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
            /// ```
            pub const fn size(mut self, size: Vec2) -> Self {
                self.size = size;
                self
            }

            /// Renders the control in the supplied egui UI.
            ///
            /// # Examples
            ///
            /// ```
            /// let theme = fluent_egui::winui::FluentTheme::Light;
            /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
            /// ```
            pub fn show<R>(
                self,
                ui: &mut Ui,
                add_fallback: impl FnOnce(&mut Ui) -> R,
            ) -> InnerResponse<R> {
                capability_surface(
                    ui,
                    $title,
                    $description,
                    FluentIcon::$icon,
                    &self.status,
                    self.size,
                    add_fallback,
                )
            }
        }
    };
}

capability_control!(
    WebView2,
    WebView2,
    "WebView2",
    "Embedded web content host",
    Globe
);

impl WebView2 {
    /// Draws only the embedded content surface, without the capability header.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_surface<R>(
        self,
        ui: &mut Ui,
        add_fallback: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let p = current_palette(ui);
        egui::Frame::new()
            .fill(if ui.visuals().dark_mode {
                Color32::from_rgb(15, 15, 15)
            } else {
                Color32::WHITE
            })
            .stroke(Stroke::new(1.0, p.control_border))
            .show(ui, |ui| {
                ui.set_min_size(self.size);
                add_fallback(ui)
            })
    }
}
capability_control!(
    CaptureElement,
    CameraCapture,
    "CaptureElement",
    "Camera preview surface",
    Camera
);

impl CaptureElement {
    /// Draws the camera-preview host without the capability header.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_surface<R>(
        self,
        ui: &mut Ui,
        add_fallback: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let p = current_palette(ui);
        egui::Frame::new()
            .fill(Color32::from_rgb(15, 15, 15))
            .stroke(Stroke::new(1.0, p.control_border))
            .show(ui, |ui| {
                ui.set_min_size(self.size);
                add_fallback(ui)
            })
    }
}
capability_control!(
    MapControl,
    Map,
    "MapControl",
    "Interactive map surface",
    Map
);
capability_control!(
    ContentIsland,
    ContentIsland,
    "ContentIsland",
    "External visual content host",
    Window
);
capability_control!(
    MultipleWindows,
    MultipleWindows,
    "Multiple windows",
    "eframe viewport / AppWindow host",
    Window
);

/// A MediaPlayerElement-style transport surface. Media decoding is supplied by an adapter.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MediaPlayerElement<'a> {
    playing: &'a mut bool,
    position: &'a mut f32,
    duration_label: String,
    status: PlatformStatus,
}

impl<'a> MediaPlayerElement<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(playing: &'a mut bool, position: &'a mut f32) -> Self {
        Self {
            playing,
            position,
            duration_label: "03:42".into(),
            status: detect_platform_capability(PlatformCapability::MediaPlayback),
        }
    }

    /// Configures or reads the `duration_label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn duration_label(mut self, label: impl Into<String>) -> Self {
        self.duration_label = label.into();
        self
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "MediaPlayerElement",
            "Transport controls with an adapter-owned decoder",
            FluentIcon::Play,
            &self.status,
            vec2(420.0, 150.0),
            |ui| {
                let p = current_palette(ui);
                let (preview, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 54.0), Sense::hover());
                ui.painter()
                    .rect_filled(preview, 3, Color32::from_rgb(24, 24, 24));
                ui.painter().text(
                    preview.center(),
                    Align2::CENTER_CENTER,
                    "Media preview",
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::from_gray(210),
                );
                ui.horizontal(|ui| {
                    if ui
                        .add(IconButton::new(if *self.playing {
                            FluentIcon::Pause
                        } else {
                            FluentIcon::Play
                        }))
                        .clicked()
                    {
                        *self.playing = !*self.playing;
                    }
                    ui.add(FluentSlider::new(self.position, 0.0..=1.0).width(250.0));
                    ui.label(&self.duration_label);
                });
                p
            },
        )
        .response
    }
}

/// A Sound settings-style volume surface.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SoundControl<'a> {
    volume: &'a mut f32,
    muted: &'a mut bool,
    status: PlatformStatus,
}

impl<'a> SoundControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(volume: &'a mut f32, muted: &'a mut bool) -> Self {
        Self {
            volume,
            muted,
            status: detect_platform_capability(PlatformCapability::Sound),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "Sound",
            "Application volume and output state",
            FluentIcon::Volume,
            &self.status,
            vec2(380.0, 100.0),
            |ui| {
                ui.horizontal(|ui| {
                    if ui.add(IconButton::new(FluentIcon::Volume)).clicked() {
                        *self.muted = !*self.muted;
                    }
                    ui.add(
                        FluentSlider::new(self.volume, 0.0..=100.0)
                            .width(260.0)
                            .show_value(true),
                    );
                });
            },
        )
        .response
    }
}

/// Operation represented by a [`StoragePickerRequest`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoragePickerKind {
    /// The `OpenFile` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    OpenFile,
    /// The `OpenFiles` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    OpenFiles,
    /// The `SaveFile` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    SaveFile,
    /// The `PickFolder` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    PickFolder,
}

/// Suggested initial folder for a storage picker.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StoragePickerLocation {
    #[default]
    /// The `DocumentsLibrary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    DocumentsLibrary,
    /// The `ComputerFolder` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ComputerFolder,
    /// The `Desktop` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Desktop,
    /// The `Downloads` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Downloads,
    /// The `HomeGroup` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    HomeGroup,
    /// The `MusicLibrary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    MusicLibrary,
    /// The `PicturesLibrary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    PicturesLibrary,
    /// The `VideosLibrary` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    VideosLibrary,
    /// The `Objects3D` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Objects3D,
    /// The `Unspecified` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Unspecified,
}

/// Initial item layout requested from a storage picker.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StoragePickerViewMode {
    #[default]
    /// The `List` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    List,
    /// The `Thumbnail` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Thumbnail,
}

/// Platform-neutral request passed to an application-owned storage picker adapter.
///
/// This crate deliberately keeps system dialog execution outside egui. Applications can build
/// this request from Gallery-style controls, then translate it to Windows App SDK, WinRT, COM, or
/// a cross-platform picker without changing the presentation layer.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoragePickerRequest {
    /// The `kind` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub kind: StoragePickerKind,
    /// The `file_filters` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub file_filters: Vec<String>,
    /// The `commit_button_text` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub commit_button_text: String,
    /// The `suggested_start_location` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub suggested_start_location: StoragePickerLocation,
    /// The `view_mode` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub view_mode: StoragePickerViewMode,
    /// The `default_extension` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub default_extension: Option<String>,
    /// The `suggested_file_name` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub suggested_file_name: Option<String>,
    /// The `suggested_folder` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub suggested_folder: Option<String>,
}

impl StoragePickerRequest {
    fn new(kind: StoragePickerKind) -> Self {
        Self {
            kind,
            file_filters: Vec::new(),
            commit_button_text: String::new(),
            suggested_start_location: StoragePickerLocation::default(),
            view_mode: StoragePickerViewMode::default(),
            default_extension: None,
            suggested_file_name: None,
            suggested_folder: None,
        }
    }

    /// Creates the `open_file` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn open_file() -> Self {
        Self::new(StoragePickerKind::OpenFile)
    }

    /// Creates the `open_files` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn open_files() -> Self {
        Self::new(StoragePickerKind::OpenFiles)
    }

    /// Creates the `save_file` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn save_file() -> Self {
        Self::new(StoragePickerKind::SaveFile)
    }

    /// Creates the `pick_folder` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn pick_folder() -> Self {
        Self::new(StoragePickerKind::PickFolder)
    }

    /// Configures or reads the `file_filter` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn file_filter(mut self, filter: impl Into<String>) -> Self {
        self.file_filters.push(filter.into());
        self
    }

    /// Configures or reads the `commit_button_text` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn commit_button_text(mut self, text: impl Into<String>) -> Self {
        self.commit_button_text = text.into();
        self
    }

    /// Configures or reads the `suggested_start_location` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn suggested_start_location(mut self, location: StoragePickerLocation) -> Self {
        self.suggested_start_location = location;
        self
    }

    /// Configures or reads the `view_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn view_mode(mut self, mode: StoragePickerViewMode) -> Self {
        self.view_mode = mode;
        self
    }

    /// Configures or reads the `default_extension` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn default_extension(mut self, extension: impl Into<String>) -> Self {
        self.default_extension = Some(extension.into());
        self
    }

    /// Configures or reads the `suggested_file_name` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn suggested_file_name(mut self, name: impl Into<String>) -> Self {
        self.suggested_file_name = Some(name.into());
        self
    }

    /// Configures or reads the `suggested_folder` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn suggested_folder(mut self, folder: impl Into<String>) -> Self {
        self.suggested_folder = Some(folder.into());
        self
    }
}

/// A storage picker command. The actual file dialog is supplied by a platform adapter.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct StoragePickers {
    status: PlatformStatus,
}

impl Default for StoragePickers {
    fn default() -> Self {
        Self::new()
    }
}

impl StoragePickers {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new() -> Self {
        Self {
            status: detect_platform_capability(PlatformCapability::StoragePicker),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "StoragePickers",
            "Open, save and folder picker commands",
            FluentIcon::Folder,
            &self.status,
            vec2(380.0, 100.0),
            |ui| {
                ui.horizontal(|ui| {
                    ui.add(AccentButton::new("Open file").icon(FluentIcon::OpenFile));
                    ui.add(FluentButton::new("Save as").icon(FluentIcon::Save));
                    ui.add(FluentButton::new("Choose folder").icon(FluentIcon::Folder));
                })
                .response
            },
        )
        .inner
    }
}

/// An application notification composer with an in-app fallback action.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AppNotification {
    status: PlatformStatus,
    title: String,
    message: String,
}

impl AppNotification {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: detect_platform_capability(PlatformCapability::AppNotification),
            title: title.into(),
            message: message.into(),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "AppNotification",
            "Windows notification with in-app Toast fallback",
            FluentIcon::Notification,
            &self.status,
            vec2(380.0, 118.0),
            |ui| {
                ui.label(egui::RichText::new(self.title).strong());
                ui.label(self.message);
                ui.add(AccentButton::new("Send notification"))
            },
        )
        .inner
    }
}

/// A taskbar/start badge notification surface.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct BadgeNotificationManager {
    value: u32,
    status: PlatformStatus,
}

impl BadgeNotificationManager {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: u32) -> Self {
        Self {
            value,
            status: detect_platform_capability(PlatformCapability::BadgeNotification),
        }
    }
}

impl Widget for BadgeNotificationManager {
    fn ui(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "BadgeNotification",
            "Shell badge with InfoBadge fallback",
            FluentIcon::Notification,
            &self.status,
            vec2(300.0, 88.0),
            |ui| ui.add(InfoBadge::value(self.value)),
        )
        .inner
    }
}

/// A JumpList editor surface.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct JumpList<'a> {
    items: &'a [String],
    status: PlatformStatus,
}

impl<'a> JumpList<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(items: &'a [String]) -> Self {
        Self {
            items,
            status: detect_platform_capability(PlatformCapability::JumpList),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let mut selected = None;
        capability_surface(
            ui,
            "JumpList",
            "Recent and pinned Shell destinations",
            FluentIcon::List,
            &self.status,
            vec2(320.0, 120.0),
            |ui| {
                for (index, item) in self.items.iter().enumerate() {
                    if ui.add(FluentButton::subtle(item)).clicked() {
                        selected = Some(index);
                    }
                }
            },
        );
        selected
    }
}

/// An AppWindow status/control surface. The winit window remains owned by eframe.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AppWindow {
    status: PlatformStatus,
}

impl Default for AppWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl AppWindow {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new() -> Self {
        Self {
            status: detect_platform_capability(PlatformCapability::AppWindow),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "AppWindow",
            "Window state commands through eframe/winit or WinAppSDK",
            FluentIcon::Window,
            &self.status,
            vec2(360.0, 98.0),
            |ui| {
                ui.horizontal(|ui| {
                    ui.add(FluentButton::new("New window"));
                    ui.add(FluentButton::new("Compact overlay"));
                })
                .response
            },
        )
        .inner
    }
}

/// Public alias for the `AppWindowTitleBar` API.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub type AppWindowTitleBar = super::TitleBar;

/// A clipboard command surface using egui text clipboard as its fallback.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ClipboardControl<'a> {
    text: &'a mut String,
    status: PlatformStatus,
}

impl<'a> ClipboardControl<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: &'a mut String) -> Self {
        Self {
            text,
            status: detect_platform_capability(PlatformCapability::Clipboard),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Response {
        capability_surface(
            ui,
            "Clipboard",
            "Text clipboard with Windows rich-format adapter slot",
            FluentIcon::Copy,
            &self.status,
            vec2(380.0, 104.0),
            |ui| {
                ui.text_edit_singleline(self.text);
                ui.horizontal(|ui| {
                    if ui
                        .add(FluentButton::new("Copy").icon(FluentIcon::Copy))
                        .clicked()
                    {
                        ui.ctx().copy_text(self.text.clone());
                    }
                    ui.add(FluentButton::new("Paste").icon(FluentIcon::Paste));
                })
                .response
            },
        )
        .inner
    }
}

/// A simple color picker wrapper around egui's native HSV picker.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ColorPicker<'a> {
    color: &'a mut Color32,
    alpha: bool,
}

impl<'a> ColorPicker<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(color: &'a mut Color32) -> Self {
        Self { color, alpha: true }
    }

    /// Configures or reads the `alpha` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn alpha(mut self, enabled: bool) -> Self {
        self.alpha = enabled;
        self
    }
}

impl Widget for ColorPicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let alpha = if self.alpha {
            egui::color_picker::Alpha::BlendOrAdditive
        } else {
            egui::color_picker::Alpha::Opaque
        };
        egui::color_picker::color_edit_button_srgba(ui, self.color, alpha)
    }
}

/// A lightweight image presenter for Image Gallery demos without an external texture.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ImagePlaceholder {
    label: String,
    size: Vec2,
}

impl ImagePlaceholder {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(label: impl Into<String>, size: Vec2) -> Self {
        Self {
            label: label.into(),
            size,
        }
    }
}

impl Widget for ImagePlaceholder {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = current_palette(ui);
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        ui.painter().rect_filled(rect, 0, p.control_pressed);
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.label,
            egui::TextStyle::Body.resolve(ui.style()),
            p.text_secondary,
        );
        response
    }
}

/// A linear playback progress surface used for Sound and media fallbacks.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PlaybackStatus {
    progress: f32,
}

impl PlaybackStatus {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(progress: f32) -> Self {
        Self { progress }
    }
}

impl Widget for PlaybackStatus {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(FluentProgressBar::new(self.progress).width(240.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_status_has_a_stable_label() {
        assert_eq!(PlatformStatus::Fallback.label(), "Using egui fallback");
    }

    #[cfg(all(windows, feature = "windows-platform"))]
    #[test]
    fn windows_capability_diagnostics_do_not_panic() {
        for capability in [
            PlatformCapability::WebView2,
            PlatformCapability::Clipboard,
            PlatformCapability::StoragePicker,
            PlatformCapability::MediaPlayback,
        ] {
            println!(
                "{capability:?}: {:?}",
                detect_platform_capability(capability)
            );
        }
    }

    #[test]
    fn storage_picker_request_preserves_adapter_options() {
        let request = StoragePickerRequest::save_file()
            .file_filter(".txt")
            .commit_button_text("Save File")
            .suggested_start_location(StoragePickerLocation::Downloads)
            .view_mode(StoragePickerViewMode::Thumbnail)
            .default_extension(".txt")
            .suggested_file_name("NewDocument")
            .suggested_folder("C:\\Temp");

        assert_eq!(request.kind, StoragePickerKind::SaveFile);
        assert_eq!(request.file_filters, [".txt"]);
        assert_eq!(request.commit_button_text, "Save File");
        assert_eq!(
            request.suggested_start_location,
            StoragePickerLocation::Downloads
        );
        assert_eq!(request.view_mode, StoragePickerViewMode::Thumbnail);
        assert_eq!(request.default_extension.as_deref(), Some(".txt"));
        assert_eq!(request.suggested_file_name.as_deref(), Some("NewDocument"));
        assert_eq!(request.suggested_folder.as_deref(), Some("C:\\Temp"));
    }
}
