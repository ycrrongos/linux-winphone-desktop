//! Discover and launch FreeDesktop applications.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

use win10x_protocol::AppEntry;

const PREFERRED_PIN: &[&str] = &[
    "brave-browser",
    "firefox",
    "chromium",
    "google-chrome",
    "org.gnome.Nautilus",
    "org.kde.dolphin",
    "thunar",
    "Alacritty",
    "foot",
    "kitty",
    "org.gnome.Terminal",
    "code",
    "code-oss",
    "org.telegram.desktop",
    "discord",
    "spotify",
    "org.gnome.Settings",
    "gnome-control-center",
    "systemsettings",
];

#[derive(Debug, Clone)]
struct DesktopApp {
    id: String,
    name: String,
    exec: String,
    desktop_id: String,
    icon: Option<String>,
    no_display: bool,
}

/// Scan standard application directories and build a shell app list.
pub fn discover_apps() -> Vec<AppEntry> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];
    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }
    if let Ok(xdg) = std::env::var("XDG_DATA_DIRS") {
        for part in xdg.split(':').filter(|s| !s.is_empty()) {
            dirs.push(PathBuf::from(part).join("applications"));
        }
    }

    let mut found: Vec<DesktopApp> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for dir in dirs {
        if !dir.is_dir() {
            continue;
        }
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }
            if let Some(app) = parse_desktop(&path) {
                if app.no_display || app.exec.is_empty() || app.name.is_empty() {
                    continue;
                }
                if seen.insert(app.desktop_id.clone()) {
                    found.push(app);
                }
            }
        }
    }

    found.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut apps: Vec<AppEntry> = found
        .into_iter()
        .map(|a| {
            let pin = PREFERRED_PIN.iter().any(|p| {
                a.desktop_id == *p || a.desktop_id.trim_end_matches(".desktop") == *p || a.id == *p
            });
            AppEntry {
                id: a.id,
                name: a.name,
                pinned: pin,
                running: false,
                desktop_id: Some(a.desktop_id),
                exec: Some(a.exec),
                icon: a.icon,
            }
        })
        .collect();

    // Ensure we always have at least a few pins for the taskbar.
    let pinned = apps.iter().filter(|a| a.pinned).count();
    if pinned < 4 {
        for app in apps.iter_mut().take(6) {
            app.pinned = true;
        }
    }

    if apps.is_empty() {
        tracing::warn!("no .desktop apps found; using protocol defaults");
        return win10x_protocol::ShellSnapshot::default().apps;
    }

    apps
}

fn parse_desktop(path: &Path) -> Option<DesktopApp> {
    let text = fs::read_to_string(path).ok()?;
    let mut in_desktop = false;
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut only_show = None::<String>;

    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_desktop = line == "[Desktop Entry]";
            continue;
        }
        if !in_desktop || line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            match k {
                "Name" if name.is_none() => name = Some(v.to_string()),
                "Name[en]" | "Name[en_US]" => name = Some(v.to_string()),
                "Exec" if exec.is_none() => exec = Some(v.to_string()),
                "Icon" if icon.is_none() => icon = Some(v.to_string()),
                "NoDisplay" => no_display = v.eq_ignore_ascii_case("true"),
                "Hidden" => hidden = v.eq_ignore_ascii_case("true"),
                "OnlyShowIn" => only_show = Some(v.to_string()),
                "Type" if v != "Application" => return None,
                _ => {}
            }
        }
    }

    if hidden || no_display {
        return None;
    }
    // Skip DE-specific entries that won't run outside that session.
    if let Some(ref only) = only_show {
        if !only.is_empty()
            && !only.to_uppercase().contains("GNOME")
            && !only.to_uppercase().contains("KDE")
            && !only.to_uppercase().contains("XFCE")
            && !only.to_uppercase().contains("LXQT")
        {
            // keep anyway for MVP — user may still launch
        }
    }

    let file_stem = path.file_stem()?.to_string_lossy().to_string();
    let desktop_id = path.file_name()?.to_string_lossy().to_string();
    let name = name?;
    let exec = exec?;

    Some(DesktopApp {
        id: file_stem.clone(),
        name,
        exec,
        desktop_id,
        icon,
        no_display: false,
    })
}

/// Launch an app for real. Nested sessions prefer the desktop file's `Exec`
/// so the process inherits `win10x-comp`'s Wayland socket directly.
pub fn launch(app: &AppEntry) -> Result<(), String> {
    let nested = std::env::var_os("WIN10X_NESTED").is_some();
    let mut nested_exec_error = None;

    if nested {
        if let Some(exec) = app.exec.as_deref() {
            match launch_exec(exec) {
                Ok(()) => return Ok(()),
                Err(error) => {
                    tracing::warn!(%error, "nested direct Exec failed, trying gtk-launch");
                    nested_exec_error = Some(error);
                }
            }
        }
    }

    if let Some(desktop_id) = app.desktop_id.as_deref() {
        let id = desktop_id.trim_end_matches(".desktop");
        match application_command("gtk-launch").arg(id).spawn() {
            Ok(_) => {
                tracing::info!(%id, "launched via gtk-launch");
                return Ok(());
            }
            Err(e) => tracing::warn!(error=%e, "gtk-launch failed, trying Exec"),
        }
        // Some systems want the full desktop file name.
        if application_command("gtk-launch")
            .arg(desktop_id)
            .spawn()
            .is_ok()
        {
            return Ok(());
        }
    }

    if !nested {
        if let Some(exec) = app.exec.as_deref() {
            return launch_exec(exec);
        }
    }

    Err(nested_exec_error.unwrap_or_else(|| format!("no launch method for {}", app.name)))
}

fn launch_exec(exec: &str) -> Result<(), String> {
    let argv = expand_exec(exec);
    if argv.is_empty() {
        return Err("empty Exec".into());
    }
    let mut cmd = application_command(&argv[0]);
    if argv.len() > 1 {
        cmd.args(&argv[1..]);
    }
    cmd.spawn()
        .map(|_| {
            tracing::info!(bin=%argv[0], "launched via Exec");
        })
        .map_err(|e| e.to_string())
}

/// Keep an application out of the shell/test PTY foreground process group.
/// Closing or rebuilding the shell must not send Ctrl+C to launched apps.
fn application_command(program: &str) -> Command {
    let mut command = Command::new(program);
    #[cfg(unix)]
    command.process_group(0);
    command
}

/// Strip FreeDesktop field codes (`%u`, `%f`, …) and split argv.
fn expand_exec(exec: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut chars = exec.chars().peekable();
    let mut in_quote: Option<char> = None;

    while let Some(c) = chars.next() {
        match (c, in_quote) {
            ('\\', Some('"')) => {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            }
            (q @ '"' | q @ '\'', None) => in_quote = Some(q),
            (q, Some(oq)) if q == oq => in_quote = None,
            ('%', None) => {
                // skip field code
                let _ = chars.next();
            }
            (' ', None) | ('\t', None) => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out.retain(|s| !s.is_empty());
    out
}

#[cfg(test)]
mod tests {
    use super::expand_exec;

    #[test]
    fn strips_field_codes() {
        assert_eq!(
            expand_exec("brave-browser %U"),
            vec!["brave-browser".to_string()]
        );
        assert_eq!(
            expand_exec("env FOO=1 alacritty -e htop"),
            vec![
                "env".to_string(),
                "FOO=1".to_string(),
                "alacritty".to_string(),
                "-e".to_string(),
                "htop".to_string(),
            ]
        );
    }
}
