//! Win10X shell UI.
//!
//! - **Runtime widgets:** `fluent-egui` (Fluent 2 / WinUI 3) — kept as the editable base
//! - **Look:** `fluent1` skin ported from `/home/rong/Win10UI` (Fluent 1 / Win10 tokens)
//!
//! L1: `cargo run -p win10x-shell`

mod apps;
mod desktop;
mod fluent1;
mod icons;
mod start_tiles;

use eframe::egui::{
    self, Align, Color32, CornerRadius, Layout, RichText, Sense, Stroke, StrokeKind, TextureHandle,
    TextureOptions, UiBuilder,
};
use fluent_egui::winui::prelude::*;
use fluent_egui::winui::{self, FluentTheme, ThemeConfig};
use win10x_protocol::{
    format_shell_title, scale_to_percent, AppEntry, CompositorSessionState, Panel, RunningApp,
    SessionCommand, ShellSnapshot, OUTPUT_SCALE_PRESETS, SESSION_COMMAND_PATH_ENV,
    SESSION_STATE_PATH_ENV, SHELL_APP_ID, SHELL_TITLE_PREFIX,
};

use desktop::{
    accent_from_bitmap, contrast_ratio, palette_from_bitmap, resolve_bitmap, wallpaper_by_id,
    WallpaperBitmap, CONTRAST_THEMES, WALLPAPERS, WIN10UI_WALLPAPER_PATH, WIN10_WINDOWS_COLORS,
};
use fluent1::{Fluent1Theme, Palette, RevealState};
use icons::{paint_app_icon, IconCache};
use serde::{Deserialize, Serialize};
use start_tiles::{
    flutter_ease_out_expo, paint_windows8_launch_tile, AppLaunchAnimationStyle, StartLayoutStyle,
    StartTileAnimationStyle, StartTileBoard, StartTileFlow, StartTileReveal,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let nested = std::env::var_os("WIN10X_NESTED").is_some();
    let theme = ThemeConfig::new(FluentTheme::Light);

    let mut options = winui::native_options(SHELL_TITLE_PREFIX);
    options.viewport = egui::ViewportBuilder::default()
        .with_title(SHELL_TITLE_PREFIX)
        .with_app_id(SHELL_APP_ID)
        .with_fullscreen(true)
        .with_decorations(false)
        .with_transparent(nested)
        .with_resizable(false);
    options.centered = false;

    if nested {
        tracing::info!("nested mode: shell chrome over win10x-comp desktop composition");
        // Nested smithay often has no MSAA configs for glutin.
        options.multisampling = 0;
        options.depth_buffer = 0;
        options.stencil_buffer = 0;
        winui::run_native_with_options(SHELL_TITLE_PREFIX, theme, options, |_| {
            ShellApp::new_nested()
        })
    } else {
        tracing::info!("standalone mode: undecorated fullscreen shell");
        winui::run_native_with_options(SHELL_TITLE_PREFIX, theme, options, |_| ShellApp::new())
    }
}

struct ShellApp {
    snap: ShellSnapshot,
    theme: Fluent1Theme,
    accent: Color32,
    wallpaper_id: String,
    /// Absolute path when user picked a file (or Win10UI sample).
    wallpaper_path: Option<String>,
    recent_wallpapers: Vec<String>,
    wallpaper_bmp: WallpaperBitmap,
    wallpaper_tex: Option<TextureHandle>,
    /// Win10: Automatically pick an accent color from my background
    accent_from_background: bool,
    /// Win10: Show accent color on Start, taskbar, and action center
    accent_on_surfaces: bool,
    recent_accents: Vec<Color32>,
    wallpaper_palette: Vec<Color32>,
    settings_tab: SettingsTab,
    custom_hue: f32,
    custom_sat: f32,
    custom_val: f32,
    contrast_idx: usize,
    pal: Palette,
    reveal: RevealState,
    search: String,
    show_all_apps: bool,
    quick_wifi: bool,
    quick_bt: bool,
    quick_a11y: bool,
    expand_wifi: bool,
    expand_bt: bool,
    expand_a11y: bool,
    media_playing: bool,
    last_error: Option<String>,
    swipe_y0: Option<f32>,
    icon_shift_t: f32,
    /// Running under `win10x-comp`; unused shell pixels stay transparent for app composition.
    nested: bool,
    /// User-pinned Start tiles (integer grid).
    tile_board: StartTileBoard,
    /// Dragging an app from the Start list onto the tile board.
    list_pin_drag: Option<ListPinDrag>,
    icon_cache: IconCache,
    /// Avoid rebuilding egui Style every frame.
    style_key: Option<(u8, u32, bool)>,
    last_pointer: Option<egui::Pos2>,
    start_reveal: StartReveal,
    pending_app_launch: Option<PendingAppLaunch>,
    pending_app_switch: Option<PendingAppSwitch>,
    start_tile_animation: StartTileAnimationStyle,
    app_launch_animation: AppLaunchAnimationStyle,
    start_layout_style: StartLayoutStyle,
    app_switch_animation_enabled: bool,
    switch_animation_policy_sent: bool,
    phone_apps_visible: bool,
    windows8_apps_visible: bool,
    start_swipe_anchor: Option<egui::Pos2>,
    start_apps_progress: f32,
    start_page_animation: Option<StartPageAnimation>,
    ui_now: f64,
    session_state_path: Option<PathBuf>,
    session_command_path: Option<PathBuf>,
    last_session_poll: f64,
    runtime_app_ids: HashMap<String, String>,
    runtime_apps: HashMap<String, RunningApp>,
    runtime_windows: HashMap<String, Vec<RunningApp>>,
    window_preview_cache: HashMap<u64, WindowPreviewTexture>,
    window_switch_preview_cache: HashMap<u64, WindowPreviewTexture>,
    active_app_id: Option<String>,
    taskbar_app_menu: Option<TaskbarAppMenu>,
    taskbar_app_rects: HashMap<String, egui::Rect>,
}

#[derive(Debug, Clone)]
struct TaskbarAppMenu {
    app_id: String,
    anchor: egui::Rect,
    opened_at: f64,
}

struct WindowPreviewTexture {
    path: String,
    modified: Option<std::time::SystemTime>,
    texture: TextureHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskbarAppAction {
    Close,
    TogglePin,
    NewWindow,
    Split,
    Float,
}

#[derive(Debug, Clone)]
struct ListPinDrag {
    app_id: String,
    name: String,
    /// True after the pointer has moved past the drag threshold.
    active: bool,
    start_pos: egui::Pos2,
}

#[derive(Debug, Clone)]
struct PendingAppLaunch {
    app_id: String,
    origin: Option<egui::Rect>,
    source: AppLaunchSource,
}

#[derive(Debug, Clone)]
struct PendingAppSwitch {
    target_app_id: String,
    target: RunningApp,
    outgoing: Option<RunningApp>,
    origin: egui::Rect,
    kind: AppSwitchKind,
    t0: f64,
}

#[derive(Debug, Clone, Copy)]
enum AppSwitchKind {
    /// flutter_metro_ui's Start/taskbar handoff: a cached page grows from the
    /// taskbar icon rectangle to the application viewport in 200 ms.
    TaskbarZoom,
    /// Its already-open page switch: the old page retreats, then the new page
    /// travels in horizontally and grows to its settled size over 500 ms.
    PageSlide { direction: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppLaunchSource {
    Tile,
    AppList,
    Rail,
}

/// Start open/close or the launcher side of an application handoff.
#[derive(Debug, Clone, Copy, Default)]
enum StartReveal {
    #[default]
    Idle,
    Opening {
        t0: f64,
    },
    Closing {
        t0: f64,
    },
}

#[derive(Debug, Clone, Copy)]
struct StartPageAnimation {
    from: f32,
    to: f32,
    t0: f64,
}

const START_PAGE_SWITCH_DURATION: f64 = 0.34;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsTab {
    Personalize,
    Colors,
    Animations,
    Contrast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ShellPreferences {
    #[serde(default)]
    start_tile_animation: StartTileAnimationStyle,
    /// `None` means an older config; migrate from the formerly coupled Start style.
    #[serde(default)]
    app_launch_animation: Option<AppLaunchAnimationStyle>,
    #[serde(default)]
    start_layout_style: StartLayoutStyle,
    #[serde(default = "default_true")]
    app_switch_animation_enabled: bool,
    /// `None` preserves discovery defaults when upgrading an older config.
    #[serde(default)]
    pinned_apps: Option<Vec<String>>,
}

const fn default_true() -> bool {
    true
}

impl Default for ShellPreferences {
    fn default() -> Self {
        Self {
            start_tile_animation: StartTileAnimationStyle::default(),
            app_launch_animation: None,
            start_layout_style: StartLayoutStyle::default(),
            app_switch_animation_enabled: true,
            pinned_apps: None,
        }
    }
}

impl ShellPreferences {
    fn load() -> Self {
        let Some(path) = shell_preferences_path() else {
            return Self::default();
        };
        fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    fn save(&self) -> Result<(), String> {
        let path = shell_preferences_path()
            .ok_or_else(|| "HOME is unavailable; animation style was not saved".to_string())?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        let text = serde_json::to_string_pretty(self).map_err(|err| err.to_string())?;
        fs::write(path, text).map_err(|err| err.to_string())
    }

    fn resolved_app_launch_animation(&self) -> AppLaunchAnimationStyle {
        self.app_launch_animation
            .unwrap_or_else(|| AppLaunchAnimationStyle::from_start_style(self.start_tile_animation))
    }
}

fn shell_preferences_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".config/win10x/shell.json"))
}

impl ShellApp {
    fn new() -> Self {
        Self::build(false)
    }

    fn new_nested() -> Self {
        Self::build(true)
    }

    fn build(nested: bool) -> Self {
        let preferences = ShellPreferences::load();
        let app_launch_animation = preferences.resolved_app_launch_animation();
        let theme = Fluent1Theme::Light;
        // Match the light 10X reference: soft solid sky, not a photo wallpaper.
        let wallpaper_id = "10x-sky".to_string();
        let wallpaper_path = None;
        let wallpaper_bmp = resolve_bitmap(wallpaper_by_id(&wallpaper_id));
        let accent = Color32::from_rgb(0x00, 0x78, 0xd4);
        let wallpaper_palette = palette_from_bitmap(&wallpaper_bmp, 6);
        let mut snap = ShellSnapshot::default();
        snap.apps = apps::discover_apps();
        if let Some(pinned_apps) = preferences.pinned_apps.as_ref() {
            let pinned_apps: HashSet<_> = pinned_apps.iter().map(String::as_str).collect();
            for app in &mut snap.apps {
                app.pinned = pinned_apps.contains(app.id.as_str());
            }
        }
        let mut out = Self {
            snap,
            theme,
            accent,
            wallpaper_id,
            wallpaper_path,
            recent_wallpapers: Vec::new(),
            wallpaper_bmp,
            wallpaper_tex: None,
            accent_from_background: false,
            accent_on_surfaces: false,
            recent_accents: vec![accent],
            wallpaper_palette,
            settings_tab: SettingsTab::Personalize,
            custom_hue: 210.0,
            custom_sat: 0.85,
            custom_val: 0.83,
            contrast_idx: 0,
            pal: Palette::for_theme_accent(theme, accent),
            reveal: RevealState::default(),
            search: String::new(),
            show_all_apps: false,
            quick_wifi: true,
            quick_bt: false,
            quick_a11y: false,
            expand_wifi: false,
            expand_bt: false,
            expand_a11y: false,
            media_playing: false,
            last_error: None,
            swipe_y0: None,
            icon_shift_t: 1.0,
            nested,
            tile_board: StartTileBoard::load(),
            list_pin_drag: None,
            icon_cache: IconCache::default(),
            style_key: None,
            last_pointer: None,
            start_reveal: StartReveal::Idle,
            pending_app_launch: None,
            pending_app_switch: None,
            start_tile_animation: preferences.start_tile_animation,
            app_launch_animation,
            start_layout_style: preferences.start_layout_style,
            app_switch_animation_enabled: preferences.app_switch_animation_enabled,
            switch_animation_policy_sent: false,
            phone_apps_visible: false,
            windows8_apps_visible: false,
            start_swipe_anchor: None,
            start_apps_progress: 0.0,
            start_page_animation: None,
            ui_now: 0.0,
            session_state_path: std::env::var_os(SESSION_STATE_PATH_ENV).map(PathBuf::from),
            session_command_path: std::env::var_os(SESSION_COMMAND_PATH_ENV).map(PathBuf::from),
            last_session_poll: f64::NEG_INFINITY,
            runtime_app_ids: HashMap::new(),
            runtime_apps: HashMap::new(),
            runtime_windows: HashMap::new(),
            window_preview_cache: HashMap::new(),
            window_switch_preview_cache: HashMap::new(),
            active_app_id: None,
            taskbar_app_menu: None,
            taskbar_app_rects: HashMap::new(),
        };
        out.icon_cache.start_preload(&out.snap.apps);
        out
    }

    fn sync_compositor_title(&self, ctx: &egui::Context) {
        if !self.nested {
            return;
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(format_shell_title(
            self.snap.panel,
            self.snap.output_scale,
        )));
    }

    fn sync_compositor_session(&mut self) {
        if !self.nested || self.ui_now - self.last_session_poll < 0.20 {
            return;
        }
        self.last_session_poll = self.ui_now;
        let Some(path) = &self.session_state_path else {
            return;
        };
        let Ok(text) = fs::read_to_string(path) else {
            return;
        };
        let Ok(session) = serde_json::from_str::<CompositorSessionState>(&text) else {
            return;
        };

        self.snap
            .apps
            .iter_mut()
            .for_each(|app| app.running = false);
        let mut runtime_app_ids = HashMap::new();
        let mut runtime_apps = HashMap::new();
        let mut runtime_windows: HashMap<String, Vec<RunningApp>> = HashMap::new();
        let mut active_app_id = None;
        let mut live_transients = HashSet::new();

        for runtime in session.apps {
            let index = self
                .snap
                .apps
                .iter()
                .position(|app| app_matches_runtime_id(app, &runtime.app_id));
            let shell_id = if let Some(index) = index {
                self.snap.apps[index].running = true;
                self.snap.apps[index].id.clone()
            } else {
                let id = runtime.app_id.clone();
                self.snap.apps.push(AppEntry {
                    id: id.clone(),
                    name: runtime.title.clone(),
                    pinned: false,
                    running: true,
                    desktop_id: None,
                    exec: None,
                    icon: Some(runtime.app_id.clone()),
                });
                live_transients.insert(id.clone());
                id
            };
            if self
                .snap
                .apps
                .iter()
                .find(|app| app.id == shell_id)
                .is_some_and(|app| app.exec.is_none() && app.desktop_id.is_none())
            {
                live_transients.insert(shell_id.clone());
            }
            if runtime.active {
                active_app_id = Some(shell_id.clone());
            }
            runtime_app_ids.insert(shell_id.clone(), runtime.app_id.clone());
            runtime_windows
                .entry(shell_id.clone())
                .or_default()
                .push(runtime.clone());
            match runtime_apps.entry(shell_id) {
                std::collections::hash_map::Entry::Occupied(mut entry) if runtime.active => {
                    entry.insert(runtime);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(runtime);
                }
                _ => {}
            }
        }

        self.snap.apps.retain(|app| {
            app.exec.is_some() || app.desktop_id.is_some() || live_transients.contains(&app.id)
        });
        self.runtime_app_ids = runtime_app_ids;
        self.runtime_apps = runtime_apps;
        let live_window_ids: HashSet<_> = runtime_windows
            .values()
            .flatten()
            .map(|window| window.window_id)
            .collect();
        self.window_preview_cache
            .retain(|window_id, _| live_window_ids.contains(window_id));
        self.window_switch_preview_cache
            .retain(|window_id, _| live_window_ids.contains(window_id));
        self.runtime_windows = runtime_windows;
        self.active_app_id = active_app_id;
    }

    fn sync_switch_animation_policy(&mut self) {
        if !self.nested || self.switch_animation_policy_sent || self.session_command_path.is_none()
        {
            return;
        }
        self.send_session_command(SessionCommand::SetAppSwitchAnimationEnabled {
            enabled: self.app_switch_animation_enabled,
        });
        self.switch_animation_policy_sent = true;
    }

    fn request_app_activation(&mut self, shell_app_id: &str) {
        self.request_session_command(shell_app_id, |app_id| SessionCommand::ActivateApp {
            app_id,
        });
    }

