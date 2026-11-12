# Win10X Denial/Flutter — remaining work breakdown

Living checklist for **all unfinished product scope**, including items that were
previously deferred. Written 2026-11-11 after user request to finish everything
except **restoring the removed acrylic stack** (new acrylic must be a clean
redesign later; see §8).

**Acceptance rules (user, 2026-11-11):**

- Every row must ship **complete and usable**, not stubs or placeholders.
- Animations must be **restored / matched** to the cited sources; **no**
  simplified stand-ins, missing segments, or “close enough” curves.
- Do **not invent** timings, layouts, or OS behaviors — look up MetroLauncher,
  Disco, legacy Rust `shell/`, Denial settings, Microsoft WP docs, and local
  research under `research/` before coding.
- Visual acceptance still belongs to the user after self-test captures.

Primary references:

| Topic | Source |
|---|---|
| Phone Start / Live Tile / drawer ops | [MetroLauncherAndroid README](https://github.com/lorddie/MetroLauncherAndroid) + `docs/metrolauncher-android-parity.md` |
| Turnstile / live-tile motion vocabulary | Disco Launcher + `research/windows-phone-launchers.md` |
| Legacy Start / launch / app-switch math | `shell/src/start_tiles.rs`, `shell/src/main.rs` |
| Flutter animation gaps | `docs/flutter-vs-rust-animation-gaps.md` |
| Interaction / session bar | `docs/interaction-checklist.md` |
| Dual-screen (now in scope) | `research/windows10x/dual-screen-hardware/` |
| WP ApplicationBar / easing | [MSDN App bar](https://learn.microsoft.com/en-us/previous-versions/windows/apps/ff431813(v=vs.105)), [WP motion](https://learn.microsoft.com/en-us/previous-versions/windows/apps/hh202871(v=vs.105)) |
| Denial Settings pages | pinned kit `SettingsPageId` (you…about) |

---

## 0. Status legend

| Tag | Meaning |
|---|---|
| `todo` | Not started or incomplete |
| `partial` | Present but missing behavior / motion / wiring |
| `verify` | Code path exists; needs visual / on-device proof |
| `blocked` | Waiting on host API, sudo, or hardware |

---

## 1. MetroLauncher / Windows Phone Start parity

### 1.1 Tile grid & edit

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| M1 | Gap fill top-to-left | After unpin / resize / drop, pack holes like WP / MetroLauncher `GridPacker` | `verify` — pack on unpin/resize/drop (2026-11-11) | MetroLauncher README Auto-placement |
| M2 | Edit wobble | Continuous edit-mode tilt while editing; exit on background tap | `verify` | MetroLauncher long-press edit |
| M3 | Resize Small/Med/Wide/Large | Full size set + menu + handle | `verify` | MetroLauncher tile sizes |
| M4 | Show more tiles | 4↔6 (phone) naming + layout | `verify` | WP “Show more tiles” |
| M5 | Per-tile accent / transparent | Menu + persistence | `verify` | MetroLauncher tile customization |
| M6 | Auto accent from icon | Palette extract on pin (optional override) | `verify` — nearest WP accent from Denial `accent` on pin (2026-11-11) | MetroLauncher `ColorUtils` / palette |

### 1.2 All Apps drawer

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| D1 | Open gestures | Swipe ← / ↑ / › / double-tap | `verify` | MetroLauncher gestures |
| D2 | Finger-follow page | ViewPager-style live drag + 340 ms easeOutExpo settle | `verify` | `docs/flutter-vs-rust-animation-gaps.md` O1 |
| D3 | Alphabet headers + side rail | Jump + continuous rail | `verify` | MetroLauncher drawer |
| D4 | Search | Filter All Apps | `verify` | MetroLauncher |
| D5 | Drawer font size | Small / Normal / Large setting | `verify` — prefs + All Apps (2026-11-11) | MetroLauncher settings |
| D6 | Tile font size | Small / Normal / Large setting | `verify` — prefs + tile captions (2026-11-11) | MetroLauncher settings |

### 1.3 Pin / info / uninstall

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| P1 | Pin / unpin Start | Menu + toast | `verify` | MetroLauncher |
| P2 | App Info | Open desktop-entry / package info surface | `verify` — WP MessageBox + open `.desktop` folder (2026-11-11) | MetroLauncher |
| P3 | Uninstall / remove | Confirm dialog → software center / flatpak path; graceful toast when unavailable | `verify` — WP MessageBox + flatpak/software-center best-effort (2026-11-11) | MetroLauncher uninstall |
| P4 | Pin to taskbar | Desktop/tablet | `verify` | checklist |

### 1.4 Live Tiles

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| L1 | Notification text | Title + body on Wide/Large | `verify` — live face + Disco flip (2026-11-11) | MetroLauncher text live tile |
| L2 | Badge count | Top-right numeric | `verify` | MetroLauncher badge |
| L3 | Messaging aggregate | Unread count + last preview when notification style exposes it | `verify` — group member count + newest body (2026-11-11) | MetroLauncher MessagingStyle notes (map to desktop notifications) |
| L4 | Media mini-player tile | Art / title / artist / progress / play-pause / prev-next on media apps | `verify` — MPRIS title/artist/progress/controls + file/http art (2026-11-11) | MetroLauncher media tiles + Denial `media` service |
| L5 | Live content flip | Icon ↔ content turnstile / vertical page (Disco vocabulary) | `verify` — Disco `hide-flip-direction-0` 1s (2026-11-11) | Disco `tileList.scss` live-tile section |
| L6 | Notification access UX | Prompt / settings deep-link equivalent on Linux | `verify` — first-open phone toast + Permissions deep-link; Action Center「管理通知」(2026-11-11) | Denial notifications + Settings permissions |

### 1.5 Wallpaper / start+theme

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| W1 | Start uses desktop wallpaper | + theme veil (not black scrim) | `verify` | WP 8.1 Start Background |
| W2 | Light / dark theme | Prefs + chrome | `verify` | WP start+theme |
| W3 | Accent palette | Full WP accents | `verify` | current `phoneAccentArgb` |
| W4 | Custom Start image | Optional override beyond desktop wallpaper | `verify` — zenity/kdialog picker + prefs (2026-11-11) | MetroLauncher wallpaper picker |
| W5 | Reset Start | Clear tiles to default seed | `verify` — mode settings danger action | MetroLauncher Reset Start |

---

## 2. WinPhone-style popups, menus, dialogs (full motion)

Phone menus go through `WpPopups` (ApplicationBar / MessageBox / toast) with
MSDN log-enter / exp-exit easing. Durations use Denial Fluent slower/slow
tokens (333 / 250 / 200 ms) — MSDN does not publish a fixed ApplicationBar
duration, only the easing vocabulary.

### 2.1 Application Bar

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| AB1 | Mini bar | 1–4 icon buttons + ellipsis along bottom | `verify` — tile + All Apps icon strips (2026-11-11) | [MSDN App bar](https://learn.microsoft.com/en-us/previous-versions/windows/apps/ff431813(v=vs.105)) |
| AB2 | Expanded menu list | Text items slide up from under icons when ellipsis opens | `verify` — expand controller + heightFactor (2026-11-11) | same |
| AB3 | Open / close animation | Built-in show/hide; enter logarithmic (ease-out), exit exponential (ease-in) per WP motion guidance | `verify` — `WpMotion` enter/exit + reverse close (2026-11-11) | [WP animations](https://learn.microsoft.com/en-us/previous-versions/windows/apps/hh202871(v=vs.105)) |
| AB4 | Landscape docking | Bar moves to short edge in landscape | `verify` — right-edge dock + horizontal slide (2026-11-11) | MSDN App bar orientation note |
| AB5 | Scrim + dismiss | Tap outside / Back closes with exit motion | `verify` — scrim tap closes with exit motion | WP UX |

### 2.2 Context menus (Start / All Apps)

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| CM1 | Phone long-press → Application Bar menu | Not a desktop flyout | `verify` — Start/All Apps → `WpPopups.applicationBarMenu` (2026-11-11) | MetroLauncher + WP |
| CM2 | Desktop / tablet near-pointer menu | Win10X square chrome + reveal | `verify` | Fluent menus |
| CM3 | Nested submenus | Size / color choosers keep enter/exit motion | `verify` — nested `_showMenu` reuses AB enter/exit (2026-11-11) | current nested `_showMenu` |
| CM4 | Item press / reveal feedback | Instant press darken; no Material ink | `verify` | Win10UI / Fluent |

### 2.3 MessageBox / ContentDialog

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| MB1 | Confirm dialog | Title + body + primary/secondary | `verify` — `WpPopups.messageBox` (2026-11-11) | WP MessageBox pattern |
| MB2 | Enter / exit motion | Scale+fade with log/exp easing from WP motion guidance | `verify` — WpMotion dialog enter/exit (2026-11-11) | WP motion docs |
| MB3 | Destructive confirm | Uninstall / reset / power | `verify` — uninstall + Action Center power MessageBox (2026-11-11) | checklist destructive ops |

### 2.4 Toast / banner

| ID | Feature | Detail | Status | Verify against |
|---|---|---|---|---|
| T1 | WP toast strip | Accent bar + bottom slide | `verify` — `WpToast` (2026-11-11) | `WpToast` |
| T2 | Enter/exit curves | Match WP toast timing (research before retune) | `verify` — WpMotion toast 250/200 + log/exp (2026-11-11) | WP toast + Fluent tokens |
| T3 | Action toast | Optional buttons | `verify` — `WpToast.show(..., actions:)` (2026-11-11) | WP toast variants |

---

## 3. Animation fidelity (Rust / Disco / W10M)

| ID | Feature | Status | Notes |
|---|---|---|---|
| A2 | Factor Start entrance timing | `verify` | constants present |
| A3 | W10M launch handoff frame-compare | `verify` | settled 100%, row lift, 80 ms selected, 200 ms veil |
| A4 | Win8 flying tile segment curves | `verify` | lift→edge→expand |
| A10 | Wallpaper + veil sync with last tile | `verify` | W10M launch holds veil |
| A-extra | Disco app-list exit | `verify` — `StartReveal.discoAppListFrame` + catalog transform (2026-11-11) | Rust `disco_app_list_frame` when launching from list |
| A-extra | App-switch proxy quality | `verify` | real previews, 200/500 ms, taskbar above |

No simplified curves. If Rust feels stiff, improve and document in
`docs/flutter-vs-rust-animation-gaps.md` as intentional.

---

## 4. Desktop / system Settings (complete)

Denial already ships a Settings app with `SettingsPageId` pages. Win10X must
**wire, theme, and complete** desktop personalization + shell pages so they are
fully usable from Start / Action Center — not MVP stubs.

### 4.1 Shell personalization (Win10X-owned)

| ID | Feature | Detail | Status |
|---|---|---|---|
| S1 | Wallpaper browse / recent / apply | Denial wallpaper selector + persistence | `verify` — Start image picker + Denial Appearance | 
| S2 | Colors / accent | Desktop accent + phone accent bridge | `verify` — phone accents + auto from host accent |
| S3 | Light / dark | Shell + Start theme | `verify` |
| S4 | Start layout style | Win10 / Phone / Win8 | `verify` — mode settings |
| S5 | Start tile animation | All styles | `verify` — mode settings |
| S6 | Launch animation | All styles | `verify` — mode settings |
| S7 | App-switch animation enable | Pref + real handoff | `verify` — mode settings toggle |
| S8 | Shell mode | Tablet / Desktop / Phone | `verify` |
| S9 | Taskbar / chrome insets | Mode-driven | `verify` |
| S10 | Dedicated Win10X Settings page(s) | Appear inside Denial Settings or shell-hosted full page with WP/Win10X chrome | `verify` — shell-hosted `ShellModeSettings` in Action Center / Start (2026-11-11); Denial Settings pages remain host-owned |

### 4.2 Denial Settings pages (must work end-to-end)

| ID | Page | Status |
|---|---|---|
| DS1 | You | `verify` — `SettingsLaunch.allPages` → `--page=you` (2026-11-11) |
| DS2 | Appearance | `verify` — deep-link + mode-settings wallpaper/colors (2026-11-11) |
| DS3 | Language | `verify` — `--page=language` |
| DS4 | Keyboard | `verify` — `--page=keyboard` |
| DS5 | Touchpad | `verify` — `--page=touchpad` |
| DS6 | Shortcuts | `verify` — `--page=shortcuts` |
| DS7 | Environment | `verify` — `--page=environment` |
| DS8 | Animations | `verify` — deep-link + shell start/launch/switch prefs (2026-11-11) |
| DS9 | Layout | `verify` — `--page=layout` |
| DS10 | Overlays | `verify` — `--page=overlays` |
| DS11 | Lock screen | `verify` — `--page=lockScreen` |
| DS12 | Fingerprint | `verify` — `--page=fingerprint` (hardware gated) |
| DS13 | Permissions | `verify` — Action Center + L6 toast |
| DS14 | Audio | `verify` — Action Center hold |
| DS15 | Displays | `verify` — Action Center quick tile |
| DS16 | Network | `verify` — Action Center quick tile |
| DS17 | Bluetooth | `verify` — Action Center quick tile |
| DS18 | Power | `verify` — Action Center hold |
| DS19 | Developer | `verify` — `--page=developer` |
| DS20 | About | `verify` — `--page=about` |

Action Center quick tiles must deep-link into the real page, not placeholders.

---

## 5. Shell chrome & Action Center

| ID | Feature | Status |
|---|---|---|
| AC1 | Finger-follow open/close | `verify` (A9) |
| AC2 | Notification groups / clear / actions | `verify` — `NotificationFeed` groups / clear / action buttons (2026-11-11) |
| AC3 | Media card | `verify` — title/artist + prev / play-pause / next (2026-11-11) |
| AC4 | Quick settings real backends | `verify` — NM Wi-Fi + BT power + DND + Settings deep-links (2026-11-11) |
| AC5 | Power confirm with MB1 | `verify` — WP MessageBox before destructive power (2026-11-11) |
| AC6 | Soft keyboard inset (content not covered) | `verify` — Start uses `viewInsets.bottom` |

---

## 6. Session: OOBE / lock / login (previously deferred — now required)

| ID | Feature | Detail | Status |
|---|---|---|---|
| SS1 | Lock screen | Touch-usable; clock / notifications / unlock affordance | `verify` — `Win10xLockScreen` on `environment.locked` + `authenticationProvider` (2026-11-11); seat test pending |
| SS2 | Login | greetd or Denial-auth path; large touch targets | `verify` — Denial PAM auth (PIN/password/fingerprint) on lock with large targets (2026-11-11); pre-seat greetd greeter remains host-owned |
| SS3 | OOBE | Large type, one step per page, essential quick settings | `verify` — `Win10xOobe` + `oobeCompleted` pref (2026-11-11) |
| SS4 | Settings lock-screen page | Wire `SettingsPageId.lockScreen` | `verify` — `SettingsLaunch.lockScreen` from mode settings / Action Center (2026-11-11) |

Research Denial lock/session APIs and greetd before inventing UI state machines.
Do not claim complete until nested/real seat test.

---

## 7. Dual-screen / hinge / Wonder Bar (previously deferred — now required)

Source: `research/windows10x/dual-screen-hardware/`.

| ID | Feature | Detail | Status |
|---|---|---|---|
| DU1 | Dual-output geometry | Two logical outputs, hinge dead zone | `verify` — `DualScreenGeometry` + tests (2026-11-11); host multi-output wiring pending hardware |
| DU2 | Extended canvas | App span across screens | `verify` — mode → `DenialBridge.configureWindow` over bounding box (2026-11-11) |
| DU3 | List–Detail / Two Page / Companion / Dual View | Shell layout modes | `verify` — mode enum + hinge-safe panes + settings picker (2026-11-11) |
| DU4 | Posture response | Fold / book / tent rearrange | `verify` — `inferPosture` from output geometry when sensors absent (2026-11-11); hardware hinge sensors still optional upgrade |
| DU5 | Cross-app drag | Drag/drop between screens | `verify` — pane hit-test + `moveWindowToOutput` window handoff (2026-11-11); generic wl_data client DnD remains compositor-owned |
| DU6 | Wonder Bar | Keyboard-cover remainder: trackpad / media / PiP | `verify` — `Win10xWonderBar` + region helper (2026-11-11); populate when keyboard cover reported |

Implement only with researched layout rules; keep research notes updated when
behavior is chosen.

---

## 8. Acrylic (new design only)

| ID | Feature | Status |
|---|---|---|
| AY1 | **Do not restore** old wallpaper sampler / CPU blur cache / HostBackdrop / GLES blur shader | **forbidden** |
| AY2 | New acrylic design from clean baseline | `verify` — `Win10xAcrylic` via Denial `ShellBackdropBlur` + accent tint (2026-11-11); **not** old sampler/GLES stack |

Until AY2 lands, Start continues wallpaper + veil alpha (intentional).

---

## 9. Packaging / targets

| ID | Feature | Status |
|---|---|---|
| PK1 | x86_64 preview / bundle self-test | `verify` each change |
| PK2 | aarch64 Cerro deploy | `verify` — current source AOT via `WIN10X_ARM64_HOST=qemu` → `target/denial-arm64/bundles/94c51a27d39d-bh1jz0hn` + `packaging/out/denial-arm64` (2026-11-11); on-device Cerro push still waits for USB/Wi-Fi |
| PK3 | L2 KVM / L3 touch hardware | per `docs/testing.md` |

---

## 10. Implementation order (for the Goal)

1. **WP popup kit** (AB*, CM*, MB*, T*) — unblocks uninstall / power / settings UX.
2. **Desktop Settings completeness** (S*, DS*) — wallpaper/colors/animations wired.
3. **Metro remaining** (P3, L3–L6, M1, M6, D5–D6, W4–W5).
4. **Animation verify / fix** (A2–A4, A10, Disco list exit).
5. **Session** (SS*).
6. **Dual-screen** (DU*).
7. **New acrylic design** (AY2) after the above are usable.
8. Continuous analyze / test / preview / arm64 deploy.

Update this file when a row closes (date + verifying path). Mirror user-facing
behavior changes into `docs/development-log.md` and blockers into
`docs/lessons-learned.md`.

## 2026-11-12 functional review — acceptance overrides

These corrections supersede earlier `verify` descriptions; historical work is
retained rather than hidden. A class/bridge call/deep-link alone is not an
end-to-end feature:

- M4: classic/compact phone projection is now 4/6 columns, matching the selected
  MetroLauncher reference; existing saved coordinates survive projection.
- M6: **partial**. The shipped option follows system accent. Actual icon palette
  extraction is not implemented; its earlier “auto from icon” label was removed.
- P3: **partial**. Confirmation and software-center handoff exist. No package
  removal was performed in this review. A successful detached process spawn
  alone is not proof of removal. The unsafe desktop-entry-launch fallback and
  silent terminal Flatpak-uninstall path were removed.
- AB5/MB/T: lifecycle/input fixes have actual preview exercises. Feedback no
  longer blocks the full scene or keyboard; action toasts also expire. Phone
  navigation remains above menus; cancelled/unmounted MessageBoxes resolve.
- DS1–DS20: **integration, not visual parity**. These open upstream Denial pages;
  the separate Settings executable has not been comprehensively restyled into
  Windows 10 Settings. Hardware backends still need target-specific checks.
- SS2/SS3: **partial**. Lock/OOBE widgets exist, but `denial/main.dart` currently
  registers only taskbar/status surfaces. Those widgets are not mounted by the
  current composition. Neither a pre-seat greeter nor real PAM/keyboard/fingerprint
  acceptance follows from class existence. Do not claim a working new login.
- DU2/DU4/DU5: **partial**. Bridge helpers/geometry exist. Two monitor rectangles
  cannot prove hinge angle, tent posture or an occluded panel. A window movement
  helper does not implement arbitrary client file/text `wl_data` drag-and-drop.
  Retain dedicated hardware/native integration work and regression acceptance.
- PK2: current ARM64 build/package can be prepared with the pinned engine and
  qemu AOT. Latest-source device deployment/testing remains pending while Cerro
  is disconnected; earlier 2026-11-11 native images are not current acceptance.
- AY2: new compositor blur code is distinct from the forbidden removed stack;
  this review does not restore that old implementation. New material still
  needs explicit visual judgment and device performance acceptance.

## 2026-11-13 — screen transitions native acceptance follow-up

- [ ] Physical-power-button pre-off handshake for the desktop containment profile:
  shipped Denial only exposes ambient-display hold on configured mobile/AOD
  hardware. Do not weaken native lock/input isolation to animate an unlocked
  desktop. Generic pre-off events/acknowledgement need a native extension.
- [ ] Real phone acceptance of configured idle timeout/inhibitors, wake hooks and
  privileged exact-command authorization after deployment. This task is explicitly
  local-only because the user cannot attach the phone.
- [ ] Observe cancellation/wake during sleep fade and physical-key/timeout races
  on hardware; verify one off/on command per actual power transition.
