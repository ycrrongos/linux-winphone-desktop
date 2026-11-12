mod compositor;
mod xdg_shell;

use crate::Win10xState;

//
// Wl Seat
//

use smithay::input::dnd::{DnDGrab, DndGrabHandler, GrabType, Source};
use smithay::input::pointer::Focus;
use smithay::input::{Seat, SeatHandler, SeatState};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::Serial;
use smithay::wayland::output::OutputHandler;
use smithay::wayland::pointer_constraints::PointerConstraintsHandler;
use smithay::wayland::selection::data_device::{
    set_data_device_focus, DataDeviceHandler, DataDeviceState, WaylandDndGrabHandler,
};
use smithay::wayland::selection::SelectionHandler;

impl SeatHandler for Win10xState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Win10xState> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }

    fn focus_changed(&mut self, seat: &Seat<Self>, focused: Option<&WlSurface>) {
        let dh = &self.display_handle;
        let client = focused.and_then(|s| dh.get_client(s.id()).ok());
        set_data_device_focus(dh, seat, client);
    }
}

impl PointerConstraintsHandler for Win10xState {}

//
// Wl Data Device
//

impl SelectionHandler for Win10xState {
    type SelectionUserData = ();
}

impl DataDeviceHandler for Win10xState {
    fn data_device_state(&mut self) -> &mut DataDeviceState {
        &mut self.data_device_state
    }
}

impl DndGrabHandler for Win10xState {}
impl WaylandDndGrabHandler for Win10xState {
    fn dnd_requested<S: Source>(
        &mut self,
        source: S,
        _icon: Option<WlSurface>,
        seat: Seat<Self>,
        serial: Serial,
        type_: GrabType,
    ) {
        match type_ {
            GrabType::Pointer => {
                let ptr = seat.get_pointer().unwrap();
                let start_data = ptr.grab_start_data().unwrap();

                // create a dnd grab to start the operation
                let grab = DnDGrab::new_pointer(&self.display_handle, start_data, source, seat);
                ptr.set_grab(self, grab, serial, Focus::Keep);
            }
            GrabType::Touch => {
                // smallvil lacks touch handling
                source.cancel();
            }
        }
    }
}

//
// Wl Output & Xdg Output
//

impl OutputHandler for Win10xState {}

//
// wp_fractional_scale_v1 — clients redraw at integer physical pixels (niri-style crisp text)
//

use smithay::wayland::compositor::{get_parent, with_states};
use smithay::wayland::fractional_scale::{with_fractional_scale, FractionalScaleHandler};

impl FractionalScaleHandler for Win10xState {
    fn new_fractional_scale(&mut self, surface: WlSurface) {
        let mut root = surface.clone();
        while let Some(parent) = get_parent(&root) {
            root = parent;
        }
        let scale = self
            .output
            .as_ref()
            .map(|o| o.current_scale().fractional_scale())
            .unwrap_or(self.applied_output_scale);
        with_states(&surface, |states| {
            with_fractional_scale(states, |fs| {
                fs.set_preferred_scale(scale);
            });
        });
        let _ = root;
    }
}

//
// linux-dmabuf (required for Glow / Mesa EGL clients under the nested session)
//

use smithay::backend::{allocator::dmabuf::Dmabuf, renderer::ImportDma};
use smithay::wayland::dmabuf::{DmabufGlobal, DmabufHandler, DmabufState, ImportNotifier};

impl DmabufHandler for Win10xState {
    fn dmabuf_state(&mut self) -> &mut DmabufState {
        self.dmabuf_state
            .as_mut()
            .expect("dmabuf_state initialized in init_winit")
    }

    fn dmabuf_imported(
        &mut self,
        _global: &DmabufGlobal,
        dmabuf: Dmabuf,
        notifier: ImportNotifier,
    ) {
        let ok = self
            .backend
            .as_mut()
            .and_then(|b| b.renderer().import_dmabuf(&dmabuf, None).ok())
            .is_some();
        if ok {
            let _ = notifier.successful::<Win10xState>();
        } else {
            notifier.failed();
        }
    }
}

smithay::delegate_dispatch2!(Win10xState);
