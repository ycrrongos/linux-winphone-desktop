# Lessons learned

Append-only log for blockers and fixes. Required by [`AGENTS.md`](../AGENTS.md).

## Entry template

```markdown
### YYYY-MM-DD — short title

- **Problem:**
- **Cause:**
- **Resolution:**
- **Tried but failed:**
```

---

### 2026-08-12 — project bootstrap

- **Problem:** Greenfield repo had research only; no compositor/shell code or agent logging path.
- **Cause:** Intentionally research-first scaffolding.
- **Resolution:** Added Cargo workspace (`win10x-protocol`, `win10x-shell`, `win10x-comp`), `agent.md`, and this file.
- **Tried but failed:** N/A (bootstrap).

### 2026-08-12 — iced 0.13 API mismatch

- **Problem:** First shell draft failed to compile (Space::new arity, button style Fn(&Theme, Status), missing wrap widget, Padding arrays).
- **Cause:** Assumed newer iced APIs than 0.13 provides.
- **Resolution:** Rewrote views for iced 0.13 (`Space::new(w,h)`, theme-aware style closures, manual app grids).
- **Tried but failed:** `iced::widget::wrap`; `Padding::from([t,r,b,l])` four-array form.

### 2026-08-12 — smithay chrome overlay after render_output

- **Problem:** Second `GlesRenderer::render` after `space::render_output` hit lifetime / inference issues; wrong `render_output` generic order treated custom elements as space elements.
- **Cause:** `render_output` type params are `(R, C=custom, E=space element, S)`; framebuffer borrow from `backend.bind()` cannot outlive the bind scope the way a second frame expected.
- **Resolution:** Draw taskbar/panels as `SolidColorRenderElement` passed in `custom_elements`; annotate `GlesRenderer` explicitly.
- **Tried but failed:** Post-`render_output` `draw_solid` frame; swapping C/E generics.

### 2026-08-13 — switch shell UI to fluent-egui

- **Problem:** Hand-rolled “Fluent-like” iced styling still did not look like Fluent UI.
- **Cause:** No real design-system widgets/tokens; iced itself is not Fluent.
- **Resolution:** Replaced iced shell with **`fluent-egui` 0.1** (WinUI 3 / Fluent 2 for egui/eframe). Kept `.desktop` discovery + `gtk-launch` / Exec spawn. Also noted `iced-fluent-theme` exists but targets iced 0.14 and is theme-only (fewer widgets).
- **Tried but failed:** Staying on iced with a custom palette; nesting FluentUi methods inside raw `egui::Ui` layout closures (need `FluentUi::column/row/wrap`).

### 2026-08-13 — Fluent 1 skin from Win10UI on fluent-egui

- **Problem:** Win10X is Fluent **1** (UWP / Win10 era); `fluent-egui` is Fluent **2**. No Rust Fluent 1 crate exists.
- **Cause:** Ecosystem only ships Fluent 2 themes; HTML reference `/home/rong/Win10UI` is closer to Win10 look.
- **Resolution:** Kept `fluent-egui` as widget runtime; added `shell/src/fluent1/` with tokens/acrylic/reveal/motion/visuals ported from Win10UI CSS, applied each frame over Fluent 2.
- **Tried but failed:** Finding a Rust Fluent 1 library; pure Fluent 2 look (too Win11).

### 2026-08-13 — force square Fluent corners

- **Problem:** Buttons/panels still looked rounded after Fluent 1 tokens.
- **Cause:** `fluent-egui` hardcodes `CornerRadius::same(4|8)` on controls; our skin only overrode egui `Visuals` and acrylic panels.
- **Resolution:** Vendored crate under `third_party/fluent-egui` and zeroed rectangular radii; set `radius_panel = 0`. Left circular geometry (toggle/radio/slider) alone.
- **Tried but failed:** Overriding only egui visuals (Fluent widgets paint their own radii).

### 2026-08-13 — Win10 acrylic + Colors page + wallpaper accents

- **Problem:** Panels lacked real frosted acrylic; color UI was a short accent list; no “theme from wallpaper”.
- **Cause:** egui has no `backdrop-filter`; earlier acrylic was flat tint only.
- **Resolution:** Implemented MS acrylic recipe (blur/saturate/exclusion/tint/noise) using wallpaper-sampled textures + Win10UI params (`blur 40/28/50`, saturate 180%/160%). Colors tab mirrors Win10 (48 Windows colors, recent, HSV custom, accent-on-surfaces, auto-from-background). Wallpaper accents via `color-thief` (Color Thief / MMCQ). Contrast tab previews Aquatic/Desert/Dusk/Night sky + WCAG ratios.
- **Tried but failed:** Relying on CSS-only acrylic libs inside native egui (no HTML surface).

### 2026-08-13 — easier wallpaper + Start acrylic retune

- **Problem:** Changing wallpaper required picking among presets; acrylic looked muddy / too opaque vs Win10 Start.
- **Cause:** Tint≈0.72 + low-res blur; Start uses HostBackdrop with ~BlurAmount 30 and more see-through tint.
- **Resolution:** Added **Browse pictures…** (`rfd`) + Recent list. Retuned acrylic to Start-like params (blur≈30, saturate≈1.25, tint≈0.55, luminosity≈0.88, baked composite, less noise) per MS Learn + Windhawk acrylic mimic notes.
- **Tried but failed:** Keeping Win10UI’s 0.72 tint as the Start look (too solid over photo wallpapers).

### 2026-08-13 — acrylic disappeared after bake

- **Problem:** After “Start acrylic retune”, panels looked like acrylic was gone (flat / invisible).
- **Cause:** Baking blur+tint into one opaque pass + fragile sliding-window blur made the surface either solid-mud or fail to read as frosted glass.
- **Resolution:** Restored layered paint: blurred wallpaper texture → light exclusion → translucent tint → noise; safer box blur; tint alpha kept ~0.4–0.55 so blur stays visible.
- **Tried but failed:** Single fully-baked acrylic texture without a separate tint layer.

### 2026-08-13 — static wallpaper blur ≠ Win10 HostBackdrop

- **Problem:** Acrylic looked “fake”: only a static blur of the wallpaper, not live content behind the panel; also CPU-heavy.
- **Cause:** Shell-side egui samples/blurs a wallpaper bitmap. Real Win10 acrylic is **DWM composition**: windows already live as GPU textures; HostBackdrop samples the partially composited desktop *behind* the surface (other windows + wallpaper, excluding self), then a GPU pixel shader does Gaussian blur + tint/noise. The app only draws translucent UI; it never blurs wallpaper itself. Cheap because blur runs once per frame in the compositor on already-resident textures (Aero/DirectComposition lineage), not CPU box-blur of bitmaps.
- **Resolution (direction):** Move acrylic to `win10x-comp`: draw wallpaper + windows first, apply GPU blur under translucent shell regions (smithay `ext-background-effect-v1` / custom GLES pass), shell paints tint/noise/UI with transparent panel backgrounds. Wallpaper-only blur in egui is at best a fallback when transparency is off / battery saver.
- **Tried but failed:** Emulating HostBackdrop by CPU-blurring wallpaper inside the shell (cannot see live windows; wrong cost model).

### 2026-08-13 — compositor HostBackdrop via framebuffer effect

- **Problem:** Needed live acrylic (see windows through taskbar/Start) without CPU wallpaper blur.
- **Cause:** Shell and compositor are separate L1 demos; egui cannot sample other windows; smithay has no built-in blur shader (only `ext-background-effect-v1` state + `is_framebuffer_effect` hooks).
- **Resolution:** Added `compositor/src/acrylic.rs`: `AcrylicBlurElement` with `is_framebuffer_effect` → `capture_framebuffer` (blit behind region into offscreen `GlesTexture`) → GLES custom texture shader (box blur + tint + grain). Wired under taskbar/Start card in `winit.rs` via `ChromeElement` enum. Shell acrylic drops opaque wallpaper blur by default (tint/noise only); `WIN10X_SHELL_WALLPAPER_BLUR` restores old fallback.
- **Tried but failed:** Relying on shell-side bitmap blur; second GLES pass after `render_output` (lifetime/API issues — keep effects inside `custom_elements`).

### 2026-08-13 — shell as nested Wayland client over HostBackdrop

- **Problem:** Needed Fluent UI + live acrylic; shell and compositor were separate demos.
- **Cause:** `space::render_output` draws all space windows under `custom_elements`, so a fullscreen shell in `space` would sit *under* HostBackdrop blur (icons frosted over). Fullscreen shell also steals all input.
- **Resolution:** Tag shell via `app_id=win10x-shell`; fullscreen layout; each frame unmap shell → render apps → acrylic custom → shell `WaylandSurfaceRenderElement` on top of blur → remap. Chrome hit-test only on taskbar/panel rects. Nested shell (`WIN10X_NESTED=1`) skips wallpaper/acrylic fills; panel state via title `Win10X Shell|Launcher`. Default spawn: shell + terminal.
- **Tried but failed:** Leaving shell in `space` under blur; layer-shell as first cut (eframe has no layer-shell client API).

### 2026-08-13 — nested EGL clients need dmabuf v4 + no MSAA

- **Problem:** Auto-spawned `win10x-shell` died under nested `win10x-comp` (`glutin` “no matching configuration” / EGL init failures).
- **Cause:** Nested winit EGL lacks `EGL_WL_bind_wayland_display`; Mesa clients need `linux-dmabuf` v4 feedback. eframe default `multisampling=4` also finds no config on the nested display.
- **Resolution:** Enabled smithay `use_system_lib` + `backend_drm`; advertise dmabuf v4 from render node; nested shell sets `multisampling/depth/stencil = 0`.
- **Tried but failed:** dmabuf v3 alone; `bind_wl_display` (extension absent on nested EGL).

### 2026-08-17 — detached-shell helper absent in Codex environment

- **Problem:** The repository-mandated `~/.qoder/skills/detached-shell/scripts/run.sh` entry point was unavailable, so the documented background test command could not start.
- **Cause:** This Codex environment does not contain the Qoder-specific detached-shell skill path referenced by `agent.md`.
- **Resolution:** Recorded the environment mismatch and used Codex's bounded PTY/background command support for this read-only review.
- **Tried but failed:** Invoking the exact `bash ~/.qoder/skills/detached-shell/scripts/run.sh exec ... "cargo check"` command from `agent.md`.

### 2026-08-17 — GUI launch requires access to the host Wayland socket

