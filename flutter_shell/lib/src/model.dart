// SPDX-License-Identifier: GPL-3.0-only
// Pure Dart: persistence and layout are tested without a Flutter engine.
import 'dart:math' as math;

enum ShellMode { tablet, desktop, phone }

const shellModeNames = ['tablet', 'desktop', 'phone'];
const shellModeLabels = ['Windows 10 平板', 'Windows 10 桌面', 'Windows 10 Mobile'];

enum StartLayout { windows10, windowsPhone, windows8 }

enum StartAnimation {
  windows10Mobile,
  discoTurnstile,
  metroRise,
  factorSlideScale,
  flutterMetroGroup,
}

enum LaunchAnimation {
  windows10Mobile,
  windows8,
  discoTurnstile,
  factorClipReveal,
  systemDefault,
}

/// MetroLauncher drawer / tile caption size (D5 / D6).
enum UiFontSize { small, normal, large }

const layoutNames = ['windows10', 'windows_phone', 'windows8'];
const startAnimationNames = [
  'windows10_mobile',
  'disco_turnstile',
  'metro_rise',
  'factor_slide_scale',
  'flutter_metro_group',
];
const launchAnimationNames = [
  'windows10_mobile',
  'windows8',
  'disco_turnstile',
  'factor_clip_reveal',
  'system_default',
];
const uiFontSizeNames = ['small', 'normal', 'large'];
const uiFontSizeLabels = ['小', '正常', '大'];

double uiFontScale(UiFontSize size) => switch (size) {
  UiFontSize.small => 0.85,
  UiFontSize.normal => 1.0,
  UiFontSize.large => 1.2,
};

int _choice(Object? value, List<String> names) =>
    math.max(0, names.indexOf(value.toString()));
int _int(Object? value, int fallback) =>
    value is num ? value.toInt() : fallback;

/// Opaque catalog IDs must be resolved before launching; never persist Exec.
String normalizedAppId(String id) => id
    .replaceFirst(RegExp(r'^desktop:'), '')
    .split('/')
    .last
    .replaceFirst(RegExp(r'\.desktop$', caseSensitive: false), '')
    .toLowerCase();

class TileGroup {
  const TileGroup(this.id, this.title);
  final int id;
  final String title;
  Map<String, Object> toJson() => {'id': id, 'title': title};
}

/// Windows Phone accent palette (MetroLauncher / WP start+theme).
const phoneAccentArgb = <int>[
  0xff0078d7, // cobalt
  0xff00aba9, // teal
  0xff008a00, // green
  0xffa4c400, // lime
  0xffe51400, // red
  0xffe671b8, // magenta
  0xffaa00ff, // purple
  0xfffa6800, // orange
  0xff825a2c, // brown
  0xff6d8764, // olive / taupe-ish
  0xff647687, // steel
  0xff76608a, // mauve
];

const phoneAccentLabels = [
  '钴蓝',
  '青绿',
  '绿色',
  '柠檬',
  '红色',
  '洋红',
  '紫色',
  '橙色',
  '棕色',
  '橄榄',
  '钢青',
  '淡紫',
];

class StartTile {
  const StartTile({
    required this.appId,
    this.groupId = 1,
    this.col = 0,
    this.row = 0,
    this.width = 2,
    this.height = 2,
    this.transparent = false,
    this.colorArgb,
  });
  final String appId;
  final int groupId, col, row, width, height;
  final bool transparent;
  final int? colorArgb;

  factory StartTile.fromJson(Map<String, dynamic> json) => StartTile(
    appId: json['app_id'] as String,
    groupId: _int(json['group_id'], 1),
    col: math.max(0, _int(json['col'], 0)),
    row: math.max(0, _int(json['row'], 0)),
    width: _int(json['w'], 2).clamp(1, 8),
    height: _int(json['h'], 2).clamp(1, 8),
    transparent: json['transparent'] == true,
    colorArgb: json['color'] is num ? (json['color'] as num).toInt() : null,
  );