    fn request_session_command(
        &mut self,
        shell_app_id: &str,
        command: impl FnOnce(String) -> SessionCommand,
    ) {
        let app_id = self
            .runtime_app_ids
            .get(shell_app_id)
            .cloned()
            .unwrap_or_else(|| shell_app_id.to_string());
        self.send_session_command(command(app_id));
    }

    fn send_session_command(&mut self, command: SessionCommand) {
        let Some(path) = &self.session_command_path else {
            return;
        };
        let write_result = serde_json::to_vec(&command)
            .map_err(|error| error.to_string())
            .and_then(|bytes| atomic_write(path, &bytes).map_err(|error| error.to_string()));
        if let Err(error) = write_result {
            tracing::warn!(%error, "failed to send application session command");
        }
    }

    fn refresh_palette(&mut self) {
        self.pal = Palette::for_theme_accent(self.theme, self.accent);
    }

    fn set_wallpaper(&mut self, id: &str) {
        self.wallpaper_id = id.to_string();
        let wp = wallpaper_by_id(id);
        if let Some(path) = wp.path {
            self.apply_wallpaper_file(PathBuf::from(path));
            return;
        }
        self.wallpaper_path = None;
        self.wallpaper_bmp = resolve_bitmap(wp);
        self.wallpaper_tex = None;
        self.wallpaper_palette = palette_from_bitmap(&self.wallpaper_bmp, 6);
        if self.accent_from_background {
            self.set_accent(accent_from_bitmap(&self.wallpaper_bmp));
        }
    }

    fn apply_wallpaper_file(&mut self, path: PathBuf) {
        match WallpaperBitmap::load_image(&path, 1920) {
            Ok(bmp) => {
                let path_s = path.display().to_string();
                self.wallpaper_id = "custom".into();
                self.wallpaper_path = Some(path_s.clone());
                self.wallpaper_bmp = bmp;
                self.wallpaper_tex = None;
                self.wallpaper_palette = palette_from_bitmap(&self.wallpaper_bmp, 6);
                self.recent_wallpapers.retain(|p| p != &path_s);
                self.recent_wallpapers.insert(0, path_s);
                self.recent_wallpapers.truncate(8);
                if self.accent_from_background {
                    self.set_accent(accent_from_bitmap(&self.wallpaper_bmp));
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "wallpaper load failed");
                self.last_error = Some(format!("Couldn’t open wallpaper: {e}"));
            }
        }
    }

    fn browse_wallpaper(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Choose a desktop background")
            .add_filter("Pictures", &["jpg", "jpeg", "png", "bmp", "webp"])
            .pick_file()
        {
            self.apply_wallpaper_file(path);
        }
    }

    fn wallpaper_label(&self) -> String {
        if let Some(path) = &self.wallpaper_path {
            Path::new(path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(path)
                .to_string()
        } else {
            wallpaper_by_id(&self.wallpaper_id).name.to_string()
        }
    }

    fn set_accent(&mut self, accent: Color32) {
        self.accent = accent;
        if !self.recent_accents.iter().any(|c| *c == accent) {
            self.recent_accents.insert(0, accent);
            self.recent_accents.truncate(8);
        }
        self.refresh_palette();
    }

    fn set_output_scale(&mut self, scale: f64) {
        use win10x_protocol::clamp_output_scale;
        self.snap.output_scale = clamp_output_scale(scale);
    }

    fn set_animation_preferences(
        &mut self,
        start_style: StartTileAnimationStyle,
        launch_style: AppLaunchAnimationStyle,
    ) {
        if self.start_tile_animation == start_style && self.app_launch_animation == launch_style {
            return;
        }
        self.start_tile_animation = start_style;
        self.app_launch_animation = launch_style;
        if let Err(error) = self.save_preferences() {
            tracing::warn!(%error, "shell preferences save failed");
            self.last_error = Some(format!("Couldn’t save animation settings: {error}"));
        }
    }

    fn set_start_layout_style(&mut self, style: StartLayoutStyle) {
        if self.start_layout_style == style {
            return;
        }
        self.start_layout_style = style;
        self.search.clear();
        self.phone_apps_visible = false;
        self.windows8_apps_visible = false;
        self.start_swipe_anchor = None;
        self.start_apps_progress = 0.0;
        self.start_page_animation = None;
        self.tile_board.confirm_resize();
        self.list_pin_drag = None;
        if let Err(error) = self.save_preferences() {
            tracing::warn!(%error, "shell preferences save failed");
            self.last_error = Some(format!("Couldn’t save Start layout: {error}"));
        }
    }

    fn set_app_switch_animation_enabled(&mut self, enabled: bool) {
        if self.app_switch_animation_enabled == enabled {
            return;
        }
        self.app_switch_animation_enabled = enabled;
        if !enabled {
            self.pending_app_switch = None;
            self.window_switch_preview_cache.clear();
        }
        self.send_session_command(SessionCommand::SetAppSwitchAnimationEnabled { enabled });
        self.switch_animation_policy_sent = true;
        if let Err(error) = self.save_preferences() {
            tracing::warn!(%error, "shell preferences save failed");
            self.last_error = Some(format!("Couldn’t save switch animation setting: {error}"));
        }
    }

    fn save_preferences(&self) -> Result<(), String> {
        ShellPreferences {
            start_tile_animation: self.start_tile_animation,
            app_launch_animation: Some(self.app_launch_animation),
            start_layout_style: self.start_layout_style,
            app_switch_animation_enabled: self.app_switch_animation_enabled,
            pinned_apps: Some(
                self.snap
                    .apps
                    .iter()
                    .filter(|app| app.pinned)
                    .map(|app| app.id.clone())
                    .collect(),
            ),
        }
        .save()
    }

    fn set_panel(&mut self, panel: Panel) {
        self.pending_app_switch = None;
        if panel != Panel::TaskbarAppMenu {
            self.taskbar_app_menu = None;
        }
        let now = self.ui_now;
        if panel == Panel::Launcher {
            if self.snap.panel == Panel::Launcher {
                self.begin_close_launcher(now);
            } else {
                self.open_launcher(now);
            }
            return;
        }
        self.pending_app_launch = None;
        self.start_reveal = StartReveal::Idle;
        if self.snap.panel == panel {
            self.snap.panel = Panel::None;
        } else {
            self.snap.panel = panel;
        }
        if self.snap.panel != Panel::Launcher {
            self.show_all_apps = false;
            self.start_page_animation = None;
            self.tile_board.confirm_resize();
            self.list_pin_drag = None;
        }
    }

    fn open_launcher(&mut self, now: f64) {
        self.pending_app_launch = None;
        self.pending_app_switch = None;
        self.snap.panel = Panel::Launcher;
        self.start_reveal = StartReveal::Opening { t0: now };
        self.list_pin_drag = None;
        self.phone_apps_visible = false;
        self.windows8_apps_visible = false;
        self.start_apps_progress = 0.0;
        self.start_page_animation = None;
    }

    fn begin_close_launcher(&mut self, now: f64) {
        if self.snap.panel != Panel::Launcher {
            return;
        }
        if matches!(self.start_reveal, StartReveal::Closing { .. }) {
            return;
        }
        self.start_reveal = StartReveal::Closing { t0: now };
        self.start_page_animation = None;
        self.tile_board.confirm_resize();
        self.list_pin_drag = None;
    }

    fn set_start_apps_visible(&mut self, visible: bool) {
        let current_target = match self.start_layout_style {
            StartLayoutStyle::WindowsPhone => &mut self.phone_apps_visible,
            StartLayoutStyle::Windows8 => &mut self.windows8_apps_visible,
            StartLayoutStyle::Windows10 => return,
        };
        let target = if visible { 1.0 } else { 0.0 };
        if *current_target == visible
            && self
                .start_page_animation
                .is_none_or(|animation| (animation.to - target).abs() < f32::EPSILON)
        {
            return;
        }
        *current_target = visible;
        if (self.start_apps_progress - target).abs() < 0.001 {
            self.start_apps_progress = target;
            self.start_page_animation = None;
        } else {
            self.start_page_animation = Some(StartPageAnimation {
                from: self.start_apps_progress,
                to: target,
                t0: self.ui_now,
            });
        }
        self.tile_board.confirm_resize();
        self.list_pin_drag = None;
    }

    fn tick_start_page_animation(&mut self, now: f64) {
        let Some(animation) = self.start_page_animation else {
            return;
        };
        let raw = ((now - animation.t0) / START_PAGE_SWITCH_DURATION).clamp(0.0, 1.0) as f32;
        let progress = flutter_ease_out_expo(raw);
        self.start_apps_progress = egui::lerp(animation.from..=animation.to, progress);
        if raw >= 1.0 {
            self.start_apps_progress = animation.to;
            self.start_page_animation = None;
        }
    }

    fn active_start_reveal_style(&self) -> StartTileAnimationStyle {
        if self.pending_app_launch.is_some() {
            self.app_launch_animation
                .reveal_style()
                .unwrap_or(self.start_tile_animation)
        } else if self.start_layout_style == StartLayoutStyle::Windows8 {
            // The Windows 8 layout is a source-faithful flutter_metro_ui mode,
            // including that project's grouped Start entrance.
            StartTileAnimationStyle::FlutterMetroGroup
        } else {
            self.start_tile_animation
        }
    }

    fn start_reveal_duration(&self, reveal: StartTileReveal) -> f32 {
        if reveal.style == StartTileAnimationStyle::FlutterMetroGroup && !reveal.launching_app {
            StartTileReveal::flutter_metro_start_duration(self.tile_board.group_count())
        } else {
            reveal.total_duration(
                self.tile_board.animation_row_span(),
                self.tile_board.tile_count(),
            )
        }
    }

    fn tick_start_reveal(&mut self, now: f64) {
        let reveal_style = self.active_start_reveal_style();
        let selected_tile_delay = self
            .pending_app_launch
            .as_ref()
            .filter(|pending| pending.source == AppLaunchSource::Tile)
            .and_then(|pending| {
                self.tile_board
                    .mobile_selected_launch_delay(&pending.app_id)
            });
        match self.start_reveal {
            StartReveal::Closing { t0 } => {
                let reveal = StartTileReveal {
                    elapsed: 0.0,
                    closing: true,
                    style: reveal_style,
                    launching_app: self.pending_app_launch.is_some(),
                    launching_tile: selected_tile_delay.is_some(),
                    selected_tile_delay: selected_tile_delay.unwrap_or(0.0),
                };
                if now - t0 >= self.start_reveal_duration(reveal) as f64 {
                    self.snap.panel = Panel::None;
                    self.start_reveal = StartReveal::Idle;
                    self.show_all_apps = false;
                    self.pending_app_launch = None;
                }
            }
            StartReveal::Opening { t0 } => {
                let reveal = StartTileReveal {
                    elapsed: 0.0,
                    closing: false,
                    style: reveal_style,
                    launching_app: false,
                    launching_tile: false,
                    selected_tile_delay: 0.0,
                };
                if now - t0 >= self.start_reveal_duration(reveal) as f64 {
                    self.start_reveal = StartReveal::Idle;
                }
            }
            _ => {}
        }
    }

    fn tick_app_switch(&mut self, now: f64) {
        let Some(pending) = &self.pending_app_switch else {
            return;
        };
        let duration = match pending.kind {
            AppSwitchKind::TaskbarZoom => 0.20,
            AppSwitchKind::PageSlide { .. } => 0.50,
        };
        if now - pending.t0 >= duration {
            self.pending_app_switch = None;
        }
    }

    fn start_tile_reveal(&self, now: f64) -> Option<StartTileReveal> {
        let reveal_style = self.active_start_reveal_style();
        let selected_tile_delay = self
            .pending_app_launch
            .as_ref()
            .filter(|pending| pending.source == AppLaunchSource::Tile)
            .and_then(|pending| {
                self.tile_board
                    .mobile_selected_launch_delay(&pending.app_id)
            });
        match self.start_reveal {
            StartReveal::Opening { t0 } => Some(StartTileReveal {
                elapsed: (now - t0) as f32,
                closing: false,
                style: reveal_style,
                launching_app: false,
                launching_tile: false,
                selected_tile_delay: 0.0,
            }),
            StartReveal::Closing { t0 } => Some(StartTileReveal {
                elapsed: (now - t0) as f32,
                closing: true,
                style: reveal_style,
                launching_app: self.pending_app_launch.is_some(),
                launching_tile: selected_tile_delay.is_some(),
                selected_tile_delay: selected_tile_delay.unwrap_or(0.0),
            }),
            StartReveal::Idle => None,
        }
    }

    fn begin_app_launch(
        &mut self,
        id: String,
        origin: Option<egui::Rect>,
        source: AppLaunchSource,
    ) {
        if self.snap.panel != Panel::Launcher
            || !matches!(self.start_reveal, StartReveal::Idle)
            || !self.snap.apps.iter().any(|app| app.id == id)
        {
            return;
        }
        if self.app_launch_animation == AppLaunchAnimationStyle::SystemDefault {
            if self.launch_now(&id) {
                self.snap.panel = Panel::None;
                self.start_reveal = StartReveal::Idle;
                self.show_all_apps = false;
                self.pending_app_launch = None;
                self.tile_board.confirm_resize();
                self.list_pin_drag = None;
            }
            return;
        }
        self.pending_app_launch = Some(PendingAppLaunch {
            app_id: id.clone(),
            origin,
            source,
        });
        self.begin_close_launcher(self.ui_now);
        if !self.launch_now(&id) {
            self.pending_app_launch = None;
            self.start_reveal = StartReveal::Idle;
        }
    }

    /// Start the process in the click frame so its cold start overlaps the shell transition.
    fn launch_now(&mut self, id: &str) -> bool {
        let Some(idx) = self.snap.apps.iter().position(|a| a.id == id) else {
            return false;
        };
        let app = self.snap.apps[idx].clone();
        match apps::launch(&app) {
            Ok(()) => {
                if let Some(a) = self.snap.apps.get_mut(idx) {
                    a.running = true;
                }
                self.snap.recent.insert(0, app.name.clone());
                self.snap.recent.truncate(10);
                self.last_error = None;
                self.icon_shift_t = 0.0;
                true
            }
            Err(e) => {
                tracing::error!(error = %e, name = %app.name, "launch failed");
                self.last_error = Some(format!("Couldn’t start {}: {e}", app.name));
                self.snap.panel = Panel::Launcher;
                false
            }
        }
    }

    fn tb_height(&self) -> f32 {
        self.snap.taskbar_size.height()
    }

    fn icon_px(&self) -> f32 {
        self.snap.taskbar_size.icon()
    }

    fn paint_wallpaper(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        self.paint_wallpaper_with_opacity(ui, desktop, 1.0);
    }

    fn paint_wallpaper_with_opacity(
        &mut self,
        ui: &mut egui::Ui,
        desktop: egui::Rect,
        opacity: f32,
    ) {
        let opacity = opacity.clamp(0.0, 1.0);
        if opacity <= 0.0 {
            return;
        }
        if self.wallpaper_tex.is_none() {
            let img = self.wallpaper_bmp.color_image();
            self.wallpaper_tex = Some(ui.ctx().load_texture(
                format!("wallpaper-{}", self.wallpaper_id),
                img,
                TextureOptions::LINEAR,
            ));
        }
        if let Some(tex) = &self.wallpaper_tex {
            let tw = self.wallpaper_bmp.width as f32;
            let th = self.wallpaper_bmp.height as f32;
            let scale = (desktop.width() / tw.max(1.0)).max(desktop.height() / th.max(1.0));
            let draw_w = tw * scale;
            let draw_h = th * scale;
            let x = desktop.center().x - draw_w * 0.5;
            let y = desktop.center().y - draw_h * 0.5;
            let dest = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(draw_w, draw_h));
            ui.painter().image(
                tex.id(),
                dest,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::from_white_alpha((255.0 * opacity).round() as u8),
            );
            return;
        }
        let wp = wallpaper_by_id(&self.wallpaper_id);
        ui.painter().rect_filled(
            desktop,
            0.0,
            Color32::from_rgba_unmultiplied(
                wp.top.r(),
                wp.top.g(),
                wp.top.b(),
                (wp.top.a() as f32 * opacity).round() as u8,
            ),
        );
    }
}

impl eframe::App for ShellApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh_palette();
        let style_key = (
            match self.theme {
                Fluent1Theme::Light => 0u8,
                Fluent1Theme::Dark => 1u8,
            },
            u32::from_le_bytes([
                self.accent.r(),
                self.accent.g(),
                self.accent.b(),
                self.accent.a(),
            ]),
            self.nested,
        );
        if self.style_key != Some(style_key) {
            fluent1::apply_to_ctx_ex(ui.ctx(), self.theme, &self.pal, self.nested);
            self.style_key = Some(style_key);
        }
        self.reveal.track(ui);
        self.sync_compositor_title(ui.ctx());

