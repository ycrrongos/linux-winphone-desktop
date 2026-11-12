//! User-pinned Start Live Tiles on an integer grid (1×1 … 8×8).
//!
//! Unit sizing: a **2×2** tile is the square you get by trimming a classic **4:3**
//! medium tile down to **3:3** (equal sides). One grid unit is half of that side.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use eframe::egui::{
    self, Align2, Color32, CornerRadius, FontId, Mesh, PointerButton, Pos2, Rect, Sense, Shape,
    Stroke, StrokeKind, Ui, Vec2,
};
use eframe::epaint::Vertex;
use serde::{Deserialize, Serialize};

use crate::fluent1::{ease_decelerate, ease_out_back, spring_step, Palette};
use crate::icons::{paint_app_icon, IconCache};

pub const TILE_MIN: u8 = 1;
pub const TILE_MAX: u8 = 8;
pub const TILE_DEFAULT: u8 = 2;
pub const DEFAULT_TILE_GROUP_ID: u32 = 1;

const GAP: f32 = 6.0;
/// Exact layout constants used by coldshineb/flutter_metro_ui's `Home.dart`.
pub const WINDOWS8_METRO_TILE: f32 = 120.0;
pub const WINDOWS8_METRO_GAP: f32 = 10.0;
pub const WINDOWS8_METRO_GROUP_GAP: f32 = 60.0;
pub const WINDOWS8_METRO_TOP_PADDING: f32 = 140.0;
pub const WINDOWS8_METRO_BOTTOM_PADDING: f32 = 60.0;
pub const WINDOWS8_METRO_START_PADDING: f32 = 100.0;
const WINDOWS8_METRO_RIGHT_PADDING: f32 = 100.0;
const WINDOWS8_METRO_GROUP_TITLE_HEIGHT: f32 = 24.0;
const WINDOWS8_METRO_GROUP_TITLE_GAP: f32 = 10.0;
const WINDOWS8_METRO_PRESS_IN: f32 = 0.150;
const WINDOWS8_METRO_PRESS_OUT: f32 = 0.300;
const TILE_GROUP_HEADER_GUTTER: f32 = 30.0;
const HANDLE: f32 = 18.0;
/// Pointer travel before a tile drag becomes an active move (same as app list).
const DRAG_ACTIVATE: f32 = 8.0;
/// Snap-to-grid after releasing a drag (ease-out-back).
const SNAP_DUR: f32 = 0.34;
/// Sibling tiles sliding out of the way (Fluent decelerate).
const AVOID_DUR: f32 = 0.28;
/// Tile width/height settle after integer resize steps.
const RESIZE_DUR: f32 = 0.26;
/// Icon center as a fraction of the tile face height.
const TILE_ICON_CENTER_Y: f32 = 0.45;
/// Disco Launcher uses a 2000 px parent perspective for its tile page.
const DISCO_PERSPECTIVE: f32 = 2_000.0;
const DISCO_OUTER_START_ANGLE: f32 = 70.0;
const DISCO_INNER_START_ANGLE: f32 = 45.0;
const DISCO_INNER_START_X: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StartLayoutStyle {
    #[default]
    Windows10,
    WindowsPhone,
    Windows8,
}