  StartTile copyWith({
    int? groupId,
    int? col,
    int? row,
    int? width,
    int? height,
    bool? transparent,
    int? colorArgb,
    bool clearColor = false,
  }) => StartTile(
    appId: appId,
    groupId: groupId ?? this.groupId,
    col: col ?? this.col,
    row: row ?? this.row,
    width: width ?? this.width,
    height: height ?? this.height,
    transparent: transparent ?? this.transparent,
    colorArgb: clearColor ? null : (colorArgb ?? this.colorArgb),
  );

  Map<String, Object> toJson() => {
    'app_id': appId,
    'group_id': groupId,
    'col': col,
    'row': row,
    'w': width,
    'h': height,
    if (transparent) 'transparent': true,
    'color': ?colorArgb,
  };
}

class ShellPreferences {
  ShellPreferences({
    this.layout = StartLayout.windows10,
    this.mode = ShellMode.tablet,
    this.phoneMoreTiles = false,
    this.phoneTaskButton = false,
    this.autoScreenOff = true,
    this.screenOffSeconds = 900,
    this.screenOffCommand = '',
    this.screenOnCommand = '',
    this.screenHooksRoot = false,
    this.desktopWallpaperPath,
    this.phoneThemeDark = true,
    this.phoneAccentIndex = 0,
    this.autoAccentFromIcon = true,
    this.startAnimation = StartAnimation.windows10Mobile,
    this.launchAnimation = LaunchAnimation.windows10Mobile,
    this.switchAnimation = true,
    this.taskbarHeight = 56,
    this.drawerFontSize = UiFontSize.normal,
    this.tileFontSize = UiFontSize.normal,
    this.startWallpaperPath,
    this.acrylicEnabled = true,
    this.oobeCompleted = false,
    this.notificationAccessPrompted = false,
    List<String>? pins,
    List<TileGroup>? groups,
    List<StartTile>? tiles,
  }) : pins = pins ?? [],
       groups = groups ?? [const TileGroup(1, '开始')],
       tiles = tiles ?? [];
  ShellMode mode;
  bool phoneMoreTiles;
  bool phoneTaskButton, autoScreenOff, screenHooksRoot;
  int screenOffSeconds;
  String screenOffCommand, screenOnCommand;
  String? desktopWallpaperPath;

  /// WP start+theme Background: dark (black chrome) vs light (white chrome).
  bool phoneThemeDark;

  /// Index into [phoneAccentArgb].
  int phoneAccentIndex;

  /// MetroLauncher ColorUtils: pick nearest WP accent from host/icon color.
  bool autoAccentFromIcon;
  bool get isPhone => mode == ShellMode.phone;
  bool get isTablet => mode == ShellMode.tablet;
  StartLayout get effectiveLayout =>
      isPhone ? StartLayout.windowsPhone : layout;
  double get topInset => isPhone ? 24 : 0;
  double get bottomInset => mode == ShellMode.desktop ? 40 : 48;
  int get phoneAccentArgbValue =>
      phoneAccentArgb[phoneAccentIndex.clamp(0, phoneAccentArgb.length - 1)];
  StartLayout layout;
  StartAnimation startAnimation;
  LaunchAnimation launchAnimation;
  bool switchAnimation;
  double taskbarHeight;
  UiFontSize drawerFontSize;
  UiFontSize tileFontSize;

  /// Optional Start background override (W4); null = desktop wallpaper.
  String? startWallpaperPath;

  /// New acrylic veil (Denial ShellBackdropBlur) — not the removed GLES stack.
  bool acrylicEnabled;

  /// First-run OOBE completed (SS3).
  bool oobeCompleted;

  /// L6: user has seen the notification-access prompt.
  bool notificationAccessPrompted;
  final List<String> pins;
  final List<TileGroup> groups;
  final List<StartTile> tiles;

