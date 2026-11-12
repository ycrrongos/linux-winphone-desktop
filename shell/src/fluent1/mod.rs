//! Fluent **1** (Windows 10 / Win10X-era) skin.
//!
//! Design tokens are ported from the local HTML library
//! [`/home/rong/Win10UI`](file:///home/rong/Win10UI) (`src/tokens/*.css`).
//!
//! Runtime widgets still come from **fluent-egui** (Fluent 2). This module
//! overrides egui visuals + shell chrome painting toward Fluent 1:
//! square corners, Win10 accent, opaque surfaces, reveal hover, Win10 motion.

mod motion;
mod reveal;
mod surfaces;
mod tokens;
mod visuals;

pub use motion::{ease_decelerate, ease_out_back, spring_step};
pub use reveal::RevealState;
pub use surfaces::{paint_strong_surface, paint_surface, paint_taskbar};
pub use tokens::{Fluent1Theme, Palette};
pub use visuals::{apply_to_ctx, apply_to_ctx_ex};
