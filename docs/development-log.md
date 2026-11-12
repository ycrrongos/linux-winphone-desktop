# Development log

Append-only record of completed features and repository changes. Required by
[`AGENTS.md`](../AGENTS.md).

## Entry template

```markdown
### YYYY-MM-DD — short title

- **Request:**
- **Change:**
- **Files/components:**
- **Verification:**
- **Status/follow-up:**
```

---

### 2026-08-12 — Win10X research library

- **Request:** Collect Windows 10X UI and interaction references locally before
  considering implementation.
- **Change:** Added sourced webpage snapshots, interface images, Chinese notes,
  image indexes, and a source manifest; separated dual-screen hardware material
  from the single-screen research set.
- **Files/components:** `research/windows10x/`,
  `research/windows10x/dual-screen-hardware/`.
- **Verification:** Checked the downloaded inventory and source mappings.
- **Status/follow-up:** Complete; single-screen research is the current product
  reference and dual-screen support is deferred.

### 2026-08-17 — taskbar buttons reach the bottom edge

- **Request:** Remove the gap between taskbar button states and the bottom edge
  at every UI scale.
- **Change:** Replaced the generic fixed-size taskbar icon control with a
  shell-specific full-height painted and interactive button slot.
- **Files/components:** `shell/src/main.rs`.
- **Verification:** `cargo check` and the five `win10x-protocol` tests passed;
  user visually confirmed the corrected edge alignment.
- **Status/follow-up:** Complete.

### 2026-08-17 — remove the previous acrylic implementation

- **Request:** Completely remove the existing acrylic blur implementation to
  prepare for a redesign.
- **Change:** Removed shell wallpaper sampling, CPU blur cache, tint/noise and
  transparency controls, plus compositor HostBackdrop/framebuffer blur code;
  established opaque flat surfaces as the baseline.
- **Files/components:** `shell/src/fluent1/`, `shell/src/main.rs`,
  `compositor/src/`, `README.md`.
- **Verification:** `cargo check -p win10x-shell -p win10x-comp` and
  `cargo test -p win10x-protocol` passed; a shell self-capture confirmed opaque
  rendering.
- **Status/follow-up:** Complete; the new acrylic design has not started.

### 2026-08-17 — eliminate taskbar debug red frames

- **Request:** Stop red lines/frames appearing on taskbar controls when Start
  opens.
- **Change:** Assigned stable egui IDs to the taskbar container, buttons, and
  clock; made standalone shell an undecorated fullscreen `win10x-shell`
  viewport. The earlier app-style Start/Task View underline was also removed.
- **Files/components:** `shell/src/main.rs`, `docs/lessons-learned.md`.
- **Verification:** `cargo check -p win10x-shell` passed and runtime monitoring
  showed no further `changed id between passes` warnings.
- **Status/follow-up:** Complete.

### 2026-08-17 — unify Live Tile icon position

- **Request:** Prevent tile icons from jumping upward when the 3D trapezoid
  reveal finishes.
- **Change:** Shared a 45% vertical icon-center constant between the animated
  and settled tile rendering branches.
- **Files/components:** `shell/src/start_tiles.rs`.
- **Verification:** `cargo check -p win10x-shell` and all five
  `win10x-protocol` tests passed; the latest shell was reopened for inspection.
- **Status/follow-up:** Implementation complete.

### 2026-08-17 — add Codex-native repository rules and change logging

- **Request:** Replace the copied Qoder instructions with Codex-compatible
  project rules and require every new feature or modification to be recorded.
- **Change:** Rewrote `AGENTS.md` with the recovered product decisions and
  Codex verification workflow, corrected conflicting documentation, and added
  this mandatory development log with a reconstructed recent baseline.
- **Files/components:** `AGENTS.md`, `docs/development-log.md`,
  `docs/interaction-checklist.md`, `docs/lessons-learned.md`, `README.md`.
- **Verification:** Documentation paths and required rule terms were checked;
  no code build was run because this change affects Markdown only.
- **Status/follow-up:** Complete; all future repository changes must append a
  new entry here.

### 2026-08-17 — replace Win8.x tile flip with Windows 10 Mobile reveal

- **Request:** Remove the trapezoid tile animation, crossfade the current app to
  the wallpaper under a semi-transparent white layer, and reveal Live Tiles
  from the bottom row upward using opacity and scale. Background and tiles must
  finish together.
- **Change:** Deleted the perspective face/side/shading path. Tiles now join a
  bottom-to-top staggered wave according to their bottom grid edge, growing
  from 82% to full size while fading from 8% to full opacity. The Start
  background now crossfades over the full desktop to the wallpaper and adds a
  white veil with a final alpha of 150; it shares the exact row-wave duration.
  Closing reverses the row order. During runtime verification, the remaining
  auto-ID clock label was replaced with an explicitly identified hover region
  and direct text painting to prevent debug red frames.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `shell/src/fluent1/`, `AGENTS.md`, `README.md`,
  `docs/interaction-checklist.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell`,
  `cargo check -p win10x-shell -p win10x-comp`, three new shell reveal tests,
  and all five `win10x-protocol` tests passed. The latest shell launched and
  remained running without further `changed id between passes` warnings during
  the monitored interval.
- **Status/follow-up:** Implementation complete; awaiting user visual
  confirmation of the animation pacing, scale, and white-layer strength.

### 2026-08-17 — trial Disco Launcher turnstile and preserve launcher research

- **Request:** Replace the current tile reveal with Disco Launcher's animation
  for a visual trial, and save the discovered Windows Phone launcher projects
  and animation findings in a reusable Markdown reference.
- **Change:** Replaced the row-based fade/scale reveal with Disco's reversed
  visible-tile indexing and normalized bottom-right-to-top-left stagger. Each
  tile now has a left-edge perspective turnstile plane plus a separately timed
  inner-content translation/rotation; decoded icon textures follow a projected
  four-vertex mesh. Closing uses Disco's short leftward outward turn. Preserved
  the existing wallpaper + semi-transparent white crossfade and synchronized
  it with the final delayed tile. Added a dated research guide covering Disco,
  MetroLauncherAndroid, Factor Launcher Reboot, Retiled, closed-source visual
  references, exact source locations, animation parameters, and license risks.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `shell/src/fluent1/mod.rs`, `research/windows-phone-launchers.md`,
  `AGENTS.md`, `README.md`, `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all five `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. The current build launched on
  the host Wayland desktop; startup showed only pre-existing font/SVG parser
  warnings and no egui changing-ID warning.
- **Status/follow-up:** Implementation and documentation complete; the latest
  shell remains open for user visual confirmation and possible timing retune.

### 2026-08-17 — rebuild the actual Disco visual-test binary

- **Request:** Investigate why the reopened desktop appeared almost unchanged
  after the Disco animation work.
- **Change:** Confirmed that the directly launched debug executable predated
  the animation patch, stopped that task-owned instance, explicitly rebuilt
  `win10x-shell`, and reopened the new binary. Documented the distinction
  between `cargo check`/test artifacts and the runnable visual-test binary.
- **Files/components:** Build artifact, `AGENTS.md`,
  `docs/lessons-learned.md`, this log.
- **Verification:** `cargo build -p win10x-shell` completed successfully; the
  rebuilt executable launched on the host Wayland desktop and remained
  running without an egui changing-ID warning at startup.
- **Status/follow-up:** Correct current build is open; awaiting renewed user
  visual confirmation of the turnstile animation.

### 2026-08-17 — selectable Start tile animation styles

- **Request:** Keep Disco as an optional style instead of the project look,
  add the other researched tile entrance animations, and let users choose from
  Settings.
- **Change:** Added four shared Start animation styles: Windows 10 Mobile's
  bottom-up scale/opacity wave (default), Disco's two-layer 3D turnstile,
  MetroLauncherAndroid's row-delayed 120epx rise/fade, and Factor Launcher's
  normal-order 20% slide/fade/90% scale. Styles have matching reverse close
  paths and style-specific scene durations. Added an Animations settings tab
  and persisted the selection to `~/.config/win10x/shell.json`. Updated the
  reusable research mapping and repository acceptance rules.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `research/windows-phone-launchers.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all ten `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. Then
  `cargo build -p win10x-shell` refreshed the actual executable and the latest
  shell launched on the host Wayland desktop without an egui changing-ID
  warning at startup.
- **Status/follow-up:** Implementation complete; current build remains open for
  visual comparison of all four styles and settings persistence confirmation.

### 2026-08-17 — source-derived application launch handoffs

- **Request:** Add application launch animations based on the previously
  researched open-source Windows Phone-style launchers.
- **Change:** Application clicks now enter a pending handoff state and the real
  process starts only after the visible transition completes. Disco uses its
  exact ordinary-tile 0.175s stagger plus the selected tile's fixed 0.2s delay
  and 0.3s left-turn exit. Factor expands a neutral target-app surface and icon
  from the clicked tile or app-list rectangle for 0.30s, matching the geometry
  passed to its Android `makeClipRevealAnimation()` call. Metro reverses its
  row-rise shell motion and then performs a plain system-style handoff because
  its upstream code calls `startActivity()` without a custom transition. The
  default Windows 10 Mobile style retains its reverse row wave. Settings text,
  acceptance rules, and the reusable launcher research now distinguish source
  behavior from project-owned approximations.
- **Files/components:** `shell/src/main.rs`, `shell/src/start_tiles.rs`,
  `research/windows-phone-launchers.md`, `docs/interaction-checklist.md`,
  `AGENTS.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 12 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. `cargo build -p win10x-shell`
  refreshed the executable; the latest shell launched on the host Wayland
  desktop without an egui changing-ID warning at startup.
- **Status/follow-up:** Implementation complete; latest build remains open for
  user visual confirmation of all four application-launch styles.

### 2026-08-17 — overlap application cold start with the handoff animation

- **Request:** Correct the launch timing to Windows Phone behavior: begin
  loading the selected application at the instant of the tap and play the
  Start transition concurrently, instead of adding the animation before cold
  start.
- **Change:** Moved process spawning into the click-frame launch handler. The
  pending launch record now drives only Disco/Factor/Metro/default visual state;
  transition completion hides Start without launching again. A failed spawn
  cancels the transition and leaves Start visible with its error banner.
  Corrected repository rules, acceptance text, research mapping, and comments
  that previously prescribed delayed spawning.
- **Files/components:** `shell/src/main.rs`, `AGENTS.md`,
  `research/windows-phone-launchers.md`, `docs/interaction-checklist.md`,
  `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 12 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed.
- **Status/follow-up:** Code complete; awaiting rebuild, runtime restart, and
  user visual confirmation.

### 2026-08-17 — complete Disco launch animation for app-list icons

- **Request:** Make the application launch change visibly match the open-source
  behavior, including launches from an application icon rather than only a
  pinned tile.
- **Change:** Added an explicit tile/app-list/rail launch-source record. Disco
  tile launches retain the selected tile's 0.2s delay plus 0.3s turn-out;
  app-list launches now use Disco's separate `appListAppTransitionAnim0` model:
  visible rows exit in reverse normalized order over 0.2s around the shared
  screen-left axis toward -90°, the selected row waits 0.3s, and letter markers
  move off the left edge. This also prevents launching a pinned application
  from the list from incorrectly marking its right-side tile as selected.
- **Files/components:** `shell/src/main.rs`, `shell/src/start_tiles.rs`,
  `AGENTS.md`, `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 13 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. A final
  `cargo build -p win10x-shell` refreshed the executable, and the corrected
  desktop launched on the host Wayland session without an egui changing-ID
  warning at startup.
- **Status/follow-up:** Implementation complete; latest desktop remains open
  for user visual confirmation of both Disco launch sources and concurrent
  cold-start timing.

### 2026-08-17 — independent application launch animation setting

- **Request:** Add a dedicated setting for application launch animation instead
  of coupling it to the Start tile animation selection.
- **Change:** Split the Animations settings page into independent “Start screen
  animation” and “Application launch animation” sections. The launch selector
  offers Windows 10 Mobile wave, Disco source-specific turnstile, Factor clip
  reveal, and system default (Metro) behavior. Start entrance/ordinary close
  continues to use its own four styles. Both choices persist in
  `~/.config/win10x/shell.json`; older one-field configurations migrate from
  their former Start choice to the equivalent launch style. System default
  launches and closes Start immediately, while the other choices keep their
  concurrent cold-start handoffs.
- **Files/components:** `shell/src/main.rs`, `shell/src/start_tiles.rs`,
  `AGENTS.md`, `research/windows-phone-launchers.md`,
  `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 14 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. `cargo build -p win10x-shell`
  refreshed the executable; the latest desktop launched on the host Wayland
  session without an egui changing-ID warning at startup.
- **Status/follow-up:** Implementation complete; latest desktop remains open
  for user confirmation of the new settings layout and independent behavior.

### 2026-08-18 — rebuild W10M Start reveal as joined row layers

- **Request:** Match the supplied Windows Phone frame sequence more closely:
  rows should feel stuck together while scaling, upper layers should build
  downward toward the rows below, and the old shrunken state should be reduced
  by half.
- **Change:** Replaced the default W10M per-tile center scale with a dedicated
  joined-row transform. Tiles sharing a grid row now use one horizontal scale
  origin, preserve their scaled inter-tile gaps as a single layer, grow from
  91% rather than 82%, and settle downward from a small above-slot offset. The
  bottom-to-top 55ms wave and opacity buildup remain. Animation duration now
  uses the layout's relative row span rather than absolute grid row numbers, so
  moved layouts still finish the tile wave and backdrop together.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `AGENTS.md`, `research/windows-phone-launchers.md`,
  `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 16 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. Tests cover the shared row gap,
  91% initial size, downward settling, completed geometry, and row-offset-
  independent duration.
- **Status/follow-up:** Rebuilt executable is running on the host Wayland
  desktop without an egui changing-ID warning; it remains open for user visual
  confirmation and any motion retuning.

### 2026-08-18 — cap the W10M joined-row reveal at 0.4 seconds

- **Request:** Keep the corrected joined-row motion but make the entire
  animation finish within 0.4 seconds.
- **Change:** Normalized all bottom-to-top row delays into a fixed 0.16s stagger
  window and shortened each row's joined scale/settle motion to 0.24s. A
  multi-row layout therefore finishes its last row and synchronized backdrop
  at exactly 0.40s, regardless of its absolute row count; a single-row layout
  finishes sooner at 0.24s.
- **Files/components:** `shell/src/start_tiles.rs`,
  `research/windows-phone-launchers.md`, `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 17 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. A regression test verifies a
  100-row span still completes both its final tile and backdrop by 0.40s.
  After cleaning only the shell package's regenerable build artifacts, the
  executable rebuilt successfully and launched without an egui changing-ID
  warning.
- **Status/follow-up:** Complete; the 0.40s build remains open on the host
  Wayland desktop for user confirmation.

### 2026-08-18 — make tile drag avoidance local and reversible

- **Request:** Fix neighboring tiles jumping to the upper-right while another
  tile is being moved. Neighbors should move only around their current area and
  return when the dragged tile leaves their space.
- **Change:** Replaced the collision path's global top-left first-free scan with
  a nearest-free search around each affected tile, preferring the grid space
  vacated by the dragged tile when distances tie. Active drags now retain the
  complete pre-drag layout and rebuild every avoidance preview from that stable
  snapshot. Moving away therefore retargets displaced neighbors back to their
  original slots instead of accumulating permanent reflow; dropping commits
  the preview, while cancelling or unpinning restores the unaffected layout.
