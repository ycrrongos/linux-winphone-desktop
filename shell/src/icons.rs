//! FreeDesktop application icon resolution + egui texture cache.
//!
//! Icons are **preloaded** on a background thread (resolve + decode). The UI thread
//! only uploads a few textures per frame so first paint stays smooth.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::OnceLock;
use std::thread;

use eframe::egui::{
    self, Color32, ColorImage, Pos2, Rect, TextureHandle, TextureOptions, Ui, Vec2,
};
use image::imageops::FilterType;
use win10x_protocol::AppEntry;

use crate::desktop::app_icon;

const CACHE_PX: u32 = 64;
/// Max GPU texture uploads per frame (keeps frame time stable while preloading).
const UPLOADS_PER_FRAME: usize = 6;

type IconIndex = HashMap<String, PathBuf>;

#[derive(Default)]
pub struct IconCache {
    textures: HashMap<String, TextureHandle>,
    failed: HashSet<String>,
    /// Decoded on worker; drained on UI thread.
    rx: Option<Receiver<(String, Option<ColorImage>)>>,
    pending_upload: VecDeque<(String, ColorImage)>,
    /// Still waiting on the worker for these ids.
    inflight: HashSet<String>,
    preload_started: bool,
}

impl IconCache {
    /// Kick off background decode for all apps (idempotent).
    pub fn start_preload(&mut self, apps: &[AppEntry]) {
        if self.preload_started {
            return;
        }
        self.preload_started = true;

        let jobs: Vec<(String, Option<String>, Option<String>)> = apps
            .iter()
            .map(|a| (a.id.clone(), a.icon.clone(), a.desktop_id.clone()))
            .collect();

        let (tx, rx) = mpsc::channel();
        self.rx = Some(rx);
        for (id, _, _) in &jobs {
            self.inflight.insert(id.clone());
        }

        thread::Builder::new()
            .name("win10x-icon-preload".into())
            .spawn(move || {
                let index = icon_index();
                for (id, icon, desktop_id) in jobs {
                    let path = icon
                        .as_deref()
                        .and_then(|n| lookup_indexed(index, n))
                        .or_else(|| lookup_indexed(index, &id))
                        .or_else(|| {
                            desktop_id
                                .as_deref()
                                .map(|d| d.trim_end_matches(".desktop"))
                                .and_then(|n| lookup_indexed(index, n))
                        })
                        .or_else(|| {
                            // Slow path for absolute Icon= paths / misses.
                            icon.as_deref()
                                .and_then(resolve_icon_path_fallback)
                                .or_else(|| resolve_icon_path_fallback(&id))
                        });

                    let img = path.as_ref().and_then(|p| load_icon_image(p, CACHE_PX));
                    if tx.send((id, img)).is_err() {
                        break;
                    }
                }
            })
            .ok();
    }

