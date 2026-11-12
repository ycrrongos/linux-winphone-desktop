//! Motion curves from `/home/rong/Win10UI/src/tokens/motion.css`.
//!
//! CSS: `cubic-bezier(0.1, 0.9, 0.2, 1)` (standard / decelerate)
//!      `cubic-bezier(0.9, 0.1, 1, 0.2)` (accelerate)
#![allow(dead_code)] // keep full Win10UI motion set available

/// Durations (ms) from Win10UI.
#[derive(Debug, Clone, Copy)]
pub enum DurationMs {
    Fast = 100,
    Normal = 167,
    Slow = 250,
    Slower = 333,
}

impl DurationMs {
    pub fn secs(self) -> f32 {
        (self as u32 as f32) / 1000.0
    }
}

/// Approximate CSS cubic-bezier(0.1, 0.9, 0.2, 1) with a smoothstep-ish decelerate.
pub fn ease_decelerate(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    // Closest simple match to Fluent 1 decelerate: 1-(1-t)^3 with slight bias.
    1.0 - (1.0 - t).powf(2.6)
}

pub fn ease_accelerate(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t.powf(2.6)
}

pub fn ease_out_back(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

pub fn spring_step(current: f32, target: f32, dt: f32, stiffness: f32) -> f32 {
    let k = stiffness.clamp(4.0, 40.0);
    let next = current + (target - current) * (1.0 - (-k * dt).exp());
    if (next - target).abs() < 0.001 {
        target
    } else {
        next
    }
}
