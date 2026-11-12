// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

/// Homography matching Rust project_turnstile_point, in tile-local pixels.
/// Kept pure Dart so background/content perspective can be checked together.
class DiscoProjection {
  DiscoProjection({
    required double width,
    required double height,
    required double outerDegrees,
    required double innerDegrees,
    required double innerX,
    required double translateX,
  }) {
    final outer = outerDegrees * math.pi / 180;
    final inner = innerDegrees * math.pi / 180;
    final a = math.cos(outer + inner);
    final b = (width / 2 + innerX) * math.cos(outer) - width / 2 * a;
    final c = -math.sin(outer + inner);
    final d = -(width / 2 + innerX) * math.sin(outer) - width / 2 * c;
    wx = -c / 2000;
    ww = 1 - d / 2000;
    xx = a + translateX * wx;
    xt = b + translateX * ww;
    yx = height / 2 * wx;
    yt = height / 2 * (ww - 1);
  }
  late final double xx, xt, yx, yt, wx, ww;
  ({double x, double y}) project(double x, double y) {
    final w = wx * x + ww;
    return (x: (xx * x + xt) / w, y: (yx * x + y + yt) / w);
  }
}
