//! Desktop wallpaper under app windows (compositor clear replacement).
//!
//! Unused shell pixels stay transparent so this layer and app windows show through.

use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{
                memory::{MemoryRenderBuffer, MemoryRenderBufferRenderElement},
                Kind,
            },
            gles::GlesRenderer,
        },
    },
    utils::{Logical, Point, Size, Transform},
};

/// Soft solid sky matching the light 10X reference screenshot (RGB 90,126,160).
const DEFAULT_SKY: [u8; 3] = [0x5a, 0x7e, 0xa0];

pub struct DesktopWallpaper {
    /// Source RGBA8 (Abgr8888 byte order for GLES).
    src_w: u32,
    src_h: u32,
    src_rgba: Vec<u8>,
    buffer: Option<MemoryRenderBuffer>,
    last_out: Size<i32, Logical>,
}

impl DesktopWallpaper {
    pub fn load() -> Self {
        // Prefer photo if WIN10X_WALLPAPER is set; otherwise use the 10X sky solid.
        if let Ok(path) = std::env::var("WIN10X_WALLPAPER") {
            match load_rgba(&path) {
                Ok((w, h, rgba)) => {
                    tracing::info!(%path, w, h, "desktop wallpaper loaded");
                    return Self {
                        src_w: w,
                        src_h: h,
                        src_rgba: rgba,
                        buffer: None,
                        last_out: Size::default(),
                    };
                }
                Err(err) => tracing::warn!(%path, %err, "WIN10X_WALLPAPER load failed"),
            }
        }
        tracing::info!("desktop wallpaper: 10X sky solid");
        let (w, h, rgba) = solid_rgba(64, 64, DEFAULT_SKY);
        Self {
            src_w: w,
            src_h: h,
            src_rgba: rgba,
            buffer: None,
            last_out: Size::default(),
        }
    }

    pub fn element(
        &mut self,
        renderer: &mut GlesRenderer,
        out: Size<i32, Logical>,
    ) -> Option<MemoryRenderBufferRenderElement<GlesRenderer>> {
        if out.w <= 0 || out.h <= 0 {
            return None;
        }
        if self.buffer.is_none() || self.last_out != out {
            self.buffer = Some(raster_cover(&self.src_rgba, self.src_w, self.src_h, out));
            self.last_out = out;
        }
        let buffer = self.buffer.as_ref()?;
        MemoryRenderBufferRenderElement::from_buffer(
            renderer,
            Point::from((0.0, 0.0)),
            buffer,
            None,
            None,
            Some(out),
            Kind::Unspecified,
        )
        .ok()
    }
}

fn load_rgba(path: &str) -> Result<(u32, u32, Vec<u8>), String> {
    let img = image::open(path).map_err(|e| e.to_string())?;
    let img = img.thumbnail(1920, 1920);
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Ok((w, h, rgba.into_raw()))
}

fn solid_rgba(w: u32, h: u32, rgb: [u8; 3]) -> (u32, u32, Vec<u8>) {
    let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    for _ in 0..(w * h) {
        rgba.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
    }
    (w, h, rgba)
}

/// CSS object-fit: cover into `out` logical pixels (Abgr8888 / RGBA8 bytes).
fn raster_cover(src: &[u8], sw: u32, sh: u32, out: Size<i32, Logical>) -> MemoryRenderBuffer {
    let dw = out.w.max(1) as u32;
    let dh = out.h.max(1) as u32;
    let mut dst = vec![0u8; (dw * dh * 4) as usize];

    if sw == 0 || sh == 0 || src.len() < (sw * sh * 4) as usize {
        return MemoryRenderBuffer::from_slice(
            &dst,
            Fourcc::Abgr8888,
            (dw as i32, dh as i32),
            1,
            Transform::Normal,
            Some(vec![smithay::utils::Rectangle::from_size(
                (dw as i32, dh as i32).into(),
            )]),
        );
    }

    let scale = (dw as f32 / sw as f32).max(dh as f32 / sh as f32);
    let cw = (dw as f32 / scale).min(sw as f32);
    let ch = (dh as f32 / scale).min(sh as f32);
    let x0 = ((sw as f32 - cw) * 0.5).max(0.0);
    let y0 = ((sh as f32 - ch) * 0.5).max(0.0);

    for y in 0..dh {
        let v = (y as f32 + 0.5) / dh as f32;
        let sy = (y0 + v * ch).min(sh as f32 - 1.0);
        let iy = sy.floor() as u32;
        for x in 0..dw {
            let u = (x as f32 + 0.5) / dw as f32;
            let sx = (x0 + u * cw).min(sw as f32 - 1.0);
            let ix = sx.floor() as u32;
            let si = ((iy * sw + ix) * 4) as usize;
            let di = ((y * dw + x) * 4) as usize;
            dst[di..di + 4].copy_from_slice(&src[si..si + 4]);
        }
    }

    MemoryRenderBuffer::from_slice(
        &dst,
        Fourcc::Abgr8888,
        (dw as i32, dh as i32),
        1,
        Transform::Normal,
        Some(vec![smithay::utils::Rectangle::from_size(
            (dw as i32, dh as i32).into(),
        )]),
    )
}
