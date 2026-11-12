use eframe::egui::{FontFamily, FontId};

pub(crate) const ICON_FONT_FAMILY: &str = "Segoe Fluent Icons";

/// Icons from the Segoe Fluent/MDL2 private-use area available on Windows.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FluentIcon {
    /// The `Home` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Home,
    /// The `Controls` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Controls,
    /// The `Palette` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Palette,
    /// The `Settings` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Settings,
    /// The `Search` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Search,
    /// The `Add` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Add,
    /// The `ChevronRight` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ChevronRight,
    /// The `ChevronDown` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ChevronDown,
    /// The `ChevronUp` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    ChevronUp,
    /// The `Sun` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Sun,
    /// The `Moon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Moon,
    /// The `Info` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Info,
    /// The `Download` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Download,
    /// The `More` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    More,
    /// The `Check` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Check,
    /// The `Close` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Close,
    /// The `Notification` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Notification,
    /// The `Copy` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Copy,
    /// The `Volume` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Volume,
    /// The `Headphones` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Headphones,
    /// The `Device` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Device,
    /// The `Back` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Back,
    /// The `Display` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Display,
    /// The `Focus` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Focus,
    /// The `Power` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Power,
    /// The `Color` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Color,
    /// The `Scale` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Scale,
    /// The `Rotate` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Rotate,
    /// The `Save` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Save,
    /// The `OpenFile` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    OpenFile,
    /// The `Delete` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Delete,
    /// The `Undo` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Undo,
    /// The `Redo` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Redo,
    /// The `Cut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Cut,
    /// The `Paste` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Paste,
    /// The `Play` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Play,
    /// The `Pause` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Pause,
    /// The `Stop` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Stop,
    /// The `Camera` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Camera,
    /// The `Map` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Map,
    /// The `Calendar` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Calendar,
    /// The `Globe` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Globe,
    /// The `Folder` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Folder,
    /// The `Refresh` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Refresh,
    /// The `Mail` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Mail,
    /// The `Window` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Window,
    /// The `List` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    List,
    /// The `Grid` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Grid,
    /// The `Zoom` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Zoom,
    /// The `Upload` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Upload,
    /// The `Favorite` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Favorite,
    /// The `Sort` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Sort,
    /// The `Person` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Person,
}

