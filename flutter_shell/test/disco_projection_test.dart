// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;

import 'package:test/test.dart';
import 'package:win10x_shell/src/disco_projection.dart';

void main() {
  test(
    'Disco content matches Rust inner-center then outer-left perspective',
    () {
      for (final size in [(120.0, 120.0), (250.0, 120.0), (120.0, 250.0)]) {
        for (final angles in [
          (70.0, 45.0, 60.0, 0.0),
          (35.0, 15.0, 20.0, 0.0),
          (-40.0, 0.0, 0.0, -120.0),
        ]) {
          final (w, h) = size;
          final (outerDeg, innerDeg, shift, exit) = angles;
          final actual = DiscoProjection(
            width: w,
            height: h,
            outerDegrees: outerDeg,
            innerDegrees: innerDeg,
            innerX: shift,
            translateX: exit,
          );
          final outer = outerDeg * math.pi / 180;
          final inner = innerDeg * math.pi / 180;
          for (final point in [
            (0.0, 0.0),
            (w, h),
            (w / 2, h * .45),
            (10.0, h - 10),
          ]) {
            final (px, py) = point;
            // Reference: shell/src/start_tiles.rs::project_turnstile_point.
            final cx = w / 2 + (px - w / 2) * math.cos(inner) + shift;
            final cz = -(px - w / 2) * math.sin(inner);
            final x = cx * math.cos(outer) + cz * math.sin(outer);
            final z = -cx * math.sin(outer) + cz * math.cos(outer);
            final scale = 2000 / (2000 - z);
            final result = actual.project(px, py);
            expect(result.x, closeTo(exit + x * scale, 1e-9));
            expect(result.y, closeTo(h / 2 + (py - h / 2) * scale, 1e-9));
          }
        }
      }
    },
  );

  test('settled Disco geometry is exactly the unanimated tile geometry', () {
    final p = DiscoProjection(
      width: 250,
      height: 120,
      outerDegrees: 0,
      innerDegrees: 0,
      innerX: 0,
      translateX: 0,
    );
    expect(p.project(0, 0), (x: 0.0, y: 0.0));
    expect(p.project(250, 120), (x: 250.0, y: 120.0));
    expect(p.project(125, 54), (x: 125.0, y: 54.0));
  });
}
