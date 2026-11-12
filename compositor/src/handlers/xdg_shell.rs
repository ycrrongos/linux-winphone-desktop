use smithay::{
    desktop::{
        find_popup_root_surface, get_popup_toplevel_coords, PopupKind, PopupManager, Space, Window,
    },
    input::{
        pointer::{Focus, GrabStartData as PointerGrabStartData},
        Seat,
    },
    reexports::{
        wayland_protocols::xdg::shell::server::xdg_toplevel,
        wayland_server::{
            protocol::{wl_seat, wl_surface::WlSurface},
            Resource,
        },
    },
    utils::{Logical, Rectangle, Serial, Size},
    wayland::{
        compositor::with_states,
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelState, ToplevelSurface, XdgShellHandler,
            XdgShellState, XdgToplevelSurfaceData,
        },
    },
};

use crate::{
    grabs::{MoveSurfaceGrab, ResizeSurfaceGrab},
    session_ipc::window_identity,
    Win10xState,
};
use win10x_protocol::AppWindowMode;

impl XdgShellHandler for Win10xState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        let window = Window::new_wayland_window(surface);
        self.space.map_element(window.clone(), (0, 0), true);
        self.relayout_windows();

        // A phone/tablet shell has one foreground application. Give a newly
        // created toplevel keyboard focus immediately while its process keeps
        // cold-starting behind the launch animation.
        let serial = smithay::utils::SERIAL_COUNTER.next_serial();
        let wl_surface = window.toplevel().unwrap().wl_surface().clone();
        self.seat
            .get_keyboard()
            .unwrap()
            .set_focus(self, Some(wl_surface), serial);
        self.publish_session_state();
    }

    fn app_id_changed(&mut self, surface: ToplevelSurface) {
        // Shell sets app_id after the first configure; re-layout so it goes fullscreen.
        let window = self
            .space
            .elements()
            .find(|w| {
                w.toplevel()
                    .is_some_and(|t| t.wl_surface() == surface.wl_surface())
            })
            .cloned();
        if let Some(window) = window {
            if Self::is_shell_window(&window) {
                self.layout_shell_window(&window);
            } else if let Some((app_id, _)) = window_identity(&window) {
                self.minimized_apps.remove(&app_id);
                self.app_window_modes
                    .insert(app_id, AppWindowMode::Fullscreen);
                self.relayout_windows();
            }
        }
        self.publish_session_state();
    }

    fn title_changed(&mut self, _surface: ToplevelSurface) {
        self.publish_session_state();
    }

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {
        let window = self
            .space
            .elements()
            .find(|window| {
                window
                    .toplevel()
                    .is_some_and(|top| top.wl_surface() == surface.wl_surface())
            })
            .cloned();
        let removed_identity = window.as_ref().and_then(window_identity);
        if let Some(window) = window {
            self.space.unmap_elem(&window);
        }
        if let Some((app_id, _)) = removed_identity {
            let app_still_exists = self.space.elements().any(|window| {
                window_identity(window).is_some_and(|(candidate, _)| candidate == app_id)
            });
            if !app_still_exists {
                let removed_mode = self.app_window_modes.remove(&app_id);
                self.minimized_apps.remove(&app_id);
                if matches!(
                    removed_mode,
                    Some(AppWindowMode::SplitLeft | AppWindowMode::SplitRight)
                ) {
                    for mode in self.app_window_modes.values_mut() {
                        if matches!(mode, AppWindowMode::SplitLeft | AppWindowMode::SplitRight) {
                            *mode = AppWindowMode::Fullscreen;
                        }
                    }
                    self.relayout_windows();
                }
            }
        }
        self.publish_session_state();
    }

    fn new_popup(&mut self, surface: PopupSurface, _positioner: PositionerState) {
        self.unconstrain_popup(&surface);
        let _ = self.popups.track_popup(PopupKind::Xdg(surface));
    }

    fn reposition_request(
        &mut self,
        surface: PopupSurface,
        positioner: PositionerState,
        token: u32,
    ) {
        surface.with_pending_state(|state| {
            let geometry = positioner.get_geometry();
            state.geometry = geometry;
            state.positioner = positioner;
        });
        self.unconstrain_popup(&surface);
        surface.send_repositioned(token);
    }

    fn move_request(&mut self, surface: ToplevelSurface, seat: wl_seat::WlSeat, serial: Serial) {
        let seat = Seat::from_resource(&seat).unwrap();

        let wl_surface = surface.wl_surface();

        if let Some(start_data) = check_grab(&seat, wl_surface, serial) {
            let pointer = seat.get_pointer().unwrap();

            let window = self
                .space
                .elements()
                .find(|w| w.toplevel().unwrap().wl_surface() == wl_surface)
                .unwrap()
                .clone();
            if Self::is_shell_window(&window) {
                return;
            }
            let initial_window_location = self.space.element_location(&window).unwrap();

            let grab = MoveSurfaceGrab {
                start_data,
                window,
                initial_window_location,
            };

            pointer.set_grab(self, grab, serial, Focus::Clear);
        }
    }

    fn resize_request(
        &mut self,
        surface: ToplevelSurface,
        seat: wl_seat::WlSeat,
        serial: Serial,
        edges: xdg_toplevel::ResizeEdge,
    ) {
        let seat = Seat::from_resource(&seat).unwrap();

        let wl_surface = surface.wl_surface();

        if let Some(start_data) = check_grab(&seat, wl_surface, serial) {
            let pointer = seat.get_pointer().unwrap();

            let window = self
                .space
                .elements()
                .find(|w| w.toplevel().unwrap().wl_surface() == wl_surface)
                .unwrap()
                .clone();
            if Self::is_shell_window(&window) {
                return;
            }
            let initial_window_location = self.space.element_location(&window).unwrap();
            let initial_window_size = window.geometry().size;

            surface.with_pending_state(|state| {
                state.states.set(xdg_toplevel::State::Resizing);
            });

            surface.send_pending_configure();

            let grab = ResizeSurfaceGrab::start(
                start_data,
                window,
                edges.into(),
                Rectangle::new(initial_window_location, initial_window_size),
            );

            pointer.set_grab(self, grab, serial, Focus::Clear);
        }
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: wl_seat::WlSeat, _serial: Serial) {
        // TODO popup grabs
    }
}