- **Files/components:** `shell/src/start_tiles.rs`, `AGENTS.md`,
  `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 18 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. The new regression test covers
  local swap-style avoidance, non-overlap, unaffected neighbors, and return to
  the original position after moving away.
- **Status/follow-up:** Complete; the refreshed shell is running on the host
  Wayland desktop without an egui changing-ID warning and remains open for user
  pointer-drag confirmation.

### 2026-08-18 — license and back up the project on GitHub

- **Request:** Publish the current project to the owner's GitHub account under
  the GNU GPL 3.0 license so the working state is preserved.
- **Change:** Set the workspace SPDX identifier to `GPL-3.0-only`, added the
  complete GNU GPL v3 license text, and documented that vendored dependencies
  retain their upstream licenses while downloaded research material is outside
  the project's GPL grant. Excluded the machine-local Qoder session backup from
  version control. Initialized the project history on `main` and uploaded it to
  the private `ycrrongos/win10x-linux-desktop` GitHub repository so copyrighted
  research references are not publicly redistributed.
- **Files/components:** `LICENSE`, `Cargo.toml`, `README.md`, `.gitignore`, Git
  repository metadata and GitHub remote.
- **Verification:** Scanned tracked-source candidates for common credential and
  private-key patterns; no matching credentials were found. Verified ignored
  build output and session backup, repository license metadata, initial commit,
  remote URL, and successful `main` push.
- **Status/follow-up:** Complete. The GitHub access token supplied in chat must
  be revoked and replaced because chat disclosure makes it unsafe for reuse.

### 2026-08-18 — give W10M application launch a dedicated outward row wave

- **Request:** Recreate the supplied Windows Phone application-opening frame
  sequence. It should begin directly from the already-expanded Start layout,
  then reverse the entrance's opacity and row direction instead of first
  returning tiles to their pre-shrunk state.
- **Change:** Split the Windows 10 Mobile application handoff geometry from the
  ordinary Start entrance/close transform. At launch elapsed time zero every
  row is exactly at its settled 100% rectangle. Joined rows then grow to 109%,
  move opposite the entrance's downward settling direction, and fade in a
  top-to-bottom wave. The process still starts in the click frame, and the full
  row sequence remains capped at 0.40s.
- **Files/components:** `shell/src/start_tiles.rs`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 20 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. `cargo build -p win10x-shell`
  rebuilt the executable, and the refreshed shell launched on the host Wayland
  desktop without an egui changing-ID warning while idle. Regression tests
  cover the exact settled launch frame, 100%→109% outward geometry, reversed
  top-to-bottom row order, and the 0.40s completion boundary.
- **Status/follow-up:** Implementation complete; refreshed shell remains open
  for user visual confirmation. The user's persisted launch style is currently
  System default, so Windows 10 Mobile wave must be selected in Settings to
  preview this particular style; the existing preference was not overwritten.

### 2026-08-18 — carry the tapped W10M tile into the application surface

- **Request:** Start the tapped tile 0.08s after the other tiles, keep it from
  disappearing after it lifts, expand it to full screen while its tile color
  fades into the application page, and make its animation 0.20s longer.
- **Change:** Removed the tapped tile from the ordinary W10M row-fade painter
  and gave it a top-level handoff proxy. It follows its own row-wave delay plus
  0.08s, holds the exact clicked rectangle during that delay, then expands to
  the full viewport over 0.44s—0.20s longer than an ordinary 0.24s row tile.
  The whole tile face fades during the expansion so nested composition exposes
  the real application launched in the click frame. Other tiles retain their
  0.40s top-to-bottom exit wave, and launches from the app list or rail do not
  receive a fake selected tile.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `AGENTS.md`, `docs/interaction-checklist.md`,
  `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 22 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. Regression tests cover the
  selected tile's row-relative delay plus 0.08s hold, its 0.44s expansion, its
  exclusion from the ordinary disappearing row layer, and the extended scene
  completion boundary. `cargo build -p win10x-shell` rebuilt the executable;
  the refreshed shell launched on the host Wayland desktop without an egui
  changing-ID warning while idle.
- **Status/follow-up:** Implementation complete; refreshed shell remains open
  for user visual confirmation. The persisted application-launch style remains
  System default and must be switched to Windows 10 Mobile wave to preview this
  option; the user's saved preference was not overwritten.

### 2026-08-18 — separate selected-tile lift from full-screen expansion

- **Request:** Correct the selected W10M launch transition: the tile must finish
  its lift animation before the full-screen animation begins, rather than
  blending both motions into one continuous expansion.
- **Change:** Replaced the single 0.44s source-to-screen interpolation with two
  explicit sequential phases after the row-relative 0.08s delay. Phase one
  applies the same 0.24s outward/upward lift geometry as the other launch tiles
  while keeping the selected face fully opaque. Phase two starts from that
  exact lifted rectangle, expands it to the viewport over 0.20s, and only then
  fades the tile face into the concurrently launched application. The clicked
  tile now carries its shared row origin and stride into the top-level proxy so
  the first phase matches the existing joined-row geometry without a jump.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `AGENTS.md`, `docs/interaction-checklist.md`,
  `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 22 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. The selected-tile regression now
  checks that full-screen progress remains exactly zero throughout the hold and
  0.24s lift—including the lift's final boundary frame—then advances only in
  the following 0.20s phase. `cargo build -p win10x-shell` rebuilt the binary;
  the refreshed host-Wayland shell launched without an egui changing-ID
  warning while idle.
- **Status/follow-up:** Implementation complete; refreshed shell remains open
  for user visual confirmation. Windows 10 Mobile wave must be selected in the
  application's launch-animation setting to preview it.

### 2026-08-18 — remove the W10M full-screen tile proxy and hold Start background

- **Request:** Correct the launch sequence again: no tile should enlarge into
  a full-screen surface. While all tiles lift away, the application list must
  fade; after the content is gone, only the Start background should remain
  while waiting for the application to start.
- **Change:** Removed the selected-tile full-screen proxy and its carried row
  geometry. The tapped tile retains its row-relative 0.08s delay but now uses
  the same 0.24s outward lift, opacity loss, and disappearance as every other
  tile. The application list, rail, and search share a synchronized fade that
  ends with the last delayed tile. W10M application launch keeps the wallpaper
  plus white veil at full strength instead of reversing its crossfade, then
  holds that background alone for 0.20s before dismissing Start to reveal the
  concurrently launched application.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `AGENTS.md`, `docs/interaction-checklist.md`,
  `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, all 23 `win10x-shell` tests,
  and all five `win10x-protocol` tests passed. Regression coverage verifies
  that the selected tile remains in the ordinary lift/fade path, Start chrome
  opacity reaches zero with the final content, the wallpaper plus white veil
  remains at full scene progress, and the total handoff includes the 0.20s
  background-only hold. `cargo build -p win10x-shell` rebuilt the executable;
  the refreshed host-Wayland shell launched without an egui changing-ID
  warning while idle.
- **Status/follow-up:** Implementation complete; refreshed shell remains open
  for user visual confirmation. Windows 10 Mobile wave must be selected in the
  application's launch-animation setting to preview it.

### 2026-08-18 — launch applications fullscreen inside the Win10X session

- **Request:** Make applications start fullscreen inside this desktop instead
  of appearing as ordinary host-desktop windows.
- **Change:** Replaced the compositor's maximized-minus-taskbar application
  layout with a real xdg-toplevel `Fullscreen` configure covering the complete
  output geometry. New toplevels are activated and receive keyboard focus,
  while the full-output shell remains the top transparent chrome layer for the
  taskbar and panels. Child toolkits are directed to the nested Wayland session
  (`WAYLAND_DISPLAY`/`XDG_SESSION_TYPE`) without inheriting the host X11
  display. Nested application launches now prefer desktop-entry `Exec` so they
  inherit that socket directly, and all shell-launched applications use their
  own Unix process group so a shell test restart cannot send them Ctrl+C.
- **Files/components:** `compositor/src/handlers/xdg_shell.rs`,
  `compositor/src/main.rs`, `shell/src/apps.rs`, `README.md`, `AGENTS.md`,
  `docs/testing.md`, `docs/interaction-checklist.md`,
  `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 23 `win10x-shell` tests, all five
  `win10x-protocol` tests, and the new `win10x-comp` full-output/fullscreen
  state regression test passed. `cargo build -p win10x-shell -p win10x-comp`
  refreshed both binaries; `git diff --check` passed. The complete nested
  compositor launched successfully with its shell and demo application and
  remains open for visual inspection; runtime emitted no egui changed-ID/red
  diagnostic warning.
- **Status/follow-up:** Implementation complete; awaiting user visual
  confirmation of the fullscreen geometry and shell overlay behavior.

### 2026-08-18 — preserve active application beneath shell chrome

- **Request:** Follow through on fullscreen in-desktop launching so the opened
  application is not only visible but also remains the active input target.
- **Change:** Separated shell stacking from window activation. Shell layout,
  app-click restacking, final relayout, and the per-frame render remap now raise
  the transparent chrome with `activate = false`; the newest fullscreen app
  keeps activation and keyboard focus while taskbar/panel hit testing remains
  above it.
- **Files/components:** `compositor/src/shell_window.rs`,
  `compositor/src/handlers/xdg_shell.rs`, `compositor/src/input.rs`,
  `compositor/src/winit.rs`, `AGENTS.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, the `win10x-comp` fullscreen layout regression
  test, `cargo build -p win10x-shell -p win10x-comp`, and `git diff --check`
  all passed. The final rebuilt nested desktop launched successfully and
  remains open; no egui changed-ID/red diagnostic warning appeared.
- **Status/follow-up:** Complete; awaiting user visual/input confirmation.

### 2026-08-18 — restore app input and synchronize real running windows

- **Request:** Fix the fullscreen application being impossible to click and
  make that application appear in both the taskbar and the background-app/Task
  View list.
- **Change:** Replaced the shell-blocked `Space::element_under` lookup with a
  top-to-bottom application hit test that skips transparent shell before
  resolving a surface. Added a per-session atomic runtime bridge: the
  compositor publishes real xdg app IDs, titles, and active state, while shell
  matches them to desktop entries or creates transient entries. Running apps
  now get stable taskbar buttons with an in-bounds bottom indicator and appear
  in Task View. Selecting either UI entry sends an activation request to the
  compositor, which raises and focuses the existing window instead of spawning
  a duplicate.
- **Files/components:** `compositor/src/state.rs`,
  `compositor/src/session_ipc.rs`, `compositor/src/handlers/xdg_shell.rs`,
  `compositor/src/input.rs`, `compositor/src/winit.rs`, `compositor/src/main.rs`,
  `compositor/Cargo.toml`, `crates/win10x-protocol/src/lib.rs`,
  `shell/src/main.rs`, `README.md`, `AGENTS.md`, `docs/testing.md`,
  `docs/interaction-checklist.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 24 `win10x-shell` tests, all five
  `win10x-protocol` tests, and the `win10x-comp` fullscreen regression test
  passed. `cargo build -p win10x-shell -p win10x-comp` refreshed both binaries.
  The rebuilt nested session launched successfully; its live compositor state
  reported Alacritty with its current title and `active: true`, and runtime
  logs contained no egui changed-ID/red diagnostic warning.
- **Status/follow-up:** Implementation complete; the repaired nested desktop
  remains open for user confirmation of application input, taskbar presence,
  and Task View presence.

### 2026-08-18 — add taskbar app commands and real window-state controls

- **Request:** Make a taskbar app icon launch an unopened app, minimize a
  running app, and restore a minimized app. Add an app menu opened by right
  click or an upward swipe with close, pin/unpin, new window, split-screen, and
  floating-window commands. After testing exposed that a floating terminal
  could not move, make floating windows draggable as well.
- **Change:** Taskbar now combines persisted pinned entries with live unpinned
  applications, keeps their groups separated, and applies the requested
  three-state primary-click behavior. A stable-ID, touch-sized modal menu is
  available from secondary click or a 30 epx upward icon drag. The session
  protocol now carries minimized state and fullscreen/split-left/split-right/
  floating placement plus activate, minimize, maximize, close, split, and
  float commands. `win10x-comp` applies those commands to real xdg-toplevels,
  moves minimized surfaces outside the output, restores the next available
  focus, and places floating windows at 72% of the output. The compositor also
  reserves the floating surface's top 36 epx as a mouse/touch drag strip, so
  borderless clients can move without implementing their own titlebar; shell
  chrome is re-raised without activation throughout the drag.
- **Files/components:** `shell/src/main.rs`,
  `crates/win10x-protocol/src/lib.rs`, `compositor/src/state.rs`,
  `compositor/src/session_ipc.rs`, `compositor/src/handlers/xdg_shell.rs`,
  `compositor/src/input.rs`, `compositor/src/grabs/move_grab.rs`,
  `compositor/src/shell_window.rs`, `README.md`,
  `AGENTS.md`, `docs/interaction-checklist.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 24 `win10x-shell` tests, all five
  `win10x-protocol` tests, and both `win10x-comp` layout tests passed. `cargo
  build -p win10x-shell -p win10x-comp` rebuilt both binaries. The refreshed
  nested session launched with the extended live state (`minimized` and
  `mode`) and no egui changed-ID/red diagnostic warning; the first visual pass
  exposed the missing borderless-window drag path, after which the rebuilt
  drag-enabled session was relaunched for user confirmation.
- **Status/follow-up:** Feature implementation complete; current nested desktop
  remains open for final user visual and touch confirmation.

### 2026-08-18 — show and activate every window from the taskbar app menu

- **Request:** When one taskbar application has several open windows, show a
  preview for every window in its application menu and let the user select the
  exact window to bring to the foreground.
- **Change:** Extended the shell/compositor bridge from app-only summaries to
  individual xdg-toplevel records with stable window IDs and preview paths.
  The compositor now renders each duplicated app window into an atomically
  refreshed, bounded PNG thumbnail. The taskbar menu expands into a touch-sized
  one- or two-column preview grid, labels every card, marks the active window,
  and falls back to the app icon until a thumbnail is ready. Selecting a card
  sends a window-specific activation request which raises and keyboard-focuses
  that xdg-toplevel without closing its sibling windows.