impl StartLayoutStyle {
    pub const ALL: [Self; 3] = [Self::Windows10, Self::WindowsPhone, Self::Windows8];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Windows10 => "Windows 10",
            Self::WindowsPhone => "Windows Phone",
            Self::Windows8 => "Windows 8",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Windows10 => "Application list on the left and a vertically scrolling tile board.",
            Self::WindowsPhone => {
                "Tiles fill Start; swipe left for the application list and right to return."
            }
            Self::Windows8 => {
                "flutter_metro_ui's resizable normal/wide tiles, column-first Metro groups, and horizontal scrolling; swipe up or use the lower-left button for all apps."
            }
        }
    }

    pub const fn tile_flow(self) -> StartTileFlow {
        match self {
            Self::Windows8 => StartTileFlow::Horizontal,
            Self::Windows10 | Self::WindowsPhone => StartTileFlow::Vertical,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StartTileFlow {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StartTileAnimationStyle {
    #[default]
    Windows10Mobile,
    DiscoTurnstile,
    MetroRise,
    FactorSlideScale,
    FlutterMetroGroup,
    /// Internal closing style used by the separate Windows 8 application-launch
    /// preference. It is deliberately omitted from `ALL`: Windows 8's launch
    /// handoff is not a Start entrance option.
    Windows8Launch,
}

impl StartTileAnimationStyle {
    pub const ALL: [Self; 5] = [
        Self::Windows10Mobile,
        Self::DiscoTurnstile,
        Self::MetroRise,
        Self::FactorSlideScale,
        Self::FlutterMetroGroup,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Windows10Mobile => "Windows 10 Mobile",
            Self::DiscoTurnstile => "Disco turnstile",
            Self::MetroRise => "Metro Launcher rise",
            Self::FactorSlideScale => "Factor slide + scale",
            Self::FlutterMetroGroup => "flutter_metro_ui group sweep",
            Self::Windows8Launch => "Windows 8 launch",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Windows10Mobile => {
                "Bottom rows build upward as joined layers that grow and settle downward."
            }
            Self::DiscoTurnstile => "Two-layer 3D tiles turn in from bottom-right toward top-left.",
            Self::MetroRise => "Rows rise 120 epx and fade in over 350 ms.",
            Self::FactorSlideScale => {
                "A quick top-left-to-bottom-right slide, fade, and scale entrance."
            }
            Self::FlutterMetroGroup => {
                "Each tile group moves from the right on its own staggered timeline while fading and scaling from 80%."
            }
            Self::Windows8Launch => {
                "Start recedes while the selected tile flies forward and turns into the app."
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AppLaunchAnimationStyle {
    #[default]
    Windows10Mobile,
    Windows8,
    DiscoTurnstile,
    FactorClipReveal,
    SystemDefault,
}

impl AppLaunchAnimationStyle {
    pub const ALL: [Self; 5] = [
        Self::Windows10Mobile,
        Self::Windows8,
        Self::DiscoTurnstile,
        Self::FactorClipReveal,
        Self::SystemDefault,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Windows10Mobile => "Windows 10 Mobile wave",
            Self::Windows8 => "Windows 8 flying tile",
            Self::DiscoTurnstile => "Disco launch turnstile",
            Self::FactorClipReveal => "Factor clip reveal",
            Self::SystemDefault => "System default (Metro)",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Windows10Mobile => {
                "Tiles lift while Start content fades, then its background waits for the app."
            }
            Self::Windows8 => {
                "Start falls backward while the tapped tile lifts, grows, flips, and covers the screen."
            }
            Self::DiscoTurnstile => {
                "Uses Disco's separate selected-tile or app-list turn-out timing."
            }
            Self::FactorClipReveal => {
                "Expands an application surface from the tapped tile or list row."
            }
            Self::SystemDefault => {
                "Closes Start immediately and leaves the new window motion to the system."
            }
        }
    }

    pub const fn from_start_style(style: StartTileAnimationStyle) -> Self {
        match style {
            StartTileAnimationStyle::Windows10Mobile => Self::Windows10Mobile,
            StartTileAnimationStyle::DiscoTurnstile => Self::DiscoTurnstile,
            StartTileAnimationStyle::MetroRise => Self::SystemDefault,
            StartTileAnimationStyle::FactorSlideScale => Self::FactorClipReveal,
            StartTileAnimationStyle::FlutterMetroGroup => Self::SystemDefault,
            StartTileAnimationStyle::Windows8Launch => Self::Windows8,
        }
    }

    pub const fn reveal_style(self) -> Option<StartTileAnimationStyle> {
        match self {
            Self::Windows10Mobile => Some(StartTileAnimationStyle::Windows10Mobile),
            Self::Windows8 => Some(StartTileAnimationStyle::Windows8Launch),
            Self::DiscoTurnstile => Some(StartTileAnimationStyle::DiscoTurnstile),
            Self::FactorClipReveal => Some(StartTileAnimationStyle::FactorSlideScale),
            Self::SystemDefault => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartTile {
    pub app_id: String,
    #[serde(default = "default_tile_group_id")]
    pub group_id: u32,
    pub col: i32,
    pub row: i32,
    pub w: u8,
    pub h: u8,
}

const fn default_tile_group_id() -> u32 {
    DEFAULT_TILE_GROUP_ID
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartTileGroup {
    pub id: u32,
    pub title: String,
}

impl StartTileGroup {
    fn default_group() -> Self {
        Self {
            id: DEFAULT_TILE_GROUP_ID,
            title: "开始".to_string(),
        }
    }
}

impl StartTile {
    pub fn clamped(mut self) -> Self {
        self.w = self.w.clamp(TILE_MIN, TILE_MAX);
        self.h = self.h.clamp(TILE_MIN, TILE_MAX);
        self.col = self.col.max(0);
        self.row = self.row.max(0);
        self
    }

    fn cells(&self) -> (i32, i32, i32, i32) {
        (
            self.col,
            self.row,
            self.col + i32::from(self.w),
            self.row + i32::from(self.h),
        )
    }

    fn overlaps(&self, other: &Self) -> bool {
        let (a0, b0, a1, b1) = self.cells();
        let (c0, d0, c1, d1) = other.cells();
        a0 < c1 && c0 < a1 && b0 < d1 && d0 < b1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TileFile {
    #[serde(default)]
    flow: StartTileFlow,
    #[serde(default)]
    groups: Vec<StartTileGroup>,
    tiles: Vec<StartTile>,
}

#[derive(Debug, Clone)]
struct PendingMove {
    idx: usize,
    start_pos: Pos2,
}

#[derive(Debug, Clone)]
struct MoveDrag {
    idx: usize,
    /// Pointer offset from tile top-left when grab started.
    grab: Vec2,
    /// Floating visual top-left (follows pointer).
    float_min: Pos2,
    /// Stable layout used to recompute reversible neighbor avoidance each frame.
    layout_before_drag: Vec<StartTile>,
}

#[derive(Debug, Clone)]
struct Windows8MetroDrag {
    app_id: String,
    grab: Vec2,
    float_min: Pos2,
    /// Stable layout captured at drag start; every live reorder preview is a
    /// pure function of this snapshot plus the pointer (same anti-thrash
    /// design as the freeform `drag_preview_layout`).
    layout_before_drag: Vec<StartTile>,
    /// Content origin / viewport captured at drag start so the preview can be
    /// recomputed on drop without the paint frame's geometry.
    origin: Pos2,
    viewport: Vec2,
}

#[derive(Debug, Clone)]
struct GroupRename {
    group_id: u32,
    draft: String,
    focus_pending: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimCurve {
    /// Fluent decelerate — avoid / slide.
    Decelerate,
    /// Overshoot settle — drop snap.
    OutBack,
}

#[derive(Debug, Clone)]
struct PosAnim {
    from: Pos2,
    to: Pos2,
    start: f64,
    dur: f32,
    curve: AnimCurve,
}

#[derive(Debug, Clone)]
struct SizeAnim {
    from: Vec2,
    to: Vec2,
    start: f64,
    dur: f32,
    curve: AnimCurve,
}

#[derive(Debug, Clone)]
struct Windows8MetroPress {
    tilt_x: f32,
    tilt_y: f32,
    from: f32,
    to: f32,
    start: f64,
    duration: f32,
}

impl Windows8MetroPress {
    fn value(&self, now: f64) -> f32 {
        let raw = ((now - self.start) as f32 / self.duration.max(f32::EPSILON)).clamp(0.0, 1.0);
        let eased = if self.to > self.from {
            flutter_ease_out_cubic(raw)
        } else {
            ease_out_back(raw)
        };
        egui::lerp(self.from..=self.to, eased)
    }

    fn finished(&self, now: f64) -> bool {
        now - self.start >= self.duration as f64
    }
}

#[derive(Debug, Default)]
pub struct StartTileBoard {
    pub tiles: Vec<StartTile>,
    groups: Vec<StartTileGroup>,
    flow: StartTileFlow,
    /// Index of tile currently in resize mode (corner drag).
    resize_idx: Option<usize>,
    dragging_handle: bool,
    /// Pending list-style drag (activate after small pointer travel).
    pending_move: Option<PendingMove>,
    /// Active move drag after threshold.
    move_drag: Option<MoveDrag>,
    /// Windows 8 derives positions from tile order, so its drag reorders the
    /// source list (with a Win10-style pending threshold, live preview, and
    /// snap settle) instead of editing freeform cells.
    windows8_drag: Option<Windows8MetroDrag>,
    /// Inline group-title editor shared by every Start layout.
    group_rename: Option<GroupRename>,
    /// Suppress launch after a completed move / long-press.
    suppress_click: bool,
    /// Current painted top-left (screen coords), keyed by app_id.
    visual: HashMap<String, Pos2>,
    /// In-flight nonlinear position animations.
    anims: HashMap<String, PosAnim>,
    /// Current painted size, keyed by app_id.
    visual_size: HashMap<String, Vec2>,
    /// In-flight nonlinear size animations.
    size_anims: HashMap<String, SizeAnim>,
    /// Per-tile 150 ms press / 300 ms release transform used only by the
    /// source-faithful Windows 8 Metro layout.
    windows8_presses: HashMap<String, Windows8MetroPress>,
    /// A group assignment changed while using a derived layout. Repack the
    /// freeform board on its next ordinary Windows 10/Phone paint.
    group_repack_pending: bool,
    /// Last screen-space content origin. Scroll offsets move it between frames;
    /// cached visual positions must move by the same delta instead of easing behind.
    paint_origin: Option<Pos2>,
    dirty: bool,
}

#[derive(Debug, Default)]
pub struct TileBoardOutcome {
    pub launch: Option<String>,
    pub launch_origin: Option<Rect>,
    pub changed: bool,
}

#[derive(Debug, Clone)]
pub struct Windows8MetroTileLayout {
    pub index: usize,
    pub group_index: usize,
    pub rect: Rect,
    pub wide: bool,
}

#[derive(Debug, Clone)]
pub struct Windows8MetroGroupLayout {
    pub id: u32,
    pub title: String,
    pub index: usize,
    pub bounds: Rect,
}

#[derive(Debug, Clone)]
pub struct Windows8MetroLayout {
    pub max_rows: usize,
    pub group_bounds: Rect,
    pub content_size: Vec2,
    pub groups: Vec<Windows8MetroGroupLayout>,
    pub tiles: Vec<Windows8MetroTileLayout>,
}

#[derive(Debug, Clone)]
struct TileGroupLayout {
    id: u32,
    title: String,
    index: usize,
    bounds: Rect,
}

impl StartTileBoard {
    pub fn load() -> Self {
        let mut board = Self::default();
        if let Some(path) = config_path() {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(file) = serde_json::from_str::<TileFile>(&text) {
                    board.flow = file.flow;
                    board.groups = file.groups;
                    board.tiles = file.tiles.into_iter().map(StartTile::clamped).collect();
                }
            }
        }
        board.normalize_groups();
        board
    }

    pub fn save_if_dirty(&mut self) {
        if !self.dirty {
            return;
        }
        let Some(path) = config_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let file = TileFile {
            flow: self.flow,
            groups: self.groups.clone(),
            tiles: self.tiles.clone(),
        };
        if let Ok(text) = serde_json::to_string_pretty(&file) {
            if fs::write(&path, text).is_ok() {
                self.dirty = false;
            }
        }
    }

    pub fn contains_app(&self, app_id: &str) -> bool {
        self.tiles.iter().any(|t| t.app_id == app_id)
    }

    fn normalize_groups(&mut self) {
        if self.groups.is_empty() {
            self.groups.push(StartTileGroup::default_group());
        }
        let fallback = self.groups[0].id;
        for tile in &mut self.tiles {
            if !self.groups.iter().any(|group| group.id == tile.group_id) {
                tile.group_id = fallback;
            }
        }
    }

    fn active_groups(&self) -> Vec<(usize, StartTileGroup)> {
        let groups: Vec<_> = self
            .groups
            .iter()
            .enumerate()
            .filter(|(_, group)| self.tiles.iter().any(|tile| tile.group_id == group.id))
            .map(|(index, group)| (index, group.clone()))
            .collect();
        if groups.is_empty() {
            vec![(0, StartTileGroup::default_group())]
        } else {
            groups
        }
    }

    pub fn group_count(&self) -> usize {
        self.active_groups().len().max(1)
    }

    fn group_index(&self, group_id: u32) -> usize {
        self.active_groups()
            .iter()
            .position(|(_, group)| group.id == group_id)
            .unwrap_or(0)
    }

    fn prune_empty_groups(&mut self) {
        self.groups.retain(|group| {
            group.id == DEFAULT_TILE_GROUP_ID
                || self.tiles.iter().any(|tile| tile.group_id == group.id)
        });
        self.normalize_groups();
    }

    fn move_tile_to_group(&mut self, tile_index: usize, group_id: u32) {
        if tile_index >= self.tiles.len()
            || !self.groups.iter().any(|group| group.id == group_id)
            || self.tiles[tile_index].group_id == group_id
        {
            return;
        }
        self.tiles[tile_index].group_id = group_id;
        self.prune_empty_groups();
        self.clear_interaction();
        self.group_repack_pending = true;
        self.dirty = true;
    }

    fn move_tile_to_new_group(&mut self, tile_index: usize) {
        if tile_index >= self.tiles.len() {
            return;
        }
        let next_id = self
            .groups
            .iter()
            .map(|group| group.id)
            .max()
            .unwrap_or(DEFAULT_TILE_GROUP_ID)
            .saturating_add(1);
        let title = format!("组 {}", self.groups.len() + 1);
        self.groups.push(StartTileGroup { id: next_id, title });
        self.tiles[tile_index].group_id = next_id;
        self.prune_empty_groups();
        self.clear_interaction();
        self.group_repack_pending = true;
        self.dirty = true;
    }

    fn begin_group_rename(&mut self, group_id: u32) {
        let Some(group) = self.groups.iter().find(|group| group.id == group_id) else {
            return;
        };
        self.group_rename = Some(GroupRename {
            group_id,
            draft: group.title.clone(),
            focus_pending: true,
        });
    }

    fn finish_group_rename(&mut self, commit: bool) {
        let Some(rename) = self.group_rename.take() else {
            return;
        };
        if !commit {
            return;
        }
        let title = rename.draft.trim();
        let title = if title.is_empty() {
            "未命名组"
        } else {
            title
        };
        if let Some(group) = self
            .groups
            .iter_mut()
            .find(|group| group.id == rename.group_id)
        {
            if group.title != title {
                group.title = title.to_string();
                self.dirty = true;
            }
        }
    }

    fn paint_editable_group_title(
        &mut self,
        ui: &mut Ui,
        group_id: u32,
        title: &str,
        rect: Rect,
        font: FontId,
        color: Color32,
        id_source: &'static str,
        editable: bool,
    ) {
        if editable
            && self
                .group_rename
                .as_ref()
                .is_some_and(|rename| rename.group_id == group_id)
        {
            let mut commit = false;
            let mut cancel = false;
            if let Some(rename) = self.group_rename.as_mut() {
                let edit = egui::TextEdit::singleline(&mut rename.draft)
                    .id(ui.id().with((id_source, "editor", group_id)))
                    .font(font)
                    .desired_width(rect.width())
                    .text_color(color);
                let response = ui.put(rect, edit);
                if rename.focus_pending {
                    response.request_focus();
                    rename.focus_pending = false;
                }
                if response.has_focus() {
                    commit = ui.input(|input| input.key_pressed(egui::Key::Enter));
                    cancel = ui.input(|input| input.key_pressed(egui::Key::Escape));
                }
                commit |= response.lost_focus() && !cancel;
            }
            if cancel {
                self.finish_group_rename(false);
            } else if commit {
                self.finish_group_rename(true);
                self.save_if_dirty();
            } else {
                ui.ctx().request_repaint();
            }
            return;
        }

        ui.painter()
            .text(rect.min, Align2::LEFT_TOP, title, font, color);
        if !editable {
            return;
        }
        let response = ui.interact(
            rect,
            ui.id().with((id_source, "label", group_id)),
            Sense::click(),
        );
        let mut rename_requested = response.clicked();
        response.context_menu(|ui| {
            if ui.button("重命名").clicked() {
                rename_requested = true;
                ui.close();
            }
        });
        if rename_requested {
            self.begin_group_rename(group_id);
            ui.ctx().request_repaint();
        }
    }

    pub fn pin_app(&mut self, app_id: &str, flow: StartTileFlow, primary_limit: i32) {
        if self.contains_app(app_id) {
            return;
        }
        self.normalize_groups();
        self.ensure_flow(flow, primary_limit);
        let w = TILE_DEFAULT;
        let h = TILE_DEFAULT;
        let (col, row) = find_free_slot_flow(&self.tiles, w, h, primary_limit, flow);
        self.tiles.push(
            StartTile {
                app_id: app_id.to_string(),
                group_id: DEFAULT_TILE_GROUP_ID,
                col,
                row,
                w,
                h,
            }
            .clamped(),
        );
        self.dirty = true;
    }

    /// Preview rect for a default-size tile under `pointer` (for list→board drag).
    pub fn drop_preview(
        area: Rect,
        pointer: Pos2,
        flow: StartTileFlow,
    ) -> Option<(i32, i32, Rect)> {
        if !area.contains(pointer) {
            return None;
        }
        let (unit, gap) = cell_metrics(area);
        let stride = unit + gap;
        let grid_origin = area.min + Vec2::new(0.0, TILE_GROUP_HEADER_GUTTER);
        let max_cols = ((area.width() + gap) / stride).floor().max(1.0) as i32;
        let max_rows = ((area.height() + gap) / stride).floor().max(1.0) as i32;
        let w = i32::from(TILE_DEFAULT);
        let h = i32::from(TILE_DEFAULT);
        let col = ((pointer.x - grid_origin.x) / stride)
            .floor()
            .clamp(0.0, (max_cols - w).max(0) as f32) as i32;
        let row = ((pointer.y - grid_origin.y) / stride)
            .floor()
            .clamp(0.0, (max_rows - h).max(0) as f32) as i32;
        let (col, row) = match flow {
            StartTileFlow::Vertical => (col, row),
            StartTileFlow::Horizontal => (col, row),
        };
        let tile = StartTile {
            app_id: String::new(),
            group_id: DEFAULT_TILE_GROUP_ID,
            col,
            row,
            w: TILE_DEFAULT,
            h: TILE_DEFAULT,
        };
        Some((col, row, tile_pixel_rect(grid_origin, &tile, unit, gap)))
    }

    /// Pin from an app-list drag; animates from `pointer` onto the grid with avoid.
    pub fn pin_from_list_drop(
        &mut self,
        app_id: &str,
        pointer: Pos2,
        area: Rect,
        now: f64,
        flow: StartTileFlow,
    ) -> bool {
        if self.contains_app(app_id) || !area.contains(pointer) {
            return false;
        }
        self.normalize_groups();
        let (unit, gap) = cell_metrics(area);
        let stride = unit + gap;
        let grid_origin = area.min + Vec2::new(0.0, TILE_GROUP_HEADER_GUTTER);
        let max_cols = ((area.width() + gap) / stride).floor().max(1.0) as i32;
        let max_rows = ((area.height() + gap) / stride).floor().max(1.0) as i32;
        let (mut col, mut row, preview) = match Self::drop_preview(area, pointer, flow) {
            Some(v) => v,
            None => return false,
        };
        let probe = StartTile {
            app_id: String::new(),
            group_id: DEFAULT_TILE_GROUP_ID,
            col,
            row,
            w: TILE_DEFAULT,
            h: TILE_DEFAULT,
        };
        let blocked = self.tiles.iter().any(|t| probe.overlaps(t));
        if blocked {
            let primary_limit = match flow {
                StartTileFlow::Vertical => max_cols,
                StartTileFlow::Horizontal => max_rows,
            };
            let (c, r) =
                find_free_slot_flow(&self.tiles, TILE_DEFAULT, TILE_DEFAULT, primary_limit, flow);
            col = c;
            row = r;
        }
        let _ = (stride, preview);
        let tile = StartTile {
            app_id: app_id.to_string(),
            group_id: DEFAULT_TILE_GROUP_ID,
            col,
            row,
            w: TILE_DEFAULT,
            h: TILE_DEFAULT,
        }
        .clamped();
        let target = tile_pixel_rect(grid_origin, &tile, unit, gap);
        let float_min = pointer - target.size() * 0.5;
        self.tiles.push(tile);
        let idx = self.tiles.len() - 1;
        resolve_avoid(&mut self.tiles, idx, max_cols, max_rows);
        let target = tile_pixel_rect(grid_origin, &self.tiles[idx], unit, gap).min;
        self.visual.insert(app_id.to_string(), float_min);
        self.anims.insert(
            app_id.to_string(),
            PosAnim {
                from: float_min,
                to: target,
                start: now,
                dur: SNAP_DUR,
                curve: AnimCurve::OutBack,
            },
        );
        self.retarget_anims(
            grid_origin,
            unit,
            gap,
            now,
            Some(idx),
            AnimCurve::Decelerate,
            AVOID_DUR,
        );
        // Keep the drop snap on the new tile.
        if let Some(a) = self.anims.get_mut(app_id) {
            a.from = float_min;
            a.to = target;
            a.start = now;
            a.dur = SNAP_DUR;
            a.curve = AnimCurve::OutBack;
        }
        self.dirty = true;
        true
    }

    pub fn ensure_flow(&mut self, flow: StartTileFlow, primary_limit: i32) {
        if self.flow == flow {
            return;
        }
        self.confirm_resize();
        self.tiles = pack_tiles_grouped(&self.tiles, &self.groups, flow, primary_limit);
        self.flow = flow;
        self.group_repack_pending = false;
        self.visual.clear();
        self.anims.clear();
        self.visual_size.clear();
        self.size_anims.clear();
        self.windows8_presses.clear();
        self.paint_origin = None;
        self.dirty = true;
    }

    pub fn content_size(&self, viewport: Rect, flow: StartTileFlow) -> Vec2 {
        let (unit, gap) = cell_metrics(viewport);
        let stride = unit + gap;
        let occupied_right = self
            .tiles
            .iter()
            .map(|tile| tile.col + i32::from(tile.w))
            .max()
            .unwrap_or(0);
        let occupied_bottom = self
            .tiles
            .iter()
            .map(|tile| tile.row + i32::from(tile.h))
            .max()
            .unwrap_or(0);
        match flow {
            StartTileFlow::Vertical => Vec2::new(
                viewport.width(),
                viewport
                    .height()
                    .max(TILE_GROUP_HEADER_GUTTER + (occupied_bottom + 8) as f32 * stride - gap),
            ),
            StartTileFlow::Horizontal => Vec2::new(
                viewport
                    .width()
                    .max((occupied_right + 8) as f32 * stride - gap),
                viewport
                    .height()
                    .max(TILE_GROUP_HEADER_GUTTER + stride - gap),
            ),
        }
    }

    fn tile_group_layouts(&self, grid_origin: Pos2, unit: f32, gap: f32) -> Vec<TileGroupLayout> {
        self.active_groups()
            .into_iter()
            .enumerate()
            .filter_map(|(index, (_, group))| {
                self.tiles
                    .iter()
                    .filter(|tile| tile.group_id == group.id)
                    .map(|tile| tile_pixel_rect(grid_origin, tile, unit, gap))
                    .reduce(|bounds, rect| bounds.union(rect))
                    .map(|bounds| TileGroupLayout {
                        id: group.id,
                        title: group.title,
                        index,
                        bounds,
                    })
            })
            .collect()
    }

    /// Source-faithful Windows 8 Start geometry from flutter_metro_ui.
    pub fn windows8_metro_layout(&self, origin: Pos2, viewport: Vec2) -> Windows8MetroLayout {
        let groups: Vec<_> = self
            .active_groups()
            .into_iter()
            .map(|(_, group)| group)
            .collect();
        windows8_metro_layout(&self.tiles, &groups, origin, viewport)
    }

    pub fn windows8_metro_max_rows(viewport_height: f32) -> i32 {
        (((viewport_height - WINDOWS8_METRO_TOP_PADDING - WINDOWS8_METRO_BOTTOM_PADDING)
            / (WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP))
            .floor() as i32)
            .clamp(1, 6)
    }

    pub fn windows8_metro_content_size(&self, viewport: Rect) -> Vec2 {
        self.windows8_metro_layout(viewport.min, viewport.size())
            .content_size
    }

    /// Paint the fixed-size, column-first Metro groups used by the Windows 8
    /// layout. Resize keeps the source project's two legal faces: normal and wide.
    pub fn paint_windows8_metro(
        &mut self,
        ui: &mut Ui,
        content_area: Rect,
        viewport_size: Vec2,
        apps: &[win10x_protocol::AppEntry],
        _pal: &Palette,
        icons: &IconCache,
        reveal: Option<StartTileReveal>,
        launching_app_id: Option<&str>,
    ) -> TileBoardOutcome {
        let mut out = TileBoardOutcome::default();
        let mut layout = self.windows8_metro_layout(content_area.min, viewport_size);
        debug_assert!((1..=6).contains(&layout.max_rows));
        let now = ui.input(|input| input.time);
        let revealing = reveal.is_some();
        let pointer = ui.ctx().pointer_interact_pos();
        let primary_down = ui.input(|input| input.pointer.primary_down());
        let primary_released = ui.input(|input| input.pointer.primary_released());
        let was_dragging = self.windows8_drag.is_some();
        self.rebase_paint_origin(content_area.min);
        if self.suppress_click && (primary_released || !primary_down) {
            self.suppress_click = false;
        }

        // —— Win10-style pending move: activate after DRAG_ACTIVATE travel ——
        if !revealing && self.resize_idx.is_none() && self.windows8_drag.is_none() {
            if let Some(pending) = self.pending_move.clone() {
                if !primary_down {
                    self.pending_move = None;
                } else if let Some(pos) = pointer {
                    if pos.distance(pending.start_pos) > DRAG_ACTIVATE {
                        if let Some(tile_layout) =
                            layout.tiles.iter().find(|tile| tile.index == pending.idx)
                        {
                            if let Some(tile) = self.tiles.get(pending.idx) {
                                let app_id = tile.app_id.clone();
                                let vmin = self.visual_min(&app_id, tile_layout.rect.min);
                                self.windows8_presses.remove(&app_id);
                                self.anims.remove(&app_id);
                                self.windows8_drag = Some(Windows8MetroDrag {
                                    app_id,
                                    grab: pos - vmin,
                                    float_min: vmin,
                                    layout_before_drag: self.tiles.clone(),
                                    origin: content_area.min,
                                    viewport: viewport_size,
                                });
                                self.suppress_click = true;
                                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                            }
                        }
                        self.pending_move = None;
                    }
                }
            }
        }

        // —— Active drag: follow the pointer and live-reorder from the stable
        //    pre-drag snapshot so neighbors slide out of the way (Win10/Phone
        //    behavior) while Metro style and grouping stay intact. ——
        if let Some(drag) = self.windows8_drag.clone() {
            if primary_down {
                if let Some(pos) = pointer {
                    let preview = windows8_drag_preview(
                        &drag.layout_before_drag,
                        &self.groups,
                        &drag.app_id,
                        pos,
                        drag.origin,
                        drag.viewport,
                    );
                    let mut next = drag.clone();
                    next.float_min = pos - drag.grab;
                    self.windows8_drag = Some(next);
                    if preview != self.tiles {
                        self.tiles = preview;
                        self.dirty = true;
                        self.group_repack_pending = true;
                        out.changed = true;
                        layout = self.windows8_metro_layout(content_area.min, viewport_size);
                    }
                }
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                ui.ctx().request_repaint();
            }
        }
        let background = ui.interact(
            content_area,
            ui.id().with("windows8_metro_background"),
            Sense::click(),
        );

        if let Some(index) = self.resize_idx.filter(|_| self.dragging_handle) {
            if let Some(tile_layout) = layout.tiles.iter().find(|tile| tile.index == index) {
                if let Some(pointer) = pointer {
                    let wide = pointer.x >= tile_layout.rect.min.x + WINDOWS8_METRO_TILE * 1.5;
                    let (w, h) = if wide { (4, 2) } else { (2, 2) };
                    if self.tiles[index].w != w || self.tiles[index].h != h {
                        self.tiles[index].w = w;
                        self.tiles[index].h = h;
                        self.dirty = true;
                        out.changed = true;
                        ui.ctx().request_repaint();
                    }
                }
            }
            if primary_released {
                self.dragging_handle = false;
                self.save_if_dirty();
            }
        }

        let launch_frame = reveal.and_then(|frame| frame.windows8_launch_frame());
        let retreat = launch_frame.map_or(0.0, |frame| frame.retreat);
        let retreat_scale = 1.0 - 0.28 * retreat;
        let retreat_origin = layout.group_bounds.center();
        let retreat_transform =
            |point: Pos2| retreat_origin + (point - retreat_origin) * retreat_scale;

        // Win10-style eased reflow for non-dragged Metro tiles. The entrance
        // and launch transforms own the geometry while they run, so settle the
        // shared visual maps instead of fighting them.
        let settled = reveal.is_none() && retreat == 0.0 && launch_frame.is_none();
        let dragged_app_id = self.windows8_drag.as_ref().map(|drag| drag.app_id.clone());
        if settled {
            if self.windows8_step_visuals(&layout, now, dragged_app_id.as_deref()) {
                ui.ctx().request_repaint();
            }
        } else {
            self.visual.clear();
            self.anims.clear();
        }

        self.windows8_presses
            .retain(|_, press| !(press.to <= 0.0 && press.finished(now)));
        if self
            .windows8_presses
            .values()
            .any(|press| !press.finished(now))
        {
            ui.ctx().request_repaint();
        }

        for group in &layout.groups {
            let group_progress = reveal.map_or(1.0, |frame| {
                if frame.style == StartTileAnimationStyle::FlutterMetroGroup {
                    frame.flutter_metro_group_progress(group.index)
                } else {
                    1.0
                }
            });
            let group_scale = 0.8 + 0.2 * group_progress;
            let group_translate = Vec2::new(100.0 * (1.0 - group_progress), 0.0);
            let group_transform = |point: Pos2| {
                group.bounds.center()
                    + (point - group.bounds.center()) * group_scale
                    + group_translate
            };
            let scene_opacity = group_progress * (1.0 - retreat).powf(1.35);
            if scene_opacity <= 0.001 {
                continue;
            }
            let group_title_pos = retreat_transform(group_transform(group.bounds.min));
            let title_rect = Rect::from_min_size(
                group_title_pos,
                Vec2::new(
                    (group.bounds.width() - WINDOWS8_METRO_GROUP_GAP).max(WINDOWS8_METRO_TILE)
                        * group_scale
                        * retreat_scale,
                    WINDOWS8_METRO_GROUP_TITLE_HEIGHT,
                ),
            );
            self.paint_editable_group_title(
                ui,
                group.id,
                &group.title,
                title_rect,
                FontId::proportional(20.0 * group_scale * retreat_scale),
                color_with_opacity(Color32::from_gray(128), scene_opacity),
                "windows8_group_title",
                !revealing,
            );
        }

        let mut unpin: Option<String> = None;
        let mut start_resize: Option<usize> = None;
        let mut move_to_group: Option<(usize, u32)> = None;
        let mut move_to_new_group: Option<usize> = None;
        let group_choices: Vec<_> = self
            .groups
            .iter()
            .map(|group| (group.id, group.title.clone()))
            .collect();
        let dragged_tile_index = self.windows8_drag.as_ref().and_then(|drag| {
            self.tiles
                .iter()
                .position(|tile| tile.app_id == drag.app_id)
        });
        let mut paint_order: Vec<usize> = (0..layout.tiles.len()).collect();
        if let Some(dragged_tile_index) = dragged_tile_index {
            if let Some(position) = paint_order
                .iter()
                .position(|&position| layout.tiles[position].index == dragged_tile_index)
            {
                let position = paint_order.remove(position);
                paint_order.push(position);
            }
        }
        for layout_position in paint_order {
            let tile_layout = &layout.tiles[layout_position];
            let Some(tile) = self.tiles.get(tile_layout.index).cloned() else {
                continue;
            };
            let Some(app) = apps.iter().find(|app| app.id == tile.app_id) else {
                continue;
            };
            if launch_frame.is_some() && launching_app_id == Some(tile.app_id.as_str()) {
                continue;
            }

            let Some(group) = layout
                .groups
                .iter()
                .find(|group| group.index == tile_layout.group_index)
            else {
                continue;
            };
            let group_progress = reveal.map_or(1.0, |frame| {
                if frame.style == StartTileAnimationStyle::FlutterMetroGroup {
                    frame.flutter_metro_group_progress(group.index)
                } else {
                    1.0
                }
            });
            let group_scale = 0.8 + 0.2 * group_progress;
            let group_translate = Vec2::new(100.0 * (1.0 - group_progress), 0.0);
            let group_transform = |point: Pos2| {
                group.bounds.center()
                    + (point - group.bounds.center()) * group_scale
                    + group_translate
            };
            let scene_opacity = group_progress * (1.0 - retreat).powf(1.35);

            let base_rect = Rect::from_min_max(
                retreat_transform(group_transform(tile_layout.rect.min)),
                retreat_transform(group_transform(tile_layout.rect.max)),
            );
            let mut rect = if settled {
                Rect::from_min_size(
                    self.visual_min(&tile.app_id, base_rect.min),
                    base_rect.size(),
                )
            } else {
                base_rect
            };
            rect = self
                .windows8_drag
                .as_ref()
                .filter(|drag| drag.app_id == tile.app_id)
                .map_or(rect, |drag| {
                    Rect::from_min_size(drag.float_min, rect.size())
                });
            if self
                .windows8_drag
                .as_ref()
                .is_some_and(|drag| drag.app_id == tile.app_id)
            {
                rect = Rect::from_center_size(rect.center(), rect.size() * 1.03);
            }
            if !ui.clip_rect().intersects(rect) && !base_rect.contains(pointer.unwrap_or_default())
            {
                continue;
            }
            let response = ui.interact(
                rect,
                ui.id()
                    .with(("windows8_metro_tile", tile_layout.index, &tile.app_id)),
                if revealing {
                    Sense::hover()
                } else if self.resize_idx.is_some() {
                    Sense::click()
                } else {
                    Sense::click_and_drag()
                },
            );

            if !revealing
                && self.resize_idx.is_none()
                && self.windows8_drag.is_none()
                && self.pending_move.is_none()
                && response.drag_started()
            {
                if let Some(pointer) = response.interact_pointer_pos() {
                    self.pending_move = Some(PendingMove {
                        idx: tile_layout.index,
                        start_pos: pointer,
                    });
                    self.windows8_presses.remove(&tile.app_id);
                    ui.ctx().request_repaint();
                }
            }

            let tile_dragging = self
                .windows8_drag
                .as_ref()
                .is_some_and(|drag| drag.app_id == tile.app_id);
            let pointer_down = !revealing && !tile_dragging && response.is_pointer_button_down_on();
            let current_press = self
                .windows8_presses
                .get(&tile.app_id)
                .map_or(0.0, |press| press.value(now));
            if pointer_down {
                let needs_press = self
                    .windows8_presses
                    .get(&tile.app_id)
                    .is_none_or(|press| press.to < 1.0);
                if needs_press {
                    let pointer = response.interact_pointer_pos().unwrap_or(rect.center());
                    let percent_x = ((pointer.x - rect.center().x) / (rect.width() * 0.5).max(1.0))
                        .clamp(-1.2, 1.2);
                    let percent_y = ((pointer.y - rect.center().y)
                        / (rect.height() * 0.5).max(1.0))
                    .clamp(-1.2, 1.2);
                    self.windows8_presses.insert(
                        tile.app_id.clone(),
                        Windows8MetroPress {
                            tilt_x: percent_y * 0.12,
                            tilt_y: -percent_x * 0.12,
                            from: current_press,
                            to: 1.0,
                            start: now,
                            duration: WINDOWS8_METRO_PRESS_IN,
                        },
                    );
                    ui.ctx().request_repaint();
                }
            } else if self
                .windows8_presses
                .get(&tile.app_id)
                .is_some_and(|press| press.to > 0.0)
            {
                let press = self.windows8_presses.get(&tile.app_id).cloned().unwrap();
                self.windows8_presses.insert(
                    tile.app_id.clone(),
                    Windows8MetroPress {
                        tilt_x: press.tilt_x,
                        tilt_y: press.tilt_y,
                        from: current_press,
                        to: 0.0,
                        start: now,
                        duration: WINDOWS8_METRO_PRESS_OUT,
                    },
                );
                ui.ctx().request_repaint();
            }

            let press = self.windows8_presses.get(&tile.app_id);
            let press_progress = press.map_or(0.0, |press| press.value(now));
            let press_scale = 1.0 - 0.04 * press_progress;
            let tilt_x = press.map_or(0.0, |press| press.tilt_x * press_progress);
            let tilt_y = press.map_or(0.0, |press| press.tilt_y * press_progress);
            let quad = windows8_metro_press_quad(rect, press_scale, tilt_x, tilt_y);
            let fill = color_with_opacity(windows8_metro_fill(tile_layout.index), scene_opacity);
            let fg = color_with_opacity(Color32::WHITE, scene_opacity);
            let border = if response.hovered() || pointer_down {
                Stroke::new(2.0, color_with_opacity(Color32::WHITE, scene_opacity * 0.4))
            } else {
                Stroke::NONE
            };
            if tile_dragging {
                ui.painter().rect_filled(
                    rect.translate(Vec2::new(5.0, 7.0)),
                    CornerRadius::ZERO,
                    Color32::from_black_alpha(80),
                );
            }
            paint_projected_quad(ui, quad, fill, border);

            let icon_center =
                windows8_metro_press_point(rect, rect.center(), press_scale, tilt_x, tilt_y);
            paint_app_icon(
                ui,
                icons,
                app,
                icon_center,
                50.0 * group_scale * retreat_scale * press_scale,
                fg,
            );
            let label_anchor = windows8_metro_press_point(
                rect,
                Pos2::new(rect.min.x + 10.0, rect.max.y - 6.0),
                press_scale,
                tilt_x,
                tilt_y,
            );
            ui.painter().text(
                label_anchor,
                Align2::LEFT_BOTTOM,
                truncate_label(&app.name, if tile_layout.wide { 32 } else { 15 }),
                FontId::proportional(13.0 * group_scale * retreat_scale * press_scale),
                fg,
            );

            response.context_menu(|ui| {
                if ui.button("更改大小").clicked() {
                    start_resize = Some(tile_layout.index);
                    ui.close();
                }
                ui.menu_button("移动到组", |ui| {
                    for (group_id, title) in &group_choices {
                        if *group_id != tile.group_id && ui.button(title).clicked() {
                            move_to_group = Some((tile_layout.index, *group_id));
                            ui.close();
                        }
                    }
                    ui.separator();
                    if ui.button("新建组").clicked() {
                        move_to_new_group = Some(tile_layout.index);
                        ui.close();
                    }
                });
                if ui.button("取消固定").clicked() {
                    unpin = Some(tile.app_id.clone());
                    ui.close();
                }
            });

            if self.resize_idx == Some(tile_layout.index) && !revealing {
                let handle = resize_handle_rect(rect);
                ui.painter().rect_filled(
                    handle,
                    CornerRadius::same(2),
                    Color32::from_rgba_unmultiplied(255, 255, 255, 230),
                );
                ui.painter().text(
                    handle.center(),
                    Align2::CENTER_CENTER,
                    "⋰",
                    FontId::proportional(14.0),
                    Color32::from_rgb(0x00, 0x78, 0xd4),
                );
                let handle_response = ui.interact(
                    handle,
                    ui.id().with(("windows8_tile_handle", tile_layout.index)),
                    Sense::drag(),
                );
                if handle_response.drag_started() || handle_response.dragged() {
                    self.dragging_handle = true;
                    ui.ctx().request_repaint();
                }
                if handle_response.hovered() || self.dragging_handle {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeEast);
                }
            }

            if self.resize_idx.is_none()
                && self.windows8_drag.is_none()
                && self.pending_move.is_none()
                && !was_dragging
                && !self.suppress_click
                && response.clicked()
            {
                out.launch = Some(tile.app_id.clone());
                out.launch_origin = Some(rect);
            }
        }

        if self.windows8_drag.is_some() && primary_released {
            if self.finish_windows8_drag(pointer, now) {
                out.changed = true;
            }
            ui.ctx().request_repaint();
        }
        if self.pending_move.is_some() || self.windows8_drag.is_some() {
            ui.ctx().request_repaint();
        }

        if let Some(index) = start_resize {
            self.clear_interaction();
            self.resize_idx = Some(index);
        }
        if let Some((index, group_id)) = move_to_group {
            self.move_tile_to_group(index, group_id);
            out.changed = true;
        }
        if let Some(index) = move_to_new_group {
            self.move_tile_to_new_group(index);
            out.changed = true;
        }
        if let Some(app_id) = unpin {
            self.unpin_app(&app_id);
            self.prune_empty_groups();
            out.changed = true;
        }
        if self.resize_idx.is_some()
            && !self.dragging_handle
            && background.clicked()
            && start_resize.is_none()
        {
            self.confirm_resize();
            out.changed = true;
        }
        out
    }

    /// Commit a Windows 8 Metro drag. The live preview already reorders the
    /// source list while the pointer moves; this recomputes the preview once
    /// more for the release position, settles the floating tile into its
    /// derived slot with the Win10 OutBack snap, and persists the result.
    fn finish_windows8_drag(&mut self, pointer: Option<Pos2>, now: f64) -> bool {
        let Some(drag) = self.windows8_drag.take() else {
            return false;
        };
        let preview = pointer.map_or_else(
            || self.tiles.clone(),
            |pos| {
                windows8_drag_preview(
                    &drag.layout_before_drag,
                    &self.groups,
                    &drag.app_id,
                    pos,
                    drag.origin,
                    drag.viewport,
                )
            },
        );
        let changed = preview != drag.layout_before_drag;
        self.tiles = preview;
        self.prune_empty_groups();
        if changed {
            self.group_repack_pending = true;
            self.dirty = true;
        }

        // Settle the floating tile into its derived Metro slot.
        let layout = self.windows8_metro_layout(drag.origin, drag.viewport);
        if let Some(slot) = self
            .tiles
            .iter()
            .position(|tile| tile.app_id == drag.app_id)
            .and_then(|index| layout.tiles.iter().find(|tile| tile.index == index))
        {
            self.visual.insert(drag.app_id.clone(), drag.float_min);
            self.anims.insert(
                drag.app_id.clone(),
                PosAnim {
                    from: drag.float_min,
                    to: slot.rect.min,
                    start: now,
                    dur: SNAP_DUR,
                    curve: AnimCurve::OutBack,
                },
            );
        }
        self.windows8_presses.clear();
        self.save_if_dirty();
        changed
    }

    /// Ease non-dragged Metro tiles toward their derived slots, mirroring the
    /// freeform board's `step_anims` reflow feedback. Returns true while any
    /// tile is still moving.
    fn windows8_step_visuals(
        &mut self,
        layout: &Windows8MetroLayout,
        now: f64,
        skip_app: Option<&str>,
    ) -> bool {
        let mut busy = false;
        let live: std::collections::HashSet<String> = layout
            .tiles
            .iter()
            .filter_map(|tile| self.tiles.get(tile.index).map(|tile| tile.app_id.clone()))
            .collect();
        self.visual.retain(|app_id, _| live.contains(app_id));
        self.anims.retain(|app_id, _| live.contains(app_id));

        for tile_layout in &layout.tiles {
            let Some(tile) = self.tiles.get(tile_layout.index) else {
                continue;
            };
            if Some(tile.app_id.as_str()) == skip_app {
                self.anims.remove(&tile.app_id);
                continue;
            }
            let target = tile_layout.rect.min;
            if let Some(anim) = self.anims.get(&tile.app_id).cloned() {
                if (anim.to - target).length_sq() < 0.25 {
                    let t = ((now - anim.start) as f32 / anim.dur).clamp(0.0, 1.0);
                    let e = match anim.curve {
                        AnimCurve::Decelerate => ease_decelerate(t),
                        AnimCurve::OutBack => ease_out_back(t),
                    };
                    let p = Pos2::new(
                        anim.from.x + (anim.to.x - anim.from.x) * e,
                        anim.from.y + (anim.to.y - anim.from.y) * e,
                    );
                    self.visual.insert(tile.app_id.clone(), p);
                    if t >= 1.0 {
                        self.visual.insert(tile.app_id.clone(), anim.to);
                        self.anims.remove(&tile.app_id);
                    } else {
                        busy = true;
                    }
                    continue;
                }
            }
            let from = self.visual_min(&tile.app_id, target);
            if (from - target).length_sq() < 0.25 {
                self.anims.remove(&tile.app_id);
                self.visual.insert(tile.app_id.clone(), target);
                continue;
            }
            self.anims.insert(
                tile.app_id.clone(),
                PosAnim {
                    from,
                    to: target,
                    start: now,
                    dur: AVOID_DUR,
                    curve: AnimCurve::Decelerate,
                },
            );
            busy = true;
        }
        busy
    }

    fn rebase_paint_origin(&mut self, origin: Pos2) {
        let Some(previous) = self.paint_origin.replace(origin) else {
            return;
        };
        let delta = origin - previous;
        if delta.length_sq() < 0.01 {
            return;
        }
        let dragged_id = self
            .move_drag
            .as_ref()
            .and_then(|drag| self.tiles.get(drag.idx))
            .map(|tile| tile.app_id.as_str())
            .or_else(|| self.windows8_drag.as_ref().map(|drag| drag.app_id.as_str()));
        for (app_id, position) in &mut self.visual {
            if dragged_id != Some(app_id.as_str()) {
                *position += delta;
            }
        }
        for animation in self.anims.values_mut() {
            animation.from += delta;
            animation.to += delta;
        }
    }

    pub fn unpin_app(&mut self, app_id: &str) {
        self.cancel_move_drag();
        let before = self.tiles.len();
        self.tiles.retain(|t| t.app_id != app_id);
        if self.tiles.len() != before {
            self.prune_empty_groups();
            self.visual.remove(app_id);
            self.anims.remove(app_id);
            self.visual_size.remove(app_id);
            self.size_anims.remove(app_id);
            self.clear_interaction();
            self.dirty = true;
        }
    }

    pub fn confirm_resize(&mut self) {
        self.cancel_move_drag();
        self.cancel_windows8_drag();
        self.finish_group_rename(true);
        self.resize_idx = None;
        self.dragging_handle = false;
        self.pending_move = None;
        self.save_if_dirty();
    }

    fn clear_interaction(&mut self) {
        self.cancel_move_drag();
        self.cancel_windows8_drag();
        self.finish_group_rename(true);
        self.resize_idx = None;
        self.dragging_handle = false;
        self.pending_move = None;
        self.windows8_presses.clear();
    }

    fn cancel_move_drag(&mut self) {
        let Some(drag) = self.move_drag.take() else {
            return;
        };
        if drag.layout_before_drag.len() == self.tiles.len() {
            self.tiles = drag.layout_before_drag;
        }
    }

    /// Restore the pre-drag Metro order when a Windows 8 drag is aborted
    /// before its release settle.
    fn cancel_windows8_drag(&mut self) {
        let Some(drag) = self.windows8_drag.take() else {
            return;
        };
        if self.tiles != drag.layout_before_drag {
            self.tiles = drag.layout_before_drag;
            self.group_repack_pending = true;
            self.dirty = true;
        }
    }

    fn visual_min(&self, app_id: &str, fallback: Pos2) -> Pos2 {
        self.visual.get(app_id).copied().unwrap_or(fallback)
    }

    /// Retarget position animations toward current grid slots.
    fn retarget_anims(
        &mut self,
        origin: Pos2,
        unit: f32,
        gap: f32,
        now: f64,
        skip: Option<usize>,
        curve: AnimCurve,
        dur: f32,
    ) {
        let live: std::collections::HashSet<String> =
            self.tiles.iter().map(|t| t.app_id.clone()).collect();
        self.visual.retain(|k, _| live.contains(k));
        self.anims.retain(|k, _| live.contains(k));

        for (i, tile) in self.tiles.iter().enumerate() {
            if Some(i) == skip {
                self.anims.remove(&tile.app_id);
                continue;
            }
            let target = tile_pixel_rect(origin, tile, unit, gap).min;
            let from = self.visual_min(&tile.app_id, target);
            if (from - target).length_sq() < 0.25 {
                self.anims.remove(&tile.app_id);
                self.visual.insert(tile.app_id.clone(), target);
                continue;
            }
            let same_target = self
                .anims
                .get(&tile.app_id)
                .is_some_and(|a| (a.to - target).length_sq() < 0.25);
            if same_target {
                // Keep running anim, but refresh endpoint if needed.
                if let Some(a) = self.anims.get_mut(&tile.app_id) {
                    a.to = target;
                }
                continue;
            }
            self.anims.insert(
                tile.app_id.clone(),
                PosAnim {
                    from,
                    to: target,
                    start: now,
                    dur,
                    curve,
                },
            );
        }
    }

    /// Advance nonlinear position anims; returns true if still moving.
    fn step_anims(&mut self, now: f64, origin: Pos2, unit: f32, gap: f32) -> bool {
        let mut busy = false;
        let drag_idx = self.move_drag.as_ref().map(|d| d.idx);
        let float = self.move_drag.as_ref().map(|d| d.float_min);

        for (i, tile) in self.tiles.iter().enumerate() {
            let target = tile_pixel_rect(origin, tile, unit, gap).min;
            if drag_idx == Some(i) {
                if let Some(p) = float {
                    self.visual.insert(tile.app_id.clone(), p);
                }
                self.anims.remove(&tile.app_id);
                continue;
            }

            if let Some(anim) = self.anims.get(&tile.app_id).cloned() {
                let t = ((now - anim.start) as f32 / anim.dur).clamp(0.0, 1.0);
                let e = match anim.curve {
                    AnimCurve::Decelerate => ease_decelerate(t),
                    AnimCurve::OutBack => ease_out_back(t),
                };
                let p = Pos2::new(
                    anim.from.x + (anim.to.x - anim.from.x) * e,
                    anim.from.y + (anim.to.y - anim.from.y) * e,
                );
                self.visual.insert(tile.app_id.clone(), p);
                if t >= 1.0 {
                    self.visual.insert(tile.app_id.clone(), anim.to);
                    self.anims.remove(&tile.app_id);
                } else {
                    busy = true;
                }
            } else {
                self.visual.entry(tile.app_id.clone()).or_insert(target);
                // If visual drifted from target without an anim, ease back.
                let cur = self.visual_min(&tile.app_id, target);
                if (cur - target).length_sq() > 0.25 {
                    self.anims.insert(
                        tile.app_id.clone(),
                        PosAnim {
                            from: cur,
                            to: target,
                            start: now,
                            dur: AVOID_DUR,
                            curve: AnimCurve::Decelerate,
                        },
                    );
                    busy = true;
                }
            }
        }
        busy
    }

    fn visual_sz(&self, app_id: &str, fallback: Vec2) -> Vec2 {
        self.visual_size.get(app_id).copied().unwrap_or(fallback)
    }

    fn retarget_size_anims(
        &mut self,
        origin: Pos2,
        unit: f32,
        gap: f32,
        now: f64,
        skip: Option<usize>,
        curve: AnimCurve,
        dur: f32,
    ) {
        let live: std::collections::HashSet<String> =
            self.tiles.iter().map(|t| t.app_id.clone()).collect();
        self.visual_size.retain(|k, _| live.contains(k));
        self.size_anims.retain(|k, _| live.contains(k));

        for (i, tile) in self.tiles.iter().enumerate() {
            if Some(i) == skip {
                continue;
            }
            let target = tile_pixel_rect(origin, tile, unit, gap).size();
            let from = self.visual_sz(&tile.app_id, target);
            if (from - target).length_sq() < 0.25 {
                self.size_anims.remove(&tile.app_id);
                self.visual_size.insert(tile.app_id.clone(), target);
                continue;
            }
            let same = self
                .size_anims
                .get(&tile.app_id)
                .is_some_and(|a| (a.to - target).length_sq() < 0.25);
            if same {
                if let Some(a) = self.size_anims.get_mut(&tile.app_id) {
                    a.to = target;
                }
                continue;
            }
            self.size_anims.insert(
                tile.app_id.clone(),
                SizeAnim {
                    from,
                    to: target,
                    start: now,
                    dur,
                    curve,
                },
            );
        }
    }

    /// Advance size anims / spring the actively resized tile. Returns true if busy.
    fn step_size_anims(&mut self, now: f64, dt: f32, origin: Pos2, unit: f32, gap: f32) -> bool {
        let mut busy = false;
        let resize_live = self
            .resize_idx
            .filter(|_| self.dragging_handle)
            .filter(|&i| i < self.tiles.len());

        for (i, tile) in self.tiles.iter().enumerate() {
            let target = tile_pixel_rect(origin, tile, unit, gap).size();
            if resize_live == Some(i) {
                // Follow integer targets with a soft spring while the handle is dragged.
                self.size_anims.remove(&tile.app_id);
                let cur = self
                    .visual_size
                    .entry(tile.app_id.clone())
                    .or_insert(target);
                let nx = spring_step(cur.x, target.x, dt, 16.0);
                let ny = spring_step(cur.y, target.y, dt, 16.0);
                if (nx - target.x).abs() > 0.15 || (ny - target.y).abs() > 0.15 {
                    busy = true;
                }
                *cur = Vec2::new(nx, ny);
                continue;
            }

            if let Some(anim) = self.size_anims.get(&tile.app_id).cloned() {
                let t = ((now - anim.start) as f32 / anim.dur).clamp(0.0, 1.0);
                let e = match anim.curve {
                    AnimCurve::Decelerate => ease_decelerate(t),
                    AnimCurve::OutBack => ease_out_back(t),
                };
                let s = Vec2::new(
                    anim.from.x + (anim.to.x - anim.from.x) * e,
                    anim.from.y + (anim.to.y - anim.from.y) * e,
                );
                self.visual_size.insert(tile.app_id.clone(), s);
                if t >= 1.0 {
                    self.visual_size.insert(tile.app_id.clone(), anim.to);
                    self.size_anims.remove(&tile.app_id);
                } else {
                    busy = true;
                }
            } else {
                self.visual_size
                    .entry(tile.app_id.clone())
                    .or_insert(target);
                let cur = self.visual_sz(&tile.app_id, target);
                if (cur - target).length_sq() > 0.25 {
                    self.size_anims.insert(
                        tile.app_id.clone(),
                        SizeAnim {
                            from: cur,
                            to: target,
                            start: now,
                            dur: RESIZE_DUR,
                            curve: AnimCurve::Decelerate,
                        },
                    );
                    busy = true;
                }
            }
        }
        busy
    }

    /// Paint the tile board.
    ///
    /// If `unpin_zone` is set (typically the app list), dropping a moved tile there removes it.
    pub fn paint(
        &mut self,
        ui: &mut Ui,
        area: Rect,
        apps: &[win10x_protocol::AppEntry],
        pal: &Palette,
        unpin_zone: Option<Rect>,
        icons: &IconCache,
        reveal: Option<StartTileReveal>,
        launching_app_id: Option<&str>,
        flow: StartTileFlow,
    ) -> TileBoardOutcome {
        let mut out = TileBoardOutcome::default();
        if area.width() < 80.0 || area.height() < 80.0 {
            return out;
        }

        let (unit, gap) = cell_metrics(area);
        let grid_origin = area.min + Vec2::new(0.0, TILE_GROUP_HEADER_GUTTER);
        self.rebase_paint_origin(grid_origin);
        let stride = unit + gap;
        let max_cols = ((area.width() + gap) / stride).floor().max(1.0) as i32;
        let max_rows = ((area.height() + gap) / stride).floor().max(1.0) as i32;
        if self.group_repack_pending {
            let primary_limit = match flow {
                StartTileFlow::Vertical => max_cols,
                StartTileFlow::Horizontal => max_rows,
            };
            self.tiles = pack_tiles_grouped(&self.tiles, &self.groups, flow, primary_limit);
            self.group_repack_pending = false;
            self.visual.clear();
            self.anims.clear();
            self.visual_size.clear();
            self.size_anims.clear();
            self.dirty = true;
        }

        if self.tiles.is_empty() {
            ui.painter().text(
                area.center(),
                Align2::CENTER_CENTER,
                "拖拽左侧应用固定 · 拖回列表可移除",
                FontId::proportional(14.0),
                pal.text_secondary,
            );
        }

        let bg = ui.interact(area, ui.id().with("tile_board_bg"), Sense::click());

        let pointer = ui.ctx().pointer_interact_pos();
        let primary_down = ui.input(|i| i.pointer.button_down(PointerButton::Primary));
        let primary_released = ui.input(|i| i.pointer.button_released(PointerButton::Primary));
        let primary_clicked = ui.input(|i| i.pointer.primary_clicked());
        let now = ui.input(|i| i.time);

        let mut clicked_tile: Option<(usize, Rect)> = None;
        let mut clicked_handle = false;
        let mut start_resize: Option<usize> = None;
        let mut unpin: Option<String> = None;
        let mut move_to_group: Option<(usize, u32)> = None;
        let mut move_to_new_group: Option<usize> = None;
        let mut drag_started_on: Option<(usize, Pos2)> = None;
        let group_choices: Vec<_> = self
            .groups
            .iter()
            .map(|group| (group.id, group.title.clone()))
            .collect();

        // —— Resize handle drag (must run even with list-style move) ——
        if let Some(idx) = self.resize_idx {
            if idx < self.tiles.len() {
                if self.dragging_handle {
                    if let Some(pos) = pointer {
                        if let Some((nw, nh)) =
                            size_from_pointer(&self.tiles[idx], pos, grid_origin, unit, gap)
                        {
                            let cur = &self.tiles[idx];
                            if cur.w != nw || cur.h != nh {
                                self.tiles[idx].w = nw;
                                self.tiles[idx].h = nh;
                                resolve_avoid(&mut self.tiles, idx, max_cols, max_rows);
                                self.retarget_anims(
                                    grid_origin,
                                    unit,
                                    gap,
                                    now,
                                    Some(idx),
                                    AnimCurve::Decelerate,
                                    AVOID_DUR,
                                );
                                self.dirty = true;
                                out.changed = true;
                            }
                        }
                    }
                    if primary_released {
                        self.dragging_handle = false;
                        let id = self.tiles[idx].app_id.clone();
                        let target =
                            tile_pixel_rect(grid_origin, &self.tiles[idx], unit, gap).size();
                        let from = self.visual_sz(&id, target);
                        self.size_anims.insert(
                            id,
                            SizeAnim {
                                from,
                                to: target,
                                start: now,
                                dur: RESIZE_DUR,
                                curve: AnimCurve::Decelerate,
                            },
                        );
                    }
                }
            } else {
                self.resize_idx = None;
                self.dragging_handle = false;
            }
        }

        // —— List-style drag: move past DRAG_ACTIVATE → active tile move ——
        if self.resize_idx.is_none() && self.move_drag.is_none() {
            if let Some(pending) = self.pending_move.clone() {
                if !primary_down {
                    self.pending_move = None;
                } else if let Some(pos) = pointer {
                    if pos.distance(pending.start_pos) > DRAG_ACTIVATE {
                        if pending.idx < self.tiles.len() {
                            let grid =
                                tile_pixel_rect(grid_origin, &self.tiles[pending.idx], unit, gap);
                            let vmin = self.visual_min(&self.tiles[pending.idx].app_id, grid.min);
                            self.anims.remove(&self.tiles[pending.idx].app_id);
                            self.visual
                                .insert(self.tiles[pending.idx].app_id.clone(), vmin);
                            self.move_drag = Some(MoveDrag {
                                idx: pending.idx,
                                grab: pos - vmin,
                                float_min: vmin,
                                layout_before_drag: self.tiles.clone(),
                            });
                            self.suppress_click = true;
                            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                        }
                        self.pending_move = None;
                    }
                }
            }
        }

        if let Some(drag) = self.move_drag.clone() {
            if drag.idx < self.tiles.len() {
                if primary_down {
                    if let Some(pos) = pointer {
                        let float_min = pos - drag.grab;
                        let (col, row) = snap_origin(
                            float_min,
                            grid_origin,
                            unit,
                            gap,
                            max_cols,
                            max_rows,
                            &self.tiles[drag.idx],
                        );
                        let preview = drag_preview_layout(
                            &drag.layout_before_drag,
                            drag.idx,
                            col,
                            row,
                            max_cols,
                            max_rows,
                        );
                        if self.tiles != preview {
                            self.tiles = preview;
                            self.retarget_anims(
                                grid_origin,
                                unit,
                                gap,
                                now,
                                Some(drag.idx),
                                AnimCurve::Decelerate,
                                AVOID_DUR,
                            );
                            self.dirty = true;
                            out.changed = true;
                        }
                        self.move_drag = Some(MoveDrag {
                            idx: drag.idx,
                            grab: drag.grab,
                            float_min,
                            layout_before_drag: drag.layout_before_drag,
                        });
                    }
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                } else {
                    // Drop onto app list → unpin; otherwise snap to grid.
                    let over_unpin = pointer.zip(unpin_zone).is_some_and(|(p, z)| z.contains(p));
                    if over_unpin {
                        let id = self.tiles[drag.idx].app_id.clone();
                        self.unpin_app(&id);
                        self.retarget_anims(
                            grid_origin,
                            unit,
                            gap,
                            now,
                            None,
                            AnimCurve::Decelerate,
                            AVOID_DUR,
                        );
                        out.changed = true;
                        self.save_if_dirty();
                    } else {
                        // Drop → nonlinear snap onto the grid slot.
                        let id = self.tiles[drag.idx].app_id.clone();
                        self.visual.insert(id.clone(), drag.float_min);
                        let target =
                            tile_pixel_rect(grid_origin, &self.tiles[drag.idx], unit, gap).min;
                        self.anims.insert(
                            id,
                            PosAnim {
                                from: drag.float_min,
                                to: target,
                                start: now,
                                dur: SNAP_DUR,
                                curve: AnimCurve::OutBack,
                            },
                        );
                        self.move_drag = None;
                        // Neighbors that still need to settle keep decelerate anims.
                        self.retarget_anims(
                            grid_origin,
                            unit,
                            gap,
                            now,
                            None,
                            AnimCurve::Decelerate,
                            AVOID_DUR,
                        );
                        // Preserve the OutBack snap we just queued for the dropped tile.
                        if let Some(a) = self.anims.get_mut(&self.tiles[drag.idx].app_id) {
                            a.from = drag.float_min;
                            a.to = target;
                            a.start = now;
                            a.dur = SNAP_DUR;
                            a.curve = AnimCurve::OutBack;
                        }
                        self.save_if_dirty();
                    }
                }
            } else {
                self.move_drag = None;
            }
        }

        let dt = ui.input(|i| i.stable_dt).min(0.05);
        let animating = self.step_anims(now, grid_origin, unit, gap);
        let sizing = self.step_size_anims(now, dt, grid_origin, unit, gap);
        if animating
            || sizing
            || self.move_drag.is_some()
            || self.pending_move.is_some()
            || self.dragging_handle
        {
            ui.ctx().request_repaint();
        }

        // Highlight app list while dragging a tile over it (unpin affordance).
        if let (Some(drag), Some(zone), Some(pos)) = (self.move_drag.as_ref(), unpin_zone, pointer)
        {
            if zone.contains(pos) && drag.idx < self.tiles.len() {
                ui.painter().rect(
                    zone,
                    CornerRadius::same(0),
                    Color32::from_rgba_unmultiplied(
                        pal.accent.r(),
                        pal.accent.g(),
                        pal.accent.b(),
                        28,
                    ),
                    Stroke::new(2.0, pal.accent),
                    StrokeKind::Inside,
                );
                ui.painter().text(
                    zone.center(),
                    Align2::CENTER_CENTER,
                    "拖放到此处以移除",
                    FontId::proportional(14.0),
                    pal.accent,
                );
            }
        }

        let group_layouts = self.tile_group_layouts(grid_origin, unit, gap);
        for group in &group_layouts {
            let progress = reveal.map_or(1.0, |frame| {
                if frame.style == StartTileAnimationStyle::FlutterMetroGroup {
                    frame.flutter_metro_group_progress(group.index)
                } else {
                    1.0
                }
            });
            if progress <= 0.001 {
                continue;
            }
            let title_base = Rect::from_min_size(
                Pos2::new(
                    group.bounds.min.x,
                    group.bounds.min.y - TILE_GROUP_HEADER_GUTTER + 4.0,
                ),
                Vec2::new(group.bounds.width().max(unit * 2.0), 24.0),
            );
            let title_rect = if reveal
                .is_some_and(|frame| frame.style == StartTileAnimationStyle::FlutterMetroGroup)
            {
                flutter_metro_group_rect(title_base, group.bounds.center(), progress)
            } else {
                title_base
            };
            self.paint_editable_group_title(
                ui,
                group.id,
                &group.title,
                title_rect,
                FontId::proportional(15.0 * (0.8 + 0.2 * progress)),
                color_with_opacity(pal.text_secondary, progress),
                "start_tile_group_title",
                reveal.is_none(),
            );
        }

        // Draw order: others first, then resizing/moving tile on top.
        let mut order: Vec<usize> = (0..self.tiles.len()).collect();
        order.sort_by_key(|&i| (self.tiles[i].row, self.tiles[i].col));
        let top = self.move_drag.as_ref().map(|d| d.idx).or(self.resize_idx);
        if let Some(ti) = top {
            order.retain(|&i| i != ti);
            order.push(ti);
        }

        // Android layout animations use normal layout order; Disco reverses it.
        let mut layout_rank = vec![None; self.tiles.len()];
        let mut layout_sequence: Vec<usize> = (0..self.tiles.len())
            .filter(|&i| {
                apps.iter().any(|a| a.id == self.tiles[i].app_id)
                    && area.intersects(tile_pixel_rect(grid_origin, &self.tiles[i], unit, gap))
            })
            .collect();
        layout_sequence.sort_by_key(|&i| (self.tiles[i].row, self.tiles[i].col));
        for (rank, &i) in layout_sequence.iter().enumerate() {
            layout_rank[i] = Some(rank);
        }
        let reveal_count = layout_sequence.len();
        let (reveal_min_row, reveal_max_row) = self.animation_row_bounds();

        for i in order {
            let tile = self.tiles[i].clone();
            let Some(app) = apps.iter().find(|a| a.id == tile.app_id) else {
                continue;
            };

            let moving = self.move_drag.as_ref().is_some_and(|d| d.idx == i);
            let resizing = self.resize_idx == Some(i);
            let base_rect = tile_pixel_rect(grid_origin, &tile, unit, gap);
            let vmin = self.visual_min(&tile.app_id, base_rect.min);
            let vsize = self.visual_sz(&tile.app_id, base_rect.size());
            let full_rect = Rect::from_min_size(vmin, vsize);

            let normal_rank = layout_rank[i].unwrap_or(0);
            let reverse_rank = reveal_count.saturating_sub(normal_rank + 1);
            let selected_for_launch = launching_app_id.is_some_and(|id| id == tile.app_id);
            let reveal_frame = reveal.map(|r| {
                r.tile_frame_in_group(
                    &tile,
                    self.group_index(tile.group_id),
                    normal_rank,
                    reverse_rank,
                    reveal_count,
                    reveal_min_row,
                    reveal_max_row,
                    selected_for_launch,
                )
            });
            if reveal_frame.is_some_and(|frame| !frame.visible) {
                continue;
            }

            if !area.intersects(full_rect) && !moving {
                continue;
            }

            let revealing = reveal.is_some();
            let sense = if revealing {
                Sense::hover()
            } else if resizing {
                Sense::click()
            } else {
                Sense::click_and_drag()
            };
            let resp = ui.interact(
                full_rect,
                ui.id().with(("start_tile", i, &tile.app_id)),
                sense,
            );

            let fill = tile_fill_for(
                &app.id,
                &app.name,
                pal,
                (!revealing && resp.hovered()) || resizing || moving,
            );
            let fg = tile_fg_for(fill);
            let stroke = Stroke::new(
                if resizing || moving { 2.0 } else { 1.0 },
                if resizing || moving {
                    pal.accent
                } else {
                    Color32::from_rgba_unmultiplied(0, 0, 0, 22)
                },
            );

            let rect = full_rect;
            let handle = resize_handle_rect(rect);

            if let Some(frame) = reveal_frame {
                match frame.kind {
                    TileAnimationKind::Disco(disco) => paint_disco_turnstile_tile(
                        ui,
                        icons,
                        app,
                        rect,
                        fill,
                        fg,
                        stroke,
                        disco,
                        ui.max_rect().width() * 0.25,
                    ),
                    TileAnimationKind::MobileStack {
                        progress,
                        launching_app,
                    } => paint_mobile_stack_tile(
                        ui,
                        icons,
                        app,
                        rect,
                        fill,
                        fg,
                        stroke,
                        progress,
                        Pos2::new(area.center().x, base_rect.min.y),
                        stride,
                        launching_app,
                    ),
                    TileAnimationKind::Flat {
                        progress,
                        scale_from,
                        opacity_from,
                        offset_y,
                    } => paint_flat_reveal_tile(
                        ui,
                        icons,
                        app,
                        rect,
                        fill,
                        fg,
                        stroke,
                        progress,
                        scale_from,
                        opacity_from,
                        offset_y,
                    ),
                    TileAnimationKind::Windows8Recede { progress } => paint_windows8_receding_tile(
                        ui,
                        icons,
                        app,
                        rect,
                        fill,
                        fg,
                        stroke,
                        progress,
                        area.center(),
                    ),
                    TileAnimationKind::FlutterMetroGroup { progress } => {
                        let group_origin = group_layouts
                            .iter()
                            .find(|group| group.id == tile.group_id)
                            .map_or(area.center(), |group| group.bounds.center());
                        paint_flutter_metro_group_tile(
                            ui,
                            icons,
                            app,
                            rect,
                            fill,
                            fg,
                            stroke,
                            progress,
                            group_origin,
                        )
                    }
                }
                continue;
            }

            if moving {
                ui.painter().rect_filled(
                    rect.translate(Vec2::new(3.0, 4.0)),
                    CornerRadius::same(0),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 50),
                );
            }

            ui.painter().rect(
                rect,
                CornerRadius::same(0),
                fill,
                stroke,
                StrokeKind::Inside,
            );

            let icon_px = (rect.height().min(rect.width()) * 0.38).clamp(14.0, 52.0);
            let label_px = 12.0_f32.min(rect.height() * 0.14).max(10.0);
            paint_app_icon(
                ui,
                icons,
                &app,
                Pos2::new(
                    rect.center().x,
                    rect.min.y + rect.height() * TILE_ICON_CENTER_Y,
                ),
                icon_px,
                fg,
            );
            let max_chars = ((rect.width() - 16.0) / (label_px * 0.55)).floor() as usize;
            ui.painter().text(
                Pos2::new(rect.min.x + 10.0, rect.max.y - 10.0),
                Align2::LEFT_BOTTOM,
                truncate_label(&app.name, max_chars.clamp(4, 28)),
                FontId::proportional(label_px),
                fg,
            );

            if moving {
                paint_drag_icon(ui, rect, fg, pal);
            }

            if resizing {
                paint_size_guides_inside(ui, rect, tile.w, tile.h, fg, pal);
                ui.painter().rect_filled(
                    handle,
                    CornerRadius::same(2),
                    Color32::from_rgba_unmultiplied(255, 255, 255, 230),
                );
                ui.painter().rect_stroke(
                    handle,
                    CornerRadius::same(2),
                    Stroke::new(1.0, pal.accent),
                    StrokeKind::Inside,
                );
                ui.painter().text(
                    handle.center(),
                    Align2::CENTER_CENTER,
                    "⋰",
                    FontId::proportional(14.0),
                    pal.accent,
                );

                let handle_resp =
                    ui.interact(handle, ui.id().with(("tile_handle", i)), Sense::drag());
                if handle_resp.drag_started() || handle_resp.dragged() {
                    self.dragging_handle = true;
                    clicked_handle = true;
                    self.pending_move = None;
                }
                if handle_resp.drag_stopped() && self.dragging_handle {
                    clicked_handle = true;
                }
                if handle_resp.hovered() || self.dragging_handle {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
                }
            }

            resp.context_menu(|ui| {
                if ui.button("更改大小").clicked() {
                    start_resize = Some(i);
                    ui.close();
                }
                ui.menu_button("移动到组", |ui| {
                    for (group_id, title) in &group_choices {
                        if *group_id != tile.group_id && ui.button(title).clicked() {
                            move_to_group = Some((i, *group_id));
                            ui.close();
                        }
                    }
                    ui.separator();
                    if ui.button("新建组").clicked() {
                        move_to_new_group = Some(i);
                        ui.close();
                    }
                });
                if ui.button("取消固定").clicked() {
                    unpin = Some(tile.app_id.clone());
                    ui.close();
                }
            });

            // List-style: drag_started → pending; small travel activates move.
            if self.resize_idx.is_none()
                && self.move_drag.is_none()
                && !resizing
                && resp.drag_started()
            {
                if let Some(pos) = resp.interact_pointer_pos() {
                    if !(resizing && handle.contains(pos)) {
                        drag_started_on = Some((i, pos));
                    }
                }
            }

            if resp.clicked() {
                clicked_tile = Some((i, rect));
            }
        }

        if let Some((i, pos)) = drag_started_on {
            self.pending_move = Some(PendingMove {
                idx: i,
                start_pos: pos,
            });
        }

        if let Some(i) = start_resize {
            self.clear_interaction();
            self.resize_idx = Some(i);
        }
        if let Some((index, group_id)) = move_to_group {
            self.move_tile_to_group(index, group_id);
            out.changed = true;
        }
        if let Some(index) = move_to_new_group {
            self.move_tile_to_new_group(index);
            out.changed = true;
        }
        if let Some(id) = unpin {
            self.confirm_resize();
            self.unpin_app(&id);
            self.prune_empty_groups();
            out.changed = true;
        }

        // Confirm resize: blank / non-handle click
        if self.resize_idx.is_some()
            && !self.dragging_handle
            && self.move_drag.is_none()
            && primary_clicked
        {
            let on_handle = pointer
                .zip(self.resize_idx)
                .and_then(|(pos, idx)| {
                    self.tiles.get(idx).map(|t| {
                        let r = tile_pixel_rect(grid_origin, t, unit, gap);
                        resize_handle_rect(r).contains(pos)
                    })
                })
                .unwrap_or(false);
            if !on_handle && !clicked_handle && (bg.clicked() || clicked_tile.is_some()) {
                self.confirm_resize();
                out.changed = true;
                return out;
            }
        }

        // Launch only when idle
        if self.resize_idx.is_none() && self.move_drag.is_none() && self.pending_move.is_none() {
            if self.suppress_click {
                if primary_released || primary_clicked {
                    self.suppress_click = false;
                }
            } else if let Some((i, rect)) = clicked_tile {
                if let Some(t) = self.tiles.get(i) {
                    out.launch = Some(t.app_id.clone());
                    out.launch_origin = Some(rect);
                }
            }
        }

        let _ = (max_rows, flow);
        out
    }

    pub fn cols_for_area(area: Rect) -> i32 {
        let (unit, gap) = cell_metrics(area);
        let stride = unit + gap;
        ((area.width() + gap) / stride).floor().max(8.0) as i32
    }

    pub fn rows_for_area(area: Rect) -> i32 {
        let (unit, gap) = cell_metrics(area);
        let stride = unit + gap;
        ((area.height() + gap) / stride).floor().max(4.0) as i32
    }

    /// True while tiles are animating / being dragged (caller should repaint).
    pub fn wants_repaint(&self) -> bool {
        self.move_drag.is_some()
            || self.pending_move.is_some()
            || self.dragging_handle
            || !self.anims.is_empty()
            || !self.size_anims.is_empty()
    }

    fn animation_row_bounds(&self) -> (i32, i32) {
        let min = self.tiles.iter().map(|tile| tile.row).min().unwrap_or(0);
        let max = self.tiles.iter().map(|tile| tile.row).max().unwrap_or(min);
        (min.max(0), max.max(min).max(0))
    }

    /// Number of grid rows crossed by the Start entrance/exit wave.
    pub fn animation_row_span(&self) -> i32 {
        let (min, max) = self.animation_row_bounds();
        max - min
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// Delay for the selected W10M launch tile: follow its row in the normal
    /// top-to-bottom exit wave, then hold it for a further 80 ms.
    pub fn mobile_selected_launch_delay(&self, app_id: &str) -> Option<f32> {
        let tile = self.tiles.iter().find(|tile| tile.app_id == app_id)?;
        let (min_row, max_row) = self.animation_row_bounds();
        let row_span = (max_row - min_row).max(0);
        let row_delay = if row_span == 0 {
            0.0
        } else {
            (tile.row - min_row).max(0) as f32 / row_span as f32
                * StartTileReveal::MOBILE_STAGGER_SPAN
        };
        Some(row_delay + StartTileReveal::MOBILE_SELECTED_DELAY)
    }
}

/// One Start open/close animation with a user-selectable tile style.
#[derive(Debug, Clone, Copy)]
pub struct StartTileReveal {
    pub elapsed: f32,
    pub closing: bool,
    pub style: StartTileAnimationStyle,
    /// The close is handing off to an application, rather than only hiding Start.
    pub launching_app: bool,
    /// The application was launched from a Start tile, so the selected tile can
    /// begin 80 ms after the rest of its row.
    pub launching_tile: bool,
    /// Absolute delay, including the row-wave delay, before the selected tile lifts.
    pub selected_tile_delay: f32,
}

impl StartTileReveal {
    const MOBILE_STAGGER_SPAN: f32 = 0.16;
    const MOBILE_DURATION: f32 = 0.24;
    const MOBILE_TOTAL_DURATION: f32 = Self::MOBILE_STAGGER_SPAN + Self::MOBILE_DURATION;
    const MOBILE_SELECTED_DELAY: f32 = 0.08;
    /// After all Start content has left, retain only the Start background while
    /// the concurrently spawned application finishes mapping its window.
    const MOBILE_APP_BACKGROUND_HOLD: f32 = 0.20;
    /// The reference frames show a restrained 9% shrink, half the old 18% amount.
    const MOBILE_SCALE_FROM: f32 = 0.91;
    /// Each joined row begins slightly above its slot and settles downward into it.
    const MOBILE_ROW_TRAVEL: f32 = 0.18;
    const DISCO_STAGGER_SPAN: f32 = 0.20;
    const DISCO_OUTER_IN_DURATION: f32 = 0.50;
    const DISCO_INNER_IN_DURATION: f32 = 0.35;
    const DISCO_OUT_DURATION: f32 = 0.175;
    const DISCO_SELECTED_DELAY: f32 = 0.20;
    const DISCO_SELECTED_OUT_DURATION: f32 = 0.30;
    const DISCO_APP_LIST_DURATION: f32 = 0.20;
    const DISCO_SELECTED_APP_LIST_DELAY: f32 = 0.30;
    const METRO_ROW_STAGGER: f32 = 0.040;
    const METRO_DURATION: f32 = 0.350;
    const FACTOR_ITEM_STAGGER: f32 = 0.030;
    const FACTOR_DURATION: f32 = 0.200;
    const FACTOR_CLIP_REVEAL_DURATION: f32 = 0.300;
    /// flutter_metro_ui runs a 3000 ms controller and gives the first tile
    /// group the 0.00..0.35 interval: 1050 ms of visible group motion.
    const FLUTTER_METRO_GROUP_DURATION: f32 = 1.050;
    /// Windows 8 completes the Start retreat early, while the selected tile
    /// continues through its airborne turn and full-output expansion.
    const WINDOWS8_RETREAT_DURATION: f32 = 0.240;
    const WINDOWS8_LAUNCH_DURATION: f32 = 0.720;

    /// flutter_metro_ui uses a 3000 ms controller. Group `n` owns the
    /// `[n×0.04, n×0.04+0.35]` interval with `easeOutExpo`.
    pub fn flutter_metro_group_progress(&self, group_index: usize) -> f32 {
        let start = (group_index as f32 * 0.04).clamp(0.0, 0.7) * 3.0;
        let duration = 0.35 * 3.0;
        let raw = ((self.elapsed - start) / duration).clamp(0.0, 1.0);
        if self.closing {
            1.0 - ease_in_expo(raw)
        } else if self.elapsed < start {
            0.0
        } else {
            ease_out_expo(raw)
        }
    }

    /// The title and user bar use the upstream 0.2..0.5 controller interval
    /// (600..1500 ms) and Flutter's exact `easeOutCubic` Bézier.
    pub fn flutter_metro_header_progress(&self) -> f32 {
        if self.closing {
            1.0 - flutter_ease_out_cubic((self.elapsed / 0.9).clamp(0.0, 1.0))
        } else if self.elapsed < 0.6 {
            0.0
        } else {
            flutter_ease_out_cubic(((self.elapsed - 0.6) / 0.9).clamp(0.0, 1.0))
        }
    }

    pub fn flutter_metro_start_duration(group_count: usize) -> f32 {
        let last_group_end = if group_count == 0 {
            0.0
        } else {
            ((group_count.saturating_sub(1) as f32 * 0.04).clamp(0.0, 0.7) + 0.35) * 3.0
        };
        last_group_end.max(1.5)
    }

    pub fn total_duration(&self, row_span: i32, count: usize) -> f32 {
        match self.style {
            StartTileAnimationStyle::Windows10Mobile => {
                let content = self.mobile_content_duration(row_span);
                if self.closing && self.launching_app {
                    content + Self::MOBILE_APP_BACKGROUND_HOLD
                } else {
                    content
                }
            }
            StartTileAnimationStyle::DiscoTurnstile => {
                if self.closing && self.launching_app {
                    Self::DISCO_SELECTED_DELAY + Self::DISCO_SELECTED_OUT_DURATION
                } else {
                    Self::DISCO_STAGGER_SPAN
                        + if self.closing {
                            Self::DISCO_OUT_DURATION
                        } else {
                            Self::DISCO_OUTER_IN_DURATION
                        }
                }
            }
            StartTileAnimationStyle::MetroRise => {
                Self::METRO_ROW_STAGGER * row_span.max(0) as f32 + Self::METRO_DURATION
            }
            StartTileAnimationStyle::FactorSlideScale => {
                if self.closing && self.launching_app {
                    Self::FACTOR_CLIP_REVEAL_DURATION
                } else {
                    Self::FACTOR_ITEM_STAGGER * count.saturating_sub(1) as f32
                        + Self::FACTOR_DURATION
                }
            }
            StartTileAnimationStyle::FlutterMetroGroup => Self::FLUTTER_METRO_GROUP_DURATION,
            StartTileAnimationStyle::Windows8Launch => Self::WINDOWS8_LAUNCH_DURATION,
        }
    }

    fn disco_delay(rank: usize, count: usize) -> f32 {
        if count <= 1 {
            0.0
        } else {
            rank as f32 / (count - 1) as f32 * Self::DISCO_STAGGER_SPAN
        }
    }

    fn tile_frame(
        &self,
        tile: &StartTile,
        normal_rank: usize,
        reverse_rank: usize,
        count: usize,
        min_row: i32,
        max_row: i32,
        selected_for_launch: bool,
    ) -> TileAnimationFrame {
        self.tile_frame_in_group(
            tile,
            0,
            normal_rank,
            reverse_rank,
            count,
            min_row,
            max_row,
            selected_for_launch,
        )
    }

    fn tile_frame_in_group(
        &self,
        tile: &StartTile,
        group_index: usize,
        normal_rank: usize,
        reverse_rank: usize,
        count: usize,
        min_row: i32,
        max_row: i32,
        selected_for_launch: bool,
    ) -> TileAnimationFrame {
        match self.style {
            StartTileAnimationStyle::Windows10Mobile => {
                let normalized_row = (tile.row - min_row).max(0);
                let order = if self.closing {
                    normalized_row
                } else {
                    (max_row - tile.row).max(0)
                };
                let row_span = (max_row - min_row).max(0);
                let row_delay = if row_span == 0 {
                    0.0
                } else {
                    order as f32 / row_span as f32 * Self::MOBILE_STAGGER_SPAN
                };
                let delay = if self.closing && self.launching_tile && selected_for_launch {
                    self.selected_tile_delay
                } else {
                    row_delay
                };
                let local = self.elapsed - delay;
                let (visible, progress) = self.animation_progress(local, Self::MOBILE_DURATION);
                TileAnimationFrame {
                    visible,
                    kind: TileAnimationKind::MobileStack {
                        progress,
                        launching_app: self.closing && self.launching_app,
                    },
                }
            }
            StartTileAnimationStyle::DiscoTurnstile => {
                if self.closing {
                    let (delay, duration) = if self.launching_app && selected_for_launch {
                        (
                            Self::DISCO_SELECTED_DELAY,
                            Self::DISCO_SELECTED_OUT_DURATION,
                        )
                    } else {
                        (
                            Self::disco_delay(reverse_rank, count),
                            Self::DISCO_OUT_DURATION,
                        )
                    };
                    let raw = ((self.elapsed - delay) / duration).clamp(0.0, 1.0);
                    let outer = cubic_bezier_ease(raw, 0.75, 0.0, 1.0, 0.0);
                    TileAnimationFrame {
                        visible: raw < 1.0,
                        kind: TileAnimationKind::Disco(DiscoTileFrame {
                            outer,
                            inner: 1.0,
                            closing: true,
                        }),
                    }
                } else if self.elapsed - Self::disco_delay(reverse_rank, count) < 0.0 {
                    TileAnimationFrame {
                        visible: false,
                        kind: TileAnimationKind::Disco(DiscoTileFrame {
                            outer: 0.0,
                            inner: 0.0,
                            closing: false,
                        }),
                    }
                } else {
                    let local = self.elapsed - Self::disco_delay(reverse_rank, count);
                    let outer_raw = (local / Self::DISCO_OUTER_IN_DURATION).clamp(0.0, 1.0);
                    let inner_raw = (local / Self::DISCO_INNER_IN_DURATION).clamp(0.0, 1.0);
                    TileAnimationFrame {
                        visible: true,
                        kind: TileAnimationKind::Disco(DiscoTileFrame {
                            outer: cubic_bezier_ease(outer_raw, 0.3, 1.0, 0.2, 1.0),
                            inner: cubic_bezier_ease(inner_raw, 0.2, 0.25, 0.25, 1.0),
                            closing: false,
                        }),
                    }
                }
            }
            StartTileAnimationStyle::MetroRise => self.flat_frame(
                self.elapsed - (tile.row - min_row).max(0) as f32 * Self::METRO_ROW_STAGGER,
                Self::METRO_DURATION,
                1.0,
                0.0,
                120.0,
            ),
            StartTileAnimationStyle::FactorSlideScale if self.closing && self.launching_app => {
                TileAnimationFrame {
                    visible: true,
                    kind: TileAnimationKind::Flat {
                        progress: 1.0,
                        scale_from: 1.0,
                        opacity_from: 1.0,
                        offset_y: 0.0,
                    },
                }
            }
            StartTileAnimationStyle::FactorSlideScale => self.flat_frame(
                self.elapsed - normal_rank as f32 * Self::FACTOR_ITEM_STAGGER,
                Self::FACTOR_DURATION,
                0.90,
                0.0,
                0.20,
            ),
            StartTileAnimationStyle::FlutterMetroGroup => {
                let progress = self.flutter_metro_group_progress(group_index);
                TileAnimationFrame {
                    visible: progress > 0.001,
                    kind: TileAnimationKind::FlutterMetroGroup { progress },
                }
            }
            StartTileAnimationStyle::Windows8Launch if self.closing && self.launching_app => {
                let retreat = ease_decelerate(
                    (self.elapsed / Self::WINDOWS8_RETREAT_DURATION).clamp(0.0, 1.0),
                );
                TileAnimationFrame {
                    // The selected tile is repainted last as a full-scene flying
                    // proxy, exactly over its settled source rectangle at t=0.
                    visible: !selected_for_launch && retreat < 0.999,
                    kind: TileAnimationKind::Windows8Recede { progress: retreat },
                }
            }
            StartTileAnimationStyle::Windows8Launch => {
                self.flat_frame(self.elapsed, Self::WINDOWS8_LAUNCH_DURATION, 1.0, 1.0, 0.0)
            }
        }
    }

    /// Progress for Factor Launcher's source-rectangle clip reveal.
    pub fn factor_clip_progress(&self) -> Option<f32> {
        (self.style == StartTileAnimationStyle::FactorSlideScale
            && self.closing
            && self.launching_app)
            .then(|| {
                ease_decelerate((self.elapsed / Self::FACTOR_CLIP_REVEAL_DURATION).clamp(0.0, 1.0))
            })
    }

    /// Phase data for the Windows 8 application handoff. Geometry is kept in
    /// the painter because it depends on the clicked tile and current output.
    pub fn windows8_launch_frame(&self) -> Option<Windows8LaunchFrame> {
        (self.style == StartTileAnimationStyle::Windows8Launch
            && self.closing
            && self.launching_app)
            .then(|| Windows8LaunchFrame {
                progress: (self.elapsed / Self::WINDOWS8_LAUNCH_DURATION).clamp(0.0, 1.0),
                retreat: ease_decelerate(
                    (self.elapsed / Self::WINDOWS8_RETREAT_DURATION).clamp(0.0, 1.0),
                ),
            })
    }

    fn mobile_content_duration(&self, row_span: i32) -> f32 {
        let ordinary = if row_span > 0 {
            Self::MOBILE_TOTAL_DURATION
        } else {
            Self::MOBILE_DURATION
        };
        if self.closing && self.launching_tile {
            ordinary.max(self.selected_tile_delay + Self::MOBILE_DURATION)
        } else {
            ordinary
        }
    }

    /// Opacity shared by the application list, rail, and search while W10M
    /// tiles lift away. Once this reaches zero, only the Start background remains.
    pub fn launcher_content_opacity(&self, row_span: i32) -> f32 {
        if !self.closing || !self.launching_app {
            return 1.0;
        }
        match self.style {
            StartTileAnimationStyle::Windows10Mobile => {
                let duration = self.mobile_content_duration(row_span).max(f32::EPSILON);
                1.0 - ease_accelerate((self.elapsed / duration).clamp(0.0, 1.0))
            }
            StartTileAnimationStyle::Windows8Launch => self
                .windows8_launch_frame()
                .map_or(1.0, |frame| 1.0 - frame.retreat),
            _ => 1.0,
        }
    }

    /// Disco has a separate exit for launching from its application list.
    pub fn disco_app_list_frame(
        &self,
        normal_rank: usize,
        count: usize,
        selected: bool,
    ) -> Option<DiscoAppListFrame> {
        if self.style != StartTileAnimationStyle::DiscoTurnstile
            || !self.closing
            || !self.launching_app
        {
            return None;
        }
        let reverse_rank = count.saturating_sub(normal_rank + 1);
        let delay = if selected {
            Self::DISCO_SELECTED_APP_LIST_DELAY
        } else {
            Self::disco_delay(reverse_rank, count)
        };
        let raw = ((self.elapsed - delay) / Self::DISCO_APP_LIST_DURATION).clamp(0.0, 1.0);
        Some(DiscoAppListFrame {
            visible: raw < 1.0,
            progress: cubic_bezier_ease(raw, 0.75, 0.0, 1.0, 0.0),
        })
    }

    fn animation_progress(&self, local: f32, duration: f32) -> (bool, f32) {
        let progress = if self.closing {
            1.0 - ease_accelerate((local / duration).clamp(0.0, 1.0))
        } else if local < 0.0 {
            0.0
        } else {
            ease_decelerate((local / duration).clamp(0.0, 1.0))
        };
        let visible = if self.closing {
            progress > 0.001
        } else {
            local >= 0.0
        };
        (visible, progress)
    }

    fn flat_frame(
        &self,
        local: f32,
        duration: f32,
        scale_from: f32,
        opacity_from: f32,
        offset_y: f32,
    ) -> TileAnimationFrame {
        let (visible, progress) = self.animation_progress(local, duration);
        TileAnimationFrame {
            visible,
            kind: TileAnimationKind::Flat {
                progress,
                scale_from,
                opacity_from,
                offset_y,
            },
        }
    }

    /// Whole-scene crossfade finishes with the final delayed tile.
    pub fn scene_progress(&self, row_span: i32, count: usize) -> f32 {
        if self.factor_clip_progress().is_some() {
            // The Start surface remains settled beneath the expanding target-app proxy.
            return 1.0;
        }
        if self.style == StartTileAnimationStyle::Windows10Mobile
            && self.closing
            && self.launching_app
        {
            // Keep wallpaper + white veil fully present while Start content exits,
            // then during the short background-only cold-start hold.
            return 1.0;
        }
        if self.windows8_launch_frame().is_some() {
            // Windows 8 keeps the wallpaper behind the receding Start scene and
            // then lets the selected tile cover it; do not fade back to the app.
            return 1.0;
        }
        let t = (self.elapsed / self.total_duration(row_span, count)).clamp(0.0, 1.0);
        match (self.style, self.closing) {
            (StartTileAnimationStyle::DiscoTurnstile, true) => {
                1.0 - cubic_bezier_ease(t, 0.75, 0.0, 1.0, 0.0)
            }
            (StartTileAnimationStyle::DiscoTurnstile, false) => {
                cubic_bezier_ease(t, 0.3, 1.0, 0.2, 1.0)
            }
            (_, true) => 1.0 - ease_accelerate(t),
            (_, false) => ease_decelerate(t),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiscoAppListFrame {
    pub visible: bool,
    pub progress: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Windows8LaunchFrame {
    pub progress: f32,
    pub retreat: f32,
}

#[derive(Debug, Clone, Copy)]
struct TileAnimationFrame {
    visible: bool,
    kind: TileAnimationKind,
}

#[derive(Debug, Clone, Copy)]
enum TileAnimationKind {
    /// Windows 10 Mobile row layer: all tiles in a row share one transform.
    MobileStack {
        progress: f32,
        /// An application handoff grows outward from the fully settled row instead of
        /// reversing the entrance back into its pre-scaled state.
        launching_app: bool,
    },
    Flat {
        progress: f32,
        scale_from: f32,
        opacity_from: f32,
        /// `120` is an absolute epx offset; values in `0..=1` are tile-height fractions.
        offset_y: f32,
    },
    /// Every non-selected tile shares one board-space depth transform.
    Windows8Recede {
        progress: f32,
    },
    /// flutter_metro_ui wraps each complete tile group in one translate/scale/
    /// opacity transform; members share their group's origin and stagger.
    FlutterMetroGroup {
        progress: f32,
    },
    Disco(DiscoTileFrame),
}

fn mobile_stack_rect(
    full_rect: Rect,
    row_origin: Pos2,
    stride: f32,
    progress: f32,
    launching_app: bool,
) -> Rect {
    let progress = progress.clamp(0.0, 1.0);
    let scale_delta = 1.0 - StartTileReveal::MOBILE_SCALE_FROM;
    let scale = if launching_app {
        1.0 + scale_delta * (1.0 - progress)
    } else {
        StartTileReveal::MOBILE_SCALE_FROM + scale_delta * progress
    };
    let travel = -stride * StartTileReveal::MOBILE_ROW_TRAVEL * (1.0 - progress);
    let transform =
        |point: Pos2| row_origin + (point - row_origin) * scale + Vec2::new(0.0, travel);
    Rect::from_min_max(transform(full_rect.min), transform(full_rect.max))
}

fn paint_mobile_stack_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    full_rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    progress: f32,
    row_origin: Pos2,
    stride: f32,
    launching_app: bool,
) {
    let rect = mobile_stack_rect(full_rect, row_origin, stride, progress, launching_app);
    let opacity = 0.08 + 0.92 * progress;
    paint_scaled_tile_face(ui, icons, app, rect, fill, fg, stroke, opacity);
}

fn paint_windows8_receding_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    full_rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    progress: f32,
    board_origin: Pos2,
) {
    let progress = progress.clamp(0.0, 1.0);
    let scale = 1.0 - 0.28 * progress;
    let transform = |point: Pos2| board_origin + (point - board_origin) * scale;
    let rect = Rect::from_min_max(transform(full_rect.min), transform(full_rect.max));
    // A depth fade makes the board feel as if it is moving behind the selected
    // tile, rather than having each tile independently shrink in place.
    let opacity = (1.0 - progress).powf(1.35);
    paint_scaled_tile_face(ui, icons, app, rect, fill, fg, stroke, opacity);
}

fn flutter_metro_group_rect(full_rect: Rect, board_origin: Pos2, progress: f32) -> Rect {
    let progress = progress.clamp(0.0, 1.0);
    let scale = 0.8 + 0.2 * progress;
    let translate = Vec2::new(100.0 * (1.0 - progress), 0.0);
    let transform = |point: Pos2| board_origin + (point - board_origin) * scale + translate;
    Rect::from_min_max(transform(full_rect.min), transform(full_rect.max))
}

fn color_with_opacity(color: Color32, opacity: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (color.a() as f32 * opacity.clamp(0.0, 1.0)).round() as u8,
    )
}

fn windows8_metro_press_point(
    rect: Rect,
    point: Pos2,
    scale: f32,
    rotate_x: f32,
    rotate_y: f32,
) -> Pos2 {
    let mut x = (point.x - rect.center().x) * scale;
    let y = (point.y - rect.center().y) * scale;
    let sin_x = rotate_x.sin();
    let cos_x = rotate_x.cos();
    let sin_y = rotate_y.sin();
    let cos_y = rotate_y.cos();
    let rotated_y = y * cos_x;
    let z_after_x = y * sin_x;
    let rotated_x = x * cos_y + z_after_x * sin_y;
    let rotated_z = -x * sin_y + z_after_x * cos_y;
    x = rotated_x;
    let perspective = 1.0 / (1.0 + 0.0008 * rotated_z).max(0.2);
    Pos2::new(
        rect.center().x + x * perspective,
        rect.center().y + rotated_y * perspective,
    )
}

fn windows8_metro_press_quad(
    rect: Rect,
    scale: f32,
    rotate_x: f32,
    rotate_y: f32,
) -> ProjectedQuad {
    ProjectedQuad {
        left_top: windows8_metro_press_point(rect, rect.left_top(), scale, rotate_x, rotate_y),
        right_top: windows8_metro_press_point(rect, rect.right_top(), scale, rotate_x, rotate_y),
        left_bottom: windows8_metro_press_point(
            rect,
            rect.left_bottom(),
            scale,
            rotate_x,
            rotate_y,
        ),
        right_bottom: windows8_metro_press_point(
            rect,
            rect.right_bottom(),
            scale,
            rotate_x,
            rotate_y,
        ),
    }
}

fn paint_flutter_metro_group_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    full_rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    progress: f32,
    board_origin: Pos2,
) {
    let rect = flutter_metro_group_rect(full_rect, board_origin, progress);
    paint_scaled_tile_face(ui, icons, app, rect, fill, fg, stroke, progress);
}

#[derive(Debug, Clone, Copy)]
struct DiscoTileFrame {
    outer: f32,
    inner: f32,
    closing: bool,
}

#[derive(Debug, Clone, Copy)]
struct ProjectedQuad {
    left_top: Pos2,
    right_top: Pos2,
    left_bottom: Pos2,
    right_bottom: Pos2,
}

fn paint_flat_reveal_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    full_rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    progress: f32,
    scale_from: f32,
    opacity_from: f32,
    offset_y: f32,
) {
    let scale = scale_from + (1.0 - scale_from) * progress;
    let offset_px = if offset_y.abs() <= 1.0 {
        full_rect.height() * offset_y
    } else {
        offset_y
    };
    let rect = Rect::from_center_size(full_rect.center(), full_rect.size() * scale)
        .translate(Vec2::new(0.0, offset_px * (1.0 - progress)));
    let opacity = opacity_from + (1.0 - opacity_from) * progress;
    paint_scaled_tile_face(ui, icons, app, rect, fill, fg, stroke, opacity);
}

fn paint_scaled_tile_face(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    opacity: f32,
) {
    let parent_opacity = ui.opacity();
    ui.set_opacity(parent_opacity * opacity);

    ui.painter().rect(
        rect,
        CornerRadius::same(0),
        fill,
        stroke,
        StrokeKind::Inside,
    );
    let icon_px = (rect.height().min(rect.width()) * 0.38).clamp(14.0, 52.0);
    let label_px = 12.0_f32.min(rect.height() * 0.14).max(10.0);
    paint_app_icon(
        ui,
        icons,
        app,
        Pos2::new(
            rect.center().x,
            rect.min.y + rect.height() * TILE_ICON_CENTER_Y,
        ),
        icon_px,
        fg,
    );
    let max_chars = ((rect.width() - 16.0) / (label_px * 0.55)).floor() as usize;
    ui.painter().text(
        Pos2::new(rect.min.x + 10.0, rect.max.y - 10.0),
        Align2::LEFT_BOTTOM,
        truncate_label(&app.name, max_chars.clamp(4, 28)),
        FontId::proportional(label_px),
        fg,
    );

    ui.set_opacity(parent_opacity);
}

fn paint_disco_turnstile_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    rect: Rect,
    fill: Color32,
    fg: Color32,
    stroke: Stroke,
    frame: DiscoTileFrame,
    exit_distance: f32,
) {
    let (outer_angle, translate_x, inner_angle, inner_x) = if frame.closing {
        (-40.0 * frame.outer, -exit_distance * frame.outer, 0.0, 0.0)
    } else {
        (
            DISCO_OUTER_START_ANGLE * (1.0 - frame.outer),
            0.0,
            DISCO_INNER_START_ANGLE * (1.0 - frame.inner),
            DISCO_INNER_START_X * (1.0 - frame.inner),
        )
    };

    let face = projected_quad(rect, outer_angle, 0.0, 0.0, translate_x);
    paint_projected_quad(ui, face, fill, stroke);

    let icon_size = (rect.height().min(rect.width()) * 0.38).clamp(14.0, 52.0);
    let icon_center = Pos2::new(
        rect.center().x,
        rect.min.y + rect.height() * TILE_ICON_CENTER_Y,
    );
    let icon_rect = Rect::from_center_size(icon_center, Vec2::splat(icon_size));
    let icon_quad = projected_content_quad(
        rect,
        icon_rect,
        outer_angle,
        inner_angle,
        inner_x,
        translate_x,
    );
    paint_projected_app_icon(ui, icons, app, icon_quad, icon_size, fg);

    let label_px = 12.0_f32.min(rect.height() * 0.14).max(10.0);
    let label_anchor = Pos2::new(rect.min.x + 10.0, rect.max.y - 10.0);
    let projected_anchor = project_turnstile_point(
        rect,
        label_anchor,
        outer_angle,
        inner_angle,
        inner_x,
        translate_x,
    );
    let content_scale = projected_content_scale(rect, outer_angle, inner_angle).clamp(0.30, 1.0);
    let max_chars = ((rect.width() - 16.0) / (label_px * 0.55)).floor() as usize;
    ui.painter().text(
        projected_anchor,
        Align2::LEFT_BOTTOM,
        truncate_label(&app.name, max_chars.clamp(4, 28)),
        FontId::proportional(label_px * content_scale),
        fg,
    );
}

fn projected_quad(
    rect: Rect,
    outer_angle: f32,
    inner_angle: f32,
    inner_x: f32,
    translate_x: f32,
) -> ProjectedQuad {
    projected_content_quad(rect, rect, outer_angle, inner_angle, inner_x, translate_x)
}

fn projected_content_quad(
    tile_rect: Rect,
    content_rect: Rect,
    outer_angle: f32,
    inner_angle: f32,
    inner_x: f32,
    translate_x: f32,
) -> ProjectedQuad {
    ProjectedQuad {
        left_top: project_turnstile_point(
            tile_rect,
            content_rect.left_top(),
            outer_angle,
            inner_angle,
            inner_x,
            translate_x,
        ),
        right_top: project_turnstile_point(
            tile_rect,
            content_rect.right_top(),
            outer_angle,
            inner_angle,
            inner_x,
            translate_x,
        ),
        left_bottom: project_turnstile_point(
            tile_rect,
            content_rect.left_bottom(),
            outer_angle,
            inner_angle,
            inner_x,
            translate_x,
        ),
        right_bottom: project_turnstile_point(
            tile_rect,
            content_rect.right_bottom(),
            outer_angle,
            inner_angle,
            inner_x,
            translate_x,
        ),
    }
}

fn project_turnstile_point(
    rect: Rect,
    point: Pos2,
    outer_angle: f32,
    inner_angle: f32,
    inner_x: f32,
    translate_x: f32,
) -> Pos2 {
    let outer = outer_angle.to_radians();
    let inner = inner_angle.to_radians();
    let center_x = rect.center().x;

    // Inner content rotates around its own center and slides in from the right.
    let inner_local_x = point.x - center_x;
    let content_x = center_x - rect.left() + inner_local_x * inner.cos() + inner_x;
    let content_z = -inner_local_x * inner.sin();

    // The whole tile then turns around its left edge, matching transform-origin: 0% 50%.
    let x = content_x * outer.cos() + content_z * outer.sin();
    let z = -content_x * outer.sin() + content_z * outer.cos();
    let perspective_scale = DISCO_PERSPECTIVE / (DISCO_PERSPECTIVE - z);

    Pos2::new(
        rect.left() + translate_x + x * perspective_scale,
        rect.center().y + (point.y - rect.center().y) * perspective_scale,
    )
}

fn paint_projected_quad(ui: &mut Ui, quad: ProjectedQuad, fill: Color32, stroke: Stroke) {
    let mut mesh = Mesh::default();
    mesh.colored_vertex(quad.left_top, fill);
    mesh.colored_vertex(quad.right_top, fill);
    mesh.colored_vertex(quad.left_bottom, fill);
    mesh.colored_vertex(quad.right_bottom, fill);
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(2, 1, 3);
    ui.painter().add(Shape::mesh(mesh));

    for points in [
        [quad.left_top, quad.right_top],
        [quad.right_top, quad.right_bottom],
        [quad.right_bottom, quad.left_bottom],
        [quad.left_bottom, quad.left_top],
    ] {
        ui.painter().line_segment(points, stroke);
    }
}

fn paint_projected_app_icon(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    quad: ProjectedQuad,
    fallback_size: f32,
    fallback_tint: Color32,
) {
    if let Some(texture) = icons.texture(app) {
        let mut mesh = Mesh::with_texture(texture.id());
        mesh.vertices.extend_from_slice(&[
            Vertex {
                pos: quad.left_top,
                uv: Pos2::new(0.0, 0.0),
                color: Color32::WHITE,
            },
            Vertex {
                pos: quad.right_top,
                uv: Pos2::new(1.0, 0.0),
                color: Color32::WHITE,
            },
            Vertex {
                pos: quad.left_bottom,
                uv: Pos2::new(0.0, 1.0),
                color: Color32::WHITE,
            },
            Vertex {
                pos: quad.right_bottom,
                uv: Pos2::new(1.0, 1.0),
                color: Color32::WHITE,
            },
        ]);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(2, 1, 3);
        ui.painter().add(Shape::mesh(mesh));
    } else {
        let center = Pos2::new(
            (quad.left_top.x + quad.right_top.x + quad.left_bottom.x + quad.right_bottom.x) * 0.25,
            (quad.left_top.y + quad.right_top.y + quad.left_bottom.y + quad.right_bottom.y) * 0.25,
        );
        let projected_w = quad.left_top.distance(quad.right_top);
        let projected_h = quad.left_top.distance(quad.left_bottom);
        let scale = (projected_w.min(projected_h) / fallback_size).clamp(0.25, 1.0);
        paint_app_icon(ui, icons, app, center, fallback_size * scale, fallback_tint);
    }
}

fn projected_content_scale(rect: Rect, outer_angle: f32, inner_angle: f32) -> f32 {
    let combined = (outer_angle + inner_angle).to_radians().cos().abs();
    let depth = DISCO_PERSPECTIVE
        / (DISCO_PERSPECTIVE + rect.width() * outer_angle.to_radians().sin().abs());
    (combined * depth).sqrt()
}

/// Paint the Windows 8 launch proxy after shell chrome so it can pass in front
/// of the complete Start scene. The selected tile first separates from its
/// slot, then turns edge-on in mid-air, and only its reverse face expands to
/// the complete output.
pub fn paint_windows8_launch_tile(
    ui: &mut Ui,
    icons: &IconCache,
    app: &win10x_protocol::AppEntry,
    source: Rect,
    full: Rect,
    frame: Windows8LaunchFrame,
    pal: &Palette,
) {
    let (surface, turn_degrees, front_face) =
        windows8_flying_tile_geometry(source, full, frame.progress);

    ui.painter().rect_filled(
        full,
        CornerRadius::ZERO,
        Color32::from_black_alpha((96.0 * frame.retreat) as u8),
    );

    let visible_angle = if front_face {
        turn_degrees
    } else {
        turn_degrees - 180.0
    };
    let face = projected_center_quad(surface, visible_angle);
    let fill = tile_fill_for(&app.id, &app.name, pal, false);
    let fg = tile_fg_for(fill);
    paint_projected_quad(
        ui,
        face,
        fill,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 42)),
    );

    let icon_size = (surface.width().min(surface.height()) * 0.38).clamp(20.0, 112.0);
    let icon_center = if front_face {
        Pos2::new(
            surface.center().x,
            surface.min.y + surface.height() * TILE_ICON_CENTER_Y,
        )
    } else {
        surface.center()
    };
    let icon_rect = Rect::from_center_size(icon_center, Vec2::splat(icon_size));
    let icon_quad = projected_center_content_quad(surface, icon_rect, visible_angle);
    paint_projected_app_icon(ui, icons, app, icon_quad, icon_size, fg);

    // The Windows 8 tile label belongs to the front face only. Once the tile
    // passes its edge, the reverse face becomes the target-app splash surface.
    if front_face && visible_angle.to_radians().cos().abs() > 0.08 {
        let label_px = (surface.height() * 0.11).clamp(11.0, 24.0);
        let label_anchor = project_center_turn_point(
            surface,
            Pos2::new(surface.min.x + 10.0, surface.max.y - 10.0),
            visible_angle,
        );
        ui.painter().text(
            label_anchor,
            Align2::LEFT_BOTTOM,
            truncate_label(&app.name, 28),
            FontId::proportional(label_px * visible_angle.to_radians().cos().abs().sqrt()),
            fg,
        );
    }
}

fn windows8_flying_tile_geometry(source: Rect, full: Rect, progress: f32) -> (Rect, f32, bool) {
    const LIFT_END: f32 = 0.24;
    const EDGE_END: f32 = 0.68;
    let progress = progress.clamp(0.0, 1.0);
    let lift_center = source.center() + (full.center() - source.center()) * 0.22;
    let lift_size = source.size() * 1.42;
    let edge_size = full.size() * 0.58;

    if progress <= LIFT_END {
        let local = cubic_bezier_ease(progress / LIFT_END, 0.10, 0.90, 0.20, 1.0);
        let center = lerp_pos(source.center(), lift_center, local);
        let size = lerp_vec2(source.size(), lift_size, local);
        (Rect::from_center_size(center, size), 18.0 * local, true)
    } else if progress <= EDGE_END {
        let local = cubic_bezier_ease(
            (progress - LIFT_END) / (EDGE_END - LIFT_END),
            0.16,
            0.84,
            0.24,
            1.0,
        );
        let center = lerp_pos(lift_center, full.center(), local);
        let size = lerp_vec2(lift_size, edge_size, local);
        (
            Rect::from_center_size(center, size),
            egui::lerp(18.0..=90.0, local),
            true,
        )
    } else {
        let local = ease_decelerate((progress - EDGE_END) / (1.0 - EDGE_END));
        let size = lerp_vec2(edge_size, full.size() + Vec2::splat(4.0), local);
        (
            Rect::from_center_size(full.center(), size),
            egui::lerp(90.0..=180.0, local),
            false,
        )
    }
}

fn projected_center_quad(rect: Rect, angle: f32) -> ProjectedQuad {
    projected_center_content_quad(rect, rect, angle)
}

fn projected_center_content_quad(tile_rect: Rect, content_rect: Rect, angle: f32) -> ProjectedQuad {
    ProjectedQuad {
        left_top: project_center_turn_point(tile_rect, content_rect.left_top(), angle),
        right_top: project_center_turn_point(tile_rect, content_rect.right_top(), angle),
        left_bottom: project_center_turn_point(tile_rect, content_rect.left_bottom(), angle),
        right_bottom: project_center_turn_point(tile_rect, content_rect.right_bottom(), angle),
    }
}

fn project_center_turn_point(rect: Rect, point: Pos2, angle: f32) -> Pos2 {
    let angle = angle.to_radians();
    let local_x = point.x - rect.center().x;
    let x = local_x * angle.cos();
    let z = -local_x * angle.sin();
    let perspective_scale = DISCO_PERSPECTIVE / (DISCO_PERSPECTIVE - z);
    Pos2::new(
        rect.center().x + x * perspective_scale,
        rect.center().y + (point.y - rect.center().y) * perspective_scale,
    )
}

fn lerp_pos(from: Pos2, to: Pos2, progress: f32) -> Pos2 {
    Pos2::new(
        egui::lerp(from.x..=to.x, progress),
        egui::lerp(from.y..=to.y, progress),
    )
}

fn lerp_vec2(from: Vec2, to: Vec2, progress: f32) -> Vec2 {
    Vec2::new(
        egui::lerp(from.x..=to.x, progress),
        egui::lerp(from.y..=to.y, progress),
    )
}

fn cubic_bezier_ease(x: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let sample = |t: f32, a: f32, b: f32| {
        let mt = 1.0 - t;
        3.0 * mt * mt * t * a + 3.0 * mt * t * t * b + t * t * t
    };
    let mut low = 0.0;
    let mut high = 1.0;
    for _ in 0..14 {
        let t = (low + high) * 0.5;
        if sample(t, x1, x2) < x {
            low = t;
        } else {
            high = t;
        }
    }
    sample((low + high) * 0.5, y1, y2)
}

/// Flutter's `Curves.easeOutExpo` control points.
fn ease_out_expo(t: f32) -> f32 {
    cubic_bezier_ease(t, 0.16, 1.0, 0.3, 1.0)
}

/// Flutter's `Curves.easeOutExpo`, shared by page and Start transitions.
pub fn flutter_ease_out_expo(t: f32) -> f32 {
    ease_out_expo(t)
}

/// The eight accent faces used by flutter_metro_ui's sample Start groups.
/// Application identity differs in this shell, so the source palette is
/// assigned in stable board order.
fn windows8_metro_fill(index: usize) -> Color32 {
    const PALETTE: [(u8, u8, u8); 8] = [
        (0x2e, 0x8d, 0xef),
        (0x00, 0xa9, 0x33),
        (0xda, 0x53, 0x2c),
        (0x9f, 0x00, 0xa7),
        (0xb9, 0x1d, 0x47),
        (0xe3, 0xa2, 0x1a),
        (0x60, 0x3c, 0xba),
        (0x2d, 0x89, 0xef),
    ];
    let (r, g, b) = PALETTE[index % PALETTE.len()];
    Color32::from_rgb(r, g, b)
}

/// Flutter's `Curves.easeOutCubic` is a named cubic Bézier rather than the
/// simpler polynomial often given the same name.
fn flutter_ease_out_cubic(t: f32) -> f32 {
    cubic_bezier_ease(t, 0.215, 0.61, 0.355, 1.0)
}

fn ease_in_expo(t: f32) -> f32 {
    1.0 - ease_out_expo(1.0 - t)
}

fn ease_accelerate(t: f32) -> f32 {
    t.clamp(0.0, 1.0).powf(2.6)
}

#[cfg(test)]
mod reveal_tests {
    use super::{
        drag_preview_layout, flutter_ease_out_expo, flutter_metro_group_rect, mobile_stack_rect,
        pack_tiles, pack_tiles_grouped, windows8_flying_tile_geometry, DiscoTileFrame, Pos2, Rect,
        StartTile, StartTileAnimationStyle, StartTileBoard, StartTileFlow, StartTileGroup,
        StartTileReveal, TileAnimationFrame, TileAnimationKind, Windows8MetroDrag,
        DEFAULT_TILE_GROUP_ID,
    };

    fn tile(row: i32) -> StartTile {
        StartTile {
            app_id: format!("app-{row}"),
            group_id: DEFAULT_TILE_GROUP_ID,
            col: 0,
            row,
            w: 2,
            h: 1,
        }
    }

    fn grid_tile(app_id: &str, col: i32, row: i32, w: u8, h: u8) -> StartTile {
        StartTile {
            app_id: app_id.to_string(),
            group_id: DEFAULT_TILE_GROUP_ID,
            col,
            row,
            w,
            h,
        }
    }

    #[test]
    fn drag_avoidance_uses_the_nearby_vacated_slot_and_is_reversible() {
        let original = vec![
            grid_tile("dragged", 4, 4, 2, 2),
            grid_tile("neighbor", 6, 4, 2, 2),
            grid_tile("unaffected", 0, 0, 2, 2),
        ];

        let colliding = drag_preview_layout(&original, 0, 6, 4, 12, 12);
        assert_eq!((colliding[0].col, colliding[0].row), (6, 4));
        assert_eq!((colliding[1].col, colliding[1].row), (4, 4));
        assert_eq!(colliding[2], original[2]);
        assert!(!colliding[0].overlaps(&colliding[1]));

        let moved_away = drag_preview_layout(&original, 0, 10, 10, 12, 12);
        assert_eq!(moved_away[1], original[1]);
        assert_eq!(moved_away[2], original[2]);
    }

    #[test]
    fn vertical_and_horizontal_flows_pack_against_different_fixed_axes() {
        let source: Vec<_> = (0..12)
            .map(|index| grid_tile(&format!("app-{index}"), 0, 0, 2, 2))
            .collect();
        let vertical = pack_tiles(&source, StartTileFlow::Vertical, 8);
        assert!(vertical
            .iter()
            .all(|tile| tile.col + i32::from(tile.w) <= 8));
        assert!(vertical.iter().any(|tile| tile.row >= 4));

        let horizontal = pack_tiles(&source, StartTileFlow::Horizontal, 6);
        assert!(horizontal
            .iter()
            .all(|tile| tile.row + i32::from(tile.h) <= 6));
        assert!(horizontal.iter().any(|tile| tile.col >= 4));
    }

    #[test]
    fn tile_content_extent_grows_along_the_selected_scroll_axis() {
        let viewport = Rect::from_min_size(Pos2::ZERO, eframe::egui::vec2(600.0, 300.0));
        let mut board = StartTileBoard::default();
        board.tiles = vec![grid_tile("deep", 0, 18, 2, 2)];
        let vertical = board.content_size(viewport, StartTileFlow::Vertical);
        assert_eq!(vertical.x, viewport.width());
        assert!(vertical.y > viewport.height());

        board.tiles = vec![grid_tile("far", 18, 0, 2, 2)];
        let horizontal = board.content_size(viewport, StartTileFlow::Horizontal);
        assert!(horizontal.x > viewport.width());
        assert_eq!(horizontal.y, viewport.height());
    }

    #[test]
    fn windows8_metro_layout_matches_flutter_reference_constants() {
        let mut board = StartTileBoard::default();
        board.tiles = (0..6)
            .map(|index| grid_tile(&format!("square-{index}"), 0, 0, 2, 2))
            .chain(std::iter::once(grid_tile("wide", 0, 0, 4, 2)))
            .collect();
        let layout = board.windows8_metro_layout(Pos2::ZERO, eframe::egui::vec2(500.0, 1032.0));

        assert_eq!(layout.max_rows, 6);
        assert_eq!(layout.tiles[0].rect.min, Pos2::new(100.0, 174.0));
        assert_eq!(
            layout.tiles[0].rect.size(),
            eframe::egui::vec2(120.0, 120.0)
        );
        assert_eq!(
            layout.tiles[1].rect.min.y - layout.tiles[0].rect.max.y,
            10.0
        );
        assert_eq!(layout.tiles[6].rect.min, Pos2::new(230.0, 174.0));
        assert_eq!(
            layout.tiles[6].rect.size(),
            eframe::egui::vec2(250.0, 120.0)
        );
        assert_eq!(layout.content_size.x, 640.0);
    }

    #[test]
    fn legacy_tiles_migrate_into_the_default_group() {
        let tile: StartTile =
            serde_json::from_str(r#"{"app_id":"legacy","col":0,"row":0,"w":2,"h":2}"#).unwrap();

        assert_eq!(tile.group_id, DEFAULT_TILE_GROUP_ID);
    }

    #[test]
    fn grouped_freeform_pack_keeps_groups_separated_on_each_scroll_axis() {
        let groups = vec![
            StartTileGroup {
                id: 1,
                title: "一".into(),
            },
            StartTileGroup {
                id: 2,
                title: "二".into(),
            },
        ];
        let mut first = grid_tile("first", 0, 0, 2, 2);
        first.group_id = 1;
        let mut second = grid_tile("second", 0, 0, 2, 2);
        second.group_id = 2;

        let vertical = pack_tiles_grouped(
            &[first.clone(), second.clone()],
            &groups,
            StartTileFlow::Vertical,
            8,
        );
        assert!(vertical[1].row >= vertical[0].row + i32::from(vertical[0].h) + 2);

        let horizontal =
            pack_tiles_grouped(&[first, second], &groups, StartTileFlow::Horizontal, 6);
        assert!(horizontal[1].col >= horizontal[0].col + i32::from(horizontal[0].w) + 2);
    }

    #[test]
    fn windows8_metro_groups_pack_side_by_side_and_stagger_independently() {
        let mut board = StartTileBoard::default();
        board.groups = vec![
            StartTileGroup {
                id: 1,
                title: "开始".into(),
            },
            StartTileGroup {
                id: 2,
                title: "工具".into(),
            },
        ];
        let mut first = grid_tile("first", 0, 0, 2, 2);
        first.group_id = 1;
        let mut second = grid_tile("second", 0, 0, 4, 2);
        second.group_id = 2;
        board.tiles = vec![first, second];

        let layout = board.windows8_metro_layout(Pos2::ZERO, eframe::egui::vec2(900.0, 1032.0));
        assert_eq!(layout.groups.len(), 2);
        assert_eq!(layout.groups[0].bounds.min.x, 100.0);
        assert_eq!(layout.groups[1].bounds.min.x, layout.groups[0].bounds.max.x);
        assert_eq!(layout.tiles[0].group_index, 0);
        assert_eq!(layout.tiles[1].group_index, 1);
        assert_eq!(layout.tiles[1].rect.width(), 250.0);

        let frame = reveal(StartTileAnimationStyle::FlutterMetroGroup, 0.06, false);
        assert!(frame.flutter_metro_group_progress(0) > 0.0);
        assert_eq!(frame.flutter_metro_group_progress(1), 0.0);
    }

    #[test]
    fn windows8_drag_reorders_tiles_and_can_move_between_groups() {
        let mut board = StartTileBoard::default();
        board.groups = vec![
            StartTileGroup {
                id: 1,
                title: "开始".into(),
            },
            StartTileGroup {
                id: 2,
                title: "工具".into(),
            },
        ];
        let mut first = grid_tile("first", 0, 0, 2, 2);
        first.group_id = 1;
        let mut second = grid_tile("second", 0, 0, 2, 2);
        second.group_id = 2;
        board.tiles = vec![first, second];
        let layout = board.windows8_metro_layout(Pos2::ZERO, eframe::egui::vec2(900.0, 1032.0));
        let target = layout.tiles[1].rect.center();
        board.windows8_drag = Some(Windows8MetroDrag {
            app_id: "first".into(),
            grab: eframe::egui::Vec2::ZERO,
            float_min: target,
            layout_before_drag: board.tiles.clone(),
            origin: Pos2::ZERO,
            viewport: eframe::egui::vec2(900.0, 1032.0),
        });

        // With "first" lifted out, "second" repacks to the first group slot;
        // dropping right of it appends after it inside group 2.
        assert!(board.finish_windows8_drag(Some(target), 0.0));
        assert_eq!(board.tiles[0].app_id, "second");
        assert_eq!(board.tiles[1].app_id, "first");
        assert_eq!(board.tiles[1].group_id, 2);
        assert!(board.group_repack_pending);
    }

    #[test]
    fn windows8_drag_inserts_before_the_tile_under_the_pointer() {
        let mut board = StartTileBoard::default();
        board.groups = vec![
            StartTileGroup {
                id: 1,
                title: "开始".into(),
            },
            StartTileGroup {
                id: 2,
                title: "工具".into(),
            },
        ];
        let mut first = grid_tile("first", 0, 0, 2, 2);
        first.group_id = 1;
        let mut second = grid_tile("second", 0, 0, 2, 2);
        second.group_id = 2;
        board.tiles = vec![first, second];
        // Center of the surviving sibling once "first" is lifted out: the
        // preview repacks "second" to the first Metro slot.
        let target = Pos2::new(160.0, 234.0);
        board.windows8_drag = Some(Windows8MetroDrag {
            app_id: "first".into(),
            grab: eframe::egui::Vec2::ZERO,
            float_min: target,
            layout_before_drag: board.tiles.clone(),
            origin: Pos2::ZERO,
            viewport: eframe::egui::vec2(900.0, 1032.0),
        });

        assert!(board.finish_windows8_drag(Some(target), 0.0));
        assert_eq!(board.tiles[0].app_id, "first");
        assert_eq!(board.tiles[0].group_id, 2);
        assert_eq!(board.tiles[1].app_id, "second");
    }

    #[test]
    fn group_rename_commits_a_stable_draft() {
        let mut board = StartTileBoard::default();
        board.groups = vec![StartTileGroup {
            id: 1,
            title: "开始".into(),
        }];
        board.begin_group_rename(1);
        board.group_rename.as_mut().unwrap().draft = "工作".into();

        board.finish_group_rename(true);

        assert_eq!(board.groups[0].title, "工作");
        assert!(board.dirty);
    }

    #[test]
    fn flutter_metro_group_and_header_keep_their_source_intervals() {
        let group_start = reveal(StartTileAnimationStyle::FlutterMetroGroup, 0.0, false);
        let group_end = reveal(StartTileAnimationStyle::FlutterMetroGroup, 1.05, false);
        let header_wait = reveal(StartTileAnimationStyle::FlutterMetroGroup, 0.59, false);
        let header_end = reveal(StartTileAnimationStyle::FlutterMetroGroup, 1.50, false);

        assert_eq!(group_start.flutter_metro_group_progress(0), 0.0);
        assert_eq!(group_end.flutter_metro_group_progress(0), 1.0);
        assert_eq!(header_wait.flutter_metro_header_progress(), 0.0);
        assert_eq!(header_end.flutter_metro_header_progress(), 1.0);
        assert_eq!(StartTileReveal::flutter_metro_start_duration(1), 1.5);
        assert!((StartTileReveal::flutter_metro_start_duration(5) - 1.53).abs() < 0.0001);
    }

    #[test]
    fn flutter_page_curve_is_strongly_nonlinear() {
        assert_eq!(flutter_ease_out_expo(0.0), 0.0);
        assert!(flutter_ease_out_expo(0.25) > 0.75);
        assert_eq!(flutter_ease_out_expo(1.0), 1.0);
    }

    fn reveal(style: StartTileAnimationStyle, elapsed: f32, closing: bool) -> StartTileReveal {
        StartTileReveal {
            elapsed,
            closing,
            style,
            launching_app: false,
            launching_tile: false,
            selected_tile_delay: 0.0,
        }
    }

    fn animation_progress(frame: TileAnimationFrame) -> f32 {
        match frame.kind {
            TileAnimationKind::MobileStack { progress, .. } => progress,
            TileAnimationKind::Flat { progress, .. } => progress,
            TileAnimationKind::Windows8Recede { progress } => progress,
            TileAnimationKind::FlutterMetroGroup { progress } => progress,
            TileAnimationKind::Disco(_) => panic!("expected progress animation"),
        }
    }

    fn disco_frame(frame: TileAnimationFrame) -> DiscoTileFrame {
        match frame.kind {
            TileAnimationKind::Disco(frame) => frame,
            TileAnimationKind::MobileStack { .. }
            | TileAnimationKind::Flat { .. }
            | TileAnimationKind::Windows8Recede { .. }
            | TileAnimationKind::FlutterMetroGroup { .. } => {
                panic!("expected Disco animation")
            }
        }
    }

    #[test]
    fn windows_mobile_is_the_default_style() {
        assert_eq!(
            StartTileAnimationStyle::default(),
            StartTileAnimationStyle::Windows10Mobile
        );
    }

    #[test]
    fn windows_mobile_wave_leads_from_the_bottom_row() {
        let reveal = reveal(StartTileAnimationStyle::Windows10Mobile, 0.12, false);
        let bottom = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);
        let top = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);

        assert!(animation_progress(bottom) > animation_progress(top));
        assert!(bottom.visible);
        assert!(!top.visible);
    }

    #[test]
    fn windows_mobile_row_tiles_share_one_scale_and_settle_downward() {
        let left = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(100.0, 100.0));
        let right = Rect::from_min_max(Pos2::new(106.0, 0.0), Pos2::new(206.0, 100.0));
        let row_origin = Pos2::new(103.0, 0.0);

        let left_start = mobile_stack_rect(left, row_origin, 100.0, 0.0, false);
        let right_start = mobile_stack_rect(right, row_origin, 100.0, 0.0, false);

        assert!((left_start.height() - 91.0).abs() < 0.001);
        assert!((right_start.min.x - left_start.max.x - 5.46).abs() < 0.001);
        assert!((left_start.min.y + 18.0).abs() < 0.001);
        assert!((right_start.min.y - left_start.min.y).abs() < 0.001);
        assert!(left_start.max.y < left.max.y);
        assert_eq!(mobile_stack_rect(left, row_origin, 100.0, 1.0, false), left);
    }

    #[test]
    fn windows_mobile_app_launch_starts_settled_then_grows_outward() {
        let full = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(100.0, 100.0));
        let row_origin = Pos2::new(50.0, 0.0);

        let launch_start = mobile_stack_rect(full, row_origin, 100.0, 1.0, true);
        let launch_exit = mobile_stack_rect(full, row_origin, 100.0, 0.0, true);

        assert_eq!(launch_start, full);
        assert!((launch_exit.height() - 109.0).abs() < 0.001);
        assert!((launch_exit.min.y + 18.0).abs() < 0.001);
        assert!(launch_exit.min.x < full.min.x);
        assert!(launch_exit.max.x > full.max.x);
    }

    #[test]
    fn windows_mobile_app_launch_fades_from_top_to_bottom() {
        let mut reveal = reveal(StartTileAnimationStyle::Windows10Mobile, 0.12, true);
        reveal.launching_app = true;

        let top = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);
        let bottom = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);

        assert!(animation_progress(top) < animation_progress(bottom));
        assert!(top.visible);
        assert!(bottom.visible);
        assert!(matches!(
            top.kind,
            TileAnimationKind::MobileStack {
                launching_app: true,
                ..
            }
        ));

        reveal.elapsed = 0.40;
        let final_row = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);
        assert!(!final_row.visible);
        assert_eq!(animation_progress(final_row), 0.0);
    }

    #[test]
    fn windows_mobile_selected_tile_waits_then_lifts_without_a_fullscreen_proxy() {
        let mut reveal = reveal(StartTileAnimationStyle::Windows10Mobile, 0.0, true);
        reveal.launching_app = true;
        reveal.launching_tile = true;
        reveal.selected_tile_delay = 0.12;

        let selected = reveal.tile_frame(&tile(1), 1, 3, 5, 0, 4, true);
        assert!(selected.visible);
        assert_eq!(animation_progress(selected), 1.0);

        reveal.elapsed = 0.119;
        let selected = reveal.tile_frame(&tile(1), 1, 3, 5, 0, 4, true);
        assert_eq!(animation_progress(selected), 1.0);

        reveal.elapsed = 0.24;
        let selected = reveal.tile_frame(&tile(1), 1, 3, 5, 0, 4, true);
        assert!(animation_progress(selected) > 0.0);
        assert!(animation_progress(selected) < 1.0);

        reveal.elapsed = 0.36;
        let selected = reveal.tile_frame(&tile(1), 1, 3, 5, 0, 4, true);
        assert!(!selected.visible);
        assert_eq!(animation_progress(selected), 0.0);
        assert!((reveal.total_duration(4, 5) - 0.60).abs() < f32::EPSILON);
    }

    #[test]
    fn windows_mobile_launch_fades_chrome_then_holds_only_the_start_background() {
        let mut reveal = reveal(StartTileAnimationStyle::Windows10Mobile, 0.0, true);
        reveal.launching_app = true;

        assert_eq!(reveal.launcher_content_opacity(4), 1.0);
        assert_eq!(reveal.scene_progress(4, 5), 1.0);
        reveal.elapsed = 0.20;
        assert!(reveal.launcher_content_opacity(4) > 0.0);
        assert!(reveal.launcher_content_opacity(4) < 1.0);
        reveal.elapsed = 0.40;
        assert_eq!(reveal.launcher_content_opacity(4), 0.0);
        assert_eq!(reveal.scene_progress(4, 5), 1.0);
        reveal.elapsed = 0.59;
        assert_eq!(reveal.launcher_content_opacity(4), 0.0);
        assert_eq!(reveal.scene_progress(4, 5), 1.0);
        assert!((reveal.total_duration(4, 5) - 0.60).abs() < f32::EPSILON);
    }

    #[test]
    fn windows8_launch_recedes_start_then_turns_the_selected_tile_in_mid_air() {
        let mut reveal = reveal(StartTileAnimationStyle::Windows8Launch, 0.0, true);
        reveal.launching_app = true;
        reveal.launching_tile = true;

        let selected = reveal.tile_frame(&tile(1), 1, 3, 5, 0, 4, true);
        let neighbor = reveal.tile_frame(&tile(2), 2, 2, 5, 0, 4, false);
        assert!(!selected.visible);
        assert!(neighbor.visible);
        assert_eq!(animation_progress(neighbor), 0.0);
        assert_eq!(reveal.launcher_content_opacity(4), 1.0);

        reveal.elapsed = 0.24;
        let frame = reveal.windows8_launch_frame().expect("Windows 8 frame");
        assert_eq!(frame.retreat, 1.0);
        assert_eq!(reveal.launcher_content_opacity(4), 0.0);
        assert_eq!(reveal.scene_progress(4, 5), 1.0);

        let source = Rect::from_min_max(Pos2::new(100.0, 100.0), Pos2::new(260.0, 220.0));
        let output = Rect::from_min_max(Pos2::ZERO, Pos2::new(1000.0, 700.0));
        reveal.elapsed = 0.72 * 0.24;
        let lift = reveal.windows8_launch_frame().expect("lift frame");
        let (lifted, lift_angle, lift_front) =
            windows8_flying_tile_geometry(source, output, lift.progress);
        assert!(lift_front);
        assert!(lifted.width() > source.width());
        assert!(
            lifted.center().distance(output.center()) < source.center().distance(output.center())
        );
        assert!((lift_angle - 18.0).abs() < 0.01);

        reveal.elapsed = 0.72 * 0.68;
        let edge = reveal.windows8_launch_frame().expect("edge frame");
        let (_, edge_angle, edge_front) =
            windows8_flying_tile_geometry(source, output, edge.progress);
        assert!(edge_front);
        assert!((edge_angle - 90.0).abs() < 0.01);

        reveal.elapsed = 0.72;
        let finished = reveal.windows8_launch_frame().expect("finished frame");
        let (surface, final_angle, front) =
            windows8_flying_tile_geometry(source, output, finished.progress);
        assert!(!front);
        assert!((final_angle - 180.0).abs() < 0.01);
        assert!((surface.width() - output.width() - 4.0).abs() < 0.01);
        assert!((reveal.total_duration(4, 5) - 0.72).abs() < f32::EPSILON);
    }

    #[test]
    fn windows_mobile_selected_delay_follows_its_row_then_adds_eighty_ms() {
        let mut board = StartTileBoard::default();
        board.tiles = vec![tile(0), tile(2), tile(4)];

        assert!((board.mobile_selected_launch_delay("app-0").unwrap() - 0.08).abs() < 0.001);
        assert!((board.mobile_selected_launch_delay("app-2").unwrap() - 0.16).abs() < 0.001);
        assert!((board.mobile_selected_launch_delay("app-4").unwrap() - 0.24).abs() < 0.001);
    }

    #[test]
    fn animation_row_span_ignores_absolute_grid_offset() {
        let mut board = StartTileBoard::default();
        board.tiles = vec![tile(17), tile(22)];

        assert_eq!(board.animation_row_span(), 5);
    }

    #[test]
    fn windows_mobile_multi_row_reveal_finishes_within_four_tenths() {
        let mut reveal = reveal(StartTileAnimationStyle::Windows10Mobile, 0.0, false);
        assert!((reveal.total_duration(100, 5) - 0.40).abs() < f32::EPSILON);

        reveal.elapsed = 0.40;
        let final_delayed_row = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 100, false);
        assert_eq!(animation_progress(final_delayed_row), 1.0);
        assert_eq!(reveal.scene_progress(100, 5), 1.0);
    }

    #[test]
    fn disco_reverses_layout_order() {
        let reveal = reveal(StartTileAnimationStyle::DiscoTurnstile, 0.12, false);
        let bottom = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);
        let top = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);

        assert!(disco_frame(bottom).outer > disco_frame(top).outer);
        assert!(bottom.visible);
        assert!(!top.visible);
    }

    #[test]
    fn metro_rise_uses_row_delay() {
        let reveal = reveal(StartTileAnimationStyle::MetroRise, 0.10, false);
        let top = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);
        let bottom = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);

        assert!(top.visible);
        assert!(!bottom.visible);
        assert!(animation_progress(top) > animation_progress(bottom));
    }

    #[test]
    fn factor_uses_normal_item_order() {
        let reveal = reveal(StartTileAnimationStyle::FactorSlideScale, 0.05, false);
        let first = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);
        let last = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);

        assert!(first.visible);
        assert!(!last.visible);
        assert!(animation_progress(first) > animation_progress(last));
    }

    #[test]
    fn flutter_metro_moves_the_board_as_one_group() {
        let reveal = reveal(StartTileAnimationStyle::FlutterMetroGroup, 0.0, false);
        let first = reveal.tile_frame(&tile(0), 0, 4, 5, 0, 4, false);
        let last = reveal.tile_frame(&tile(4), 4, 0, 5, 0, 4, false);
        assert_eq!(animation_progress(first), animation_progress(last));

        let left = Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(100.0, 100.0));
        let right = Rect::from_min_max(Pos2::new(106.0, 0.0), Pos2::new(206.0, 100.0));
        let origin = Pos2::new(103.0, 50.0);
        let left_start = flutter_metro_group_rect(left, origin, 0.0);
        let right_start = flutter_metro_group_rect(right, origin, 0.0);

        assert!((left_start.width() - 80.0).abs() < 0.001);
        assert!((right_start.min.x - left_start.max.x - 4.8).abs() < 0.001);
        assert!((left_start.min.x - 120.6).abs() < 0.001);
        assert_eq!(flutter_metro_group_rect(left, origin, 1.0), left);
    }

    #[test]
    fn disco_inner_content_settles_before_the_outer_tile() {
        let reveal = reveal(StartTileAnimationStyle::DiscoTurnstile, 0.40, false);
        let frame = disco_frame(reveal.tile_frame(&tile(0), 0, 0, 1, 0, 0, false));

        assert_eq!(frame.inner, 1.0);
        assert!(frame.outer < 1.0);
    }

    #[test]
    fn every_style_synchronizes_the_scene_with_its_final_tile() {
        for style in StartTileAnimationStyle::ALL {
            let mut reveal = reveal(style, 0.0, false);
            reveal.elapsed = reveal.total_duration(4, 5);
            assert_eq!(reveal.scene_progress(4, 5), 1.0, "{style:?}");
        }
    }

    #[test]
    fn disco_selected_launch_tile_uses_its_longer_delayed_exit() {
        let mut reveal = reveal(StartTileAnimationStyle::DiscoTurnstile, 0.39, true);
        reveal.launching_app = true;

        let selected = reveal.tile_frame(&tile(0), 0, 0, 2, 0, 1, true);
        let ordinary = reveal.tile_frame(&tile(1), 1, 1, 2, 0, 1, false);

        assert!(selected.visible);
        assert!(!ordinary.visible);
        assert_eq!(reveal.total_duration(1, 2), 0.50);
    }

    #[test]
    fn factor_launch_keeps_start_settled_below_the_clip_reveal() {
        let mut reveal = reveal(StartTileAnimationStyle::FactorSlideScale, 0.15, true);
        reveal.launching_app = true;

        let frame = reveal.tile_frame(&tile(0), 0, 0, 1, 0, 0, true);

        assert!(frame.visible);
        assert_eq!(animation_progress(frame), 1.0);
        assert_eq!(reveal.scene_progress(0, 1), 1.0);
        assert!(reveal.factor_clip_progress().is_some_and(|p| p > 0.5));
    }

    #[test]
    fn disco_app_list_keeps_the_selected_icon_until_its_fixed_delay() {
        let mut reveal = reveal(StartTileAnimationStyle::DiscoTurnstile, 0.32, true);
        reveal.launching_app = true;

        let selected = reveal
            .disco_app_list_frame(0, 4, true)
            .expect("Disco app-list frame");
        let ordinary = reveal
            .disco_app_list_frame(3, 4, false)
            .expect("Disco app-list frame");

        assert!(selected.visible);
        assert!(!ordinary.visible);
        assert!(selected.progress < 0.5);
    }
}