  factory ShellPreferences.fromJson(
    Map<String, dynamic> json, {
    Map<String, dynamic>? legacyTiles,
  }) {
    final board = legacyTiles ?? json;
    final groups = <TileGroup>[
      for (final entry in (board['groups'] as List? ?? []))
        TileGroup(_int(entry['id'], 1), entry['title'] as String? ?? '开始'),
    ];
    if (groups.isEmpty) groups.add(const TileGroup(1, '开始'));
    final tiles = <StartTile>[
      for (final entry in (board['tiles'] as List? ?? []))
        StartTile.fromJson(Map<String, dynamic>.from(entry as Map)),
    ];
    // Retain old/temporarily unavailable applications and orphan groups.
    for (final tile in tiles) {
      if (!groups.any((group) => group.id == tile.groupId)) {
        groups.add(TileGroup(tile.groupId, '组 ${tile.groupId}'));
      }
    }
    final start = _choice(json['start_tile_animation'], startAnimationNames);
    final launch = json['app_launch_animation'] == null
        ? [0, 2, 4, 3, 4][start]
        : _choice(json['app_launch_animation'], launchAnimationNames);
    final accent = _int(
      json['phone_accent_index'],
      0,
    ).clamp(0, phoneAccentArgb.length - 1);
    return ShellPreferences(
      mode: ShellMode.values[_choice(json['shell_mode'], shellModeNames)],
      phoneMoreTiles: json['phone_more_tiles'] == true,
      phoneThemeDark: json['phone_theme_dark'] != false,
      phoneTaskButton: json['phone_task_button'] == true,
      autoScreenOff: json['auto_screen_off'] != false,
      screenOffSeconds: ((json['screen_off_seconds'] as num?)?.toInt() ?? 900)
          .clamp(15, 86400),
      screenOffCommand: json['screen_off_command'] as String? ?? '',
      screenOnCommand: json['screen_on_command'] as String? ?? '',
      screenHooksRoot: json['screen_hooks_root'] == true,
      desktopWallpaperPath: json['desktop_wallpaper_path'] as String?,
      phoneAccentIndex: accent,
      autoAccentFromIcon: json['auto_accent_from_icon'] != false,
      layout:
          StartLayout.values[_choice(json['start_layout_style'], layoutNames)],
      startAnimation: StartAnimation.values[start],
      launchAnimation: LaunchAnimation.values[launch],
      switchAnimation: json['app_switch_animation_enabled'] != false,
      taskbarHeight: (json['taskbar_height'] as num? ?? 56).toDouble().clamp(
        40,
        56,
      ),
      drawerFontSize:
          UiFontSize.values[_choice(json['drawer_font_size'], uiFontSizeNames)],
      tileFontSize:
          UiFontSize.values[_choice(json['tile_font_size'], uiFontSizeNames)],
      startWallpaperPath: json['start_wallpaper_path'] as String?,
      acrylicEnabled: json['acrylic_enabled'] != false,
      oobeCompleted: json['oobe_completed'] == true,
      notificationAccessPrompted: json['notification_access_prompted'] == true,
      pins: List<String>.from(json['pinned_apps'] as List? ?? []),
      groups: groups,
      tiles: tiles,
    );
  }

  Map<String, Object> toJson() => {
    'version': 1,
    'shell_mode': shellModeNames[mode.index],
    'phone_more_tiles': phoneMoreTiles,
    'phone_theme_dark': phoneThemeDark,
    'phone_task_button': phoneTaskButton,
    'auto_screen_off': autoScreenOff,
    'screen_off_seconds': screenOffSeconds,
    'screen_off_command': screenOffCommand,
    'screen_on_command': screenOnCommand,
    'screen_hooks_root': screenHooksRoot,
    'desktop_wallpaper_path': ?desktopWallpaperPath,
    'phone_accent_index': phoneAccentIndex,
    'auto_accent_from_icon': autoAccentFromIcon,
    'start_layout_style': layoutNames[layout.index],
    'start_tile_animation': startAnimationNames[startAnimation.index],
    'app_launch_animation': launchAnimationNames[launchAnimation.index],
    'app_switch_animation_enabled': switchAnimation,
    'taskbar_height': taskbarHeight,
    'drawer_font_size': uiFontSizeNames[drawerFontSize.index],
    'tile_font_size': uiFontSizeNames[tileFontSize.index],
    'start_wallpaper_path': ?startWallpaperPath,
    'acrylic_enabled': acrylicEnabled,
    'oobe_completed': oobeCompleted,
    'notification_access_prompted': notificationAccessPrompted,
    'pinned_apps': pins,
    'groups': groups.map((group) => group.toJson()).toList(),
    'tiles': tiles.map((tile) => tile.toJson()).toList(),
  };
}

