// SPDX-License-Identifier: GPL-3.0-only
/// Phone equivalent of showing an enabled editor on activation. Native focus
/// and text ownership remain compositor-owned. Explicit user dismissal wins
/// until a new editor activation; desktop callers retain requested-only policy.
class TouchKeyboardPolicy {
  int? _automaticSerial;
  int? _dismissedSerial;
  void reset() {
    _automaticSerial = null;
    _dismissedSerial = null;
  }

  void dismiss(int serial) {
    _dismissedSerial = serial;
    _automaticSerial = null;
  }

  bool update({
    required bool active,
    required bool requested,
    required int serial,
    required bool allowAutofocus,
  }) {
    if (!active) {
      reset();
      return false;
    }
    if (_dismissedSerial == serial) return false;
    if (!allowAutofocus) {
      _automaticSerial = null;
      return requested;
    }
    if (_automaticSerial != serial) _automaticSerial = serial;
    return requested || _automaticSerial == serial;
  }
}