        // Upload a few preloaded icons each frame (non-blocking).
        let icons_busy = self.icon_cache.pump(ui.ctx());
        self.ui_now = ui.ctx().input(|i| i.time);
        self.sync_switch_animation_policy();
        self.sync_compositor_session();
        self.tick_start_reveal(self.ui_now);
        self.tick_app_switch(self.ui_now);
        self.tick_start_page_animation(self.ui_now);

        // Standalone: crisp egui scale. Nested: compositor drives wl fractional-scale.
        if !self.nested {
            let want = self.snap.output_scale as f32;
            if (ui.ctx().pixels_per_point() - want).abs() > 0.01 {
                ui.ctx().set_pixels_per_point(want);
            }
        }

        let dt = ui.ctx().input(|i| i.stable_dt).min(0.05);
        self.icon_shift_t = fluent1::spring_step(self.icon_shift_t, 1.0, dt, 12.0);

        let full = ui.max_rect();
        let tb_h = self.tb_height();
        let desktop = egui::Rect::from_min_max(full.min, egui::pos2(full.max.x, full.max.y - tb_h));
        let taskbar = egui::Rect::from_min_max(egui::pos2(full.min.x, full.max.y - tb_h), full.max);

        // Nested: desktop area stays clear so compositor wallpaper/windows show through.
        // Standalone: paint wallpaper into the shell window.
        if !self.nested {
            self.paint_wallpaper(ui, desktop);
        }

        if self.snap.panel == Panel::None {
            ui.scope_builder(UiBuilder::new().max_rect(desktop), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(desktop.height() * 0.32);
                    ui.label(
                        RichText::new("Win10X")
                            .size(40.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                    ui.label(
                        RichText::new("Tap Start · swipe up from the taskbar")
                            .size(14.0)
                            .color(Color32::from_rgba_unmultiplied(255, 255, 255, 180)),
                    );
                });
            });
        }

        match self.snap.panel {
            Panel::Launcher => self.draw_launcher(ui, desktop),
            Panel::ActionCenter => self.draw_action_center(ui, desktop),
            Panel::TaskView => self.draw_task_view(ui, desktop),
            Panel::DesktopSettings => self.draw_desktop_settings(ui, desktop),
            Panel::TaskbarAppMenu => {}
            Panel::None => {}
        }

        self.paint_pending_app_switch(ui, desktop);
        self.draw_taskbar(ui, taskbar);
        self.draw_taskbar_app_menu(ui, desktop);
        self.paint_pending_app_launch(ui, full);

        if let Some(err) = &self.last_error {
            let banner = egui::Rect::from_min_size(full.min, egui::vec2(full.width(), 36.0));
            ui.painter().rect_filled(banner, 0.0, self.pal.critical);
            ui.scope_builder(UiBuilder::new().max_rect(banner), |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(12.0);
                    ui.label(RichText::new(err.clone()).color(Color32::WHITE).size(13.0));
                });
            });
        }

        let pointer = ui.ctx().pointer_latest_pos();
        let pointer_moved = pointer != self.last_pointer;
        self.last_pointer = pointer;
        let anim_busy = (self.icon_shift_t - 1.0).abs() > 0.002
            || self.tile_board.wants_repaint()
            || self.list_pin_drag.as_ref().is_some_and(|d| d.active)
            || icons_busy
            || !matches!(self.start_reveal, StartReveal::Idle)
            || self.pending_app_switch.is_some()
            || self.start_page_animation.is_some();
        // Only spin the render loop when something is actually changing.
        if anim_busy || pointer_moved {
            ui.ctx().request_repaint();
        } else if self.nested {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(200));
        } else {
            // Keep the taskbar clock fresh without a continuous 60fps loop.
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_secs(15));
        }
    }
}

