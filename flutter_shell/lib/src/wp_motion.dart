// SPDX-License-Identifier: GPL-3.0-only
// Windows Phone motion guidance (MSDN "Animations, motion, and output"):
// enter → logarithmic ease-out; leave → exponential ease-in.
import 'package:flutter/animation.dart';

import 'dart:math' as math;

/// WP Toolkit SlideUpFadeIn/SlideDownFadeOut (pinned research):350/250ms,
/// ExponentialEase exponent6. Reverse flight uses mirrored exponential easing.
/// This does not claim every W10M dialog uses the older slide transition.
abstract final class WpMotion {
  /// ApplicationBar / bottom menu enter (slide to rest).
  static const barEnter = Duration(milliseconds: 350);

  /// ApplicationBar / bottom menu exit (accelerate off-screen).
  static const barExit = Duration(milliseconds: 250);

  /// MessageBox / content dialog enter.
  static const dialogEnter = Duration(milliseconds: 350);

  /// MessageBox / content dialog exit.
  static const dialogExit = Duration(milliseconds: 250);

  /// Toast enter.
  static const toastEnter = Duration(milliseconds: 250);

  /// Toast exit.
  static const toastExit = Duration(milliseconds: 200);

  /// Logarithmic enter — WP guidance for objects arriving on screen.
  static const enter = _ExpoOutCurve();

  /// Exponential exit — WP guidance for objects leaving the screen.
  static const exit = _ExpoInCurve();
}

class _ExpoOutCurve extends Curve {
  const _ExpoOutCurve();
  @override
  double transformInternal(double t) =>
      (1 - math.exp(-6 * t)) / (1 - math.exp(-6));
}

class _ExpoInCurve extends Curve {
  const _ExpoInCurve();
  @override
  double transformInternal(double t) =>
      (math.exp(6 * t) - 1) / (math.exp(6) - 1);
}