fn config_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".config/win10x/start_tiles.json"))
}

fn windows8_metro_layout(
    tiles: &[StartTile],
    groups: &[StartTileGroup],
    origin: Pos2,
    viewport: Vec2,
) -> Windows8MetroLayout {
    let max_rows = StartTileBoard::windows8_metro_max_rows(viewport.y) as usize;
    let tile_top = origin.y
        + WINDOWS8_METRO_TOP_PADDING
        + WINDOWS8_METRO_GROUP_TITLE_HEIGHT
        + WINDOWS8_METRO_GROUP_TITLE_GAP;
    let group_height = WINDOWS8_METRO_GROUP_TITLE_HEIGHT
        + WINDOWS8_METRO_GROUP_TITLE_GAP
        + max_rows as f32 * (WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP)
        - WINDOWS8_METRO_GAP;
    let mut cursor_x = origin.x + WINDOWS8_METRO_START_PADDING;
    let mut tile_layouts = Vec::with_capacity(tiles.len());
    let mut group_layouts = Vec::new();
    let mut board_bounds: Option<Rect> = None;

    for (group_index, group) in groups.iter().enumerate() {
        let indices: Vec<_> = tiles
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| (tile.group_id == group.id).then_some(index))
            .collect();
        if indices.is_empty() {
            continue;
        }
        let mut occupied = vec![vec![false; indices.len().saturating_mul(2).max(2) + 2]; max_rows];
        let mut packed: Vec<(usize, usize, usize, bool)> = Vec::with_capacity(indices.len());
        let mut max_col = 0usize;

        for index in indices {
            let tile = &tiles[index];
            // The reference data model exposes only normal and wide faces.
            let wide = tile.w > tile.h;
            let cells = if wide { 2 } else { 1 };
            let mut placed = None;
            'columns: for col in 0..=occupied[0].len().saturating_sub(cells) {
                for row in 0..max_rows {
                    if (0..cells).all(|offset| !occupied[row][col + offset]) {
                        placed = Some((row, col));
                        break 'columns;
                    }
                }
            }
            let (row, col) = placed.unwrap_or((0, max_col));
            for offset in 0..cells {
                if col + offset >= occupied[0].len() {
                    for row_cells in &mut occupied {
                        row_cells.resize(col + cells + 2, false);
                    }
                }
                occupied[row][col + offset] = true;
            }
            max_col = max_col.max(col + cells);
            packed.push((index, row, col, wide));
        }

        let group_width =
            max_col as f32 * (WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP) - WINDOWS8_METRO_GAP;
        let bounds = Rect::from_min_size(
            Pos2::new(cursor_x, origin.y + WINDOWS8_METRO_TOP_PADDING),
            Vec2::new(group_width + WINDOWS8_METRO_GROUP_GAP, group_height),
        );
        board_bounds = Some(board_bounds.map_or(bounds, |board| board.union(bounds)));
        group_layouts.push(Windows8MetroGroupLayout {
            id: group.id,
            title: group.title.clone(),
            index: group_index,
            bounds,
        });
        tile_layouts.extend(packed.into_iter().map(|(index, row, col, wide)| {
            Windows8MetroTileLayout {
                index,
                group_index,
                rect: Rect::from_min_size(
                    Pos2::new(
                        cursor_x + col as f32 * (WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP),
                        tile_top + row as f32 * (WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP),
                    ),
                    Vec2::new(
                        if wide {
                            WINDOWS8_METRO_TILE * 2.0 + WINDOWS8_METRO_GAP
                        } else {
                            WINDOWS8_METRO_TILE
                        },
                        WINDOWS8_METRO_TILE,
                    ),
                ),
                wide,
            }
        }));
        cursor_x += group_width + WINDOWS8_METRO_GROUP_GAP;
    }

    let group_bounds = board_bounds.unwrap_or_else(|| {
        Rect::from_min_size(
            Pos2::new(
                origin.x + WINDOWS8_METRO_START_PADDING,
                origin.y + WINDOWS8_METRO_TOP_PADDING,
            ),
            Vec2::ZERO,
        )
    });
    let content_width = viewport
        .x
        .max(cursor_x - origin.x + WINDOWS8_METRO_RIGHT_PADDING);
    Windows8MetroLayout {
        max_rows,
        group_bounds,
        content_size: Vec2::new(content_width, viewport.y),
        groups: group_layouts,
        tiles: tile_layouts,
    }
}