impl ShellApp {
    fn draw_taskbar(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let pal = self.pal;
        let accent = self.accent_on_surfaces.then_some(self.accent);
        fluent1::paint_taskbar(ui, rect, &pal, accent);
        self.reveal.paint_on(ui, rect, &self.pal);

        let response = ui.interact(rect, ui.id().with("taskbar"), Sense::click_and_drag());
        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.swipe_y0 = Some(pos.y);
            }
        }
        if response.dragged() {
            if let (Some(y0), Some(pos)) = (self.swipe_y0, response.interact_pointer_pos()) {
                if y0 - pos.y > 36.0 {
                    self.open_launcher(self.ui_now);
                    self.swipe_y0 = None;
                }
            }
        }
        if response.drag_stopped() {
            self.swipe_y0 = None;
        }

        let mut toggle: Option<Panel> = None;
        let mut app_click: Option<(String, egui::Rect)> = None;
        let mut open_app_menu: Option<TaskbarAppMenu> = None;
        self.taskbar_app_rects.clear();
        let mut taskbar_apps: Vec<_> = self
            .snap
            .apps
            .iter()
            .filter(|app| app.pinned || app.running)
            .cloned()
            .collect();
        taskbar_apps.sort_by_key(|app| {
            (
                !app.pinned,
                self.active_app_id.as_deref() != Some(app.id.as_str()),
                app.name.to_lowercase(),
            )
        });
        let clock = {
            use chrono::Timelike;
            let now = chrono::Local::now();
            format!("{}:{:02}", now.hour(), now.minute())
        };

        // Windows taskbar buttons occupy the full taskbar height. The glyph stays
        // centered, but hover/pressed/selected chrome must reach the screen edge.
        ui.scope_builder(
            UiBuilder::new().id_salt("taskbar_contents").max_rect(rect),
            |ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let button_extent = rect.height();
                    let tray_w = 58.0 + button_extent * 3.0;
                    let available_for_apps =
                        (rect.width() - tray_w - button_extent * 2.0 - 24.0).max(button_extent);
                    let max_apps = (available_for_apps / button_extent).floor() as usize;
                    taskbar_apps.truncate(max_apps.max(1));
                    let separator_w = if taskbar_apps.is_empty() { 0.0 } else { 9.0 };
                    let first_unpinned = taskbar_apps
                        .iter()
                        .position(|app| !app.pinned)
                        .filter(|index| *index > 0);
                    let pin_separator_w = first_unpinned.map_or(0.0, |_| 9.0);
                    let cluster = button_extent * (2 + taskbar_apps.len()) as f32
                        + separator_w
                        + pin_separator_w;
                    let left = ((ui.available_width() - cluster) * 0.5).max(8.0);
                    ui.add_space(left);

                    if taskbar_icon_button(
                        ui,
                        "start",
                        FluentIcon::Home,
                        "Start",
                        button_extent,
                        self.snap.panel == Panel::Launcher,
                        &pal,
                    )
                    .clicked()
                    {
                        toggle = Some(Panel::Launcher);
                    }
                    if taskbar_icon_button(
                        ui,
                        "task_view",
                        FluentIcon::Grid,
                        "Task View",
                        button_extent,
                        self.snap.panel == Panel::TaskView,
                        &pal,
                    )
                    .clicked()
                    {
                        toggle = Some(Panel::TaskView);
                    }

                    if !taskbar_apps.is_empty() {
                        let (_, separator) =
                            ui.allocate_space(egui::vec2(separator_w, button_extent));
                        ui.painter().vline(
                            separator.center().x,
                            separator.y_range().shrink(button_extent * 0.25),
                            Stroke::new(1.0, pal.stroke_divider),
                        );
                    }
                    for (index, app) in taskbar_apps.iter().enumerate() {
                        if first_unpinned == Some(index) {
                            let (_, separator) =
                                ui.allocate_space(egui::vec2(pin_separator_w, button_extent));
                            ui.painter().vline(
                                separator.center().x,
                                separator.y_range().shrink(button_extent * 0.25),
                                Stroke::new(1.0, pal.stroke_divider),
                            );
                        }
                        let app_response = taskbar_app_button(
                            ui,
                            &self.icon_cache,
                            app,
                            button_extent,
                            self.active_app_id.as_deref() == Some(app.id.as_str()),
                            &pal,
                        );
                        self.taskbar_app_rects
                            .insert(app.id.clone(), app_response.rect);
                        if app_response.clicked_by(egui::PointerButton::Primary) {
                            app_click = Some((app.id.clone(), app_response.rect));
                        }
                        if app_response.secondary_clicked()
                            || (app_response.dragged()
                                && app_response.drag_delta().y <= -30.0
                                && app_response.drag_delta().x.abs() < button_extent * 1.5)
                        {
                            open_app_menu = Some(TaskbarAppMenu {
                                app_id: app.id.clone(),
                                anchor: app_response.rect,
                                opened_at: self.ui_now,
                            });
                        }
                    }

                    let gap = (ui.available_width() - tray_w).max(8.0);
                    ui.add_space(gap);

                    let (_, clock_rect) = ui.allocate_space(egui::vec2(58.0, button_extent));
                    let clock_response =
                        ui.interact(clock_rect, ui.id().with("taskbar_clock"), Sense::hover());
                    ui.painter().text(
                        clock_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &clock,
                        egui::FontId::proportional(13.0),
                        pal.text,
                    );
                    clock_response.on_hover_text("Clock");
                    if taskbar_icon_button(
                        ui,
                        "network",
                        FluentIcon::Globe,
                        "Network",
                        button_extent,
                        self.snap.panel == Panel::ActionCenter,
                        &pal,
                    )
                    .clicked()
                    {
                        toggle = Some(Panel::ActionCenter);
                    }
                    if taskbar_icon_button(
                        ui,
                        "power",
                        FluentIcon::Power,
                        "Battery / power",
                        button_extent,
                        self.snap.panel == Panel::ActionCenter,
                        &pal,
                    )
                    .clicked()
                    {
                        toggle = Some(Panel::ActionCenter);
                    }
                    if taskbar_icon_button(
                        ui,
                        "action_center",
                        FluentIcon::Notification,
                        "Action Center",
                        button_extent,
                        self.snap.panel == Panel::ActionCenter,
                        &pal,
                    )
                    .clicked()
                    {
                        toggle = Some(Panel::ActionCenter);
                    }
                });
            },
        );

        // Quiet access to desktop settings: double-click empty taskbar band.
        if response.double_clicked() {
            toggle = Some(Panel::DesktopSettings);
        }

        if let Some(p) = toggle {
            self.taskbar_app_menu = None;
            self.set_panel(p);
        }
        if let Some(menu) = open_app_menu {
            self.taskbar_app_menu = Some(menu);
            self.snap.panel = Panel::TaskbarAppMenu;
        } else if let Some((id, origin)) = app_click {
            self.handle_taskbar_app_click(&id, origin);
            self.taskbar_app_menu = None;
            self.snap.panel = Panel::None;
        }
    }

    fn handle_taskbar_app_click(&mut self, app_id: &str, origin: egui::Rect) {
        let Some(runtime) = self.runtime_apps.get(app_id).cloned() else {
            let _ = self.launch_now(app_id);
            return;
        };
        let launcher_open = self.snap.panel == Panel::Launcher;
        if runtime.active && !runtime.minimized && !launcher_open {
            self.pending_app_switch = None;
            self.request_session_command(app_id, |app_id| SessionCommand::MinimizeApp { app_id });
            return;
        }

        self.begin_app_switch(app_id, runtime, origin, launcher_open);
    }

    fn begin_app_switch(
        &mut self,
        app_id: &str,
        target: RunningApp,
        origin: egui::Rect,
        from_launcher: bool,
    ) {
        let outgoing = self
            .active_app_id
            .as_ref()
            .and_then(|active_id| self.runtime_apps.get(active_id))
            .filter(|window| window.window_id != target.window_id)
            .cloned();
        let active_center_x = self
            .active_app_id
            .as_ref()
            .and_then(|active_id| self.taskbar_app_rects.get(active_id))
            .map_or(origin.center().x, |rect| rect.center().x);
        let kind = if from_launcher || outgoing.is_none() {
            AppSwitchKind::TaskbarZoom
        } else {
            AppSwitchKind::PageSlide {
                direction: if origin.center().x >= active_center_x {
                    1.0
                } else {
                    -1.0
                },
            }
        };

        self.pending_app_launch = None;
        self.start_reveal = StartReveal::Idle;
        if !self.app_switch_animation_enabled {
            self.pending_app_switch = None;
            self.send_session_command(SessionCommand::ActivateWindow {
                window_id: target.window_id,
            });
            return;
        }
        self.pending_app_switch = Some(PendingAppSwitch {
            target_app_id: app_id.to_string(),
            target: target.clone(),
            outgoing,
            origin,
            kind,
            t0: self.ui_now,
        });
        self.send_session_command(SessionCommand::ActivateWindow {
            window_id: target.window_id,
        });
    }

    fn draw_taskbar_app_menu(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        let Some(menu) = self.taskbar_app_menu.clone() else {
            if self.snap.panel == Panel::TaskbarAppMenu {
                self.snap.panel = Panel::None;
            }
            return;
        };
        let Some(app) = self
            .snap
            .apps
            .iter()
            .find(|app| app.id == menu.app_id)
            .cloned()
        else {
            self.taskbar_app_menu = None;
            self.snap.panel = Panel::None;
            return;
        };

        const HEADER_HEIGHT: f32 = 42.0;
        const ROW_HEIGHT: f32 = 42.0;
        let windows = self
            .runtime_windows
            .get(&app.id)
            .cloned()
            .unwrap_or_default();
        let show_window_previews = windows.len() > 1;
        let requested_width: f32 = if show_window_previews { 360.0 } else { 232.0 };
        let width = requested_width.min((desktop.width() - 16.0).max(160.0));
        let preview_columns = if show_window_previews && width >= 300.0 {
            2
        } else {
            1
        };
        let preview_rows = if show_window_previews {
            windows.len().div_ceil(preview_columns)
        } else {
            0
        };
        let action_height = ROW_HEIGHT * 5.0;
        let preview_available = (desktop.height() - 16.0 - HEADER_HEIGHT - action_height).max(0.0);
        let preview_cell_height = if preview_rows > 0 {
            (preview_available / preview_rows as f32).min(112.0)
        } else {
            0.0
        };
        let preview_height = preview_cell_height * preview_rows as f32;
        let height = HEADER_HEIGHT + preview_height + action_height;
        let x = (menu.anchor.center().x - width * 0.5).clamp(
            desktop.left() + 8.0,
            (desktop.right() - width - 8.0).max(desktop.left() + 8.0),
        );
        let y = (menu.anchor.top() - height - 4.0).max(desktop.top() + 8.0);
        let menu_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height));
        fluent1::paint_surface(ui, menu_rect, &self.pal);
        ui.painter().text(
            egui::pos2(
                menu_rect.left() + 14.0,
                menu_rect.top() + HEADER_HEIGHT * 0.5,
            ),
            egui::Align2::LEFT_CENTER,
            &app.name,
            egui::FontId::proportional(14.0),
            self.pal.text,
        );
        ui.painter().hline(
            menu_rect.x_range().shrink(10.0),
            menu_rect.top() + HEADER_HEIGHT,
            Stroke::new(1.0, self.pal.stroke_divider),
        );

        let mut activate_window: Option<RunningApp> = None;
        if show_window_previews {
            let gap = 6.0;
            let inset = 8.0;
            let card_width = (menu_rect.width() - inset * 2.0 - gap * (preview_columns - 1) as f32)
                / preview_columns as f32;
            for (index, window) in windows.iter().enumerate() {
                let column = index % preview_columns;
                let row = index / preview_columns;
                let card = egui::Rect::from_min_size(
                    egui::pos2(
                        menu_rect.left() + inset + (card_width + gap) * column as f32,
                        menu_rect.top() + HEADER_HEIGHT + preview_cell_height * row as f32 + 4.0,
                    ),
                    egui::vec2(card_width, (preview_cell_height - 8.0).max(1.0)),
                );
                let response = ui.interact(
                    card,
                    ui.id().with(("taskbar_window_preview", window.window_id)),
                    Sense::click(),
                );
                ui.painter().rect_filled(
                    card,
                    CornerRadius::ZERO,
                    if response.is_pointer_button_down_on() {
                        self.pal.reveal_strong
                    } else if response.hovered() || window.active {
                        self.pal.reveal
                    } else {
                        self.pal.control_secondary
                    },
                );
                let label_height = 22.0_f32.min(card.height() * 0.3);
                let image_rect = egui::Rect::from_min_max(
                    card.min + egui::vec2(4.0, 4.0),
                    egui::pos2(card.right() - 4.0, card.bottom() - label_height - 2.0),
                );
                if image_rect.is_positive() {
                    if let Some(texture) = self.window_preview_texture(ui.ctx(), window) {
                        let texture_size = texture.size_vec2();
                        let scale = (image_rect.width() / texture_size.x)
                            .min(image_rect.height() / texture_size.y);
                        let fitted =
                            egui::Rect::from_center_size(image_rect.center(), texture_size * scale);
                        ui.painter().image(
                            texture.id(),
                            fitted,
                            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    } else {
                        paint_app_icon(
                            ui,
                            &self.icon_cache,
                            &app,
                            image_rect.center(),
                            image_rect.height().min(36.0),
                            self.pal.accent,
                        );
                    }
                }
                let title_painter = ui.painter().with_clip_rect(card);
                title_painter.text(
                    egui::pos2(card.left() + 6.0, card.bottom() - label_height * 0.5),
                    egui::Align2::LEFT_CENTER,
                    &window.title,
                    egui::FontId::proportional(12.0),
                    self.pal.text,
                );
                ui.painter().rect_stroke(
                    card,
                    CornerRadius::ZERO,
                    Stroke::new(
                        if window.active { 2.0 } else { 1.0 },
                        if window.active {
                            self.pal.accent
                        } else {
                            self.pal.stroke_card
                        },
                    ),
                    StrokeKind::Inside,
                );
                if response.clicked() {
                    activate_window = Some(window.clone());
                }
            }
            ui.painter().hline(
                menu_rect.x_range().shrink(10.0),
                menu_rect.top() + HEADER_HEIGHT + preview_height,
                Stroke::new(1.0, self.pal.stroke_divider),
            );
        }

        let running = self.runtime_apps.contains_key(&app.id);
        let launchable = app.exec.is_some() || app.desktop_id.is_some();
        let rows = [
            ("Close", TaskbarAppAction::Close, running),
            (
                if app.pinned {
                    "Unpin from taskbar"
                } else {
                    "Pin to taskbar"
                },
                TaskbarAppAction::TogglePin,
                true,
            ),
            ("Open new window", TaskbarAppAction::NewWindow, launchable),
            ("Split screen", TaskbarAppAction::Split, running),
            ("Floating window", TaskbarAppAction::Float, running),
        ];
        let mut selected = None;
        for (index, (label, action, enabled)) in rows.into_iter().enumerate() {
            let row = egui::Rect::from_min_size(
                egui::pos2(
                    menu_rect.left(),
                    menu_rect.top() + HEADER_HEIGHT + preview_height + ROW_HEIGHT * index as f32,
                ),
                egui::vec2(menu_rect.width(), ROW_HEIGHT),
            );
            let response = ui.interact(
                row,
                ui.id()
                    .with(("taskbar_app_menu_action", app.id.as_str(), index)),
                if enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                },
            );
            if enabled && (response.hovered() || response.is_pointer_button_down_on()) {
                ui.painter().rect_filled(
                    row,
                    CornerRadius::ZERO,
                    if response.is_pointer_button_down_on() {
                        self.pal.reveal_strong
                    } else {
                        self.pal.reveal
                    },
                );
            }
            ui.painter().text(
                egui::pos2(row.left() + 14.0, row.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::proportional(14.0),
                if enabled {
                    self.pal.text
                } else {
                    self.pal.text_disabled
                },
            );
            if enabled && response.clicked() {
                selected = Some(action);
            }
        }

        let outside_press = self.ui_now - menu.opened_at > 0.08
            && ui.ctx().input(|input| {
                input.pointer.any_pressed()
                    && input
                        .pointer
                        .interact_pos()
                        .is_some_and(|pos| !menu_rect.contains(pos))
            });
        if outside_press || ui.ctx().input(|input| input.key_pressed(egui::Key::Escape)) {
            self.taskbar_app_menu = None;
            self.snap.panel = Panel::None;
            return;
        }

        if let Some(window) = activate_window {
            if !window.active {
                self.begin_app_switch(&app.id, window, menu.anchor, false);
            }
            self.taskbar_app_menu = None;
            self.snap.panel = Panel::None;
        } else if let Some(action) = selected {
            self.handle_taskbar_app_action(&app, action);
            self.taskbar_app_menu = None;
            self.snap.panel = Panel::None;
        }
    }

    fn window_preview_texture(
        &mut self,
        ctx: &egui::Context,
        window: &RunningApp,
    ) -> Option<TextureHandle> {
        load_window_texture(
            ctx,
            &mut self.window_preview_cache,
            window.window_id,
            window.preview_path.as_deref()?,
            "window-preview",
        )
    }

    fn window_switch_preview_texture(
        &mut self,
        ctx: &egui::Context,
        window: &RunningApp,
    ) -> Option<TextureHandle> {
        let path = window
            .transition_preview_path
            .as_deref()
            .or(window.preview_path.as_deref())?;
        load_window_texture(
            ctx,
            &mut self.window_switch_preview_cache,
            window.window_id,
            path,
            "window-switch-preview",
        )
    }

    fn handle_taskbar_app_action(&mut self, app: &AppEntry, action: TaskbarAppAction) {
        match action {
            TaskbarAppAction::Close => {
                self.request_session_command(&app.id, |app_id| SessionCommand::CloseApp { app_id })
            }
            TaskbarAppAction::TogglePin => {
                if let Some(entry) = self.snap.apps.iter_mut().find(|entry| entry.id == app.id) {
                    entry.pinned = !entry.pinned;
                }
                if let Err(error) = self.save_preferences() {
                    tracing::warn!(%error, "taskbar pins save failed");
                    self.last_error = Some(format!("Couldn’t save taskbar pins: {error}"));
                }
            }
            TaskbarAppAction::NewWindow => match apps::launch(app) {
                Ok(()) => {
                    if let Some(entry) = self.snap.apps.iter_mut().find(|entry| entry.id == app.id)
                    {
                        entry.running = true;
                    }
                }
                Err(error) => self.last_error = Some(error),
            },
            TaskbarAppAction::Split => {
                self.request_session_command(&app.id, |app_id| SessionCommand::SplitApp { app_id })
            }
            TaskbarAppAction::Float => {
                self.request_session_command(&app.id, |app_id| SessionCommand::FloatApp { app_id })
            }
        }
    }

    fn draw_desktop_settings(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        // Dim the desktop behind the opaque settings surface.
        ui.painter().rect_filled(
            desktop,
            0.0,
            if self.nested {
                Color32::from_rgba_unmultiplied(0, 0, 0, 60)
            } else {
                self.pal.overlay
            },
        );
        let card = inset_card(desktop, 20.0, 20.0, 12.0, 20.0);
        let pal = self.pal;
        fluent1::paint_surface(ui, card, &pal);
        self.reveal.paint_on(ui, card, &self.pal);

        let mut close = false;
        let mut set_theme: Option<Fluent1Theme> = None;
        let mut set_accent: Option<Color32> = None;
        let mut set_wallpaper: Option<&'static str> = None;
        let mut browse_wallpaper = false;
        let mut recent_pick: Option<String> = None;
        let mut tab = self.settings_tab;
        let mut accent_from_bg = self.accent_from_background;
        let mut accent_on = self.accent_on_surfaces;
        let mut contrast_idx = self.contrast_idx;
        let mut custom_hue = self.custom_hue;
        let mut custom_sat = self.custom_sat;
        let mut custom_val = self.custom_val;
        let mut apply_custom = false;
        let mut set_scale: Option<f64> = None;
        let mut tile_animation = self.start_tile_animation;
        let mut app_launch_animation = self.app_launch_animation;
        let mut start_layout_style = self.start_layout_style;
        let mut app_switch_animation_enabled = self.app_switch_animation_enabled;
        let output_scale = self.snap.output_scale;
        let current_wp = self.wallpaper_id.clone();
        let current_accent = self.accent;
        let recent = self.recent_accents.clone();
        let wall_pal = self.wallpaper_palette.clone();
        let wallpaper_label = self.wallpaper_label();
        let recent_walls = self.recent_wallpapers.clone();
        let has_file_wallpaper = self.wallpaper_path.is_some();
        let is_dark = matches!(self.theme, Fluent1Theme::Dark);

        ui.scope_builder(UiBuilder::new().max_rect(card), |ui| {
            ui.winui(|ui| {
                ui.column(|ui| {
                    ui.add_space(10.0);
                    ui.row(|ui| {
                        ui.label(RichText::new("Personalization").strong().size(22.0));
                        let spacer = (ui.available_width() - 40.0).max(0.0);
                        ui.add_space(spacer);
                        if ui
                            .icon_button(FluentIcon::Close)
                            .on_hover_text("Close")
                            .clicked()
                        {
                            close = true;
                        }
                    });
                    ui.add_space(8.0);
                    ui.row(|ui| {
                        if ui
                            .button(if tab == SettingsTab::Personalize {
                                "● Background"
                            } else {
                                "Background"
                            })
                            .clicked()
                        {
                            tab = SettingsTab::Personalize;
                        }
                        if ui
                            .button(if tab == SettingsTab::Colors {
                                "● Colors"
                            } else {
                                "Colors"
                            })
                            .clicked()
                        {
                            tab = SettingsTab::Colors;
                        }
                        if ui
                            .button(if tab == SettingsTab::Animations {
                                "● Animations"
                            } else {
                                "Animations"
                            })
                            .clicked()
                        {
                            tab = SettingsTab::Animations;
                        }
                        if ui
                            .button(if tab == SettingsTab::Contrast {
                                "● Contrast"
                            } else {
                                "Contrast"
                            })
                            .clicked()
                        {
                            tab = SettingsTab::Contrast;
                        }
                    });
                    ui.add_space(12.0);

                    match tab {
                        SettingsTab::Personalize => {
                            ui.label(RichText::new("Background").strong().size(15.0));
                            ui.label(
                                RichText::new(format!("Current picture: {wallpaper_label}"))
                                    .size(13.0),
                            );
                            ui.add_space(8.0);
                            ui.row(|ui| {
                                if ui.button("Browse pictures…").clicked() {
                                    browse_wallpaper = true;
                                }
                                if ui.button("Win10UI sample").clicked() {
                                    recent_pick = Some(WIN10UI_WALLPAPER_PATH.to_string());
                                }
                            });
                            if !recent_walls.is_empty() {
                                ui.add_space(10.0);
                                ui.label(RichText::new("Recent").strong().size(13.0));
                                ui.add_space(4.0);
                                for path in recent_walls.iter().take(5) {
                                    let name = Path::new(path)
                                        .file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or(path);
                                    if ui.button(name).on_hover_text(path).clicked() {
                                        recent_pick = Some(path.clone());
                                    }
                                }
                            }
                            ui.add_space(12.0);
                            ui.label(RichText::new("Suggested colors").weak().size(12.0));
                            ui.add_space(6.0);
                            ui.wrap(|ui| {
                                for wp in WALLPAPERS.iter().filter(|w| w.path.is_none()) {
                                    let selected = current_wp == wp.id && !has_file_wallpaper;
                                    let (rect, resp) = ui
                                        .allocate_exact_size(egui::vec2(88.0, 56.0), Sense::click());
                                    paint_wallpaper_swatch(
                                        ui,
                                        rect,
                                        wp.top,
                                        wp.bottom,
                                        selected,
                                        current_accent,
                                    );
                                    if resp.clicked() {
                                        set_wallpaper = Some(wp.id);
                                    }
                                    resp.on_hover_text(wp.name);
                                    ui.add_space(6.0);
                                }
                            });
                            ui.add_space(16.0);
                            ui.label(RichText::new("Choose your mode").strong().size(15.0));
                            ui.add_space(8.0);
                            ui.row(|ui| {
                                let light =
                                    mode_chip(ui, "Light", FluentIcon::Sun, !is_dark, current_accent);
                                ui.add_space(8.0);
                                let dark =
                                    mode_chip(ui, "Dark", FluentIcon::Moon, is_dark, current_accent);
                                if light.clicked() {
                                    set_theme = Some(Fluent1Theme::Light);
                                }
                                if dark.clicked() {
                                    set_theme = Some(Fluent1Theme::Dark);
                                }
                            });
                            ui.add_space(16.0);
                            ui.label(RichText::new("Display scale").strong().size(15.0));
                            ui.label(
                                RichText::new(
                                    "Uses Wayland fractional scale (niri-style) so apps redraw crisply — not a stretched screenshot.",
                                )
                                .weak()
                                .size(11.0),
                            );
                            ui.add_space(8.0);
                            let scale_pct = scale_to_percent(output_scale) as f32;
                            ui.label(
                                RichText::new(format!("Current: {scale_pct:.0}%"))
                                    .size(13.0)
                                    .color(pal.text),
                            );
                            ui.add_space(6.0);
                            ui.row(|ui| {
                                for &pct in OUTPUT_SCALE_PRESETS {
                                    let selected = scale_to_percent(output_scale) == pct;
                                    let label = format!("{pct}%");
                                    if selected {
                                        if ui.button(&label).clicked() {
                                            set_scale = Some(pct as f64 / 100.0);
                                        }
                                    } else if ui.button(label).clicked() {
                                        set_scale = Some(pct as f64 / 100.0);
                                    }
                                }
                            });
                            ui.add_space(8.0);
                            ui.row(|ui| {
                                if ui.button("−").on_hover_text("Decrease 25%").clicked() {
                                    set_scale = Some(output_scale - 0.25);
                                }
                                if ui.button("+").on_hover_text("Increase 25%").clicked() {
                                    set_scale = Some(output_scale + 0.25);
                                }
                                if ui.button("Reset 100%").clicked() {
                                    set_scale = Some(1.0);
                                }
                            });
                        }
                        SettingsTab::Colors => {
                            ui.toggle(
                                &mut accent_from_bg,
                                "Automatically pick an accent color from my background",
                            );
                            ui.add_space(6.0);
                            if !wall_pal.is_empty() {
                                ui.label(
                                    RichText::new("Colors from background (Color Thief)")
                                        .strong()
                                        .size(13.0),
                                );
                                ui.add_space(6.0);
                                ui.wrap(|ui| {
                                    for c in &wall_pal {
                                        let (rect, resp) = ui.allocate_exact_size(
                                            egui::vec2(32.0, 32.0),
                                            Sense::click(),
                                        );
                                        paint_swatch(ui, rect, *c, current_accent == *c);
                                        if resp.clicked() {
                                            set_accent = Some(*c);
                                            accent_from_bg = false;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            }
                            ui.add_space(12.0);
                            ui.label(RichText::new("Windows colors").strong().size(15.0));
                            ui.add_space(6.0);
                            ui.wrap(|ui| {
                                for c in WIN10_WINDOWS_COLORS {
                                    let (rect, resp) =
                                        ui.allocate_exact_size(egui::vec2(28.0, 28.0), Sense::click());
                                    paint_swatch(ui, rect, *c, current_accent == *c);
                                    if resp.clicked() {
                                        set_accent = Some(*c);
                                        accent_from_bg = false;
                                    }
                                    ui.add_space(4.0);
                                }
                            });
                            if !recent.is_empty() {
                                ui.add_space(12.0);
                                ui.label(RichText::new("Recent colors").strong().size(13.0));
                                ui.add_space(6.0);
                                ui.wrap(|ui| {
                                    for c in &recent {
                                        let (rect, resp) = ui.allocate_exact_size(
                                            egui::vec2(28.0, 28.0),
                                            Sense::click(),
                                        );
                                        paint_swatch(ui, rect, *c, current_accent == *c);
                                        if resp.clicked() {
                                            set_accent = Some(*c);
                                            accent_from_bg = false;
                                        }
                                        ui.add_space(4.0);
                                    }
                                });
                            }
                            ui.add_space(14.0);
                            ui.label(RichText::new("Custom color").strong().size(15.0));
                            ui.add_space(4.0);
                            ui.label(RichText::new("Hue").size(12.0));
                            ui.add(
                                egui::Slider::new(&mut custom_hue, 0.0..=360.0).show_value(true),
                            );
                            ui.label(RichText::new("Saturation").size(12.0));
                            ui.add(egui::Slider::new(&mut custom_sat, 0.0..=1.0));
                            ui.label(RichText::new("Value").size(12.0));
                            ui.add(egui::Slider::new(&mut custom_val, 0.0..=1.0));
                            let preview = hsv_to_rgb(custom_hue, custom_sat, custom_val);
                            ui.row(|ui| {
                                let (rect, _) =
                                    ui.allocate_exact_size(egui::vec2(48.0, 28.0), Sense::hover());
                                ui.painter().rect_filled(rect, 0.0, preview);
                                if ui.button("Done").clicked() {
                                    apply_custom = true;
                                    accent_from_bg = false;
                                }
                            });
                            ui.add_space(10.0);
                            ui.toggle(
                                &mut accent_on,
                                "Show accent color on Start, taskbar, and action center",
                            );
                        }
                        SettingsTab::Animations => {
                            ui.label(RichText::new("Start layout").strong().size(15.0));
                            ui.label(
                                RichText::new(
                                    "Choose the navigation and scrolling model. Windows 8 also uses flutter_metro_ui's matching Start motion.",
                                )
                                .weak()
                                .size(12.0),
                            );
                            ui.add_space(10.0);
                            for style in StartLayoutStyle::ALL {
                                let selected = start_layout_style == style;
                                if ui
                                    .button(if selected {
                                        format!("● {}", style.label())
                                    } else {
                                        style.label().to_string()
                                    })
                                    .clicked()
                                {
                                    start_layout_style = style;
                                }
                                ui.label(
                                    RichText::new(style.description()).weak().size(12.0),
                                );
                                ui.add_space(8.0);
                            }
                            ui.add_space(6.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Start screen animation")
                                    .strong()
                                    .size(15.0),
                            );
                            ui.label(
                                RichText::new(
                                    if start_layout_style == StartLayoutStyle::Windows8 {
                                        "Windows 8 layout always uses flutter_metro_ui's source group sweep (1.05 s) and delayed header (1.5 s). Your saved choice remains available for the other layouts."
                                    } else {
                                        "Controls only how pinned tiles enter Start and how Start closes without launching an app."
                                    },
                                )
                                .weak()
                                .size(12.0),
                            );
                            ui.add_space(12.0);
                            if start_layout_style == StartLayoutStyle::Windows8 {
                                ui.label(format!(
                                    "● {}",
                                    StartTileAnimationStyle::FlutterMetroGroup.label()
                                ));
                                ui.label(
                                    RichText::new(
                                        StartTileAnimationStyle::FlutterMetroGroup.description(),
                                    )
                                    .weak()
                                    .size(12.0),
                                );
                                ui.add_space(10.0);
                            } else {
                                for style in StartTileAnimationStyle::ALL {
                                    let selected = tile_animation == style;
                                    if ui
                                        .button(if selected {
                                            format!("● {}", style.label())
                                        } else {
                                            style.label().to_string()
                                        })
                                        .clicked()
                                    {
                                        tile_animation = style;
                                    }
                                    ui.label(
                                        RichText::new(style.description())
                                            .weak()
                                            .size(12.0),
                                    );
                                    ui.add_space(10.0);
                                }
                            }
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Application launch animation")
                                    .strong()
                                    .size(15.0),
                            );
                            ui.label(
                                RichText::new(
                                    "Controls the visual handoff after tapping a tile or app icon. Application loading always begins immediately.",
                                )
                                .weak()
                                .size(12.0),
                            );
                            ui.add_space(12.0);
                            for style in AppLaunchAnimationStyle::ALL {
                                let selected = app_launch_animation == style;
                                if ui
                                    .button(if selected {
                                        format!("● {}", style.label())
                                    } else {
                                        style.label().to_string()
                                    })
                                    .clicked()
                                {
                                    app_launch_animation = style;
                                }
                                ui.label(
                                    RichText::new(style.description())
                                        .weak()
                                        .size(12.0),
                                );
                                ui.add_space(10.0);
                            }
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Application switch animation")
                                    .strong()
                                    .size(15.0),
                            );
                            ui.toggle(
                                &mut app_switch_animation_enabled,
                                "Animate switching between running applications",
                            );
                            ui.label(
                                RichText::new(if app_switch_animation_enabled {
                                    "Keeps quarter-resolution switch snapshots ready in the background."
                                } else {
                                    "Switches immediately and stops background GPU snapshot capture and PNG encoding. Small previews are captured only while a multi-window menu is open."
                                })
                                .weak()
                                .size(12.0),
                            );
                            ui.add_space(4.0);
                            ui.label(
                                RichText::new(
                                    "Close Settings, open Start, then tap an app to preview the launch selection.",
                                )
                                .size(12.0)
                                .color(pal.accent),
                            );
                        }
                        SettingsTab::Contrast => {
                            ui.label(
                                RichText::new("High contrast themes")
                                    .strong()
                                    .size(15.0),
                            );
                            ui.label(
                                RichText::new(
                                    "Preview like Win10 Ease of Access → High contrast",
                                )
                                .weak()
                                .size(12.0),
                            );
                            ui.add_space(10.0);
                            for (i, theme) in CONTRAST_THEMES.iter().enumerate() {
                                let selected = contrast_idx == i;
                                if ui
                                    .button(if selected {
                                        format!("● {}", theme.name)
                                    } else {
                                        theme.name.to_string()
                                    })
                                    .clicked()
                                {
                                    contrast_idx = i;
                                }
                            }
                            ui.add_space(12.0);
                            let theme = CONTRAST_THEMES[contrast_idx.min(CONTRAST_THEMES.len() - 1)];
                            paint_contrast_preview(ui, theme, current_accent);
                            ui.add_space(10.0);
                            ui.label(RichText::new("Accent contrast checks").strong().size(13.0));
                            let pairs = [
                                ("Text on accent", Color32::WHITE, current_accent),
                                ("Accent on dark surface", current_accent, Color32::from_rgb(32, 32, 32)),
                                ("Accent on light surface", current_accent, Color32::from_rgb(243, 243, 243)),
                            ];
                            for (label, fg, bg) in pairs {
                                let ratio = contrast_ratio(fg, bg);
                                let pass = if ratio >= 4.5 {
                                    "AA ✓"
                                } else if ratio >= 3.0 {
                                    "Large AA"
                                } else {
                                    "Fail"
                                };
                                ui.label(format!("{label}: {ratio:.2}:1 · {pass}"));
                            }
                        }
                    }
                });
            });
        });

        self.settings_tab = tab;
        self.custom_hue = custom_hue;
        self.custom_sat = custom_sat;
        self.custom_val = custom_val;
        self.contrast_idx = contrast_idx;

        if tile_animation != self.start_tile_animation
            || app_launch_animation != self.app_launch_animation
        {
            self.set_animation_preferences(tile_animation, app_launch_animation);
        }
        if start_layout_style != self.start_layout_style {
            self.set_start_layout_style(start_layout_style);
        }
        if app_switch_animation_enabled != self.app_switch_animation_enabled {
            self.set_app_switch_animation_enabled(app_switch_animation_enabled);
        }

        if accent_on != self.accent_on_surfaces {
            self.accent_on_surfaces = accent_on;
        }
        if accent_from_bg != self.accent_from_background {
            self.accent_from_background = accent_from_bg;
            if accent_from_bg {
                self.set_accent(accent_from_bitmap(&self.wallpaper_bmp));
            }
        }
        if let Some(id) = set_wallpaper {
            self.set_wallpaper(id);
        }
        if let Some(path) = recent_pick {
            self.apply_wallpaper_file(PathBuf::from(path));
        }
        if browse_wallpaper {
            self.browse_wallpaper();
        }
        if let Some(s) = set_scale {
            self.set_output_scale(s);
        }
        if let Some(c) = set_accent {
            self.set_accent(c);
        }
        if apply_custom {
            self.set_accent(hsv_to_rgb(
                self.custom_hue,
                self.custom_sat,
                self.custom_val,
            ));
        }
        if let Some(t) = set_theme {
            self.theme = t;
            self.refresh_palette();
        }
        if close {
            self.snap.panel = Panel::None;
            self.start_reveal = StartReveal::Idle;
        }
    }

    /// Full-scene background crossfade, synchronized with the final delayed tile.
    fn start_scene_progress(&self) -> f32 {
        self.start_tile_reveal(self.ui_now)
            .map(|reveal| {
                if self.start_layout_style == StartLayoutStyle::Windows8
                    && reveal.style == StartTileAnimationStyle::FlutterMetroGroup
                    && !reveal.launching_app
                {
                    // flutter_metro_ui wraps Start in a 300 ms AnimatedOpacity.
                    let raw = (reveal.elapsed / 0.3).clamp(0.0, 1.0);
                    if reveal.closing {
                        1.0 - raw
                    } else {
                        raw
                    }
                } else if reveal.style == StartTileAnimationStyle::FlutterMetroGroup
                    && !reveal.launching_app
                {
                    let duration = StartTileReveal::flutter_metro_start_duration(
                        self.tile_board.group_count(),
                    );
                    let progress = flutter_ease_out_expo(
                        (reveal.elapsed / duration.max(f32::EPSILON)).clamp(0.0, 1.0),
                    );
                    if reveal.closing {
                        1.0 - progress
                    } else {
                        progress
                    }
                } else {
                    reveal.scene_progress(
                        self.tile_board.animation_row_span(),
                        self.tile_board.tile_count(),
                    )
                }
            })
            .unwrap_or(1.0)
    }

    fn paint_start_backdrop_crossfade(
        &mut self,
        ui: &mut egui::Ui,
        desktop: egui::Rect,
        progress: f32,
    ) {
        const WHITE_VEIL_ALPHA: f32 = 150.0;
        let p = progress.clamp(0.0, 1.0);
        if p <= 0.0 {
            return;
        }

        // In nested mode this fades the current app out under the wallpaper;
        // standalone mode already has the same wallpaper below this layer.
        self.paint_wallpaper_with_opacity(ui, desktop, p);
        ui.painter().rect_filled(
            desktop,
            0.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, (WHITE_VEIL_ALPHA * p) as u8),
        );
    }

    fn paint_pending_app_switch(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        let Some(pending) = self.pending_app_switch.clone() else {
            return;
        };
        let elapsed = (self.ui_now - pending.t0).max(0.0) as f32;
        match pending.kind {
            AppSwitchKind::TaskbarZoom => {
                let raw = (elapsed / 0.20).clamp(0.0, 1.0);
                let progress = 1.0 - (1.0 - raw).powi(2);
                let surface = lerp_rect(pending.origin, desktop, progress);
                self.paint_window_switch_snapshot(
                    ui,
                    &pending.target,
                    &pending.target_app_id,
                    surface,
                    desktop,
                );
            }
            AppSwitchKind::PageSlide { direction } => {
                let progress = (elapsed / 0.50).clamp(0.0, 1.0);
                ui.painter().with_clip_rect(desktop).rect_filled(
                    desktop,
                    CornerRadius::ZERO,
                    Color32::from_rgb(0x1d, 0x1d, 0x1d),
                );

                if let Some(outgoing) = &pending.outgoing {
                    // AnimatedSwitcher drives the exiting child's animation from
                    // 1 back to 0 through Interval(0.4, 1.0).
                    let reverse_interval = ((1.0 - progress - 0.4) / 0.6).clamp(0.0, 1.0);
                    let scale = 0.8 + 0.2 * ease_out_cubic(reverse_interval);
                    let rect =
                        egui::Rect::from_center_size(desktop.center(), desktop.size() * scale);
                    self.paint_window_switch_snapshot(
                        ui,
                        outgoing,
                        &outgoing.app_id,
                        rect,
                        desktop,
                    );
                }

                let slide = ease_out_cubic(((progress - 0.4) / 0.3).clamp(0.0, 1.0));
                let grow = ease_out_cubic(((progress - 0.7) / 0.3).clamp(0.0, 1.0));
                let scale = 0.5 + 0.5 * grow;
                let center = desktop.center()
                    + egui::vec2(direction * desktop.width() * 1.5 * (1.0 - slide), 0.0);
                let rect = egui::Rect::from_center_size(center, desktop.size() * scale);
                self.paint_window_switch_snapshot(
                    ui,
                    &pending.target,
                    &pending.target_app_id,
                    rect,
                    desktop,
                );
            }
        }
    }

    fn paint_window_switch_snapshot(
        &mut self,
        ui: &mut egui::Ui,
        window: &RunningApp,
        shell_app_id: &str,
        rect: egui::Rect,
        clip: egui::Rect,
    ) {
        let painter = ui.painter().with_clip_rect(clip);
        painter.rect_filled(
            rect,
            CornerRadius::ZERO,
            Color32::from_rgb(0x1d, 0x1d, 0x1d),
        );
        if let Some(texture) = self.window_switch_preview_texture(ui.ctx(), window) {
            painter.image(
                texture.id(),
                rect,
                cover_uv(texture.size_vec2(), rect.size()),
                Color32::WHITE,
            );
            return;
        }

        let fallback = self
            .snap
            .apps
            .iter()
            .find(|app| app.id == shell_app_id || app_matches_runtime_id(app, &window.app_id))
            .cloned();
        if let Some(app) = fallback {
            paint_app_icon(
                ui,
                &self.icon_cache,
                &app,
                rect.center(),
                rect.width()
                    .min(rect.height())
                    .mul_add(0.18, 0.0)
                    .clamp(24.0, 72.0),
                self.pal.accent,
            );
        }
    }

    /// Paint source-aware app handoff proxies after taskbar chrome. The Linux
    /// process already started in the click frame; these surfaces cover its cold
    /// start until the transition completes.
    fn paint_pending_app_launch(&self, ui: &mut egui::Ui, full: egui::Rect) {
        let Some(reveal) = self.start_tile_reveal(self.ui_now) else {
            return;
        };
        let Some(pending) = &self.pending_app_launch else {
            return;
        };
        let Some(app) = self.snap.apps.iter().find(|app| app.id == pending.app_id) else {
            return;
        };

        let fallback = egui::Rect::from_center_size(full.center(), egui::vec2(48.0, 48.0));
        let source = pending
            .origin
            .map(|origin| origin.intersect(full))
            .filter(|origin| origin.is_positive())
            .unwrap_or(fallback);

        if let Some(frame) = reveal.windows8_launch_frame() {
            paint_windows8_launch_tile(ui, &self.icon_cache, app, source, full, frame, &self.pal);
            return;
        }

        let Some(progress) = reveal.factor_clip_progress() else {
            return;
        };

        let lerp_pos = |from: egui::Pos2, to: egui::Pos2| {
            egui::pos2(
                egui::lerp(from.x..=to.x, progress),
                egui::lerp(from.y..=to.y, progress),
            )
        };
        let surface = egui::Rect::from_min_max(
            lerp_pos(source.min, full.min),
            lerp_pos(source.max, full.max),
        );

        ui.painter()
            .rect_filled(surface, CornerRadius::ZERO, self.pal.bg);
        ui.painter().rect_stroke(
            surface,
            CornerRadius::ZERO,
            Stroke::new(1.0, self.pal.stroke_card),
            StrokeKind::Inside,
        );

        let icon_center = lerp_pos(source.center(), full.center());
        let source_icon = (source.width().min(source.height()) * 0.38).clamp(20.0, 52.0);
        let icon_size = egui::lerp(source_icon..=72.0, progress);
        paint_app_icon(
            ui,
            &self.icon_cache,
            app,
            icon_center,
            icon_size,
            self.pal.accent,
        );
    }

    fn draw_launcher(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        // One persisted tile collection rendered through three navigation models.
        let pal = self.pal;
        let layout_style = self.start_layout_style;
        let tile_flow = layout_style.tile_flow();
        let reveal = self.start_tile_reveal(self.ui_now);
        let scene_progress = self.start_scene_progress();
        if scene_progress < 0.002 {
            return;
        }
        let content_opacity = reveal
            .map(|reveal| reveal.launcher_content_opacity(self.tile_board.animation_row_span()))
            .unwrap_or(1.0);

        self.paint_start_backdrop_crossfade(ui, desktop, scene_progress);
        if layout_style == StartLayoutStyle::Windows8 {
            ui.painter().rect_filled(
                desktop,
                CornerRadius::ZERO,
                Color32::from_rgba_unmultiplied(0x1d, 0x1d, 0x1d, (255.0 * scene_progress) as u8),
            );
        }

        let (pointer_pressed, pointer_released, pointer_pos) = ui.input(|input| {
            (
                input.pointer.primary_pressed(),
                input.pointer.primary_released(),
                input.pointer.interact_pos(),
            )
        });
        if pointer_pressed && pointer_pos.is_some_and(|position| desktop.contains(position)) {
            self.start_swipe_anchor = pointer_pos;
        }
        if pointer_released {
            let delta = self
                .start_swipe_anchor
                .take()
                .zip(pointer_pos)
                .map_or(egui::Vec2::ZERO, |(start, end)| end - start);
            let tile_interaction = self.tile_board.wants_repaint() || self.list_pin_drag.is_some();
            if delta.x.abs() > delta.y.abs() * 1.25 && delta.x.abs() > 56.0 {
                if layout_style == StartLayoutStyle::WindowsPhone && !tile_interaction {
                    self.set_start_apps_visible(delta.x < 0.0);
                }
            } else if delta.y.abs() > delta.x.abs() * 1.25 && delta.y.abs() > 56.0 {
                if layout_style == StartLayoutStyle::Windows8 && !tile_interaction {
                    self.set_start_apps_visible(delta.y < 0.0);
                }
            }
        }

        // Rail tint follows the same whole-scene crossfade.
        let chrome = scene_progress;
        let apps_progress = self.start_apps_progress.clamp(0.0, 1.0);

        let show_rail = layout_style == StartLayoutStyle::Windows10;
        let show_standard_list = layout_style == StartLayoutStyle::Windows10
            || (layout_style == StartLayoutStyle::WindowsPhone && apps_progress > 0.001);
        let show_windows8_apps =
            layout_style == StartLayoutStyle::Windows8 && apps_progress > 0.001;
        let show_tiles = match layout_style {
            StartLayoutStyle::Windows10 => true,
            StartLayoutStyle::WindowsPhone | StartLayoutStyle::Windows8 => apps_progress < 0.999,
        };
        let show_search = show_standard_list;
        let rail_expanded = show_rail && self.show_all_apps;
        let rail_w = if rail_expanded { 168.0 } else { 48.0 };
        let list_w = (desktop.width() * 0.22).clamp(220.0, 300.0);
        let search_h = 44.0;
        let pad = 8.0;

        let (rail, mut search, mut list, mut tiles) = match layout_style {
            StartLayoutStyle::Windows10 => {
                let rail = egui::Rect::from_min_max(
                    desktop.min,
                    egui::pos2(desktop.min.x + rail_w, desktop.max.y),
                );
                let search = egui::Rect::from_min_max(
                    egui::pos2(rail.max.x + pad, desktop.max.y - search_h - 8.0),
                    egui::pos2(rail.max.x + pad + list_w, desktop.max.y - 8.0),
                );
                let list = egui::Rect::from_min_max(
                    egui::pos2(rail.max.x + pad, desktop.min.y + 12.0),
                    egui::pos2(rail.max.x + pad + list_w, search.min.y - 8.0),
                );
                let tiles = egui::Rect::from_min_max(
                    egui::pos2(list.max.x + 16.0, desktop.min.y + 20.0),
                    egui::pos2(desktop.max.x - 20.0, desktop.max.y - 16.0),
                );
                (rail, search, list, tiles)
            }
            StartLayoutStyle::WindowsPhone => {
                let page = desktop.shrink2(egui::vec2(28.0, 16.0));
                let search = egui::Rect::from_min_max(
                    egui::pos2(page.min.x, page.max.y - search_h),
                    page.max,
                );
                let list = egui::Rect::from_min_max(
                    egui::pos2(page.min.x, page.min.y + 38.0),
                    egui::pos2(page.max.x, search.min.y - 10.0),
                );
                (egui::Rect::NOTHING, search, list, page)
            }
            StartLayoutStyle::Windows8 => {
                let app_page = egui::Rect::from_min_max(
                    egui::pos2(desktop.min.x + 72.0, desktop.min.y + 108.0),
                    egui::pos2(desktop.max.x - 12.0, desktop.max.y - 54.0),
                );
                (egui::Rect::NOTHING, egui::Rect::NOTHING, app_page, desktop)
            }
        };
        match layout_style {
            StartLayoutStyle::Windows10 => {}
            StartLayoutStyle::WindowsPhone => {
                let tile_offset = egui::vec2(-desktop.width() * apps_progress, 0.0);
                let apps_offset = egui::vec2(desktop.width() * (1.0 - apps_progress), 0.0);
                tiles = tiles.translate(tile_offset);
                list = list.translate(apps_offset);
                search = search.translate(apps_offset);
            }
            StartLayoutStyle::Windows8 => {
                let tile_offset = egui::vec2(0.0, -desktop.height() * apps_progress);
                let apps_offset = egui::vec2(0.0, desktop.height() * (1.0 - apps_progress));
                tiles = tiles.translate(tile_offset);
                list = list.translate(apps_offset);
            }
        }

        let q = self.search.to_lowercase();
        let apps_all: Vec<_> = {
            let mut v: Vec<_> = self.snap.apps.iter().cloned().collect();
            v.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            v
        };
        let apps_filtered: Vec<_> = apps_all
            .iter()
            .filter(|a| q.is_empty() || a.name.to_lowercase().contains(&q))
            .cloned()
            .collect();
        let mut launch_id: Option<String> = None;
        let mut launch_origin: Option<egui::Rect> = None;
        let mut launch_source = AppLaunchSource::Rail;
        let mut pin_id: Option<String> = None;
        let mut unpin_id: Option<String> = None;
        let mut open_settings = false;
        let mut close = false;
        let mut toggle_rail = false;

        // —— Left rail ——
        let base_ui_opacity = ui.opacity();
        ui.set_opacity(base_ui_opacity * content_opacity);
        if show_rail {
            ui.painter().rect_filled(
                rail,
                0.0,
                Color32::from_rgba_unmultiplied(
                    255,
                    255,
                    255,
                    ((if rail_expanded { 40.0 } else { 24.0 }) * chrome) as u8,
                ),
            );
            let icon_btn = |ui: &mut egui::Ui,
                            _id: &str,
                            glyph: FluentIcon,
                            label: &str,
                            expanded: bool|
             -> egui::Response {
                let h = 44.0;
                let (rect, resp) =
                    ui.allocate_exact_size(egui::vec2(ui.available_width(), h), Sense::click());
                if resp.hovered() {
                    ui.painter().rect_filled(
                        rect.shrink2(egui::vec2(4.0, 2.0)),
                        CornerRadius::same(2),
                        Color32::from_rgba_unmultiplied(0, 0, 0, 18),
                    );
                }
                let icon_c = egui::pos2(rect.min.x + 24.0, rect.center().y);
                ui.painter().text(
                    icon_c,
                    egui::Align2::CENTER_CENTER,
                    glyph.glyph(),
                    fluent_egui::winui::icon_font_id(18.0),
                    pal.text,
                );
                if expanded {
                    ui.painter().text(
                        egui::pos2(rect.min.x + 48.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        egui::FontId::proportional(13.0),
                        pal.text,
                    );
                }
                resp.on_hover_text(label)
            };

            ui.scope_builder(UiBuilder::new().max_rect(rail), |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    if icon_btn(
                        ui,
                        "hamburger",
                        FluentIcon::List,
                        if rail_expanded { "Collapse" } else { "Expand" },
                        rail_expanded,
                    )
                    .clicked()
                    {
                        toggle_rail = true;
                    }
                    let spacer = (ui.available_height() - 44.0 * 5.0 - 16.0).max(8.0);
                    ui.add_space(spacer);
                    let _ = icon_btn(ui, "user", FluentIcon::Person, "User", rail_expanded);
                    if icon_btn(ui, "docs", FluentIcon::Folder, "Documents", rail_expanded)
                        .clicked()
                    {
                        // Best-effort: open file manager if present.
                        if let Some(a) = self.snap.apps.iter().find(|a| {
                            let k = a.name.to_lowercase();
                            k.contains("files") || k.contains("file") || a.id.contains("nautilus")
                        }) {
                            launch_id = Some(a.id.clone());
                            launch_source = AppLaunchSource::Rail;
                        }
                    }
                    if icon_btn(
                        ui,
                        "settings",
                        FluentIcon::Settings,
                        "Settings",
                        rail_expanded,
                    )
                    .clicked()
                    {
                        open_settings = true;
                    }
                    if icon_btn(ui, "power", FluentIcon::Power, "Power", rail_expanded).clicked() {
                        close = true;
                    }
                    ui.add_space(8.0);
                });
            });
        }

        let disco_list_reveal = self
            .pending_app_launch
            .as_ref()
            .filter(|pending| pending.source == AppLaunchSource::AppList)
            .and_then(|_| self.start_tile_reveal(self.ui_now));
        let disco_list_selected_id = self
            .pending_app_launch
            .as_ref()
            .filter(|pending| pending.source == AppLaunchSource::AppList)
            .map(|pending| pending.app_id.clone());
        let disco_list_count = apps_filtered.len().max(1);
        let disco_list_exit = desktop.width().min(1000.0) / 3.0;
        let project_disco_list_x = |x: f32, progress: f32| {
            let scale_x = (std::f32::consts::FRAC_PI_2 * progress).cos().max(0.0);
            desktop.min.x + (x - desktop.min.x) * scale_x - disco_list_exit * progress
        };
        let launcher_transitioning =
            !matches!(self.start_reveal, StartReveal::Idle) || self.start_page_animation.is_some();

        if layout_style == StartLayoutStyle::WindowsPhone {
            if show_tiles {
                ui.painter().text(
                    egui::pos2(
                        desktop.min.x + 28.0 - desktop.width() * apps_progress,
                        desktop.min.y + 24.0,
                    ),
                    egui::Align2::LEFT_CENTER,
                    "Start",
                    egui::FontId::proportional(28.0),
                    pal.text,
                );
            }
            if show_standard_list {
                ui.painter().text(
                    egui::pos2(list.min.x, desktop.min.y + 24.0),
                    egui::Align2::LEFT_CENTER,
                    "All apps",
                    egui::FontId::proportional(28.0),
                    pal.text,
                );
            }
        } else if layout_style == StartLayoutStyle::Windows8 {
            if show_tiles {
                let header_progress = reveal.map_or(1.0, |frame| {
                    if frame.style == StartTileAnimationStyle::FlutterMetroGroup {
                        frame.flutter_metro_header_progress()
                    } else {
                        1.0
                    }
                });
                let header_opacity = header_progress * content_opacity;
                let page_y = -desktop.height() * apps_progress;
                let header_x = 50.0 * (1.0 - header_progress);
                let header_color = Color32::from_white_alpha(
                    (255.0 * header_opacity.clamp(0.0, 1.0)).round() as u8,
                );
                ui.painter().text(
                    egui::pos2(
                        desktop.min.x + 100.0 + header_x,
                        desktop.min.y + 40.0 + page_y,
                    ),
                    egui::Align2::LEFT_TOP,
                    "开始",
                    egui::FontId::proportional(56.0),
                    header_color,
                );
                let avatar = egui::Rect::from_min_size(
                    egui::pos2(
                        desktop.max.x - 60.0 - 50.0 + header_x,
                        desktop.min.y + 60.0 + page_y,
                    ),
                    egui::vec2(50.0, 50.0),
                );
                ui.painter().rect_filled(
                    avatar,
                    CornerRadius::ZERO,
                    Color32::from_black_alpha(
                        (255.0 * header_opacity.clamp(0.0, 1.0)).round() as u8
                    ),
                );
                ui.painter().text(
                    egui::pos2(avatar.min.x - 12.0, avatar.center().y),
                    egui::Align2::RIGHT_CENTER,
                    "Metro UI",
                    egui::FontId::proportional(24.0),
                    header_color,
                );
                ui.painter().text(
                    avatar.center(),
                    egui::Align2::CENTER_CENTER,
                    FluentIcon::Person.glyph(),
                    fluent_egui::winui::icon_font_id(34.0),
                    header_color,
                );
            }
            if show_windows8_apps {
                ui.painter().text(
                    egui::pos2(
                        desktop.min.x + 100.0,
                        desktop.min.y + 40.0 + desktop.height() * (1.0 - apps_progress),
                    ),
                    egui::Align2::LEFT_TOP,
                    "应用",
                    egui::FontId::proportional(56.0),
                    Color32::WHITE,
                );
            }
        }

        // —— Alphabetical app list ——
        if show_standard_list {
            ui.scope_builder(UiBuilder::new().max_rect(list), |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("start_app_list")
                    .show(ui, |ui| {
                        let mut last_letter: Option<char> = None;
                        for (app_index, app) in apps_filtered.iter().enumerate() {
                            let letter = app
                                .name
                                .chars()
                                .next()
                                .map(|c| c.to_ascii_uppercase())
                                .unwrap_or('#');
                            let letter = if letter.is_ascii_alphabetic() {
                                letter
                            } else {
                                '#'
                            };
                            if last_letter != Some(letter) {
                                last_letter = Some(letter);
                                ui.add_space(6.0);
                                let (_, letter_rect) =
                                    ui.allocate_space(egui::vec2(ui.available_width(), 16.0));
                                let letter_frame = disco_list_reveal.and_then(|reveal| {
                                    reveal.disco_app_list_frame(app_index, disco_list_count, false)
                                });
                                if letter_frame.is_none_or(|frame| frame.visible) {
                                    let (x, scale) = letter_frame
                                        .map(|frame| {
                                            (
                                                project_disco_list_x(
                                                    letter_rect.min.x,
                                                    frame.progress,
                                                ) - desktop.width() * frame.progress,
                                                (std::f32::consts::FRAC_PI_2 * frame.progress)
                                                    .cos()
                                                    .max(0.12),
                                            )
                                        })
                                        .unwrap_or((letter_rect.min.x, 1.0));
                                    ui.painter().text(
                                        egui::pos2(x, letter_rect.center().y),
                                        egui::Align2::LEFT_CENTER,
                                        letter.to_string(),
                                        egui::FontId::proportional(13.0 * scale),
                                        pal.text_secondary,
                                    );
                                }
                            }
                            let row_h = 36.0;
                            let (row, resp) = ui.allocate_exact_size(
                                egui::vec2(ui.available_width(), row_h),
                                if launcher_transitioning {
                                    Sense::hover()
                                } else {
                                    Sense::click_and_drag()
                                },
                            );
                            let pinned = self.tile_board.contains_app(&app.id);
                            let dragging_this = self
                                .list_pin_drag
                                .as_ref()
                                .is_some_and(|d| d.app_id == app.id && d.active);
                            let list_frame = disco_list_reveal.and_then(|reveal| {
                                reveal.disco_app_list_frame(
                                    app_index,
                                    disco_list_count,
                                    disco_list_selected_id.as_deref() == Some(app.id.as_str()),
                                )
                            });
                            let list_scale = list_frame
                                .map(|frame| {
                                    (std::f32::consts::FRAC_PI_2 * frame.progress)
                                        .cos()
                                        .max(0.05)
                                })
                                .unwrap_or(1.0);
                            let paint_row = list_frame
                                .map(|frame| {
                                    egui::Rect::from_min_max(
                                        egui::pos2(
                                            project_disco_list_x(row.min.x, frame.progress),
                                            row.min.y,
                                        ),
                                        egui::pos2(
                                            project_disco_list_x(row.max.x, frame.progress),
                                            row.max.y,
                                        ),
                                    )
                                })
                                .unwrap_or(row);
                            let paint_visible = list_frame.is_none_or(|frame| frame.visible);
                            if paint_visible
                                && list_frame.is_none()
                                && (resp.hovered() || dragging_this)
                                && !pinned
                            {
                                ui.painter().rect_filled(
                                    paint_row,
                                    CornerRadius::same(2),
                                    Color32::from_rgba_unmultiplied(0, 0, 0, 16),
                                );
                            }
                            if paint_visible {
                                paint_app_icon(
                                    ui,
                                    &self.icon_cache,
                                    app,
                                    egui::pos2(
                                        project_disco_list_x(
                                            row.min.x + 14.0,
                                            list_frame.map_or(0.0, |frame| frame.progress),
                                        ),
                                        row.center().y,
                                    ),
                                    20.0 * list_scale,
                                    if pinned {
                                        pal.text_secondary
                                    } else {
                                        pal.accent
                                    },
                                );
                                ui.painter().text(
                                    egui::pos2(
                                        project_disco_list_x(
                                            row.min.x + 32.0,
                                            list_frame.map_or(0.0, |frame| frame.progress),
                                        ),
                                        row.center().y,
                                    ),
                                    egui::Align2::LEFT_CENTER,
                                    &app.name,
                                    egui::FontId::proportional(13.0 * list_scale.max(0.12)),
                                    if pinned { pal.text_secondary } else { pal.text },
                                );
                            }
                            resp.context_menu(|ui| {
                                if pinned {
                                    if ui.button("取消固定").clicked() {
                                        unpin_id = Some(app.id.clone());
                                        ui.close();
                                    }
                                } else if ui.button("固定到开始屏幕").clicked() {
                                    pin_id = Some(app.id.clone());
                                    ui.close();
                                }
                            });

                            // Start / track list→tile drag (skip if already pinned).
                            if !pinned {
                                if resp.drag_started() {
                                    if let Some(pos) = resp.interact_pointer_pos() {
                                        self.list_pin_drag = Some(ListPinDrag {
                                            app_id: app.id.clone(),
                                            name: app.name.clone(),
                                            active: false,
                                            start_pos: pos,
                                        });
                                    }
                                }
                                if resp.dragged() {
                                    if let Some(d) = self.list_pin_drag.as_mut() {
                                        if d.app_id == app.id {
                                            if let Some(pos) = ui.ctx().pointer_interact_pos() {
                                                if !d.active && pos.distance(d.start_pos) > 8.0 {
                                                    d.active = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            let suppress_click = self
                                .list_pin_drag
                                .as_ref()
                                .is_some_and(|d| d.app_id == app.id && d.active);
                            if resp.on_hover_text(&app.name).clicked() && !suppress_click {
                                launch_id = Some(app.id.clone());
                                launch_origin = Some(row);
                                launch_source = AppLaunchSource::AppList;
                            }
                        }
                        if apps_filtered.is_empty() {
                            ui.label(RichText::new("No apps match").weak());
                        }
                    });
            });
        }

        if show_windows8_apps {
            ui.scope_builder(UiBuilder::new().max_rect(list), |ui| {
                let row_height = 54.0;
                let rows = (list.height() / row_height).floor().max(1.0) as usize;
                let columns = apps_filtered.len().div_ceil(rows).max(1);
                let column_width = 224.0;
                egui::ScrollArea::horizontal()
                    .id_salt("windows8_all_apps")
                    .scroll_source(egui::containers::scroll_area::ScrollSource::ALL)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let content_min = ui.available_rect_before_wrap().min;
                        ui.set_min_size(egui::vec2(
                            columns as f32 * column_width + 80.0,
                            list.height(),
                        ));
                        for (index, app) in apps_filtered.iter().enumerate() {
                            let col = index / rows;
                            let row = index % rows;
                            let rect = egui::Rect::from_min_size(
                                content_min
                                    + egui::vec2(
                                        col as f32 * column_width,
                                        row as f32 * row_height,
                                    ),
                                egui::vec2(column_width - 12.0, row_height - 4.0),
                            );
                            let response = ui.interact(
                                rect,
                                ui.id().with(("windows8_app", &app.id)),
                                if launcher_transitioning {
                                    Sense::hover()
                                } else {
                                    Sense::click()
                                },
                            );
                            if response.hovered() {
                                ui.painter().rect_filled(
                                    rect,
                                    CornerRadius::ZERO,
                                    Color32::from_white_alpha(22),
                                );
                            }
                            paint_app_icon(
                                ui,
                                &self.icon_cache,
                                app,
                                egui::pos2(rect.min.x + 22.0, rect.center().y),
                                30.0,
                                Color32::WHITE,
                            );
                            ui.painter().text(
                                egui::pos2(rect.min.x + 46.0, rect.center().y),
                                egui::Align2::LEFT_CENTER,
                                &app.name,
                                egui::FontId::proportional(15.0),
                                Color32::WHITE,
                            );
                            let pinned = self.tile_board.contains_app(&app.id);
                            response.context_menu(|ui| {
                                if pinned {
                                    if ui.button("取消固定").clicked() {
                                        unpin_id = Some(app.id.clone());
                                        ui.close();
                                    }
                                } else if ui.button("固定到开始屏幕").clicked() {
                                    pin_id = Some(app.id.clone());
                                    ui.close();
                                }
                            });
                            if response.clicked() {
                                launch_id = Some(app.id.clone());
                                launch_origin = Some(rect);
                                launch_source = AppLaunchSource::AppList;
                            }
                        }
                    });
            });
        }

        // —— User-pinned Live tile board (1×1 … 8×8 integer grid) ——
        ui.set_opacity(base_ui_opacity);
        let board_primary_limit = match layout_style {
            StartLayoutStyle::Windows8 => StartTileBoard::windows8_metro_max_rows(tiles.height()),
            _ => match tile_flow {
                StartTileFlow::Vertical => StartTileBoard::cols_for_area(tiles),
                StartTileFlow::Horizontal => StartTileBoard::rows_for_area(tiles),
            },
        };
        if layout_style != StartLayoutStyle::Windows8 {
            // The source Metro page derives its own column-first positions and
            // must not rewrite the user's freeform Windows 10/Phone geometry.
            self.tile_board.ensure_flow(tile_flow, board_primary_limit);
        }
        self.tile_board.save_if_dirty();
        let board_content_size = if layout_style == StartLayoutStyle::Windows8 {
            self.tile_board.windows8_metro_content_size(tiles)
        } else {
            self.tile_board.content_size(tiles, tile_flow)
        };
        let launching_app_id = self
            .pending_app_launch
            .as_ref()
            .filter(|pending| pending.source == AppLaunchSource::Tile)
            .map(|pending| pending.app_id.clone());
        let mut board_content_area = tiles;
        if show_tiles {
            let tile_board = &mut self.tile_board;
            let apps = &self.snap.apps;
            let icons = &self.icon_cache;
            let mut tile_out = start_tiles::TileBoardOutcome::default();
            ui.scope_builder(UiBuilder::new().max_rect(tiles), |ui| {
                let scroll = match tile_flow {
                    StartTileFlow::Vertical => egui::ScrollArea::vertical(),
                    StartTileFlow::Horizontal => egui::ScrollArea::horizontal(),
                }
                .id_salt(("start_tile_scroll", layout_style))
                .scroll_source(egui::containers::scroll_area::ScrollSource::ALL)
                .auto_shrink([false, false]);
                let output = scroll.show(ui, |ui| {
                    let content_min = ui.available_rect_before_wrap().min;
                    ui.set_min_size(board_content_size);
                    let content = egui::Rect::from_min_size(content_min, board_content_size);
                    board_content_area = content;
                    if layout_style == StartLayoutStyle::Windows8 {
                        tile_board.paint_windows8_metro(
                            ui,
                            content,
                            tiles.size(),
                            apps,
                            &pal,
                            icons,
                            reveal,
                            launching_app_id.as_deref(),
                        )
                    } else {
                        tile_board.paint(
                            ui,
                            content,
                            apps,
                            &pal,
                            show_standard_list.then_some(list),
                            icons,
                            reveal,
                            launching_app_id.as_deref(),
                            tile_flow,
                        )
                    }
                });
                tile_out = output.inner;
            });
            if let Some(id) = tile_out.launch {
                launch_id = Some(id);
                launch_origin = tile_out.launch_origin;
                launch_source = AppLaunchSource::Tile;
            }
        }

        // List → board pin drag: drop target highlight, ghost, and commit on release.
        let pointer = ui.ctx().pointer_interact_pos();
        let primary_down = ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary));
        let now = ui.input(|i| i.time);
        if let Some(drag) = self.list_pin_drag.clone().filter(|_| show_tiles) {
            if drag.active {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                let over_board = pointer.is_some_and(|p| tiles.contains(p));
                if over_board {
                    ui.painter().rect_stroke(
                        tiles,
                        CornerRadius::same(0),
                        Stroke::new(2.0, pal.accent),
                        StrokeKind::Inside,
                    );
                    if let Some(pos) = pointer {
                        if let Some((_, _, ghost)) =
                            StartTileBoard::drop_preview(board_content_area, pos, tile_flow)
                        {
                            ui.painter().rect(
                                ghost,
                                CornerRadius::same(0),
                                Color32::from_rgba_unmultiplied(
                                    pal.accent.r(),
                                    pal.accent.g(),
                                    pal.accent.b(),
                                    48,
                                ),
                                Stroke::new(2.0, pal.accent),
                                StrokeKind::Inside,
                            );
                        }
                    }
                }
                // Floating chip under the cursor.
                if let Some(pos) = pointer {
                    let chip = egui::Rect::from_center_size(pos, egui::vec2(160.0, 36.0));
                    ui.painter().rect_filled(
                        chip.translate(egui::vec2(2.0, 3.0)),
                        CornerRadius::same(4),
                        Color32::from_rgba_unmultiplied(0, 0, 0, 40),
                    );
                    ui.painter().rect_filled(
                        chip,
                        CornerRadius::same(4),
                        Color32::from_rgba_unmultiplied(255, 255, 255, 235),
                    );
                    if let Some(app) = self.snap.apps.iter().find(|a| a.id == drag.app_id) {
                        paint_app_icon(
                            ui,
                            &self.icon_cache,
                            app,
                            egui::pos2(chip.min.x + 18.0, chip.center().y),
                            20.0,
                            pal.accent,
                        );
                    }
                    ui.painter().text(
                        egui::pos2(chip.min.x + 34.0, chip.center().y),
                        egui::Align2::LEFT_CENTER,
                        &drag.name,
                        egui::FontId::proportional(13.0),
                        pal.text,
                    );
                }
                ui.ctx().request_repaint();
            }

            if !primary_down {
                if drag.active {
                    if let Some(pos) = pointer {
                        if self.tile_board.pin_from_list_drop(
                            &drag.app_id,
                            pos,
                            board_content_area,
                            now,
                            tile_flow,
                        ) {
                            self.tile_board.save_if_dirty();
                        }
                    }
                }
                self.list_pin_drag = None;
            }
        }

        // —— Search (bottom-left, above taskbar) ——
        ui.set_opacity(base_ui_opacity * content_opacity);
        if show_search {
            ui.painter().rect_filled(
                search,
                CornerRadius::same(2),
                Color32::from_rgba_unmultiplied(255, 255, 255, 230),
            );
            ui.painter().rect_stroke(
                search,
                CornerRadius::same(2),
                Stroke::new(1.0, Color32::from_rgba_unmultiplied(0, 0, 0, 28)),
                StrokeKind::Inside,
            );
            ui.scope_builder(UiBuilder::new().max_rect(search), |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(FluentIcon::Search.glyph())
                            .size(14.0)
                            .color(pal.text_secondary),
                    );
                    ui.add_sized(
                        egui::vec2((ui.available_width() - 8.0).max(80.0), 28.0),
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("Search")
                            .frame(egui::Frame::NONE)
                            .desired_width(f32::INFINITY),
                    );
                });
            });
        }
        ui.set_opacity(base_ui_opacity);

        if layout_style == StartLayoutStyle::Windows8 {
            let button = egui::Rect::from_center_size(
                egui::pos2(desktop.min.x + 38.0, desktop.max.y - 30.0),
                egui::vec2(36.0, 36.0),
            );
            let response = ui.interact(
                button,
                ui.id().with("windows8_all_apps_button"),
                Sense::click(),
            );
            ui.painter().circle(
                button.center(),
                16.0,
                if response.hovered() {
                    Color32::from_white_alpha(32)
                } else {
                    Color32::TRANSPARENT
                },
                Stroke::new(1.5, Color32::WHITE),
            );
            ui.painter().text(
                button.center(),
                egui::Align2::CENTER_CENTER,
                if self.windows8_apps_visible {
                    FluentIcon::ChevronUp.glyph()
                } else {
                    FluentIcon::ChevronDown.glyph()
                },
                fluent_egui::winui::icon_font_id(16.0),
                Color32::WHITE,
            );
            if response.clicked() {
                self.set_start_apps_visible(!self.windows8_apps_visible);
            }
        }

        if toggle_rail {
            self.show_all_apps = !self.show_all_apps;
        }
        if let Some(id) = pin_id {
            self.tile_board.pin_app(&id, tile_flow, board_primary_limit);
            self.tile_board.save_if_dirty();
        }
        if let Some(id) = unpin_id {
            self.tile_board.unpin_app(&id);
            self.tile_board.save_if_dirty();
        }
        if open_settings {
            self.snap.panel = Panel::DesktopSettings;
        }
        if close {
            self.begin_close_launcher(self.ui_now);
        }
        if let Some(id) = launch_id {
            if matches!(self.start_reveal, StartReveal::Idle) {
                self.begin_app_launch(id, launch_origin, launch_source);
            }
        }
    }

    fn draw_action_center(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        ui.painter().rect_filled(
            desktop,
            0.0,
            if self.nested {
                Color32::from_rgba_unmultiplied(0, 0, 0, 60)
            } else {
                self.pal.overlay
            },
        );
        let width = 360.0_f32.min(desktop.width() - 24.0);
        let card = egui::Rect::from_min_max(
            egui::pos2(desktop.max.x - width - 16.0, desktop.min.y + 20.0),
            egui::pos2(desktop.max.x - 16.0, desktop.max.y - 12.0),
        );
        let pal = self.pal;
        let accent = self.accent_on_surfaces.then_some(self.accent);
        fluent1::paint_strong_surface(ui, card, &pal, accent);
        self.reveal.paint_on(ui, card, &self.pal);

        let notes = self.snap.notifications.clone();
        let mut dismiss: Option<u64> = None;
        let mut clear_all = false;
        let mut close = false;
        let mut open_settings = false;

        ui.scope_builder(UiBuilder::new().max_rect(card), |ui| {
            ui.winui(|ui| {
                ui.column(|ui| {
                    ui.add_space(10.0);
                    ui.row(|ui| {
                        ui.label(RichText::new("Notifications").strong().size(16.0));
                        if ui.button("Clear all").clicked() {
                            clear_all = true;
                        }
                    });

                    if notes.is_empty() {
                        ui.label(RichText::new("No new notifications").weak());
                    } else {
                        for n in &notes {
                            ui.card(&n.app, &n.title, |ui| {
                                ui.label(&n.body);
                                if ui.button("Dismiss").clicked() {
                                    dismiss = Some(n.id);
                                }
                            });
                        }
                    }

                    ui.add_space(8.0);
                    ui.card("Media", "Now playing", |ui| {
                        ui.label(if self.media_playing {
                            "Demo Track — Artist"
                        } else {
                            "Paused"
                        });
                        if ui
                            .button(if self.media_playing { "Pause" } else { "Play" })
                            .clicked()
                        {
                            self.media_playing = !self.media_playing;
                        }
                    });

                    ui.add_space(8.0);
                    ui.label(RichText::new("Quick settings").strong().size(16.0));
                    ui.toggle(&mut self.quick_wifi, "Wi‑Fi");
                    if ui
                        .button(if self.expand_wifi {
                            "Hide Wi‑Fi"
                        } else {
                            "Wi‑Fi details"
                        })
                        .clicked()
                    {
                        self.expand_wifi = !self.expand_wifi;
                    }
                    if self.expand_wifi {
                        ui.label(RichText::new("Connected · DemoNet").weak());
                    }
                    ui.toggle(&mut self.quick_bt, "Bluetooth");
                    if ui
                        .button(if self.expand_bt {
                            "Hide Bluetooth"
                        } else {
                            "Bluetooth details"
                        })
                        .clicked()
                    {
                        self.expand_bt = !self.expand_bt;
                    }
                    if self.expand_bt {
                        ui.label(RichText::new("Devices nearby").weak());
                    }
                    ui.toggle(&mut self.quick_a11y, "Accessibility");
                    if ui
                        .button(if self.expand_a11y {
                            "Hide Accessibility"
                        } else {
                            "Accessibility details"
                        })
                        .clicked()
                    {
                        self.expand_a11y = !self.expand_a11y;
                    }
                    if self.expand_a11y {
                        ui.label(RichText::new("Magnifier · Narrator stubs").weak());
                    }

                    let spacer = (ui.available_height() - 56.0).max(8.0);
                    ui.add_space(spacer);
                    ui.row(|ui| {
                        ui.label("86%");
                        if ui
                            .icon_button(FluentIcon::Settings)
                            .on_hover_text("Desktop settings")
                            .clicked()
                        {
                            open_settings = true;
                        }
                        if ui
                            .icon_button(FluentIcon::Power)
                            .on_hover_text("Power")
                            .clicked()
                        {
                            close = true;
                        }
                        if ui
                            .icon_button(FluentIcon::Person)
                            .on_hover_text("User")
                            .clicked()
                        {
                            close = true;
                        }
                    });
                });
            });
        });

        if clear_all {
            self.snap.notifications.clear();
        }
        if let Some(id) = dismiss {
            self.snap.notifications.retain(|x| x.id != id);
        }
        if open_settings {
            self.snap.panel = Panel::DesktopSettings;
        }
        if close {
            self.snap.panel = Panel::None;
        }
    }

    fn draw_task_view(&mut self, ui: &mut egui::Ui, desktop: egui::Rect) {
        ui.painter().rect_filled(
            desktop,
            0.0,
            if self.nested {
                Color32::from_rgba_unmultiplied(0, 0, 0, 60)
            } else {
                self.pal.overlay
            },
        );
        let card = inset_card(desktop, 20.0, 16.0, 12.0, 16.0);
        let pal = self.pal;
        fluent1::paint_surface(ui, card, &pal);
        self.reveal.paint_on(ui, card, &self.pal);

        let running: Vec<_> = self
            .snap
            .apps
            .iter()
            .filter(|a| a.running)
            .cloned()
            .collect();
        let mut activate: Option<(String, egui::Rect)> = None;
        let mut close = false;

        ui.scope_builder(UiBuilder::new().max_rect(card), |ui| {
            ui.winui(|ui| {
                ui.column(|ui| {
                    ui.add_space(12.0);
                    ui.label(RichText::new("Task View").strong().size(22.0));
                    ui.label(RichText::new("Tap a window to switch").weak().size(13.0));
                    ui.add_space(14.0);

                    if running.is_empty() {
                        ui.label("No open apps — launch from Start");
                    } else {
                        ui.wrap(|ui| {
                            for app in &running {
                                let id = app.id.clone();
                                let tip = app.name.clone();
                                let (rect, resp) =
                                    ui.allocate_exact_size(egui::vec2(48.0, 48.0), Sense::click());
                                paint_app_icon(
                                    ui,
                                    &self.icon_cache,
                                    app,
                                    rect.center(),
                                    36.0,
                                    pal.accent,
                                );
                                if resp.on_hover_text(tip).clicked() {
                                    activate = Some((id, rect));
                                }
                            }
                        });
                    }

                    let spacer = (ui.available_height() - 48.0).max(8.0);
                    ui.add_space(spacer);
                    if ui
                        .icon_button(FluentIcon::Close)
                        .on_hover_text("Close")
                        .clicked()
                    {
                        close = true;
                    }
                });
            });
        });

        if let Some((id, origin)) = activate {
            if self.active_app_id.as_deref() != Some(id.as_str()) {
                if let Some(window) = self.runtime_apps.get(&id).cloned() {
                    self.begin_app_switch(&id, window, origin, false);
                } else {
                    self.request_app_activation(&id);
                }
            }
            self.snap.panel = Panel::None;
        }
        if close {
            self.snap.panel = Panel::None;
        }
    }
}

