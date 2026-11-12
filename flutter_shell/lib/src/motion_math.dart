// SPDX-License-Identifier: GPL-3.0-only
// Numerical motion from shell/src/fluent1/motion.rs and start_tiles.rs.
import 'dart:math' as math;

double easeDecelerate(double t) =>
    1 - math.pow(1 - t.clamp(0.0, 1.0), 2.6).toDouble();
double easeAccelerate(double t) => math.pow(t.clamp(0.0, 1.0), 2.6).toDouble();

double cubicBezier(double x, double x1, double y1, double x2, double y2) {
  if (x <= 0) return 0;
  if (x >= 1) return 1;
  double sample(double t, double a, double b) =>
      3 * (1 - t) * (1 - t) * t * a + 3 * (1 - t) * t * t * b + t * t * t;
  var low = 0.0, high = 1.0;
  for (var i = 0; i < 14; i++) {
    final t = (low + high) / 2;
    if (sample(t, x1, x2) < x) {
      low = t;
    } else {
      high = t;
    }
  }
  return sample((low + high) / 2, y1, y2);
}

double easeOutExpo(double t) => cubicBezier(t, .16, 1, .3, 1);
double easeInExpo(double t) => 1 - easeOutExpo(1 - t);
double flutterEaseOutCubic(double t) => cubicBezier(t, .215, .61, .355, 1);

/// Legacy Rust `ease_out_back` (c1 = 1.70158) used by tile drop settle.
double easeOutBack(double t) {
  final x = t.clamp(0.0, 1.0);
  const c1 = 1.70158;
  const c3 = c1 + 1.0;
  final u = x - 1.0;
  return 1.0 + c3 * u * u * u + c1 * u * u;
}
