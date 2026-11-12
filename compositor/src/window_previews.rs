//! Runtime images for individual xdg-toplevel previews and switch handoffs.

use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{sync_channel, SyncSender, TrySendError},
        OnceLock,
    },
    time::Duration,
};

use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{surface::WaylandSurfaceRenderElement, AsRenderElements},
            gles::{GlesRenderer, GlesTexture},
            utils::draw_render_elements,
            Bind, Color32F, ExportMem, Frame, Offscreen, Renderer,
        },
    },
    desktop::Window,
    utils::{Rectangle, Scale, Transform},
};
use win10x_protocol::Panel;

use crate::{session_ipc::window_identity, Win10xState};

const OPEN_REFRESH_INTERVAL: Duration = Duration::from_millis(700);
const BACKGROUND_REFRESH_INTERVAL: Duration = Duration::from_secs(2);
const SWITCH_RESOLUTION_SCALE: f64 = 0.25;
const THUMBNAIL_MAX_WIDTH: u32 = 320;
const THUMBNAIL_MAX_HEIGHT: u32 = 180;

struct PreviewEncodeJob {
    window_id: usize,
    image: image::RgbaImage,
    thumbnail_path: PathBuf,
    transition_path: Option<PathBuf>,
}

static SWITCH_ANIMATION_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_switch_animation_enabled(enabled: bool) {
    SWITCH_ANIMATION_ENABLED.store(enabled, Ordering::Release);
}

pub fn refresh_if_needed(state: &mut Win10xState) {
    let menu_open = state.effective_panel() == Panel::TaskbarAppMenu;
    // With switch motion disabled there is no background GPU readback at all.
    // A deliberately opened multi-window menu may still request its small cards.
    let Some(include_transition) = capture_policy(state.app_switch_animation_enabled, menu_open)
    else {
        state.last_preview_capture = None;
        state.preview_cursor = 0;
        return;
    };
    let windows: Vec<_> = state
        .space
        .elements()
        .filter(|window| !Win10xState::is_shell_window(window))
        .filter(|window| window_identity(window).is_some())
        .map(|window| {
            let window_id = window.id();
            (
                window.clone(),
                state.preview_dir.join(format!("window-{window_id}.png")),
                state
                    .preview_dir
                    .join(format!("window-{window_id}-switch.png")),
            )
        })
        .collect();
    // App-switch animations need a cached surface even when an application has
    // only one toplevel. Multi-window menus use the same files at a faster rate.
    if windows.is_empty() {
        state.last_preview_capture = None;
        state.preview_cursor = 0;
        return;
    }
    let refresh_cycle = if menu_open {
        OPEN_REFRESH_INTERVAL
    } else {
        BACKGROUND_REFRESH_INTERVAL
    };
    let slice_interval = preview_slice_interval(refresh_cycle, windows.len());
    if state
        .last_preview_capture
        .is_some_and(|last| last.elapsed() < slice_interval)
    {
        return;
    }
    state.last_preview_capture = Some(std::time::Instant::now());

    // Spread one complete refresh cycle across frames instead of synchronously
    // reading back every toplevel in the same render pass.
    let index = state.preview_cursor % windows.len();
    state.preview_cursor = (index + 1) % windows.len();
    let (window, thumbnail_path, transition_path) = windows
        .into_iter()
        .nth(index)
        .expect("preview index is inside the collected window list");

    let Some(backend) = state.backend.as_mut() else {
        return;
    };
    let renderer = backend.renderer();
    let output_scale = state.applied_output_scale;
    match capture_window(
        renderer,
        &window,
        thumbnail_path,
        transition_path,
        output_scale,
        include_transition,
    ) {
        Ok(job) => enqueue_preview(job),
        Err(error) => {
            tracing::debug!(window_id = window.id(), %error, "window preview capture skipped")
        }
    }
}