class TileRect {
  const TileRect(this.x, this.y, this.width, this.height);
  final double x, y, width, height;
  double get right => x + width;
  double get bottom => y + height;
  bool overlaps(TileRect other) =>
      x < other.right &&
      right > other.x &&
      y < other.bottom &&
      bottom > other.y;
}

class TilePlacement {
  const TilePlacement(this.tile, this.rect, this.groupIndex);
  final StartTile tile;
  final TileRect rect;
  final int groupIndex;
}

/// Windows 8 derives geometry from list order; other layouts retain free cells.
List<TilePlacement> layoutTiles(
  ShellPreferences prefs,
  double width,
  double height,
) {
  final result = <TilePlacement>[];
  var groupX = 0.0;
  var groupY = prefs.isPhone ? 0.0 : 38.0;
  final columns = tileGridColumns(prefs, width);
  final cell = tileGridCell(prefs, width);
  final groupWidth = columns * cell;
  final lanes = prefs.isTablet && prefs.effectiveLayout == StartLayout.windows10
      ? math.max(1, ((width + 32) / (groupWidth + 32)).floor())
      : 1;
  var laneBottom = groupY;
  for (var groupIndex = 0; groupIndex < prefs.groups.length; groupIndex++) {
    final group = prefs.groups[groupIndex];
    final tiles = prefs.tiles.where((tile) => tile.groupId == group.id);
    if (prefs.effectiveLayout == StartLayout.windows8) {
      final rows = ((height - 200) / 130).floor().clamp(1, 6);
      final occupied = <(int, int)>{};
      var maxCol = 0;
      for (final tile in tiles) {
        final wide = tile.width > tile.height;
        final span = wide ? 2 : 1;
        var col = 0;
        var row = 0;
        while (true) {
          if (!occupied.contains((col, row)) &&
              (span == 1 || !occupied.contains((col + 1, row)))) {
            break;
          }
          row++;
          if (row == rows) {
            row = 0;
            col++;
          }
        }
        for (var offset = 0; offset < span; offset++) {
          occupied.add((col + offset, row));
        }
        maxCol = math.max(maxCol, col + span);
        result.add(
          TilePlacement(
            tile,
            TileRect(groupX + col * 130, 34 + row * 130, wide ? 250 : 120, 120),
            groupIndex,
          ),
        );
      }
      groupX += math.max(120, maxCol * 130 - 10) + 60;
    } else {
      groupX = (groupIndex % lanes) * (groupWidth + 32);
      var bottom = groupY;
      final used = <TileRect>[];
      for (final tile in tiles) {
        final span = math.min(columns, tile.width);
        var col = tile.col;
        var row = tile.row;
        if (col + span > columns) {
          col = 0;
          row += 2;
        }
        var rect = TileRect(
          groupX + col * cell,
          groupY + row * cell,
          span * cell - 6,
          tile.height * cell - 6,
        );
        // Responsive projection never rewrites the stored freeform board.
        while (used.any(rect.overlaps)) {
          col++;
          if (col + span > columns) {
            col = 0;
            row++;
          }
          rect = TileRect(
            groupX + col * cell,
            groupY + row * cell,
            span * cell - 6,
            tile.height * cell - 6,
          );
        }
        used.add(rect);
        result.add(TilePlacement(tile, rect, groupIndex));
        bottom = math.max(bottom, rect.bottom);
      }
      laneBottom = math.max(laneBottom, bottom);
      if (groupIndex % lanes == lanes - 1) {
        groupY = laneBottom + (prefs.isPhone ? 8 : 58);
      }
    }
  }
  return result;
}

