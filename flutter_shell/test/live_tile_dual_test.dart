// SPDX-License-Identifier: GPL-3.0-only
import 'package:test/test.dart';
import 'package:win10x_shell/src/dual_screen.dart';
import 'package:win10x_shell/src/motion_math.dart';

/// Mirrors Disco hide-flip-direction-0 (pure-dart; Curves live in Flutter).
double _flipDegrees(double t) {
  final x = t.clamp(0.0, 1.0);
  if (x <= 0.20) {
    final u = x / 0.20;
    return -90.0 * u * u;
  }
  final u = ((x - 0.21) / 0.79).clamp(0.0, 1.0);
  final e = 1 - (1 - u) * (1 - u);
  return 90.0 * (1.0 - e);
}

void main() {
  test('Disco live-tile flip degrees match hide-flip-direction-0 segments', () {
    expect(_flipDegrees(0), closeTo(0, 0.01));
    expect(_flipDegrees(0.20), closeTo(-90, 0.5));
    expect(_flipDegrees(1.0), closeTo(0, 0.5));
    expect(cubicBezier(0.5, .2, 1.25, .5, 1), greaterThan(0.5));
  });

  test('dual-screen geometry keeps panes off the hinge', () {
    final geo = DualScreenGeometry.fromOutputs(
      primary: const ShellRect.fromLTWH(0, 0, 800, 1200),
      secondary: const ShellRect.fromLTWH(824, 0, 800, 1200),
      hingeThickness: 24,
    );
    expect(geo.hinge.width, 24);
    final panes = geo.panesFor(DualScreenLayoutMode.listDetail);
    expect(panes.$1.right, lessThanOrEqualTo(geo.hinge.left + 0.01));
    expect(panes.$2.left, greaterThanOrEqualTo(geo.hinge.right - 0.01));
    final wonder = DualScreenGeometry.wonderBarRegion(
      const ShellRect.fromLTWH(0, 0, 800, 1200),
      const ShellRect.fromLTWH(0, 900, 800, 300),
    );
    expect(wonder.height, closeTo(900, 0.1));
  });

  test('posture inference matches dual-screen layout heuristics', () {
    expect(
      inferPosture(
        const ShellRect.fromLTWH(0, 0, 800, 1200),
        const ShellRect.fromLTWH(824, 0, 800, 1200),
      ),
      DualScreenPosture.book,
    );
    expect(
      inferPosture(
        const ShellRect.fromLTWH(0, 0, 800, 600),
        const ShellRect.fromLTWH(0, 624, 800, 600),
      ),
      DualScreenPosture.tent,
    );
    expect(
      inferPosture(
        const ShellRect.fromLTWH(0, 0, 800, 1200),
        const ShellRect.fromLTWH(0, 900, 800, 300),
      ),
      DualScreenPosture.keyboardCover,
    );
  });

  test('extended canvas pane is the dual bounding box', () {
    final geo = DualScreenGeometry.fromOutputs(
      primary: const ShellRect.fromLTWH(0, 0, 800, 1200),
      secondary: const ShellRect.fromLTWH(824, 0, 800, 1200),
    );
    final panes = geo.panesFor(DualScreenLayoutMode.extendedCanvas);
    expect(panes.$1.width, closeTo(1624, 0.1));
    expect(panes.$2.isEmpty, isTrue);
  });
}