- **Files/components:** `crates/win10x-protocol/src/lib.rs`,
  `compositor/src/window_previews.rs`, `compositor/src/session_ipc.rs`,
  `compositor/src/state.rs`, `compositor/src/winit.rs`,
  `compositor/src/main.rs`, `shell/src/main.rs`, `README.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 24 `win10x-shell` tests, all five
  `win10x-protocol` tests, and both `win10x-comp` tests passed. `cargo build -p
  win10x-shell -p win10x-comp` rebuilt the nested session. Runtime verification
  opened two Alacritty xdg-toplevels with the same app ID, produced two valid
  150 x 180 RGBA previews containing their actual rendered surfaces, and
  confirmed that an `ActivateWindow` request for window 1 changed only window
  1 to `active: true`. No egui changed-ID/red diagnostic warning appeared.
- **Status/follow-up:** Implementation complete; the rebuilt nested desktop is
  left open with two Alacritty windows for user visual confirmation of the menu
  layout and preview-card click behavior.

### 2026-08-18 — add the Windows 8 flying-tile application launch style

- **Request:** Add an optional Windows 8 application-opening animation informed
  by `coldshineb/flutter_metro_ui`, while restoring the original behavior the
  demo omits: the rest of the Start screen moves backward and the selected tile
  first flies outward while growing before its 3D flip reaches full screen.
- **Change:** Added a separately persisted `Windows 8 flying tile` option to
  Application launch animation settings. On click, the real process still
  starts immediately. Non-selected tiles transform around one shared board
  origin to 72% scale and fade over 240 ms while rail/list/search content fades
  with them. The selected tile is removed from that layer and repainted exactly
  over its settled source, then follows a three-stage 720 ms path: lift and
  enlarge, move toward center while turning edge-on, then switch to an
  unmirrored back face while completing the 180-degree turn and filling the
  output. The wallpaper remains behind the retreat instead of crossfading away.
  Added source notes recording the reference project's 850 ms interpolation,
  its missing scene retreat, and its absent license file; no third-party source
  code or assets were copied.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `README.md`, `AGENTS.md`, `docs/interaction-checklist.md`,
  `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 26 `win10x-shell` tests—including new
  source-geometry, scene-retreat, mid-air 90-degree, final
  180-degree/full-output, and preference-persistence regressions—all five
  `win10x-protocol` tests, both `win10x-comp` tests, `git diff --check`, and
  `cargo build -p win10x-shell -p win10x-comp` passed. The rebuilt nested
  desktop launched with `app_launch_animation: windows8`, Start was opened for
  inspection, and runtime logs contained no egui changed-ID/red diagnostic
  warning.
- **Status/follow-up:** Implementation complete; the Windows 8 style is selected
  in the currently open nested desktop and awaits user visual confirmation of
  the three-stage timing and perceived depth.

### 2026-08-18 — add flutter_metro_ui Start and application-switch animations

- **Request:** Bring in flutter_metro_ui's application-switch animation and add
  that project's Start-screen startup motion to the selectable Start animation
  list, without merging either behavior into the separate Windows 8 flying-tile
  application-launch style.
- **Change:** Added the persisted `flutter_metro_ui group sweep` Start option.
  The current freeform tile board is treated as one joined group: all tiles use
  a shared board-space center while moving 100 epx from the right, fading from
  zero, and scaling from 80% with Flutter's `easeOutExpo` control points over
  the reference first group's 1.05 s interval; ordinary Start close reverses
  it. Existing-window activation now begins in the click frame and overlays a
  compositor-captured target snapshot. Restoring from Start or with no active
  app uses the source taskbar rectangle → app viewport interpolation for 200 ms
  with ease-out-quad. Switching between two active apps uses the reference
  500 ms sequence: the old snapshot retreats to 80%, then the target slides in
  from ±1.5 output widths at 50% scale and grows to 100%. The taskbar remains
  above both proxies. Single-window thumbnails are now refreshed as well as
  duplicate-window menu previews, with a neutral surface/app-icon fallback.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `compositor/src/window_previews.rs`, `README.md`, `AGENTS.md`,
  `docs/testing.md`, `docs/interaction-checklist.md`,
  `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 28 `win10x-shell` tests (including joined
  group geometry and independent preference persistence), all five
  `win10x-protocol` tests, both `win10x-comp` layout tests, `cargo build -p
  win10x-shell -p win10x-comp`, and `git diff --check` passed. The rebuilt
  nested compositor and shell launched successfully, opening Start produced no
  egui changed-ID/red diagnostic warning, and the compositor generated a real
  PNG snapshot for its single Alacritty toplevel. Automated geometry and
  runtime data flow are verified; pointer-driven perception of the 200/500 ms
  handoffs remains for user visual confirmation.
- **Status/follow-up:** Implementation complete; awaiting user visual
  confirmation of the two new motions and their pacing.

### 2026-08-18 — keep application-switch animation snapshots sharp

- **Request:** Fix the visibly blurred application content during the
  flutter_metro_ui window-switch animation.
- **Change:** Split each compositor preview into two explicit artifacts. The
  taskbar menu keeps an aspect-fitted image bounded to 320×180, while the
  application-switch path receives a separately published PNG rendered at the
  output's physical resolution. The shell now caches and loads those sources
  independently, retaining a compatibility fallback for older session-state
  files. Also removed a redundant vertical flip found while inspecting the new
  high-resolution capture, so both preview types remain upright.
- **Files/components:** `crates/win10x-protocol/src/lib.rs`,
  `compositor/src/session_ipc.rs`, `compositor/src/window_previews.rs`,
  `shell/src/main.rs`, `README.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `docs/testing.md`,
  `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 28 `win10x-shell` tests, all five
  `win10x-protocol` tests, all four `win10x-comp` tests (including new output
  scale and thumbnail-bound regressions), `cargo build -p win10x-shell -p
  win10x-comp`, and the real nested desktop passed. In the current host-fullscreen
  run, the compositor generated an upright 2561×1601 switch PNG plus a 288×180
  taskbar PNG for the same Alacritty window; session JSON publishes both paths
  and current runtime logs contain no egui changed-ID/red diagnostic warning.
- **Status/follow-up:** Fix complete; the rebuilt full-screen desktop remains
  open for user visual confirmation of the 200/500 ms animations.

### 2026-08-18 — reduce switch snapshots to 25% output dimensions

- **Request:** Reduce the application-switch snapshot resolution to 25% of the
  screen after the native-resolution implementation made the desktop severely
  laggy.
- **Change:** Added a fixed 0.25 switch-capture scale applied to both physical
  output dimensions. On the current 2561×1601 output this produces a 640×400
  animation image (about 1/16 of the pixels), while taskbar thumbnails remain
  independently bounded to 320×180. Updated protocol comments, product rules,
  acceptance text, and tests to make this performance/quality choice durable.
- **Files/components:** `compositor/src/window_previews.rs`,
  `crates/win10x-protocol/src/lib.rs`, `README.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `docs/testing.md`,
  `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 28 shell tests, all five protocol tests,
  all four compositor tests, and `cargo build -p win10x-shell -p win10x-comp`
  passed. The rebuilt host-fullscreen session generated correctly oriented
  640×400 switch PNGs for all three live windows, including Brave, while menu
  PNGs stayed bounded. Five-second `top` sampling showed the previous sustained
  68% compositor load replaced by idle intervals around 14–16% with brief
  synchronous refresh spikes up to about 50%.
- **Status/follow-up:** Requested 25% scaling is complete and the full-screen
  test desktop remains running. If periodic capture spikes remain perceptible,
  move PNG encoding off the render thread or refresh full switch images on
  demand in a separate follow-up.

### 2026-08-18 — remove periodic preview freezes from the render thread

- **Request:** Fix the remaining behavior where the desktop was fully smooth
  between updates but froze for about a second, initially suspected to be a
  memory leak.
- **Change:** Kept the user-selected 25% switch resolution but restructured the
  preview pipeline. Each render-side refresh now reads back only one toplevel;
  a cursor distributes all live windows across the 2-second background or
  700-ms open-menu cycle. RGBA buffers go to one named encoder worker through a
  capacity-one `sync_channel`; resizing, PNG compression, and atomic writes are
  no longer on the compositor render thread, and stale captures are dropped
  when the worker is behind instead of growing memory usage.
- **Files/components:** `compositor/src/window_previews.rs`,
  `compositor/src/state.rs`, `README.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `docs/testing.md`,
  `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 28 shell tests, all five protocol tests,
  all five compositor tests (including the new refresh-cycle distribution
  regression), and `cargo build -p win10x-shell -p win10x-comp` passed. In the
  rebuilt host-fullscreen session with three Alacritty windows, 640×400 preview
  writes were spaced roughly 0.67 seconds apart rather than grouped into one
  burst. Thread-level sampling showed the encoder doing 15–60% CPU while the
  compositor main/render thread stayed around 0–5%. The compositor remained at
  exactly 195112 KiB RSS with seven threads across a 15-second sample, proving
  the bounded queue did not accumulate images; runtime logs showed no encoder,
  egui changed-ID, or red diagnostic errors.
- **Status/follow-up:** Fix complete; the rebuilt full-screen three-window test
  desktop remains open for user interaction and visual confirmation.

### 2026-08-18 — selectable Start layouts, scrollable tile boards, and page motion

- **Request:** Allow application-switch animation to be disabled without paying
  its snapshot cost; let a full Start board continue beyond the viewport; add
  persisted Windows 10, Windows Phone, and Windows 8 Start layouts; then add a
  sliding transition when the Phone/Windows 8 layouts enter or leave their
  application lists.
- **Change:** Added a separate persisted switch-animation toggle. Disabling it
  activates real windows immediately without a proxy and sends a compositor
  policy that stops background GPU readback and transition-image encoding;
  bounded thumbnails are captured only while a multi-window taskbar menu is
  open. The shared tile collection now supports vertical and horizontal flow,
  axis-aware packing/drop previews, and content extents with reserve space for
  additional tiles. Windows 10 keeps its split list/vertical-board layout;
  Windows Phone uses a vertically scrolling tiles-only page and horizontal
  swipe navigation; Windows 8 uses the source-informed dark Metro treatment,
  horizontally scrolling tiles, a column-first horizontal app list, and both
  upward swipe and lower-left button navigation. Phone pages now push left/right
  and Windows 8 pages push up/down over 280 ms with `easeOutCubic`; outgoing and
  incoming content plus their headers coexist and move together, and an
  in-progress transition can reverse from its current position.
- **Files/components:** `shell/src/main.rs`, `shell/src/start_tiles.rs`,
  `compositor/src/state.rs`, `compositor/src/session_ipc.rs`,
  `compositor/src/window_previews.rs`, `crates/win10x-protocol/src/lib.rs`,
  `README.md`, `AGENTS.md`, `docs/interaction-checklist.md`,
  `docs/lessons-learned.md`, `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 30 `win10x-shell` tests, all seven
  `win10x-comp` tests (including disabled-capture policy, 25% transition size,
  and menu-thumbnail bounds), all five `win10x-protocol` tests, and `cargo build
  -p win10x-shell -p win10x-comp` passed. The rebuilt nested compositor/shell
  launched successfully and its startup/runtime log contained no egui
  changed-ID/red diagnostic warning. The accepted three layouts are preserved;
  simulated host input could not bypass the outer compositor's reserved Super
  key or inject into a nested compositor without virtual-keyboard support, so
  final touch perception of the new 280 ms pushes remains a user visual check.
- **Status/follow-up:** Implementation complete; rebuilt desktop left running
  for user confirmation of the Phone horizontal and Windows 8 vertical page
  transitions.

### 2026-08-18 — nonlinear app-list paging and source-faithful Win8 Start

- **Request:** Make the application-list page switch visibly nonlinear and make
  the Windows 8 Start layout and animation strictly follow
  `coldshineb/flutter_metro_ui`.
- **Change:** Replaced the Phone/Win8 page-push interpolation with Flutter's
  exact cubic-Bézier `easeOutExpo` curve over 340 ms. Rebuilt the Win8 tile page
  around the reference's `#1d1d1d` surface, fixed 120×120 normal and 250×120
  wide faces, 10 px gaps, 100 px start / 140 px top / 60 px bottom padding,
  at-most-six-row column-first packing, horizontal extent, Metro palette, and
  150/300 ms press tilt. Win8 now always uses the reference's 3-second timeline:
  its first group fades, moves from +100 px, and scales 80%→100% through the
  0.00..0.35 `easeOutExpo` interval (1.05 s), while the title and user bar use
  the 0.20..0.50 `easeOutCubic` interval (0.6..1.5 s). The Settings page shows
  this fixed Win8 motion instead of a misleading selectable style, while
  retaining the saved choice for Windows 10/Phone. Entering Win8 no longer
  repacks or overwrites those layouts' freeform tile coordinates. Since the
  shell has no persisted category metadata, its pinned apps are one source-
  shaped Metro group instead of inventing the demo repository's domain-specific
  group names.
