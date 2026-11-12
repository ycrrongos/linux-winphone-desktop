// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/animation.dart';

import 'motion_math.dart';

/// Flutter supplies the clock; the motion is the legacy Rust Fluent policy.
abstract final class FluentMotion {
  static const fast = Duration(milliseconds: 100);
  static const normal = Duration(milliseconds: 167);
  static const slow = Duration(milliseconds: 250);
  static const slower = Duration(milliseconds: 333);
  static const standard = _LegacyCurve(false);
  static const accelerate = _LegacyCurve(true);
}

class _LegacyCurve extends Curve {
  const _LegacyCurve(this.accelerating);
  final bool accelerating;
  @override
  double transformInternal(double t) =>
      accelerating ? easeAccelerate(t) : easeDecelerate(t);
}