impl FluentIcon {
    /// Configures or reads the `name` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn name(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Controls => "Controls",
            Self::Palette => "Palette",
            Self::Settings => "Settings",
            Self::Search => "Search",
            Self::Add => "Add",
            Self::ChevronRight => "Next",
            Self::ChevronDown => "Open",
            Self::ChevronUp => "Up",
            Self::Sun => "Sun",
            Self::Moon => "Moon",
            Self::Info => "Information",
            Self::Download => "Download",
            Self::More => "More",
            Self::Check => "Check",
            Self::Close => "Close",
            Self::Notification => "Notification",
            Self::Copy => "Copy",
            Self::Volume => "Volume",
            Self::Headphones => "Headphones",
            Self::Device => "Device",
            Self::Back => "Back",
            Self::Display => "Display",
            Self::Focus => "Focus",
            Self::Power => "Power",
            Self::Color => "Color",
            Self::Scale => "Scale",
            Self::Rotate => "Rotate",
            Self::Save => "Save",
            Self::OpenFile => "Open file",
            Self::Delete => "Delete",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Cut => "Cut",
            Self::Paste => "Paste",
            Self::Play => "Play",
            Self::Pause => "Pause",
            Self::Stop => "Stop",
            Self::Camera => "Camera",
            Self::Map => "Map",
            Self::Calendar => "Calendar",
            Self::Globe => "Web",
            Self::Folder => "Folder",
            Self::Refresh => "Refresh",
            Self::Mail => "Mail",
            Self::Window => "Window",
            Self::List => "List",
            Self::Grid => "Grid",
            Self::Zoom => "Zoom",
            Self::Upload => "Upload",
            Self::Favorite => "Favorite",
            Self::Sort => "Sort",
            Self::Person => "Person",
        }
    }

    /// Configures or reads the `glyph` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn glyph(self) -> char {
        #[cfg(windows)]
        {
            match self {
                Self::Home => '\u{E80F}',
                Self::Controls => '\u{E71D}',
                Self::Palette => '\u{E790}',
                Self::Settings => '\u{E713}',
                Self::Search => '\u{E721}',
                Self::Add => '\u{E710}',
                Self::ChevronRight => '\u{E76C}',
                Self::ChevronDown => '\u{E70D}',
                Self::ChevronUp => '\u{E70E}',
                Self::Sun => '\u{E706}',
                Self::Moon => '\u{E708}',
                Self::Info => '\u{E946}',
                Self::Download => '\u{E896}',
                Self::More => '\u{E712}',
                Self::Check => '\u{E73E}',
                Self::Close => '\u{E711}',
                Self::Notification => '\u{E7ED}',
                Self::Copy => '\u{E8C8}',
                Self::Volume => '\u{E767}',
                Self::Headphones => '\u{E7F6}',
                Self::Device => '\u{E7F4}',
                Self::Back => '\u{E72B}',
                Self::Display => '\u{E7F8}',
                Self::Focus => '\u{E708}',
                Self::Power => '\u{E7E8}',
                Self::Color => '\u{E790}',
                Self::Scale => '\u{E740}',
                Self::Rotate => '\u{E7AD}',
                Self::Save => '\u{E74E}',
                Self::OpenFile => '\u{E8E5}',
                Self::Delete => '\u{E74D}',
                Self::Undo => '\u{E7A7}',
                Self::Redo => '\u{E7A6}',
                Self::Cut => '\u{E8C6}',
                Self::Paste => '\u{E77F}',
                Self::Play => '\u{E768}',
                Self::Pause => '\u{E769}',
                Self::Stop => '\u{E71A}',
                Self::Camera => '\u{E722}',
                Self::Map => '\u{E707}',
                Self::Calendar => '\u{E787}',
                Self::Globe => '\u{E774}',
                Self::Folder => '\u{E8B7}',
                Self::Refresh => '\u{E72C}',
                Self::Mail => '\u{E715}',
                Self::Window => '\u{E737}',
                Self::List => '\u{EA37}',
                Self::Grid => '\u{F0E2}',
                Self::Zoom => '\u{E71E}',
                Self::Upload => '\u{E898}',
                Self::Favorite => '\u{E734}',
                Self::Sort => '\u{E8CB}',
                Self::Person => '\u{E77B}',
            }
        }
        #[cfg(not(windows))]
        {
            match self {
                Self::Home => '\u{F480}',
                Self::Controls => '\u{F133}',
                Self::Palette | Self::Color => '\u{F2F5}',
                Self::Settings => '\u{F6A9}',
                Self::Search => '\u{F68F}',
                Self::Add => '\u{F109}',
                Self::ChevronRight => '\u{F2B0}',
                Self::ChevronDown => '\u{F2A3}',
                Self::ChevronUp => '\u{F2B6}',
                Self::Sun => '\u{F8A1}',
                Self::Moon => '\u{F87D}',
                Self::Info => '\u{F4A3}',
                Self::Download => '\u{F150}',
                Self::More => '\u{E824}',
                Self::Check => '\u{F294}',
                Self::Close => '\u{F369}',
                Self::Notification => '\u{F114}',
                Self::Copy => '\u{F32B}',
                Self::Volume => '\u{EB42}',
                Self::Headphones => '\u{E6EE}',
                Self::Device => '\u{E4B1}',
                Self::Back => '\u{F15B}',
                Self::Display => '\u{F359}',
                Self::Focus => '\u{F781}',
                Self::Power => '\u{F60E}',
                Self::Scale => '\u{F66C}',
                Self::Rotate => '\u{F185}',
                Self::Save => '\u{F67F}',
                Self::OpenFile => '\u{F42E}',
                Self::Delete => '\u{F34C}',
                Self::Undo => '\u{F199}',
                Self::Redo => '\u{F16E}',
                Self::Cut => '\u{F33A}',
                Self::Paste => '\u{F2D5}',
                Self::Play => '\u{F605}',
                Self::Pause => '\u{F5A1}',
                Self::Stop => '\u{F72A}',
                Self::Camera => '\u{F254}',
                Self::Map => '\u{E7DA}',
                Self::Calendar => '\u{F0284}',
                Self::Globe => '\u{F45A}',
                Self::Folder => '\u{F418}',
                Self::Refresh => '\u{F190}',
                Self::Mail => '\u{F506}',
                Self::Window => '\u{F8B5}',
                Self::List => '\u{F4ED}',
                Self::Grid => '\u{F462}',
                Self::Zoom => '\u{F8C4}',
                Self::Upload => '\u{F1A4}',
                Self::Favorite => '\u{F70F}',
                Self::Sort => '\u{F18A}',
                Self::Person => '\u{F5BD}',
            }
        }
    }
}

/// Returns a font id bound to the Fluent icon font installed by [`super::install`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn icon_font_id(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(ICON_FONT_FAMILY.into()))
}