- **Problem:** `cargo run -p win10x-shell` compiled successfully but exited with `WaylandError(Connection(NoCompositor))` instead of opening the desktop.
- **Cause:** The sandboxed process inherited `WAYLAND_DISPLAY=wayland-1`, but could not connect to the host compositor socket.
- **Resolution:** Relaunched the same current-worktree build with approved host GUI access; it connected to Wayland and remained running.
- **Tried but failed:** Launching the GUI directly inside the default filesystem sandbox despite the correct Wayland environment variables and an existing socket.

### 2026-08-17 — workspace-wide rustfmt trips over missing vendored examples

- **Problem:** `cargo fmt --all` stopped before formatting the project crates because three example files declared by vendored `fluent-egui` do not exist.
- **Cause:** The vendored crate manifest references `examples/android.rs`, `examples/demo.rs`, and `examples/web.rs`, but those files are absent from this checkout.
- **Resolution:** Scoped rustfmt to the two changed packages with `cargo fmt -p win10x-shell -p win10x-comp`.
- **Tried but failed:** Running workspace-wide `cargo fmt --all`.

### 2026-08-17 — compositor screenshot tools could not capture a background test window

- **Problem:** The first visual-verification path could not capture the running Win10X shell while the user continued working in another niri workspace.
- **Cause:** `grim` is not installed, and niri screen screenshots capture the currently focused screen state rather than a background application window.
- **Resolution:** Used the vendored shell runtime's `WINUI_EGUI_SCREENSHOT` one-shot self-capture path, which saved the shell's own rendered frame and exited the temporary capture instance.
- **Tried but failed:** Calling unavailable `grim`; focusing the background window and then taking niri screen screenshots while the active user session reclaimed focus.

### 2026-08-17 — egui debug build painted red taskbar rectangles

- **Problem:** Opening Start briefly painted red lines/frames around taskbar controls even after the explicit selected underline was removed.
- **Cause:** Taskbar controls used auto-generated egui IDs inside an auto-ID child UI. Adding the Start panel changed the ID sequence between layout passes; egui 0.35 debug builds intentionally draw a 2 px red rectangle when `warn_if_rect_changes_id` detects this mismatch.
- **Resolution:** Gave the taskbar child UI a fixed `id_salt` and every taskbar command button a stable explicit interaction ID. Also made standalone mode an undecorated fullscreen `win10x-shell` viewport so host window chrome cannot be confused with shell pixels.
- **Tried but failed:** Removing the selected-button underline alone; attributing the red frame solely to niri focus/urgent borders.

### 2026-08-17 — stable clock child scope did not stabilize its widget

- **Problem:** Runtime verification of the new Start animation again logged
  `changed id between passes` for the taskbar's 58×48 clock rectangle, risking
  another egui debug red frame.
- **Cause:** The clock child UI had a fixed `id_salt`, but its centered label
  still created an automatic widget response whose ID changed with the
  surrounding multi-pass layout.
- **Resolution:** Kept the layout reservation, replaced the label widget with a
  stable explicit `ui.interact(..., ui.id().with("taskbar_clock"), ...)`, and
  painted the clock text directly.
- **Tried but failed:** Relying only on a fixed `id_salt` for the clock child UI.

### 2026-08-17 — `cargo check` left the visual-test executable stale

- **Problem:** The user saw almost no change after the Disco turnstile was
  implemented and the reported test shell was opened.
- **Cause:** The verification ran `cargo check` and unit tests, then launched
  `target/debug/win10x-shell` directly. `cargo check` validates code without
  refreshing that runnable binary, and the test harness does not replace it,
  so the GUI was still the previous build.
- **Resolution:** Stopped only the shell instance launched by the task, ran
  `cargo build -p win10x-shell`, and started the freshly rebuilt executable.
- **Tried but failed:** Treating a successful `cargo check` plus test build as
  proof that an existing `target/debug/win10x-shell` contains the latest code.

### 2026-08-17 — application transition must overlap cold start

- **Problem:** The source-derived application transition felt unchanged and
  added a pause before the target application began loading.
- **Cause:** The first implementation treated the shell animation as a
  pre-launch gate: it stored the clicked application, played the whole Start
  exit, and spawned the process only at animation completion. This inverted
  the Windows Phone latency-hiding behavior.
- **Resolution:** Enter the visual handoff state and spawn the application in
  the same click handler. The pending state now retains only the visual source
  rectangle/application identity while cold start and animation run together;
  animation completion merely hides Start and clears that state.
- **Tried but failed:** Delaying process creation until the shell transition
  finished in an attempt to guarantee that every animation frame remained
  visible.

### 2026-08-18 — linker bus errors can precede an exhausted build cache

- **Problem:** The refreshed shell first failed while linking with an `ld`
  bus error; an immediate retry then reported `No space left on device` and
  rustc terminated while writing the binary.
- **Cause:** This workspace had accumulated about 15 GiB of build output,
  including several gigabytes of stale shell objects and executables. The
  linker could not complete a new large debug binary reliably.
- **Resolution:** Inspected filesystem and target-directory usage, then ran
  `cargo clean -p win10x-shell`, removing only 3.3 GiB of regenerable artifacts.
  The next `cargo build -p win10x-shell` completed normally.
- **Tried but failed:** Retrying the same link without first reclaiming the
  package's build-cache space.

### 2026-08-18 — shell PTY Ctrl+C propagated into launched applications

- **Problem:** Stopping the standalone shell test with Ctrl+C also terminated
  applications that had been launched from that shell.
- **Cause:** `Command::spawn` left application processes in the shell's
  foreground terminal process group, so the terminal delivered SIGINT to the
  complete group rather than only to `win10x-shell`.
- **Resolution:** Create application commands in a separate Unix process group.
  Nested mode also prefers the desktop entry's direct `Exec` path so the new
  process inherits the project compositor's Wayland socket before trying
  `gtk-launch`.
- **Tried but failed:** Treating Ctrl+C sent to the shell's PTY as a
  process-local and therefore safe visual-test shutdown.

### 2026-08-18 — topmost shell chrome must not become the active window

- **Problem:** A newly created application received fullscreen geometry and
  keyboard focus, but the shell overlay could immediately mark it inactive.
- **Cause:** Shell layout, input restacking, and the per-frame composition
  remap all raised the transparent shell with Smithay's `activate = true`.
  Smithay correctly deactivated every other mapped window each time.
- **Resolution:** Keep shell z-order above applications with `activate = false`
  everywhere it is mapped or raised. The new application remains the activated
  keyboard-focus owner while shell chrome still receives hit-tested pointer
  input.
- **Tried but failed:** Activating the application only in `new_toplevel`;
  subsequent shell restacking silently overrode that state.

### 2026-08-18 — rejecting shell after topmost lookup does not provide input pass-through

- **Problem:** The fullscreen application rendered below transparent shell
  chrome but could not be clicked; its real window also never appeared in the
  taskbar or Task View.
- **Cause:** `Space::element_under` stopped at the full-output shell surface,
  after which the code rejected shell and returned no target instead of
  continuing to the application below. Separately, running indicators used
  only shell-local `AppEntry.running` flags, so compositor-created windows such
  as the demo terminal were invisible to shell UI.
- **Resolution:** Iterate the space top-to-bottom while filtering shell before
  surface hit testing. Added an atomic JSON runtime bridge through which the
  compositor publishes real app IDs, titles, and active state; shell uses it
  for taskbar and Task View and sends one-shot activation requests back.
- **Tried but failed:** Treating a post-lookup `if is_shell { None }` branch as
  equivalent to a transparent input region, and treating launch-time shell
  bookkeeping as authoritative compositor window state.

### 2026-08-18 — floating geometry does not imply a draggable window

- **Problem:** The compositor resized and centered Alacritty in floating mode,
  but the user could not drag it.
- **Cause:** The initial floating implementation only changed xdg-toplevel
  geometry and depended on the client to issue an xdg move request. Borderless
  clients do not expose a titlebar or another reliable move-request region.
- **Resolution:** Reserve a compositor-owned 36 epx strip at the top of a
  floating surface. Start a Smithay pointer move grab from that strip and add
  an equivalent direct touch drag path; after every move, raise shell chrome
  with `activate = false` so the taskbar stays visible without stealing app
  activation.
- **Tried but failed:** Treating a smaller non-fullscreen configure as a
  complete floating-window implementation and relying solely on optional
  client-side decorations.

### 2026-08-18 — a window's first committed buffer is not a durable preview

- **Problem:** The first generated window thumbnail was a valid PNG but showed
  only the client's blank initial surface instead of its subsequently painted
  content.
- **Cause:** Preview generation ran as soon as the xdg-toplevel had non-zero
  geometry, then treated the newly created file as permanently fresh. Clients
  such as Alacritty commit their visible content after that initial buffer.
- **Resolution:** Enable preview work only when an app actually has duplicate
  windows, then refresh at a low background cadence and a faster cadence while
  the taskbar app menu is open. Write every update through a temporary PNG and
  atomic rename so shell never decodes a partial frame.
- **Tried but failed:** Capturing only when the preview file was missing and
  using file existence as proof that the client's final visible frame had been
  captured.

### 2026-08-18 — taskbar thumbnails cannot serve as full-screen transition frames

- **Problem:** Application-switch animations made otherwise sharp application
  content severely blurry when the proxy approached full-screen size.
- **Cause:** The shell reused the taskbar menu's compositor PNG, capped at
  320×180, then linearly enlarged it across the complete output. Inspecting the
  new full-size capture also exposed an older extra vertical flip in the
  offscreen readback path.
- **Resolution:** Publish separate thumbnail and switch-image paths per window.
  Render the switch image once at the output's physical scale, derive the
  bounded taskbar thumbnail from it, keep independent shell texture caches, and
  remove the redundant image flip. Runtime inspection confirmed an upright
  2561×1601 switch image beside a 288×180 menu image on the current full-screen
  output.
- **Tried but failed:** Reusing the bounded menu PNG for both purposes; texture
  filtering can smooth magnified pixels but cannot reconstruct missing detail.

### 2026-08-18 — full-output PNG refresh can monopolize the render thread

- **Problem:** After switching to native-output animation snapshots, the nested
  desktop became severely unresponsive; `win10x-comp` stayed near 68% CPU with
  two application windows.
- **Cause:** Every two seconds the compositor synchronously rendered, read back,
  PNG-compressed, and wrote a 2561×1601 image for every window. A complete
  two-window pass exceeded the refresh interval, so the next pass began almost
  immediately on the render thread.
