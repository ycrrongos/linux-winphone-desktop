//! Shell chrome interaction hooks for the nested compositor session.
//! When `win10x-shell` is connected, the Fluent client owns chrome input;
//! compositor keeps panel hit-test state in sync via the shell title.

use smithay::utils::{Logical, Point};
use win10x_protocol::Panel;

use crate::state::Win10xState;

impl Win10xState {
    pub fn taskbar_height_i32(&self) -> i32 {
        self.shell.taskbar_size.height() as i32
    }

    pub fn handle_shell_pointer_press(&mut self, pos: Point<f64, Logical>, output_h: i32) -> bool {
        // Fluent shell client owns Start / tray / panels.
        if self.has_shell_client() {
            return false;
        }

        let tb_top = (output_h - self.taskbar_height_i32()) as f64;
        let out_w = self
            .space
            .outputs()
            .next()
            .and_then(|o| self.space.output_geometry(o).map(|g| g.size.w as f64));

        if pos.y >= tb_top {
            self.gesture_y0 = Some(pos.y);
            if let Some(out_w) = out_w {
                if pos.x > out_w - 96.0 {
                    self.shell.panel = if self.shell.panel == Panel::ActionCenter {
                        Panel::None
                    } else {
                        Panel::ActionCenter
                    };
                    self.gesture_y0 = None;
                    return true;
                }
                let cx = out_w / 2.0;
                if (pos.x - (cx - 30.0)).abs() < 28.0 {
                    self.shell.panel = if self.shell.panel == Panel::Launcher {
                        Panel::None
                    } else {
                        Panel::Launcher
                    };
                    self.gesture_y0 = None;
                    return true;
                }
                if (pos.x - (cx + 30.0)).abs() < 28.0 {
                    self.toggle_task_view();
                    self.gesture_y0 = None;
                    return true;
                }
            }
            return true;
        }

        if self.shell.panel != Panel::None {
            self.shell.panel = Panel::None;
            return true;
        }
        false
    }

    pub fn handle_shell_pointer_motion(&mut self, pos: Point<f64, Logical>) -> bool {
        if self.has_shell_client() {
            return false;
        }
        if let Some(y0) = self.gesture_y0 {
            if y0 - pos.y > 36.0 {
                self.shell.panel = Panel::Launcher;
                self.gesture_y0 = None;
                return true;
            }
            return true;
        }
        false
    }

    pub fn handle_shell_pointer_release(&mut self) {
        self.gesture_y0 = None;
    }

    pub fn cycle_taskbar_size(&mut self) {
        self.shell.taskbar_size = self.shell.taskbar_size.cycle();
        self.relayout_windows();
    }

    pub fn toggle_task_view(&mut self) {
        self.shell.panel = if self.shell.panel == Panel::TaskView {
            Panel::None
        } else {
            Panel::TaskView
        };
    }
}
