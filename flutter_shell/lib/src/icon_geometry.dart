// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

({double scale, double width, double height, double dx, double dy})
fitIconBounds(List<double> bounds, double extent, double dpr) {
  final optical = (extent * .86 * dpr).roundToDouble() / dpr;
  final scale =
      optical / math.max(bounds[4] - bounds[2], bounds[5] - bounds[3]);
  return (
    scale: scale,
    width: bounds[0] * scale,
    height: bounds[1] * scale,
    dx: (bounds[0] / 2 - (bounds[2] + bounds[4]) / 2) * scale,
    dy: (bounds[1] / 2 - (bounds[3] + bounds[5]) / 2) * scale,
  );
}