- **Resolution:** At the user's requested quality/performance point, switch
  captures now use 25% of the output width and height—about 640×400 on the
  current full-screen output and 1/16 of the original pixels. Three-window
  runtime sampling no longer showed the former continuous 68% load, although
  synchronous refresh still produces brief CPU spikes and remains a candidate
  for later on-demand or worker-thread capture.
- **Tried but failed:** Periodically PNG-encoding the complete physical output
  for every window; lowering only the taskbar thumbnail size did not reduce that
  separate full-resolution work.

### 2026-08-18 — lower resolution does not remove synchronous capture stalls

- **Problem:** At 25% switch resolution, interaction was perfectly smooth
  between refreshes but could still freeze for roughly a second when several
  previews updated.
- **Cause:** This was not a memory leak: compositor RSS remained exactly
  192868 KiB and shell RSS changed by only 4 KiB over a 15-second sample. The
  remaining path still captured every window serially, then resized,
  PNG-compressed, and atomically wrote all images on the render thread. Three
  windows completed in one roughly 0.42-second burst; an open app menu repeated
  the burst every 700 ms.
- **Resolution:** Capture one window at a time, distributing a complete window
  set across the existing refresh cycle. Send the resulting RGBA buffer to a
  named encoder worker through a capacity-one queue; the worker performs resize,
  PNG compression, and writes, and a busy queue drops stale frames instead of
  retaining memory. Under three-window load, files updated about 0.67 seconds
  apart, the encoder carried 15–60% CPU while the render thread stayed around
  0–5%, and compositor RSS held at 195112 KiB over another 15-second sample.
- **Tried but failed:** Merely reducing each image to 640×400 while leaving all
  compression and all-window iteration synchronous; it reduced total CPU work
  but preserved the periodic stop-the-world shape.

### 2026-08-18 — hiding a shell transition does not stop compositor capture

- **Problem:** Turning off the visible application-switch proxy was expected to
  remove its GPU cost, but the compositor could still keep refreshing switch
  snapshots in the background.
- **Cause:** Proxy painting belongs to the shell while render/readback and PNG
  production belong to an independent compositor refresh loop; changing only
  shell state cannot suspend that producer.
- **Resolution:** Added a persisted shell preference carried over session IPC.
  When disabled, the compositor returns before background GPU readback and also
  drops queued transition outputs. It performs only bounded thumbnail captures
  while a real multi-window taskbar menu is open.
- **Tried but failed:** Treating cancellation of `pending_app_switch` and its
  texture cache as a complete performance switch; that removes the consumer but
  leaves the expensive producer running.

### 2026-08-18 — sampled cubic Bézier curves need exact endpoint guards

- **Problem:** The source-faithful flutter_metro_ui entrance regression expected
  a fully hidden group at time zero, but the curve returned a small non-zero
  value and made the first frame faintly visible.
- **Cause:** The generic 14-step binary-search Bézier sampler approximated the
  parameter even at input 0 and 1 instead of preserving those mathematical
  endpoints exactly.
- **Resolution:** Return exact 0 and 1 before sampling interior values, then add
  endpoint and strong-nonlinearity tests for Flutter's `easeOutExpo` curve.
- **Tried but failed:** Relying on additional binary-search precision alone;
  an approximation is unnecessary and semantically wrong at exact endpoints.

### 2026-08-18 — a source group animation needs real group data

- **Problem:** The nominally source-faithful flutter_metro_ui Start entrance
  still looked unlike the reference, especially across the horizontally spread
  blocks shown in the original project.
- **Cause:** The renderer copied the first group's transform but collapsed every
  pinned tile into that one group. The source actually wraps each semantic group
  in its own center-aligned scale/translation/opacity transform and assigns it a
  distinct `groupIndex × 0.04` start time.
- **Resolution:** Make group ID/title part of the persisted tile model and use
  the real group bounds and index in both the ordinary flutter style and the
  fixed Windows 8 layout. Migrate old tiles to one default group rather than
  inventing classifications, then let the user create, rename, and populate
  additional groups.
- **Tried but failed:** Treating the complete tile board as source group zero;
  exact easing and duration values cannot reproduce a multi-group composition
  when the semantic grouping itself is absent.

### 2026-08-18 — derived layouts cannot reuse freeform drag coordinates

- **Problem:** Tiles remained visually fixed when dragged in the Windows 8
  layout, and editing a group title in its context menu did not reliably keep or
  save the typed value.
- **Cause:** Windows 8 ignores persisted `col`/`row` coordinates and repacks from
  tile order on every frame, while the only existing drag state changes those
  freeform cells. The title field also lived inside a transient context popup,
  so closing or refocusing the popup destroyed the editor before a reliable
  commit boundary.
- **Resolution:** Give the derived layout its own pointer-following drag state
  and convert a drop into source-list ordering plus group membership. Keep group
  renaming in board state and render it inline until Enter/focus loss commits or
  Escape cancels.
- **Tried but failed:** Exposing `Sense::click_and_drag()` without implementing a
  data mutation understood by the derived renderer, and directly mutating the
  title through a text field owned by the context menu.

### 2026-11-10 — a Denial launcher plugin does not own desktop placement

- **Problem:** The planned migration could not obtain our full-screen Start
  simply by supplying a `ShellLauncher` implementation; the reference host still
  bounded the launcher as a popup, slid it onto the output and applied glass.
- **Cause:** In Denial 0.6.2, launcher content is public plugin policy, but output
  geometry, panel transition and native input layout are reference-desktop host
  responsibilities. Window rendering and input publication share its geometry.
- **Resolution:** Pin the runtime and compiler kit to one release. Patch the
  prepared reference plugin's shared `launcherRect`, launcher-only transition and
  concurrent launch adapter with exact context checks. Start supplies wallpaper
  plus white alpha veil and bypasses the host blur wrapper. Keep downloaded
  upstream sources unchanged and record modifications in the build helper.
- **Tried but failed:** Treating `denial-nested` as the full Flutter session: its
  source is a topology demonstration. A standalone native host validates UI/AOT,
  while native split/focus acceptance still needs a DRM/KMS session. Painting a
  larger widget without changing the shared hit-test geometry is insufficient.

### 2026-11-10 — release-only Flutter authoring needs the matching kit and SDK

- **Problem:** There was no Flutter/Dart installation on this machine. Large SDK
  downloads timed out with partial files, and the initial online Pub resolution
  waited on package downloads. A direct `dart test` then rejected this package
  because it declares a Flutter dependency.
- **Cause:** The full Dart archive is roughly 239 MB and single transfers were
  slower than the bounded command timeout. Denial's engine/compiler declarations
  are patched and version-coupled; a stock Flutter substitution would not prove
  compatibility. Dart's package resolution also needs `FLUTTER_ROOT` when invoked
  directly in a Flutter package.
- **Resolution:** Use the official Denial 0.6.2 release/compiler packages, verify
  their recorded SHA-256 values, and use Dart 3.13.4. Resume or range-download
  partial archives and verify the assembled archive against its complete
  published hash before extraction. Seed locked Pub versions, verify fetched
  package hashes and resolve offline once cached. The helper exports the prepared
  `FLUTTER_ROOT`; nine pure-Dart migration/layout tests then passed without a
  Flutter debug engine. The release AOT bundle and raw engine ABI also validated.
- **Tried but failed:** Repeatedly starting the large SDK download from zero;
  treating an incomplete ZIP as usable; running `dart test` without the matching
  Flutter root; running the legacy Cargo check offline with an empty crate cache
  (`color-thief` was absent), which says nothing about source correctness.

### 2026-11-10 — Denial desktop popups have no Flutter Navigator

- **Problem:** Initial port code used Material route-based dialogs, menus and
  dropdowns, but the Denial shell provides an Overlay without a Navigator.
- **Cause:** The reference desktop deliberately uses `ShellOverlayHost` and
  `ShellPopupController` to coordinate native input, focus and retained popup
  lifetimes. Importing Material widgets does not create a Navigator.
- **Resolution:** Replace route-based menus/Task View with managed shell popups
  and inline group editors. Use RadioGroup/ListTile selection within those
  popups rather than dropdown routes. Keep each popup's handle for dismissal and
  let the shared controller own outside-tap/Escape handling.
- **Tried but failed:** The initial `showDialog`/`showModalBottomSheet` approach
  was rejected after tracing the host tree, before interactive acceptance.
  Adding a separate Navigator would bypass the existing popup/input lifecycle
  instead of integrating it.

### 2026-11-10 — an incremental release can keep an obsolete icon subset

- **Problem:** The newly ported Action Center compiled and opened, but Wi-Fi,
  Bluetooth, media and power icons were missing and other icons showed unrelated
  glyphs. The release icon font remained 21,644 bytes with its first-build time.
- **Cause:** Confirmed in the pinned Flutter source and build stamp:
  `BundleLinuxAssets.inputs` does not include `app.dill`. Changes to Dart
  `IconData` constants rebuilt the AOT image but left the old tree-shaken
  MaterialIcons font cached. Direct cmap inspection showed the new Wi-Fi,
  volume, battery, power and Bluetooth codepoints absent from that subset while
  all were present in the source font.
- **Resolution:** The build helper invalidates only the Linux release-assets
  stamp when the project source digest changes. Verify the assembled font's cmap
  against the actual icon names used by our UI, along with bundle checksums and
  the native engine ABI. Reject source edits made during compilation rather than
  labeling the resulting bundle current.
- **Tried but failed:** Rebuilding the release AOT repeatedly without invalidating
  the asset target; static analysis and a successful engine ABI check cannot
  detect absent glyphs. Avoid clearing the entire compiler/toolchain cache when
  the stale boundary is only the generated asset bundle.

### 2026-11-10 — Start close looked broken because the host faded the whole page

- **Problem:** Closing Start showed no tile reverse wave; the screen simply
  faded or snapped away.
- **Cause:** The prepared `DesktopPanelTransition` wrapped the fullscreen
  launcher in `FadeTransition`, and Start only forwarded `_entrance` on open.
  When `visible` became false the host faded the entire child before any reverse
  board animation could read.
- **Resolution:** Make the launcher transition return the child without host
  opacity, lengthen the close hold to 420 ms, and reverse `_entrance` inside
  Start on ordinary dismiss (skip reverse after a launch handoff that already
  cleared the board).
