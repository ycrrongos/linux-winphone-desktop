// SPDX-License-Identifier: GPL-3.0-only
/// Keep IDs stable while the compositor updates active flags or removes windows.
class PhoneTaskOrder {
  final ids = <int>[];
  bool _initialized = false;
  void sync(Iterable<int> current, int? active) {
    final next = current.toSet();
    if (!_initialized) {
      if (active != null && next.contains(active)) ids.add(active);
      ids.addAll(next.where((id) => id != active));
      _initialized = true;
    } else {
      ids.removeWhere((id) => !next.contains(id));
      ids.addAll(next.where((id) => !ids.contains(id)));
    }
  }
}

int phoneTaskPageCapacity(double width, double height) =>
    width >= 420 && height >= 720 ? 4 : 1;

bool dismissPhoneTask({
  required double distance,
  required double height,
  required double velocity,
}) => velocity >= -300 && (distance >= height * .2 || velocity > 650);
