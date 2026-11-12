# Cerro phone desktop and KDE login

The 2026-11-12 user request explicitly selects Win10X Mobile as the default
SDDM session and asks for KDE's login screen. This supersedes the earlier
experimental-session instruction to preserve Plasma autologin.

## Device layout

- Debian 13.6 arm64 on Nubia Z60 Ultra; pinned Denial 0.6.2.
- Versioned root-owned runtimes: `/opt/win10x-denial/<source>-arm64-<date>`.
  `/opt/win10x-denial/current` selects the installed generation. Keep previous
  generations for rollback; runtime directories must be readable/traversable.
- `/usr/local/bin/win10x-denial-phone-session` comes from
  `packaging/win10x-denial-phone-session`. It uses a private session D-Bus and
  the existing phone configuration, native DSI output at 1116×2480 / 2.5 scale,
  without replacing Plasma's configuration.
- `/usr/share/wayland-sessions/win10x-denial.desktop` remains a separate entry;
  `/usr/share/wayland-sessions/plasma.desktop` remains available.
- SDDM and Breeze were already installed. Autologin is disabled, Breeze is
  enabled, and `/var/lib/sddm/state.conf` selects Win10X Mobile for `rong`.
  RememberLastSession retains a later explicit session choice.
- `/etc/sddm.conf.d/zz-win10x-phone.conf` uses the distribution's KWin/Maliit
  Wayland greeter pattern, with `QT_SCALE_FACTOR=1.25`. The initial 2.0 scale
  squeezed controls in portrait. The user confirmed 1.25 fits on 2026-11-12.
  This scale is separate from the desktop's DSI output scale.

The KWin/Maliit greeter command follows the
[Debian Plasma 6.3.6 configuration](https://sources.debian.org/src/plasma-workspace/4%3A6.3.6-2/sddm-wayland-session/plasma-wayland.conf/).
No new display manager or distribution upgrade was needed.

## Recovery and testing

Pre-change phone backup: `/var/backups/win10x-login-20261112/` (root only),
including old SDDM configuration/state, wrapper, previous-generation link target
and phone settings. Its `rollback.sh` restores the earlier login configuration
and restarts SDDM. This ends a current graphical session; use SSH when needed.
The temporary smoke service and rollback timer were runtime-only, not enabled
for subsequent boots.

Host USB NetworkManager profile `codex-cerro-usb` is restricted to the verified
USB gadget interface/MAC, with 192.168.68.1/24, no default route and no DNS.
SSH uses the previously verified phone host key, not disabled verification.
Credentials remain only in the phone project's ignored private credential file.

Native Start and a separate Konsole were captured for the reviewed c79511384aa8
ARM64 generation. A later observed boot loaded SDDM/Breeze; journal records
successful password authentication and Win10X startup through the real SDDM
session entry. The user confirmed the corrected login sizing. This does not
establish acceptance of every desktop backend, application or gesture.

The staged wrapper also seeds a missing Denial power-button preference to DPMS
(screen off/on), preserving any explicit setting. Native preflight had exposed
upstream's desktop default `PowerOff`, which is unsuitable as the phone default.
The seed uses an atomic JSON update and was tested against missing settings and
an explicit existing choice. This wrapper correction is staged only while the
phone is disconnected; hardware power-key behavior has not been re-tested.

Latest task-card / alphabet source `ecc33d3a5564` is built and staged for ARM64,
not installed. Its archive is `target/denial-arm64/win10x-phone-ecc33d3a5564.tar.gz`;
last verified installed phone source remains `c79511384aa8`. Host UI exercises
do not establish real-window focus/close or Chromium acceptance.

## 2026-11-13 installed Chinese/motion generation

Current now selects `e805bf9200f7-arm64-ime-20261113`. Archive and app/engine
checksums verified before installation; backup/rollback is root-only at
`/var/backups/win10x-ime-20261113/`. Phone wrapper and screen-hook helper updated;
no root-hook permission granted. Authentication generation42a6755e88f4 and
accepted Breeze125% configuration unchanged (hash-verified).

Bounded separate VT8 verified actual GTK Chinese candidate commit你好, unlocked
native gpio volume top-only, secure swipe/PAM/editor automatic keyboard/test
key, and native toast finger-follow/cancel. Evidence is under
`target/denial-arm64/qa/ime-motion-20261113/`. No password submitted. Chromium
Chinese input was not tested. Locked hardware volume remains suppressed by
upstream's native secure shortcut policy, not accepted as working.

Tests returned to original tty2/PID1907, same bootffba7009. That preserved running
session still has its old loaded engine; the new installed default takes effect
on the user's normal next login. No forced logout/SIGUSR1/SDDM restart/reboot.