- **Tried but failed:** Leaving the host fade and only reverse-driving tiles
  still looked like a uniform dissolve; the host opacity owned the frame.

### 2026-11-10 — tile drag broke under per-row parents and scroll gestures

- **Problem:** Start tile drag/avoidance jumped, failed to follow the pointer,
  or never activated; neighbors did not ease back from the drag-start snapshot.
- **Cause:** W10M painted joined rows as separate parent Stacks keyed by row Y,
  so a preview that moved a tile across rows remounted it and dropped
  AnimatedPositioned continuity. Tile `GestureDetector` pans also lost the
  gesture arena to the board `SingleChildScrollView`. Avoidance used a weak
  spiral search unlike legacy nearest-free ranking.
- **Resolution:** Keep every tile in one board-space AnimatedPositioned slot
  (W10M scales per-tile about the shared row origin; drag/settle uses a flat
  interactive stack). Track 8 epx activation with a raw pointer global route,
  lock scroll physics while dragging, and port `resolve_avoid_toward`/
  nearest-free ranking into `previewFreeformDrop`.
- **Tried but failed:** `Draggable`/`onPan*` on the tile face; both still lost
  to scrolling or remounted under per-row transforms during avoidance.

### 2026-11-10 — Fluent timing parity requires the actual curve, not its name

- **Problem:** The Flutter port's chrome overshot its destination and some
  helpers labelled Rust parity moved at a different speed from the legacy UI.
- **Cause:** Confirmed in source: Denial's workspace polynomial and layout
  Bézier deliberately overshoot; tile reflow used an OutBack Bézier. The port's
  `easeDecelerate`/`easeAccelerate` were quadratic instead of Rust's power 2.6;
  its source Expo/Cubic helpers were exponential/polynomial approximations
  rather than the legacy 14-iteration Bézier solver.
- **Resolution:** Share the numerical legacy curves between our controls and
  prepared desktop host, test bounded monotonic progress and source samples,
  and keep explicit Start-style timelines separate from ordinary chrome.
  The latest user decision removes bounce from normal Flutter tile reflow too.
- **Tried but failed:** A blue, square Material theme alone does not replace
  circular slider thumbs, ink ripples, icon-button hit feedback or host curves.
  Matching an easing name without matching its equation is insufficient.

### 2026-11-10 — Escape and the launcher close lifetime have separate owners

- **Problem:** Escape in focused Start search did not dismiss Start. A fixed
  host close hold could also cut off later staggered tile groups.
- **Cause:** EditableText owns a `DismissIntent` action for its toolbar, so the
  generic Start intent resolved to that action. The host retained the launcher
  for a fixed duration independent of saved layout/group timing.
- **Resolution:** Use a Start-specific intent, and evaluate the selected legacy
  close duration at each visibility change. Include the last group delay in the
  close duration. Inspect actual intermediate close frames, not just compilation.
- **Tried but failed:** Injecting Escape into the original generic DismissIntent
  path produced an unchanged Start screenshot; using a fixed 1100 ms hold is
  insufficient for arbitrarily many staggered groups.

### 2026-11-10 — preview captures must wait for a freshly painted frame

- **Problem:** Later Wayland preview captures showed an initial black desktop,
  empty taskbar and stale status icons, although the process remained alive.
- **Cause:** The initial suspicion of different prepared sources was disproved
  by comparing the clean kit against the previous kit. Runtime probes confirmed
  the expected startup environment, 216 desktop files and 67 loaded apps, with
  no catalog/network provider error. The capture read an old boundary layer;
  the Wayland host can defer frame callbacks when a preview is occluded.
- **Resolution:** Request fresh frames and await `endOfFrame` before reading the
  boundary, with a bounded timeout rather than silently delivering stale pixels.
  The refreshed capture showed the live Action Center and status state. Gesture
  exercises also wait for a painted frame before injecting preview-local input.
- **Tried but failed:** Repeating captures at a fixed delay, waiting longer on
  the timer alone, and blaming a fresh kit without checking runtime providers.
  Focusing the preview did not make the old one-shot image a valid fresh sample.

### 2026-11-10 — layered prepared-source patches must remain repeatable

- **Problem:** A second `prepare` failed after adding a dynamic Start close hold.
- **Cause:** An earlier patch used its previous replacement text as the
  idempotence marker, but a later patch had already changed that marker.
- **Resolution:** Make the original replacement emit the final dynamic-duration
  expression. Verify preparation against both an untouched release kit and an
  already prepared kit.
- **Tried but failed:** Adding another replacement for the intermediate fixed
  duration without updating the earlier patch's final-state check.

### 2026-11-10 — fresh-frame preview checks can be blocked by host presentation

- **Problem:** The final audio-detail exercise could not obtain its requested
  frame before the 8-second deadline, so no accepted screenshot was produced.
- **Cause:** Confirmed symptom is a missing `endOfFrame` completion; host
  occlusion/presentation is the best hypothesis, not a proven kit/source defect.
  Earlier runtime probes showed loaded applications and a resumed lifecycle.
- **Resolution:** Keep the timeout explicit and do not label the audio page
  visually accepted. Verify that page in a visible native session; retain the
  successful static/build and other preview evidence separately.
- **Tried but failed:** Extra fixed capture delay and temporary KWin activation/
  keep-above requests did not reliably obtain a frame. Those scripts were
  unloaded and their task-owned test process stopped. Do not silently fall back
  to an old PNG or call a stale image a successful visual check.

### 2026-11-11 — new shell modes need shared geometry, not layout aliases

- **Problem:** A narrow Start layout still had desktop chrome, and early mode
  work could draw tiles at one cell size while dragging/resizing used another.
- **Cause:** `start_layout_style` described the tile renderer, not the whole
  shell. The drawing path used a changed grid size while drag/resize retained
  hard-coded 70 px cells; drag snapshots omitted the new mode/density fields.
- **Resolution:** Persist `shell_mode` independently, use shared grid helpers for
  drawing/resize/drop and mode-aware snapshots, and share launcher/Action Center
  bounds with the native input publisher. Add round-trip, responsive projection,
  disjoint-group and geometry tests. Never infer phone mode merely from width.
- **Tried but failed:** Renaming the existing Windows Phone tile layout into a
  phone mode would preserve the wrong taskbar, status placement and edge
  commands. Repeated independent geometry constants broke drop targeting.

### 2026-11-11 — never overwrite an AOT library mapped by a running preview

- **Problem:** A phone preview completed its reachability capture, then died
  with SIGBUS while another rebuild published over the same bundle.
- **Cause:** The helper used `shutil.copy2` directly onto `bundle/lib/libapp.so`
  (and replaced its asset directory). A live engine maps that file; truncating
  the inode can invalidate its pages. Changing only the timestamp or waiting
  for a successful compile does not protect a mapped library.
- **Resolution:** Build/verify a separate generation, atomically publish the
  latest-bundle symlink, and resolve that symlink once for preview/session paths.
  Keep older generations so lazy font loads stay with the matching code.
  `tools/test_denial_bundle.py` verifies live mmap bytes and a pinned asset path
  both survive subsequent publications, including initial directory migration.
- **Tried but failed:** In-place copying even when the rebuilt library's contents
  are otherwise valid; replacing assets under a live process can also mismatch
  an old AOT image and a new tree-shaken font. Stopping only at source-digest/ABI
  checks does not test this runtime publication boundary.

### 2026-11-11 — retain phone navigation above a modal task switcher

- **Problem:** The first phone task-card popup covered the Back/Start/Search bar.
- **Cause:** Managed popups intentionally own a full-scene modal layer. A normal
  surface below it cannot remain the navigation control for that layer.
- **Resolution:** Compose live status/navigation bars inside the phone task
  surface; Start/Search explicitly dismiss owned popups before changing the
  native launcher. Keep the same real window IDs and back/home commands.
- **Tried but failed:** Assuming the ordinary above-window bar could render or
  receive taps through an SDK-owned modal popup.

### 2026-11-11 — ARM64 native session startup differs from UI preview

- **Problem:** The first VT2 phone session passed the native-app `--check` but
  Denial aborted with `shell has no libflutter_engine.so`; the local rollback
  returned to existing Plasma, same boot and working SSH.
- **Cause:** Under investigation: the compositor resolves its engine differently
  from `denial-app --engine`; validate the exact pinned loader before changing
  package paths. ARM64 ELF and glibc checks passed, so architecture mismatch is
  not supported by this evidence.
- **Resolution:** Pending loader inspection and a corrected native-session run.
- **Tried but failed:** Treating successful explicit-engine preview ABI checks
  as validation of the compositor's engine discovery path.

### 2026-11-11 — confirmed Win10X ARM64 startup permission failure

- **Problem:** Native compositor reported no Flutter engine although staged ABI checks passed.
- **Cause:** Confirmed by `namei -l`: bundle generations originate in `mkdtemp` (0700). Copying that directory and changing ownership to root made the installed shell inaccessible to rong; the generic loader message hid EACCES.
- **Resolution:** Make ARM64 published/staged bundle roots 0755 and normalize the installed public runtime with `a+rX`. The same explicit-engine check now passes as rong against the root-owned /opt installation. Native retry follows; Plasma and boot ID remained unchanged.
- **Tried but failed:** User-owned staging checks alone, and interpreting the engine discovery message as a missing file or wrong architecture.

### 2026-11-11 — automatic VT login displaced the desktop test

- **Problem:** After fixing package permissions, two VT2 test starts exited successfully before ExecStart/PAM produced a native log.
- **Cause:** Confirmed journal ordering: changing to VT2 starts `getty@tty2.service`; its conflict stops the test service before ExecStart. MainPID snapshots briefly showing a process were not proof of compositor startup.
- **Resolution:** Move the isolated test to unused VT8 outside the default automatic VT range; leave the existing Plasma VT1 and login configuration unchanged. Local timeouts still return to VT1.
- **Tried but failed:** Removing TTYReset/TTYVHangup and redirecting native output; these did not address automatic getty activation.

### 2026-11-11 — first touch-mode placement must configure the real client

- **Problem:** First real phone Konsole rendered in the large phone viewport
  with visibly stretched glyphs; its committed buffer retained the smaller
  initial desktop height. UI-only preview could not reveal this.
- **Cause:** The touch policy adjusts a newly discovered window immediately,
  but upstream `DesktopWindowConfigureTracker` suppresses its first configure
  assuming the initial native geometry remains authoritative. Its cache then
  sees the adjusted rectangle as unchanged and never requests that client size.