fn taskbar_icon_button(
    ui: &mut egui::Ui,
    id_salt: &'static str,
    icon: FluentIcon,
    tooltip: &str,
    extent: f32,
    selected: bool,
    pal: &Palette,
) -> egui::Response {
    let (_, rect) = ui.allocate_space(egui::vec2(extent, extent));
    let response = ui.interact(
        rect,
        ui.id().with(("taskbar_button", id_salt)),
        Sense::click(),
    );
    let fill = if response.is_pointer_button_down_on() {
        pal.reveal_strong
    } else if response.hovered() || selected {
        pal.reveal
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, CornerRadius::ZERO, fill);
    ui.painter().text(
        rect.center() + egui::vec2(0.0, 1.0),
        egui::Align2::CENTER_CENTER,
        icon.glyph(),
        fluent_egui::winui::icon_font_id(18.0),
        pal.text,
    );
    // Start and Task View are shell commands, not running applications, so
    // their selected state must not draw an app-style bottom indicator.
    response.on_hover_text(tooltip)
}

fn taskbar_app_button(
    ui: &mut egui::Ui,
    cache: &IconCache,
    app: &AppEntry,
    extent: f32,
    active: bool,
    pal: &Palette,
) -> egui::Response {
    let (_, rect) = ui.allocate_space(egui::vec2(extent, extent));
    let response = ui.interact(
        rect,
        ui.id().with(("taskbar_app", app.id.as_str())),
        Sense::click_and_drag(),
    );
    let fill = if response.is_pointer_button_down_on() {
        pal.reveal_strong
    } else if response.hovered() || active {
        pal.reveal
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, CornerRadius::ZERO, fill);
    paint_app_icon(ui, cache, app, rect.center(), extent * 0.58, pal.text);
    if app.running {
        paint_running_dot(ui, rect, extent * 0.58, pal.accent);
    }
    response.on_hover_text(&app.name)
}