- **Files/components:** `shell/src/main.rs`, `shell/src/start_tiles.rs`,
  `README.md`, `AGENTS.md`, `docs/interaction-checklist.md`,
  `docs/lessons-learned.md`, `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 33 shell tests, all seven compositor tests,
  all five protocol tests, and `cargo build -p win10x-shell -p win10x-comp`
  passed. Regression tests assert the reference geometry, row cap, animation
  intervals, exact curve endpoints, and strong nonlinear response. A full-screen
  nested run rendered the fixed geometry/header without egui changed-ID or red
  diagnostic warnings; the rebuilt preview remains running for final motion and
  palette judgment.
- **Status/follow-up:** Complete, awaiting user visual confirmation. The
  reference project has no all-apps page, so that page remains the documented
  shell extension and uses the new nonlinear push.

### 2026-08-18 — shared Start groups, Win8 resizing, and exact grouped entrance

- **Request:** Let Windows 8 tiles change size like tiles in the other Start
  layouts, add flutter_metro_ui-style tile grouping to every layout, and correct
  the Windows 8 Start-opening motion to match that repository rather than using
  a single approximation for the whole board.
- **Change:** Added persisted group IDs, editable group titles, legacy migration
  into a default “开始” group, and tile-menu actions for moving to an existing
  group or creating a new group. Windows 10 and Phone retain freeform tiles but
  pack groups with separation along their scrolling axis; Windows 8 lays out
  each group independently from left to right. Its resize entry now exposes the
  reference project's two legal faces—120×120 normal and 250×120 wide—without
  rewriting the freeform geometry used by other layouts. The flutter entrance
  now transforms every real group around its own center from +100 px, 80% scale,
  and zero opacity on the exact 3000 ms controller intervals
  `groupIndex × 0.04 .. +0.35`; later groups therefore begin 120 ms apart, and
  Start remains alive through whichever is later: the final group or the
  0.6..1.5 s header interval.
- **Files/components:** `shell/src/start_tiles.rs`, `shell/src/main.rs`,
  `README.md`, `AGENTS.md`, `docs/interaction-checklist.md`,
  `docs/lessons-learned.md`, `research/windows-phone-launchers.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`, `cargo check -p
  win10x-shell -p win10x-comp`, all 36 shell tests, all seven compositor tests,
  all five protocol tests, and `cargo build -p win10x-shell -p win10x-comp`
  passed. New regressions cover legacy-group migration, group-separated packing
  on both axes, Win8 per-group horizontal geometry, the two exact Metro face
  sizes, independent stagger progress, and the full five-group duration. The
  rebuilt compositor started and was placed full screen; its runtime log has no
  egui changed-ID/red-rectangle diagnostic. Automated screenshot capture was
  unavailable on this host because `grim` is not installed.
- **Status/follow-up:** Implementation complete; the latest full-screen desktop
  is running for user confirmation of group editing, resizing, and the motion.

### 2026-08-18 — repair Win8 tile dragging and group-title editing

- **Request:** Fix the newly added grouping UI because tiles could not be
  dragged and group labels could not actually be changed.
- **Change:** Added a dedicated Windows 8 drag state that lifts the selected
  tile above its siblings, follows the pointer, resolves the destination group
  and insertion point on release, and reorders the source tile list used by the
  column-first Metro packer. Cross-group drops update persisted group membership
  and trigger freeform repacking when returning to Windows 10/Phone. Replaced
  the fragile text field inside an auto-closing context menu with a persistent
  inline title editor shared by every layout. Clicking/tapping a title or using
  its right-click “重命名” command starts editing; Enter or focus loss commits,
  Escape cancels, and blank names normalize to “未命名组”.
- **Files/components:** `shell/src/start_tiles.rs`, `README.md`, `AGENTS.md`,
  `docs/interaction-checklist.md`, `docs/lessons-learned.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp` and all 38 shell
  tests passed, including new regressions for Win8 reorder/cross-group drops and
  durable rename commits. Full workspace checks, rebuild, and visual runtime
  inspection follow in this task.
- **Status/follow-up:** Code complete; awaiting final rebuild and user visual
  confirmation.

### 2026-08-18 — drag/rename repair verification follow-up

- **Request:** Complete verification and launch the repaired interaction build.
- **Change:** No additional behavior change; rebuilt the shell and compositor
  after the drag and inline-title patches and replaced the stale running preview.
- **Files/components:** `shell/src/start_tiles.rs` and the runtime build output.
- **Verification:** `cargo check -p win10x-shell -p win10x-comp`, all 38 shell
  tests, all seven compositor tests, all five protocol tests, `cargo build -p
  win10x-shell -p win10x-comp`, and `git diff --check` passed. The rebuilt nested
  session launched full screen and emitted no egui changed-ID/red-rectangle
  diagnostic; the host's existing SVG parser warnings are unrelated.
- **Status/follow-up:** Fix complete; latest desktop left running for direct
  pointer/touch confirmation.

### 2026-08-19 — Win8 Metro tile drag upgraded to Win10-style live reorder

- **Request:** The Windows 8 Start screen tile drag was nearly unusable; add
  the same drag logic as Windows 10/Phone while preserving the Metro style and
  tile grouping.
- **Change:** The Win8 drag now mirrors the Win10/Phone interaction end to
  end: `drag_started` records the shared pending move and the grab activates
  after the same 8 epx `DRAG_ACTIVATE` travel; the floating tile follows the
  pointer (existing shadow / 1.03 lift visuals kept); and every frame computes
  a live reorder preview as a pure function of the layout captured at drag
  start plus the pointer (`windows8_drag_preview`): the dragged tile is lifted
  out, the remaining siblings repack with the ordinary column-first Metro
  packer, and the pointer maps onto that sibling grid's column-first anchor
  slots with a center tie-break to pick the insertion point. Siblings ease
  into their new derived slots through the shared `visual`/`anims` maps
  (`windows8_step_visuals`, decelerate avoid curve), and paint uses those
  eased positions only in the settled state so entrance/launch transforms stay
  untouched. Drops can never be silently cancelled: the pointer-to-group
  distance metric means the nearest group always accepts the drop (the old
  12 px containment requirement is gone). Release settles the floating tile
  into its derived slot with the OutBack snap and persists; aborts restore the
  pre-drag snapshot (`cancel_windows8_drag`, wired into `clear_interaction` /
  `confirm_resize`). Grouping semantics unchanged: the drag still only
  reorders the source list and updates group membership, never freeform
  `col`/`row` cells, and `group_repack_pending` still re-packs freeform
  layouts afterwards. Launch clicks are gated by the pending move and
  `suppress_click` so a cancelled short drag never launches.
- **Files/components:** `shell/src/start_tiles.rs` (Windows8MetroDrag state,
  paint_windows8_metro prologue/drop, finish_windows8_drag, new
  windows8_drag_preview/rect_distance_sq/windows8_step_visuals helpers,
  tests), `AGENTS.md`, `README.md`, `docs/interaction-checklist.md`.
- **Verification:** `cargo fmt -p win10x-shell -p win10x-comp`,
  `cargo check -p win10x-shell -p win10x-comp`, and all 39 shell tests passed,
  including the rewritten cross-group drop regression and a new
  insert-before-the-tile-under-the-pointer case. Runtime visual confirmation
  follows in this task.
- **Status/follow-up:** Rebuilt nested session left running for the user's
  pointer/touch confirmation.

### 2026-08-25 — remove research HTML snapshots and screenshots from the repo

- **Request:** Delete unused HTML and images from GitHub and keep source code.
- **Change:** Removed the Windows 10X research page snapshots, UI screenshots,
  dual-screen hardware images/HTML, and the Start-menu backup reference PNG
  from the repository. Kept notes, source TSVs, and code backups. Updated the
  research READMEs and image index to point at source links instead of local
  media. Left vendored `third_party/fluent-egui` demo assets alone.
- **Files/components:** `research/windows10x/**`,
  `research/backups/start-menu-10x/`, `docs/development-log.md`.
- **Verification:** Confirmed no non-`third_party` HTML/PNG/JPG files remain
  tracked; commit and push follow in this task.
- **Status/follow-up:** Complete after push to `origin/main`.

### 2026-08-25 — remove unused fluent-egui demo PNGs

- **Request:** Delete unused `third_party/fluent-egui` images that do not affect
  project runtime from GitHub as well.
- **Change:** Removed all demo/gallery PNGs under
  `third_party/fluent-egui/src/assets/`. Kept the runtime-required icon font,
  IconsData.json, and Fluent System Icons license.
- **Files/components:** `third_party/fluent-egui/src/assets/*.png`,
  `docs/development-log.md`.
- **Verification:** `cargo check -p win10x-shell` passed after deletion.
- **Status/follow-up:** Complete after push to `origin/main`.

### 2026-08-25 — fix GitHub language bar after media removal

- **Request:** The GitHub language sidebar still showed ~96% HTML after the
  research HTML/images were deleted.
- **Change:** Confirmed the default branch no longer tracks any `.html` files.
  Added `.gitattributes` so `third_party/**`, research notes, fonts, and JSON
  are marked vendored/documentation for Linguist, and pushed to trigger a
  language recompute centered on Rust source.
- **Files/components:** `.gitattributes`, `docs/development-log.md`.
- **Verification:** `git ls-tree origin/main` shows zero `.html` paths; only
  remaining large non-Rust blob is the Fluent icon font under vendored assets.
- **Status/follow-up:** Complete after push; GitHub may still cache the old
  language bar briefly before Linguist refreshes.

### 2026-11-10 — Denial/Flutter migration baseline and experimental shell

- **Request:** Migrate the project onto Denial to reuse its window layouts,
  rendering/effects and native platform. The user chose Flutter while preserving
  Win10X appearance, full-screen Start/tiles and touch interaction, then chose to
  perform full local-session acceptance themselves after the code work.
- **Change:** Created `codex/denial-migration`; pinned Denial 0.6.2 source,
  runtime/compiler packages, Flutter 3.47.5 and Dart 3.13.4. Added a checksum-
  verified user-local bootstrap and build workflow, committed Pub locks, stale-
  bundle/source guards, release font coverage and native ABI verification. The
  custom application composes our Flutter taskbar/Start contributions with the
  Denial reference desktop; exact prepared-source patches enable full-output
  launcher geometry/input, an in-place wallpaper/white-veil transition without
  blur, concurrent native launch and independent window-motion control. Ported
  application catalog/search, grouped tiles, layout selection, persisted legacy
  settings import, pin/size/group menus, inline rename, reversible drag previews,
  native window selection/previews and layout commands. Added a Win10X Action
  Center backed by Denial notification, media, radio, volume, battery and power
  services. Added an opt-in shell-only self-capture, isolated preview configs,
  standalone TTY test entry, experimental session staging and an explicit
  installer. Legacy Rust/egui code/configuration and the running host session
  remain intact. Updated project rules/docs to distinguish legacy checklist
  acceptance from the Flutter port.
- **Files/components:** `flutter_shell/`, `denial/`, `tools/win10x-denial`,
  `packaging/win10x-denial-session`, `packaging/win10x-denial.desktop`,
  `packaging/install-denial-session.sh`, `README.md`, `AGENTS.md`, `.gitignore`,
  `docs/denial-migration.md`, interaction/testing docs and lessons learned.
- **Verification:** Formatted the new Dart files; matching-kit analysis passed
  for our UI/tests and generated application. All 9 pure-Dart tests passed,
  covering legacy import/independent settings, unavailable apps/orphan groups,
  identity matching, mixed-size Metro packing, immutable narrow-screen geometry,
  nearest-group drops and reversible local collision previews. The release
  bundle built; native `denial-app --check`, bundle checksums and all 26 UI icon
  codepoints passed. Inspected actual Wayland UI for Windows 10, Windows Phone
  (390×780 logical) and Windows 8 layouts, plus Action Center. Corrected a
  visually discovered stale icon-subset defect and confirmed the repaired media,
  Wi-Fi, Bluetooth, volume, battery and power icons with shell self-capture.
  Runtime output showed no Flutter layout/exception diagnostics in the sampled
  settled paths. Python compile and Bash syntax checks passed. Staged a runtime
  package, installed it only under `target/denial/install-check` with `DESTDIR`,
  and verified the launcher, desktop entry, Settings symlink, custom bundle and
  upstream license notices. Confirmed the session helper rejects invocation
  inside the active graphical desktop. Legacy Cargo checks were not required
  because no legacy Rust code changed; the earlier offline attempt lacked its
  crate cache. No host system install, commit/push, full native compositor seat
  test, ARM64 build or real touch acceptance was performed.
- **Status/follow-up:** A runnable, packageable migration baseline is implemented;
  **the overall migration remains in progress**. Full native split/focus/input
  acceptance is deferred to the user's later local session. Remaining code
  parity includes precise optional Disco/Factor launches and joined page/header
  behavior, default full-output client-state policy, complete personalization/
  connectivity detail flows, ARM64 inputs and distribution service integration.
  See the explicit open checklist in `docs/denial-migration.md`; do not treat the
  old checked checklist or this successful build as acceptance of those items.

### 2026-11-10 — touch gestures, Start close, Action Center merge, WP status bar

- **Request:** Continue the Denial/Flutter port: add tablet/Windows Phone style
  edge gestures for notifications/control center, restore Start close animation,
  merge Denial's left control-center controls into our Action Center with Win10X
  styling, and add a Windows Phone–like top status bar.
- **Change:** Start now reverses its own entrance on ordinary dismiss (420 ms)
  while the prepared host stops fading the whole launcher away and keeps the
  scene for that exit. Action Center is the native dashboard content (right slide,
  flat material) and absorbs brightness/power-profile controls from Denial's left
  dashboard into the Win10X quick-action row. Taskbar clock/tray opens that same
  dashboard. Edge strips plus published native edge-swipe regions open Action
  Center from left/right/top; a thin WinPhone-style status bar sits above windows
  and also opens the panel. Hover edge triggers are disabled so they no longer
  fight deliberate gestures.
- **Files/components:** `flutter_shell/lib/src/{start,action_center,gestures,status_bar,taskbar}.dart`,
  `flutter_shell/lib/denial.dart`, `denial/{main,shell_chrome,desktop_overrides}.dart`,
  `tools/win10x-denial`, `docs/denial-migration.md`, `docs/development-log.md`.
- **Verification:** `tools/win10x-denial analyze` and `test` (9/9) passed;
  `bundle` verified ABI/checksums and 30 UI icon codepoints. Preview self-captures
  for Action Center and Start+status bar were inspected. Full native touch-seat
  acceptance remains deferred.
- **Status/follow-up:** Code complete for this request; awaiting user visual
  confirmation of Start close reverse, edge swipes and status bar. Preview left
  running with Start open.

### 2026-11-10 — restore Start/tile animation parity with legacy Rust

- **Request:** Compare the Flutter Start against the old nested Rust compositor;
  animations looked completely different and tile logic/animations were missing.
- **Change:** Root causes: entrance used a scrubbed `_entrance*1.8` clock with a
  `time < 0.4` gate so close barely played; W10M transformed per-group instead of
  board-wide joined rows; Disco/Metro/Factor were single group fades; drag end
  cancelled drops that missed the DragTarget. Added `start_motion.dart` with the
  legacy constants/easing, style-correct open/close durations, board-wide W10M
  row layers (91% scale, shared origin, reverse row order on close), per-tile
  Disco/Metro/Factor delays, flutter_metro group centers, OutBack-ish neighbor
  settle, drag commit-on-release, and icon sizing at the 45% slot. Host launcher
  close hold extended to 1100 ms.
- **Files/components:** `flutter_shell/lib/src/{start,start_motion}.dart`,
  `flutter_shell/test/model_test.dart`, `tools/win10x-denial`, docs.
- **Verification:** analyze + 10 tests; bundle in progress / pending confirm.
- **Status/follow-up:** Core timing/row/drag regressions fixed; Disco still lacks
  the full two-layer 3D mesh and Factor launch still uses a provisional fade.
  Awaiting user visual confirmation against the legacy shell.

### 2026-11-10 — Start origin, drag/resize, app-list fade, status-bar layout

- **Request:** Start open/close scaled from the top-left instead of the middle;
  tile resize/drag and related animations were incomplete; the app list did not
  fade with Start open/close; WP status clock should be centered; status bar had
  no content inset and covered chrome.
- **Change:** W10M joined rows now scale about the board horizontal center at
  the row top (full-width layer + Alignment(0,-1), matching Rust
  `area.center().x`). flutter_metro/Factor use center alignment; Win8 launch
  retreat uses the board bounding-box center. Replaced Draggable with an 8 epx
  pan grab, pointer-following float overlay, snapshot-based freeform/Metro
  previews (correct group origin snap), OutBack neighbor settle, and OutBack
  drop settle. Menu resize resolves freeform collisions via
  `resolveFreeformResize`. App list opacity tracks scene×launch. Status bar
  clock is centered; Start/Action Center/Task View/menus reserve the 28 px top
  strip.
- **Files/components:** `flutter_shell/lib/src/{start,status_bar,action_center,taskbar,model}.dart`,
  `flutter_shell/test/model_test.dart`, docs.
- **Verification:** `tools/win10x-denial analyze` clean; `test` 11/11; `bundle`
  ABI/checksums and 30 UI icon codepoints verified. Preview restarted with Start
  open for visual check.
- **Status/follow-up:** Awaiting user visual confirmation of origin, drag/resize,
  app-list fade, centered clock, and status-bar insets.

### 2026-11-10 — fix Start tile drag follow and avoidance

- **Request:** Tile drag and avoidance were still broken after the origin fix;
  open/close animation was already acceptable.
- **Change:** Flat per-tile W10M/interactive stacks so avoidance can animate
  across rows; raw 8 epx pointer grab with scroll lock; Rust-like nearest-free
  collision preview from the drag-start snapshot; regression test for
  vacated-cell and restore behavior.
- **Files/components:** `flutter_shell/lib/src/{start,model}.dart`,
  `flutter_shell/test/model_test.dart`, `docs/lessons-learned.md`,
  `docs/development-log.md`.
- **Verification:** analyze clean; test 12/12; bundle ABI/checksums/icons OK;
  preview restarted with Start open.
- **Status/follow-up:** Awaiting user visual confirmation of drag follow,
  neighbor ease-out/back, and drop snap.

### 2026-11-10 — Win10 Fluent skin and legacy tile resize mode

- **Request:** Replace Material-looking Flutter chrome with Windows 10 Fluent 1
  (not Win11 Sun Valley), using `/data/Win10UI` tokens and little/no acrylic;
  fully restore Rust-parity Start tile size adjustment.
- **Change:** Added `Win10xFluent` light ThemeData (square corners, `#0078D4`,
  opaque `#f3f3f3` chrome) and wrapped Start/taskbar/status/Action Center.
  Tile context menu again uses 「更改大小」to enter resize mode with in-tile
  size guides, bottom-right handle drag (`size_from_pointer` / Win8 wide
  toggle), neighbor resolve, and background tap to confirm—matching
  `shell/src/start_tiles.rs` rather than discrete 小/中/宽/大 menu picks.
- **Files/components:** `flutter_shell/lib/src/{fluent,start,model,controls,action_center,taskbar,status_bar}.dart`,
  `flutter_shell/lib/denial.dart`, `denial/shell_chrome.dart`, tests, docs.
- **Verification:** analyze clean; test 13/13; bundle ABI/checksums/icons OK;
  preview restarted with Start open.
- **Status/follow-up:** Awaiting user visual confirmation of Fluent chrome and
  corner-handle resize. Acrylic redesign remains deferred.

### 2026-11-10 — real Fluent Start menus, not Material tiles

- **Request:** Start settings still looked like purple Material radios/switches;
  Win10/Phone/8 Start menus should be near the pointer or a full bottom bar.
- **Change:** Denial popups sit outside the launcher Theme, so Material
  RadioListTile/SwitchListTile kept the host purple seed. Replaced them with
  painted Win10UI Fluent radio/toggle/menu rows (`fluent_controls.dart`), wrap
  every popup in `Win10xFluent.wrap`, put Start settings on a full-width bottom
  command bar, and anchor tile/app context menus at the pointer. Taskbar command
  menus use the same Fluent surface.
- **Files/components:** `flutter_shell/lib/src/{fluent_controls,start,taskbar}.dart`,
  docs.
- **Verification:** analyze clean; test 13/13; bundle OK (28 UI icons).
  Preview restarted with Start open.
- **Status/follow-up:** Awaiting user confirmation of bottom settings bar and
  near-pointer tile menus.

### 2026-11-10 — Fluent control corrections and legacy motion policy

- **Request:** Review Cursor's Fluent UI work, correct inconsistent controls and
  behavior, remove general bounce, reuse the previous Rust motion and add
  nonlinear easing where transitions were linear.
- **Change:** Removed Material ink ripples/overscroll and circular command
  feedback; added a rectangular Fluent slider thumb, square 44 px icon targets,
  keyboard activation/focus/pressed states, animated toggles and measured popup
  positioning. Bounded media titles and made the lower Action Center scroll on
  short screens. Added audio device/application volume and available system,
  CPU and GPU performance details using Denial's managed providers. Corrected
  the settings entry that previously opened Power settings. Shared numerical
  Rust power-2.6 easing across our controls and prepared desktop/window host;
  removed workspace/layout/tile reflow overshoot, added nonlinear popup/panel
  and launch fades, restored source Bézier math and the three-stage Windows 8
  flying tile geometry. Start close lifetime follows the selected staggered
  duration, and focused search uses a dedicated Escape intent. Added opt-in
  preview-local input exercises and fresh-frame capture with a bounded timeout.
- **Files/components:** `flutter_shell/lib/src/{fluent,fluent_controls,controls,
  fluent_motion,motion_math,start_motion,start,launch,action_center,
  system_details}.dart`, `flutter_shell/lib/denial.dart`,
  `flutter_shell/test/motion_test.dart`, `denial/{desktop_overrides,shell_chrome,
  preview_capture}.dart`, `tools/win10x-denial`, migration docs and lessons.
- **Verification:** Matching-kit `analyze` clean for shell/tests and generated
  application; all 17 Dart tests passed. Release `bundle` and `verify` passed
  native ABI, checksums and 28 icon codepoints. Recreated the kit from the locked
  release and ran `prepare` twice to check fresh preparation/idempotence.
  Python compilation and `git diff --check` passed. Refreshed the staged runtime
  package without installing it. Inspected actual Wayland captures of phone
  settings, Action Center, long-title media, landscape scroll clipping, left/
  right/top gesture entry and intermediate Start close frames. A refreshed
  boundary capture recovered a stale initial black scene during QA. The final
  audio-detail screenshot exercise remained unaccepted: the host withheld a
  requested frame and the new bounded wait reported an 8-second timeout;
  no successful audio-detail screenshot or hardware mutation is claimed.
  Preview-focus scripts were temporary and unloaded. No legacy Rust tests were
  needed because legacy code was unchanged. No live-session switch, host
  install, ARM64 build, commit or push was performed.
- **Status/follow-up:** Code changes and build checks complete; visual judgment,
  the new audio/performance detail flows and real touch/native window motion
  remain for the user's local Denial-session acceptance. The overall migration
  and standalone upstream Settings restyling are not declared complete.

### 2026-11-11 — researched tablet-first Windows 10 / Mobile shell modes

- **Request:** Research Windows 10 Mobile in BetaWiki and original UI/interaction
  material, create a dedicated phone mode, and correct Windows 10 styling.
  The user clarified that both tablet and ordinary desktop are required, with
  tablet implemented first.
- **Change:** Recorded version-specific BetaWiki metadata, original Microsoft
  Lumia reviewer/manual evidence and Microsoft tablet/Start documentation in
  `research/windows10-modes/README.md` before implementation. Added persisted
  tablet/desktop/phone selection independent from tile layout/animation choices.
  Tablet has full-screen pinned/all-apps pages, left rail, multi-column catalog,
  left-aligned Back/Search/Task View, separate calendar/volume/notification
  buttons and distinct left/right/top edge commands. Desktop has a lower-left
  bounded Start, app list with tiles, real running/pinned buttons and flyouts.
  Phone has six/eight-column tiles, dark app page/search/letter jump, hold/edit
  tile controls, left connectivity/right battery-time, Back/Start/Search,
  long-Back live tasks and long-Start timed reachability with legacy nonlinear
  easing. Phone Action Center drops from the top with quick actions above grouped
  notifications; PC modes retain the right-side panel and bottom quick actions.
  Added real notification dismissal/actions, category expansion and mode quick
  action. Native mode policy maximizes primary touch clients while preserving
  desktop rectangles, transient dialogs, explicit fullscreen and managed tablet
  tiling. Work-area reservations and reachability drawing/input share geometry.
  Host chrome is square, opaque and blue. Hardened prepared-source regeneration
  and atomic immutable bundle publication after a reproduced SIGBUS. Updated
  readme, migration/acceptance/testing docs and append-only lessons.
- **Files/components:** `flutter_shell/lib/{denial.dart,src/}` mode/navigation,
  Start/catalog, action center/notifications, taskbar/calendar, phone controls,
  status/gestures and Fluent tokens; `flutter_shell/test/modes_test.dart`;
  `denial/{main,shell_chrome,desktop_overrides,preview_capture}.dart`;
  `tools/win10x-denial`, `tools/test_denial_bundle.py`; research and docs.
- **Verification:** Matching-kit analysis passed for shell/tests and generated
  application; all 24 Dart tests passed, including six/eight-column projection,
  mode persistence, saved-layout independence, cross-mode bounds and disjoint
  responsive groups. Python mmap/asset publication regression passed; Python
  compilation and repeated `prepare` passed. Release bundle and native engine
  ABI/checksums/icon coverage passed. Inspected actual Wayland self-captures of
  tablet full-screen Start/all-apps, phone Start/apps/top shade/hold-Start
  reachability, hold-Back task surface, desktop bounded Start and a real tap on
  the tablet quick action switching to desktop. The first modal task view was
  corrected to retain its navigation; native session behavior remains separate
  acceptance. Staged the runtime package without a host install. No Microsoft
  research images/fonts were added to shipped assets. No Rust legacy code,
  current login session, user desktop configuration, commit or push was changed.
- **Status/follow-up:** Three-mode shell implementation is available for user
  visual review, with tablet first. This is not a pixel-perfect Windows release:
  native back/maximize/restore/touch handoff, exact two-app Snap policy and real
  client task cards still need local Denial-session acceptance. Phone navigation
  auto-hide, full quick-action reorder/inline reply editors, Live Folders,
  language-specific alphabet indexing and complete standalone Settings restyling
  remain explicit fidelity gaps. Authentication/OOBE and ARM64 remain deferred.

### 2026-11-11 — final three-mode preview and package verification

- **Request:** Continue the tablet-first implementation and verify the resulting
  tablet, desktop and phone interfaces.
- **Change:** Completed final preview acceptance for the modal phone navigation
  fix and responsive tablet layout; staged the latest matching runtime package.
  Left the current 1280 × 800 tablet preview running for user inspection.
- **Files/components:** Generated preview captures under `target/denial/qa/`,
  runtime generation `4e2aa0ec5afb-ray11ji4`, and `packaging/out/denial`.
- **Verification:** Fresh `tools/win10x-denial test` passed all 24 Dart tests;
  `verify` passed checksums, native ABI and UI icon coverage;
  `python3 tools/test_denial_bundle.py` and `git diff --check` passed. Package
  staging succeeded without installation. Visually inspected final captures of
  tablet portrait (800 × 1280) and landscape (1280 × 800), bounded desktop Start,
  and phone hold-Back task view with visible status/navigation and its honest
  zero-window state. Preview processes used the current immutable generation;
  completed short-lived captures were stopped by their recorded child PID.
  Desktop capture logged one engine frame-time clamping warning; no Flutter
  layout exception was observed. No real client task-card behavior was tested.
- **Status/follow-up:** Ready for user visual review. Native session input,
  window activation/back, touch maximization/restoration and two-app Snap remain
  unaccepted; the scoped fidelity gaps in the preceding entry still apply.

### 2026-11-11 — phone navigation auto-hide and keyboard reachability

- **Request:** Continue Codex phone adaptation from the documented fidelity
  gaps after reading Codex/docs context.
- **Change:** Persisted `phone_nav_auto_hide` (default on). Leaving Start/Action
  Center schedules a hide of the Back/Start/Search surface and clears the work-
  area reservation; bottom swipe or Back/Start/Search/home reveals it again.
  Soft keyboard exits reachability and pads Start/Action Center. Restored the
  centered W10M status clock with connectivity left and battery percent right.
  Mode settings expose the auto-hide toggle. Host rebuilds surfaces when chrome
  visibility changes; Denial input/scene listeners also watch the hide flag.
- **Files/components:** `flutter_shell/lib/{denial.dart,src/{model,shell_navigation,gestures,status_bar,mode_settings,start,action_center}.dart}`,
  `flutter_shell/test/modes_test.dart`, `denial/{main,desktop_overrides}.dart`,
  `tools/win10x-denial`, migration docs.
- **Verification:** Matching-kit analyze clean; Dart tests 25/25. Release
  bundle verified checksums/ABI and 29 UI icons. Phone Start capture shows
  centered clock with connectivity left and battery percent right. Forced
  `phone-nav-hide` exercise logged `hidden=true` with nav fade 0 and wrote
  `target/denial/qa/phone-nav-autohide-hidden-20261111.png`. Interactive phone
  preview left running for user judgment.
- **Status/follow-up:** Awaiting user visual confirmation of auto-hide/reveal
  and status clock. Live Folders, quick-action reorder, language alphabet
  indexing and native-session acceptance remain open.

### 2026-11-11 — keep phone navigation visible with apps

- **Request:** Phone mode must not hide Back/Start/Search while an application
  is open; the previous auto-hide path incorrectly dismissed it.
- **Change:** Removed phone navigation auto-hide preference, hide/reveal
  scheduling, bottom reveal strip and work-area release. Phone chrome stays
  reserved and visible with apps. Kept keyboard-aware reachability and the
  centered status clock.
- **Files/components:** `flutter_shell/lib/{denial.dart,src/{shell_navigation,gestures,model,mode_settings}.dart}`,
  tests, `denial/{main,desktop_overrides,preview_capture}.dart`,
  `tools/win10x-denial`, docs.
- **Verification:** analyze clean; tests 24/24; bundle ABI/checksums/29 icons
  OK. Phone preview relaunched on the new generation for visual check with apps.
- **Status/follow-up:** Awaiting user confirmation that navigation stays while
  apps are open.

### 2026-11-11 — install current Denial shell on Cerro phone for native test

- **Request:** Install the complete desktop onto the user's Cerro phone and
  test on-device, following the prior Codex `/opt` + VT acceptance path.
- **Change:** Rebuilt the matching ARM64 bundle with the phone as AOT host
  (`WIN10X_ARM64_HOST=cerro-usb` / USB ECM 192.168.68.2). Staged checksummed
  package, installed generation `5e32b208cc04-arm64-20261111` under
  `/opt/win10x-denial` with `current` symlink, phone wrapper and DM entry.
  Started a temporary VT8 native session (avoiding the known VT2/getty
  conflict) with 240s RuntimeMaxSec plus independent rollback to Plasma VT1.
- **Files/components:** `tools/win10x-denial-arm64`,
  `packaging/out/denial-arm64`, phone `/opt/win10x-denial`,
  `/usr/local/bin/win10x-denial-phone-session`, VT8 test unit;
  capture `target/denial/qa/cerro-phone-native-start-20261111.png`.
- **Verification:** ARM64 libapp/engine machine `0xb7`; deniald started Flutter
  at 1116×2480 @2.5 on DSI-1; Start capture shows phone mode tiles, centered
  clock, and persistent Back/Start/Search. Session left active for user touch
  acceptance until auto-rollback.
- **Status/follow-up:** Awaiting on-device app launch / navigation judgment.
  Non-fatal warnings: systemd user-environment snapshot and i2c DDC EACCES.

### 2026-11-11 — restore Cerro VT8 and finger-follow Start page swipe

- **Request:** Desktop fell back to KDE; restore it. Compare Flutter vs legacy
  Rust animations. Touch+animation feel stiff / not finger-following; study
  open-source WinPhone launchers (ViewPager-style).
- **Change:** Hardened the VT8 unit: removed `ExecStopPost=chvt 1` (that dumped
  users to Plasma on any stop), dropped RuntimeMaxSec, added `Restart=on-failure`.
  Start↔All Apps page push now tracks the finger while dragging (linear page
  progress) and settles with 340 ms easeOutExpo, matching MetroLauncher's
  ViewPager2 pattern rather than release-then-animate. Redeployed ARM64
  generation `8144e7850f5f-arm64-20261111` and restarted the native session.
- **Files/components:** `flutter_shell/lib/src/start.dart`; Cerro
  `win10x-phone-test.service` / `win10x-phone-start-persistent.py`;
  `/opt/win10x-denial/8144e7850f5f-arm64-20261111`.
- **Verification:** analyze clean; tests 24/24; host + ARM64 bundles OK.
  Session active on tty8 with deniald on the new generation. Visual/touch
  acceptance left to the user.
- **Status/follow-up:** Awaiting on-device judgment of page swipe feel. Remaining
  feel gaps (Action Center edge strips still End-only; app-switch snapshot
  handoff not ported; tile settle uses easeDecelerate instead of OutBack) are
  listed in the chat reply.

### 2026-11-11 — WP parity docs, Start toggle, theme, icons

- **Request:** Write Flutter↔Rust animation gaps to md; remaining gaps must
  match Rust (optimize stiff ones); add MetroLauncherAndroid to AGENTS and
  port its WP features; switch icons to 马克笔UI flat pack; fix phone Start
  toggle; WP Start wallpaper + light/dark/accent settings; WP popup style.
- **Change:** Added `docs/flutter-vs-rust-animation-gaps.md` and
  `docs/metrolauncher-android-parity.md`; updated `AGENTS.md`. Phone Start
  second press closes Start; wallpaper shows through a theme veil; start+theme
  dark/light + accent prefs; tile transparent/color menus; double-tap opens
  All Apps; bottom-bar menus slide up; nav glyphs from 马克笔UI navbar APK;
  tile settle uses `easeOutBack`.
- **Files/components:** AGENTS/docs; `flutter_shell` model/start/fluent/
  symbols/mode_settings/shell_navigation/phone_chrome/denial/assets/wp_icons.
- **Verification:** analyze + tests after fix; bundle in progress. Full
  MetroLauncher live-tile/media and Rust Disco/Factor/app-switch ports remain
  tracked in the gap docs.
- **Status/follow-up:** Deploy/visual acceptance pending; continue A1–A7/A9
  and live-tile parity.

### 2026-11-11 — redeploy WP-parity shell to Cerro VT8

- **Request:** Phone reconnected; install the latest build.
- **Change:** Built ARM64 `47a735146a75`, staged/rsync’d, installed under
  `/opt/win10x-denial/47a735146a75-arm64-20261111`, restarted hardened VT8
  unit (Restart=on-failure, no chvt1 on stop).
- **Files/components:** phone `/opt/win10x-denial/current`,
  `win10x-phone-test.service`.
- **Verification:** session active on tty8; deniald Flutter 1116×2480 @2.5.
  Non-fatal: fuse doc portal, DDC i2c EACCES, systemd env snapshot warn.
- **Status/follow-up:** Awaiting on-device check of Start toggle, wallpaper
  veil, theme/accent, and page swipe feel.

### 2026-11-11 — fix Start flash-close on open

- **Request:** Start button opens Start then immediately closes (user report).
- **Change:** `StartRequest.home` toggle-close only on non-force navigation
  updates; open-time force sync no longer dismisses.
- **Files/components:** `flutter_shell/lib/src/start.dart`;
  `docs/lessons-learned.md`; this log.
- **Verification:** `tools/win10x-denial analyze`; preview `--mode phone
  --start --capture target/denial/qa/start-stays-open-20261111.png` still
  shows Start tiles after 2.5 s.
- **Status/follow-up:** Complete for the flash bug; ARM64 phone redeploy of
  the full parity build still pending sudo on Cerro.

### 2026-11-11 — remaining-work breakdown + Goal

- **Request:** Subdivide all unfinished work (including previously deferred
  items; acrylic must not restore the old stack), require full WinPhone
  popups/menus with restored animation, complete desktop Settings, write a
  Goal, then implement everything to full usability with tests.
- **Change:** Added `docs/remaining-work-breakdown.md` with numbered feature
  rows (Metro/Live Tile, WP ApplicationBar/MessageBox/toast, animation
  verify, Settings S*/DS*, session SS*, dual-screen DU*, acrylic AY*,
  packaging). Created a Cursor Goal pointing at that document.