/// Win10-style live reorder preview for the derived Windows 8 Metro order.
///
/// The result is a pure function of the pre-drag snapshot plus the pointer:
/// the dragged tile is removed, the remaining siblings are laid out by the
/// ordinary Metro packer, and the pointer is mapped onto that sibling grid to
/// pick the insertion slot. Deriving every preview from the same snapshot
/// keeps the decision stable (no thrash) exactly like the freeform board's
/// `drag_preview_layout`. Grouping is preserved: the tile always joins an
/// existing group (the one under the pointer, or the nearest one), never a
/// freeform cell.
fn windows8_drag_preview(
    layout_before_drag: &[StartTile],
    groups: &[StartTileGroup],
    dragged_app_id: &str,
    pointer: Pos2,
    origin: Pos2,
    viewport: Vec2,
) -> Vec<StartTile> {
    let Some(source_index) = layout_before_drag
        .iter()
        .position(|tile| tile.app_id == dragged_app_id)
    else {
        return layout_before_drag.to_vec();
    };
    let mut dragged = layout_before_drag[source_index].clone();
    let mut others: Vec<StartTile> = layout_before_drag
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != source_index)
        .map(|(_, tile)| tile.clone())
        .collect();
    if others.is_empty() {
        return vec![dragged];
    }
    let layout = windows8_metro_layout(&others, groups, origin, viewport);
    // Containment and nearest-group fallback share one distance metric, so a
    // drop can never be silently cancelled between or below groups.
    let Some(target) = layout.groups.iter().min_by(|a, b| {
        rect_distance_sq(a.bounds, pointer).total_cmp(&rect_distance_sq(b.bounds, pointer))
    }) else {
        return layout_before_drag.to_vec();
    };

    let group_tiles: Vec<&Windows8MetroTileLayout> = layout
        .tiles
        .iter()
        .filter(|tile| tile.group_index == target.index)
        .collect();
    let stride = WINDOWS8_METRO_TILE + WINDOWS8_METRO_GAP;
    let x0 = group_tiles
        .iter()
        .map(|tile| tile.rect.min.x)
        .reduce(f32::min)
        .unwrap_or(origin.x);
    let y0 = group_tiles
        .iter()
        .map(|tile| tile.rect.min.y)
        .reduce(f32::min)
        .unwrap_or(origin.y);
    let grid_cols = group_tiles
        .iter()
        .map(|tile| ((tile.rect.min.x - x0) / stride).round() as isize)
        .max()
        .unwrap_or(0) as isize
        + 1;
    let max_rows = layout.max_rows.max(1) as isize;
    let pointer_col = (((pointer.x - x0) / stride).round() as isize).clamp(0, grid_cols);
    let pointer_row = (((pointer.y - y0) / stride).round() as isize).clamp(0, max_rows);
    let pointer_slot = pointer_col * max_rows + pointer_row;

    // Column-first anchor slots are unique per tile, so at most one sibling
    // shares the pointer's slot; resolve it with a center tie-break.
    let anchored: Vec<(isize, &Windows8MetroTileLayout)> = group_tiles
        .iter()
        .map(|tile| {
            let col = ((tile.rect.min.x - x0) / stride).round() as isize;
            let row = ((tile.rect.min.y - y0) / stride).round() as isize;
            (col * max_rows + row, *tile)
        })
        .collect();
    let mut local = anchored
        .iter()
        .filter(|(slot, _)| *slot < pointer_slot)
        .count();
    if let Some((_, tile)) = anchored.iter().find(|(slot, _)| *slot == pointer_slot) {
        let center = tile.rect.center();
        let after = pointer.y > center.y
            || ((pointer.y - center.y).abs() < tile.rect.height() * 0.35 && pointer.x > center.x);
        if after {
            local += 1;
        }
    }

    dragged.group_id = target.id;
    let members: Vec<usize> = others
        .iter()
        .enumerate()
        .filter(|(_, tile)| tile.group_id == target.id)
        .map(|(index, _)| index)
        .collect();
    let insert_at = if local >= members.len() {
        members
            .last()
            .map(|index| index + 1)
            .unwrap_or(others.len())
    } else {
        members[local]
    };
    others.insert(insert_at, dragged);
    others
}