fn paint_running_dot(ui: &mut egui::Ui, rect: egui::Rect, icon: f32, accent: Color32) {
    ui.painter().rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(rect.center().x - icon * 0.175, rect.bottom() - 3.0),
            egui::pos2(rect.center().x + icon * 0.175, rect.bottom()),
        ),
        CornerRadius::ZERO,
        accent,
    );
}

fn lerp_rect(from: egui::Rect, to: egui::Rect, progress: f32) -> egui::Rect {
    let lerp_pos = |a: egui::Pos2, b: egui::Pos2| {
        egui::pos2(
            egui::lerp(a.x..=b.x, progress),
            egui::lerp(a.y..=b.y, progress),
        )
    };
    egui::Rect::from_min_max(lerp_pos(from.min, to.min), lerp_pos(from.max, to.max))
}

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress.clamp(0.0, 1.0)).powi(3)
}

fn load_window_texture(
    ctx: &egui::Context,
    cache: &mut HashMap<u64, WindowPreviewTexture>,
    window_id: u64,
    path: &str,
    texture_namespace: &str,
) -> Option<TextureHandle> {
    let modified = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok();
    if let Some(cached) = cache.get(&window_id) {
        if cached.path == path && cached.modified == modified {
            return Some(cached.texture.clone());
        }
    }
    let image = image::open(path).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let texture = ctx.load_texture(
        format!("{texture_namespace}-{window_id}"),
        egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
        TextureOptions::LINEAR,
    );
    cache.insert(
        window_id,
        WindowPreviewTexture {
            path: path.to_string(),
            modified,
            texture: texture.clone(),
        },
    );
    Some(texture)
}