- **Files/components:** `docs/remaining-work-breakdown.md`, this log.
- **Verification:** not run (documentation / Goal creation).
- **Status/follow-up:** Implementation starts with WP popup kit then Settings.

### 2026-11-11 — WP popup kit + uninstall + gap pack (Goal kickoff)

- **Request:** Subdivide all remaining work into MD, create Goal, implement
  fully (WP menus/dialogs with restored motion, desktop settings, remaining
  Metro features, previously deferred scopes; no old acrylic).
- **Change:** Added `docs/remaining-work-breakdown.md` + Goal; AGENTS pointer.
  Shipped `WpMotion` / `WpPopups` ApplicationBar + MessageBox with logarithmic
  enter / exponential exit; phone Start menus route through ApplicationBar;
  toast uses same curves; App Info + uninstall via MessageBox + system tools;
  `packTilesTopLeft` (MetroLauncher GridPacker) on phone unpin; Reset Start in
  mode settings.
- **Files/components:** `flutter_shell/lib/src/{wp_motion,wp_popups,wp_toast,
  app_actions,model,mode_settings,start}.dart`, tests, docs, AGENTS.
- **Verification:** `tools/win10x-denial analyze` + `test` (incl. packTilesTopLeft).
- **Status/follow-up:** Goal continues — live-tile media/messaging/flip, full
  desktop Settings wiring, ApplicationBar landscape, session SS*, dual-screen
  DU*, new acrylic AY2.

