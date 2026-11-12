use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    path::PathBuf,
    sync::Arc,
};

use smithay::{
    backend::{
        input::TouchSlot,
        renderer::{damage::OutputDamageTracker, gles::GlesRenderer},
        winit::WinitGraphicsBackend,
    },
    desktop::{PopupManager, Space, Window, WindowSurfaceType},
    input::{Seat, SeatState},
    output::Output,
    reexports::{
        calloop::{generic::Generic, EventLoop, Interest, LoopSignal, Mode, PostAction},
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::wl_surface::WlSurface,
            Display, DisplayHandle,
        },
    },
    utils::{Logical, Point},
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        dmabuf::{DmabufGlobal, DmabufState},
        fractional_scale::FractionalScaleManagerState,
        output::OutputManagerState,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
        socket::ListeningSocketSource,
        viewporter::ViewporterState,
    },
};
use win10x_protocol::{AppWindowMode, ShellSnapshot};

pub struct Win10xState {
    pub start_time: std::time::Instant,
    pub socket_name: OsString,
    pub session_state_path: PathBuf,
    pub session_command_path: PathBuf,
    pub preview_dir: PathBuf,
    pub last_preview_capture: Option<std::time::Instant>,
    pub preview_cursor: usize,
    pub app_switch_animation_enabled: bool,
    pub minimized_apps: HashSet<String>,
    pub app_window_modes: HashMap<String, AppWindowMode>,
    pub display_handle: DisplayHandle,

    pub space: Space<Window>,
    pub loop_signal: LoopSignal,

    // Smithay State
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Win10xState>,
    pub data_device_state: DataDeviceState,
    pub popups: PopupManager,
    pub fractional_scale_manager_state: FractionalScaleManagerState,
    pub viewporter_state: ViewporterState,

    pub seat: Seat<Self>,

    /// Nested winit GLES backend (set in `init_winit`).
    pub backend: Option<WinitGraphicsBackend<GlesRenderer>>,
    pub damage_tracker: Option<OutputDamageTracker>,
    pub output: Option<Output>,
    /// linux-dmabuf for EGL/Glow clients (shell, terminals).
    pub dmabuf_state: Option<DmabufState>,
    pub _dmabuf_global: Option<DmabufGlobal>,

    /// Shared Win10X shell snapshot (panels, taskbar size, demo apps).
    pub shell: ShellSnapshot,
    /// Taskbar swipe-up gesture anchor (logical Y).
    pub gesture_y0: Option<f64>,
    /// Last applied output scale (avoids redundant configures).
    pub applied_output_scale: f64,
    /// Touch-owned drag for the compositor's title-bar-free floating mode.
    pub floating_touch_drag: Option<FloatingTouchDrag>,
}

pub struct FloatingTouchDrag {
    pub slot: TouchSlot,
    pub window: Window,
    pub start: Point<f64, Logical>,
    pub initial_location: Point<i32, Logical>,
}