fn check_grab(
    seat: &Seat<Win10xState>,
    surface: &WlSurface,
    serial: Serial,
) -> Option<PointerGrabStartData<Win10xState>> {
    let pointer = seat.get_pointer()?;

    // Check that this surface has a click grab.
    if !pointer.has_grab(serial) {
        return None;
    }

    let start_data = pointer.grab_start_data()?;

    let (focus, _) = start_data.focus.as_ref()?;
    // If the focus was for a different surface, ignore the request.
    if !focus.id().same_client_as(&surface.id()) {
        return None;
    }

    Some(start_data)
}

/// Should be called on `WlSurface::commit`
pub fn handle_commit(popups: &mut PopupManager, space: &Space<Window>, surface: &WlSurface) {
    // Handle toplevel commits.
    if let Some(window) = space
        .elements()
        .find(|w| w.toplevel().unwrap().wl_surface() == surface)
        .cloned()
    {
        let initial_configure_sent = with_states(surface, |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        });

        if !initial_configure_sent {
            window.toplevel().unwrap().send_configure();
        }
    }

    // Handle popup commits.
    popups.commit(surface);
    if let Some(popup) = popups.find_popup(surface) {
        match popup {
            PopupKind::Xdg(ref xdg) => {
                if !xdg.is_initial_configure_sent() {
                    // NOTE: This should never fail as the initial configure is always
                    // allowed.
                    xdg.send_configure().expect("initial configure failed");
                }
            }
            PopupKind::InputMethod(ref _input_method) => {}
        }
    }
}

impl Win10xState {
    /// Place application surfaces according to compositor-owned session modes.
    pub fn relayout_windows(&mut self) {
        let Some(output) = self.space.outputs().next().cloned() else {
            return;
        };
        let Some(geo) = self.space.output_geometry(&output) else {
            return;
        };
        let windows: Vec<_> = self.space.elements().cloned().collect();
        for window in windows {
            if Self::is_shell_window(&window) {
                self.layout_shell_window(&window);
                continue;
            }
            let toplevel = window.toplevel().unwrap();
            let app_id = window_identity(&window).map(|(app_id, _)| app_id);
            let minimized = app_id
                .as_ref()
                .is_some_and(|app_id| self.minimized_apps.contains(app_id));
            let mode = app_id
                .as_ref()
                .and_then(|app_id| self.app_window_modes.get(app_id))
                .copied()
                .unwrap_or_default();
            let (location, size) = application_geometry(mode, geo);
            let fullscreen_output = (mode == AppWindowMode::Fullscreen).then(|| {
                self.display_handle
                    .get_client(toplevel.wl_surface().id())
                    .ok()
                    .and_then(|client| output.client_outputs(&client).next())
            });
            toplevel.with_pending_state(|state| {
                configure_app_mode(state, mode, size, geo.size);
                state.fullscreen_output = fullscreen_output.flatten();
            });
            toplevel.send_configure();
            let location = if minimized {
                (geo.loc.x + geo.size.w + 64, geo.loc.y).into()
            } else {
                location
            };
            self.space.map_element(window, location, false);
        }

        // Keep shell above app windows for hit-testing / raise order.
        if let Some(shell) = self.shell_window() {
            self.space.raise_element(&shell, false);
        }
    }