### 2026-11-11 — Goal batch: live tiles, settings, session, dual-screen, acrylic

- **Request:** Finish remaining-work breakdown items including previously
  deferred session/dual-screen; full WP popups; complete desktop settings;
  no old acrylic; research before inventing; all features usable and tested.
- **Change:** Disco 1s live-tile Y-flip + MPRIS media mini-player faces;
  pack on resize/drop; auto accent from host; drawer/tile font sizes; Start
  image picker; expanded mode settings (layout/animations/acrylic); ApplicationBar
  landscape dock; action toasts; power MessageBox; Denial `ShellBackdropBlur`
  acrylic veil; lock screen + OOBE surfaces; dual-screen geometry + Wonder Bar;
  wire `ShellNavigation.services` for Start media.
- **Files/components:** `flutter_shell/lib/src/{live_tile,acrylic,dual_screen,
  session_lock,wonder_bar,start,model,mode_settings,wp_popups,wp_toast,
  action_center,app_catalog,denial}.dart`, `denial/shell_chrome.dart`, tests,
  `docs/remaining-work-breakdown.md`.
- **Verification:** `tools/win10x-denial analyze` + `test` (29 passed) +
  `bundle` + phone preview capture
  `target/denial/qa/goal-batch-live-settings-20261111.png`.
- **Status/follow-up:** Goal still open for album-art bytes, dual-output host
  wiring / cross-app DnD, greetd seat login, Disco app-list exit polish, and
  user visual acceptance of motion.

### 2026-11-11 — Disco app-list exit + media art

- **Request:** Continue Goal remaining items (no simplified motion).
- **Change:** Ported Rust `disco_app_list_frame` (0.2 s / selected 0.3 s delay,
  cubic-bezier(0.75,0,1,0)) into All Apps launch exit; media tiles load
  `file://` / http album art when MPRIS supplies `artUrl`.
- **Files/components:** `start_motion.dart`, `app_catalog.dart`, `start.dart`,
  `live_tile.dart`, remaining-work MD.
- **Verification:** `tools/win10x-denial analyze` + `test` (29 passed).
- **Status/follow-up:** Dual-output compositor DnD and greetd seat login still
  need host/hardware; user visual QA of Disco list exit and live flip.

### 2026-11-11 — Gap close: AB icons, Settings deep-links, AC media, DU5 shell DnD

- **Request:** Continue Goal — finish remaining-work partials; full WP menus;
  complete desktop Settings; no old acrylic; research before inventing; test.
- **Change:** All Apps ApplicationBar mini icons (pin/info/uninstall); Action
  Center media card prev/play/next; mode-settings deep-links to Denial
  Appearance / Animations / Lock screen; CrossAppDrag pane hit-test against
  DualScreenController; Action Center lock/appearance icons use existing
  MaterialIcons subset glyphs; remaining-work statuses advanced for AB/CM/MB/T,
  L6, DS2/DS8/DS11, AC2–AC4, SS4, DU5 shell half.
- **Files/components:** `start.dart`, `action_center.dart`, `mode_settings.dart`,
  `dual_screen_host.dart`, `docs/remaining-work-breakdown.md`.
- **Verification:** `tools/win10x-denial analyze` (clean) + `test` (29 passed) +
  `bundle`; Cerro arm64 blocked (host unreachable).
- **Status/follow-up:** Goal still open for compositor dual-output DnD (DU5),
  posture sensors (DU4), greetd seat login (SS2), and Cerro when online.

### 2026-11-11 — Dual-screen span/posture + Denial lock login polish

- **Request:** Close remaining SS2 / DU2 / DU4 / DU5 from remaining-work breakdown.
- **Change:** Dual-screen Extended Canvas spans focused window via
  `DenialBridge.configureWindow`; posture inferred from output geometry;
  cross-pane window handoff via `moveWindowToOutput`; mode picker in shell
  settings; lock screen PIN/password/fingerprint + large unlock targets
  (Denial PAM). Pre-seat greetd greeter remains host-owned.
- **Files/components:** `dual_screen.dart`, `dual_screen_host.dart`,
  `session_lock.dart`, `mode_settings.dart`, `denial/shell_chrome.dart`, tests,
  remaining-work / Metro parity docs.
- **Verification:** `tools/win10x-denial analyze` + `test` (31) + `bundle`;
  Cerro still unreachable.
- **Status/follow-up:** Goal open only for PK2 arm64 when Cerro returns;
  optional hinge sensors / wl_data client DnD are host upgrades beyond shell.

### 2026-11-11 — Full Denial SettingsPageId deep-links (DS1–DS20)

- **Request:** Complete desktop Settings wiring end-to-end for Goal audit.
- **Change:** `SettingsLaunch.allPages` mirrors Denial `SettingsPageId` (20
  pages); shell mode settings lists every page; Action Center adds Displays /
  Network / Bluetooth Settings deep-links; remaining-work DS* rows updated;
  PK2 remains blocked (Cerro down; stale prior ARM64 AOT).
- **Files/components:** `settings_launch.dart`, `mode_settings.dart`,
  `action_center.dart`, `docs/remaining-work-breakdown.md`,
  `docs/flutter-vs-rust-animation-gaps.md`.
- **Verification:** analyze clean, test 31, bundle OK.
- **Status/follow-up:** Goal open for PK2 when Cerro returns; user visual QA.

### 2026-11-11 — Local qemu ARM64 AOT + Reset Start seed

- **Request:** Unblock PK2 while Cerro is offline; finish Reset Start (W5).
- **Change:** `tools/win10x-denial-arm64` accepts `WIN10X_ARM64_HOST=qemu` and
  runs aarch64 `gen_snapshot` under `qemu-aarch64-static` +
  `aarch64-linux-gnu` root; built and packaged current-source ARM64 session.
  `resetStartBoard` reseeds one「开始」group from taskbar pins (all modes).
  WP log/exp motion assertion added to motion tests.
- **Files/components:** `tools/win10x-denial-arm64`, `model.dart`,
  `mode_settings.dart`, tests, remaining-work MD.
- **Verification:** analyze clean; test 33; `WIN10X_ARM64_HOST=qemu … bundle`
  → aarch64 `libapp.so`; `package` staged under `packaging/out/denial-arm64`.
- **Status/follow-up:** Push to Cerro when the phone is back on USB/Wi-Fi;
  Goal remains for on-device confirm.

### 2026-11-12 — Cursor changes: functional and animation review, corrective pass

- **Request:** Test Cursor's additions, find differences from Windows 10 / Windows
  Phone behavior, fix UI/animation bugs, and verify whether features really work.
- **Change:** Corrected left-edge Task View routing; cancelled/stale dashboard
  settles; stable deferred popup controllers; MessageBox result completion on
  Back/removal (including pre-build removal); body-owned exit timing and phone
  navigation above modal menus. Replaced modal feedback with a timed non-modal
  toast strip. App-switch proxies no longer acquire/restore focus or cover shell
  bars. Removed uninstall fallbacks that could silently run terminal removal or
  launch the desktop entry; corrected the system-accent setting's misleading
  icon-extraction label. Fixed light-theme All Apps link contrast, lock entrance
  progress evaluation, page-aware PC Start presses and automatic home toggles.
  Corrected Disco face/content perspective against Rust and added projection
  tests. Phone classic/compact density is 4/6 per the newly selected launcher
  reference, using full-width shared cells without rewriting stored tile cells.
  Appended factual acceptance corrections for settings/login/dual-screen gaps.
- **Files/components:** `flutter_shell/lib/src/{gestures,dashboard_reveal,
  wp_popups,wp_toast,app_switch,app_actions,phone_chrome,start,taskbar,
  mode_settings,model,session_lock,disco_projection,disco_turnstile}.dart`;
  `tools/win10x-denial` prepared popup-host policy;
  `denial/{shell_chrome,preview_capture}.dart`;
  projection/mode tests; parity, motion, remaining-work, testing/lessons docs.
- **Verification:** Initial analyze and 33 tests passed before the review, despite
  the functional bugs. After fixes, matching-kit analyze and 35 tests passed
  (two added Rust-reference projection tests). Actual intermediate-preview
  exercises already checked menu open/close, Back and forced dialog cancellation,
  landscape long-text clipping, non-modal toast routing, phone apps/top shade,
  tablet left-edge input and bounded desktop Start. Final source build and
  expanded current-generation preview/ARM64 checks are recorded in the next
  appended entry, rather than treating earlier images as current acceptance.
- **Status/follow-up:** Corrective review in verification. Latest-source native
  activation/input and hardware backends remain unaccepted: Cerro is absent
  from host USB enumeration and the previous USB/Wi-Fi SSH routes are unreachable.
  Existing Cursor preview processes and unrelated work were preserved. No
  kernel/boot/partition/radio change, system session switch, commit or push was
  performed in this review. Overall product completion is not claimed.

### 2026-11-12 — current-generation visual regression results

- **Request:** Verify the corrective review on current code and prepare the
  matching ARM64 desktop for the user's phone.
- **Change:** Added opt-in actual-rendering exercises for popup lifetime,
  immediate cancellation, page-aware Start toggles, non-modal feedback and an
  interrupted panel settle. Added ARM64 icon-font coverage gates and invalidated
  stale ARM64 assembly assets after reproducing missing media glyphs.
- **Files/components:** `denial/preview_capture.dart`,
  `tools/win10x-denial-arm64`, `target/denial/qa/audit-20261112/` (ignored QA
  outputs), README/migration/testing/parity/remaining-work documentation.
- **Verification:** Current x86_64 source `c79511384aa8` passed pinned-kit analyze,
  all 35 Dart tests, release bundle and native ABI/checksum/41-icon verification.
  Eighteen actual Wayland preview cases captured successfully with no detected
  Flutter exception/layout overflow: phone menu, popup lifecycle, landscape long
  dialog, desktop feedback toast, tablet left-edge input (empty-window target),
  short phone top-edge cancellation, phone apps, desktop Start, phone/desktop
  Start close-reopen-apps-return, interrupted panel settle, phone light theme,
  Disco entrance and an in-close frame, tablet/desktop narrow layouts, phone
  and tablet Action Centers. Multi-step cases explicitly asserted their state;
  empty previews do not establish native Task View/app activation. Inspected
  current captures, including modal phone navigation and light-theme contrast.
  A short 120 px top pull correctly cancelled; full shade was separately
  inspected using the Action Center startup entry. Font gate rejected the first
  current-source ARM64 bundle because its separately cached OTF lacked
  skip_previous/skip_next; fresh-asset rebuild results are appended below.
  Python compilation, bundle-publication regression and git diff whitespace
  checks passed. Left only this task's reviewed phone preview running; unrelated
  Cursor previews were preserved. One engine frame-time clamping warning
  (~1.2 ms) appeared in that preview; no application layout exception appeared.
- **Status/follow-up:** Review fixes available for visual judgment. Phone
  reconnection remains pending; no current-source phone install or native
  window/radio/authentication acceptance is claimed. Exact original-Windows
  animation timing, genuine icon palette extraction, upstream Settings restyle,
  unmounted login/OOBE composition and complete dual-screen/client data drag
  remain explicit gaps rather than silently accepted functionality.

### 2026-11-12 — ARM64 icon regression fixed and current package verified

- **Request:** Finish the current review's ARM64 package verification.
- **Change:** Rebuilt fresh ARM64 assets with the matching pinned engine and
  qemu AOT, then staged the corrected session. Missing media glyphs now block
  both publication and packaging instead of being silently shipped.