fn capture_window(
    renderer: &mut GlesRenderer,
    window: &Window,
    thumbnail_path: PathBuf,
    transition_path: PathBuf,
    output_scale: f64,
    include_transition: bool,
) -> Result<PreviewEncodeJob, String> {
    let bbox = window.bbox();
    if bbox.size.w <= 0 || bbox.size.h <= 0 {
        return Err("window has no committed buffer".into());
    }
    // A quarter-width/quarter-height switch image retains more detail than the
    // taskbar thumbnail without synchronously encoding a full-output PNG.
    let capture_scale = if include_transition {
        (output_scale * SWITCH_RESOLUTION_SCALE).max(0.01)
    } else {
        thumbnail_capture_scale((bbox.size.w, bbox.size.h))
    };
    let scale = Scale::from(capture_scale);
    let size = capture_dimensions((bbox.size.w, bbox.size.h), capture_scale);
    let location =
        smithay::utils::Point::<i32, smithay::utils::Logical>::from((-bbox.loc.x, -bbox.loc.y))
            .to_physical_precise_round(scale);
    let elements = AsRenderElements::<GlesRenderer>::render_elements::<
        WaylandSurfaceRenderElement<GlesRenderer>,
    >(window, renderer, location, scale, 1.0);

    let mut offscreen =
        Offscreen::<GlesTexture>::create_buffer(renderer, Fourcc::Abgr8888, size.into())
            .map_err(|error| error.to_string())?;
    let mut framebuffer = renderer
        .bind(&mut offscreen)
        .map_err(|error| error.to_string())?;
    let damage = Rectangle::from_size(size.into());
    let mut frame = renderer
        .render(&mut framebuffer, size.into(), Transform::Normal)
        .map_err(|error| error.to_string())?;
    frame
        .clear(Color32F::new(0.0, 0.0, 0.0, 0.0), &[damage])
        .map_err(|error| error.to_string())?;
    draw_render_elements(&mut frame, scale, &elements, &[damage])
        .map_err(|error| error.to_string())?;
    frame
        .finish()
        .map_err(|error| error.to_string())?
        .wait()
        .map_err(|error| error.to_string())?;

    let mapping = renderer
        .copy_framebuffer(
            &framebuffer,
            Rectangle::<i32, smithay::utils::Buffer>::from_size(size.into()),
            Fourcc::Abgr8888,
        )
        .map_err(|error| error.to_string())?;
    let pixels = renderer
        .map_texture(&mapping)
        .map_err(|error| error.to_string())?
        .to_vec();
    let image = image::RgbaImage::from_raw(size.0 as u32, size.1 as u32, pixels)
        .ok_or_else(|| "invalid RGBA preview buffer".to_string())?;

    Ok(PreviewEncodeJob {
        window_id: window.id(),
        image,
        thumbnail_path,
        transition_path: include_transition.then_some(transition_path),
    })
}

fn enqueue_preview(job: PreviewEncodeJob) {
    match preview_encoder().try_send(job) {
        Ok(()) => {}
        Err(TrySendError::Full(job)) => {
            tracing::debug!(
                window_id = job.window_id,
                "preview encoder busy; dropping stale capture"
            );
        }
        Err(TrySendError::Disconnected(job)) => {
            tracing::warn!(
                window_id = job.window_id,
                "preview encoder stopped; dropping capture"
            );
        }
    }
}

fn preview_encoder() -> &'static SyncSender<PreviewEncodeJob> {
    static ENCODER: OnceLock<SyncSender<PreviewEncodeJob>> = OnceLock::new();
    ENCODER.get_or_init(|| {
        let (sender, receiver) = sync_channel::<PreviewEncodeJob>(1);
        std::thread::Builder::new()
            .name("win10x-preview-encoder".into())
            .spawn(move || {
                while let Ok(job) = receiver.recv() {
                    let window_id = job.window_id;
                    if job.transition_path.is_some()
                        && !SWITCH_ANIMATION_ENABLED.load(Ordering::Acquire)
                    {
                        tracing::debug!(window_id, "discarding disabled switch capture");
                        continue;
                    }
                    if let Err(error) = encode_preview(job) {
                        tracing::warn!(window_id, %error, "window preview encode failed");
                    }
                }
            })
            .expect("preview encoder thread should start");
        sender
    })
}

