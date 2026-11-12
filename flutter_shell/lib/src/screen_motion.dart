// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/foundation.dart';

/// Shared screen timeline: wallpaper 0–400ms; clock 200–600ms.
abstract final class ScreenMotion {
  static Duration? startCloseDuration;
  static final reenter = ValueNotifier<int>(0);
  static final progress = ValueNotifier<double>(1);
  static final startMotion = ValueNotifier<bool>(false);
  static double wallpaper(double t) => (t * 1.5).clamp(0, 1);
  static double chrome(double t) => ((t * 600 - 200) / 400).clamp(0, 1);
}
