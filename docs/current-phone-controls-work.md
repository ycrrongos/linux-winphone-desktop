# Active work — 2026-11-12 phone controls/motion/input

User requests all of: volume/brightness bars; nonlinear WP motion throughout;
continuous Win8 flying tile; normalized icon proportions at multiple scales;
real administrator authorization; usable screen keyboard with WP UI/motion;
phone now attached, deploy/test current result. Do not stop at this interim file.

Current evidence:
- Verified USB SSH through /tmp/win10x-phone-ssh.py (credentials private, never
  print). aarch64 boot915c1523-031c-4173-bc80-62f626d2af65, session4 tty2,
  deniald2525, Wayland wayland-1. Installed c79511384aa8; preserve user Chromium.
- Agent missing because packaging session explicitly --no-polkit-agent and
  staged binary's defaults refer to /usr/lib/denial rather than isolated prefix.
- Started task-owned transient user unit win10x-auth-audit.service (PID9560)
  with DENIAL_POLKIT_AGENT=1, WAYLAND_DISPLAY=wayland-1, explicit runner/bundle/
  engine paths to current runtime polkit dir. Journal confirms registration for
  current login session. Safe pkcheck action org.freedesktop.policykit.exec with
  subject --process2525 and --allow-user-interaction launched real native Flutter
  dev.denial.Polkit PID9649. Timeout probe is /tmp/win10x-auth-probe.log.
  No privileged operation executed. Need capture real dialog, cancellation and
  authorized credential flow, then persistent session-integrated agent startup.
- Keyboard auto-open is gated by ReferenceShellProfile.mobile, but our shell
  deliberately uses desktop containment even in phone mode. SystemKeyboardLayer
  exists in desktop root; no Maliit needed for built-in OSK. Patch added ONLY
  _handleTextInputState guard in tools/win10x-denial prepare, allowing phone and
  tablet mode auto-open; unrelated mobile minimize policy stays off. Not built.
- Phone footer hid brightness explicitly and volume only lived in audio detail.
  action_center.dart footer now includes both existing real-service sliders.
  Not built/tested yet.

Next: restyle OSK and replace spring settle with WP nonlinear motion in pinned
prepare patches; continuous Win8 geometry; icon alpha-bound normalization; audit
missing motion. Analyze/test/bundle/current UI checks at several scales; build
ARM64 and safe in-process bundle activation, preserve apps. Native acceptance.

Use known pinned upstream source /tmp/win10x-denial-upstream (Rust/text broker/
polkit agent), patched kit target/denial/kit/runtime/plugins/denial_desktop.
OSK: widgets/osk/shell_osk_panel.dart; motion widgets/edge_panel_layer.dart
(springTo at214, report spring duration at180). Do not edit generated kit only;
persist patches in tools/win10x-denial or repo source copied by prepare.

Read phone AGENTS + active experiment before mutation. Active card appended
win10x_phone_auth_input_20261112. No kernel/boot/slot/radio/driver edits, SDDM125%
already accepted and must remain. No restarting display manager. New requests
supersede old phone-unavailable status. No subagents authorized.

Last completed candidate ecc33d3a5564, 46tests/15previews passed, ARM64 archive
staged target/denial-arm64/win10x-phone-ecc33d3a5564.tar.gz, not installed.
Current host review preview701623. Current task edits make its source stale.
Research: Microsoft WP motion MSDN April2011 mobile navigation part2;
Windows Developer July22 2014 universal XAML controls (turnstile/slide/continuum);
Windows Experience June19 2014 sound personalization (ringer/media sliders).
Lumia950 PDF local /tmp/win10x-research-20261111/lumia950-manual.pdf.
Need append lessons/development log for completed fixes and current blockers.

Progress19:40: authorization screenshot captured at
`target/denial-arm64/qa/controls-20261112/auth.png`; real gray translucent upstream
native authentication overlay visible over Firefox, no password/authentication
success tested. Safe probes time out/cancel. Agent transient unit still running.
Tools prepare OSK patches analyze PASS: automatic gate touch modes only;
edge spring replaced animateTo333/250ms Fluent standard/accelerate; native inset
motion report matches duration; WP dark keys/controls and square caps/popups.
Win8 widget changed to one global smoothstep center/rotation + squared growth
path with lift arc; no build/visual test yet. All this is WIP, not deployed.