- **Files/components:** `tools/win10x-denial-arm64` and generated
  `packaging/out/denial-arm64`; immutable bundle
  `c79511384aa8-3iem9_x9` (source matches the reviewed x86_64 generation).
- **Verification:** The new package gate first rejected the previous ARM64 OTF
  for missing skip_next/skip_previous. Fresh build and package then passed
  41-icon coverage. Independently checked current source/builder hashes, AOT
  and engine checksums, ARM64 ELF machine IDs, staged metadata and staged font
  coverage. Re-ran x86_64 native bundle/ABI/font verification and Python compile
  checks. No phone connection, native ARM64 execution, install or session switch
  occurred in this corrective review.
- **Status/follow-up:** Local build/package verification complete; current phone
  preview remains open. Device-side acceptance is pending reconnection; all
  previously recorded product/visual gaps remain explicit.

### 2026-11-12 — default phone desktop, KDE login and MarkUI coverage

- **Request:** Install the reviewed ARM64 desktop as the default; enable KDE's
  login page; fix oversized login controls; complete missing MarkUI shell icons.
- **Change:** Installed reviewed c79511384aa8 natively with a separate phone
  wrapper/private D-Bus. Enabled existing SDDM/Breeze/KWin/Maliit, disabled
  Plasma autologin, selected Win10X, retained Plasma. Changed greeter scale
  2.0→1.25; user confirmed the corrected sizing/buttons. Added shared MarkUI
  application aliases and command/status rendering across our shell, original
  18-level battery images, package-qualified assets and navbar padding handling.
  Added exact source/checksum provenance, bitmap source hashing and both-target
  packaged-PNG checks. Kept explicit WP line adaptations where the pack lacks
  a command; unknown third-party icons and native-client toolbars are separate.
- **Files/components:** packaging/win10x-denial-phone-session;
  docs/phone-session.md; flutter_shell/src markui_catalog/markui_icons/symbols,
  Start/catalog/taskbar/phone cards, Action Center/status/menu/media controls;
  assets/wp_icons; tools/import-markui-icons and both Denial build helpers;
  parity/lessons/README; device SDDM config and versioned /opt generation.
- **Verification:** Native c795 Start and Konsole captures, new observed boot
  87b3a80a and successful real SDDM password login into Win10X tty2; user125%
  confirmation. New MarkUI source705af037af60 passed pinned analyze,39 tests,
  x86_64 and native-ARM64 AOT builds, ABI/checksums,93 packaged PNG checks and41
  icon font checks. Inspected current light Start/app list, plus phone menu,
  phone/tablet Action Center captures; all five captured without detected asset,
  Flutter exception or layout-overflow errors. Initial previews caught opaque
  desktop ID prefix misses and padded navbar size; corrected before current
  generation. Bash/Python syntax and git diff whitespace checks passed.
- **Status/follow-up:** Default desktop/login configuration is live; MarkUI
  package705af037af60 is ready but not yet installed: phone USB disappeared and
  verified Wi-Fi discovery found no matching SSH host. Continue the user's new
  native launch-animation and distorted-client-rendering request before final
  deployment. Native acceptance of that upcoming change remains pending. No
  kernel/modules/boot/dtbo/slot/radio mutations, commit or push were performed.

### 2026-11-12 — own the full launch handoff and prevent client distortion

- **Request:** Fully replace Flutter/Denial application opening with the
  project's launch motion and fix squashed/stretched application contents.
- **Change:** Disable the native random-origin squircle/lead-in so it cannot
  play over the selected Rust-derived Start handoff. Preserve independent Start,
  launch and switch settings. Phone/tablet first placement now configures even
  when initial geometry is null. Shared uniform fitting replaces independent
  X/Y stretch during retained window/popup transitions and committed-buffer
  presentation. Input/popup mapping uses the same fit; ordinary local Flutter
  windows reflow directly, while transformed previews retain layout proportionally.
- **Files/components:** tools/win10x-denial prepared-host patches;
  denial/client_surface_policy.dart; flutter_shell/lib/src/client_geometry.dart;
  client geometry invariant tests; opt-in retained/client fixture in
  denial/preview_capture.dart; animation gaps/testing/lessons documentation.
- **Verification:** Pinned analyze passed;42 Dart tests passed. Current x86_64
  source89b862014481 built with ABI/checksums,93 PNG and41 font-codepoint checks.
  Five current preview cases captured without detected Flutter/asset/overflow
  errors: phone All Apps, desktop Start, light phone Start, phone Action Center,
  client proportions. The actual rendered composition fixture asserted equal
  axis scales and reciprocal coordinates in-animation and settled; inspected its
  circular outline and square grid. Source inspection confirmed native initial
  reveal disabled and geometry-null configure gap removed. ARM64 build/package
  verification is recorded in the next appended result. One engine timestamp
  clamp (~1.1ms) appeared in fixture logs, with no Flutter layout exception.
- **Status/follow-up:** Corrective code and host UI verified; current-source
  native device acceptance remains pending. Phone is absent from USB and known
  USB SSH timed out; local Wi-Fi SSH candidates had no verified phone key.
  Asked for reconnection/current IP and affected application names. No device
  writes occurred after disconnect, and no claim is made that specific native
  applications have been fixed on the phone. Current reviewed preview refreshed;
  unrelated Cursor processes preserved.

### 2026-11-12 — final ARM64 artifact and device reconnection boundary

- **Request:** Prepare the same verified launch/proportion/MarkUI fixes for the
  phone and preserve a usable default phone desktop.
- **Change:** Built/staged matching ARM64 source89b862014481. The phone wrapper
  now seeds only a missing power-button preference to DPMS, retaining explicit
  user settings; native preflight had shown upstream's default PowerOff.
  This wrapper correction is staged separately for the next device connection.
- **Files/components:** packaging/out/denial-arm64; versioned bundle
  target/denial-arm64/bundles/89b862014481-pz3pcro1;
  packaging/win10x-denial-phone-session; docs/phone-session.md.
- **Verification:** ARM64 qemu AOT build/package succeeded;93 packaged PNGs,
  41 font glyph references, native ARM64 ELF/checksums and matching current-source
  hash verified. Fresh x86_64 ABI verification and independent source/AOT/engine
  checksum comparison across both bundles and staged ARM64 passed. Wrapper Bash
  syntax and isolated Python policy exercise passed (missing→DPMS; explicit
  suspend and unrelated appearance preserved). Git whitespace check passed.
  Current review preview627587 captured without Flutter/asset exceptions;
  stopped only obsolete task-owned preview4045393. Other previews untouched.
- **Status/follow-up:** Package ready; native source89b862014481 and revised
  wrapper are not installed. Last verified installed phone source is c79511384aa8,
  with Win10X default / Breeze login and user-accepted125% scale. Phone remained
  absent from USB at final check; no unverified native repair claim is made.
  Reconnect, deploy atomically, then test affected real applications, resize / mode
  changes, popup/touch alignment and short power-key behavior. No reboot, kernel,
  partition/slot/radio write, commit or push was made for the later corrections.

### 2026-11-12 — Chromium identified as the native distortion regression target

- **Request:** User clarified that Chromium and similar applications deform and
  that the phone cannot be connected for now.
- **Change:** Recorded Chromium-specific native acceptance, distinguishing the
  currently unknown Wayland/XWayland backend, requested geometry versus committed
  buffer, first paint/settled/mode-change cases, and root/popup input mapping.
  Future browser reproduction uses an isolated temporary profile. No additional
  live connection attempt or unverified rendering change was made.
- **Files/components:** docs/testing.md; ignored deployment-pending manifest.
- **Verification:** Documentation whitespace check; no code build/tests rerun
  because this change only records the user's report and pending acceptance.
- **Status/follow-up:** Source89b862014481 package remains staged, not installed;
  Chromium-specific native resolution remains unverified until reconnection.

### 2026-11-12 — researched Windows10 Mobile tasks and fast alphabet navigation

- **Request:** Research how Windows10 Mobile opens/displays task cards; improve
  the phone application list with tap/drag alphabet lookup.
- **Change:** Documented Microsoft's long-Back contract and Lumia manual UI,
  plus versioned large-phone grid / landscape evidence. Phone tasks now use
  accent-backed horizontal cards, app icon/name/close above real previews,
  narrow single-height or larger portrait2×2 pages, stable exact window IDs and
  an explicit Start destination. Downward dismissal follows the finger and
  accepts distance or velocity; cancellation uses existing Fluent/Rust curves,
  page snap has no spring overshoot, and rejected/confirmation-pending closes
  restore still-live cards. Removed the extra Return/count toolbar. Start-card
  selection reveals tiles without changing the approved Start-button toggle.
  App indexing now supports tap and continuous scrub with active-letter badge,
  fixed shared section/row/total geometry and truly lazy child construction.
  Exact sliver extents prevent far jumps being truncated by estimates. The
  original header→letter-grid flow retains phone navigation and fits all five
  rows in common landscape sizes. Fixed five MarkUI glyph selections whose
  opaque original badges lost detail under tint; originals/provenance preserved.
- **Files/components:** flutter_shell/lib/src/{app_catalog,catalog_index,
  phone_chrome,phone_tasks,phone_tasks_model,shell_navigation,start}.dart;
  flutter_shell/test/phone_navigation_test.dart; denial/preview_capture.dart;
  tools/import-markui-icons; wp_icons assets/manifest; task/index research,
  MetroLauncher parity, testing and lessons documentation.
- **Verification:** Pinned analyze passed;46 Dart tests passed. Source
  ecc33d3a5564 x86_64 and local-qemu ARM64 bundles/package built with matching
  source hashes, native ABI/ELF/checksums,93 packaged PNGs and41 font-codepoint
  coverage. Fifteen fresh preview cases passed without detected Flutter layout,
  asset or exercise exceptions: Start toggle; alphabet tap/scrub/reverse/grid;
  portrait/landscape pickers; landscape/light/large-font lists; narrow/large/
  landscape/bright-accent cards; slow dismissal; rejected close restoration;
  real long-Back/Start-card routing and empty native task popup. Inspected actual
  captures including both card layouts, landscape picker, index badge and real
  task-popup chrome. Some runs emitted the engine's frame-time clamp diagnostic;
  no claim is made that UI fixtures prove native focus/close behavior. Current
  review preview701623 refreshed; only obsolete task-owned627587 and690010 stopped.
- **Status/follow-up:** Implementation, host UI checks and matching ARM64 artifact
  complete; phone visual/native acceptance pending. Phone remains unavailable;
  no live reconnection attempt or device write occurred. Latest generation and
  revised DPMS wrapper remain staged, not installed; last verified phone source
  is c79511384aa8. Later verify real-window captures/activation/close, unsaved
  prompts and Chromium proportions on the phone. No recent-window cap, fake
  native windows, invented Start screenshot or exact original animation timing
  is claimed. Unrelated Cursor work/previews preserved; no commit/push.

### 2026-11-12 — phone controls/motion/input candidate (acceptance in progress)

- **Request:** Volume/brightness bars, nonlinear WP motion, continuous Win8 launch,
  consistent icon scaling, usable authorization and WP-style OSK on attached phone.
- **Change:** Added both real-service sliders to phone Action Center and nonlinear
  quick-grid resizing; top WP media-volume HUD; continuous Win8 trajectory; shared
  original-alpha optical icon fitting with pixel-rounded targets and no old nav
  magnifiers; native text-input auto-open gate for touch modes, square dark WP
  keycaps and non-spring keyboard settle; isolated-prefix authorization launcher.
- **Files/components:** action_center, volume_hud, launch, icon_geometry,
  markui_icons/metrics, symbols, shell_chrome, prepare/ARM packaging helpers and
  session/auth wrappers; opt-in icon/keyboard exercises; icon invariant test.
- **Verification:** Pinned analyze and47 tests passed; x86_64 and real-phone ARM64
  AOT build/package sourcecb4f17bab749 verified93 PNGs/41 font references/ELF/hash.
  Icon fixture16/24/32/48/64 at DPR1/1.25/1.5/2.5 visually inspected. Action Center
  and Start preview captured without detected exceptions. Keyboard fixture FAILED
  (keys offstage), so actual text input is not accepted. A temporary session agent
  registered and safe pkcheck produced a photographed native authentication dialog;
  no privileged action or credential-success claim. Shell wrappers syntax checked.
- **Status/follow-up:** Candidate installed into immutable /opt generation with
  labelled /var/backups/win10x-controls-20261112 recovery. SIGUSR1 mapped the new
  AOT, generation2, same boot/PID2525. Runtime refresh relocked native session;
  awaiting user's normal unlock and actual search-input check. Backup timer stopped
  after healthy native/AOT verification; rollback script retained. Existing user
  apps/SDDM125% preserved. Native slider, OSK, auth credentials, Win8 motion and
  persistent next-login agent startup still need acceptance; keep working.

### 2026-11-12 — WP picture lock, volume strip, popup motion and touch input

- **Request:** Continue phone control/input fixes; follow supplied Lumia lock and
  volume pictures, upward finger-follow reveal of password page; research and
  redesign phone popups/menus with nonlinear entry/reverse exit. Preserve Fluent
  authorization, continuous Win8 launch and consistent MarkUI proportions.
- **Change:** Custom presentation is now wired into Denial's real secure stage,
  with current wallpaper, lower-left thin time/date/notice count, top native
  connectivity/battery, finger-follow picture page and350/250ms source-derived
  WP settle. Revealed credentials use original native PAM, secure standard editor,
  delayed focus, scrollable insets, secret clearing and cancellation. Top native
  volume HUD uses0–30 media levels, native key/HUD events even at limits, real
  mute/MPRIS controls and WP motion. Anchored context flyouts and ApplicationBar
  have separate motion; square MessageBox uniformly flies from fullscreen and
  reverses on close, with scrollable body and persistent actions. Genuine active
  editors open OSK in phone/tablet without requiring a separate panel-show hint;
  serial dismissal is respected, Home/search always dismiss keys, Qt defaults to
  text-input-v3. Optical fit uses actual parent constraints. Continuous Win8
  geometry and initial sliders were built earlier in this task. New Fluent
  production auth bundle preserves original agent/PAM protocol and fixes secure
  editor pointer handling. Generated kit is restored from pinned inputs before
  every prepare to prevent repeated listener insertion.
- **Files/components:** flutter_shell/src phone_lock_pane/session_lock,
  phone_volume_panel/volume_hud, wp_motion/wp_popups/phone_dialog_motion,
  keyboard_policy/shell_navigation, icon_geometry/markui_icons, launch/action_center;
  denial composition hooks and opt-in preview checks; denial/polkit_app;
  tools/win10x-denial[-arm64/-polkit], packaging session/agent wrappers; AGENTS,
  interaction checklist, research and lessons.
- **Verification:** Matching patched-kit analyze PASS,51 pure tests PASS; x64 and
  ARM64 AOT+package PASS (93 bitmaps/41 glyphs);15 latest UI scenarios passed on
  source2dcc237605df, including short/cancel/full/reverse lock swipe, landscape,
  context/dialog overflow/lifecycle, keyboard/Home and20 icon size/DPR combinations.
  Repeated prepare produced identical generated digest and exactly one command /
  HUD binding. Native VT8 sourceabc5519c9fa9 verified real secure swipe, PAM prompt,
  automatic OSK and test-character delivery/clearing; no credential submitted.
  Native GTK editor visibly auto-opened OSK on2dcc237605df; final case-insensitive
  key receipt check is running on497c56017cd9. Auth layout inspected at phone,
  desktop and landscape sizes, production42a6755e88f4 registered on real session;
  native style check remains pending. Final deployment/acceptance update follows.