- **Resolution:** Permit the initial configure in touch modes when the received
  client size differs from the requested content rectangle; preserve the native
  drag single-writer rule and desktop's original startup behavior. Native
  rebuild/retest is in progress; acceptance requires a new real-client capture.
- **Tried but failed:** Considering maximized Flutter placement alone proof of
  client resize; an existing client texture can stretch to that placement.

### 2026-11-11 — VT8 stop handler returned the phone to Plasma

- **Problem:** After a Denial/session stop the phone appeared to "crash back"
  to KDE even when Win10X had been the intended desktop.
- **Cause:** The VT8 test unit used `ExecStopPost=+/usr/bin/chvt 1`. Any stop
  (timeout, crash, manual restart) switched the active VT to Plasma on tty1.
  Earlier temporary units also carried `RuntimeMaxSec` which forced the same
  path after a few minutes.
- **Resolution:** Persistent unit with `Restart=on-failure`, no RuntimeMaxSec,
  and no `chvt 1` on stop. Keep Win10X on tty8 across failures; switch to
  Plasma only intentionally.
- **Tried but failed:** Treating an active Plasma session on tty1 as proof that
  deniald had exited when the real issue was the stop-post VT switch.

### 2026-11-11 — Start open force-sync must not toggle-close

- **Problem:** Pressing Start made the launcher flash once and disappear.
- **Cause:** Opening Start runs `_navigationChanged(force: true)`. The pending
  request is still `StartRequest.home` (from the open press / default). The
  new toggle-close path treated "already on tiles" as a second Start press and
  called `onDismiss` immediately.
- **Resolution:** Only dismiss on `home` when `force` is false (a real revision
  bump while Start is already visible). Force sync on open may land on tiles
  but must not close.
- **Tried but failed:** Relying on preview launch alone without capturing after
  settle; the flash looked like a successful open in the first frames.

### 2026-11-11 — release MaterialIcons subset stays stale for brand-new glyphs

- **Problem:** `tools/win10x-denial bundle` failed verify with
  `Release icon font is stale; missing glyphs: …` after adding new
  `Icons.*` constants.
- **Cause:** Flutter 3.47.5's `release_bundle_linux-x64_assets` can rebuild
  without regenerating the tree-shaken MaterialIcons subset when only the
  assets stamp is cleared; unused-new glyphs never enter the OTF.
- **Resolution:** Prefer glyphs already used by the shell (e.g. `Icons.apps`),
  or wipe the full `flutter_build/*/…assets` outputs and force a dill-aware
  assets rebuild before verify.
- **Tried but failed:** Deleting only
  `release_bundle_linux-x64_assets.stamp` — assemble still emitted the old
  subset.

### 2026-11-11 — Cerro ARM64 blocked by network, not sudo

- **Problem:** `WIN10X_ARM64_HOST=cerro-usb` failed during Goal packaging.
- **Cause:** `192.168.68.2` (USB ECM) timed out / 100% ping loss; no USB
  gadget present in `lsusb`. Fallback `cerro-wifi` (`192.168.0.117`) returned
  “No route to host” even though the workstation is on `192.168.0.229/24`.
- **Resolution:** Leave PK2 `blocked` until Cerro is back on USB ECM or Wi-Fi;
  continue x86_64 analyze/test/bundle/preview without claiming arm64.
- **Tried but failed:** Assuming the prior sudo failure was still the blocker;
  network reachability failed first.

### 2026-11-11 — ARM64 AOT without Cerro via qemu-user-static

- **Problem:** Cerro USB/Wi-Fi unreachable blocked `win10x-denial-arm64` SSH AOT.
- **Cause:** Tool assumed a live aarch64 SSH host for `gen_snapshot`.
- **Resolution:** Install `qemu-user-static` + `aarch64-linux-gnu-glibc`, set
  `WIN10X_ARM64_HOST=qemu`, and run the kit's aarch64 `gen_snapshot` under
  `qemu-aarch64-static -L /usr/aarch64-linux-gnu`.
- **Tried but failed:** Bare `qemu-aarch64-static gen_snapshot` without
  `QEMU_LD_PREFIX` / `-L` — missing `/lib/ld-linux-aarch64.so.1`.

### 2026-11-12 — popup builders are deferred; feedback must not become modal

- **Problem:** ApplicationBar returned `box.value!` immediately after `show`,
  although `show` stores its builder for a later frame. MessageBox futures could
  remain unresolved when Start/mode changes removed the popup. Feedback toasts
  captured the whole scene and keyboard; toasts with actions never expired.
- **Cause:** Widget construction was mistaken for synchronous popup creation.
  Toasts inherited the managed popup defaults (`fullScene`, keyboard capture,
  scrim, focus restoration). Their timer was bound to a second handle replaced
  when the deferred builder ran. Independent host/body transitions compounded
  the popup motion, and generic Back bypassed the body's exit animation.
- **Resolution:** Allocate controllers before `show`; observe removal even before
  the body mounts; complete cancelled dialogs; route phone Back/Escape through
  the owned exit controller. WP menus own their motion without a second host
  fade. Keep phone navigation inside modal layers. Toasts use a bounded,
  non-modal overlay and dispose their own timer, including actionable feedback.
  Visual app-switch proxies neither acquire nor restore keyboard focus and clip
  to the application viewport, leaving taskbar/status/navigation visible.
- **Tried but failed:** Static analysis and pure model tests did not detect the
  deferred builder null reference or modal input policy. Run opt-in actual
  preview exercises, including immediate removal before a builder mounts.

### 2026-11-12 — edge follow callbacks changed the command being invoked

- **Problem:** Tablet left-edge swipe opened Action Center instead of Task View.
  A new drag could also fight an older settle ticker; cancelled recognition
  could leave an intermediate panel reveal value behind.
- **Cause:** The shared edge widget's `onFollow`/`onSettle` branch returns before
  `onOpen`; dashboard callbacks attached to the left edge therefore bypassed its
  Task View callback. Settles had no ownership generation and cancellation path.
- **Resolution:** Only Action Center edges use DashboardReveal. Retain left-edge
  native Task View routing. New drag progress invalidates older settles;
  cancelled gestures settle closed. Hold zero progress through host retention
  before clearing it, avoiding a fully open flash during close.
- **Tried but failed:** Checking that `onOpen` names Task View without following
  the earlier return in `_EdgeStripState._end`. Empty-preview Task View has no
  real window cards, so it cannot prove native activation or app containment.

### 2026-11-12 — compare turnstile face and content projections separately

- **Problem:** Disco's background and its icon/caption diverged during rotation,
  despite a correct settled drawing and copied timing constants.
- **Cause:** The face incorrectly used the inner-content angle/translation;
  the child used only an outer rotation and included a second opaque tile face.
  A 3D rotation in Flutter alone was not equivalent to Rust's projected points.
- **Resolution:** Project the face with outer rotation only. Use a transparent
  content layer and an exact homography matching Rust's inner-center then
  outer-left projection, including depth and final translation. Added numeric
  tests against `project_turnstile_point` for wide/tall/square faces and the
  settled identity geometry. Original timing and selected animation remain.
- **Tried but failed:** Calling a projected background plus a separately rotated
  opaque child “true perspective parity”; checking only final frames hides it.

### 2026-11-12 — classic phone density also needs full-width cell geometry

- **Problem:** Correcting phone density from 6/8 to the selected launcher's 4/6
  made the new expected-width check fail (134 actual versus 174 expected at a
  360 px board); classic mode left unused space instead of filling its row.
- **Cause:** The shared cell helper still capped phone cells at 70 px, a hidden
  assumption from the previous six-column default. Four columns need 90 px
  cells at that board width. Stored tile coordinates were not the cause.
- **Resolution:** Phone drawing/drag/resize share width divided by the selected
  column count; desktop/tablet retain their existing cap. Check both densities
  and preservation of stored cells in modes_test.dart.
- **Tried but failed:** Changing only the column count and assuming responsive
  cell sizing followed it; the focused test exposed the separate cap.

### 2026-11-12 — Start toggles belong only to user presses

- **Problem:** PC taskbar Start bypassed the page-aware navigation command and
  closed All Apps immediately. Automatic “last window closed” home requests
  could also toggle an already-visible Start off after the new all-mode toggle
  requirement was added.
- **Cause:** PC used `services.toggleLauncher` directly; automatic and user home
  paths shared a toggle without checking whether Start was already visible.
- **Resolution:** PC/phone user presses use the same page-aware home request.
  Automatic startup/last-window requests only open Start when it is hidden.
  Start-close/reopen/All-Apps-return is exercised independently of tile layout.
- **Tried but failed:** Checking phone's navigation bar alone did not cover the
  separate desktop/tablet taskbar callback or the automatic window listener.

### 2026-11-12 — ARM64 asset cache omitted newly used media glyphs

- **Problem:** A freshly built current-source ARM64 AOT passed ELF/checksum
  checks, but an explicit UI-font coverage check found `skip_next` and
  `skip_previous` missing. The x86_64 bundle contained both glyphs.
- **Cause:** The separate `release_bundle_linux-arm64_assets` target retained an
  older tree-shaken font while recompiling Dart. The ARM64 helper had neither
  the x86_64 source-change invalidation nor an icon coverage gate.
- **Resolution:** Remove the ARM64 assets stamp and disposable assembly assets
  before assembly, keeping the expensive AOT cache and immutable live bundles.
  Check every shell MaterialIcons codepoint before publishing and packaging.
  The new package gate reproduced the old bundle failure before rebuilding.
- **Tried but failed:** Assuming host icon validation also covers a separate
  ARM64 assembly cache, or treating current libapp.so/source hashes as evidence
  that the accompanying icon font is current.

### 2026-11-12 — MarkUI files present does not mean icon coverage

- **Problem:** Most visible command/status/application icons were still Material
  or native theme icons although MarkUI files had been added to assets.
- **Cause:** Application call sites still delegated directly to Denial's icon
  builder; only four navigation glyphs were wired, without a package-qualified
  asset lookup. Some previously copied base app images were solid-face variants
  instead of the pack's recognizable `*8` flat glyphs. Source hashing omitted
  bitmap contents, so asset-only edits could pass stale-source checks.
- **Resolution:** Use original flat variants, shared MarkUI command/status
  rendering, exact application aliases and package-relative asset lookup. Hash
  shell assets and verify every packaged PNG against source for both targets.
  Keep exact source/checksum provenance; disclose WP vector adaptations and
  unmapped third-party artwork rather than calling them original pack assets.
