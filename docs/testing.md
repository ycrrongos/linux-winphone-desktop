# Testing (L1 / L2 / L3)

The Denial/Flutter migration has its own build and acceptance procedure in
[`denial-migration.md`](denial-migration.md). The L1 commands and completed
legacy MVP behavior below refer to Rust/egui. Denial's `denial-nested` executable
is a topology demonstration, not a replacement for a full Flutter desktop
session. Use `tools/win10x-denial preview` for UI inspection and a separate
DRM/KMS seat or VM for native compositor acceptance.

Matches the MVP plan: day-to-day work is **not** KVM-first.

## L1 — Nested (default)

On your existing desktop session:

```bash
# UI + Win10X panels (fastest iteration)
cargo run -p win10x-shell

# Nested compositor (smithay/winit window); spawns a terminal client if available
cargo run -p win10x-comp
cargo run -p win10x-comp -- -c alacritty
```

Checks:

- Centered taskbar; Start / Task View; tray opens Action Center (shell) / tray band (comp)
- Nested apps receive Wayland `Fullscreen`, fill the complete compositor output,
  and remain below the shell taskbar/panel overlay
- Click and type in the fullscreen app outside shell chrome; verify its real
  icon appears in both taskbar and Task View, then activate it from either icon
- With two apps open, use their taskbar icons in both directions: the old
  snapshot retreats and the target snapshot slides/scales over 500 ms while
  the taskbar stays visible; restore from Start/minimized state and verify the
  200 ms icon-rectangle-to-app zoom. Verify the shell uses its dedicated switch
  capture at 25% of the output width and height rather than the smaller taskbar
  thumbnail, and that periodic capture does not cause sustained compositor CPU
  use or visible input stalls. With three windows, verify file updates are
  distributed across the cycle, PNG work appears on the named background
  encoder thread, the render thread remains responsive, and RSS does not grow
- Taskbar swipe-up opens Launcher
- Launch apps → icon displacement / pinned vs running separator (shell)
- Touch targets ~44epx on Large taskbar

Mouse drag on the taskbar approximates swipe-up in L1; **gesture acceptance is L2/L3**.

## L2 — KVM / QEMU regression (x86_64)

Use before a release to validate a real seat without nested side effects.

Suggested QEMU shape (adjust disk/ISO to your guest):

```bash
qemu-system-x86_64 -enable-kvm -m 4096 -smp 4 \
  -device virtio-vga-gl -display gtk,gl=on \
  -usb -device usb-tablet \
  -drive file=win10x-dev.qcow2,if=virtio
```

Inside the guest: install release binaries from `packaging/out`, then either:

- `win10x-comp` from a TTY/`greetd` session (`packaging/win10x.desktop`), or
- run `win10x-shell` for UI-only checks.

Expect: session starts, at least one Wayland client runs, taskbar chrome responds.

## L3 — Touch + aarch64

1. Cross-build: `./packaging/build-release.sh aarch64`
2. Prefer a **real touchscreen** (phone/tablet/2-in-1) for swipe and target-size acceptance.
3. Without hardware: QEMU `aarch64` system emulation running the aarch64 binaries (slow; smoke-test only).

MVP bar for arm64: **binaries build** + optional smoke run.

## Automation

MVP relies on manual checklist review ([`interaction-checklist.md`](interaction-checklist.md)) plus `cargo check` / `cargo test -p win10x-protocol`. No UI record/replay suite yet.
用

## Mode acceptance (2026-11-11)

Use `preview --mode tablet|desktop|phone`; mode is persisted only in the isolated
preview configuration. See `research/windows10-modes/README.md` for versioned
reference behavior. Check tablet before desktop/phone. Useful dimensions:
1280×800 landscape, 800×1280 portrait, and 390×780 phone. Start has a real mode
menu; the tablet Action Center quick action must switch to desktop immediately.

For preview-local pointer checks use `WIN10X_PREVIEW_EXERCISE=all-apps`,
`tap:所有应用`, `tap:平板模式`, `hold:开始；长按单手模式`, or
`hold:返回；长按切换应用` with `--capture`. Real-client input and placement still
require a separate Denial seat/VM; no native window is fabricated by these tests.

Run `python3 tools/test_denial_bundle.py` when changing publication. Previews pin
an immutable generation, so rebuilding must not truncate a loaded AOT library
or change its lazy font assets. Keep only the current task's intended test
window running for review and stop it by its recorded PID when restarting.

