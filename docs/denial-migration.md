# Denial / Flutter migration

Decision: 2026-11-10. The user selected Denial's embedded Flutter architecture,
while retaining the Win10X look, full-screen Start/Live Tiles and touch behavior.
This is an implementation in progress, not a completed replacement release.

## Reproducible baseline

`denial/upstream.lock.json` pins Denial **0.6.2** at
`3f08a93ec42b9851ebdd73039def1ff12a639858`, Flutter **3.47.5** and Dart **3.13.4**.
The exact upstream source is available at
[the pinned commit](https://github.com/denialwm/denial/tree/3f08a93ec42b9851ebdd73039def1ff12a639858).
Use its [architecture](https://github.com/denialwm/denial/blob/3f08a93ec42b9851ebdd73039def1ff12a639858/docs/architecture.md),
[custom-shell](https://github.com/denialwm/denial/blob/3f08a93ec42b9851ebdd73039def1ff12a639858/docs/CUSTOM_SHELLS.md)
and [plugin](https://github.com/denialwm/denial/blob/3f08a93ec42b9851ebdd73039def1ff12a639858/docs/PLUGIN_DEVELOPMENT.md)
contracts rather than documentation from a moving main branch.

`tools/win10x-denial bootstrap` downloads official x86_64 packages and Dart into
`$XDG_CACHE_HOME/win10x-denial/0.6.2` (normally `~/.cache/win10x-denial/0.6.2`).
It verifies the SHA-256 values recorded in the lock. No root access, package
installation, software-source configuration or display-manager change occurs.
`WIN10X_DENIAL_CACHE` overrides the cache's parent directory.

The writable kit, generated application and outputs live under `target/denial/`.
The kit's declared files are verified when copying it for the first time.
Official kit/source identity remains in the immutable downloaded cache. Our
patches apply only to the prepared project-local copy, with exact context checks.
Do not present the resulting bundle as an unmodified upstream artifact.

```bash
tools/win10x-denial bootstrap
tools/win10x-denial analyze
tools/win10x-denial test
tools/win10x-denial bundle
tools/win10x-denial preview --start
```

When all Pub packages are cached, set `WIN10X_DENIAL_OFFLINE=1` for offline
resolution. The project and generated application have separate committed
dependency locks. `denial/application.pubspec.lock` belongs to the generated
`target/denial/app` location; `prepare` copies it there for fresh builds.

Preview rejects a bundle whose recorded source digest differs from this
worktree. Rebuild after code changes. Its compositor-provided host window border
belongs to the current desktop; it is not a Win10X design element.

## Ownership and retained implementation

Denial owns native Wayland resources, window tiling, workspaces, focus, input,
live window textures, rendering and native services. We reuse the reference
desktop's composition/input publisher so rendering a window does not become a
second competing window manager. The app uses `ReferenceDesktop` explicitly;
it does not select the upstream top bar, clock, pet or launcher UI plugins.

`flutter_shell` contributes the bottom taskbar, Start presentation and named
Start action through public SDK contracts. Application launch uses catalog IDs
and the native-backed launcher. Saved Exec lines are never executed. Taskbar
activation identifies real windows; preview cards consume live window textures,
not the legacy PNG polling bridge.

`shell/`, `compositor/`, `crates/win10x-protocol/` and their Cargo workflow remain
available for comparison and rollback. The migration does not delete or rewrite
the legacy board, shell preferences or IPC. The new application does not use
that file-based IPC.

The release kit currently targets Linux x86_64. Denial's ARM64 support does not
make this project's ARM64 artifact verified: provide and pin the corresponding
runtime/compiler/engine packages, then build and smoke-test them before claiming
that target. Do not use x86_64 engine binaries on ARM64.

## Deliberate upstream adaptation

The public `ShellLauncher` controls content, while Denial's reference desktop
controls placement, transitions and input routing. Consequently a launcher
plugin alone is insufficient for our full-screen Start. `prepare` applies these
small patches to the pinned reference plugin:

1. Shared mode geometry drives drawing and native hit testing: full-output
   Start for tablet/phone, bounded lower-left Start for desktop; a right-side
   Action Center for PC modes and a full-width shade for phone.
2. The launcher enters in place, rather than sliding a bounded popup onto the
   output. Its transition bypasses `ShellBackdropBlur`; our background is the
   wallpaper plus a white alpha veil.
3. Native desktop-app startup begins in the click frame. Start remains mounted
   during our handoff and dismisses when the handoff completes. In-bundle local
   Flutter apps currently retain Denial's immediate host handoff.
4. A narrowly scoped window-motion notifier is exported by our prepared
   reference plugin. The application synchronizes it with the imported switch
   preference; it updates window-frame/scene motion and reduced-motion handling
   without changing Start's independent animation settings. Direct live textures
   replace the legacy PNG capture queue entirely.

The taskbar is in `aboveWindows` and remains present over fullscreen apps.
No taskbar work-area reservation is selected. This does not yet force newly
created stacking windows to acquire the previous compositor's full-output
`Fullscreen` state; that remains an explicit acceptance gap below.

Do not import upstream private SDK implementation libraries into our widgets.
Use `ShellPopupController` for menus and transient surfaces: Denial intentionally
does not provide a Navigator. Reuse the existing native bridge and providers.

## Settings and previews

The new shell writes `$XDG_CONFIG_HOME/win10x/flutter-shell.json`, defaulting to
`~/.config/win10x/flutter-shell.json`. On first use it reads the previous shell's
`~/.config/win10x/shell.json` and `start_tiles.json` without modifying them.
It retains group IDs, titles, cells, source order, dimensions, taskbar pins and
the independent Start/launch/switch animation preferences. Missing applications
are retained. Malformed or future-version settings block writes and surface an
error instead of silently replacing the document. Writes are serialized and
atomically renamed.

An ordinary preview uses `target/denial/preview-config` for its new configuration.
An explicitly seeded demo is opt-in and contains application IDs only:

```bash
tools/win10x-denial preview --demo --start
tools/win10x-denial preview --demo --start --layout windows_phone --width 390 --height 780
tools/win10x-denial preview --demo --start --layout windows8 --width 1100 --height 760
tools/win10x-denial preview --panel action-center
tools/win10x-denial preview --start --capture target/denial/qa/start.png
```

`--demo` keeps existing preview settings unless `--layout` explicitly requests a
new demo layout. These example IDs may be unavailable on another machine and
will show the neutral icon. No demo applications are installed by this command.

`--capture` is an opt-in one-shot Flutter repaint-boundary capture after the
preview settles. It captures only this shell, without host decorations or the
currently focused host application. Prefer it when inspecting a preview while
the user continues working in other windows.

`tools/win10x-denial verify` checks source freshness, AOT/engine checksums, the
native ABI and the release font's coverage of our actual Material icon constants.
`bundle` runs this verification automatically; a successful AOT compile alone
does not prove that incremental assets include newly added icons.

## Verification and acceptance

The old interaction checklist describes Rust/egui acceptance only. The current
port has compiled and rendered in the standalone native Wayland host; that is
UI evidence, not compositor-session acceptance or a stability benchmark.

Implemented paths awaiting full manual acceptance:

- Centered bottom taskbar, full-height command targets, live running-window
  projections, pins, activation/minimization and multi-window selection.
- Task View using Denial's live preview service, close-app commands and native
  selection of stacking, dwindle and scrolling layouts.
- Full-screen Start, native application catalog/search, retained tile groups,
  three layout choices, tile pinning/size/group menus and inline rename.
- Freeform collision preview from the drag-start snapshot; Metro list reorder
  and nearest-group drop; responsive projection leaves stored cells intact.
- Retained push pages for Phone and Windows 8; Start entrance and launch
  animation data/settings are separate.
- Start/launch selectors and a separately persisted window-motion toggle, with
  a consumer in the Denial scene/window-frame path.
- Win10X Action Center is the native dashboard panel: notifications, media,
  Wi-Fi/Bluetooth/DND/performance quick actions, brightness and volume, battery
  and power. Audio output selection, per-application volume and available
  system/CPU/GPU performance modes use the same managed Denial services as the
  former left dashboard, rendered with our flat Win10 controls.
  The PC right edge and phone top edge/status bar open it. The PC left edge
  invokes Task View; the taskbar clock opens its separate calendar. Start plays a reverse entrance on
  ordinary close instead of a host-wide fade.
  Detailed Wi-Fi connection setup currently opens Denial Settings.

Still required before migration is complete:

- [ ] Full Denial-session application containment, focus and transparent-chrome
      hit testing, real multi-window taskbar switching and preview selection.
- [ ] Match the intended default full-output application policy while retaining
      Denial's native split/float modes; verify client Fullscreen protocol state.
- [ ] Real touch acceptance: drag threshold/follow/neighbor easing/drop snap,
      group rename, taskbar swipe and page gestures. Shared gesture arenas need
      checking where tile drag and page-scroll directions overlap.
- [ ] Visual parity for all saved Start/launch animation styles. W10M row wave
      and Windows 8 flying-tile paths are ported, but the optional Disco/Factor
      launches currently use a provisional fade/lift and need their exact source
      treatment. Do not label them visually accepted.
- [ ] Windows 8's application page/header/padding behavior and precise normal/
      wide face constraints. The app list now scrolls horizontally, but joined
      page headers and exact source spacing still need matching.
- [ ] Port the complete personalization panel and finish Wi-Fi/Bluetooth detail
      flows. Denial's standalone Settings remains available.
- [ ] Cold-start timing/failure handling, repeated open/close and reduced-motion
      paths in the actual embedded compositor, including local Flutter apps.
- [ ] ARM64 kit, build and hardware smoke test; native session/portal/PolicyKit
      distribution integration beyond the staged experimental session.

Denial's `denial-nested` executable is a topology demonstration, not the complete
Flutter desktop. `deniald --wayland` enables its Wayland frontend; it does not
select a host-compositor nested backend. Native acceptance therefore needs an
isolated DRM/KMS seat, a suitable QEMU VM or a spare machine. A normal host
preview cannot supply Denial's real managed windows.

The user chose to perform native acceptance later by switching their own local
session. Never stop their current compositor as a side effect of testing. No
native full-session test has been run in this task.

## User-controlled local session acceptance

For a manual TTY test, first build the current worktree. After logging out of
the graphical desktop yourself, log in on a local TTY and select the display
GPU explicitly (choose the correct card for the machine):

```bash
tools/win10x-denial session --device /dev/dri/cardN
```

The helper refuses to run while `WAYLAND_DISPLAY` or `DISPLAY` is present. It
does not stop graphical-session targets or reconfigure the display manager.
The runtime, control tool and standalone Settings paths point at the matching
local release. PolicyKit-agent startup is disabled for this experimental direct
session; complete portal/PolicyKit integration is separate.

To prepare a separate display-manager choice without replacing the old one:

```bash
tools/win10x-denial package
# Review packaging/out/denial, then install explicitly if desired:
sudo packaging/install-denial-session.sh
```

The staging command includes the custom bundle, matching native runtime,
standalone Settings, upstream license notices and source lock. It omits compiler
tools. The installer supports `DESTDIR` for an isolated file-install check and
does not start/restart the session. It adds **Win10X (Denial / Flutter,
experimental)** beside the legacy entry. Keep the current desktop choice for
returning after the test. The installed path resolves its own prefix, so it
does not rely on this repository or a user's compiler cache.

## Licensing

Our new code retains this repository's GPL-3.0-only license. Denial sources and
their notices remain under the upstream license in the fetched kit; Flutter,
fonts, icons and other bundled components retain their own notices. Keep those
notices with any future redistributed runtime/bundle. Research screenshots and
Microsoft marks remain reference material and are not runtime assets.


## Fluent controls and motion policy (2026-11-10)

The current Flutter chrome follows Fluent 1: square command targets, opaque
light surfaces, blue selection, rectangular slider thumbs, flat hover/press
feedback, keyboard focus and 44 logical-pixel menu targets. Scroll extents clamp
without rubber-band bounce or Material overscroll effects. Native application
icons remain application artwork; this is not a replacement icon pack.

Flutter still renders and schedules frames. The general motion equations come
from `shell/src/fluent1/motion.rs`: `1-(1-t)^2.6` on approach and `t^2.6` on exit,
with 100/167/250/333 ms control/panel tokens. The prepared desktop window host
uses these monotonic curves for workspace transitions, window positioning,
overview/minimize effects and layout reflow. Start retains its independent
saved source styles. Its Expo/Cubic curves use the same Bézier solver as
`start_tiles.rs`, and the Windows 8 flying tile follows the legacy three-stage
lift/edge/reverse-face geometry. Normal tile reflow no longer overshoots, as
requested in the latest animation direction; the older OutBack notes describe
the legacy behavior rather than the current Flutter policy.

The native launcher host retains Start for the selected close duration,
including delayed groups. Start has a separate Escape intent so a focused
search field does not consume dismissal as an EditableText toolbar command.
Menus clamp using their measured size; the media card bounds long titles, and
the lower Action Center section scrolls on short landscape outputs.

Opt-in preview exercises inject touch/key events only inside the preview's own
Flutter tree. They do **not** prove native touch handoff over application
surfaces. Example (after building):

```bash
WIN10X_PREVIEW_EXERCISE=edge-left tools/win10x-denial preview \
  --width 390 --height 780 --capture target/denial/qa/edge-left.png
WIN10X_PREVIEW_EXERCISE=escape-close WIN10X_PREVIEW_CAPTURE_MS=2400 \
  tools/win10x-denial preview --demo --start \
  --capture target/denial/qa/start-closing.png
```

Other exercise values are `edge-right`, `edge-top`, `start-close`, `settings`
and `audio`; the latter requires `--panel action-center`. A normal preview
without capture/exercise variables performs no injected interaction.


## Three shell modes (2026-11-11)

The user requested Windows 10 tablet mode first, then ordinary Windows 10
desktop and Windows 10 Mobile. The evidence, version boundaries and deliberate
exceptions are in `research/windows10-modes/README.md`. `shell_mode` is stored
separately from `start_layout_style`; switching into phone temporarily uses its
phone layout without rewriting the user's previous board/layout preference.
Default migration mode is tablet. No automatic display-size mode switching
silently changes the user's explicit selection.

- Tablet: full-screen Start, pinned/all-apps rail, multi-column alphabetical
  catalog and letter jump, larger left-aligned Back/Search/Task View controls,
  hidden running-app buttons, separate clock/calendar and notification buttons.
  Left edge invokes real Task View, right edge invokes Action Center; a long
  downward drag from the top closes the active client.
- Desktop: bounded lower-left Start, app list beside grouped tiles, persistent
  pins and running app buttons. Calendar and volume are separate flyouts.
- Phone: black Start without a desktop header, six small columns (optional
  eight), horizontally joined app page with search and letter jump; hold a tile
  to edit/drag, then corner commands resize/unpin. Connectivity is left and
  battery/time right. Back/Start/Search replace the taskbar; hold Back opens
  live per-window cards with exact activation/close, hold Start enables timed
  reachability. Only phone exposes the full-width top-down Action Center.
- Native placement policy saves desktop rectangles while touch modes maximize
  main clients; transient dialogs remain separate. The taskbar/navigation strip
  has a real work-area reservation, and phone adds the top status inset. Existing
  Denial managed split layouts are retained in tablet mode. Phone input and
  drawing apply the same reachability offset. Returning to desktop restores the
  session's saved geometry rather than minimizing/respawning clients.
- The compiled shell host uses blue accent, square opaque chrome and disabled
  inherited backdrop material. Standalone Denial Settings and Linux applications
  keep their own UI; this change is not a complete Windows system-app clone.

Preview modes:

```bash
tools/win10x-denial preview --demo --mode tablet --layout windows10 --start
tools/win10x-denial preview --demo --mode desktop --layout windows10 --start
tools/win10x-denial preview --demo --mode phone --start --width 390 --height 780
```

For preview-local checks, `WIN10X_PREVIEW_EXERCISE=all-apps`, `tap:所有应用`,
`tap:平板模式`, `hold:开始；长按单手模式`, and `hold:返回；长按切换应用`
exercise the actual shell widgets/commands. Edge exercises remain available;
phone only uses the top edge for notifications. These are Flutter-side input
checks, not evidence of native client touch handoff.

Bundles now publish as immutable generations under `target/denial/bundles/`.
`target/denial/bundle` points to the latest verified generation; preview/session
resolve that path once so a later rebuild cannot truncate their mapped AOT
library or replace their lazy-loaded font assets. Older generations are kept
until the developer removes them after stopping their previews.

Phone keeps Back/Start/Search visible while applications are open. Soft keyboard
dismisses one-handed mode and pads Start/Action Center; the status clock stays
centered between connectivity and battery.

Remaining fidelity limits: pixel-perfect motion against original-device video,
real-session native back/maximize/split/close and reachability input acceptance,
arbitrary UWP notification inline editors, native quick-action reorder
customization, Live Folders, language-specific alphabet indexing, and full
Windows-style standalone Settings. No Microsoft screenshots, proprietary fonts
or wallpaper have been added to shipped assets.
