// SPDX-License-Identifier: GPL-3.0-only
import 'package:test/test.dart';
import 'package:win10x_shell/src/client_geometry.dart';

void main() {
  test(
    'old landscape client remains circular while phone configure is pending',
    () {
      const target = (left: 0.0, top: 24.0, width: 446.4, height: 920.0);
      final fit = fitClientRect(target, 800, 600);
      expect(fit.width / 800, closeTo(fit.height / 600, 1e-12));
      expect(fit.left, closeTo(0, 1e-9));
      expect(fit.top, greaterThan(target.top));
      expect(
        fit.top + fit.height,
        lessThanOrEqualTo(target.top + target.height),
      );
      // Native pointer inverse lands on the same source pixel that is painted.
      final px = fit.left + 400 * fit.width / 800;
      final py = fit.top + 300 * fit.height / 600;
      expect((px - fit.left) * 800 / fit.width, closeTo(400, 1e-9));
      expect((py - fit.top) * 600 / fit.height, closeTo(300, 1e-9));
    },
  );
  test(
    'all intermediate landscape to portrait retained rectangles use one scale',
    () {
      for (var i = 0; i <= 100; i++) {
        final t = i / 100;
        final fit = fitClientRect(
          (
            left: 20 * t,
            top: 24 * t,
            width: 800 + (446.4 - 800) * t,
            height: 600 + (920 - 600) * t,
          ),
          446.4,
          920,
        );
        expect(fit.width / 446.4, closeTo(fit.height / 920, 1e-12));
      }
    },
  );
  test(
    'new matching buffer fills its requested content rect without inset drift',
    () {
      const target = (left: 3.0, top: 27.0, width: 440.0, height: 914.0);
      expect(fitClientRect(target, 440, 914), target);
      expect(fitClientRect(target, 0, 0), target);
    },
  );
}