Progress19:44:
- `packaging/win10x-denial-polkit-agent` new relative-prefix wrapper; generic
  session now exports DENIAL_POLKIT_BINARY and removes explicit --no-polkit-agent.
  Both packaging helpers copy agent wrapper. Upstream startup fallback uses this
  env binary after Wayland readiness if no systemd session activation succeeded.
  Current private-bus isolated installation lacks service integration; verify
  actual new startup path and avoid duplicate existing transient agent.
- Icon algorithm rewritten in MarkUiAsset: source-alpha bbox, uniform fit to86%
  optical slot, centering by painted bounds and physical-pixel target rounding.
  Original PNG bytes preserved. New generated markui_metrics.dart contains all
  nav/UI/app original bbox geometry; ShellGlyph nav now uses same algorithm and
  drops fixed3.6/1.35 multipliers. tools/import-markui-icons regenerates metrics
  using PIL (available host). Test scales/DPR still pending.
- Host analyze/test/bundle running exec session25048, logs /tmp/win10x-controls-
  {analyze,test,bundle}.log. No ARM64 build/deploy yet for this task.
- Task-owned live agent win10x-auth-audit.service9560 remains active. Safe
  pkcheck probes time out without privileged operation. Auth screenshot visible
  over Firefox confirms dialog rendering, not credential success.
- Need add preview icon-scale and keyboard cases, inspect motion in-animation/
  settled, phone apply through denialctl ui activate to avoid closing user apps.
  Plan physical input tests using existing tools only with bounded task-owned
  injectors; never type credentials into unverified focus.

Progress19:56 (supersedes earlier stage statements):
- cb4f17bab749 built: analyzePASS,47testsPASS, x64 bundle and real ARM64 SSH
  AOT/package succeeded. Icons fixture inspected good at20 size/DPR combinations.
  Start/ActionCenter preview clean; standalone keyboard fixture FAIL offstage.
  Policy.none and real TextField tap did not make standalone native broker show
  keys. Do not repeat blind fixture adjustments; native acceptance is required.
- Installed immutable /opt/win10x-denial/cb4f17bab749-arm64-controls-20261112.
  Archive SHA694198dff8d69176eb74778eb90299de896cedc2fbbef4162d8a744c82103514.
  Root backup /var/backups/win10x-controls-20261112, rollback.sh retained.
  Updated /usr/local/bin/win10x-denial-phone-session from repo. Native2525
  refreshed with SIGUSR1; /proc/maps proves NEW AOT; same boot/session/apps.
  Generation2, locked=true due native secure boundary. Screenshot denied while
  locked.120s rollback timer was stopped after healthy runtime/AOT confirmation.
- Asked user asynchronously to normally unlock, tap Start search and input a.
  Required physical input, not permission. No reply yet at writing. Do not bypass
  native lock or restart SDDM. Cannot use /dev/uinput (absent) or wtype virtual
  keyboard (compositor only accepts IM-owner); do not load kernel modules.