fn rect_distance_sq(rect: Rect, point: Pos2) -> f32 {
    let dx = if point.x < rect.min.x {
        rect.min.x - point.x
    } else if point.x > rect.max.x {
        point.x - rect.max.x
    } else {
        0.0
    };
    let dy = if point.y < rect.min.y {
        rect.min.y - point.y
    } else if point.y > rect.max.y {
        point.y - rect.max.y
    } else {
        0.0
    };
    dx * dx + dy * dy
}

fn cell_metrics(area: Rect) -> (f32, f32) {
    let target_rows = 6.0;
    let unit_from_h = ((area.height() - GAP) / target_rows - GAP).clamp(26.0, 52.0);
    let target_cols = 12.0;
    let unit_from_w = ((area.width() - GAP) / target_cols - GAP).clamp(26.0, 52.0);
    let unit = unit_from_h.min(unit_from_w);
    (unit, GAP)
}

fn tile_pixel_rect(origin: Pos2, tile: &StartTile, unit: f32, gap: f32) -> Rect {
    let stride = unit + gap;
    let x = origin.x + tile.col as f32 * stride;
    let y = origin.y + tile.row as f32 * stride;
    let w = tile.w as f32 * unit + (tile.w.saturating_sub(1) as f32) * gap;
    let h = tile.h as f32 * unit + (tile.h.saturating_sub(1) as f32) * gap;
    Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h))
}