impl Win10xState {
    pub fn new(event_loop: &mut EventLoop<Self>, display: Display<Self>) -> Self {
        let start_time = std::time::Instant::now();

        let dh = display.handle();

        // Here we initialize implementations of some wayland protocols
        // Some of them require us to implement traits on the Win10xState state,
        // you can find those implementations in the `crate::handlers` module

        // Initialize protocols needed for displaying windows
        let compositor_state = CompositorState::new::<Self>(&dh);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);
        let shm_state = ShmState::new::<Self>(&dh, vec![]);
        let popups = PopupManager::default();
        let fractional_scale_manager_state = FractionalScaleManagerState::new::<Self>(&dh);
        let viewporter_state = ViewporterState::new::<Self>(&dh);

        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);

        // Data device is responsible for clipboard and drag-and-drop
        let data_device_state = DataDeviceState::new::<Self>(&dh);

        // A seat is a group of keyboards, pointer and touch devices.
        // A seat typically has a pointer and maintains a keyboard focus and a pointer focus.
        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, "winit");

        // Notify clients that we have a keyboard, for the sake of the example we assume that keyboard is always present.
        // You may want to track keyboard hot-plug in real compositor.
        seat.add_keyboard(Default::default(), 200, 25).unwrap();

        // Notify clients that we have a pointer (mouse)
        // Here we assume that there is always pointer plugged in
        seat.add_pointer();
        // Touch path for phones / touchscreen PCs (hover must not be required).
        seat.add_touch();

        // A space represents a two-dimensional plane. Windows and Outputs can be mapped onto it.
        //
        // Windows get a position and stacking order through mapping.
        // Outputs become views of a part of the Space and can be rendered via Space::render_output.
        let space = Space::default();

        // Setup a wayland socket that will be used to accept clients
        let socket_name = Self::init_wayland_listener(display, event_loop);
        let runtime_dir = std::env::var_os("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let socket_tag = socket_name.to_string_lossy();
        let session_state_path = runtime_dir.join(format!("win10x-{socket_tag}-state.json"));
        let session_command_path = runtime_dir.join(format!("win10x-{socket_tag}-command.json"));
        let preview_dir = runtime_dir.join(format!("win10x-{socket_tag}-previews"));
        let _ = std::fs::remove_dir_all(&preview_dir);
        if let Err(error) = std::fs::create_dir_all(&preview_dir) {
            tracing::warn!(?error, "failed to create window preview directory");
        }

        // Get the loop signal, used to stop the event loop
        let loop_signal = event_loop.get_signal();

        Self {
            start_time,
            display_handle: dh,

            space,
            loop_signal,
            socket_name,
            session_state_path,
            session_command_path,
            preview_dir,
            last_preview_capture: None,
            preview_cursor: 0,
            app_switch_animation_enabled: true,
            minimized_apps: HashSet::new(),
            app_window_modes: HashMap::new(),

            compositor_state,
            xdg_shell_state,
            shm_state,
            output_manager_state,
            seat_state,
            data_device_state,
            popups,
            fractional_scale_manager_state,
            viewporter_state,
            seat,
            backend: None,
            damage_tracker: None,
            output: None,
            dmabuf_state: None,
            _dmabuf_global: None,
            shell: ShellSnapshot::default(),
            gesture_y0: None,
            applied_output_scale: 1.0,
            floating_touch_drag: None,
        }
    }

    fn init_wayland_listener(
        display: Display<Win10xState>,
        event_loop: &mut EventLoop<Self>,
    ) -> OsString {
        // Creates a new listening socket, automatically choosing the next available `wayland` socket name.
        let listening_socket = ListeningSocketSource::new_auto().unwrap();

        // Get the name of the listening socket.
        // Clients will connect to this socket.
        let socket_name = listening_socket.socket_name().to_os_string();

        let loop_handle = event_loop.handle();

        loop_handle
            .insert_source(listening_socket, move |client_stream, _, state| {
                // Inside the callback, you should insert the client into the display.
                //
                // You may also associate some data with the client when inserting the client.
                state
                    .display_handle
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .unwrap();
            })
            .expect("Failed to init the wayland event source.");

        // You also need to add the display itself to the event loop, so that client events will be processed by wayland-server.
        loop_handle
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |_, display, state| {
                    // Safety: we don't drop the display
                    unsafe {
                        display.get_mut().dispatch_clients(state).unwrap();
                    }
                    Ok(PostAction::Continue)
                },
            )
            .unwrap();

        socket_name
    }

    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<f64, Logical>)> {
        let output_h = self
            .space
            .outputs()
            .next()
            .and_then(|o| self.space.output_geometry(o).map(|g| g.size.h))
            .unwrap_or(0);

        // Fullscreen shell would steal every click; only hit it on chrome rects.
        if let Some(shell) = self.shell_window() {
            if self.pos_in_shell_chrome(pos, output_h) {
                if let Some(location) = self.space.element_location(&shell) {
                    if let Some((s, p)) =
                        shell.surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    {
                        return Some((s, (p + location).to_f64()));
                    }
                }
            }
        }

        self.application_window_under(pos)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(s, p)| (s, (p + location).to_f64()))
            })
    }

    /// Find the topmost application beneath a point while deliberately
    /// skipping the full-output transparent shell window.
    pub fn application_window_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(Window, Point<i32, Logical>)> {
        self.space.elements().rev().find_map(|window| {
            if Self::is_shell_window(window) {
                return None;
            }
            let location = self.space.element_location(window)?;
            window
                .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                .map(|_| (window.clone(), location))
        })
    }
}

/// Data associated with a wayland client that connects to Win10xState.
/// One instance of this type per client.
#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {}
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
