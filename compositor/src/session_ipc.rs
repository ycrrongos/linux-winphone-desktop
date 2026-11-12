//! Small local runtime bridge between the compositor and shell processes.

use std::{fs, io, path::Path};

use smithay::{
    desktop::Window,
    utils::SERIAL_COUNTER,
    wayland::{compositor::with_states, shell::xdg::XdgToplevelSurfaceData},
};
use win10x_protocol::{AppWindowMode, CompositorSessionState, RunningApp, SessionCommand};

use crate::Win10xState;

impl Win10xState {
    pub fn publish_session_state(&self) {
        let active_window_id = self
            .space
            .elements()
            .rev()
            .filter(|window| !Self::is_shell_window(window))
            .filter(|window| {
                window_identity(window)
                    .is_some_and(|(app_id, _)| !self.minimized_apps.contains(&app_id))
            })
            .find(|window| window_identity(window).is_some())
            .map(Window::id);

        let apps = self
            .space
            .elements()
            .rev()
            .filter(|window| !Self::is_shell_window(window))
            .filter_map(|window| {
                let (app_id, title) = window_identity(window)?;
                let window_id = window.id() as u64;
                Some(RunningApp {
                    window_id,
                    active: active_window_id == Some(window.id())
                        && !self.minimized_apps.contains(&app_id),
                    minimized: self.minimized_apps.contains(&app_id),
                    mode: self
                        .app_window_modes
                        .get(&app_id)
                        .copied()
                        .unwrap_or_default(),
                    preview_path: Some(
                        self.preview_dir
                            .join(format!("window-{window_id}.png"))
                            .to_string_lossy()
                            .into_owned(),
                    ),
                    transition_preview_path: Some(
                        self.preview_dir
                            .join(format!("window-{window_id}-switch.png"))
                            .to_string_lossy()
                            .into_owned(),
                    ),
                    app_id,
                    title,
                })
            })
            .collect();

        let state = CompositorSessionState { apps };
        match serde_json::to_vec(&state) {
            Ok(bytes) => {
                if let Err(error) = atomic_write(&self.session_state_path, &bytes) {
                    tracing::warn!(?error, "failed to publish compositor session state");
                }
            }
            Err(error) => tracing::warn!(?error, "failed to encode compositor session state"),
        }
    }

    pub fn handle_session_command(&mut self) {
        let Ok(text) = fs::read_to_string(&self.session_command_path) else {
            return;
        };
        let _ = fs::remove_file(&self.session_command_path);
        let Ok(command) = serde_json::from_str::<SessionCommand>(&text) else {
            tracing::warn!("ignored malformed shell session command");
            return;
        };

        match command {
            SessionCommand::SetAppSwitchAnimationEnabled { enabled } => {
                self.app_switch_animation_enabled = enabled;
                self.last_preview_capture = None;
                self.preview_cursor = 0;
                crate::window_previews::set_switch_animation_enabled(enabled);
            }
            SessionCommand::ActivateApp { app_id } => {
                self.minimized_apps.remove(&app_id);
                self.relayout_windows();
                let window = self.app_window(&app_id);
                if let Some(window) = window {
                    self.activate_application(window);
                }
            }
            SessionCommand::ActivateWindow { window_id } => {
                let window = self
                    .space
                    .elements()
                    .find(|window| window.id() as u64 == window_id)
                    .cloned();
                if let Some(window) = window {
                    if let Some((app_id, _)) = window_identity(&window) {
                        self.minimized_apps.remove(&app_id);
                        self.relayout_windows();
                    }
                    self.activate_application(window);
                }
            }
            SessionCommand::MinimizeApp { app_id } => self.minimize_application(&app_id),
            SessionCommand::MaximizeApp { app_id } => {
                self.minimized_apps.remove(&app_id);
                self.app_window_modes
                    .insert(app_id.clone(), AppWindowMode::Fullscreen);
                self.relayout_windows();
                if let Some(window) = self.app_window(&app_id) {
                    self.activate_application(window);
                }
            }
            SessionCommand::CloseApp { app_id } => {
                let windows = self.app_windows(&app_id);
                for window in windows {
                    if let Some(toplevel) = window.toplevel() {
                        toplevel.send_close();
                    }
                }
            }
            SessionCommand::SplitApp { app_id } => self.split_application(&app_id),
            SessionCommand::FloatApp { app_id } => {
                self.minimized_apps.remove(&app_id);
                self.app_window_modes
                    .insert(app_id.clone(), AppWindowMode::Floating);
                self.relayout_windows();
                if let Some(window) = self.app_window(&app_id) {
                    self.activate_application(window);
                }
            }
        }
    }

