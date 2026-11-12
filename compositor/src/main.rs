#![allow(irrefutable_let_patterns)]

mod grabs;
mod handlers;
mod input;
mod session_ipc;
mod shell_chrome;
mod shell_window;
mod state;
mod wallpaper;
mod window_previews;
mod winit;

use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
pub use state::Win10xState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let mut event_loop: EventLoop<Win10xState> = EventLoop::try_new()?;
    let display: Display<Win10xState> = Display::new()?;
    let mut state = Win10xState::new(&mut event_loop, display);

    crate::winit::init_winit(&mut event_loop, &mut state)?;

    // Child clients must talk to this compositor, not the host Wayland/X11
    // session. Toolkits use XDG_SESSION_TYPE to select their Wayland backend.
    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);
        std::env::set_var("XDG_SESSION_TYPE", "wayland");
        std::env::remove_var("DISPLAY");
        std::env::set_var(
            win10x_protocol::SESSION_STATE_PATH_ENV,
            &state.session_state_path,
        );
        std::env::set_var(
            win10x_protocol::SESSION_COMMAND_PATH_ENV,
            &state.session_command_path,
        );
    }
    state.publish_session_state();

    tracing::info!(
        socket = ?state.socket_name,
        "win10x-comp nested session ready (L1). Swipe up from the taskbar band to open launcher overlay."
    );

    spawn_client();

    event_loop.run(None, &mut state, move |_| {})?;
    let _ = std::fs::remove_file(&state.session_state_path);
    let _ = std::fs::remove_file(&state.session_command_path);
    let _ = std::fs::remove_dir_all(&state.preview_dir);
    Ok(())
}

fn init_logging() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }
}

fn spawn_client() {
    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();

    match (flag.as_deref(), arg) {
        (Some("-c") | Some("--command"), Some(command)) => {
            let mut parts = command.split_whitespace();
            if let Some(bin) = parts.next() {
                let rest: Vec<_> = parts.collect();
                std::process::Command::new(bin).args(rest).spawn().ok();
            }
        }
        _ => {
            // Nested session: Fluent shell chrome + a demo application window.
            spawn_win10x_shell();
            // Delay terminal so dmabuf globals are visible and shell maps first.
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(400));
                for cand in ["alacritty", "weston-terminal", "foot", "kitty", "xterm"] {
                    if std::process::Command::new(cand).spawn().is_ok() {
                        break;
                    }
                }
            });
        }
    }
}

fn spawn_win10x_shell() {
    // Prefer the just-built binary next to this process / workspace target dir.
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("win10x-shell"));
        }
    }
    candidates.push(std::path::PathBuf::from("target/debug/win10x-shell"));
    candidates.push(std::path::PathBuf::from("target/release/win10x-shell"));

    for path in &candidates {
        if !path.is_file() {
            continue;
        }
        let mut cmd = std::process::Command::new(path);
        cmd.env("WIN10X_NESTED", "1");
        match cmd.spawn() {
            Ok(_) => {
                tracing::info!(?path, "spawned win10x-shell nested client");
                return;
            }
            Err(err) => tracing::warn!(?path, ?err, "failed to spawn shell"),
        }
    }

    // PATH fallback
    match std::process::Command::new("win10x-shell")
        .env("WIN10X_NESTED", "1")
        .spawn()
    {
        Ok(_) => tracing::info!("spawned win10x-shell from PATH"),
        Err(err) => tracing::warn!(
            ?err,
            "win10x-shell not found — run `cargo build -p win10x-shell` or pass `-c`"
        ),
    }
}
