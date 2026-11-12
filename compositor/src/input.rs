use smithay::{
    backend::input::{
        AbsolutePositionEvent, Axis, AxisSource, ButtonState, Event, InputBackend, InputEvent,
        KeyState, KeyboardKeyEvent, PointerAxisEvent, PointerButtonEvent, TouchEvent,
    },
    input::{
        keyboard::FilterResult,
        pointer::{
            AxisFrame, ButtonEvent, Focus, GrabStartData as PointerGrabStartData, MotionEvent,
        },
        touch::{DownEvent, MotionEvent as TouchMotionEvent, UpEvent},
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::SERIAL_COUNTER,
};
use win10x_protocol::{AppWindowMode, Panel};

use crate::{
    grabs::MoveSurfaceGrab,
    session_ipc::window_identity,
    state::{FloatingTouchDrag, Win10xState},
};

impl Win10xState {
    pub fn process_input_event<I: InputBackend>(&mut self, event: InputEvent<I>) {
        match event {
            InputEvent::Keyboard { event, .. } => {
                let serial = SERIAL_COUNTER.next_serial();
                let time = Event::time_msec(&event);
                let key_code = event.key_code();
                let state = event.state();

                // Compositor hotkeys (before client focus): Super opens Launcher; Tab+Super Task View.
                if state == KeyState::Pressed {
                    // keysym handling via xkb would be better; use raw codes for MVP demos:
                    // 125/133 often Super_L on PC keyboards via libinput keycodes — also accept Escape.
                }

                self.seat.get_keyboard().unwrap().input::<(), _>(
                    self,
                    key_code,
                    state,
                    serial,
                    time,
                    |comp, _, keysym| {
                        // When Fluent shell is connected it owns Super / Escape / panels.
                        if comp.has_shell_client() {
                            return FilterResult::Forward;
                        }
                        if state == KeyState::Pressed {
                            use smithay::input::keyboard::Keysym;
                            let sym = keysym.modified_sym();
                            if sym == Keysym::Escape {
                                comp.shell.panel = Panel::None;
                                return FilterResult::Intercept(());
                            }
                            if sym == Keysym::Super_L || sym == Keysym::Super_R {
                                comp.shell.panel = if comp.shell.panel == Panel::Launcher {
                                    Panel::None
                                } else {
                                    Panel::Launcher
                                };
                                return FilterResult::Intercept(());
                            }
                            if sym == Keysym::F3 {
                                comp.toggle_task_view();
                                return FilterResult::Intercept(());
                            }
                            if sym == Keysym::F4 {
                                comp.cycle_taskbar_size();
                                return FilterResult::Intercept(());
                            }
                        }
                        FilterResult::Forward
                    },
                );
            }
            InputEvent::PointerMotion { .. } => {}
            InputEvent::PointerMotionAbsolute { event, .. } => {
                let output = self.space.outputs().next().unwrap();
                let output_geo = self.space.output_geometry(output).unwrap();
                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();
                let serial = SERIAL_COUNTER.next_serial();
                let pointer = self.seat.get_pointer().unwrap();

                let _ = self.handle_shell_pointer_motion(pos);

                let under = self.surface_under(pos);
                pointer.motion(
                    self,
                    under,
                    &MotionEvent {
                        location: pos,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerButton { event, .. } => {
                let pointer = self.seat.get_pointer().unwrap();
                let keyboard = self.seat.get_keyboard().unwrap();
                let serial = SERIAL_COUNTER.next_serial();
                let button = event.button_code();
                let button_state = event.state();
                let pos = pointer.current_location();

                if ButtonState::Pressed == button_state && !pointer.is_grabbed() {
                    let output = self.space.outputs().next().unwrap();
                    let output_geo = self.space.output_geometry(output).unwrap();
                    if self.handle_shell_pointer_press(pos, output_geo.size.h) {
                        pointer.button(
                            self,
                            &ButtonEvent {
                                button,
                                state: button_state,
                                serial,
                                time: event.time_msec(),
                            },
                        );
                        pointer.frame(self);
                        return;
                    }

                    const BTN_LEFT: u32 = 0x110;
                    if button == BTN_LEFT {
                        if let Some((window, initial_window_location)) =
                            self.floating_drag_target(pos)
                        {
                            self.activate_application(window.clone());
                            pointer.set_grab(
                                self,
                                MoveSurfaceGrab {
                                    start_data: PointerGrabStartData {
                                        focus: None,
                                        button,
                                        location: pos,
                                    },
                                    window,
                                    initial_window_location,
                                },
                                serial,
                                Focus::Clear,
                            );
                            pointer.button(
                                self,
                                &ButtonEvent {
                                    button,
                                    state: button_state,
                                    serial,
                                    time: event.time_msec(),
                                },
                            );
                            pointer.frame(self);
                            return;
                        }
                    }

                    // Prefer shell surface when clicking chrome; otherwise focus app under cursor.
                    if let Some((surface, _)) = self.surface_under(pos) {
                        if let Some(shell) = self.shell_window() {
                            if shell.toplevel().is_some_and(|t| *t.wl_surface() == surface) {
                                keyboard.set_focus(self, Some(surface), serial);
                                pointer.button(
                                    self,
                                    &ButtonEvent {
                                        button,
                                        state: button_state,
                                        serial,
                                        time: event.time_msec(),
                                    },
                                );
                                pointer.frame(self);
                                return;
                            }
                        }
                    }

                    if let Some((window, _loc)) = self.application_window_under(pos) {
                        self.activate_application(window);
                    } else {
                        self.space.elements().for_each(|window| {
                            if !Self::is_shell_window(window) {
                                window.set_activated(false);
                                window.toplevel().unwrap().send_pending_configure();
                            }
                        });
                        keyboard.set_focus(self, Option::<WlSurface>::None, serial);
                    }
                }

                if ButtonState::Released == button_state {
                    self.handle_shell_pointer_release();
                }

                pointer.button(
                    self,
                    &ButtonEvent {
                        button,
                        state: button_state,
                        serial,
                        time: event.time_msec(),
                    },
                );
                pointer.frame(self);
            }
            InputEvent::PointerAxis { event, .. } => {
                let source = event.source();
                let horizontal_amount = event.amount(Axis::Horizontal).unwrap_or_else(|| {
                    event.amount_v120(Axis::Horizontal).unwrap_or(0.0) * 15.0 / 120.
                });
                let vertical_amount = event.amount(Axis::Vertical).unwrap_or_else(|| {
                    event.amount_v120(Axis::Vertical).unwrap_or(0.0) * 15.0 / 120.
                });
                let horizontal_amount_discrete = event.amount_v120(Axis::Horizontal);
                let vertical_amount_discrete = event.amount_v120(Axis::Vertical);

                let mut frame = AxisFrame::new(event.time_msec()).source(source);
                if horizontal_amount != 0.0 {
                    frame = frame.value(Axis::Horizontal, horizontal_amount);
                    if let Some(discrete) = horizontal_amount_discrete {
                        frame = frame.v120(Axis::Horizontal, discrete as i32);
                    }
                }
                if vertical_amount != 0.0 {
                    frame = frame.value(Axis::Vertical, vertical_amount);
                    if let Some(discrete) = vertical_amount_discrete {
                        frame = frame.v120(Axis::Vertical, discrete as i32);
                    }
                }
                if source == AxisSource::Finger {
                    if event.amount(Axis::Horizontal) == Some(0.0) {
                        frame = frame.stop(Axis::Horizontal);
                    }
                    if event.amount(Axis::Vertical) == Some(0.0) {
                        frame = frame.stop(Axis::Vertical);
                    }
                }

                let pointer = self.seat.get_pointer().unwrap();
                pointer.axis(self, frame);
                pointer.frame(self);
            }
            InputEvent::TouchDown { event, .. } => {
                let output = self.space.outputs().next().unwrap();
                let output_geo = self.space.output_geometry(output).unwrap();
                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();
                let serial = SERIAL_COUNTER.next_serial();

                if self.handle_shell_pointer_press(pos, output_geo.size.h) {
                    return;
                }

                if let Some((window, initial_location)) = self.floating_drag_target(pos) {
                    self.activate_application(window.clone());
                    self.floating_touch_drag = Some(FloatingTouchDrag {
                        slot: event.slot(),
                        window,
                        start: pos,
                        initial_location,
                    });
                    return;
                }

                let under = self.surface_under(pos);
                let touch = self.seat.get_touch().unwrap();
                touch.down(
                    self,
                    under,
                    &DownEvent {
                        slot: event.slot(),
                        location: pos,
                        serial,
                        time: event.time_msec(),
                    },
                );
                touch.frame(self);
            }
            InputEvent::TouchMotion { event, .. } => {
                let output = self.space.outputs().next().unwrap();
                let output_geo = self.space.output_geometry(output).unwrap();
                let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();
                let _ = self.handle_shell_pointer_motion(pos);
                if let Some(drag) = self
                    .floating_touch_drag
                    .as_ref()
                    .filter(|drag| drag.slot == event.slot())
                {
                    let location = drag.initial_location.to_f64() + (pos - drag.start);
                    let window = drag.window.clone();
                    self.space
                        .map_element(window, location.to_i32_round(), true);
                    if let Some(shell) = self.shell_window() {
                        self.space.raise_element(&shell, false);
                    }
                    return;
                }
                let under = self.surface_under(pos);
                let touch = self.seat.get_touch().unwrap();
                touch.motion(
                    self,
                    under,
                    &TouchMotionEvent {
                        slot: event.slot(),
                        location: pos,
                        time: event.time_msec(),
                    },
                );
                touch.frame(self);
            }
            InputEvent::TouchUp { event, .. } => {
                self.handle_shell_pointer_release();
                if self
                    .floating_touch_drag
                    .as_ref()
                    .is_some_and(|drag| drag.slot == event.slot())
                {
                    self.floating_touch_drag = None;
                    return;
                }
                let serial = SERIAL_COUNTER.next_serial();
                let touch = self.seat.get_touch().unwrap();
                touch.up(
                    self,
                    &UpEvent {
                        slot: event.slot(),
                        serial,
                        time: event.time_msec(),
                    },
                );
                touch.frame(self);
            }
            InputEvent::TouchCancel { event, .. } => {
                self.handle_shell_pointer_release();
                self.floating_touch_drag = None;
                let touch = self.seat.get_touch().unwrap();
                touch.cancel(self);
                let _ = event;
            }
            InputEvent::TouchFrame { .. } => {
                if let Some(touch) = self.seat.get_touch() {
                    touch.frame(self);
                }
            }
            _ => {}
        }
    }

    fn floating_drag_target(
        &self,
        pos: smithay::utils::Point<f64, smithay::utils::Logical>,
    ) -> Option<(
        smithay::desktop::Window,
        smithay::utils::Point<i32, smithay::utils::Logical>,
    )> {
        const DRAG_STRIP_HEIGHT: f64 = 36.0;
        let (window, location) = self.application_window_under(pos)?;
        let (app_id, _) = window_identity(&window)?;
        if self.app_window_modes.get(&app_id) != Some(&AppWindowMode::Floating) {
            return None;
        }
        let local_y = pos.y - f64::from(location.y);
        (0.0..=DRAG_STRIP_HEIGHT)
            .contains(&local_y)
            .then_some((window, location))
    }
}