/// Metro reordering is recomputed from the immutable drag-start list.
List<StartTile> previewMetroDrop(
  List<StartTile> original,
  String appId,
  int groupId,
  int insertionIndex,
) {
  final dragged = original.firstWhere((tile) => tile.appId == appId);
  final remaining = original.where((tile) => tile.appId != appId).toList();
  final siblings = remaining.where((tile) => tile.groupId == groupId).toList();
  final index = insertionIndex.clamp(0, siblings.length);
  final globalIndex = index < siblings.length
      ? remaining.indexOf(siblings[index])
      : siblings.isNotEmpty
      ? remaining.indexOf(siblings.last) + 1
      : remaining.length;
  remaining.insert(globalIndex, dragged.copyWith(groupId: groupId));
  return remaining;
}

bool _cellsOverlap(StartTile a, StartTile b) =>
    a.groupId == b.groupId &&
    a.col < b.col + b.width &&
    a.col + a.width > b.col &&
    a.row < b.row + b.height &&
    a.row + a.height > b.row;

/// After a menu resize, keep the tile in place and push overlapping neighbors.
List<StartTile> resolveFreeformResize(
  List<StartTile> tiles,
  String appId,
  int maxColumns,
) {
  final tile = tiles.firstWhere((item) => item.appId == appId);
  return previewFreeformDrop(
    tiles,
    appId,
    tile.groupId,
    tile.col,
    tile.row,
    maxColumns,
  );
}

/// Freeform corner-handle sizing. Matches Rust `size_from_pointer`.
(int, int) sizeFromPointer({
  required StartTile tile,
  required double boardX,
  required double boardY,
  required double originX,
  required double originY,
  required double cell,
}) {
  final endCol = ((boardX - originX) / cell).ceil();
  final endRow = ((boardY - originY) / cell).ceil();
  final width = (endCol - tile.col).clamp(1, 8);
  final height = (endRow - tile.row).clamp(1, 8);
  return (width.toInt(), height.toInt());
}

/// Windows 8 Metro handle: medium 2×2 vs wide 4×2 from the source face.
(int, int) windows8SizeFromPointer({
  required double boardX,
  required TileRect rect,
}) {
  final wide = boardX >= rect.x + 120 * 1.5;
  return wide ? (4, 2) : (2, 2);
}

(int, int)? _nearestFreeSlot(
  List<StartTile> tiles,
  int movingIndex,
  int maxColumns,
  int maxRows,
  (int, int) preferredVacancy,
) {
  final moving = tiles[movingIndex];
  final origin = (moving.col, moving.row);
  final occupiedRight = tiles.fold(
    0,
    (value, tile) => math.max(value, tile.col + tile.width),
  );
  final occupiedBottom = tiles.fold(
    0,
    (value, tile) => math.max(value, tile.row + tile.height),
  );
  final searchCols = math.max(
    maxColumns,
    math.max(occupiedRight + 8, moving.width),
  );
  final searchRows = math.max(
    maxRows,
    math.max(occupiedBottom + 8, moving.height),
  );
  final lastCol = math.max(0, searchCols - moving.width);
  final lastRow = math.max(0, searchRows - moving.height);
  (int, int, int, int, int)? bestKey;
  (int, int)? bestPos;
  for (var row = 0; row <= lastRow; row++) {
    for (var col = 0; col <= lastCol; col++) {
      final candidate = moving.copyWith(col: col, row: row);
      var blocked = false;
      for (var i = 0; i < tiles.length; i++) {
        if (i == movingIndex) continue;
        if (_cellsOverlap(candidate, tiles[i])) {
          blocked = true;
          break;
        }
      }
      if (blocked) continue;
      final distance = (col - origin.$1).abs() + (row - origin.$2).abs();
      final vacancy =
          (col - preferredVacancy.$1).abs() + (row - preferredVacancy.$2).abs();
      final key = (distance, vacancy, (row - origin.$2).abs(), row, col);
      if (bestKey == null ||
          key.$1 < bestKey.$1 ||
          (key.$1 == bestKey.$1 && key.$2 < bestKey.$2) ||
          (key.$1 == bestKey.$1 &&
              key.$2 == bestKey.$2 &&
              key.$3 < bestKey.$3) ||
          (key.$1 == bestKey.$1 &&
              key.$2 == bestKey.$2 &&
              key.$3 == bestKey.$3 &&
              key.$4 < bestKey.$4) ||
          (key.$1 == bestKey.$1 &&
              key.$2 == bestKey.$2 &&
              key.$3 == bestKey.$3 &&
              key.$4 == bestKey.$4 &&
              key.$5 < bestKey.$5)) {
        bestKey = key;
        bestPos = (col, row);
      }
    }
  }
  return bestPos;
}

