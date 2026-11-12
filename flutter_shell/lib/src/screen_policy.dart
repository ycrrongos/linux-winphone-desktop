// SPDX-License-Identifier: GPL-3.0-only
/// A startup snapshot is not a transition; repeated native status never reruns hooks.
class ScreenPowerEdges {
  bool? _powered;
  bool? observe(bool powered) {
    final old = _powered;
    _powered = powered;
    return old == null || old == powered ? null : powered;
  }
}

bool screenIdleDue({
  required bool enabled,
  required bool powered,
  required bool transitioning,
  required bool inhibited,
  required int idleMs,
  required int seconds,
}) =>
    enabled &&
    powered &&
    !transitioning &&
    !inhibited &&
    idleMs >= seconds * 1000;