- **Tried but failed:** Checking pubspec or copied file count alone; static IconData
  keys cannot be used in a const Dart map because IconData overrides equality.

### 2026-11-12 — SDDM portrait scale and current-session evidence

- **Problem:** Breeze login controls were crowded at 200% on the phone; a
  screenshot probe first found no greeter process, then was refused by KWin.
- **Cause:** Greeter scaling is independent from the Denial output scale. Linux
  comm-name matching is limited to 15 characters; a direct Spectacle SSH launch
  lacks KWin's restricted screenshot authorization. Later absence of greeter was
  normal: the user had already authenticated into Win10X. A new boot ID also
  proved a reboot, so USB disappearance was not solely host network state.
- **Resolution:** Set greeter scale to 125%; user confirmed controls fit. Verify
  current boot/session and SDDM authentication journal. Keep screenshot access
  checks intact. Host USB NM profile prevents unrelated Wi-Fi changes removing
  the manually configured USB address.
- **Tried but failed:** pgrep -x sddm-greeter-qt6; treating root/Spectacle invocation
  as screenshot authorization, or interpreting missing greeter as startup failure.

### 2026-11-12 — verify opaque catalog IDs and PNG visible bounds

- **Problem:** First actual MarkUI preview still showed Firefox/Dolphin system
  icons; package-qualified navigation became visibly too small.
- **Cause:** Real Denial LauncherContext IDs use `desktop:` / `local:` prefixes,
  unlike window app IDs. Initial alias tests only used plain/path desktop IDs.
  Three original navbar PNGs are 222×144 with only ~60×60 visible content;
  merely fixing their formerly missing package path exposed that padding.
- **Resolution:** Normalize only known opaque prefixes; test real desktop/local
  identities. Compensate original navbar padding in the clipped widget, keeping
  source pixels unchanged. Inspect actual rendered app list and navigation.
- **Tried but failed:** Counting mapped aliases or treating original bitmap
  dimensions as the visible glyph size. The first preview, not passing model
  tests, exposed both errors; it was not installed as accepted on the phone.

### 2026-11-12 — native reveal and retained geometry survive shell motion ports

- **Problem:** User still saw Denial-style app opening and reported squashed /
  stretched application contents after the Rust-style shell animations existed.
- **Cause:** Confirmed separate code paths: TrackedDesktopWindowReveal still
  enabled a random-quarter squircle with a lead-in; retained positioning used
  independent X/Y scales; ordinary local Flutter clients were forced through a
  retained BoxFit.fill. The first touch configure also required non-null native
  geometry, allowing its target cache to suppress the needed first request.
  These explain distortion-capable paths; exact affected native apps remain
  unconfirmed while the phone is disconnected.
- **Resolution:** Remove the native initial reveal, retaining our selected
  handoff. Share proportional client fit between paint, retained hit transforms,
  native routing and popups; let normal local Flutter content reflow. Always
  permit the first touch placement configure. Add aspect/coordinate invariant
  tests and an actual-rendered retained/client fixture with in-animation checks.
- **Tried but failed:** Porting easing constants alone does not disable a second
  animation controller; maximizing a visual rectangle or smoothing the texture
  does not cause the real application to submit a newly sized buffer. Static
  model tests do not establish native-client acceptance.

### 2026-11-12 — alphabet rail needs shared list geometry and gesture ownership

- **Problem:** Phone side letters only handled taps; jumps guessed48px header
  offsets while header widgets sized themselves, and ignored list padding.
  The narrow single-column case could fall back to an unmounted GlobalKey just
  because the caller allowed columns. A distant lazy section cannot be found
  with ensureVisible before it has been built.
- **Cause:** Separate jump implementations and implicit geometry; no continuous
  rail drag owner. Group insertion order could also disagree with the rail's
  #/A–Z order. Per-letter animateTo calls during a scrub would fight each other.
- **Resolution:** Shared CatalogIndex with explicit48px headers, real padding,
  fixed row extents and lazy list building. Use actual resolved column count,
  known offsets for distant sections, immediate scrub jumps, cancellable tap
  jumps, selected-letter feedback and matching group order. Capture services/
  monitor before deferred alphabet popup construction rather than watching a
  disposed owner ref. Keep phone navigation inside the alphabet picker.
- **Tried but failed:** Widget keys alone cannot locate offscreen lazy headers;
  adding only onVerticalDragEnd cannot provide finger-follow alphabet lookup.

### 2026-11-12 — alphabet maximum extent and preview target ownership

- **Problem:** The first rail exercise reached no section; after scoping its
  target, a real far-end scrub still stopped short. Preview app-grid glyphs also
  became filled circles under monochrome tint.
- **Cause:** The test first matched the kept-alive, offscreen Start app-list rail
  instead of its popup fixture. Separately, SliverVariedExtentList delegates its
  maximum to the visible-children average, even with known item extents; clamping
  a distant target to that estimate can stop early and change the maximum after
  the jump. Some original MarkUI notification/theme/check bitmaps encode their
  inner detail in RGB on an opaque badge; srcIn tint erases that detail.
- **Resolution:** Scope semantic lookup to the exercised catalog. Supply an
  exact full child extent through a lazy SliverChildBuilderDelegate and share
  section offsets/total extent. Select original transparent glyph variants from
  the same MarkUI APK (apps/check/message/play/theme), preserving source bytes
  and provenance, then inspect the actual tinted rendering.
- **Tried but failed:** A generic whole-tree semantic lookup is ambiguous when
  outgoing pages stay mounted. itemExtentBuilder alone does not promise an exact
  maximum. A successfully decoded PNG and a source-pack match do not prove its
  alpha silhouette is appropriate for monochrome tint.

### 2026-11-12 — phone fixture must reserve the live navigation strip

- **Problem:** After correcting the far-end extent, a reverse alphabet scrub
  opened search/keyboard instead of moving back to A.
- **Cause:** The test popup gave its catalog the full screen; its bottommost
  rail point overlapped the real, higher-layer Search navigation key. A drag
  starting there correctly went to Search. The normal Start catalog reserves
  chrome space; the opt-in fixture did not.
- **Resolution:** Reserve24px status and48px navigation space inside the fixture
  and derive injected coordinates from that scoped rail's real bounds.
- **Tried but failed:** Treating the resulting offset assertion as another list
  bug; the rendered keyboard revealed that the pointer hit a different control.

### 2026-11-12 — re-resolve scroll position after managed popup rebuilds

- **Problem:** The alphabet-grid exercise reported offset112 (A) after selecting
  M, while the actual captured list visibly showed M correctly.
- **Cause:** Confirmed stale test reference: the exercise retained an earlier
  ScrollPosition across managed popup/theme reconstruction. The catalog's own
  controller used its current position; the old captured position stayed at A.
- **Resolution:** Re-resolve the scoped ScrollableState.position for each check.
  The same grid→M→rail→Z flow then passed with matching rendered contents and
  exact current offsets. Allow bounded settling of the nonlinear jump.
- **Tried but failed:** Waiting two more seconds on the obsolete position;
  extending a timer cannot make a detached position represent the live list.

### 2026-11-12 — phone alphabet grid must account for landscape height

- **Problem:** The corrected live-position exercise still could not select M in
  landscape; M's square grid cell lay below the visible picker viewport.
- **Cause:** A fixed six-column square grid uses width to determine row height.
  At780×390, five such rows exceed the height remaining above phone navigation.
  An injected offscreen coordinate hit the outside dismissal surface instead.
- **Resolution:** Bound phone grid row height by both column width and one fifth
  of available picker height, with44px minimum touch targets and scroll retained
  for genuinely smaller windows. Keep explicit zero grid padding and preserve
  portrait square cells. Add a landscape picker capture as well as its selection
  exercise. Stop the task-owned superseded ARM64 compiler before rebuilding the
  corrected source; no intermediate build is deployed.
- **Tried but failed:** Re-resolving position or extending a settle timeout cannot
  select a grid cell that is outside the clipped viewport. A portrait-only picker
  screenshot misses this geometry defect.

### 2026-11-12 — synthetic light-theme fixture must use the real phone theme

- **Problem:** Light alphabet preview had white text on a white surface despite
  its navigation assertions passing.
- **Cause:** The opt-in fixture chose a light Material background but left
  wrapPhone's default dark theme enabled. Production phone builders already pass
  the saved theme. This was a fixture mismatch, not evidence of native acceptance.
- **Resolution:** Pass saved dark/accent settings to the fixture as production
  does and inspect its rendered light text/icons after rebuilding both targets.
- **Tried but failed:** Successful pointer/offset assertions alone cannot verify
  contrast; a fresh screenshot caught the mismatched theme.

### 2026-11-12 — phone authorization and keyboard session boundary

- **Problem:** User reports no administrator dialog and unusable OSK.
- **Cause:** Installed isolated session explicitly disabled the polkit agent;
  upstream agent defaults also refer to distribution paths, not /opt prefix.
  Keyboard auto-open tests ReferenceShellProfile.mobile while our phone shell
  deliberately retains desktop native containment.
- **Resolution in progress:** A task-owned transient agent with explicit pinned
  runner/bundle/engine registered for the current graphical login. Safe pkcheck
  against compositor PID2525 launched a real native dialog, captured in ignored
  controls QA. No privileged action executed. Persistent startup and credential
  flow remain pending. Patch only text-input auto-open gate for touch shell
  modes, avoiding the separate mobile auto-minimize policy; OSK WP styling and
  nonlinear non-spring settle added to pinned prepare, analyze passed.
- **Tried but failed:** The phone has no rg; use grep there. Do not infer that
  launching Maliit solves an inactive built-in broker; both keyboard owners
  must not be started speculatively. Upstream keyboard file has no dart:math
  import, so the first patch context failed safely before building.

### 2026-11-12 — runtime refresh crosses Denial's secure boundary

- **Problem:** After sourcecb4f17bab749 was verified/staged and SIGUSR1 refreshed
  Flutter in process, compositor remained alive but screenshots were denied.
- **Cause:** Native ui status reported generation2 and locked=true. /proc/2525/maps
  confirmed the new immutable ARM64 AOT mapped; same compositor PID/boot remained.
  The refresh crosses the native authentication boundary and relocks the session.
