//! Recognize and layout the `win10x-shell` Wayland client.

use smithay::{
    desktop::Window,
    reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
    utils::{Logical, Point, Rectangle},
    wayland::{compositor::with_states, shell::xdg::XdgToplevelSurfaceData},
};
use win10x_protocol::{Panel, SHELL_APP_ID, SHELL_TITLE_PREFIX};

use crate::Win10xState;

impl Win10xState {
    pub fn is_shell_window(window: &Window) -> bool {
        let Some(toplevel) = window.toplevel() else {
            return false;
        };
        with_states(toplevel.wl_surface(), |states| {
            let guard = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .and_then(|d| d.lock().ok());
            let Some(role) = guard else {
                return false;
            };
            if role.app_id.as_deref() == Some(SHELL_APP_ID) {
                return true;
            }
            role.title
                .as_deref()
                .is_some_and(|t| t.starts_with(SHELL_TITLE_PREFIX))
        })
    }

    pub fn shell_window(&self) -> Option<Window> {
        self.space
            .elements()
            .find(|w| Self::is_shell_window(w))
            .cloned()
    }

    pub fn has_shell_client(&self) -> bool {
        self.shell_window().is_some()
    }

    /// Parse `Win10X Shell|Launcher|s125` title into a panel for chrome placement.
    pub fn panel_from_shell_title(&self) -> Option<Panel> {
        let window = self.shell_window()?;
        let toplevel = window.toplevel()?;
        with_states(toplevel.wl_surface(), |states| {
            let guard = states
                .data_map
                .get::<XdgToplevelSurfaceData>()?
                .lock()
                .ok()?;
            let title = guard.title.as_deref()?;
            Some(win10x_protocol::parse_shell_title(title).0)
        })
    }

    pub fn effective_panel(&self) -> Panel {
        self.panel_from_shell_title().unwrap_or(self.shell.panel)
    }

    /// Full-output geometry for the shell overlay (includes taskbar band).
    pub fn layout_shell_window(&mut self, window: &Window) {
        let Some(output) = self.space.outputs().next().cloned() else {
            return;
        };
        let Some(geo) = self.space.output_geometry(&output) else {
            return;
        };
        window.toplevel().unwrap().with_pending_state(|state| {
            state.states.unset(xdg_toplevel::State::Maximized);
            state.states.set(xdg_toplevel::State::Fullscreen);
            state.size = Some(geo.size);
        });
        window.toplevel().unwrap().send_configure();
        // Topmost chrome and active application are separate concepts. Keep
        // the shell above apps without deactivating the foreground app.
        self.space.map_element(window.clone(), (0, 0), false);
        self.space.raise_element(window, false);
        tracing::info!(
            size = ?geo.size,
            "mapped win10x-shell fullscreen overlay"
        );
    }

    /// Whether a pointer/touch position should hit the shell chrome (taskbar / open panel).
    pub fn pos_in_shell_chrome(&self, pos: Point<f64, Logical>, output_h: i32) -> bool {
        let tb_top = (output_h - self.taskbar_height_i32()) as f64;
        if pos.y >= tb_top {
            return true;
        }
        let panel = self.effective_panel();
        if panel == Panel::None {
            return false;
        }
        let Some(output) = self.space.outputs().next() else {
            return false;
        };
        let Some(geo) = self.space.output_geometry(output) else {
            return false;
        };
        let Some(card) = panel_card_rect(panel, geo, self.taskbar_height_i32()) else {
            return false;
        };
        pos.x >= card.loc.x as f64
            && pos.y >= card.loc.y as f64
            && pos.x < (card.loc.x + card.size.w) as f64
            && pos.y < (card.loc.y + card.size.h) as f64
    }

    /// Apply niri-style output scale: clients redraw via fractional-scale (crisp text).
    pub fn set_output_scale(&mut self, scale: f64) {
        use smithay::{
            backend::renderer::damage::OutputDamageTracker, output::Scale,
            wayland::fractional_scale::with_fractional_scale,
        };
        use win10x_protocol::clamp_output_scale;

        let scale = clamp_output_scale(scale);
        if (self.applied_output_scale - scale).abs() < 0.001 {
            return;
        }
        self.applied_output_scale = scale;
        self.shell.output_scale = scale;

        let Some(output) = self.output.clone() else {
            return;
        };
        output.change_current_state(None, None, Some(Scale::Fractional(scale)), None);
        self.damage_tracker = Some(OutputDamageTracker::from_output(&output));

        for window in self.space.elements() {
            window.with_surfaces(|surface, states| {
                with_fractional_scale(states, |fs| {
                    fs.set_preferred_scale(scale);
                });
                let _ = surface;
            });
        }

        self.relayout_windows();
        tracing::info!(scale, "output scale updated (fractional-scale)");
    }

    /// Pull panel + scale from the shell xdg title each frame.
    pub fn sync_from_shell_title(&mut self) {
        let Some(window) = self.shell_window() else {
            return;
        };
        let Some(toplevel) = window.toplevel() else {
            return;
        };
        let parsed = with_states(toplevel.wl_surface(), |states| {
            let guard = states
                .data_map
                .get::<XdgToplevelSurfaceData>()?
                .lock()
                .ok()?;
            let title = guard.title.as_deref()?;
            Some(win10x_protocol::parse_shell_title(title))
        });
        let Some((panel, scale)) = parsed else {
            return;
        };
        self.shell.panel = panel;
        if let Some(s) = scale {
            self.set_output_scale(s);
        }
    }
}

/// Panel hit-test rect — keep in sync with `shell` `inset_card` / panel layout.
pub fn panel_card_rect(
    panel: Panel,
    output_geo: Rectangle<i32, Logical>,
    taskbar_h: i32,
) -> Option<Rectangle<i32, Logical>> {
    if panel == Panel::None {
        return None;
    }
    let tb_top = (output_geo.size.h - taskbar_h).max(0);
    let desktop_h = tb_top;
    let desktop_w = output_geo.size.w;
    match panel {
        // shell: fullscreen Start (Live Tiles) — entire desktop above taskbar
        Panel::Launcher | Panel::TaskbarAppMenu => {
            let w = desktop_w.max(120);
            let h = desktop_h.max(120);
            Some(Rectangle::new((0, 0).into(), (w, h).into()))
        }
        // shell: inset_card(desktop, 20, 16, 12, 16) for Task View
        Panel::TaskView => {
            let left = 16;
            let right = 16;
            let top = 20;
            let bottom = 12;
            let w = (desktop_w - left - right).max(120);
            let h = (desktop_h - top - bottom).max(120);
            Some(Rectangle::new((left, top).into(), (w, h).into()))
        }
        // shell: inset_card(desktop, 20, 20, 12, 20)
        Panel::DesktopSettings => {
            let left = 20;
            let right = 20;
            let top = 20;
            let bottom = 12;
            let w = (desktop_w - left - right).max(120);
            let h = (desktop_h - top - bottom).max(120);
            Some(Rectangle::new((left, top).into(), (w, h).into()))
        }
        // shell: width = min(360, desktop_w-24), right-aligned with 16px margin
        Panel::ActionCenter => {
            let w = 360.min(desktop_w - 24).max(200);
            let x = (desktop_w - w - 16).max(0);
            let top = 20;
            let bottom = 12;
            let h = (desktop_h - top - bottom).max(120);
            Some(Rectangle::new((x, top).into(), (w, h).into()))
        }
        Panel::None => None,
    }
}
