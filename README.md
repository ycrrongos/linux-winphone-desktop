# Linux WinPhone Desktop

由 **ycrrongos 使用 Codex 开发**。键盘输入法独立仓库：
[winphone-pinyin-keyboard](https://github.com/ycrrongos/linux-pinyin-keybord-wp)。
本仓库保留可构建的固定版本键盘集成，来源见 `third_party/winphone-pinyin/UPSTREAM.json`。


Touch-first Linux shell with independent **Windows 10 tablet**, **Windows 10
 desktop** and **Windows 10 Mobile** modes. The tablet mode is the current
implementation priority. Windows 10X and the legacy Rust shell remain behavior
references; see [source research](research/windows10-modes/README.md).

## Denial / Flutter migration (in progress)

The new implementation uses **Denial 0.6.2 + its embedded Flutter engine**.
Windows 10/10 Mobile appearance, Start/Live Tiles and touch interaction remain
the product direction; the selected shell mode is independent from saved Start styles. `flutter_shell/` supplies our UI; `denial/main.dart` composes
it with Denial's reference desktop so native window layouts, rendering, focus,
workspaces and services stay with Denial.

```bash
tools/win10x-denial bootstrap       # checksum-verified, user-local tools; no system install
tools/win10x-denial analyze
tools/win10x-denial test
tools/win10x-denial bundle
tools/win10x-denial preview --demo --mode tablet --start # UI preview, not a native session
# Alternative previews: --mode desktop or --mode phone
```

Choose **模式与个性化** in Start (phone: the All Apps page's `…` button),
or expand Action Center and choose **切换模式**. The tablet quick action
switches directly between tablet and desktop; all three choices are persisted
without rewriting the existing tile board. `--mode` affects isolated preview
settings only. Phone mode supplies Back/Start/Search, hold-Back task switching,
hold-Start reachability and a top-down dark Action Center.

The default binary-kit workflow targets Linux x86_64. The matching ARM64 kit
can build and stage the same shell with `WIN10X_ARM64_HOST=qemu
tools/win10x-denial-arm64 bundle` followed by `tools/win10x-denial-arm64 package`
(requires qemu-user and an ARM64 glibc sysroot), or use an authenticated ARM64
SSH build host. Current-source device/session acceptance remains separate from
build/package success. A UI preview does not test application containment or prove stability. See
[`docs/denial-migration.md`](docs/denial-migration.md) for the pinned source,
reproduction steps, acceptance gaps and upstream patches.

The Rust/egui implementation described below remains available as the behavior
reference and rollback until the new desktop passes acceptance. Existing checked
items describe that implementation, not automatic acceptance of the Flutter port.

Phone installation and KDE/Breeze login configuration are recorded in
[`docs/phone-session.md`](docs/phone-session.md). MarkUI assets and exact source
provenance live under `flutter_shell/assets/wp_icons/`; shell icon coverage does
not restyle native application toolbars.

## Legacy Rust/egui status (MVP)

| Piece | Crate / path | Role |
|-------|----------------|------|
| Shared types | `crates/win10x-protocol` | Panels, taskbar sizes, apps, notifications |
| Shell UI | `shell` → `win10x-shell` | **Fluent 1** skin (tokens from `/home/rong/Win10UI`) on **fluent-egui** Fluent 2 widgets |
| Compositor | `compositor` → `win10x-comp` | Nested smithay Wayland session + chrome overlays + touch/swipe hooks |
| Research | `research/windows10x/`, [`research/windows-phone-launchers.md`](research/windows-phone-launchers.md) | Win10X facts and reusable Windows Phone launcher/animation references |
| Checklist | `docs/interaction-checklist.md` | Acceptance |

**MVP includes:** core shell surfaces and compositor substrate.  
**Not in MVP (full product later):** lock screen / login, OOBE, real NetworkManager backends, soft keyboard, file manager, Settings app, installable OS image. See checklist items marked 「完整版」.

## Quick start (L1 nested)

```bash
# Build both, then run the nested session (spawns shell + a terminal)
cargo build -p win10x-shell -p win10x-comp
cargo run -p win10x-comp

# Standalone Fluent shell (host Wayland/X11)
cargo run -p win10x-shell
```

Nested stack: transparent full-output `win10x-shell` chrome → fullscreen app
surface → wallpaper. The shell paints the taskbar/panels above the app while
passing ordinary desktop-area input through to the active app.
The compositor publishes its real xdg-toplevel list to the shell's local
runtime bridge, so taskbar and Task View represent the nested session rather
than a separate demo-only list. Taskbar app icons launch, minimize, restore, and
switch their real windows with a snapshot handoff; right-click or swipe upward
opens commands for close, pinning, a new window, split screen, and floating mode.
Full-screen handoffs use a separate capture at 25% of the output width and
height, while compact taskbar preview cards keep bounded thumbnails instead of
sharing the same image. Windows are captured round-robin, and thumbnail scaling,
PNG compression, and disk writes run on a bounded background encoder so they do
not stall compositor rendering or accumulate an unbounded image queue. The
persisted application-switch animation toggle disables that background GPU
readback and encoding completely; when the animation is off, only opening a
multi-window taskbar menu requests bounded thumbnails on demand.
Floating windows can be
dragged from their compositor-owned top strip even when the app has no titlebar.
When one app owns multiple windows, the same menu shows a live preview card for
each real xdg-toplevel; selecting a card raises and focuses that exact window.
Shell syncs the open panel via window title (`Win10X Shell|Launcher`) for hit testing.  
Without a shell client, compositor keeps solid Start/Task View placeholders (`Super` / swipe).

## UI stack (Fluent 1 on Fluent 2)

- **Widgets / runtime:** vendored [`third_party/fluent-egui`](third_party/fluent-egui) (Fluent **2** base; **all rectangular radii forced to 0** for Win10 look).
- **Look & tokens:** `shell/src/fluent1/` from **`/home/rong/Win10UI`** (Fluent **1**: square chrome, `#0078d4` / dark `#60cdff`, Reveal, Win10 motion).
- **Surfaces:** Taskbar and ordinary panels use opaque flat fills. Start uses a
  simple wallpaper crossfade with a semi-transparent white veil. No backdrop
  sampling, blur shader, wallpaper-blur fallback, acrylic tint stack, or noise
  layer is active.
- **Motion:** Start entrance and application-launch styles are selected and
  persisted independently. The optional Windows 8 handoff makes the Start
  scene recede while the tapped tile flies forward, turns in 3D, and expands
  across the output; application loading still begins in the click frame.
- **Start layouts:** Windows 10 keeps the application list beside a vertically
  scrollable tile board; Windows Phone uses a vertically scrollable tiles-only
  page with a 340 ms strongly decelerating `easeOutExpo` push into the
  application list. Windows 8 follows `flutter_metro_ui`: a `#1d1d1d` page,
  resizable 120 px normal / 250 px wide tiles, 10 px gaps, 100 px left and
  140 px top padding, column-first packing with at most six rows, and horizontal
  scrolling. Tile groups and their click-to-edit names are shared and persisted
  by all three layouts. Windows 8 tile dragging use the same pending-grab,
  floating follow, and live reorder preview as the Windows 10/Phone board:
  siblings ease out of the way while the source list is reordered (never
  ignored freeform cells), the nearest group always accepts the drop, and the
  tile settles into its derived Metro slot on release.
  In Windows 8, every real group independently runs the
  source 80% scale / +100 px / `easeOutExpo` entrance, with successive groups
  starting 120 ms apart; the delayed header remains fixed to that layout. Its
  project-added application list uses the same 340 ms nonlinear vertical page
  push. The selected layout is persisted.
- **Colors:** Win10-style 48-color grid, recent colors, HSV custom mixer, contrast preview; accent-from-wallpaper via [`color-thief`](https://github.com/RazrFalcon/color-thief-rs) (port of [lokesh/color-thief](https://github.com/lokesh/color-thief)).
- Circles (toggle knob, radio, slider thumb) stay circular; panels/buttons/cards are square.

## Build

```bash
cargo build -p win10x-shell -p win10x-comp
./packaging/build-release.sh            # host arch
./packaging/build-release.sh aarch64    # cross (needs rustup target)
```

## Testing

See [`docs/testing.md`](docs/testing.md) (L1 nested / L2 KVM / L3 device·QEMU aarch64).

## Agent rules

Codex uses [`AGENTS.md`](AGENTS.md); Qoder keeps its separate rules under
`.qoder/rules/agent.md`. On blockers, append to
[`docs/lessons-learned.md`](docs/lessons-learned.md); record every completed
feature or repository change in
[`docs/development-log.md`](docs/development-log.md).

## License / assets

Project-authored code is licensed under
[GNU GPL v3.0 only](LICENSE) (`GPL-3.0-only`). Vendored dependencies retain
their own licenses; in particular, `third_party/fluent-egui` remains available
under its upstream MIT/Apache-2.0 terms.

Downloaded research pages, screenshots, product imagery, and Microsoft marks
under `research/` are reference material and are **not** covered by the GPL
grant. Do not redistribute or use them as product assets.

## Keyboard editing and full computer mode


The WP screen keyboard adds fixed Edit/Computer/keypad entries, space-swipe
cursor movement, selection/word movement and copy/cut/paste/select-all/undo/redo.
Computer mode includes F1–F12, Fn, Ctrl/Alt/Shift/Win/AltGr and navigation; it
retains Chinese candidates for ordinary letters. Modifier chords are balanced
and the keyboard controls preserve editor focus. See
[interaction and acceptance](docs/keyboard-editing-and-pc.md) and
[native x64/ARM64 build recipes](denial/native/README.md). Packaging requires the
patched native broker rather than silently shipping unsupported function keys.