fn encode_preview(job: PreviewEncodeJob) -> Result<(), String> {
    if let Some(path) = &job.transition_path {
        save_atomic_png(&job.image, path)?;
    }

    let thumbnail_size = thumbnail_dimensions((job.image.width(), job.image.height()));
    let thumbnail = image::imageops::resize(
        &job.image,
        thumbnail_size.0,
        thumbnail_size.1,
        image::imageops::FilterType::Triangle,
    );
    save_atomic_png(&thumbnail, &job.thumbnail_path)
}

fn thumbnail_capture_scale(logical: (i32, i32)) -> f64 {
    (f64::from(THUMBNAIL_MAX_WIDTH) / f64::from(logical.0.max(1)))
        .min(f64::from(THUMBNAIL_MAX_HEIGHT) / f64::from(logical.1.max(1)))
        .min(1.0)
        .max(0.01)
}

fn capture_policy(switch_animation_enabled: bool, menu_open: bool) -> Option<bool> {
    if switch_animation_enabled {
        Some(true)
    } else if menu_open {
        Some(false)
    } else {
        None
    }
}

fn save_atomic_png(image: &image::RgbaImage, path: &Path) -> Result<(), String> {
    let mut temporary = path.to_path_buf();
    temporary.set_extension("tmp.png");
    image.save(&temporary).map_err(|error| error.to_string())?;
    std::fs::rename(&temporary, path).map_err(|error| error.to_string())?;
    Ok(())
}

fn capture_dimensions(logical: (i32, i32), output_scale: f64) -> (i32, i32) {
    (
        (f64::from(logical.0) * output_scale).round().max(1.0) as i32,
        (f64::from(logical.1) * output_scale).round().max(1.0) as i32,
    )
}

fn preview_slice_interval(refresh_cycle: Duration, window_count: usize) -> Duration {
    refresh_cycle / u32::try_from(window_count.max(1)).unwrap_or(u32::MAX)
}

fn thumbnail_dimensions(source: (u32, u32)) -> (u32, u32) {
    let scale = (f64::from(THUMBNAIL_MAX_WIDTH) / f64::from(source.0))
        .min(f64::from(THUMBNAIL_MAX_HEIGHT) / f64::from(source.1))
        .min(1.0);
    (
        (f64::from(source.0) * scale).round().max(1.0) as u32,
        (f64::from(source.1) * scale).round().max(1.0) as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_capture_uses_quarter_output_dimensions() {
        assert_eq!(
            capture_dimensions((1707, 1067), 1.5 * SWITCH_RESOLUTION_SCALE),
            (640, 400)
        );
        assert_eq!(
            capture_dimensions((800, 480), 1.25 * SWITCH_RESOLUTION_SCALE),
            (250, 150)
        );
        assert_eq!(
            capture_dimensions((1600, 900), 0.75 * SWITCH_RESOLUTION_SCALE),
            (300, 169)
        );
    }

    #[test]
    fn taskbar_thumbnail_remains_bounded() {
        assert_eq!(thumbnail_dimensions((1707, 1067)), (288, 180));
        assert_eq!(thumbnail_dimensions((480, 800)), (108, 180));
        assert_eq!(thumbnail_dimensions((200, 100)), (200, 100));
    }

    #[test]
    fn refresh_cycle_is_distributed_across_windows() {
        assert_eq!(
            preview_slice_interval(Duration::from_secs(2), 3).as_millis(),
            666
        );
        assert_eq!(
            preview_slice_interval(Duration::from_millis(700), 3).as_millis(),
            233
        );
        assert_eq!(
            preview_slice_interval(Duration::from_secs(2), 0),
            Duration::from_secs(2)
        );
    }

    #[test]
    fn disabled_switch_animation_stops_background_capture() {
        assert_eq!(capture_policy(false, false), None);
        assert_eq!(capture_policy(false, true), Some(false));
        assert_eq!(capture_policy(true, false), Some(true));
    }

    #[test]
    fn menu_only_capture_is_bounded_to_thumbnail_pixels() {
        assert!((thumbnail_capture_scale((1707, 1067)) - 180.0 / 1067.0).abs() < 0.0001);
        assert_eq!(
            capture_dimensions((1707, 1067), thumbnail_capture_scale((1707, 1067))),
            (288, 180)
        );
    }
}