- **Status/follow-up:** Implementation/build complete; final497c native input /
  hardware-volume HUD checks in progress. Do not claim actual successful unlock,
  every browser/backend or exact W10M1:1 motion. No active-session SIGUSR1, SDDM
  restart, reboot, kernel/boot/radio change, or forced closure of user apps.

### 2026-11-13 — user screen settings, wallpaper and phone navigation (local validation)

- **Request:** Keep/add lock wallpaper; user-selectable automatic screen-off /
  timeout, separate on/off commands including passwordless authorized root hooks;
  specified bottom-up wallpaper/clock wake and reversed sleep; Start-style tile
  sleep/unlock entrance; content-width undimmed ordinary menus/volume; wallpaper
  picker; opt-in task-list navigation replacing the missing Search glyph. User
  explicitly says phone is unavailable: no SSH/deployment or host DPMS testing.
- **Change:** Added persistent screen/navigation preferences and settings UI;
  shared desktop/Start/secure-lock wallpaper picker with native wallpaper state
  synchronization; non-blocking native idle/power observation and command queue;
  root-owned exact-command snapshot helper with fixed off/on sudoers entries,
  reauthorization on edits, type/owner/no-follow checks and execution budgets;
  shared wake timeline, selected Start close/reopen integration; transparent
  menu barriers and intrinsic context-menu sizing; explicit navigation icon slots
  and live, persisted Search/task-list toggle. Native idle-DPMS is delegated only
  when ScreenPowerHost is enabled in a real session; preview does not change host
  power or execute hooks. Existing native authentication is retained.
- **Files/components:** flutter_shell screen_settings/screen_power/screen_policy /
  screen_motion/user_wallpaper, model/preferences consumers, phone_chrome/symbols,
  phone_lock_pane/start/wp_popups; denial main/composition/preview hooks;
  packaging/win10x-screen-hooks + installer, both package helpers, model tests,
  tools/test-screen-hooks.py; docs/screen-power-and-hooks.md and research.
- **Verification:** Patched-kit analyze PASS and53 tests PASS on initial candidate;
  isolated helper contract tests2 PASS (no real sudoers/root command writes),
  Python compilation and shell syntax PASS. First preview caught missing outer
  Directionality; fixed absolute alignment and current-bundle rerun in progress.
- **Status/follow-up:** Local preview acceptance ongoing. No phone connected or
  contacted, no SDDM changes. Automatic sleep delays native off until animation;
  physical-key pre-off animation requires Denial ambient handshake (not provided
  by every installed native runtime), documented in screen-power-and-hooks.md.
  Native DPMS timing, root authorization and physical-key behavior require later
  device acceptance; do not label local preview as hardware verification.

### 2026-11-13 — screen settings local review corrections

- **Request:** Finish the same wallpaper/power/menu/navigation request locally.
- **Change:** In-shell wallpaper browser replaces external dialog dependency;
  decode validation preserves old wallpaper on failure. Wallpaper reveal now has
  a soft bottom-up fade; status indicators share the delayed clock entrance.
  Navigation toggle rebuilds live and Search glyph uses an explicit slot.
- **Files/components:** wallpaper_picker/screen_settings/phone_lock_pane,
  phone_chrome/symbols, denial preview fixtures and screen-power documentation.
- **Verification:** Previous current-bundle11 scenarios all passed, including six
  wake phases, settings, task navigation, intrinsic-width context menu and volume;
  inspected content-width menu over undimmed Start and visible Search glyph.
  Latest revised picker/fade analyze PASS; final test/build/preview in progress.
- **Status/follow-up:** Awaiting final local review; phone remains untouched.

### 2026-11-13 — final local screen-settings verification

- **Request:** Validate new controls locally while phone is unavailable.
- **Change:** Final picker separates its path label from the title; actual image
  decode/persistence exercised through the picker callback. Installer includes
  staged screen-hook helper and existing auth wrapper, with DESTDIR respected.
- **Files/components:** wallpaper_picker, opt-in denial preview scenario,
  packaging/install-denial-session.sh; staged x64 package only.
- **Verification:** Current sourcec3ded5b6e9de analyze PASS,53 tests PASS, pinned
  x64 bundle/ABI/93 bitmaps/41 glyph checks PASS. Prior614ab source12 preview
  scenarios PASS (six wake phases, picker, settings, task button, context menu,
  volume, Start); final focused4 scenarios exercise actual wallpaper selection /
  persistence, corrected picker, settings and task navigation. Root helper
  isolated tests2 PASS; real visudo parsed temporary exact off/on rule; installer
  smoke PASS in a temporary DESTDIR; Python and bash syntax PASS; x64 package
  staged successfully. No system helper/sudoers installed on host, no phone contact.
- **Status/follow-up:** Local feature review complete subject to final focused
  capture results; real DPMS/privileged authorization/physical-key pre-off
  protocol remains explicitly pending as recorded in screen-power-and-hooks.md.

### 2026-11-13 — local focused acceptance result

- **Request:** Complete final local verification of screen settings and wallpaper.
- **Change:** No further implementation edits; current phone-mode review preview
  started from sourcec3ded5b6e9de.
- **Files/components:** docs/development-log.md; task-owned preview only.
- **Verification:** Final focused4 captures all PASS with no runtime exceptions:
  actual decoded/persisted wallpaper, picker, settings and task navigation. Inspected
  corrected path label and visible controls. Review preview remains running via
  tools/win10x-denial preview. Host power/privilege configuration and phone untouched.
- **Status/follow-up:** Local scope verified. Native physical-key pre-off handshake
  and real DPMS/hook authorization remain pending device/native protocol acceptance.

### 2026-11-13 — Chinese OSK, native phone volume route and gesture audit

- **Request:** Audit missing/linear/non-following motion, fix the phone still
  showing a bottom volume popup, add Chinese input/candidates on the WP keyboard
  referencing Sixuan IME's design. Continue local-only testing; phone unavailable.
- **Change:** Added independently implemented offline pinyin decoder with complete
  pinned Apache-2.0 Rime simplified-pinyin data, full/initial/incomplete spelling,
  apostrophe/ü handling, sentence segmentation and bounded personal learning.
  Persistent worker isolate does indexing/ranking; a WP dark keyboard header
  shows raw composition and scrollable/expandable selectable candidates; only
  confirmed candidates commit to the existing native text broker. Editor/hide
  resets stale composition; password/PIN/sensitive and URL/email/numeric contexts
  retain direct input without learning. Local personal dictionary uses700/600
  permissions and atomic serialized writes; preview state is isolated.
  Native SystemLevelHudLayer phone audio branch now renders the WP top strip
  directly, removing duplicate desktop/lock overlays and event forwarding.
  Toast follows drag by real height with continuous nonlinear settle; panel
  cancellation preserves actual drag position rather than snapping to zero.
  Existing staged controller clocks already apply nonlinear drawing curves;
  direct finger coupling stays1:1 rather than adding easing-induced lag.
- **Files/components:** flutter_shell/lib/src/ime, assets/ime notices/data,
  denial/keyboard and native HUD composition hooks, volume_hud/wp_toast/gestures /
  dashboard_reveal; tools/win10x-denial preparation/state isolation; pinyin tests,
  preview scenarios, docs/chinese-input-and-motion-audit.md.
- **Verification:** Analyze PASS;56 tests PASS including real65k-entry dictionary
  lookup/segmentation/initials/learning boundaries. Source19757050be6f x64+ARM64
  local-qemu build/package PASS. Nine UI cases clean, inspected Chinese候选 / expanded
  grid, actual“你好”commit, direct sensitive typing, landscape and native audio/
  limit provider route (top panel only; no legacy bottom card). Personal files
  show one learned entry for the Chinese fixture and zero for the sensitive
  fixture, mode600. New pointer-follow/cancel fixtures and final direct-purpose
  refinements are in final rebuild; no phone deployment or real-device acceptance.
- **Status/follow-up:** Final local motion/input checks ongoing. Build current
  ARM64 generation locally for later phone update; do not claim the installed
  phone changed or that a UI callback fixture proves native client Chinese input.

### 2026-11-13 — actual gesture-cancellation acceptance

- **Request:** Verify corrected nonlinear/finger-follow behavior, not just settled UI.
- **Change:** Raw cancellation guards now cover accepted Flutter drags; panel
  dismiss uses true travel geometry and retains its grab origin; stale IME
  actions/candidate taps are scoped to the active editor/composition.
- **Files/components:** wp_toast/gestures/action_center/dashboard_reveal /
  phone_lock_pane, denial/keyboard/pinyin_keyboard and pointer fixtures.
- **Verification:** Source2a3e4b5fe463 analyze/56 tests/x64 bundle PASS. Full10 local
  scenarios PASS: Chinese candidates/expanded grid/Chinese commit/password direct /
  landscape, native top HUD route, menu, actual toast pointer drag+PointerCancel,
  edge cancel and keyboard/Home. Viewed Chinese and top-HUD screenshots. ARM64
  qemu build ongoing; production HUD also being audited for secure-stage placement.
- **Status/follow-up:** Local corrected gestures accepted by fixtures; real phone
  update/native app delivery remain untested this round. No SSH or refresh issued.

### 2026-11-13 — correction to gesture/IME sweep status

- **Request:** Reliable local verification of the Chinese keyboard and gestures.
- **Change:** Correct the preceding entry: the source2a3e4b5fe463 sweep had eight
  clean cases and two failed Chinese commit/landscape cases, not ten passes.
  Both selected the outgoing keyboard immediately after the mode switch. The
  fixture now waits for the transition, asserts real candidates, and clears its
  isolated persisted state. Production phone HUD now mounts once above the
  secure stage; desktop's ordinary HUD remains inside its scene.
- **Files/components:** denial/preview_capture.dart, tools/win10x-denial;
  docs/lessons-learned.md. QA runner state isolation is task-local under/tmp.
- **Verification:** Prior generation analyze/56 tests and ARM64 packaging passed;
  corrected final UI sweep and secure-stage composition rebuild ongoing.
- **Status/follow-up:** Earlier all-ten claim superseded; final results to follow.

### 2026-11-13 — final local motion/Chinese and phone HUD acceptance

- **Request:** Fix missing/nonlinear/finger-follow motion, phone hardware-volume
  presentation and WP-style keyboard Chinese candidate selection.
- **Change:** Final native phone HUD mounts once above secure stage; scene HUD
  remains desktop-only. Chinese decoder/candidate UI, cancellation/editor guards
  and corrected clean-state gesture/IME fixtures are included in e805bf9200f7.
- **Files/components:** denial/keyboard, flutter_shell IME/gesture/volume/lock UI,
  tools/win10x-denial preparation, preview_capture and audit/current-work docs.
- **Verification:** Final pinned analyze PASS,56 tests PASS, x64 bundle PASS.
  Ten-case QA runner exit0: candidates, expanded grid, Chinese commit, sensitive
  direct input, landscape, native audio/limit top-only route, menu, actual toast
  drag+PointerCancel, edge cancel, keyboard/navigation. Inspected Chinese and
  native-volume screenshots; personal data1 vs0 entries, mode600. Replaced only
  task-owned old preview; latest preview is e805bf9200f7 and remains running.
- **Status/follow-up:** Local implementation checks complete. ARM64 local-qemu
  build/package underway; phone deployment, physical keys and Chinese delivery
  into actual native clients remain untested. No phone/session/login changes.

### 2026-11-13 — ARM64 local build/package finalized

- **Request:** Keep the phone-target build ready while device is unavailable.
- **Change:** Staged final e805bf9200f7 ARM64 shell/session with Chinese dictionary
  and updated motion/HUD, retaining existing Fluent authentication presentation.
- **Files/components:** Generated target/denial-arm64/bundle and
  packaging/out/denial-arm64 (build output; no host installation).
- **Verification:** WIN10X_ARM64_HOST=qemu tools/win10x-denial-arm64 bundle and
  package both exit0. Verified93 bitmaps and41 icon codepoints, generation
  e805bf9200f7-0p23xhfa. Latest x64 preview still running without runtime errors.
- **Status/follow-up:** Local x64/ARM64 implementation and packaging complete;
  phone deployment and native/hardware acceptance pending device availability.

### 2026-11-13 — phone installation and native Chinese/volume/motion tests

- **Request:** Install the new ARM64 generation on the connected phone and test.
- **Change:** Installed immutable e805bf9200f7-arm64-ime-20261113, verified archive
  SHA8417c1296147972dde11f7f716506d27a9aba38eaa0cf249926fc14f3fb5a41b and
  app/engine hashes against metadata. Atomically selected current; installed
  phone wrapper and screen-hook helper without authorizing any root hook.
  Root rollback backup /var/backups/win10x-ime-20261113 retains prior current,
  wrapper/helper and SDDM hashes. Existing auth-current and Breeze125% retained.
- **Files/components:** Generated ARM64 runtime on phone under/opt; host evidence
  target/denial-arm64/qa/ime-motion-20261113; phone/current-work/audit documents.
- **Verification:** Verified USB SSH, bootffba7009, normal tty2/PID1907. Separate
  VT8 with private config/runtime,32s compositor /45s unit /52s independent restore.
  Actual GTK editor auto-opened OSK; injected touch taps typednihao, selected你好,
  and application receipt JSON confirms text你好. Injected gpio-keys volume-up
  through evdev: inspected actual top WP HUD screenshot, no bottom volume card.
  Native secure swipe/PAM prompt/automatic OSK/test-a delivery passed; field
  cleared, no authentication submitted. Real native toast follow/PointerCancel
  fixture passed in log. Returned tty2, original PID1907 and boot unchanged;
  test unit inactive, current symlink new, login/SDDM hashes unchanged.
- **Status/follow-up:** Installed/tested; existing ordinary session still has
  the older already-loaded engine. New default activates on normal next login;
  no SIGUSR1, forced logout, reboot or SDDM restart. Native Chromium Chinese input
  not tested in this turn. Locked volume-key display does not work: upstream
  native shortcut security guard consumes system controls while locked; record
  separately from the passing unlocked top-HUD test. No native policy changed.

### 2026-11-13 — split GitHub publication prepared

- **Request:** Two new GitHub repositories, desktop and keyboard/input method;
  contributor identity only ycrrongos, descriptions disclose Codex development.
- **Change:** Export fresh independent Git histories without changing original
  origin/history; standalone Dart pinyin package and Denial adapter, desktop
  pinned keyboard source/hash manifest and sync verifier. Preserve upstream
  license/copyright/data notices, no agent coauthor trailers. Default private
  after unanswered optional visibility question; descriptions/README credit Codex.
- **Files/components:** target/github-publication-20261113 exports;
  docs/github-publication.md; standalone adapter installer and source sync tool.
- **Verification:** Authenticated GitHub identity ycrrongos/id124893015 verified.
  Keyboard analyze/3 tests pass; pinned-host adapter install/idempotence/refusal
  checks pass. Desktop export analyze/56 tests/bundle pass, same e805bf9200f7 source
  generation,93 bitmaps/41 glyphs. Pinned keyboard integration parity passes.
  Credential/build-path scan passes; git root commits use only user identity.
- **Status/follow-up:** Prepared/verified; remote repository creation/push and
  remote author/contributor verification follow before completion claim.