/// Reversible collision preview. Unaffected neighbors keep their original cells.
List<StartTile> previewFreeformDrop(
  List<StartTile> original,
  String appId,
  int groupId,
  int col,
  int row,
  int maxColumns,
) {
  final grabbedIndex = original.indexWhere((tile) => tile.appId == appId);
  if (grabbedIndex < 0) return List.of(original);
  final grabbed = original[grabbedIndex];
  final preferredVacancy = (grabbed.col, grabbed.row);
  final result = [
    for (final tile in original)
      tile.appId == appId
          ? grabbed.copyWith(
              groupId: groupId,
              col: col.clamp(0, math.max(0, maxColumns - grabbed.width)),
              row: math.max(0, row),
            )
          : tile,
  ];
  final dragged = result[grabbedIndex];
  final conflicts =
      [
        for (var i = 0; i < result.length; i++)
          if (i != grabbedIndex && _cellsOverlap(result[i], dragged)) i,
      ]..sort((a, b) {
        final da =
            (result[a].row - dragged.row).abs() +
            (result[a].col - dragged.col).abs();
        final db =
            (result[b].row - dragged.row).abs() +
            (result[b].col - dragged.col).abs();
        if (da != db) return da.compareTo(db);
        if (result[a].row != result[b].row) {
          return result[a].row.compareTo(result[b].row);
        }
        return result[a].col.compareTo(result[b].col);
      });
  for (final index in conflicts) {
    final slot = _nearestFreeSlot(
      result,
      index,
      maxColumns,
      64,
      preferredVacancy,
    );
    if (slot == null) continue;
    result[index] = result[index].copyWith(col: slot.$1, row: slot.$2);
  }
  return result;
}

/// Every point selects a group, including the gap or area below its last tile.
int nearestTileGroup(
  List<TilePlacement> placements,
  List<TileGroup> groups,
  double x,
  double y,
) {
  var bestId = groups.first.id;
  var bestDistance = double.infinity;
  for (final group in groups) {
    final members = placements
        .where((tile) => tile.tile.groupId == group.id)
        .toList();
    if (members.isEmpty) continue;
    final left = members.map((tile) => tile.rect.x).reduce(math.min);
    final top = members.map((tile) => tile.rect.y).reduce(math.min);
    final right = members.map((tile) => tile.rect.right).reduce(math.max);
    final bottom = members.map((tile) => tile.rect.bottom).reduce(math.max);
    final dx = x < left
        ? left - x
        : x > right
        ? x - right
        : 0.0;
    final dy = y < top
        ? top - y
        : y > bottom
        ? y - bottom
        : 0.0;
    final distance = dx * dx + dy * dy;
    if (distance < bestDistance) {
      bestDistance = distance;
      bestId = group.id;
    }
  }
  return bestId;
}

