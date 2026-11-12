//! Shared types between `win10x-comp` and `win10x-shell`.

use serde::{Deserialize, Serialize};

/// xdg_toplevel app_id the compositor uses to recognize the shell client.
pub const SHELL_APP_ID: &str = "win10x-shell";

/// Title prefix; shell appends `|Panel|sNNN` so the compositor can sync chrome + scale.
pub const SHELL_TITLE_PREFIX: &str = "Win10X Shell";

/// Runtime files used for the small shell/compositor session bridge.
pub const SESSION_STATE_PATH_ENV: &str = "WIN10X_SESSION_STATE_PATH";
pub const SESSION_COMMAND_PATH_ENV: &str = "WIN10X_SESSION_COMMAND_PATH";

/// Output scale presets (percent). Clients redraw via `wp_fractional_scale` (niri-style).
pub const OUTPUT_SCALE_PRESETS: &[u32] = &[75, 100, 125, 150, 175, 200];

/// Clamp / snap a scale factor to a supported preset (0.75 … 2.0).
pub fn clamp_output_scale(scale: f64) -> f64 {
    let pct = (scale * 100.0).round() as i32;
    let pct = pct.clamp(75, 200);
    // Snap to nearest 25% step (same cadence as anvil / niri hotkeys).
    let snapped = ((pct + 12) / 25) * 25;
    let snapped = snapped.clamp(75, 200);
    snapped as f64 / 100.0
}

pub fn scale_to_percent(scale: f64) -> u32 {
    (clamp_output_scale(scale) * 100.0).round() as u32
}

/// Build the shell xdg title: `Win10X Shell|<Panel>|s125`.
pub fn format_shell_title(panel: Panel, scale: f64) -> String {
    let panel = match panel {
        Panel::None => "None",
        Panel::Launcher => "Launcher",
        Panel::ActionCenter => "ActionCenter",
        Panel::TaskView => "TaskView",
        Panel::DesktopSettings => "DesktopSettings",
        Panel::TaskbarAppMenu => "TaskbarAppMenu",
    };
    format!("{SHELL_TITLE_PREFIX}|{panel}|s{}", scale_to_percent(scale))
}

/// Parse panel + optional scale from a shell title.
pub fn parse_shell_title(title: &str) -> (Panel, Option<f64>) {
    let Some(rest) = title.strip_prefix(SHELL_TITLE_PREFIX) else {
        return (Panel::None, None);
    };
    let rest = rest.strip_prefix('|').unwrap_or(rest);
    let mut parts = rest.split('|');
    let panel = match parts.next().unwrap_or("") {
        "Launcher" => Panel::Launcher,
        "ActionCenter" => Panel::ActionCenter,
        "TaskView" => Panel::TaskView,
        "DesktopSettings" => Panel::DesktopSettings,
        "TaskbarAppMenu" => Panel::TaskbarAppMenu,
        "None" | "" => Panel::None,
        _ => Panel::None,
    };
    let scale = parts.next().and_then(|p| {
        let n = p.strip_prefix('s')?.parse::<u32>().ok()?;
        Some(clamp_output_scale(n as f64 / 100.0))
    });
    (panel, scale)
}

/// Taskbar height presets (logical px). `Medium` matches the common 10X chrome height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskbarSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TaskbarSize {
    pub fn height(self) -> f32 {
        match self {
            Self::Small => 40.0,
            Self::Medium => 48.0,
            Self::Large => 56.0,
        }
    }

    pub fn icon(self) -> f32 {
        match self {
            Self::Small => 24.0,
            Self::Medium => 32.0,
            Self::Large => 44.0,
        }
    }

    pub fn cycle(self) -> Self {
        match self {
            Self::Small => Self::Medium,
            Self::Medium => Self::Large,
            Self::Large => Self::Small,
        }
    }
}

/// Which floating system surface is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Panel {
    #[default]
    None,
    Launcher,
    ActionCenter,
    TaskView,
    /// Lightweight personalization: wallpaper, accent, light/dark.
    DesktopSettings,
    /// Modal taskbar application command menu.
    TaskbarAppMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
}

/// App descriptor used by shell UI and compositor overlays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub pinned: bool,
    pub running: bool,
    /// Desktop file id without path, e.g. `Alacritty.desktop` / `brave-browser`.
    #[serde(default)]
    pub desktop_id: Option<String>,
    /// Raw Exec= line (field codes stripped at launch time).
    #[serde(default)]
    pub exec: Option<String>,
    /// FreeDesktop `Icon=` value (name or absolute path).
    #[serde(default)]
    pub icon: Option<String>,
}

/// A real xdg-toplevel currently managed by `win10x-comp`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunningApp {
    /// Stable for this toplevel during the compositor session.
    #[serde(default)]
    pub window_id: u64,
    pub app_id: String,
    pub title: String,
    pub active: bool,
    #[serde(default)]
    pub minimized: bool,
    #[serde(default)]
    pub mode: AppWindowMode,
    /// Bounded runtime PNG generated for taskbar/Task View preview cards.
    #[serde(default)]
    pub preview_path: Option<String>,
    /// PNG rendered at 25% of the output width and height for application-switch
    /// animation proxies.
    #[serde(default)]
    pub transition_preview_path: Option<String>,
}

