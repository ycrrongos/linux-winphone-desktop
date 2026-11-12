import 'dart:math' as math;

import 'package:test/test.dart';
import 'package:win10x_shell/src/icon_geometry.dart';
import 'package:win10x_shell/src/markui_metrics.dart';

void main() {
  test(
    'original icon alpha is centered, proportional and bounded at all scales',
    () {
      for (final entry in markUiBounds.entries) {
        final b = entry.value;
        for (final size in [16.0, 24.0, 32.0, 48.0, 64.0]) {
          for (final dpr in [1.0, 1.25, 1.5, 2.5]) {
            final f = fitIconBounds(b, size, dpr);
            final painted = math.max(b[4] - b[2], b[5] - b[3]) * f.scale;
            expect(painted, lessThanOrEqualTo(size));
            expect(
              (painted * dpr).roundToDouble(),
              closeTo(painted * dpr, 1e-8),
            );
            expect(f.width / f.height, closeTo(b[0] / b[1], 1e-8));
            expect(
              ((b[2] + b[4]) / 2 - b[0] / 2) * f.scale + f.dx,
              closeTo(0, 1e-8),
            );
            expect(
              ((b[3] + b[5]) / 2 - b[1] / 2) * f.scale + f.dy,
              closeTo(0, 1e-8),
            );
          }
        }
      }
    },
  );
}
