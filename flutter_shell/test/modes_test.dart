// SPDX-License-Identifier: GPL-3.0-only
import 'package:test/test.dart';
import 'package:win10x_shell/src/model.dart';

void main() {
  test(
    'legacy configuration migrates to tablet without losing saved layouts',
    () {
      final p = ShellPreferences.fromJson({
        'start_layout_style': 'windows8',
        'pinned_apps': ['a'],
      });
      expect(p.mode, ShellMode.tablet);
      p.mode = ShellMode.phone;
      expect(p.effectiveLayout, StartLayout.windowsPhone);
      p.mode = ShellMode.desktop;
      expect(p.effectiveLayout, StartLayout.windows8);
      expect(p.pins, ['a']);
    },
  );
  test('all modes and phone density round-trip independently', () {
    for (final mode in ShellMode.values) {
      final p = ShellPreferences(
        mode: mode,
        layout: StartLayout.windows10,
        phoneMoreTiles: true,
        tiles: [
          const StartTile(appId: 'a', col: 6, row: 3, width: 2, height: 2),
        ],
      );
      final restored = ShellPreferences.fromJson(p.toJson());
      expect(restored.mode, mode);
      expect(restored.phoneMoreTiles, isTrue);
      expect(restored.tiles.single.col, 6);
      expect(restored.tiles.single.row, 3);
    }
  });
  test(
    'phone uses classic four columns and optional six without rewriting cells',
    () {
      final p = ShellPreferences(
        mode: ShellMode.phone,
        tiles: [const StartTile(appId: 'a', col: 4)],
      );
      final four = layoutTiles(p, 360, 700).single.rect;
      expect(four.width, closeTo(174, .001));
      p.phoneMoreTiles = true;
      final six = layoutTiles(p, 360, 700).single.rect;
      expect(six.width, closeTo(114, .001));
      expect(p.tiles.single.col, 4);
    },
  );
  test(
    'tablet and phone fill output while desktop Start stays above its taskbar',
    () {
      for (final mode in ShellMode.values) {
        final p = ShellPreferences(mode: mode);
        final r = launcherGeometry(p, 1280, 800);
        if (mode == ShellMode.desktop) {
          expect(r.width, 900);
          expect(r.bottom, 760);
          expect(r.height, 640);
        } else {
          expect(r.width, 1280);
          expect(r.height, 800);
        }
        final center = actionCenterGeometry(p, 1280, 800);
        expect(center.bottom, 800 - p.bottomInset);
        expect(center.y, p.isPhone ? 24 : 0);
        expect(center.width, p.isPhone ? 1280 : 380);
      }
    },
  );
  test('panel geometry clamps at small portrait and short landscape sizes', () {
    for (final mode in ShellMode.values) {
      for (final size in [(320.0, 480.0), (780.0, 320.0), (240.0, 60.0)]) {
        final p = ShellPreferences(mode: mode);
        for (final r in [
          launcherGeometry(p, size.$1, size.$2),
          actionCenterGeometry(p, size.$1, size.$2),
        ]) {
          expect(r.x, greaterThanOrEqualTo(0));
          expect(r.y, greaterThanOrEqualTo(0));
          expect(r.width, inInclusiveRange(0, size.$1));
          expect(r.height, inInclusiveRange(0, size.$2));
        }
      }
    }
  });
  test(
    'alphabet jump groups real names and does not invent Latin readings',
    () {
      expect(catalogSection('  firefox'), 'F');
      expect(catalogSection('3D Viewer'), '#');
      expect(catalogSection('终端'), '#');
      expect(catalogSection(''), '#');
    },
  );
  test(
    'tablet groups share columns and keep all projected rectangles disjoint',
    () {
      final p = ShellPreferences(
        mode: ShellMode.tablet,
        groups: [const TileGroup(1, 'A'), const TileGroup(2, 'B')],
        tiles: [
          const StartTile(appId: 'a'),
          const StartTile(appId: 'b', groupId: 2),
        ],
      );
      final wide = layoutTiles(p, 1000, 700);
      expect(wide[0].rect.y, wide[1].rect.y);
      expect(wide[1].rect.x, greaterThan(wide[0].rect.right));
      final narrow = layoutTiles(p, 320, 700);
      expect(narrow[1].rect.y, greaterThan(narrow[0].rect.bottom));
      expect(p.tiles[1].col, 0);
      expect(p.tiles[1].row, 0);
      for (final width in [240.0, 320.0, 390.0, 800.0, 1000.0]) {
        for (final mode in ShellMode.values) {
          p.mode = mode;
          final placements = layoutTiles(p, width, 700);
          for (final a in placements) {
            for (final b in placements) {
              if (!identical(a, b)) expect(a.rect.overlaps(b.rect), isFalse);
            }
          }
          expect(placements.every((tile) => tile.rect.right <= width), isTrue);
        }
      }
    },
  );
}