    fn app_window(&self, app_id: &str) -> Option<Window> {
        self.space
            .elements()
            .rev()
            .find(|window| {
                !Self::is_shell_window(window)
                    && window_identity(window).is_some_and(|(candidate, _)| candidate == app_id)
            })
            .cloned()
    }

    fn app_windows(&self, app_id: &str) -> Vec<Window> {
        self.space
            .elements()
            .filter(|window| {
                !Self::is_shell_window(window)
                    && window_identity(window).is_some_and(|(candidate, _)| candidate == app_id)
            })
            .cloned()
            .collect()
    }

    fn minimize_application(&mut self, app_id: &str) {
        self.minimized_apps.insert(app_id.to_string());
        let next = self
            .space
            .elements()
            .rev()
            .find(|window| {
                !Self::is_shell_window(window)
                    && window_identity(window).is_some_and(|(candidate, _)| {
                        candidate != app_id && !self.minimized_apps.contains(&candidate)
                    })
            })
            .cloned();
        self.relayout_windows();
        if let Some(window) = next {
            self.activate_application(window);
        } else {
            let serial = SERIAL_COUNTER.next_serial();
            self.seat
                .get_keyboard()
                .unwrap()
                .set_focus(self, None, serial);
            self.publish_session_state();
        }
    }

    fn split_application(&mut self, app_id: &str) {
        let other_id = self
            .space
            .elements()
            .rev()
            .filter_map(window_identity)
            .map(|(candidate, _)| candidate)
            .find(|candidate| {
                candidate != app_id && !self.minimized_apps.contains(candidate.as_str())
            });
        self.minimized_apps.remove(app_id);
        self.app_window_modes
            .insert(app_id.to_string(), AppWindowMode::SplitLeft);
        if let Some(other_id) = other_id {
            self.app_window_modes
                .insert(other_id, AppWindowMode::SplitRight);
        }
        self.relayout_windows();
        if let Some(window) = self.app_window(app_id) {
            self.activate_application(window);
        }
    }

    pub fn activate_application(&mut self, window: Window) {
        self.space.raise_element(&window, true);
        if let Some(shell) = self.shell_window() {
            self.space.raise_element(&shell, false);
        }
        let surface = window.toplevel().unwrap().wl_surface().clone();
        let serial = SERIAL_COUNTER.next_serial();
        self.seat
            .get_keyboard()
            .unwrap()
            .set_focus(self, Some(surface), serial);
        self.space.elements().for_each(|window| {
            window.toplevel().unwrap().send_pending_configure();
        });
        self.publish_session_state();
    }
}

pub(crate) fn window_identity(window: &Window) -> Option<(String, String)> {
    let toplevel = window.toplevel()?;
    with_states(toplevel.wl_surface(), |states| {
        let role = states
            .data_map
            .get::<XdgToplevelSurfaceData>()?
            .lock()
            .ok()?;
        let app_id = role.app_id.as_deref()?.trim();
        if app_id.is_empty() {
            return None;
        }
        let title = role
            .title
            .as_deref()
            .map(str::trim)
            .filter(|title| !title.is_empty())
            .unwrap_or(app_id)
            .to_string();
        Some((app_id.to_string(), title))
    })
}

fn atomic_write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut temporary = path.to_path_buf();
    temporary.set_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)
}