fn resize_handle_rect(tile: Rect) -> Rect {
    Rect::from_min_size(
        Pos2::new(tile.max.x - HANDLE, tile.max.y - HANDLE),
        Vec2::splat(HANDLE),
    )
}

fn size_from_pointer(
    tile: &StartTile,
    pos: Pos2,
    origin: Pos2,
    unit: f32,
    gap: f32,
) -> Option<(u8, u8)> {
    let stride = unit + gap;
    let end_col = ((pos.x - origin.x + gap) / stride).ceil() as i32;
    let end_row = ((pos.y - origin.y + gap) / stride).ceil() as i32;
    let w = (end_col - tile.col).clamp(i32::from(TILE_MIN), i32::from(TILE_MAX)) as u8;
    let h = (end_row - tile.row).clamp(i32::from(TILE_MIN), i32::from(TILE_MAX)) as u8;
    Some((w, h))
}

fn snap_origin(
    float_min: Pos2,
    origin: Pos2,
    unit: f32,
    gap: f32,
    max_cols: i32,
    max_rows: i32,
    tile: &StartTile,
) -> (i32, i32) {
    let stride = unit + gap;
    let col = ((float_min.x - origin.x) / stride)
        .round()
        .clamp(0.0, (max_cols - i32::from(tile.w)).max(0) as f32) as i32;
    let row = ((float_min.y - origin.y) / stride)
        .round()
        .clamp(0.0, (max_rows - i32::from(tile.h) + 8).max(0) as f32) as i32;
    (col.max(0), row.max(0))
}