    /// Pull worker results and upload a few textures. Returns true if more work remains.
    pub fn pump(&mut self, ctx: &egui::Context) -> bool {
        if let Some(rx) = &self.rx {
            loop {
                match rx.try_recv() {
                    Ok((id, img)) => {
                        self.inflight.remove(&id);
                        match img {
                            Some(img) => self.pending_upload.push_back((id, img)),
                            None => {
                                self.failed.insert(id);
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        self.rx = None;
                        break;
                    }
                }
            }
        }

        let mut n = 0;
        while n < UPLOADS_PER_FRAME {
            let Some((id, img)) = self.pending_upload.pop_front() else {
                break;
            };
            if self.textures.contains_key(&id) {
                continue;
            }
            let tex = ctx.load_texture(format!("app-icon-{id}"), img, TextureOptions::LINEAR);
            self.textures.insert(id, tex);
            n += 1;
        }

        self.rx.is_some() || !self.pending_upload.is_empty() || !self.inflight.is_empty()
    }

    pub fn texture(&self, app: &AppEntry) -> Option<&TextureHandle> {
        self.textures.get(&app.id)
    }
}

/// Paint a real app icon if available, otherwise the Fluent glyph fallback.
pub fn paint_app_icon(
    ui: &mut Ui,
    cache: &IconCache,
    app: &AppEntry,
    center: Pos2,
    size: f32,
    fallback_tint: Color32,
) {
    let rect = Rect::from_center_size(center, Vec2::splat(size));
    if let Some(tex) = cache.texture(app) {
        ui.painter().image(
            tex.id(),
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        return;
    }
    let ic = app_icon(&app.name, &app.id);
    ui.painter().text(
        center,
        egui::Align2::CENTER_CENTER,
        ic.glyph(),
        fluent_egui::winui::icon_font_id(size * 0.85),
        fallback_tint,
    );
}

fn icon_index() -> &'static IconIndex {
    static INDEX: OnceLock<IconIndex> = OnceLock::new();
    INDEX.get_or_init(build_icon_index)
}

fn lookup_indexed(index: &IconIndex, name: &str) -> Option<PathBuf> {
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    let path = Path::new(name);
    if path.is_absolute() || name.contains('/') {
        return path.exists().then(|| path.to_path_buf());
    }
    index.get(name).cloned()
}

/// One-time scan of common icon locations. Prefer mid-size PNG over SVG.
fn build_icon_index() -> IconIndex {
    let mut best: HashMap<String, (i32, PathBuf)> = HashMap::new();

    let score = |path: &Path| -> i32 {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let s = path.to_string_lossy();
        let mut sc = 0i32;
        if ext == "png" {
            sc += 100;
        } else if ext == "svg" {
            sc += 40;
        } else {
            sc -= 50;
        }
        // Prefer hicolor and typical launcher sizes.
        if s.contains("/hicolor/") {
            sc += 30;
        }
        if s.contains("/48x48/") || s.contains("/64x64/") {
            sc += 25;
        } else if s.contains("/128x128/") || s.contains("/96x96/") {
            sc += 15;
        } else if s.contains("/scalable/") {
            sc += 10;
        } else if s.contains("/16x16/") || s.contains("/22x22/") {
            sc -= 20;
        }
        if s.contains("HighContrast") {
            sc -= 40;
        }
        sc
    };

    let consider = |best: &mut HashMap<String, (i32, PathBuf)>, path: PathBuf| {
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            return;
        };
        if stem.is_empty() {
            return;
        }
        let sc = score(&path);
        match best.get(stem) {
            Some((old, _)) if *old >= sc => {}
            _ => {
                best.insert(stem.to_string(), (sc, path));
            }
        }
    };

    // Focused dirs first (fast + high quality).
    let focused = [
        "/usr/share/icons/hicolor/48x48/apps",
        "/usr/share/icons/hicolor/64x64/apps",
        "/usr/share/icons/hicolor/128x128/apps",
        "/usr/share/icons/hicolor/scalable/apps",
        "/usr/share/icons/hicolor/32x32/apps",
        "/usr/share/pixmaps",
    ];
    for dir in focused {
        let dir = PathBuf::from(dir);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.flatten() {
            let p = ent.path();
            let ext = p
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if matches!(ext.as_str(), "png" | "svg") {
                consider(&mut best, p);
            }
        }
    }

    // Broader pass for themes (skip huge trees by only apps/ + common sizes).
    for root in icon_roots() {
        if root.ends_with("pixmaps") {
            continue;
        }
        let Ok(themes) = fs::read_dir(root) else {
            continue;
        };
        for theme in themes.flatten().map(|e| e.path()).filter(|p| p.is_dir()) {
            let name = theme
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if name == "hicolor" {
                continue; // already scanned focused paths
            }
            for size in ["48x48", "64x64", "128x128", "scalable", "32x32"] {
                for cat in ["apps", "applications"] {
                    let dir = theme.join(size).join(cat);
                    let Ok(rd) = fs::read_dir(&dir) else {
                        continue;
                    };
                    for ent in rd.flatten() {
                        let p = ent.path();
                        let ext = p
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_ascii_lowercase();
                        if matches!(ext.as_str(), "png" | "svg") {
                            consider(&mut best, p);
                        }
                    }
                }
            }
            // char-white style: theme/apps/48/
            for px in ["48", "64", "128", "32"] {
                let dir = theme.join("apps").join(px);
                let Ok(rd) = fs::read_dir(&dir) else {
                    continue;
                };
                for ent in rd.flatten() {
                    let p = ent.path();
                    let ext = p
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if matches!(ext.as_str(), "png" | "svg") {
                        consider(&mut best, p);
                    }
                }
            }
        }
    }

    best.into_iter().map(|(k, (_, p))| (k, p)).collect()
}

fn icon_roots() -> &'static [PathBuf] {
    static ROOTS: OnceLock<Vec<PathBuf>> = OnceLock::new();
    ROOTS.get_or_init(|| {
        let mut v = Vec::new();
        if let Ok(home) = std::env::var("HOME") {
            v.push(PathBuf::from(&home).join(".local/share/icons"));
            v.push(PathBuf::from(&home).join(".icons"));
        }
        if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
            for part in xdg.split(':').filter(|s| !s.is_empty()) {
                v.push(PathBuf::from(part).join("icons"));
            }
        }
        v.push(PathBuf::from("/usr/share/icons"));
        v.push(PathBuf::from("/usr/local/share/icons"));
        v.push(PathBuf::from("/usr/share/pixmaps"));
        v
    })
}

fn resolve_icon_path_fallback(icon: &str) -> Option<PathBuf> {
    let icon = icon.trim();
    if icon.is_empty() {
        return None;
    }
    let path = Path::new(icon);
    if path.is_absolute() || icon.contains('/') {
        return path.exists().then(|| path.to_path_buf());
    }
    // Last resort: direct pixmaps / hicolor only (no full theme walk).
    for dir in [
        PathBuf::from("/usr/share/pixmaps"),
        PathBuf::from("/usr/share/icons/hicolor/48x48/apps"),
        PathBuf::from("/usr/share/icons/hicolor/64x64/apps"),
        PathBuf::from("/usr/share/icons/hicolor/scalable/apps"),
    ] {
        for ext in ["png", "svg"] {
            let p = dir.join(format!("{icon}.{ext}"));
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

fn load_icon_image(path: &Path, size: u32) -> Option<ColorImage> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let rgba = if ext == "svg" {
        render_svg(path, size)?
    } else {
        let img = image::open(path).ok()?;
        let resized = img.resize(size, size, FilterType::Triangle).into_rgba8();
        resized.into_raw()
    };
    Some(ColorImage::from_rgba_unmultiplied(
        [size as usize, size as usize],
        &rgba,
    ))
}

fn render_svg(path: &Path, size: u32) -> Option<Vec<u8>> {
    let data = fs::read(path).ok()?;
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&data, &opt).ok()?;
    let mut pixmap = tiny_skia::Pixmap::new(size, size)?;
    let raw_size = tree.size();
    let sx = size as f32 / raw_size.width().max(1.0);
    let sy = size as f32 / raw_size.height().max(1.0);
    let scale = sx.min(sy);
    let tx = (size as f32 - raw_size.width() * scale) * 0.5;
    let ty = (size as f32 - raw_size.height() * scale) * 0.5;
    let transform = tiny_skia::Transform::from_row(scale, 0.0, 0.0, scale, tx, ty);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(pixmap.take())
}