/// Shared mode geometry is consumed by drawing and native hit testing.
TileRect launcherGeometry(ShellPreferences prefs, double width, double height) {
  if (prefs.mode != ShellMode.desktop) return TileRect(0, 0, width, height);
  final available = math.max(0.0, height - prefs.bottomInset);
  final panelHeight = math.min(640.0, available);
  return TileRect(
    0,
    available - panelHeight,
    math.min(900.0, width),
    panelHeight,
  );
}

TileRect actionCenterGeometry(
  ShellPreferences prefs,
  double width,
  double height,
) {
  final panelWidth = prefs.isPhone ? width : math.min(380.0, width);
  return TileRect(
    width - panelWidth,
    prefs.topInset,
    panelWidth,
    math.max(0.0, height - prefs.topInset - prefs.bottomInset),
  );
}

String catalogSection(String name) {
  final text = name.trimLeft();
  final first = text.isEmpty ? '#' : text.substring(0, 1).toUpperCase();
  return RegExp(r'^[A-Z]$').hasMatch(first) ? first : '#';
}

int tileGridColumns(ShellPreferences prefs, double width) => prefs.isPhone
    ? (prefs.phoneMoreTiles ? 6 : 4)
    : math.max(2, (math.min(372, width) / 62).floor());
double tileGridCell(ShellPreferences prefs, double width) => prefs.isPhone
    ? width / tileGridColumns(prefs, width)
    : math.min(62.0, width / tileGridColumns(prefs, width));

/// Reset Start to MetroLauncher-style default seed: one「开始」group and
/// medium tiles from [pins] packed top-to-left (empty board when no pins).
void resetStartBoard(ShellPreferences prefs, {int columns = 4}) {
  prefs.groups
    ..clear()
    ..add(const TileGroup(1, '开始'));
  final seeded = <StartTile>[
    for (var index = 0; index < prefs.pins.length; index++)
      StartTile(
        appId: prefs.pins[index],
        groupId: 1,
        col: (index * 2) % math.max(2, columns),
        row: (index * 2) ~/ math.max(2, columns) * 2,
        width: 2,
        height: 2,
      ),
  ];
  prefs.tiles
    ..clear()
    ..addAll(packTilesTopLeft(seeded, columns));
}

/// MetroLauncher `GridPacker.autoLayout`: fill gaps top-to-left per group.
/// Preserves relative order (row, then col) while eliminating holes.
List<StartTile> packTilesTopLeft(List<StartTile> tiles, int columns) {
  final cols = math.max(1, columns);
  final byGroup = <int, List<StartTile>>{};
  for (final tile in tiles) {
    byGroup.putIfAbsent(tile.groupId, () => []).add(tile);
  }
  final packed = <StartTile>[];
  for (final groupTiles in byGroup.values) {
    final sorted = [...groupTiles]
      ..sort((a, b) {
        final byRow = a.row.compareTo(b.row);
        return byRow != 0 ? byRow : a.col.compareTo(b.col);
      });
    final placed = <StartTile>[];
    for (final tile in sorted) {
      final span = math.min(cols, tile.width);
      final height = tile.height;
      final slot = _firstFreeSlot(placed, cols, span, height);
      placed.add(tile.copyWith(col: slot.$1, row: slot.$2, width: span));
    }
    packed.addAll(placed);
  }
  return packed;
}

(int, int) _firstFreeSlot(
  List<StartTile> placed,
  int cols,
  int span,
  int height,
) {
  for (var row = 0; row < 1000; row++) {
    for (var col = 0; col <= cols - span; col++) {
      if (_fits(placed, row, col, span, height)) return (col, row);
    }
  }
  return (0, 0);
}

bool _fits(List<StartTile> placed, int row, int col, int span, int height) {
  for (final tile in placed) {
    if (col + span <= tile.col ||
        tile.col + tile.width <= col ||
        row + height <= tile.row ||
        tile.row + tile.height <= row) {
      continue;
    }
    return false;
  }
  return true;
}
