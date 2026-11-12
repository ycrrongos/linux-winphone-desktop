use std::time::Duration;

use crate::wallpaper::DesktopWallpaper;
use crate::Win10xState;
use smithay::{
    backend::{
        egl::EGLDevice,
        renderer::{
            damage::OutputDamageTracker,
            element::{
                solid::{SolidColorBuffer, SolidColorRenderElement},
                AsRenderElements, Kind,
            },
            gles::GlesRenderer,
            Color32F, ImportDma, ImportEgl, ImportMemWl,
        },
        winit::{self, WinitEvent, WinitGraphicsBackend},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::EventLoop,
    utils::{Rectangle, Scale, Transform},
    wayland::dmabuf::{DmabufFeedbackBuilder, DmabufState},
};

smithay::backend::renderer::element::render_elements! {
    /// Desktop composition stack: shell → applications → wallpaper.
    pub DesktopElement<=GlesRenderer>;
    Shell=smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement<GlesRenderer>,
    Solid=smithay::backend::renderer::element::solid::SolidColorRenderElement,
    Wallpaper=smithay::backend::renderer::element::memory::MemoryRenderBufferRenderElement<GlesRenderer>,
}

pub fn init_winit(
    event_loop: &mut EventLoop<Win10xState>,
    state: &mut Win10xState,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut backend, winit) = winit::init::<GlesRenderer>()?;

    // linux-dmabuf + wl_drm so Glow/EGL clients (eframe shell, terminals) can render.
    init_client_graphics(state, &mut backend);

    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000,
    };

    let output = Output::new(
        "winit".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Win10X".into(),
            model: "Nested".into(),
            serial_number: "0".into(),
        },
    );
    let _global = output.create_global::<Win10xState>(&state.display_handle);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    state.space.map_output(&output, (0, 0));
    state.backend = Some(backend);
    state.damage_tracker = Some(OutputDamageTracker::from_output(&output));
    state.output = Some(output.clone());

    // Fallback chrome icons when Fluent shell is not connected.
    let mut start_buf = SolidColorBuffer::new((1, 1), Color32F::from([0.00, 0.47, 0.83, 1.0]));
    let mut view_buf = SolidColorBuffer::new((1, 1), Color32F::from([0.35, 0.55, 0.90, 1.0]));
    let mut tray_buf = SolidColorBuffer::new((1, 1), Color32F::from([1.0, 1.0, 1.0, 0.18]));
    let mut wallpaper = DesktopWallpaper::load();

    event_loop
        .handle()
        .insert_source(winit, move |event, _, state| {
            match event {
                WinitEvent::Resized { size, .. } => {
                    if let Some(output) = &state.output {
                        output.change_current_state(
                            Some(Mode {
                                size,
                                refresh: 60_000,
                            }),
                            None,
                            None,
                            None,
                        );
                    }
                    state.relayout_windows();
                }
                WinitEvent::Input(event) => state.process_input_event(event),
                WinitEvent::Redraw => {
                    state.handle_session_command();
                    state.sync_from_shell_title();
                    crate::window_previews::refresh_if_needed(state);

                    let Some(output) = state.output.clone() else {
                        return;
                    };
                    if state.backend.is_none() || state.damage_tracker.is_none() {
                        return;
                    }

                    let tb_h = state.taskbar_height_i32();
                    let icon = state.shell.taskbar_size.icon() as i32;
                    let shell_window = state.shell_window();
                    let shell_loc = shell_window
                        .as_ref()
                        .and_then(|w| state.space.element_location(w));
                    if let Some(ref w) = shell_window {
                        state.space.unmap_elem(w);
                    }

                    // First in list = topmost.
                    let mut elements: Vec<DesktopElement> = Vec::new();

                    let size = state.backend.as_ref().unwrap().window_size();
                    let damage = Rectangle::from_size(size);
                    let tb_top = size.h - tb_h;
                    let iy = tb_top + (tb_h - icon) / 2;
                    let cx = size.w / 2;
                    let clear = [0.35, 0.49, 0.63, 1.0]; // 10X sky fallback

                    {
                        let backend = state.backend.as_mut().unwrap();
                        let damage_tracker = state.damage_tracker.as_mut().unwrap();
                        let (renderer, mut framebuffer) = backend.bind().unwrap();

                        if let (Some(ref w), Some(loc)) = (&shell_window, shell_loc) {
                            let scale = Scale::from(1.0);
                            elements.extend(AsRenderElements::<GlesRenderer>::render_elements::<
                                DesktopElement,
                            >(
                                w,
                                renderer,
                                loc.to_physical_precise_round(scale),
                                scale,
                                1.0,
                            ));
                        } else {
                            start_buf.resize((icon, icon));
                            view_buf.resize((icon, icon));
                            tray_buf.resize((56, icon));
                            elements.push(DesktopElement::Solid(
                                SolidColorRenderElement::from_buffer(
                                    &start_buf,
                                    (cx - icon - 8, iy),
                                    1f64,
                                    1.0,
                                    Kind::Unspecified,
                                ),
                            ));
                            elements.push(DesktopElement::Solid(
                                SolidColorRenderElement::from_buffer(
                                    &view_buf,
                                    (cx + 8, iy),
                                    1f64,
                                    1.0,
                                    Kind::Unspecified,
                                ),
                            ));
                            elements.push(DesktopElement::Solid(
                                SolidColorRenderElement::from_buffer(
                                    &tray_buf,
                                    (size.w - 72, iy),
                                    1f64,
                                    1.0,
                                    Kind::Unspecified,
                                ),
                            ));
                        }

                        if let Some(output_geo) = state.space.output_geometry(&output) {
                            let scale = output.current_scale().fractional_scale();
                            elements.extend(
                                state
                                    .space
                                    .render_elements_for_region::<GlesRenderer, _>(
                                        renderer,
                                        &output_geo,
                                        scale,
                                        1.0,
                                    )
                                    .into_iter()
                                    .map(DesktopElement::Shell),
                            );
                        }
                        if let Some(wp) = wallpaper.element(renderer, size.to_logical(1)) {
                            elements.push(DesktopElement::Wallpaper(wp));
                        }

                        damage_tracker
                            .render_output(renderer, &mut framebuffer, 0, &elements, clear)
                            .unwrap();
                    }

                    let backend = state.backend.as_mut().unwrap();
                    backend.submit(Some(&[damage])).unwrap();
                    backend.window().request_redraw();

                    if let (Some(w), Some(loc)) = (shell_window, shell_loc) {
                        // Restore shell z-order after composition without
                        // stealing activation from the fullscreen app.
                        state.space.map_element(w, loc, false);
                    }

                    state.space.elements().for_each(|window| {
                        window.send_frame(
                            &output,
                            state.start_time.elapsed(),
                            Some(Duration::ZERO),
                            |_, _| Some(output.clone()),
                        )
                    });

                    state.space.refresh();
                    state.popups.cleanup();
                    let _ = state.display_handle.flush_clients();
                }
                WinitEvent::CloseRequested => {
                    state.loop_signal.stop();
                }
                _ => (),
            };
        })?;

    Ok(())
}