/// UV crop matching Flutter's `BoxFit.cover`.
fn cover_uv(texture: egui::Vec2, destination: egui::Vec2) -> egui::Rect {
    let texture_aspect = texture.x / texture.y.max(1.0);
    let destination_aspect = destination.x / destination.y.max(1.0);
    if texture_aspect > destination_aspect {
        let visible = (destination_aspect / texture_aspect).clamp(0.0, 1.0);
        egui::Rect::from_min_max(
            egui::pos2((1.0 - visible) * 0.5, 0.0),
            egui::pos2((1.0 + visible) * 0.5, 1.0),
        )
    } else {
        let visible = (texture_aspect / destination_aspect).clamp(0.0, 1.0);
        egui::Rect::from_min_max(
            egui::pos2(0.0, (1.0 - visible) * 0.5),
            egui::pos2(1.0, (1.0 + visible) * 0.5),
        )
    }
}

fn normalize_app_id(id: &str) -> String {
    id.trim().trim_end_matches(".desktop").to_ascii_lowercase()
}

fn app_matches_runtime_id(app: &AppEntry, runtime_id: &str) -> bool {
    let runtime_id = normalize_app_id(runtime_id);
    let mut candidates = vec![app.id.as_str()];
    if let Some(desktop_id) = app.desktop_id.as_deref() {
        candidates.push(desktop_id);
    }
    if let Some(icon) = app.icon.as_deref() {
        candidates.push(icon);
    }
    if let Some(exec) = app
        .exec
        .as_deref()
        .and_then(|exec| exec.split_whitespace().next())
    {
        candidates.push(exec.rsplit('/').next().unwrap_or(exec));
    }
    candidates
        .into_iter()
        .any(|candidate| normalize_app_id(candidate) == runtime_id)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut temporary = path.to_path_buf();
    temporary.set_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)
}

