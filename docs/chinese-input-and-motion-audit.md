# Chinese keyboard and motion audit — 2026-11-13

## Offline Chinese input

Studied Sixuan IME revision62bf21599601ec0521eb03a3f356c7328b92a520:
https://github.com/sixuan-ai/sixuan-ime . Its pinyin.c uses Rime; dictionary.go /
bundled_dictionary.go separate local and personal candidates and validate bundled
data. Its original PolyForm Noncommercial code is not copied into this GPL shell.

Our independently implemented decoder uses the unmodified Apache-2.0 Rime
pinyin-simp dictionary (over65k entries), pinned revision and full notice/license
under flutter_shell/assets/ime/. This is a portable Dart decoder, not a claim
that the native librime engine is installed. It supports full pinyin, initials,
partial-last-syllable suggestions, explicit apostrophes, ü/v, dictionary-word
sentence segmentation, prefix selection and local frequency learning. It does
not provide cloud predictions, voice recognition, or Rime's full schema/plugin
compatibility.

The dictionary/index live in a worker isolate. Queries are debounced, stale
responses are rejected, input length/candidate count and personal entries are
bounded. A retained keyboard owns the worker rather than spawning one per key.
Composing pinyin stays on the keyboard; only choosing a candidate or committing
it with Space sends Chinese to Denial's existing native text-input broker.
Candidates scroll horizontally and expand in the keyboard area, never in an
external IBus candidate window. Switching editor/hiding keyboard cancels pending
composition; password/PIN/sensitive/numeric purposes use direct input with no
candidate queries or learning. Personal data is local under XDG_STATE_HOME,
private700 directory /600 atomic file; no composing/learned text is logged.

## Motion/input findings

- Raw controller clocks are not necessarily linear visual animations. Start,
  launch, live tiles, app switching, lock wake wallpaper/clock and MessageBox
  already apply nonlinear transforms in their drawing branches. Keep direct
  finger translation linear (1:1); easing on a drag would create lag.
- Toast previously only reacted to release velocity and moved40px on dismissal:
  now its real measured height follows the finger, release/cancel settles with
  WP curves, and interrupted entrance/exit uses continuous controller position.
- Edge-panel cancel previously supplied zero instead of the actual retained
  progress, creating a jump. Preserve progress and settle back nonlinearly.
- Task cards already stop their settle controller on grab, preserve its exact
  offset, follow deltas, and use nonlinear release. Page physics is critically
  damped (nonlinear, without bounce); not an unhandled linear animation.
- Volume used a separate desktop-event forwarding listener plus suppression of
  the original bottom HUD. Replace the phone branch in the actual native HUD
  presenter with one WP top renderer, above desktop and secure stage. Native
  limit/repeated-key events now take that same path; desktop HUD remains native.
  Register only the top controls' bounds for native input.

Phone remains unavailable per the user's last instruction. Native key routing is
exercised locally by injecting audio/limit events into the real HUD provider;
compiled code must still be deployed before the physical phone can change from
its older installed generation. No active-session refresh or phone SSH is used.

Further audit fixes: Action Center close originally normalized by55% of screen
height but painted with full-height travel. It now shares actual geometry with
painting, preserves mid-settle grab origin, and restores on cancellation. Raw
PointerCancel is handled before Flutter's accepted-drag onEnd across these paths;
losing a gesture to a child scroll does not trigger a fake panel drag. Lock cover
stays mounted even when dragged a full screen until the actual release requests
credentials. Queued IME actions and stale candidate buttons are checked against
editor ownership/current composition so they cannot commit to a newly focused
field after an asynchronous lookup.

## 2026-11-13 phone follow-up

User subsequently reconnected/authorized deployment. e805bf9200f7 is installed
and selected. Actual native GTK received你好 via keyboard candidate tap; real
native audio key path showed top-only WP HUD. Secure input and native toast
cancel assertions pass. Original active session retained (new default on next
normal login). Upstream locked shortcut guard suppresses volume controls, so
lock-screen volume is not accepted; Chromium Chinese delivery remains untested.