fn init_client_graphics(state: &mut Win10xState, backend: &mut WinitGraphicsBackend<GlesRenderer>) {
    let dmabuf_formats = backend.renderer().dmabuf_formats();
    let mut dmabuf_state = DmabufState::new();

    // Prefer dmabuf v4 feedback (works when nested EGL lacks EGL_WL_bind_wayland_display).
    let dmabuf_global =
        match EGLDevice::device_for_display(backend.renderer().egl_context().display())
            .and_then(|d| d.try_get_render_node())
        {
            Ok(Some(node)) => {
                match DmabufFeedbackBuilder::new(node.dev_id(), dmabuf_formats.clone()).build() {
                    Ok(feedback) => {
                        tracing::info!(?node, "linux-dmabuf v4 feedback ready");
                        dmabuf_state.create_global_with_default_feedback::<Win10xState>(
                            &state.display_handle,
                            &feedback,
                        )
                    }
                    Err(err) => {
                        tracing::warn!(?err, "dmabuf v4 feedback failed; using v3");
                        dmabuf_state
                            .create_global::<Win10xState>(&state.display_handle, dmabuf_formats)
                    }
                }
            }
            Ok(None) => {
                tracing::warn!("no DRM render node; linux-dmabuf v3");
                dmabuf_state.create_global::<Win10xState>(&state.display_handle, dmabuf_formats)
            }
            Err(err) => {
                tracing::warn!(?err, "EGL device query failed; linux-dmabuf v3");
                dmabuf_state.create_global::<Win10xState>(&state.display_handle, dmabuf_formats)
            }
        };

    state.dmabuf_state = Some(dmabuf_state);
    state._dmabuf_global = Some(dmabuf_global);

    // Optional on nested hosts; Mesa often lacks EGL_WL_bind_wayland_display here.
    match backend.renderer().bind_wl_display(&state.display_handle) {
        Ok(()) => tracing::info!("EGL wl_drm bound — hardware clients enabled"),
        Err(err) => tracing::warn!(?err, "bind_wl_display unavailable (ok if dmabuf v4 works)"),
    }

    state
        .shm_state
        .update_formats(backend.renderer().shm_formats());
}