fn paint_swatch(ui: &mut egui::Ui, rect: egui::Rect, color: Color32, selected: bool) {
    ui.painter().rect_filled(rect, CornerRadius::ZERO, color);
    ui.painter().rect_stroke(
        rect,
        CornerRadius::ZERO,
        Stroke::new(
            if selected { 2.0 } else { 1.0 },
            if selected {
                Color32::WHITE
            } else {
                Color32::from_rgba_unmultiplied(0, 0, 0, 70)
            },
        ),
        StrokeKind::Inside,
    );
}

fn paint_contrast_preview(ui: &mut egui::Ui, theme: desktop::ContrastTheme, accent: Color32) {
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().min(420.0), 140.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme.bg);
    ui.painter().text(
        egui::pos2(rect.min.x + 16.0, rect.min.y + 24.0),
        egui::Align2::LEFT_CENTER,
        "Sample hyperlink text",
        egui::FontId::proportional(16.0),
        theme.hyperlink,
    );
    ui.painter().text(
        egui::pos2(rect.min.x + 16.0, rect.min.y + 52.0),
        egui::Align2::LEFT_CENTER,
        "Body text on contrast background",
        egui::FontId::proportional(14.0),
        theme.text,
    );
    ui.painter().text(
        egui::pos2(rect.min.x + 16.0, rect.min.y + 78.0),
        egui::Align2::LEFT_CENTER,
        "Disabled text",
        egui::FontId::proportional(13.0),
        theme.disabled,
    );
    let btn = egui::Rect::from_min_size(
        egui::pos2(rect.min.x + 16.0, rect.max.y - 40.0),
        egui::vec2(110.0, 28.0),
    );
    ui.painter().rect_filled(btn, 0.0, theme.button);
    ui.painter().rect_stroke(
        btn,
        0.0,
        Stroke::new(1.0, theme.selected),
        StrokeKind::Inside,
    );
    ui.painter().text(
        btn.center(),
        egui::Align2::CENTER_CENTER,
        "Button",
        egui::FontId::proportional(13.0),
        theme.text,
    );
    let accent_chip = egui::Rect::from_min_size(
        egui::pos2(rect.max.x - 56.0, rect.min.y + 16.0),
        egui::vec2(36.0, 36.0),
    );
    ui.painter().rect_filled(accent_chip, 0.0, accent);
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color32 {
    let h = ((h % 360.0) + 360.0) % 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

fn paint_wallpaper_swatch(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    top: Color32,
    bottom: Color32,
    selected: bool,
    accent: Color32,
) {
    let mid = rect.center().y;
    ui.painter().rect_filled(
        egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, mid)),
        CornerRadius::ZERO,
        top,
    );
    ui.painter().rect_filled(
        egui::Rect::from_min_max(egui::pos2(rect.min.x, mid), rect.max),
        CornerRadius::ZERO,
        bottom,
    );
    let stroke = if selected {
        Stroke::new(2.0, accent)
    } else {
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 40))
    };
    ui.painter()
        .rect_stroke(rect, CornerRadius::ZERO, stroke, StrokeKind::Inside);
}

fn mode_chip(
    ui: &mut egui::Ui,
    label: &str,
    icon: FluentIcon,
    selected: bool,
    accent: Color32,
) -> egui::Response {
    let desired = egui::vec2(120.0, 72.0);
    let (rect, resp) = ui.allocate_exact_size(desired, Sense::click());
    let fill = if selected {
        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 48)
    } else {
        Color32::from_rgba_unmultiplied(128, 128, 128, 36)
    };
    ui.painter().rect(
        rect,
        CornerRadius::ZERO,
        fill,
        Stroke::new(
            if selected { 2.0 } else { 1.0 },
            if selected {
                accent
            } else {
                Color32::from_rgba_unmultiplied(255, 255, 255, 40)
            },
        ),
        StrokeKind::Inside,
    );
    ui.painter().text(
        egui::pos2(rect.center().x, rect.min.y + 28.0),
        egui::Align2::CENTER_CENTER,
        icon.glyph(),
        fluent_egui::winui::icon_font_id(22.0),
        ui.visuals().text_color(),
    );
    ui.painter().text(
        egui::pos2(rect.center().x, rect.max.y - 16.0),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(13.0),
        ui.visuals().text_color(),
    );
    resp
}

fn inset_card(desktop: egui::Rect, top: f32, right: f32, bottom: f32, left: f32) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::pos2(desktop.min.x + left, desktop.min.y + top),
        egui::pos2(desktop.max.x - right, desktop.max.y - bottom),
    )
}

#[cfg(test)]
mod preference_tests {
    use super::{
        app_matches_runtime_id, AppEntry, AppLaunchAnimationStyle, ShellPreferences,
        StartLayoutStyle, StartTileAnimationStyle,
    };

    #[test]
    fn animation_style_round_trips_through_preferences_json() {
        let preferences = ShellPreferences {
            start_tile_animation: StartTileAnimationStyle::DiscoTurnstile,
            app_launch_animation: Some(AppLaunchAnimationStyle::FactorClipReveal),
            start_layout_style: StartLayoutStyle::WindowsPhone,
            app_switch_animation_enabled: false,
            pinned_apps: Some(vec!["terminal".into()]),
        };
        let json = serde_json::to_string(&preferences).unwrap();
        let decoded: ShellPreferences = serde_json::from_str(&json).unwrap();

        assert_eq!(
            decoded.start_tile_animation,
            StartTileAnimationStyle::DiscoTurnstile
        );
        assert_eq!(
            decoded.app_launch_animation,
            Some(AppLaunchAnimationStyle::FactorClipReveal)
        );
        assert_eq!(decoded.pinned_apps, Some(vec!["terminal".into()]));
        assert_eq!(decoded.start_layout_style, StartLayoutStyle::WindowsPhone);
        assert!(!decoded.app_switch_animation_enabled);
    }

    #[test]
    fn flutter_metro_start_style_round_trips_independently() {
        let preferences = ShellPreferences {
            start_tile_animation: StartTileAnimationStyle::FlutterMetroGroup,
            app_launch_animation: Some(AppLaunchAnimationStyle::Windows8),
            pinned_apps: None,
            ..ShellPreferences::default()
        };
        let json = serde_json::to_string(&preferences).unwrap();
        assert!(json.contains("flutter_metro_group"));
        let decoded: ShellPreferences = serde_json::from_str(&json).unwrap();
        assert_eq!(
            decoded.start_tile_animation,
            StartTileAnimationStyle::FlutterMetroGroup
        );
        assert_eq!(
            decoded.app_launch_animation,
            Some(AppLaunchAnimationStyle::Windows8)
        );
    }

    #[test]
    fn missing_animation_style_uses_the_product_default() {
        let decoded: ShellPreferences = serde_json::from_str("{}").unwrap();

        assert_eq!(
            decoded.start_tile_animation,
            StartTileAnimationStyle::Windows10Mobile
        );
        assert_eq!(decoded.app_launch_animation, None);
        assert_eq!(decoded.pinned_apps, None);
        assert_eq!(decoded.start_layout_style, StartLayoutStyle::Windows10);
        assert!(decoded.app_switch_animation_enabled);
        assert_eq!(
            decoded.resolved_app_launch_animation(),
            AppLaunchAnimationStyle::Windows10Mobile
        );
    }

    #[test]
    fn old_coupled_animation_setting_migrates_to_a_matching_launch_style() {
        let decoded: ShellPreferences =
            serde_json::from_str(r#"{"start_tile_animation":"disco_turnstile"}"#).unwrap();

        assert_eq!(
            decoded.resolved_app_launch_animation(),
            AppLaunchAnimationStyle::DiscoTurnstile
        );
    }

    #[test]
    fn windows8_launch_style_round_trips_independently_from_start_entrance() {
        let preferences = ShellPreferences {
            start_tile_animation: StartTileAnimationStyle::Windows10Mobile,
            app_launch_animation: Some(AppLaunchAnimationStyle::Windows8),
            pinned_apps: None,
            ..ShellPreferences::default()
        };
        let json = serde_json::to_string(&preferences).unwrap();
        let decoded: ShellPreferences = serde_json::from_str(&json).unwrap();

        assert_eq!(
            decoded.resolved_app_launch_animation(),
            AppLaunchAnimationStyle::Windows8
        );
        assert_eq!(
            decoded.start_tile_animation,
            StartTileAnimationStyle::Windows10Mobile
        );
    }

    #[test]
    fn compositor_app_id_matches_desktop_entry_case_and_suffix() {
        let app = AppEntry {
            id: "Alacritty".into(),
            name: "Alacritty".into(),
            pinned: true,
            running: false,
            desktop_id: Some("Alacritty.desktop".into()),
            exec: Some("alacritty".into()),
            icon: Some("Alacritty".into()),
        };

        assert!(app_matches_runtime_id(&app, "alacritty"));
        assert!(app_matches_runtime_id(&app, "Alacritty.desktop"));
    }
}
