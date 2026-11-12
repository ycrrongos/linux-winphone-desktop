// SPDX-License-Identifier: GPL-3.0-only
import 'dart:convert';

import 'package:test/test.dart';
import 'package:win10x_shell/src/model.dart';
import 'package:win10x_shell/src/start_motion.dart';
import 'package:win10x_shell/src/screen_policy.dart';

void main() {
  test('screen hooks are edges, not startup or repeated power reports', () {
    final edges = ScreenPowerEdges();
    expect(edges.observe(true), isNull);
    expect(edges.observe(true), isNull);
    expect(edges.observe(false), false);
    expect(edges.observe(false), isNull);
    expect(edges.observe(true), true);
    expect(
      screenIdleDue(
        enabled: true,
        powered: true,
        transitioning: false,
        inhibited: false,
        idleMs: 29999,
        seconds: 30,
      ),
      false,
    );
    expect(
      screenIdleDue(
        enabled: true,
        powered: true,
        transitioning: false,
        inhibited: false,
        idleMs: 30000,
        seconds: 30,
      ),
      true,
    );
    expect(
      screenIdleDue(
        enabled: true,
        powered: true,
        transitioning: false,
        inhibited: true,
        idleMs: 30000,
        seconds: 30,
      ),
      false,
    );
    expect(
      screenIdleDue(
        enabled: false,
        powered: true,
        transitioning: false,
        inhibited: false,
        idleMs: 30000,
        seconds: 30,
      ),
      false,
    );
  });

  test('screen settings and navigation choice survive round trip with bounded timeout', () {
    final prefs = ShellPreferences.fromJson({
      'phone_task_button': true,
      'auto_screen_off': false,
      'screen_off_seconds': -1,
      'screen_hooks_root': true,
      'screen_off_command': 'sudo example off',
      'screen_on_command': 'echo on',
      'desktop_wallpaper_path': '/tmp/wallpaper.png',
    });
    final restored = ShellPreferences.fromJson(prefs.toJson());
    expect(restored.phoneTaskButton, isTrue);
    expect(restored.autoScreenOff, isFalse);
    expect(restored.screenOffSeconds, 15);
    expect(restored.screenHooksRoot, isTrue);
    expect(restored.screenOffCommand, 'sudo example off');
    expect(restored.screenOnCommand, 'echo on');
    expect(restored.desktopWallpaperPath, '/tmp/wallpaper.png');
  });

  test(
    'freeform collisions prefer the vacated cell and reverse from the snapshot',
    () {
      final original = [
        const StartTile(appId: 'a', col: 0),
        const StartTile(appId: 'b', col: 2),
        const StartTile(appId: 'c', col: 4),
      ];
      final preview = previewFreeformDrop(original, 'a', 1, 2, 0, 8);
      expect(preview.first.col, 2);
      expect(preview[1].col, 0);
      expect(preview.last.col, 4);
      final reversed = previewFreeformDrop(original, 'a', 1, 0, 0, 8);
      expect(reversed[1].col, 2);
      expect(original.first.col, 0);
    },
  );

  test('menu resize keeps the tile and pushes overlapping neighbors aside', () {
    final tiles = [
      const StartTile(appId: 'a', col: 0, width: 2, height: 2),
      const StartTile(appId: 'b', col: 2, width: 2, height: 2),
    ];
    final next = [tiles.first.copyWith(width: 4, height: 2), tiles.last];
    final resolved = resolveFreeformResize(next, 'a', 8);
    expect(resolved.first.width, 4);
    expect(resolved.first.col, 0);
    expect(resolved.last.col, isNot(2));
  });

  test('corner-handle sizing grows from the tile origin like legacy', () {
    const tile = StartTile(appId: 'a', col: 2, row: 1, width: 2, height: 2);
    expect(
      sizeFromPointer(
        tile: tile,
        boardX: 2 * 70 + 4 * 70,
        boardY: 38 + 1 * 70 + 3 * 70,
        originX: 0,
        originY: 38,
        cell: 70,
      ),
      (4, 3),
    );
    expect(
      windows8SizeFromPointer(
        boardX: 100 + 120 * 1.5,
        rect: const TileRect(100, 40, 120, 120),
      ),
      (4, 2),
    );
    expect(
      windows8SizeFromPointer(
        boardX: 100 + 50,
        rect: const TileRect(100, 40, 120, 120),
      ),
      (2, 2),
    );
  });

  test(
    'freeform drag previews prefer the vacated cell and restore from snapshot',
    () {
      final original = [
        const StartTile(appId: 'a', col: 0),
        const StartTile(appId: 'b', col: 2),
        const StartTile(appId: 'c', col: 4),
      ];
      final overB = previewFreeformDrop(original, 'a', 1, 2, 0, 8);
      expect(overB.map((tile) => (tile.appId, tile.col)).toList(), [
        ('a', 2),
        ('b', 0),
        ('c', 4),
      ]);
      // Conflicts move to the nearest free cell from their own origin; the
      // vacated slot is only a tie-break, matching legacy resolve_avoid_toward.
      final farther = previewFreeformDrop(original, 'a', 1, 4, 0, 8);
      expect(farther.map((tile) => (tile.appId, tile.col)).toList(), [
        ('a', 4),
        ('b', 2),
        ('c', 6),
      ]);
      final home = previewFreeformDrop(original, 'a', 1, 0, 0, 8);
      expect(home.map((tile) => (tile.appId, tile.col)).toList(), [
        ('a', 0),
        ('b', 2),
        ('c', 4),
      ]);
    },
  );

  test(
    'a drop far below Metro groups still selects the nearest real group',
    () {
      final groups = [const TileGroup(1, 'A'), const TileGroup(2, 'B')];
      final prefs = ShellPreferences(
        layout: StartLayout.windows8,
        groups: groups,
        tiles: [
          const StartTile(appId: 'a'),
          const StartTile(appId: 'b', groupId: 2),
        ],
      );
      final slots = layoutTiles(prefs, 1000, 900);
      expect(nearestTileGroup(slots, groups, slots.last.rect.x + 20, 4000), 2);
    },
  );
  test(
    'imports separate legacy shell and tile documents without losing groups',
    () {
      final prefs = ShellPreferences.fromJson(
        {
          'start_layout_style': 'windows_phone',
          'start_tile_animation': 'disco_turnstile',
          'pinned_apps': ['Alacritty'],
          'app_switch_animation_enabled': false,
        },
        legacyTiles: {
          'groups': [
            {'id': 9, 'title': '工作'},
          ],
          'tiles': [
            {
              'app_id': 'Alacritty',
              'group_id': 9,
              'col': 4,
              'row': 6,
              'w': 4,
              'h': 2,
            },
          ],
        },
      );
      expect(prefs.layout, StartLayout.windowsPhone);
      expect(prefs.launchAnimation, LaunchAnimation.discoTurnstile);
      expect(prefs.switchAnimation, isFalse);
      expect(prefs.groups.single.title, '工作');
      expect(prefs.tiles.single.toJson(), {
        'app_id': 'Alacritty',
        'group_id': 9,
        'col': 4,
        'row': 6,
        'w': 4,
        'h': 2,
      });
      expect(prefs.pins, ['Alacritty']);
    },
  );

  test('explicit launch style stays independent from Start style', () {
    final prefs = ShellPreferences.fromJson({
      'start_tile_animation': 'flutter_metro_group',
      'app_launch_animation': 'windows8',
    });
    expect(prefs.launchAnimation, LaunchAnimation.windows8);
    expect(prefs.startAnimation, StartAnimation.flutterMetroGroup);
  });

  test('temporarily unavailable apps and orphan groups survive round trip', () {
    final prefs = ShellPreferences.fromJson({
      'tiles': [
        {
          'app_id': 'org.example.Uninstalled',
          'group_id': 8,
          'col': 2,
          'row': 3,
          'w': 2,
          'h': 2,
        },
      ],
    });
    final restored = ShellPreferences.fromJson(
      jsonDecode(jsonEncode(prefs.toJson())) as Map<String, dynamic>,
    );
    expect(restored.tiles.single.appId, 'org.example.Uninstalled');
    expect(restored.groups.any((group) => group.id == 8), isTrue);
  });

  test(
    'catalog normalization resolves legacy desktop IDs without persisted Exec',
    () {
      expect(
        normalizedAppId('desktop:Alacritty.desktop'),
        normalizedAppId('Alacritty'),
      );
      expect(
        normalizedAppId('/usr/share/applications/org.gnome.Nautilus.desktop'),
        'org.gnome.nautilus',
      );
    },
  );

  test('Metro column-first packing keeps mixed wide tiles collision-free', () {
    final prefs = ShellPreferences(
      layout: StartLayout.windows8,
      tiles: [
        for (var i = 0; i < 32; i++)
          StartTile(appId: '$i', width: i.isEven ? 4 : 2),
      ],
    );
    final placements = layoutTiles(prefs, 1000, 1032);
    expect(placements.first.rect.width, 250);
    expect(placements[1].rect.width, 120);
    expect(placements[1].rect.y - placements[0].rect.y, 130);
    for (var i = 0; i < placements.length; i++) {
      for (final other in placements.skip(i + 1)) {
        expect(placements[i].rect.overlaps(other.rect), isFalse);
      }
    }
    expect(placements.map((item) => item.rect.y).toSet().length, 6);
  });

  test(
    'derived Metro layout and narrow projection do not rewrite freeform cells',
    () {
      final prefs = ShellPreferences(
        tiles: [
          const StartTile(appId: 'wide', col: 12, row: 6, width: 4),
          const StartTile(appId: 'other', col: 12, row: 6),
        ],
      );
      final before = jsonEncode(prefs.toJson());
      final narrow = layoutTiles(prefs, 220, 600);
      expect(narrow.first.rect.overlaps(narrow.last.rect), isFalse);
      expect(jsonEncode(prefs.toJson()), before);
      prefs.layout = StartLayout.windows8;
      layoutTiles(prefs, 1200, 900);
      expect(prefs.tiles.first.col, 12);
      expect(prefs.tiles.first.row, 6);
    },
  );

  test(
    'Metro reorder accepts an empty destination group and is reversible',
    () {
      final original = [
        const StartTile(appId: 'a'),
        const StartTile(appId: 'b'),
      ];
      final moved = previewMetroDrop(original, 'a', 7, 20);
      expect(moved.last.appId, 'a');
      expect(moved.last.groupId, 7);
      expect(original.first.groupId, 1);
      expect(previewMetroDrop(original, 'a', 1, 0).map((tile) => tile.appId), [
        'a',
        'b',
      ]);
      expect(previewMetroDrop(original, 'a', 1, 1).map((tile) => tile.appId), [
        'b',
        'a',
      ]);
    },
  );

  test('W10M reveal math matches legacy stagger and reverse row order', () {
    expect(StartReveal.mobileTotalDuration, closeTo(0.40, 1e-9));
    expect(StartReveal.mobileScaleFrom, 0.91);
    expect(
      StartReveal.mobileRowDelay(rowIndex: 0, rowSpan: 3, closing: false),
      closeTo(StartReveal.mobileStaggerSpan, 1e-9),
    );
    expect(
      StartReveal.mobileRowDelay(rowIndex: 3, rowSpan: 3, closing: false),
      closeTo(0.0, 1e-9),
    );
    expect(
      StartReveal.mobileRowDelay(rowIndex: 0, rowSpan: 3, closing: true),
      closeTo(0.0, 1e-9),
    );
    final opening = StartReveal.animationProgress(
      local: 0.06,
      duration: StartReveal.mobileDuration,
      closing: false,
    );
    final closing = StartReveal.animationProgress(
      local: 0.06,
      duration: StartReveal.mobileDuration,
      closing: true,
    );
    expect(opening.$1, isTrue);
    expect(opening.$2, greaterThan(0.3));
    // Closing starts near settled and accelerates away; early local stays high.
    expect(closing.$2, greaterThan(0.9));
    expect(closing.$2, greaterThan(opening.$2));
    expect(
      StartReveal.flutterMetroGroupProgress(
        elapsed: 0.0,
        closing: false,
        groupIndex: 0,
      ),
      0.0,
    );
    expect(
      StartReveal.flutterMetroGroupProgress(
        elapsed: 1.05,
        closing: false,
        groupIndex: 0,
      ),
      1.0,
    );
  });

  test(
    'packTilesTopLeft fills holes top-to-left like MetroLauncher GridPacker',
    () {
      final tiles = [
        const StartTile(appId: 'a', col: 0, row: 0, width: 2, height: 2),
        const StartTile(appId: 'b', col: 4, row: 2, width: 2, height: 2),
      ];
      final packed = packTilesTopLeft(tiles, 6);
      expect(packed.first.appId, 'a');
      expect(packed.first.col, 0);
      expect(packed.first.row, 0);
      expect(packed.last.appId, 'b');
      expect(packed.last.col, 2);
      expect(packed.last.row, 0);
    },
  );

  test('prefs persist font sizes, wallpaper path, acrylic and oobe', () {
    final prefs = ShellPreferences(
      drawerFontSize: UiFontSize.large,
      tileFontSize: UiFontSize.small,
      startWallpaperPath: '/tmp/start.png',
      acrylicEnabled: false,
      oobeCompleted: true,
      autoAccentFromIcon: false,
    );
    final roundTrip = ShellPreferences.fromJson(
      Map<String, dynamic>.from(prefs.toJson()),
    );
    expect(roundTrip.drawerFontSize, UiFontSize.large);
    expect(roundTrip.tileFontSize, UiFontSize.small);
    expect(roundTrip.startWallpaperPath, '/tmp/start.png');
    expect(roundTrip.acrylicEnabled, isFalse);
    expect(roundTrip.oobeCompleted, isTrue);
    expect(roundTrip.autoAccentFromIcon, isFalse);
    expect(uiFontScale(UiFontSize.large), 1.2);
  });

  test('resetStartBoard reseeds from pins into one Start group', () {
    final prefs = ShellPreferences(
      pins: ['desktop:a.desktop', 'desktop:b.desktop', 'desktop:c.desktop'],
      tiles: [const StartTile(appId: 'orphan', groupId: 9, col: 0, row: 0)],
      groups: [const TileGroup(9, '旧组')],
    );
    resetStartBoard(prefs, columns: 6);
    expect(prefs.groups, hasLength(1));
    expect(prefs.groups.first.title, '开始');
    expect(prefs.tiles, hasLength(3));
    expect(prefs.tiles.every((tile) => tile.groupId == 1), isTrue);
    expect(prefs.tiles.map((tile) => tile.appId).toSet(), {
      'desktop:a.desktop',
      'desktop:b.desktop',
      'desktop:c.desktop',
    });
  });
}