### 2026-11-12 popup and interaction regression exercises

With the matching current bundle, opt-in `WIN10X_PREVIEW_EXERCISE` cases now
include `phone-menu`, `popup-lifecycle`, `dialog-overflow`, `toast-feedback`,
`start-toggle` and `panel-interrupt`. Set `WIN10X_PREVIEW_CAPTURE_MS=7000` or
longer for multi-step exercises. `popup-lifecycle` checks normal menu close,
Back cancellation, forced removal after mounting and removal before first build;
`panel-interrupt` checks that a stale settle cannot overwrite a new drag.
These actions operate inside the test application's Flutter tree only.

The pure `disco_projection_test.dart` compares animation geometry with the legacy
Rust projection at intermediate angles as well as the settled identity. It is
not an original-Windows animation timing certification. Existing model tests
now use classic/compact phone columns 4/6 and preserve stored freeform cells.

### Client proportions and launch ownership (2026-11-12)

`WIN10X_PREVIEW_EXERCISE=client-proportions WIN10X_PREVIEW_CAPTURE_MS=5000
 tools/win10x-denial preview --mode phone --capture /tmp/win10x-client-fit.png`
opens an opt-in synthetic source-buffer grid through the prepared native-client
presentation widget and retained positioning widget. It checks equal X/Y scale
and reciprocal coordinate transforms both in motion and settled. Inspect the
circle and grid; a square source cell must not become a rectangle. This fixture
is not an actual Wayland/XWayland client and does not prove native configure ACK,
minimum-size behavior or device input. Recheck those on the phone after reconnect.

The prepared window frame must have its upstream TrackedDesktopWindowReveal
initial animation disabled; our persisted launch style owns the handoff. Do not
mistake changing upstream easing constants for removing that second controller.

### Chromium device regression — user report, 2026-11-12

The user identifies **Chromium and similar applications** as affected by squeezed
or stretched content, and cannot reconnect the phone for now. This narrows the
report to native client presentation; its actual Wayland versus XWayland backend
and exact failure sequence remain unconfirmed. Do not mark Chromium fixed based
on the composition fixture, or ask the user to reconnect repeatedly.

After reconnection, first observe the installed Chromium backend without changing
its normal flags. Reproduce in a separate temporary browser profile using a local
page with square cells, circles and input targets; keep the user's real profile
and browsing state untouched. Compare first paint, settled layout, resize and
phone↔desktop transitions. Record requested geometry and committed client content
size separately, including fractional output scale. Confirm equal X/Y scale and
matching root/popup input coordinates. If needed, compare explicit Wayland and
XWayland runs as separate cases; do not silently force a backend or disable GPU
acceleration and call that the rendering fix. A constrained client fitting with
unused space is distinct from the client actually reflowing to the requested size.

### Phone task cards and fast alphabet index (2026-11-12)

Use the pinned preview's opt-in `WIN10X_PREVIEW_EXERCISE` cases:

- `alphabet-index`: a clearly synthetic #/A–Z catalog; tap M, slowly scrub to Z,
  reverse to A, open the real header alphabet picker, select M, then scrub back
  to Z. Assert shared exact offsets and no accidental application launch.
- `alphabet-picker`: stop with the actual phone jump grid open; inspect retained
  Back/Start/Search. Repeat light theme, large drawer text and landscape.
- `tasks-board`: synthetic IDs supplied only to the shared presentation widget,
  never the native bridge. Inspect 390×780 single cards, 446×992 two-row/two-column
  cards and 780×390 landscape. Preview text explicitly identifies a UI fixture.
- `tasks-dismiss` / `tasks-refused`: a slow downward drag must close exact ID11;
  if a backend does not remove it, the card returns and its close control works.
- `tasks-native-entry`: actual phone navigation long-Back opens the real task
  popup; its Start destination reveals tiles even when Start was already open.
  In an empty native preview this proves navigation only, not native activation.

Scope catalog semantic/scrollable lookups to the popup fixture: Start keeps its
other page alive, so the first whole-tree alphabet rail can be offscreen. Delete
old captures before starting each case. Keep synthetic fixtures separate from
real compositor window acceptance. On-device follow-up: actual screenshots,
per-window focus/close, unsaved-document prompt and popup/touch alignment; do not
substitute these UI fixtures for native acceptance while the phone is unavailable.