- **Resolution in progress:** Asked user to unlock normally and exercise actual
  search keyboard. Do not bypass lock or call denied captures proof of a crash.
  Cancelled the120s rollback timer after new AOT/healthy native status confirmed;
  retained root-owned rollback script and old generation. App processes preserved.
- **Tried but failed:** UI-only keyboard fixture found retained hidden key widgets;
  actual offstage checks exposed that native text-input delivery is unproven.
  Focus/tap changes alone did not make the standalone bridge show its keyboard.
  Move to real session evidence rather than claiming visible components work.

### 2026-11-12 — optical fit must use the resolved icon slot

- **Problem:** Explicit-size icon gallery passed, but Start showed small file /
  notes glyphs beside larger native icons.
- **Cause:** New optical fit used requested IconTheme extent before layout. A
  tight parent tile overrode SizedBox's requested dimensions, while the bitmap's
  already-computed optical scale stayed small. This was a regression in the
  candidate's new algorithm, caught by real Start capture.
- **Resolution in progress:** Compute optical fit inside LayoutBuilder using the
  resolved shortest slot side. Keep uniform source scaling, alpha centering and
  pixel rounding. Add a parent-forced-size original file glyph to the gallery,
  enlarge its test viewport to fit all rows, then inspect Start again.
- **Tried but failed:** Testing only explicit size arguments never exercises a
  parent that overrides the requested icon size. Numeric aspect tests alone
  cannot establish that the caller's actual layout constraints are respected.

### 2026-11-12 — user could not unlock after runtime refresh

- **Problem:** User reports the refresh-induced lock could not be unlocked; they
  pressed Power and the phone rebooted. Earlier healthy-PID/new-AOT checks did not
  establish a usable session, so cancelling the rollback timer was insufficient
  acceptance. Do not repeat SIGUSR1 on the active phone session.
- **Cause:** Secure refresh relocked; unusable unlock input is confirmed by user,
  exact lock UI cause unconfirmed. Old live runtime had the desktop default power
  action; the new wrapper's missing-setting DPMS seed only takes effect at login.
  Do not label a reboot's cause proven without power/native logs.
- **Resolution in progress:** Reconnected after user reboot. Bootc3b56429, native
  PID2427/session3, generation1/unlocked; cb4f17bab749 remains selected. Settings
  now show powerButtonAction=dpms. Actual native startup spawned the new relative-
  prefix agent PID2545 without the old transient unit. No new reboot/refresh issued.
- **Tried but failed:** Treating a successful hot refresh as safe to leave with
  an untested lock screen. Future deployment must stage for a normal session
  transition or use a verified method that preserves secure state; no repeated
  lock/unlock requests to the user for routine UI iteration.

### 2026-11-12 — native lock must replace the secure layer, not add a scene widget

- **Problem:** Previous lock chrome was not wired into denial/main's surfaces,
  while native ShellSecureStage independently displayed its upstream lock layer.
  User explicitly reopened lock/unlock design after an unusable refresh lock.
- **Cause:** A normal aboveWindows surface cannot safely replace the native lock
  lifecycle. Its immediate auth.begin also ran before auth sync and could no-op;
  a full-output column could be obscured by OSK instead of scrolling.
- **Resolution in progress:** Add a presentation-only secure-stage lock builder,
  retain native frame acknowledgement/isolation/PAM/unlock transition. Picture
  cover reveals scrollable credentials on finger-follow swipe; auth begins only
  after native synchronization and availability. Clear the secret on submit /
  return / disposal; return cancels the native conversation. Never unlock by
  completing a UI animation. Test layout/gesture in opt-in fixture first.
- **Tried but failed:** Adding another shell overlay or treating successful AOT
  refresh as usable lock acceptance. No further active-session SIGUSR1 testing.

### 2026-11-12 — repeated generated-kit preparation must start from pristine inputs

- **Problem:** Generated desktop_shell.dart accumulated seven duplicate command
  bindings and HUD listeners during analyze/test/bundle repetitions.
- **Cause:** Adjacent insertion patches both retained `return ShellActionsBinding`.
  The second insertion split the first replacement's after-string; substring
  idempotence checks no longer recognized it on the next preparation.
- **Resolution:** Restore checksum-pinned runtime inputs before every preparation,
  then apply each composition patch once. Verify repeated preparation generates
  identical file digests and exactly one command/HUD binding.
- **Tried but failed:** Restoring only when the preparation script digest changes
  does not prevent non-idempotent edits during repeated unchanged commands.

### 2026-11-12 — keyboard Home and long dialogs need behavioral checks

- **Problem:** A forced-keyboard preview left Home reachable visually but Home
  did not dismiss keys; a long MessageBox hid its action buttons below content.
- **Cause:** Home only called the native callback if Start was not already open.
  The dialog put title/body/actions inside one scrolling column.
- **Resolution:** Dismiss keyboard explicitly for Home/search/reveal commands,
  regardless of launcher visibility; scroll the flexible body while retaining
  title and actions. The same preview failed before and passes after these fixes.
- **Tried but failed:** Empty keyboard padding and a successful layout alone do
  not prove navigation dispatch; overflow-free scrolling alone does not keep
  confirmation/cancel controls visible.

### 2026-11-12 — custom auth must build with the patched local engine

- **Problem:** First x64 auth presentation build attempted to download a stock
  Flutter engine; a second agent instance refused registration during live update.
- **Cause:** Auth helper omitted the pinned local-engine arguments on x64.
  `/proc/PID/task/PID/children` was unavailable in the first idle-agent guard, so
  the original agent had not actually stopped before the trial registration.
- **Resolution:** Use matching local engine for both architectures, independent
  source/entry stamps and production/preview metadata. Root `pgrep -P` confirmed
  no dialog child before replacing the idle backend. Exactly one live task-owned
  agent registered. Preserve rollback wrappers and immutable old/new bundles.
- **Tried but failed:** A healthy backend registration cannot establish native
  dialog appearance: a subsequent safe pkcheck was immediately dismissed while
  the phone had idle-locked. Do not treat that as styled-dialog acceptance or
  repeat active-session SIGUSR1 to work around the lock.

### 2026-11-12 — native lock tests must wait for secure-stage layout

- **Problem:** Native test displayed the new picture lock but its synthetic swipe
  did not reveal credentials; no PAM prompt followed. Auth capability was available.
- **Cause:** Test computed `localToGlobal` before the pending secure-stage frame
  moved the lock from its prior unlock position. Hit diagnostics contained only
  RenderView/binding, so the gesture was outside the laid-out UI, not a PAM failure.
- **Resolution:** Await the secure frame before reading geometry, inject in the
  real view's logical viewport, then check the actual hit path and revealed state.
  On sourceabc5519c9fa9, bounded VT8 test reached the real PAM prompt, automatically
  opened OSK and delivered/cleared a test character in the secure password editor.
  No credential submitted. Native compositor returned to tty2 automatically.
- **Tried but failed:** Increasing auth wait without first proving the cover
  received the gesture. Release-mode Offset/Size.toString hides numeric values;
  log only explicit nonsecret geometry/state booleans for useful diagnostics.

### 2026-11-12 — native keyboard fixture must accept sentence capitalization

- **Problem:** GTK keyboard check reported missing semantic target `a`, although
  the real input field had automatically shown a usable OSK.
- **Cause:** GTK's content hints requested sentence capitalization: the first
  letter's semantic label was `A`. A screenshot confirmed the active Shift key.
  An earlier attempt never launched because its desktop ID lacked `.desktop`.
- **Resolution:** Require a successful bridge launch; match alphabetic test key
  case-insensitively and verify receipt in the temporary GTK editor independently.
  Keep the active-editor policy independent of arbitrary screen taps.
- **Tried but failed:** Treating missing lowercase test semantics as a native
  keyboard activation failure or omitting SDK launch validation return values.

### 2026-11-12 — Chromium input QA needs a fresh bounded profile

- **Problem:** First temporary Chromium capture showed a new-keyring password
  dialog, and a repeated capture showed its prior unclean-session restore bubble;
  both stole focus from the test textarea after an initial keyboard activation.
- **Cause:** A private DBus/data root has no default keyring; ending a bounded
  compositor disconnects test clients. Reusing that temporary browser profile
  carries its expected unclean-exit state into the next experiment.
- **Resolution:** Give each browser test a fresh task-owned profile and use basic
  password storage only for this empty fixture, with no user account or data.
  No IME/Ozone-enabling flags added; observe native text-input-v3 enter/enable
  and the actual textarea/key result. Existing browser profiles/flags untouched.
- **Tried but failed:** Calling an early OSK-visible event a completed browser
  input test without inspecting its later focused surface. Recursively chowning
  a temporary Chromium profile also follows dangling SingletonLock symlinks;
  use no-follow ownership for generated fixture trees.

### 2026-11-13 — screen-power presentation sits above shell Directionality

- **Problem:** First wake preview rendered a null-check exception at
  AlignmentDirectional.resolve before the feature scene mounted.
- **Cause:** ScreenPowerHost wraps the Denial feature root; its Stack initially
  used AlignmentDirectional.topStart, but Directionality belongs to the inner
  shell. Compilation/analyze did not exercise that first layout.
- **Resolution:** Give the outer Stack an absolute Alignment.topLeft and use
  optional MediaQuery at this boundary. Rerun current-bundle wake/settings/nav
  fixtures; never treat a successful screenshot write as absence of UI errors.
- **Tried but failed:** Running captures without checking runtime logs. A build
  during ongoing edits was also correctly rejected by the source digest guard;
  keep source stable for final analyze/test/bundle/preview instead of packaging
  whatever AOT happens to be present.

### 2026-11-13 — wallpaper selection should not depend on external dialog apps

- **Problem:** An external zenity/kdialog entry is not a complete wallpaper UI
  for a minimal phone image, and a process launched directly from embedded
  Flutter may not inherit Denial's client-specific Wayland activation context.
- **Cause:** Direct Process.run is not the native desktop-application launch
  bridge; installed picker packages and client endpoints cannot be assumed.
- **Resolution:** Supply a managed in-shell file picker with directory browsing,
  typed paths, cancellation and image decoding before committing. It runs inside
  the same surface/input system; no extra package or host display is required.
- **Tried but failed:** Throwing from precacheImage's asynchronous error callback
  is not reliable validation. Decode a bounded-size candidate image explicitly,
  report failure in the picker, and keep the old selection intact.

### 2026-11-13 — native volume presentation must replace the actual HUD branch

- **Problem:** User still sees the bottom volume popup on the phone; an earlier
  real-device capture had shown both top and bottom. Recent fixes were local-only
  and were never deployed to that older installed generation.