    fn unconstrain_popup(&self, popup: &PopupSurface) {
        let Ok(root) = find_popup_root_surface(&PopupKind::Xdg(popup.clone())) else {
            return;
        };
        let Some(window) = self
            .space
            .elements()
            .find(|w| w.toplevel().unwrap().wl_surface() == &root)
        else {
            return;
        };

        let output = self.space.outputs().next().unwrap();
        let output_geo = self.space.output_geometry(output).unwrap();
        let window_geo = self.space.element_geometry(window).unwrap();

        // The target geometry for the positioner should be relative to its parent's geometry, so
        // we will compute that here.
        let mut target = output_geo;
        target.loc -= get_popup_toplevel_coords(&PopupKind::Xdg(popup.clone()));
        target.loc -= window_geo.loc;

        popup.with_pending_state(|state| {
            state.geometry = state.positioner.get_unconstrained_geometry(target);
        });
    }
}

fn configure_app_mode(
    state: &mut ToplevelState,
    mode: AppWindowMode,
    size: Size<i32, Logical>,
    output_size: Size<i32, Logical>,
) {
    state.states.unset(xdg_toplevel::State::Maximized);
    state.states.unset(xdg_toplevel::State::Fullscreen);
    if mode == AppWindowMode::Fullscreen {
        state.states.set(xdg_toplevel::State::Fullscreen);
    }
    state.size = Some(size);
    state.bounds = Some(output_size);
}

fn application_geometry(
    mode: AppWindowMode,
    output: Rectangle<i32, Logical>,
) -> (smithay::utils::Point<i32, Logical>, Size<i32, Logical>) {
    let left_width = output.size.w / 2;
    match mode {
        AppWindowMode::Fullscreen => (output.loc, output.size),
        AppWindowMode::SplitLeft => (output.loc, (left_width, output.size.h).into()),
        AppWindowMode::SplitRight => (
            (output.loc.x + left_width, output.loc.y).into(),
            (output.size.w - left_width, output.size.h).into(),
        ),
        AppWindowMode::Floating => {
            let width = (output.size.w * 72 / 100).max(1).min(output.size.w.max(1));
            let height = (output.size.h * 72 / 100).max(1).min(output.size.h.max(1));
            (
                (
                    output.loc.x + (output.size.w - width) / 2,
                    output.loc.y + (output.size.h - height) / 2,
                )
                    .into(),
                (width, height).into(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_layout_uses_the_complete_output_as_fullscreen() {
        let mut state = ToplevelState::default();
        state.states.set(xdg_toplevel::State::Maximized);

        let output_size = Size::from((1080, 1920));
        configure_app_mode(
            &mut state,
            AppWindowMode::Fullscreen,
            output_size,
            output_size,
        );

        assert_eq!(state.size, Some(output_size));
        assert_eq!(state.bounds, Some(output_size));
        assert!(state.states.contains(xdg_toplevel::State::Fullscreen));
        assert!(!state.states.contains(xdg_toplevel::State::Maximized));
    }

    #[test]
    fn split_and_floating_layouts_stay_inside_the_output() {
        let output = Rectangle::new((20, 30).into(), (1000, 800).into());
        assert_eq!(
            application_geometry(AppWindowMode::SplitLeft, output),
            ((20, 30).into(), (500, 800).into())
        );
        assert_eq!(
            application_geometry(AppWindowMode::SplitRight, output),
            ((520, 30).into(), (500, 800).into())
        );
        assert_eq!(
            application_geometry(AppWindowMode::Floating, output),
            ((160, 142).into(), (720, 576).into())
        );
    }
}