/// Width along the top edge, height along the left edge — both **inside** the tile.
fn paint_size_guides_inside(ui: &mut Ui, rect: Rect, w: u8, h: u8, fg: Color32, pal: &Palette) {
    let guide = if fg == Color32::WHITE {
        Color32::from_rgba_unmultiplied(255, 255, 255, 230)
    } else {
        pal.accent
    };
    let pad = 8.0;
    let top_y = rect.min.y + pad + 6.0;
    ui.painter().line_segment(
        [
            Pos2::new(rect.min.x + pad, top_y),
            Pos2::new(rect.max.x - pad, top_y),
        ],
        Stroke::new(1.5, guide),
    );
    ui.painter().text(
        Pos2::new(rect.center().x, top_y - 1.0),
        Align2::CENTER_BOTTOM,
        format!("{w}"),
        FontId::proportional(12.0),
        guide,
    );

    let left_x = rect.min.x + pad + 6.0;
    ui.painter().line_segment(
        [
            Pos2::new(left_x, rect.min.y + pad),
            Pos2::new(left_x, rect.max.y - pad),
        ],
        Stroke::new(1.5, guide),
    );
    ui.painter().text(
        Pos2::new(left_x - 1.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("{h}"),
        FontId::proportional(12.0),
        guide,
    );
}

/// Four-way move affordance shown while long-pressing / dragging a tile.
fn paint_drag_icon(ui: &mut Ui, rect: Rect, fg: Color32, pal: &Palette) {
    let badge = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.min.y + 18.0),
        Vec2::new(28.0, 22.0),
    );
    ui.painter().rect_filled(
        badge,
        CornerRadius::same(3),
        Color32::from_rgba_unmultiplied(0, 0, 0, 90),
    );
    ui.painter().rect_stroke(
        badge,
        CornerRadius::same(3),
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 60)),
        StrokeKind::Inside,
    );
    let icon_c = if fg == Color32::WHITE {
        Color32::WHITE
    } else {
        pal.accent
    };
    // Simple ✥ / four-way arrows as drag glyph (works with CJK-capable proportional).
    ui.painter().text(
        badge.center(),
        Align2::CENTER_CENTER,
        "✥",
        FontId::proportional(14.0),
        icon_c,
    );
}