/// Compositor-owned placement mode for an application toplevel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AppWindowMode {
    #[default]
    Fullscreen,
    SplitLeft,
    SplitRight,
    Floating,
}

/// Compositor-owned runtime state consumed by the separate shell process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompositorSessionState {
    pub apps: Vec<RunningApp>,
}

/// One-shot requests from shell chrome back to the compositor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionCommand {
    SetAppSwitchAnimationEnabled { enabled: bool },
    ActivateApp { app_id: String },
    ActivateWindow { window_id: u64 },
    MinimizeApp { app_id: String },
    MaximizeApp { app_id: String },
    CloseApp { app_id: String },
    SplitApp { app_id: String },
    FloatApp { app_id: String },
}

impl AppEntry {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            pinned: false,
            running: false,
            desktop_id: None,
            exec: None,
            icon: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub app: String,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellSnapshot {
    pub panel: Panel,
    pub taskbar_size: TaskbarSize,
    pub orientation: Orientation,
    pub search_query: String,
    pub show_all_apps: bool,
    pub apps: Vec<AppEntry>,
    pub recent: Vec<String>,
    pub notifications: Vec<Notification>,
    pub media_playing: bool,
    /// Soft-keyboard inset hook for a future OSK (MVP: reserved only).
    pub keyboard_inset: f32,
    /// Wayland output scale (1.0 = 100%). Applied via fractional-scale, not bitmap zoom.
    #[serde(default = "default_output_scale")]
    pub output_scale: f64,
}

fn default_output_scale() -> f64 {
    1.0
}

impl Default for ShellSnapshot {
    fn default() -> Self {
        Self {
            panel: Panel::None,
            taskbar_size: TaskbarSize::Medium,
            orientation: Orientation::Portrait,
            search_query: String::new(),
            show_all_apps: false,
            apps: default_apps(),
            recent: Vec::new(),
            notifications: vec![Notification {
                id: 1,
                app: "System".into(),
                title: "Welcome".into(),
                body: "Win10X shell — Fluent surfaces ready.".into(),
            }],
            media_playing: false,
            keyboard_inset: 0.0,
            output_scale: 1.0,
        }
    }
}

fn default_apps() -> Vec<AppEntry> {
    // Fallback stubs when no .desktop scan is available (compositor overlays).
    let mut apps = vec![
        AppEntry {
            id: "terminal".into(),
            name: "Terminal".into(),
            pinned: true,
            running: false,
            desktop_id: Some("Alacritty.desktop".into()),
            exec: Some("alacritty".into()),
            icon: Some("Alacritty".into()),
        },
        AppEntry {
            id: "browser".into(),
            name: "Browser".into(),
            pinned: true,
            running: false,
            desktop_id: Some("brave-browser.desktop".into()),
            exec: Some("brave".into()),
            icon: Some("brave-browser".into()),
        },
        AppEntry {
            id: "files".into(),
            name: "Files".into(),
            pinned: true,
            running: false,
            desktop_id: None,
            exec: Some("xdg-open".into()),
            icon: Some("system-file-manager".into()),
        },
    ];
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}

/// Touch target guidance (epx), from project research notes.
pub const MIN_TOUCH_TARGET: f32 = 40.0;
pub const PREFERRED_TOUCH_TARGET: f32 = 44.0;
pub const MIN_TOUCH_GAP: f32 = 4.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medium_is_default_taskbar() {
        assert_eq!(TaskbarSize::default(), TaskbarSize::Medium);
        assert!(TaskbarSize::Large.icon() >= PREFERRED_TOUCH_TARGET - f32::EPSILON);
    }

    #[test]
    fn cycle_sizes() {
        assert_eq!(TaskbarSize::Small.cycle(), TaskbarSize::Medium);
        assert_eq!(TaskbarSize::Large.cycle(), TaskbarSize::Small);
    }

    #[test]
    fn default_snapshot_has_pinned_apps() {
        let s = ShellSnapshot::default();
        assert!(s.apps.iter().any(|a| a.pinned));
        assert_eq!(s.panel, Panel::None);
    }

    #[test]
    fn shell_title_roundtrips_scale() {
        let t = format_shell_title(Panel::Launcher, 1.25);
        assert_eq!(t, "Win10X Shell|Launcher|s125");
        let (p, s) = parse_shell_title(&t);
        assert_eq!(p, Panel::Launcher);
        assert_eq!(s, Some(1.25));
    }

    #[test]
    fn clamp_scale_snaps() {
        assert_eq!(clamp_output_scale(1.3), 1.25);
        assert_eq!(clamp_output_scale(0.5), 0.75);
        assert_eq!(clamp_output_scale(3.0), 2.0);
    }
}