- Old auth-audit9560 stopped. New task-owned transient unit win10x-auth-session
  launches actual installed prefix wrapper; check journal registration. Persisted
  wrapper supports native startup fallback via DENIAL_POLKIT_BINARY next login;
  entire next-login lifecycle still unverified (don't close user apps to test).
- Latest HOST source now differs from installed cb4f: volume HUD interruption
  continuity fixed (controller owns animateTo/animateBack curve, no status-switched
  curve jump) and new win8-motion:<p> opt-in actual-widget frame fixture. Host
  analyze/test/bundle running session17496; next ARM build only after final edits.
- New /tmp/win10x-controls-qa.py runs icons,keyboard,action-center,start. Add Win8
  phase cases .23/.24/.25/.5/.67/.68/.69/1 at1280x800 to inspect no seams. Sample
  duration/curves now explicit; still need rigorous visual/native motion verdict.
- Full implementation/status appended development-log and lessons; task remains
  active, do not report all requested functionality fixed without native evidence.

Progress20:01:
- Win8 eight phase frames (.23,.24,.25,.5,.67,.68,.69,1) all captured clean from
  source21db4f8b5075; inspect images before claiming motion accepted.
- Important new icon regression identified in actual Start: requested theme20
  ignored parent tight tile size; fit computed before layout made MarkUI apps
  tiny. Fixed with LayoutBuilder resolved-slot shortest side, preserving .86
  optical/alpha fit. Added parent-forced-size file glyph in gallery; gallery
  test height now1280 to fit4 icons per size. Host build running59931; must use
  final updated source for ARM + phone update before visual completion.
- Latest host21db4 source was ARM built/staged but NOT installed; phone remains
  cb4f17bab749, locked, generation2, samePID2525. User unlock/input question pending.
- No need more changes to unrelated UI. Finish new icon QA, stage final build,
  update immutable phone generation through guarded SIGUSR1 while already locked,
  preserve apps. Agent win10x-auth-session registered through actual installed
  launcher. If user unlocks before final refresh, explain the necessary refresh
  locks again; do not bypass authentication.

Recovery20:05 (USER steering supersedes prior SIGUSR1 plan): user could not unlock
refresh-induced lock, pressed Power and rebooted. Do NOT use SIGUSR1 again on
active phone. New bootc3b56429-da2a-414f-9220-3cbe383da737, nativePID2427/session3,
currentcb4f17bab749 generation1/unlocked. Native startup now automatically runs
relative-prefix agent2545; power preference nowdpms seeded by installed wrapper.
Latest hostcfec832b194a contains resolved-slot icon fix and volume interruption
fix; not installed. Need native keyboard test, current user accidental-looking
text po000000陪婆婆 might be keyboard test; asked asynchronously, reply pending.
Continue original feature task. Keep SDDM/apps and existing unlocked session.

New user scope (latest): lock picture layout from supplied images, upward
finger-follow credentials with nonlinear settle; Lumia top volume/media strip;
research/rework W10M popup/context/command menus, requested viewport→dialog zoom.
User explicitly says auth + OSK there now work, but normal fields everywhere
fail auto-open; navbar search works. Accidental text was not a keyboard test.

Current local implementation WIP: secure-stage lockBuilder now actually replaces
upstream presentation (normal aboveWindows plugin was never wired); PhoneLockPane
pure gesture cover and Win10xLockScreen native auth/PAM wrapper. Deferred begin
waits for sync/available; secrets clear on return/submit/dispose. No unlock bypass.
PhoneVolumePanel + HUD top controls/media, event notifier hooks real native HUD
revision (including volume at limit). Context menus separate anchored elevator
reveal from ApplicationBar; uniform fullscreen→dialog customflight; no accentstripe.
WpMotion now normalized exponent6,350/250ms source WP Toolkit, not claimed allW10M.

New keyboard policy opens a genuinely active editor in phone mode, retains manual
hide until a new activation serial; desktop requested-only policy retained.
Terminal legacy registration is now wired in DesktopShellServices launch path.
OSK reserves48px bottom nav and hit test defers over empty padding. Start/Back
command dismiss OSK. UI fixtures added lock-cover/reveal/mid/return/short,
volume-panel/context-menu; need rebuild latest source and QA.51tests lastpassed.

Fluent auth is independently vendored pinned Denial polkit_app in denial/polkit_app,
GPL attribution and original agent protocol intact. Standard secure TextField
replaces custom rendererIgnoresPointer control, Fluent square bordered buttons /
opaque card, Chinese labels. tools/win10x-denial-polkit builds preview/production
x64/ARM via pinned engines; stage_auth helper packages checked production bundle.
ARM42a6755e88f4 built, tarSHA7946893294b5f81ba03843b81bd0157568226743faa2153b7fb0aee8e78f28f4.
Auth source digest excludes unrelated shell UI (depends on fluent.dart only).
Three new style previews running84779. Actual auth deploy pending transfer/helper.
Full shell host build4093716fb25d is stale (lockreveal/focus/menu/event edits after).
Do not build auth and shell simultaneously (shared prepare engine/cache).

Progress21:45 — latest lock/volume/popups and input acceptance:
- User's new pictures explicitly reopen lock. AGENTS and interaction checklist
  now reflect that; OOBE stays deferred, SDDM125% unchanged. Actual secure-stage
  builder uses custom lock, retains native PAM/isolation/frame acknowledgements.
- WP-style picture page has lower-left thin time/day/date + notice count, native
  current wallpaper and small connectivity/battery. Finger-follow is linear;
  release/cancel350/250ms exponent6 WP-derived easing. Credentials remain unfocusable
  until revealed. Reveal completion requests focus again after ExcludeFocus clears.
- Top volume panel includes actual media slider/mute, native limit-key event
  trigger, optional MPRIS track/control rows; top slide uses nonlinear motion.
- Distinct anchored context flyouts versus edge ApplicationBar. MessageBox uses
  square title/body/buttons and user-requested fullscreen-to-dialog uniform flight;
  reverse exit. Long bodies scroll independently of persistent actions.
- Original15 previews caught Home failing to dismiss OSK and long-dialog hidden
  actions. Both fixed and focused3-case rerun passed. 51 pure tests passed.
- Generated kit repeated prepare had accumulated duplicate listeners/bindings:
  restore pristine checksum-pinned runtime every prepare before applying patches.
- Native auto-input follows genuine active editor, independent of explicit
  show-panel hints in phone/tablet; manual dismissal respected by activation serial.
  Qt Wayland defaults to v3. Browser/native acceptance remains to be done.
- Auth production42a6755e88f4 installed separately with root rollback backup;
  idle old agent replaced by task-owned win10x-fluent-auth-live.service5264.
  Registered successfully. Current user session had independently idle-locked;
  pkcheck rejected while locked. New style real-native acceptance still pending.
- b05bc1f795d0 x64/ARM64 built, but reveal-settled focus correction supersedes it.
  Latest matching build running /tmp/win10x-lock-focus-{analyze,test,bundle,arm64}.log.
  Do not hot refresh current phone. Next is bounded VT8 with isolated config/runtime
  and independent VT2 restore, then stage default for normal next login only.

Progress21:55 — native secure input result:
- sourceabc5519c9fa9 bounded VT8 passed: physical-view swipe revealed true,
  synchronized native PAM capability/prompt present, OSK auto-opened and delivered
  then cleared a test `a` in the secure password editor. No authentication submitted.
  Screenshot/log under target/denial-arm64/qa/lock-motion-20261112/
  native-lock-input-current.{png,log}. Returned tty2, same boot/PID2427.
- First native lock fixture failed because gesture coordinates were computed
  before secure-stage layout; hit path contained only RenderView. Awaiting frame
  and using real viewport fixed the fixture; not evidence of a PAM bug.
- Native GTK fixture launch rejected locally: desktopFileId lacked `.desktop`,
  as confirmed in SDK session_client.dart validation. Fix fixture ID/check return;
  this is not evidence native keyboard failed for a launched editor (none launched).
- Volume HUD now uses actual normalized service but original0–30 media steps,
  shared WP350/250ms exponent6 curves. Current release build is superseded by the
  forthcoming GTK fixture ID correction. No candidate selected as default yet.

## 2026-11-13 — current local-only motion/Chinese candidate

Latest user says the phone cannot be connected. Do not use earlier device access
instructions above to deploy or refresh it. Current source e805bf9200f7: native
phone audio HUD routed once to WP top controls above secure stage, offline
Chinese keyboard candidates, editor-scoped sensitive-input handling, and actual
finger-follow/cancellation fixes for toast/edge/Action Center/lock. See
`docs/chinese-input-and-motion-audit.md` for design/provenance and boundaries.

Final x64 analyze/56 tests/bundle pass. Corrected clean-state ten-case UI sweep
passes; inspected actual Chinese commit in portrait/landscape, candidate grid
and native-provider top-volume screenshots. Learned count1 for Chinese and0 for
sensitive input, both mode600. Preview now runs e805bf9200f7. ARM64 local-qemu
build/package in progress; real native application Chinese delivery, physical
volume keys and locked-session presentation still need phone acceptance. No
phone SSH, install, session refresh or SDDM change this turn. Earlier claim of
ten passing source2a3 tests was corrected in development-log/lessons-learned;
use the final e805bf9200f7 results instead.

Final update: e805bf9200f7 ARM64 local-qemu bundle/package both exit0; staged at
packaging/out/denial-arm64,93 bitmaps/41 icon glyphs verified. No device update.

## 2026-11-13 — user reconnected phone and requested deployment

Supersedes the earlier local-only restriction. Installed/selected e805bf9200f7
with root rollback backup; actual GTK receipt你好 and native unlocked top-only
volume screenshots verified. Secure swipe/PAM/automatic keyboard/test-a cleared
and real toast-follow/cancel assertions pass. Restored original tty2/PID1907,
bootffba7009 unchanged. No active-session refresh; new default on next login.
Locked volume keys remain consumed by upstream native secure shortcut policy;
Chromium Chinese delivery not tested. See phone-session/development-log for
exact evidence and boundaries. No SDDM/kernel/boot/radio changes.