fn drag_preview_layout(
    layout_before_drag: &[StartTile],
    fixed_idx: usize,
    col: i32,
    row: i32,
    max_cols: i32,
    max_rows: i32,
) -> Vec<StartTile> {
    let mut preview = layout_before_drag.to_vec();
    let preferred_vacancy = preview
        .get(fixed_idx)
        .map(|fixed| (fixed.col, fixed.row))
        .unwrap_or((col, row));
    let Some(fixed) = preview.get_mut(fixed_idx) else {
        return preview;
    };
    fixed.col = col;
    fixed.row = row;
    resolve_avoid_toward(
        &mut preview,
        fixed_idx,
        max_cols,
        max_rows,
        preferred_vacancy,
    );
    preview
}

fn resolve_avoid(tiles: &mut [StartTile], fixed_idx: usize, max_cols: i32, max_rows: i32) {
    if fixed_idx >= tiles.len() {
        return;
    }
    let preferred_vacancy = (tiles[fixed_idx].col, tiles[fixed_idx].row);
    resolve_avoid_toward(tiles, fixed_idx, max_cols, max_rows, preferred_vacancy);
}

fn resolve_avoid_toward(
    tiles: &mut [StartTile],
    fixed_idx: usize,
    max_cols: i32,
    max_rows: i32,
    preferred_vacancy: (i32, i32),
) {
    let fixed = tiles[fixed_idx].clone();
    let mut conflicts: Vec<usize> = (0..tiles.len())
        .filter(|&i| i != fixed_idx && tiles[i].overlaps(&fixed))
        .collect();
    conflicts.sort_by_key(|&i| {
        (
            (tiles[i].row - fixed.row).abs() + (tiles[i].col - fixed.col).abs(),
            tiles[i].row,
            tiles[i].col,
        )
    });

    for i in conflicts {
        if let Some((col, row)) =
            find_nearest_free_slot(tiles, i, max_cols, max_rows, preferred_vacancy)
        {
            tiles[i].col = col;
            tiles[i].row = row;
        }
    }
}

fn find_nearest_free_slot(
    tiles: &[StartTile],
    moving_idx: usize,
    max_cols: i32,
    max_rows: i32,
    preferred_vacancy: (i32, i32),
) -> Option<(i32, i32)> {
    let moving = tiles.get(moving_idx)?;
    let origin = (moving.col, moving.row);
    let occupied_right = tiles
        .iter()
        .map(|tile| tile.col + i32::from(tile.w))
        .max()
        .unwrap_or(0);
    let occupied_bottom = tiles
        .iter()
        .map(|tile| tile.row + i32::from(tile.h))
        .max()
        .unwrap_or(0);
    let search_cols = max_cols.max(occupied_right + 8).max(i32::from(moving.w));
    let search_rows = max_rows.max(occupied_bottom + 8).max(i32::from(moving.h));
    let last_col = (search_cols - i32::from(moving.w)).max(0);
    let last_row = (search_rows - i32::from(moving.h)).max(0);

    let mut best: Option<((i32, i32, i32, i32, i32), (i32, i32))> = None;
    for row in 0..=last_row {
        for col in 0..=last_col {
            let candidate = StartTile {
                app_id: String::new(),
                group_id: moving.group_id,
                col,
                row,
                w: moving.w,
                h: moving.h,
            };
            if tiles
                .iter()
                .enumerate()
                .any(|(i, tile)| i != moving_idx && candidate.overlaps(tile))
            {
                continue;
            }

            let distance = (col - origin.0).abs() + (row - origin.1).abs();
            let vacancy_distance =
                (col - preferred_vacancy.0).abs() + (row - preferred_vacancy.1).abs();
            let key = (distance, vacancy_distance, (row - origin.1).abs(), row, col);
            if best.as_ref().is_none_or(|(best_key, _)| key < *best_key) {
                best = Some((key, (col, row)));
            }
        }
    }
    best.map(|(_, position)| position)
}

fn find_free_slot(tiles: &[StartTile], w: u8, h: u8, max_cols: i32) -> (i32, i32) {
    find_free_slot_excluding(tiles, w, h, max_cols, 64, &[], None).unwrap_or((0, 0))
}

fn find_free_slot_flow(
    tiles: &[StartTile],
    w: u8,
    h: u8,
    primary_limit: i32,
    flow: StartTileFlow,
) -> (i32, i32) {
    let primary_limit = primary_limit.max(match flow {
        StartTileFlow::Vertical => i32::from(w),
        StartTileFlow::Horizontal => i32::from(h),
    });
    for advancing in 0..256 {
        for cross in 0..primary_limit {
            let (col, row) = match flow {
                StartTileFlow::Vertical => (cross, advancing),
                StartTileFlow::Horizontal => (advancing, cross),
            };
            let candidate = StartTile {
                app_id: String::new(),
                group_id: DEFAULT_TILE_GROUP_ID,
                col,
                row,
                w,
                h,
            };
            let cross_end = match flow {
                StartTileFlow::Vertical => candidate.col + i32::from(candidate.w),
                StartTileFlow::Horizontal => candidate.row + i32::from(candidate.h),
            };
            if cross_end <= primary_limit && !tiles.iter().any(|tile| candidate.overlaps(tile)) {
                return (col, row);
            }
        }
    }
    (0, 0)
}

fn pack_tiles_grouped(
    tiles: &[StartTile],
    groups: &[StartTileGroup],
    flow: StartTileFlow,
    primary_limit: i32,
) -> Vec<StartTile> {
    let mut packed_all = Vec::with_capacity(tiles.len());
    let mut axis_offset = 0;
    let mut ordered_group_ids: Vec<u32> = groups.iter().map(|group| group.id).collect();
    for tile in tiles {
        if !ordered_group_ids.contains(&tile.group_id) {
            ordered_group_ids.push(tile.group_id);
        }
    }

    for group_id in ordered_group_ids {
        let members: Vec<_> = tiles
            .iter()
            .filter(|tile| tile.group_id == group_id)
            .cloned()
            .collect();
        if members.is_empty() {
            continue;
        }
        let mut packed = pack_tiles(&members, flow, primary_limit);
        match flow {
            StartTileFlow::Vertical => {
                for tile in &mut packed {
                    tile.row += axis_offset;
                }
                axis_offset = packed
                    .iter()
                    .map(|tile| tile.row + i32::from(tile.h))
                    .max()
                    .unwrap_or(axis_offset)
                    + 2;
            }
            StartTileFlow::Horizontal => {
                for tile in &mut packed {
                    tile.col += axis_offset;
                }
                axis_offset = packed
                    .iter()
                    .map(|tile| tile.col + i32::from(tile.w))
                    .max()
                    .unwrap_or(axis_offset)
                    + 2;
            }
        }
        packed_all.extend(packed);
    }
    packed_all
}

fn pack_tiles(tiles: &[StartTile], flow: StartTileFlow, primary_limit: i32) -> Vec<StartTile> {
    let mut packed = Vec::with_capacity(tiles.len());
    for tile in tiles {
        let mut tile = tile.clone().clamped();
        let (col, row) = find_free_slot_flow(&packed, tile.w, tile.h, primary_limit, flow);
        tile.col = col;
        tile.row = row;
        packed.push(tile);
    }
    packed
}

fn find_free_slot_excluding(
    tiles: &[StartTile],
    w: u8,
    h: u8,
    max_cols: i32,
    max_rows: i32,
    against: &[usize],
    skip: Option<usize>,
) -> Option<(i32, i32)> {
    let probe_against: Vec<usize> = if against.is_empty() {
        (0..tiles.len()).filter(|&i| Some(i) != skip).collect()
    } else {
        against.to_vec()
    };
    for row in 0..max_rows {
        for col in 0..max_cols {
            let cand = StartTile {
                app_id: String::new(),
                group_id: DEFAULT_TILE_GROUP_ID,
                col,
                row,
                w,
                h,
            };
            let hit = probe_against.iter().any(|&j| {
                if Some(j) == skip {
                    return false;
                }
                cand.overlaps(&tiles[j])
            });
            if !hit {
                return Some((col, row));
            }
        }
    }
    None
}

fn tile_fill_for(id: &str, name: &str, pal: &Palette, hovered: bool) -> Color32 {
    const SWATCH: [[u8; 3]; 10] = [
        [0x00, 0x78, 0xd4],
        [0x10, 0x7c, 0x10],
        [0x88, 0x17, 0x98],
        [0xda, 0x3b, 0x01],
        [0x00, 0xb2, 0x94],
        [0xe3, 0x00, 0x8c],
        [0x00, 0x63, 0xb1],
        [0xc2, 0x39, 0xb3],
        [0x03, 0x83, 0x87],
        [0x4c, 0x4a, 0x78],
    ];
    let key = format!("{id}:{name}");
    let mut h = 0u32;
    for b in key.bytes() {
        h = h.wrapping_mul(16777619) ^ u32::from(b);
    }
    let [r, g, b] = SWATCH[(h as usize) % SWATCH.len()];
    let solid = h % 3 == 0;
    let base = if solid {
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgba_unmultiplied(255, 255, 255, 210)
    };
    if hovered {
        Color32::from_rgba_unmultiplied(
            base.r().saturating_add(12),
            base.g().saturating_add(12),
            base.b().saturating_add(12),
            base.a().saturating_add(20).min(255),
        )
    } else {
        let _ = pal;
        base
    }
}

fn tile_fg_for(fill: Color32) -> Color32 {
    let lum = 0.2126 * fill.r() as f32 + 0.7152 * fill.g() as f32 + 0.0722 * fill.b() as f32;
    if lum < 140.0 {
        Color32::WHITE
    } else {
        Color32::from_rgb(0x20, 0x20, 0x20)
    }
}

fn truncate_label(s: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i >= max_chars {
            out.push('…');
            break;
        }
        out.push(ch);
    }
    out
}