- **Cause:** Desktop composition forwarded native audio events into an extra
  overlay while the upstream SystemLevelHudLayer retained its own presentation.
  That extra overlay also depended on desktop initialization and lock visibility.
- **Resolution:** Route the actual phone audio branch in SystemLevelHudLayer to
  the WP top renderer, with native visible/revision/dismissal lifecycle. Remove
  the parallel desktop/lock HUD instances and forwarding listener; register the
  top controls as a native input region. Native limit events share that route.
- **Tried but failed:** An ordinary volume-panel layout fixture never exercises
  the native HUD route. The new scoped audio-signal fixture initially reused its
  parent's already-created provider; explicitly scope the controller as well as
  its signals before treating a missed synthetic event as a product failure.
  Device behavior cannot change until a new ARM64 build is actually deployed.

### 2026-11-13 — release captures can hide fixture overflow

- **Problem:** A landscape Chinese keyboard test wrote a screenshot and reported
  no runtime exception, but its last key row extended below the image.
- **Cause:** The test used a portrait-fixed330px keyboard plus a title and bottom
  padding inside a390px viewport. Flutter release did not emit the debug overflow
  assertion. This was the fixture's column, not the native keyboard size policy.
- **Resolution:** Size the fixture to its available viewport and inspect the
  actual capture, including bottom keys. Keep log checks as well as visual review.
- **Tried but failed:** Equating a capture file and empty error list with complete
  layout acceptance, or blindly replacing all linear controller clocks even when
  their rendered wallpaper/clock/tile curves are already nonlinear.

### 2026-11-13 — Action Center dismiss used a different distance from its paint transform

- **Problem:** The panel ran ahead of a finger and could jump fully open when
  grabbed during settle, or retain a partial progress after cancellation.
- **Cause:** Phone close normalized drag against55% of the screen while native
  presentation translated the complete panel height plus entry distance. Start
  reset progress to1; cancellation had no handler. A recognizer losing to an
  inner scroll can also cancel without ever beginning a panel drag.
- **Resolution:** Use actionCenterGeometry for both normalization and custom
  translation; preserve current progress as grab origin; restore nonlinearly on
  cancel and guard cancellation with an active-drag flag. Keep child scrolls /
  sliders independent. Toast also clears its interrupted closing flag on grab.
- **Tried but failed:** Adding easing to raw drag motion would mask the wrong
  scale by creating lag. Linear clocks with nonlinear rendered transforms are
  not a missing animation; input remains1:1 until release.

### 2026-11-13 — accepted Flutter drags report PointerCancel through onEnd

- **Problem:** The toast follow test moved correctly, then its RenderBox became
  detached after PointerCancel instead of returning to rest. An onDragCancel
  callback alone did not cover the real interrupted-drag path.
- **Cause:** Pinned Flutter monodrag.dart gives up a cancelled pointer; if the
  drag was already accepted, didStopTrackingLastPointer calls _checkEnd rather
  than _checkCancel. The resulting release velocity dismissed the toast. The
  test's later localToGlobal null check was a consequence of that real removal.
- **Resolution:** Observe raw PointerCancel with Listener before the gesture
  router, restore the retained drag position, and suppress the subsequent
  onEnd. Apply consistently to toast, edge opening, Action Center dismiss and
  lock cover. Keep the lock cover recognizer mounted during a fully travelled
  finger drag until credentials are actually requested on release.
- **Tried but failed:** Filtering offstage text alone did not fix the detached
  box. Logging the stack isolated the failure after cancellation; treating it
  merely as a stale test reference would leave the gesture bug in production.

### 2026-11-13 — do not infer a passing UI sweep from a yielded process

- **Problem:** Initial motion/IME sweep was described as passing before its final
  log was read. Chinese commit and landscape actually missed the first key.
- **Cause:** AnimatedSwitcher retains outgoing keys during its250ms fade; the
  synthetic semantic lookup targeted that ignored outgoing keyboard immediately
  after toggling Chinese. Reused fixture state also changed the initial mode.
- **Resolution:** Wait for the mode transition before locating keys, clear each
  fixture's isolated state, assert candidate presence, inspect captures and the
  final exit status. Correct the earlier development-log claim with a new entry.
- **Tried but failed:** A fresh PNG and an alive process alone prove neither
  completed exercise assertions nor Chinese delivery. Reading partial sweep
  output hid later failures; every final result must be inspected.

### 2026-11-13 — lock volume keys are filtered before Flutter HUD routing

- **Problem:** Phone's unlocked native volume event shows the correct WP top HUD,
  but an identical injected key produces no HUD on the actual secure lock page.
- **Cause:** Confirmed upstream wayland_frontend/input.rs handle-native-shortcut
  locked branch permits VT switch/lock/shutdown and consumes all other shortcut
  dispositions, explicitly including system controls. Moving the Flutter HUD
  above secure stage cannot create a native audio event that was suppressed.
- **Resolution:** Mark locked volume as unaccepted and distinguish it from the
  successfully verified unlocked top-only route. No authentication/native
  security policy weakened as part of this userspace deployment test.
- **Tried but failed:** Expecting the presenter layer change alone to allow
  hardware volume while locked; lock screenshot alone cannot prove a HUD test.

### 2026-11-13 — capture native QA output explicitly with PAM/TTY services

- **Problem:** System journal contained only test-unit lifecycle/PAM lines, hiding
  native lock and pointer assertions despite visible UI.
- **Cause:** The original PAM/TTY unit's actual child output did not reach the
  expected journal destination; exact descriptor reassignment was not audited.
- **Resolution:** Task-owned test launch explicitly redirects child stdout/stderr
  to a private QA file. The rerun confirms secure key delivery and toast cancel
  assertions; collect that log alongside actual screen captures.
- **Tried but failed:** Treating absence of journal errors as completed assertions.

### 2026-11-13 — initial-import whitespace checks include untouched upstream CRLF

- **Problem:** git diff --cached --check on a fresh desktop root commit emitted
  enormous trailing-whitespace diagnostics for untouched vendored source.
- **Cause:** Every imported line is new to that repository; upstream CRLF files
  and pre-existing Markdown hard line breaks are all inspected. This is not a
  focused check of task-authored changes.
- **Resolution:** Preserve upstream bytes/notices, use focused checks for new
  publication files, and verify exported source integrity/builds independently.
- **Tried but failed:** Running an unrestricted initial-import whitespace check
  interactively; it produced noisy output unrelated to the split's behavior.
  This check was not claimed as passing and no wholesale upstream rewrite was
  made merely to silence it.

### 2026-11-13 — Git backend may lag repository rename/public metadata

- **Problem:** First push immediately after keyboard rename/private→public
  returned403: repository disabled, owner should check account.
- **Cause:** API immediately reported disabledfalse/public/admin permissions;
  later anonymous ls-remote succeeded. Best hypothesis is transient Git backend
  propagation; no account suspension or policy restriction was established.
- **Resolution:** Preserve completed metadata state, inspect authenticated and
  anonymous API, prove anonymous Git readiness, and resume idempotently. Both
  renamed public repositories then pushed and verified successfully.
- **Tried but failed:** Treating successful metadata PATCH as immediate Git
  readiness. Do not recreate repositories or conclude account suspension from
  this message alone; preserve IDs/history and check the actual backend state.

### 2026-11-13 — full keyboard requires native key mappings and compatible Rust std

- **Problem:** Denial's SDK can press/release keys, but the pinned native named
  key table rejects modifiers, Home/End and function keys. Official ARM64 std
  then failed E0514 under distro Rust despite identical version/commit strings.
- **Cause:** Native whitelist incomplete; distro compiler metadata includes a
  vendor-specific compatibility identity. A subsequent build-std run lacked
  visible target std for io-lifetimes feature probes, selecting its fallback
  AsFd trait and failing Smithay compilation.
- **Resolution:** Extend only the named mapping table using the existing native
  ownership protocol. Build matching std from source and expose it to feature
  probes before third-party dependencies. ARM64 native build and QEMU --help pass.
  Preserve exact source/patch/compiler/binary and sysroot package provenance.
- **Tried but failed:** Showing unsupported UI buttons alone; using official std
  with the distro compiler; rebuilding std without making it visible to build
  probes. No system toolchain, phone driver, or native lock policy was changed.

### 2026-11-13 — drag consumption must not disable subsequent space moves

- **Problem:** Space swipe emitted only one Right even across64 logical pixels.
- **Cause:** Marking touch.done suppresses release's Space, but the original
  _touchMove early return also ignored every later move of that same touch.
- **Resolution:** Keep done for consumed release, allow moves while the shared
  cursor tracker is active, and claim the recognizer after its12px threshold.
- **Tried but failed:** Adding gesture claim alone; the retained touch had
  already been marked done. Actual multi-move UI fixture exposed it, while the
  isolated distance accumulator unit test alone could not cover integration.

### 2026-11-13 — visible editing controls must not steal the input target focus

- **Problem:** New toolbar TextButtons can request focus from a shell TextField,
  making subsequent OSK shortcuts target the toolbar rather than the editor.
- **Cause:** Ordinary Flutter buttons participate in focus traversal; a software
  keyboard must preserve the active editor while receiving touch/semantics input.
- **Resolution:** Exclude keyboard descendants from focus acquisition, retain
  pointer/semantics activation, and test an external framework focus target
  across Edit/Undo taps. Native modifier chords still use the existing broker.
- **Tried but failed:** Treating an onShortcut callback spy as proof of focus
  preservation. A dedicated focus fixture is required in addition to mapping.

### 2026-11-13 — keep keypad entry visible on narrow computer layouts

- **Problem:** The narrow-PC test did not activate a keypad button at the far
  right of a horizontally scrolled main board; its next tap sent F7 instead.
- **Cause:** Confirmed intended mode was still computer; exact offscreen tap
  failure was not isolated. Waiting for transitions/removing animated ensure-
  visible did not resolve that test setup. Do not label those attempts fixes.
- **Resolution:** Use a fixed toolbar keypad/main-board entry, remove the
  redundant far-edge button, and test this real user-visible route. Keypad
  activation and KP_7 then pass. Main keyboard remains freely scrollable.
- **Tried but failed:** Timing-only changes and assuming a semantic lookup means
  a scrolled control is tappable. Render proxy hit-path identity alone also
  cannot establish clipping, so that diagnostic was not treated as proof.
