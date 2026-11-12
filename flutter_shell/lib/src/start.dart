// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';
import 'dart:io';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/launcher.dart';
import 'package:denial_flutter_sdk/popups.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:flutter/material.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'fluent.dart';
import 'controls.dart';
import 'disco_turnstile.dart';
import 'fluent_motion.dart';
import 'fluent_controls.dart';
import 'model.dart';
import 'launch.dart';
import 'preferences.dart';
import 'user_wallpaper.dart';
import 'start_motion.dart';
import 'screen_motion.dart';
import 'shell_navigation.dart';
import 'symbols.dart';
import 'mode_settings.dart';
import 'acrylic.dart';
import 'app_actions.dart';
import 'app_catalog.dart';
import 'live_tile.dart';
import 'settings_launch.dart';
import 'wp_popups.dart';
import 'wp_toast.dart';

class Win10xStart extends ConsumerStatefulWidget {
  const Win10xStart({
    super.key,
    required this.launcher,
    this.background = const UserWallpaper(),
  });
  final LauncherContext launcher;
  final Widget background;
  @override
  ConsumerState<Win10xStart> createState() => _Win10xStartState();
}

class _Win10xStartState extends ConsumerState<Win10xStart>
    with TickerProviderStateMixin {
  final _preferences = PreferenceController.instance;
  final _search = TextEditingController();
  late final AnimationController _entrance;
  late final AnimationController _page;
  late final AnimationController _launch;
  late final AnimationController _wobble;
  String? _launchId;
  bool _launchFromApps = false;
  List<StartTile>? _dragStart;
  ShellPreferences? _dragSnapshot;
  String? _dragId;
  final _boardKey = GlobalKey();
  final _sceneKey = GlobalKey();
  List<TilePlacement> _placements = [];
  Rect? _launchRect;
  Offset _grab = Offset.zero;
  Offset? _floatBoardPos;
  Size? _floatSize;
  Offset? _settleFrom;
  Offset? _settleTo;
  bool _settling = false;
  double _layoutWidth = 0, _layoutHeight = 0;
  int? _renamingGroup;
  final _groupName = TextEditingController();
  Object? _menuHandle;
  Offset? _menuAnchor;
  bool _appsVisible = false;
  bool _searchVisible = false;
  bool _railExpanded = false, _phoneEditing = false, _touchPointer = false;
  bool _pageDragging = false;
  double _pageGrab = 0;
  int _navigationRevision = -1;
  final _tileScroll = ScrollController();
  late final AnimationController _settle;
  int? _pointer;
  Offset? _pointerDownGlobal;
  TilePlacement? _pendingPlacement;
  bool _pointerRouteActive = false;
  bool _suppressTap = false;
  String? _resizeId;
  bool _draggingHandle = false;

  /// Matches Rust `StartTileReveal.closing`: open and close both run elapsed
  /// forward from 0 with style-specific duration, not a single reversed scrub.
  bool _closing = false;

  static const _reflowCurve = FluentMotion.standard;
  static const _dragActivate = 8.0;
  static const _resizeHandle = 18.0;

  bool get _dragLocked => _dragId != null || _settling || _draggingHandle;

  StartAnimation get _startStyle => StartReveal.styleFor(_preferences.value);

  int get _rowSpan {
    if (_placements.isEmpty) return 0;
    final rows = _placements.map((item) => item.rect.y).toSet();
    return math.max(0, rows.length - 1);
  }

  double get _elapsed {
    final ms = _entrance.duration?.inMilliseconds ?? 1;
    return _entrance.value * ms / 1000.0;
  }

  void _playOpen() {
    _closing = false;
    _entrance.duration = StartReveal.openDuration(
      _preferences.value,
      rowSpan: math.max(1, _rowSpan),
    );
    _entrance.forward(from: 0);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _maybePromptNotificationAccess();
    });
  }

  void _playClose({required bool afterLaunch}) {
    _closing = true;
    if (afterLaunch) {
      // Launch handoff already cleared Start content; keep the veil only.
      _entrance.duration = Duration(
        milliseconds: (StartReveal.mobileAppBackgroundHold * 1000).round(),
      );
      _entrance.value = 1;
      return;
    }
    _entrance.duration = StartReveal.closeDuration(
      _preferences.value,
      rowSpan: math.max(1, _rowSpan),
    );
    _entrance.forward(from: 0);
  }

  @override
  void initState() {
    super.initState();
    ShellNavigation.instance.addListener(_navigationChanged);
    ScreenMotion.startMotion.addListener(_screenMotionChanged);
    ScreenMotion.reenter.addListener(_playOpen);
    _entrance = AnimationController(
      vsync: this,
      duration: StartReveal.openDuration(_preferences.value),
    );
    _page = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 340),
    );
    _launch = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 720),
    );
    _settle = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
    );
    _wobble = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 420),
    );
    if (widget.launcher.visible) {
      _playOpen();
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _navigationChanged(force: true);
      });
    }
  }

  @override
  void didUpdateWidget(covariant Win10xStart oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.launcher.visible && !oldWidget.launcher.visible) {
      _search.clear();
      _appsVisible = false;
      _page.value = 0;
      _launchId = null;
      _launchFromApps = false;
      _launch.reset();
      _playOpen();
      _navigationChanged(force: true);
    } else if (!widget.launcher.visible && oldWidget.launcher.visible) {
      _cancelDrag(restore: true);
      _confirmResize(persist: false);
      _playClose(afterLaunch: _launchId != null && _launch.value > 0);
    }
    if (!widget.launcher.visible) {
      _cancelDrag(restore: true);
      _confirmResize(persist: false);
    }
  }

  @override
  void dispose() {
    ShellNavigation.instance.removeListener(_navigationChanged);
    ScreenMotion.startMotion.removeListener(_screenMotionChanged);
    ScreenMotion.reenter.removeListener(_playOpen);
    _tileScroll.dispose();
    _detachPointerRoute();
    _search.dispose();
    _groupName.dispose();
    _entrance.dispose();
    _page.dispose();
    _launch.dispose();
    _settle.dispose();
    _wobble.dispose();
    super.dispose();
  }

  void _attachPointerRoute() {
    if (_pointerRouteActive) return;
    GestureBinding.instance.pointerRouter.addGlobalRoute(_handleGlobalPointer);
    _pointerRouteActive = true;
  }

  void _detachPointerRoute() {
    if (!_pointerRouteActive) return;
    GestureBinding.instance.pointerRouter.removeGlobalRoute(
      _handleGlobalPointer,
    );
    _pointerRouteActive = false;
  }

  void _clearPointerPending() {
    _pointer = null;
    _pointerDownGlobal = null;
    _pendingPlacement = null;
    _detachPointerRoute();
  }

  void _handleGlobalPointer(PointerEvent event) {
    if (_pointer == null || event.pointer != _pointer) return;
    if (event is PointerMoveEvent) {
      if (_draggingHandle && _resizeId != null) {
        _updateResize(event.position);
        return;
      }
      final down = _pointerDownGlobal;
      final pending = _pendingPlacement;
      if (_resizeId != null) return;
      if (_dragId == null && down != null && pending != null) {
        if ((event.position - down).distance < _dragActivate) return;
        if (_preferences.value.isPhone && _touchPointer && !_phoneEditing) {
          return;
        }
        _suppressTap = true;
        setState(() => _beginDrag(pending, _grab));
        _updateDrag(event.position);
        return;
      }
      if (_dragId != null && !_settling) {
        _updateDrag(event.position);
      }
      return;
    }
    if (event is PointerUpEvent || event is PointerCancelEvent) {
      final dragging = _dragId != null && !_settling;
      final resizing = _draggingHandle;
      _clearPointerPending();
      if (event is PointerCancelEvent) {
        if (dragging) setState(() => _cancelDrag(restore: true));
        if (resizing) setState(() => _draggingHandle = false);
        return;
      }
      if (resizing) {
        setState(() => _draggingHandle = false);
        return;
      }
      if (dragging) {
        _endDrag();
      }
    }
  }

  void _enterResize(StartTile tile) {
    _cancelDrag(restore: true);
    setState(() {
      _resizeId = tile.appId;
      _draggingHandle = false;
    });
  }

  void _confirmResize({bool persist = true}) {
    if (_resizeId == null && !_draggingHandle) return;
    setState(() {
      _resizeId = null;
      _draggingHandle = false;
    });
    if (persist) {
      _preferences.change((prefs) {
        if (prefs.isPhone) {
          final packed = packTilesTopLeft(
            prefs.tiles,
            tileGridColumns(prefs, _layoutWidth),
          );
          prefs.tiles
            ..clear()
            ..addAll(packed);
        }
      });
    }
  }

  void _updateResize(Offset globalPosition) {
    final appId = _resizeId;
    final box = _boardKey.currentContext?.findRenderObject() as RenderBox?;
    if (appId == null || box == null) return;
    final point = box.globalToLocal(globalPosition);
    final prefs = _preferences.value;
    final index = prefs.tiles.indexWhere((tile) => tile.appId == appId);
    if (index < 0) return;
    final tile = prefs.tiles[index];
    late final int width;
    late final int height;
    if (prefs.effectiveLayout == StartLayout.windows8) {
      TilePlacement? placement;
      for (final item in layoutTiles(prefs, _layoutWidth, _layoutHeight)) {
        if (item.tile.appId == appId) {
          placement = item;
          break;
        }
      }
      if (placement == null) return;
      final sized = windows8SizeFromPointer(
        boardX: point.dx,
        rect: placement.rect,
      );
      width = sized.$1;
      height = sized.$2;
    } else {
      final cell = tileGridCell(prefs, _layoutWidth);
      final baseline = layoutTiles(prefs, _layoutWidth, _layoutHeight);
      final groupTiles = baseline
          .where((item) => item.tile.groupId == tile.groupId)
          .toList();
      final originY = groupTiles.isEmpty
          ? 38.0
          : groupTiles
                .map((item) => item.rect.y - item.tile.row * cell)
                .reduce(math.min);
      final sized = sizeFromPointer(
        tile: tile,
        boardX: point.dx,
        boardY: point.dy,
        originX: groupTiles.isEmpty
            ? 0
            : groupTiles
                  .map((p) => p.rect.x - p.tile.col * cell)
                  .reduce(math.min),
        originY: originY,
        cell: cell,
      );
      width = sized.$1;
      height = sized.$2;
    }
    if (tile.width == width && tile.height == height) return;
    setState(() {
      final next = List.of(prefs.tiles);
      next[index] = tile.copyWith(width: width, height: height);
      if (prefs.effectiveLayout == StartLayout.windows8) {
        prefs.tiles
          ..clear()
          ..addAll(next);
      } else {
        final columns = tileGridColumns(prefs, _layoutWidth);
        prefs.tiles
          ..clear()
          ..addAll(resolveFreeformResize(next, appId, columns));
      }
    });
  }

  LauncherApplication? _app(String id) {
    for (final app in widget.launcher.applications) {
      if (normalizedAppId(app.id) == normalizedAppId(id)) return app;
    }
    return null;
  }

  void _showApps(bool visible) {
    _pageDragging = false;
    setState(() => _appsVisible = visible);
    final target = visible ? 1.0 : 0.0;
    if (!mounted || MediaQuery.disableAnimationsOf(context)) {
      _page.value = target;
      return;
    }
    // Follow the finger's current page progress, then settle with the same
    // 340 ms easeOutExpo used by the legacy Rust page push.
    _page.animateTo(
      target,
      duration: const Duration(milliseconds: 340),
      curve: const _EaseOutExpoCurve(),
    );
  }

  Future<void> _start(LauncherApplication app, {bool fromApps = false}) async {
    if (_launchId != null || _dragId != null || _settling) return;
    // The prepared host submits the native launch here without dismissing Start.
    // Loading and shell motion run concurrently, never as a pre-launch delay.
    widget.launcher.onLaunch(app.id);
    _launchId = app.id;
    _launchFromApps = fromApps || _appsVisible || _searchVisible;
    final source = _placements.where(
      (item) => normalizedAppId(item.tile.appId) == normalizedAppId(app.id),
    );
    final board = _boardKey.currentContext?.findRenderObject() as RenderBox?;
    final scene = _sceneKey.currentContext?.findRenderObject() as RenderBox?;
    if (!_launchFromApps &&
        source.isNotEmpty &&
        board != null &&
        scene != null) {
      final rect = source.first.rect;
      _launchRect = Rect.fromLTWH(
        board.localToGlobal(Offset(rect.x, rect.y), ancestor: scene).dx,
        board.localToGlobal(Offset(rect.x, rect.y), ancestor: scene).dy,
        rect.width,
        rect.height,
      );
    } else {
      _launchRect = null;
    }
    if (MediaQuery.disableAnimationsOf(context) ||
        _preferences.value.launchAnimation == LaunchAnimation.systemDefault) {
      widget.launcher.onDismiss();
      return;
    }
    _launch.duration = StartReveal.launchDuration(
      _preferences.value.launchAnimation,
      tileCount: math.max(1, _placements.length),
      fromAppList:
          _launchFromApps &&
          _preferences.value.launchAnimation == LaunchAnimation.discoTurnstile,
    );
    setState(() {});
    await _launch.forward(from: 0).orCancel.catchError((Object _) {});
    if (mounted && widget.launcher.visible) widget.launcher.onDismiss();
  }

  void _cancelDrag({bool restore = true}) {
    final original = _dragStart;
    if (original != null && restore) {
      _preferences.value.tiles
        ..clear()
        ..addAll(original);
    }
    _dragStart = null;
    _dragSnapshot = null;
    _dragId = null;
    _floatBoardPos = null;
    _floatSize = null;
    _settleFrom = null;
    _settleTo = null;
    _settling = false;
    _settle.stop();
    _settle.value = 0;
    _clearPointerPending();
  }

  void _beginDrag(TilePlacement placement, Offset localPosition) {
    if (_dragId != null || _settling || _launchId != null) return;
    final tile = placement.tile;
    // Capture the drag-start board once. Every avoidance preview is derived
    // from this snapshot so neighbors can walk back when the float leaves.
    _dragStart = List.of(_preferences.value.tiles);
    _dragId = tile.appId;
    _dragSnapshot = ShellPreferences(
      layout: _preferences.value.layout,
      mode: _preferences.value.mode,
      phoneMoreTiles: _preferences.value.phoneMoreTiles,
      groups: List.of(_preferences.value.groups),
      tiles: List.of(_dragStart!),
    );
    _grab = localPosition;
    _floatBoardPos = Offset(placement.rect.x, placement.rect.y);
    _floatSize = Size(placement.rect.width, placement.rect.height);
  }

  void _updateDrag(Offset globalPosition) {
    final snapshot = _dragSnapshot;
    final original = _dragStart;
    final appId = _dragId;
    final box = _boardKey.currentContext?.findRenderObject() as RenderBox?;
    if (snapshot == null ||
        original == null ||
        appId == null ||
        box == null ||
        _settling) {
      return;
    }
    final point = box.globalToLocal(globalPosition);
    final floatMin = point - _grab;
    final baseline = layoutTiles(snapshot, _layoutWidth, _layoutHeight);
    final group = nearestTileGroup(
      baseline,
      snapshot.groups,
      floatMin.dx + (_floatSize?.width ?? 0) / 2,
      floatMin.dy + (_floatSize?.height ?? 0) / 2,
    );
    List<StartTile> preview;
    if (snapshot.effectiveLayout == StartLayout.windows8) {
      final without = ShellPreferences(
        layout: snapshot.layout,
        groups: snapshot.groups,
        tiles: original.where((tile) => tile.appId != appId).toList(),
      );
      final slots = layoutTiles(
        without,
        _layoutWidth,
        _layoutHeight,
      ).where((item) => item.tile.groupId == group).toList();
      var insertion = slots.length;
      var best = double.infinity;
      final probe = Offset(
        floatMin.dx + (_floatSize?.width ?? 0) / 2,
        floatMin.dy + (_floatSize?.height ?? 0) / 2,
      );
      for (var i = 0; i < slots.length; i++) {
        final rect = slots[i].rect;
        final center = Offset(
          rect.x + rect.width / 2,
          rect.y + rect.height / 2,
        );
        final distance = (probe - center).distanceSquared;
        if (distance < best) {
          best = distance;
          insertion = i + (probe.dy > center.dy ? 1 : 0);
        }
      }
      preview = previewMetroDrop(original, appId, group, insertion);
    } else {
      final groupTiles = baseline
          .where((item) => item.tile.groupId == group)
          .toList();
      final columns = tileGridColumns(snapshot, _layoutWidth);
      final cell = tileGridCell(snapshot, _layoutWidth);
      final groupOriginY = groupTiles.isEmpty
          ? 38.0
          : groupTiles
                .map((item) => item.rect.y - item.tile.row * cell)
                .reduce(math.min);
      final grabbed = original.firstWhere((tile) => tile.appId == appId);
      final originX = groupTiles.isEmpty
          ? 0.0
          : groupTiles
                .map((p) => p.rect.x - p.tile.col * cell)
                .reduce(math.min);
      final col = ((floatMin.dx - originX) / cell)
          .round()
          .clamp(0, math.max(0, columns - grabbed.width))
          .toInt();
      final row = ((floatMin.dy - groupOriginY) / cell)
          .round()
          .clamp(0, 64)
          .toInt();
      preview = previewFreeformDrop(original, appId, group, col, row, columns);
    }
    setState(() {
      _floatBoardPos = floatMin;
      _preferences.value.tiles
        ..clear()
        ..addAll(preview);
    });
  }

  void _endDrag() {
    final appId = _dragId;
    if (appId == null || _settling) return;
    TilePlacement? target;
    for (final item in _placements) {
      if (item.tile.appId == appId) {
        target = item;
        break;
      }
    }
    _clearPointerPending();
    _preferences.change((prefs) {
      if (prefs.isPhone) {
        final packed = packTilesTopLeft(
          prefs.tiles,
          tileGridColumns(prefs, _layoutWidth),
        );
        prefs.tiles
          ..clear()
          ..addAll(packed);
      }
    });
    if (target == null ||
        _floatBoardPos == null ||
        MediaQuery.disableAnimationsOf(context)) {
      setState(() {
        _dragStart = null;
        _dragSnapshot = null;
        _dragId = null;
        _floatBoardPos = null;
        _floatSize = null;
      });
      return;
    }
    final slot = target;
    setState(() {
      _dragStart = null;
      _dragSnapshot = null;
      _settleFrom = _floatBoardPos;
      _settleTo = Offset(slot.rect.x, slot.rect.y);
      _floatSize = Size(slot.rect.width, slot.rect.height);
      _settling = true;
    });
    _settle.forward(from: 0).whenComplete(() {
      if (!mounted) return;
      setState(() {
        _dragId = null;
        _floatBoardPos = null;
        _floatSize = null;
        _settleFrom = null;
        _settleTo = null;
        _settling = false;
        _settle.value = 0;
      });
    });
  }

  Offset? get _floatingPos {
    if (_floatBoardPos == null) return null;
    if (!_settling || _settleFrom == null || _settleTo == null) {
      return _floatBoardPos;
    }
    return Offset.lerp(_settleFrom, _settleTo, easeOutBack(_settle.value));
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: _preferences,
    builder: (context, _) => AnimatedBuilder(
      animation: Listenable.merge([_entrance, _page, _launch, _settle]),
      builder: (context, _) {
        final prefs = _preferences.value;
        final metro = prefs.effectiveLayout == StartLayout.windows8;

        final elapsed = MediaQuery.disableAnimationsOf(context)
            ? 10.0
            : _elapsed;
        final style = _startStyle;
        final contentDuration = _closing
            ? StartReveal.closeDuration(
                    prefs,
                    rowSpan: math.max(1, _rowSpan),
                  ).inMilliseconds /
                  1000.0
            : StartReveal.openDuration(
                    prefs,
                    rowSpan: math.max(1, _rowSpan),
                  ).inMilliseconds /
                  1000.0;
        final scene = MediaQuery.disableAnimationsOf(context)
            ? 1.0
            : StartReveal.sceneProgress(
                style: style,
                elapsed: elapsed,
                closing: _closing,
                launchingApp: _launchId != null,
                contentDuration: contentDuration,
                groupCount: prefs.groups.length,
              );
        final discoListExit =
            _launchFromApps &&
            _launchId != null &&
            prefs.launchAnimation == LaunchAnimation.discoTurnstile;
        // Disco app-list exit owns opacity via rotateY; keep the list at full
        // alpha so row transforms are not double-faded.
        final listFade = discoListExit
            ? scene
            : (scene * (1 - easeDecelerate(_launch.value))).clamp(0.0, 1.0);
        return Shortcuts(
          shortcuts: const {
            SingleActivator(LogicalKeyboardKey.escape): _DismissStartIntent(),
          },
          child: Actions(
            actions: {
              _DismissStartIntent: CallbackAction<_DismissStartIntent>(
                onInvoke: (_) {
                  if (_appsVisible) {
                    _showApps(false);
                  } else {
                    widget.launcher.onDismiss();
                  }
                  return null;
                },
              ),
            },
            child: Material(
              key: _sceneKey,
              color: Colors.transparent,
              child: Stack(
                fit: StackFit.expand,
                children: [
                  if (!metro)
                    Opacity(
                      opacity: scene,
                      child: Stack(
                        fit: StackFit.expand,
                        children: [
                          // WP 8.1 Start Background: desktop wallpaper (or W4
                          // custom Start image) shows through tiles.
                          if (prefs.startWallpaperPath case final path?
                              when path.isNotEmpty)
                            Image.file(
                              File(path),
                              fit: BoxFit.cover,
                              errorBuilder: (_, error, stack) =>
                                  widget.background,
                            )
                          else
                            widget.background,
                          if (prefs.acrylicEnabled)
                            Win10xAcrylic(
                              dark: prefs.isPhone
                                  ? prefs.phoneThemeDark
                                  : prefs.mode != ShellMode.desktop,
                              accent: Color(prefs.phoneAccentArgbValue),
                              strength: prefs.isPhone ? 0.45 : 0.35,
                            )
                          else
                            ColoredBox(
                              color: prefs.isPhone
                                  ? (prefs.phoneThemeDark
                                        ? const Color(0x66000000)
                                        : const Color(0x66ffffff))
                                  : prefs.mode == ShellMode.desktop
                                  ? Win10xFluent.chrome
                                  : const Color(0xb3ffffff),
                            ),
                        ],
                      ),
                    ),
                  if (metro) const ColoredBox(color: Win10xFluent.metroStart),
                  Padding(
                    padding: EdgeInsets.only(
                      top: prefs.topInset,
                      bottom: prefs.mode == ShellMode.desktop
                          ? MediaQuery.viewInsetsOf(context).bottom
                          : prefs.bottomInset +
                                MediaQuery.viewInsetsOf(context).bottom,
                    ),
                    child: ListenableBuilder(
                      listenable: ShellNavigation.instance.reachOffset,
                      builder: (context, _) {
                        final keyboard = MediaQuery.viewInsetsOf(context)
                            .bottom;
                        final height = MediaQuery.sizeOf(context).height;
                        return Transform.translate(
                          offset: Offset(
                            0,
                            prefs.isPhone
                                ? (height - keyboard) *
                                      ShellNavigation.instance.reachOffset.value
                                : 0,
                          ),
                          child: _modeContent(context, scene, listFade),
                        );
                      },
                    ),
                  ),
                  if (_launchId != null &&
                      _launchRect != null &&
                      prefs.launchAnimation == LaunchAnimation.windows8)
                    Windows8FlyingTile(
                      source: _launchRect!,
                      output: Offset.zero & MediaQuery.sizeOf(context),
                      progress: _launch.value,
                      color: const Color(0xff0078d4),
                      face: buildMarkUiApplicationIcon(
                        context,
                        _launchId!,
                        widget.launcher.buildIcon,
                        color: Colors.white,
                      ),
                    ),
                  if (_launchId != null &&
                      _launchRect != null &&
                      prefs.launchAnimation == LaunchAnimation.factorClipReveal)
                    Positioned.fill(
                      child: FactorClipReveal(
                        source: _launchRect!,
                        output: Offset.zero & MediaQuery.sizeOf(context),
                        progress: _launch.value,
                        child: ColoredBox(
                          color: Color(prefs.phoneAccentArgbValue),
                          child: Center(
                            child: SizedBox(
                              width: 72,
                              height: 72,
                              child: _buildAppIcon(context, _launchId!),
                            ),
                          ),
                        ),
                      ),
                    ),
                ],
              ),
            ),
          ),
        );
      },
    ),
  );

  void _screenMotionChanged() {
    if (!mounted || !widget.launcher.visible) return;
    if (ScreenMotion.startMotion.value) {
      _playClose(afterLaunch: false);
      ScreenMotion.startCloseDuration = _entrance.duration;
    } else {
      _playOpen();
    }
  }

  void _navigationChanged({bool force = false}) {
    final nav = ShellNavigation.instance;
    if (!widget.launcher.visible ||
        (!force && nav.revision == _navigationRevision)) {
      return;
    }
    _navigationRevision = nav.revision;
    switch (nav.request) {
      case StartRequest.home:
      case StartRequest.reveal:
        // Second Start press while already on tiles closes Start (toggle).
        // From All Apps / search / edit, first return to the tiles page.
        // Never toggle-close on the force sync that runs when Start opens —
        // request stays `home` from the open press / default and would flash
        // the launcher closed immediately.
        if (!_appsVisible && !_searchVisible && !_phoneEditing) {
          if (!force && nav.request == StartRequest.home) {
            widget.launcher.onDismiss();
          }
          return;
        }
        _search.clear();
        _searchVisible = false;
        _phoneEditing = false;
        _wobble
          ..stop()
          ..value = 0;
        _showApps(false);
        if (_tileScroll.hasClients) {
          _tileScroll.animateTo(
            0,
            duration: FluentMotion.slow,
            curve: FluentMotion.standard,
          );
        }
      case StartRequest.apps:
        _searchVisible = false;
        _showApps(true);
      case StartRequest.search:
        _searchVisible = true;
        _showApps(true);
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted && widget.launcher.visible) {
            widget.launcher.searchFocusNode.requestFocus();
          }
        });
      case StartRequest.back:
        if (_phoneEditing) {
          setState(() => _phoneEditing = false);
        } else if (_appsVisible) {
          _search.clear();
          _searchVisible = false;
          _showApps(false);
        } else if (_preferences.value.mode == ShellMode.desktop ||
            ref.read(shellControllerProvider).openAppWindowCount > 0) {
          widget.launcher.onDismiss();
        }
    }
  }

  Color prefsColor(TilePlacement placement, Color legacy) {
    final tile = placement.tile;
    if (tile.transparent) return const Color(0x00000000);
    if (tile.colorArgb != null) return Color(tile.colorArgb!);
    if (_preferences.value.effectiveLayout == StartLayout.windows8) {
      return legacy;
    }
    if (_preferences.value.isPhone) {
      return Color(_preferences.value.phoneAccentArgbValue);
    }
    return Win10xFluent.accent;
  }

  Widget _catalog(BuildContext context, {bool columns = false}) {
    final discoList =
        _launchFromApps &&
        _launchId != null &&
        _preferences.value.launchAnimation == LaunchAnimation.discoTurnstile;
    final elapsed = discoList
        ? _launch.value *
              (_launch.duration ?? const Duration(milliseconds: 500))
                  .inMilliseconds /
              1000.0
        : 0.0;
    return AppCatalogView(
      apps: _filteredApps,
      columns: columns,
      buildIcon: _buildAppIcon,
      onLaunch: (app) => unawaited(_start(app, fromApps: true)),
      onMenu: (app, point) {
        _menuAnchor = point;
        unawaited(_appMenu(context, app));
      },
      discoListExit: discoList,
      launchingAppId: _launchId,
      launchElapsed: elapsed,
    );
  }

  Widget _searchField(BuildContext context) => SizedBox(
    height: 44,
    child: TextField(
      controller: _search,
      focusNode: widget.launcher.searchFocusNode,
      onChanged: (_) => setState(() {}),
      onSubmitted: (_) {
        if (_filteredApps.isNotEmpty) unawaited(_start(_filteredApps.first));
      },
      decoration: const InputDecoration(
        hintText: '搜索应用',
        prefixIcon: MarkUiIcon(Icons.search),
      ),
    ),
  );

  Widget _modeContent(BuildContext context, double scene, double listFade) {
    final prefs = _preferences.value;
    if (prefs.isPhone) return _phonePages(context, listFade);
    if (prefs.effectiveLayout == StartLayout.windows8 ||
        prefs.effectiveLayout == StartLayout.windowsPhone) {
      return Column(
        children: [
          Opacity(
            opacity: scene,
            child: _header(
              context,
              prefs.effectiveLayout == StartLayout.windows8,
            ),
          ),
          Expanded(child: _phonePages(context, listFade)),
        ],
      );
    }
    return Row(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        _startRail(context),
        Expanded(
          child: Column(
            children: [
              if (_searchVisible)
                Padding(
                  padding: const EdgeInsets.all(12),
                  child: _searchField(context),
                ),
              Expanded(
                child: LayoutBuilder(
                  builder: (context, c) {
                    if (_appsVisible || _searchVisible) {
                      return Opacity(
                        opacity: listFade,
                        child: _catalog(context, columns: prefs.isTablet),
                      );
                    }
                    if (prefs.mode == ShellMode.desktop) {
                      return Row(
                        children: [
                          SizedBox(
                            width: math.min(248, c.maxWidth * .42),
                            child: Opacity(
                              opacity: listFade,
                              child: _catalog(context),
                            ),
                          ),
                          Expanded(
                            child: LayoutBuilder(
                              builder: (context, area) =>
                                  _tileBoard(context, area.biggest),
                            ),
                          ),
                        ],
                      );
                    }
                    return Align(
                      alignment: Alignment.topCenter,
                      child: SizedBox(
                        width: math.min(1000, c.maxWidth),
                        child: _tileBoard(
                          context,
                          Size(math.min(1000, c.maxWidth), c.maxHeight),
                        ),
                      ),
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _startRail(BuildContext context) {
    Widget command(
      Widget icon,
      String label,
      VoidCallback? action, {
      bool selected = false,
    }) => Semantics(
      button: true,
      label: label,
      child: SizedBox(
        height: 48,
        child: Material(
          color: selected ? Win10xFluent.taskbarActive : Colors.transparent,
          child: InkWell(
            onTap: action,
            child: Row(
              children: [
                SizedBox(width: 48, child: Center(child: icon)),
                if (_railExpanded)
                  Expanded(
                    child: Text(
                      label,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
    return AnimatedContainer(
      duration: FluentMotion.normal,
      curve: FluentMotion.standard,
      width: _railExpanded ? 220 : 48,
      color: Win10xFluent.chrome,
      child: Column(
        children: [
          command(
            const ShellGlyph(ShellSymbol.menu),
            '开始',
            () => setState(() => _railExpanded = !_railExpanded),
          ),
          if (_preferences.value.isTablet) ...[
            command(
              const ShellGlyph(ShellSymbol.tiles),
              '固定的磁贴',
              () => _showApps(false),
              selected: !_appsVisible,
            ),
            command(
              const ShellGlyph(ShellSymbol.apps),
              '所有应用',
              () => _showApps(true),
              selected: _appsVisible,
            ),
          ] else
            command(
              const MarkUiIcon(Icons.folder_outlined),
              '文件资源管理器',
              widget.launcher.applications.any(
                    (app) => [
                      'dolphin',
                      'nautilus',
                      'thunar',
                      'nemo',
                    ].any((name) => app.id.toLowerCase().contains(name)),
                  )
                  ? () {
                      final app = widget.launcher.applications.firstWhere(
                        (app) => [
                          'dolphin',
                          'nautilus',
                          'thunar',
                          'nemo',
                        ].any((name) => app.id.toLowerCase().contains(name)),
                      );
                      unawaited(_start(app));
                    }
                  : null,
            ),
          const Spacer(),
          command(
            const MarkUiIcon(Icons.more_horiz),
            '模式与个性化',
            () => _settings(context),
          ),
          command(
            const MarkUiIcon(Icons.settings_outlined),
            '设置',
            ShellNavigation.instance.settings,
          ),
          command(
            const MarkUiIcon(Icons.power_settings_new),
            '电源',
            ShellNavigation.instance.power,
          ),
        ],
      ),
    );
  }

  Widget _phonePages(BuildContext context, double listFade) => LayoutBuilder(
    builder: (context, c) {
      // Page progress is linear while the finger moves (ViewPager-style).
      // Programmed settles apply easeOutExpo through animateTo's curve.
      final p = _page.value;
      final metro = _preferences.value.effectiveLayout == StartLayout.windows8;
      final paging = !_dragLocked && !_phoneEditing;
      Widget page(Widget child) =>
          SizedBox(width: c.maxWidth, height: c.maxHeight, child: child);
      void grabStart() {
        if (!paging) return;
        _page.stop();
        _pageDragging = true;
        _pageGrab = _page.value;
      }

      void grabUpdate(double deltaAlongAxis, double extent) {
        if (!_pageDragging || extent <= 0) return;
        // Negative axis motion opens the apps page (left on Phone, up on Win8).
        _pageGrab = (_pageGrab - deltaAlongAxis / extent).clamp(0.0, 1.0);
        _page.value = _pageGrab;
      }

      void grabEnd(double velocityAlongAxis) {
        if (!_pageDragging) return;
        _pageDragging = false;
        final bool open;
        if (velocityAlongAxis.abs() > 400) {
          open = velocityAlongAxis < 0;
        } else {
          open = _page.value >= 0.35;
        }
        _showApps(open);
      }

      final home = Column(
        children: [
          Expanded(child: _tileBoard(context, c.biggest)),
          Align(
            alignment: Alignment.centerRight,
            child: TextButton.icon(
              onPressed: () => _showApps(true),
              icon: const MarkUiIcon(Icons.arrow_forward, size: 18),
              label: const Text('所有应用'),
              style: TextButton.styleFrom(
                foregroundColor: _preferences.value.isPhone
                    ? Theme.of(context).colorScheme.onSurface
                    : Win10xFluent.text,
              ),
            ),
          ),
        ],
      );
      final apps = Opacity(
        opacity: listFade,
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(12, 8, 8, 8),
              child: Row(
                children: [
                  Expanded(child: _searchField(context)),
                  FluentIconButton(
                    tooltip: '模式与个性化',
                    icon: const MarkUiIcon(Icons.more_horiz),
                    onPressed: () => _settings(context),
                  ),
                ],
              ),
            ),
            Expanded(
              child: metro ? _appList(context, metro: true) : _catalog(context),
            ),
          ],
        ),
      );
      return ClipRect(
        child: GestureDetector(
          behavior: HitTestBehavior.translucent,
          // MetroLauncherAndroid: double-tap Start background → All Apps.
          onDoubleTap: paging ? () => _showApps(!_appsVisible) : null,
          onHorizontalDragStart: metro || !paging ? null : (_) => grabStart(),
          onHorizontalDragUpdate: metro || !paging
              ? null
              : (d) => grabUpdate(d.delta.dx, c.maxWidth),
          onHorizontalDragEnd: metro || !paging
              ? null
              : (d) => grabEnd(d.primaryVelocity ?? 0),
          onHorizontalDragCancel: metro || !paging
              ? null
              : () {
                  if (_pageDragging) grabEnd(0);
                },
          onVerticalDragStart: !metro || !paging ? null : (_) => grabStart(),
          onVerticalDragUpdate: !metro || !paging
              ? null
              : (d) => grabUpdate(d.delta.dy, c.maxHeight),
          onVerticalDragEnd: !metro || !paging
              ? null
              : (d) => grabEnd(d.primaryVelocity ?? 0),
          onVerticalDragCancel: !metro || !paging
              ? null
              : () {
                  if (_pageDragging) grabEnd(0);
                },
          child: Stack(
            children: [
              Positioned(
                left: metro ? 0 : -c.maxWidth * p,
                top: metro ? -c.maxHeight * p : 0,
                child: IgnorePointer(
                  ignoring: _appsVisible && !_pageDragging && p > 0.5,
                  child: ExcludeFocus(
                    excluding: _appsVisible && !_pageDragging && p > 0.5,
                    child: page(home),
                  ),
                ),
              ),
              Positioned(
                left: metro ? 0 : c.maxWidth * (1 - p),
                top: metro ? c.maxHeight * (1 - p) : 0,
                child: IgnorePointer(
                  ignoring: !(_appsVisible || _pageDragging || p > 0.5),
                  child: ExcludeFocus(
                    excluding: !(_appsVisible || _pageDragging || p > 0.5),
                    child: page(apps),
                  ),
                ),
              ),
            ],
          ),
        ),
      );
    },
  );

  Widget _header(BuildContext context, bool metro) => Padding(
    padding: const EdgeInsets.fromLTRB(24, 16, 16, 12),
    child: Row(
      children: [
        Text(
          '开始',
          style: TextStyle(
            fontSize: 28,
            fontWeight: FontWeight.w300,
            color: metro ? Colors.white : Colors.black87,
          ),
        ),
        const Spacer(),
        SizedBox(
          width: math.min(320.0, MediaQuery.sizeOf(context).width * .45),
          height: 44,
          child: TextField(
            controller: _search,
            focusNode: widget.launcher.searchFocusNode,
            onChanged: (_) {
              setState(() {});
              if (_search.text.isNotEmpty) _showApps(true);
            },
            onSubmitted: (_) {
              final matches = _filteredApps;
              if (matches.isNotEmpty) unawaited(_start(matches.first));
            },
            decoration: const InputDecoration(
              hintText: '搜索应用',
              prefixIcon: MarkUiIcon(Icons.search),
            ),
          ),
        ),
        FluentIconButton(
          tooltip: '开始设置',
          onPressed: () {
            _menuAnchor = null;
            _settings(context);
          },
          icon: MarkUiIcon(
            Icons.settings_outlined,
            color: metro ? Colors.white : Win10xFluent.text,
          ),
        ),
      ],
    ),
  );

  List<LauncherApplication> get _filteredApps {
    final query = _search.text.trim().toLowerCase();
    return widget.launcher.applications
        .where(
          (app) =>
              query.isEmpty ||
              app.searchableText.contains(query) ||
              app.name.toLowerCase().contains(query),
        )
        .toList()
      ..sort((a, b) => a.name.toLowerCase().compareTo(b.name.toLowerCase()));
  }

  Widget _buildAppIcon(BuildContext context, String id) =>
      buildMarkUiApplicationIcon(context, id, widget.launcher.buildIcon);

  Widget _appList(BuildContext context, {required bool metro}) {
    final apps = _filteredApps;
    final color = metro ? Colors.white : Colors.black87;
    Widget appItem(BuildContext context, int index) {
      final app = apps[index];
      return GestureDetector(
        onSecondaryTapDown: (details) {
          _menuAnchor = details.globalPosition;
          unawaited(_appMenu(context, app));
        },
        onLongPressStart: (details) {
          _menuAnchor = details.globalPosition;
          unawaited(_appMenu(context, app));
        },
        child: ListTile(
          key: ValueKey('list.${app.id}'),
          minVerticalPadding: 8,
          leading: SizedBox(
            width: 32,
            height: 32,
            child: buildMarkUiApplicationIcon(
              context,
              app.id,
              widget.launcher.buildIcon,
              color: Colors.white,
            ),
          ),
          title: Text(
            app.name,
            style: TextStyle(color: color, fontFamily: 'Segoe UI'),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          onTap: () => unawaited(_start(app)),
        ),
      );
    }

    if (metro) {
      return LayoutBuilder(
        builder: (context, constraints) => GridView.builder(
          scrollDirection: Axis.horizontal,
          padding: const EdgeInsets.fromLTRB(100, 50, 100, 60),
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: ((constraints.maxHeight - 110) / 56).floor().clamp(
              1,
              12,
            ),
            mainAxisExtent: 290,
          ),
          itemCount: apps.length,
          itemBuilder: appItem,
        ),
      );
    }
    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      itemCount: apps.length,
      itemBuilder: appItem,
    );
  }

  Widget _tileBoard(BuildContext context, Size size) {
    final prefs = _preferences.value;
    final metro = prefs.effectiveLayout == StartLayout.windows8;
    final horizontal = metro;
    final padding = metro
        ? const EdgeInsets.fromLTRB(100, 50, 100, 60)
        : _preferences.value.isPhone
        ? const EdgeInsets.fromLTRB(10, 6, 10, 8)
        : const EdgeInsets.fromLTRB(20, 12, 20, 20);
    final boardWidth = math.max(100.0, size.width - padding.horizontal);
    _layoutWidth = boardWidth;
    _layoutHeight = size.height;
    final placements = layoutTiles(prefs, boardWidth, size.height);
    _placements = placements;
    final extentX = placements.fold(
      boardWidth,
      (value, item) => math.max(value, item.rect.right),
    );
    final extentY =
        placements.fold(
          100.0,
          (value, item) => math.max(value, item.rect.bottom),
        ) +
        32;
    final floating = _floatingPos;
    return GestureDetector(
      behavior: HitTestBehavior.translucent,
      onTap: () {
        if (_resizeId != null && !_draggingHandle) {
          _confirmResize();
        }
      },
      child: SingleChildScrollView(
        controller: _tileScroll,
        scrollDirection: horizontal ? Axis.horizontal : Axis.vertical,
        physics: _dragLocked ? const NeverScrollableScrollPhysics() : null,
        child: Padding(
          padding: padding,
          child: SizedBox(
            key: _boardKey,
            width: horizontal ? extentX : boardWidth,
            height: horizontal
                ? math.max(150, size.height - padding.vertical)
                : extentY,
            child: Stack(
              clipBehavior: Clip.none,
              children: [
                for (var group = 0; group < prefs.groups.length; group++)
                  ..._groupChrome(
                    context,
                    placements
                        .where((item) => item.groupIndex == group)
                        .toList(),
                    group,
                    metro,
                  ),
                // Drag/settle always uses one flat stack so avoidance can animate
                // across rows/groups. W10M open/close is also flat per-tile so
                // the same keys stay mounted when a drag begins mid-board.
                if (_dragLocked || _resizeId != null)
                  ..._interactiveTiles(context, placements)
                else if (_startStyle == StartAnimation.windows10Mobile &&
                    !(_launchId != null &&
                        (_preferences.value.launchAnimation ==
                                LaunchAnimation.discoTurnstile ||
                            _preferences.value.launchAnimation ==
                                LaunchAnimation.factorClipReveal)))
                  ..._windows10MobileTiles(context, placements)
                else
                  for (var group = 0; group < prefs.groups.length; group++)
                    ..._groupTiles(
                      context,
                      placements
                          .where((item) => item.groupIndex == group)
                          .toList(),
                      group,
                      metro,
                    ),
                ..._resizeChrome(placements),
                if (floating != null && _floatSize != null && _dragId != null)
                  Positioned(
                    left: floating.dx,
                    top: floating.dy,
                    width: _floatSize!.width,
                    height: _floatSize!.height,
                    child: IgnorePointer(
                      child: Material(
                        elevation: 12,
                        color: Colors.transparent,
                        child: _tileFace(
                          context,
                          TilePlacement(
                            StartTile(appId: _dragId!),
                            TileRect(
                              floating.dx,
                              floating.dy,
                              _floatSize!.width,
                              _floatSize!.height,
                            ),
                            0,
                          ),
                        ),
                      ),
                    ),
                  ),
                if (prefs.tiles.isEmpty)
                  Positioned(
                    left: 0,
                    right: 0,
                    top: 60,
                    child: Center(
                      child: Text(
                        '长按应用，将它固定到开始屏幕',
                        style: TextStyle(
                          color: metro
                              ? Colors.white70
                              : Win10xFluent.textSecondary,
                        ),
                      ),
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  List<Widget> _groupChrome(
    BuildContext context,
    List<TilePlacement> tiles,
    int index,
    bool metro,
  ) {
    if (tiles.isEmpty || _preferences.value.isPhone) return [];
    final prefs = _preferences.value;
    final group = prefs.groups[index];
    final left = tiles.map((item) => item.rect.x).reduce(math.min);
    final top = tiles.map((item) => item.rect.y).reduce(math.min);
    final right = tiles.map((item) => item.rect.right).reduce(math.max);
    final elapsed = MediaQuery.disableAnimationsOf(context) ? 10.0 : _elapsed;
    final titleOpacity = _startStyle == StartAnimation.flutterMetroGroup
        ? StartReveal.flutterMetroGroupProgress(
            elapsed: elapsed,
            closing: _closing,
            groupIndex: index,
          )
        : StartReveal.sceneProgress(
            style: _startStyle,
            elapsed: elapsed,
            closing: _closing,
            launchingApp: _launchId != null,
            contentDuration: StartReveal.mobileTotalDuration,
            groupCount: prefs.groups.length,
          );
    return [
      Positioned(
        left: left,
        top: math.max(0, top - 34),
        width: math.max(140, right - left),
        height: 32,
        child: Opacity(
          opacity: (titleOpacity * (1 - easeDecelerate(_launch.value))).clamp(
            0.0,
            1.0,
          ),
          child: _renamingGroup == group.id
              ? Focus(
                  onFocusChange: (focused) {
                    if (!focused && _renamingGroup == group.id) {
                      _commitGroupName(group.id);
                    }
                  },
                  onKeyEvent: (_, event) {
                    if (event is KeyDownEvent &&
                        event.logicalKey == LogicalKeyboardKey.escape) {
                      setState(() => _renamingGroup = null);
                      return KeyEventResult.handled;
                    }
                    return KeyEventResult.ignored;
                  },
                  child: TextField(
                    controller: _groupName,
                    autofocus: true,
                    style: TextStyle(
                      color: metro ? Colors.white : Colors.black87,
                    ),
                    decoration: const InputDecoration(
                      isDense: true,
                      border: UnderlineInputBorder(),
                    ),
                    onSubmitted: (_) => _commitGroupName(group.id),
                  ),
                )
              : TextButton(
                  style: TextButton.styleFrom(
                    alignment: Alignment.centerLeft,
                    padding: EdgeInsets.zero,
                  ),
                  onPressed: () => setState(() {
                    _renamingGroup = group.id;
                    _groupName.text = group.title;
                  }),
                  child: Text(
                    group.title,
                    style: TextStyle(
                      color: metro ? Colors.white70 : Colors.black87,
                    ),
                  ),
                ),
        ),
      ),
    ];
  }

  /// Settled/drag board: one AnimatedPositioned per tile so neighbors can ease
  /// across row and group boundaries without remounting.
  List<Widget> _interactiveTiles(
    BuildContext context,
    List<TilePlacement> placements,
  ) {
    return [
      for (final item in placements)
        AnimatedPositioned(
          key: ValueKey('position.${item.tile.appId}'),
          duration: const Duration(milliseconds: 280),
          curve: _reflowCurve,
          left: item.rect.x,
          top: item.rect.y,
          width: item.rect.width,
          height: item.rect.height,
          child: Opacity(
            opacity: _dragId == item.tile.appId ? 0 : 1,
            child: _tile(context, item),
          ),
        ),
    ];
  }

  /// Windows 10 Mobile: each tile keeps a stable board-space slot and scales
  /// about the shared row origin (`area.center().x`, row top), matching Rust
  /// `mobile_stack_rect` without grouping tiles under per-row parents.
  List<Widget> _windows10MobileTiles(
    BuildContext context,
    List<TilePlacement> placements,
  ) {
    if (placements.isEmpty) return const [];
    final elapsed = MediaQuery.disableAnimationsOf(context) ? 10.0 : _elapsed;
    final rowYs = placements.map((item) => item.rect.y).toSet().toList()
      ..sort();
    final rowSpan = math.max(0, rowYs.length - 1);
    final boardCenterX = _layoutWidth / 2;
    final launching = _launchId != null;
    final lp = _launch.value;
    return [
      for (final item in placements)
        Builder(
          builder: (context) {
            final row = rowYs.indexOf(item.rect.y);
            final delay = StartReveal.mobileRowDelay(
              rowIndex: math.max(0, row),
              rowSpan: rowSpan,
              closing: _closing && !launching,
            );
            final (visible, progress) = StartReveal.animationProgress(
              local: launching
                  ? lp * StartReveal.windows8LaunchDuration - delay
                  : elapsed - delay,
              duration: StartReveal.mobileDuration,
              closing: launching ? true : _closing,
            );
            if (!visible && !launching) return const SizedBox.shrink();
            final rowTiles = placements
                .where((other) => other.rect.y == item.rect.y)
                .toList();
            final stride = rowTiles
                .map((other) => other.rect.height)
                .fold(70.0, math.max);
            final scaleDelta = 1 - StartReveal.mobileScaleFrom;
            final scale = launching
                ? 1 + scaleDelta * (1 - progress)
                : StartReveal.mobileScaleFrom + scaleDelta * progress;
            final travel =
                -stride * StartReveal.mobileRowTravel * (1 - progress);
            final selected =
                launching &&
                normalizedAppId(item.tile.appId) == normalizedAppId(_launchId!);
            final selectedFade = selected
                ? (1 -
                      ((lp * StartReveal.windows8LaunchDuration -
                                  delay -
                                  StartReveal.mobileSelectedDelay) /
                              StartReveal.mobileDuration)
                          .clamp(0.0, 1.0))
                : 1.0;
            return AnimatedPositioned(
              key: ValueKey('position.${item.tile.appId}'),
              duration: const Duration(milliseconds: 280),
              curve: _reflowCurve,
              left: item.rect.x,
              top: item.rect.y + travel,
              width: item.rect.width,
              height: item.rect.height,
              child: Opacity(
                opacity: launching
                    ? (progress * selectedFade).clamp(0.0, 1.0)
                    : (0.08 + 0.92 * progress),
                child: Transform.scale(
                  scale: scale,
                  alignment: Alignment(
                    ((boardCenterX - item.rect.x) /
                                math.max(1, item.rect.width)) *
                            2 -
                        1,
                    -1,
                  ),
                  child: _tile(context, item),
                ),
              ),
            );
          },
        ),
    ];
  }

  List<Widget> _groupTiles(
    BuildContext context,
    List<TilePlacement> tiles,
    int index,
    bool metro,
  ) {
    if (tiles.isEmpty) return [];
    final prefs = _preferences.value;
    final left = tiles.map((item) => item.rect.x).reduce(math.min);
    final top = tiles.map((item) => item.rect.y).reduce(math.min);
    final right = tiles.map((item) => item.rect.right).reduce(math.max);
    final bottom = tiles.map((item) => item.rect.bottom).reduce(math.max);
    final discoLaunch =
        _launchId != null &&
        prefs.launchAnimation == LaunchAnimation.discoTurnstile;
    final elapsed = MediaQuery.disableAnimationsOf(context)
        ? 10.0
        : discoLaunch
        ? _launch.value *
              (StartReveal.discoStaggerSpan + StartReveal.discoOutDuration)
        : _elapsed;
    final style = discoLaunch ? StartAnimation.discoTurnstile : _startStyle;
    final closing = discoLaunch || _closing;
    final count = math.max(1, tiles.length);
    Widget groupContent = Stack(
      clipBehavior: Clip.none,
      children: [
        for (var i = 0; i < tiles.length; i++)
          Builder(
            builder: (context) {
              final item = tiles[i];
              final reverseRank = count - 1 - i;
              final selected =
                  discoLaunch &&
                  normalizedAppId(item.tile.appId) ==
                      normalizedAppId(_launchId!);
              final delay = switch (style) {
                StartAnimation.discoTurnstile =>
                  (closing ? reverseRank : i) /
                          math.max(1, count - 1) *
                          StartReveal.discoStaggerSpan +
                      (selected ? StartReveal.mobileSelectedDelay : 0),
                StartAnimation.metroRise => i * StartReveal.metroRowStagger,
                StartAnimation.factorSlideScale =>
                  i * StartReveal.factorItemStagger,
                _ => 0.0,
              };
              final duration = switch (style) {
                StartAnimation.discoTurnstile =>
                  closing
                      ? StartReveal.discoOutDuration
                      : StartReveal.discoOuterInDuration,
                StartAnimation.metroRise => StartReveal.metroDuration,
                StartAnimation.factorSlideScale => StartReveal.factorDuration,
                _ => StartReveal.flutterMetroGroupDuration,
              };
              final (
                visible,
                progress,
              ) = style == StartAnimation.flutterMetroGroup
                  ? (
                      true,
                      StartReveal.flutterMetroGroupProgress(
                        elapsed: elapsed,
                        closing: closing,
                        groupIndex: index,
                      ),
                    )
                  : StartReveal.animationProgress(
                      local: elapsed - delay,
                      duration: duration,
                      closing: closing,
                    );
              if (!visible) return const SizedBox.shrink();
              final hidden = _dragId == item.tile.appId;
              Widget face = AnimatedPositioned(
                key: ValueKey('position.${item.tile.appId}'),
                duration: const Duration(milliseconds: 280),
                curve: _reflowCurve,
                left: item.rect.x - left,
                top: item.rect.y - top,
                width: item.rect.width,
                height: item.rect.height,
                child: Opacity(
                  opacity: hidden ? 0 : 1,
                  child: _tile(context, item),
                ),
              );
              if (style == StartAnimation.flutterMetroGroup) {
                return face;
              }
              if (style == StartAnimation.discoTurnstile) {
                final local = elapsed - delay;
                final outerRaw = closing
                    ? (local / StartReveal.discoOutDuration).clamp(0.0, 1.0)
                    : (local / StartReveal.discoOuterInDuration).clamp(
                        0.0,
                        1.0,
                      );
                final innerRaw = closing
                    ? 1.0
                    : (local / StartReveal.discoInnerInDuration).clamp(
                        0.0,
                        1.0,
                      );
                final outer = closing
                    ? cubicBezier(outerRaw, 0.75, 0.0, 1.0, 0.0)
                    : cubicBezier(outerRaw, 0.3, 1.0, 0.2, 1.0);
                final inner = closing
                    ? 1.0
                    : cubicBezier(innerRaw, 0.2, 0.25, 0.25, 1.0);
                if (hidden || !visible) return const SizedBox.shrink();
                return Positioned(
                  left: item.rect.x - left,
                  top: item.rect.y - top,
                  width: item.rect.width,
                  height: item.rect.height,
                  child: DiscoTurnstileTile(
                    rect: Offset.zero & Size(item.rect.width, item.rect.height),
                    frame: DiscoTurnstileFrame(
                      outer: outer,
                      inner: inner,
                      closing: closing,
                    ),
                    color: prefsColor(item, Win10xFluent.accent),
                    child: _tile(context, item, transparentFace: true),
                  ),
                );
              }
              if (style == StartAnimation.metroRise) {
                return Positioned(
                  left: item.rect.x - left,
                  top: item.rect.y - top + 120 * (1 - progress),
                  width: item.rect.width,
                  height: item.rect.height,
                  child: Opacity(
                    opacity: hidden ? 0 : progress,
                    child: _tile(context, item),
                  ),
                );
              }
              // Factor: slide/scale from 80% about the tile center.
              return Positioned(
                left: item.rect.x - left,
                top: item.rect.y - top + 16 * (1 - progress),
                width: item.rect.width,
                height: item.rect.height,
                child: Opacity(
                  opacity: hidden ? 0 : progress,
                  child: Transform.scale(
                    scale: 0.8 + 0.2 * progress,
                    alignment: Alignment.center,
                    child: _tile(context, item),
                  ),
                ),
              );
            },
          ),
      ],
    );
    if (style == StartAnimation.flutterMetroGroup) {
      final p = StartReveal.flutterMetroGroupProgress(
        elapsed: elapsed,
        closing: closing,
        groupIndex: index,
      );
      groupContent = Transform.translate(
        offset: Offset(100 * (1 - p), 0),
        child: Transform.scale(
          scale: 0.8 + 0.2 * p,
          alignment: Alignment.center,
          child: Opacity(opacity: p, child: groupContent),
        ),
      );
    }
    if (_launchId != null && !discoLaunch) {
      final lp = _launch.value;
      final retreat = prefs.launchAnimation == LaunchAnimation.windows8;
      final boardLeft = _placements.isEmpty
          ? left
          : _placements.map((item) => item.rect.x).reduce(math.min);
      final boardTop = _placements.isEmpty
          ? top
          : _placements.map((item) => item.rect.y).reduce(math.min);
      final boardRight = _placements.isEmpty
          ? right
          : _placements.map((item) => item.rect.right).reduce(math.max);
      final boardBottom = _placements.isEmpty
          ? bottom
          : _placements.map((item) => item.rect.bottom).reduce(math.max);
      final boardCenter = Offset(
        (boardLeft + boardRight) / 2,
        (boardTop + boardBottom) / 2,
      );
      if (retreat) {
        final progress = easeDecelerate(
          (lp *
                  StartReveal.windows8LaunchDuration /
                  StartReveal.windows8RetreatDuration)
              .clamp(0.0, 1.0),
        );
        groupContent = Transform.scale(
          scale: 1 - 0.28 * progress,
          alignment: Alignment(
            ((boardCenter.dx - left) / math.max(1, right - left)) * 2 - 1,
            ((boardCenter.dy - top) / math.max(1, bottom - top)) * 2 - 1,
          ),
          child: Opacity(
            opacity: math.pow(1 - progress, 1.35).toDouble(),
            child: groupContent,
          ),
        );
      } else if (prefs.launchAnimation != LaunchAnimation.windows10Mobile &&
          prefs.launchAnimation != LaunchAnimation.factorClipReveal) {
        groupContent = Opacity(
          opacity: 1 - easeDecelerate((lp * 1.5).clamp(0.0, 1.0)),
          child: groupContent,
        );
      }
    }
    return [
      Positioned(
        left: left,
        top: top,
        width: right - left,
        height: bottom - top,
        child: groupContent,
      ),
    ];
  }

  ({String summary, String body, int count})? _liveTile(String appId) {
    final notices = ref.watch(desktopNotificationsProvider);
    final norm = normalizedAppId(appId);
    for (final group in notices.groups) {
      final n = group.newest.notification;
      final entry = normalizedAppId(n.desktopEntry);
      final name = normalizedAppId(n.appName);
      if (entry == norm ||
          name == norm ||
          entry.contains(norm) ||
          norm.contains(entry) && entry.isNotEmpty) {
        return (summary: n.summary, body: n.body, count: group.members.length);
      }
    }
    return null;
  }

  Widget _tileFace(
    BuildContext context,
    TilePlacement placement, {
    bool transparentFace = false,
  }) {
    final tile = placement.tile;
    final app = _app(tile.appId);
    final color = prefsColor(
      placement,
      const [
        Color(0xff0078d4),
        Color(0xff008a00),
        Color(0xffa200ff),
        Color(0xffd24726),
        Color(0xff008299),
      ][tile.appId.codeUnits.fold(0, (a, b) => a + b) % 5],
    );
    final resizing = _resizeId == tile.appId;
    final live = _liveTile(tile.appId);
    final wide = tile.width > 1 || tile.height > 1;
    final services = ShellNavigation.instance.services;
    final mediaAsync = services == null
        ? null
        : ref.watch(services.media).asData?.value;
    final media = mediaAsync != null && mediaMatchesApp(mediaAsync, tile.appId)
        ? mediaAsync
        : null;
    final mediaCommands = services == null
        ? null
        : ref.read(services.mediaCommands);
    final tileFont = 12.0 * uiFontScale(_preferences.value.tileFontSize);
    final iconFace = Align(
      alignment: const Alignment(0, -0.1),
      child: SizedBox(
        width: math.min(
          42,
          math.min(placement.rect.width, placement.rect.height) * 0.45,
        ),
        height: math.min(
          42,
          math.min(placement.rect.width, placement.rect.height) * 0.45,
        ),
        child: app == null
            ? const MarkUiIcon(Icons.apps, color: Colors.white)
            : buildMarkUiApplicationIcon(
                context,
                app.id,
                widget.launcher.buildIcon,
                color: Colors.white,
              ),
      ),
    );
    final contentFace = live == null
        ? iconFace
        : Padding(
            padding: const EdgeInsets.fromLTRB(10, 10, 10, 28),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  live.summary,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: tileFont + 2,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                if (live.body.isNotEmpty) ...[
                  const SizedBox(height: 4),
                  Expanded(
                    child: Text(
                      live.body,
                      maxLines: tile.height > 1 ? 5 : 2,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        color: Colors.white70,
                        fontSize: tileFont,
                      ),
                    ),
                  ),
                ],
              ],
            ),
          );
    final liveLayer = wide && (live != null || media != null)
        ? LiveTileFace(
            enabled: live != null || media != null,
            iconFace: iconFace,
            contentFace: contentFace,
            media: media,
            mediaCommands: mediaCommands,
            accent: Color(_preferences.value.phoneAccentArgbValue),
          )
        : iconFace;
    Widget face = Material(
      color: transparentFace ? Colors.transparent : color,
      shape: resizing
          ? const RoundedRectangleBorder(
              borderRadius: Win10xFluent.zero,
              side: BorderSide(color: Win10xFluent.accent, width: 2),
            )
          : Win10xFluent.square,
      child: Stack(
        children: [
          // 45% vertical center — same slot while the tile face scales.
          Positioned.fill(child: liveLayer),
          if (live != null && live.count > 0)
            Positioned(
              right: 6,
              top: 6,
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                color: Colors.white.withValues(alpha: 0.92),
                child: Text(
                  live.count > 99 ? '99+' : '${live.count}',
                  style: TextStyle(
                    color: Color(_preferences.value.phoneAccentArgbValue),
                    fontSize: 11,
                    fontWeight: FontWeight.w700,
                  ),
                ),
              ),
            ),
          if (_preferences.value.isPhone && _phoneEditing) ...[
            Positioned(
              right: 0,
              top: 0,
              child: SizedBox(
                width: 32,
                height: 32,
                child: IconButton(
                  tooltip: '取消固定',
                  padding: EdgeInsets.zero,
                  onPressed: () {
                    _preferences.change((p) {
                      p.tiles.removeWhere((t) => t.appId == tile.appId);
                      if (p.isPhone) {
                        final cols = tileGridColumns(p, _layoutWidth);
                        final packed = packTilesTopLeft(p.tiles, cols);
                        p.tiles
                          ..clear()
                          ..addAll(packed);
                      }
                    });
                    WpToast.show(ref, '已从开始屏幕取消固定');
                  },
                  icon: const MarkUiIcon(
                    Icons.push_pin_outlined,
                    size: 18,
                    color: Colors.white,
                  ),
                ),
              ),
            ),
            Positioned(
              right: 0,
              bottom: 0,
              child: SizedBox(
                width: 32,
                height: 32,
                child: IconButton(
                  tooltip: '更改大小',
                  padding: EdgeInsets.zero,
                  onPressed: () => _enterResize(tile),
                  icon: const MarkUiIcon(
                    Icons.open_in_full,
                    size: 18,
                    color: Colors.white,
                  ),
                ),
              ),
            ),
          ],
          if (wide)
            Positioned(
              left: 10,
              bottom: 8,
              right: 8,
              child: Text(
                app?.name ?? tile.appId,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(
                  color: Colors.white,
                  fontSize: 12 * uiFontScale(_preferences.value.tileFontSize),
                ),
              ),
            ),
        ],
      ),
    );
    if (_preferences.value.isPhone && _phoneEditing) {
      final phase = (tile.appId.hashCode % 7) / 7.0;
      face = AnimatedBuilder(
        animation: _wobble,
        child: face,
        builder: (context, child) {
          final angle =
              math.sin((_wobble.value * 2 * math.pi) + phase * math.pi) * 0.035;
          return Transform.rotate(angle: angle, child: child);
        },
      );
    }
    return face;
  }

  Widget _tile(
    BuildContext context,
    TilePlacement placement, {
    bool transparentFace = false,
  }) {
    final tile = placement.tile;
    final app = _app(tile.appId);
    if (_launchId != null &&
        _launchRect != null &&
        _preferences.value.launchAnimation == LaunchAnimation.windows8 &&
        normalizedAppId(tile.appId) == normalizedAppId(_launchId!)) {
      return const SizedBox.shrink();
    }
    final face = _tileFace(
      context,
      placement,
      transparentFace: transparentFace,
    );
    return Listener(
      key: ValueKey('tile.${tile.appId}'),
      behavior: HitTestBehavior.opaque,
      onPointerDown: (event) {
        if (_dragLocked || _launchId != null || _resizeId != null) return;
        _touchPointer = event.kind == PointerDeviceKind.touch;
        _pointer = event.pointer;
        _pointerDownGlobal = event.position;
        _pendingPlacement = placement;
        _grab = event.localPosition;
        _suppressTap = false;
        _attachPointerRoute();
      },
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        onTap: app == null
            ? null
            : () {
                if (_suppressTap || _dragId != null || _resizeId != null) {
                  _suppressTap = false;
                  return;
                }
                if (_phoneEditing) {
                  setState(() => _phoneEditing = false);
                  _wobble
                    ..stop()
                    ..value = 0;
                  return;
                }
                unawaited(_start(app));
              },
        onLongPressStart: (details) {
          if (_preferences.value.isPhone) {
            setState(() {
              _phoneEditing = true;
              _beginDrag(placement, _grab);
            });
            if (!_wobble.isAnimating) _wobble.repeat(reverse: true);
          } else {
            _menuAnchor = details.globalPosition;
            _tileMenu(context, tile);
          }
        },
        onSecondaryTapDown: (details) {
          _menuAnchor = details.globalPosition;
          _tileMenu(context, tile);
        },
        child: face,
      ),
    );
  }

  Future<void> _appMenu(BuildContext context, LauncherApplication app) async {
    final pinned = _preferences.value.tiles.any(
      (tile) => normalizedAppId(tile.appId) == normalizedAppId(app.id),
    );
    void togglePin() {
      _preferences.change((prefs) {
        if (pinned) {
          prefs.tiles.removeWhere(
            (tile) => normalizedAppId(tile.appId) == normalizedAppId(app.id),
          );
        } else {
          final group = prefs.groups.first.id;
          final row = prefs.tiles
              .where((tile) => tile.groupId == group)
              .fold<int>(
                0,
                (value, tile) => math.max(value, tile.row + tile.height),
              );
          int? autoColor;
          if (prefs.autoAccentFromIcon) {
            final host = ShellNavigation.instance.services;
            if (host != null) {
              final accent = ref.read(host.accent);
              autoColor = nearestPhoneAccentArgb(accent, phoneAccentArgb);
            }
          }
          prefs.tiles.add(
            StartTile(
              appId: app.id,
              groupId: group,
              row: row,
              colorArgb: autoColor,
            ),
          );
          if (prefs.isPhone) {
            final packed = packTilesTopLeft(
              prefs.tiles,
              tileGridColumns(prefs, _layoutWidth),
            );
            prefs.tiles
              ..clear()
              ..addAll(packed);
          }
        }
      });
      _closeMenu();
      WpToast.show(ref, pinned ? '已从开始屏幕取消固定' : '已固定到开始屏幕');
    }

    _showMenu(
      context,
      placement: FluentMenuPlacement.nearPointer,
      icons: [
        IconButton(
          tooltip: pinned ? '取消固定' : '固定',
          onPressed: togglePin,
          icon: MarkUiIcon(
            pinned ? Icons.push_pin_outlined : Icons.push_pin_rounded,
            color: Colors.white,
          ),
        ),
        IconButton(
          tooltip: '应用信息',
          onPressed: () {
            _closeMenu();
            unawaited(AppActions.showInfo(ref, app));
          },
          icon: const MarkUiIcon(
            Icons.insert_drive_file_rounded,
            color: Colors.white,
          ),
        ),
        IconButton(
          tooltip: '卸载',
          onPressed: () {
            _closeMenu();
            unawaited(AppActions.uninstall(ref, app));
          },
          icon: const MarkUiIcon(
            Icons.delete_outline_rounded,
            color: Colors.white,
          ),
        ),
      ],
      children: [
        FluentMenuItem(
          label: pinned ? '从开始屏幕取消固定' : '固定到开始屏幕',
          onTap: togglePin,
        ),
        FluentMenuItem(
          label: '应用信息',
          onTap: () {
            _closeMenu();
            unawaited(AppActions.showInfo(ref, app));
          },
        ),
        FluentMenuItem(
          label: '卸载',
          danger: true,
          onTap: () {
            _closeMenu();
            unawaited(AppActions.uninstall(ref, app));
          },
        ),
        if (!_preferences.value.isPhone)
          FluentMenuItem(
            label: '固定到任务栏',
            onTap: () {
              _preferences.change((prefs) {
                if (!prefs.pins.contains(app.id)) prefs.pins.add(app.id);
              });
              _closeMenu();
              WpToast.show(ref, '已固定到任务栏');
            },
          ),
      ],
    );
  }

  void _closeMenu() {
    final handle = _menuHandle;
    _menuHandle = null;
    if (handle is WpPopupController) {
      handle.close();
    } else if (handle is ShellPopupHandle) {
      handle.close();
    }
  }

  void _commitGroupName(int id) {
    if (_renamingGroup != id) return;
    final value = _groupName.text.trim();
    _renamingGroup = null;
    _preferences.change((prefs) {
      final index = prefs.groups.indexWhere((group) => group.id == id);
      if (index >= 0 && value.isNotEmpty) {
        prefs.groups[index] = TileGroup(id, value);
      }
    });
    if (mounted) setState(() {});
  }

  void _tileMenu(BuildContext context, StartTile tile) => _showMenu(
    context,
    placement: FluentMenuPlacement.nearPointer,
    icons: [
      IconButton(
        tooltip: '更改大小',
        onPressed: () {
          _closeMenu();
          _enterResize(tile);
        },
        icon: const MarkUiIcon(Icons.open_in_full, color: Colors.white),
      ),
      IconButton(
        tooltip: '取消固定',
        onPressed: () {
          _preferences.change((prefs) {
            prefs.tiles.removeWhere((item) => item.appId == tile.appId);
            if (prefs.isPhone) {
              final packed = packTilesTopLeft(
                prefs.tiles,
                tileGridColumns(prefs, _layoutWidth),
              );
              prefs.tiles
                ..clear()
                ..addAll(packed);
            }
          });
          _closeMenu();
          WpToast.show(ref, '已从开始屏幕取消固定');
        },
        icon: const MarkUiIcon(Icons.push_pin_outlined, color: Colors.white),
      ),
    ],
    children: [
      FluentMenuItem(
        label: '更改大小',
        onTap: () {
          _closeMenu();
          _enterResize(tile);
        },
      ),
      if (_preferences.value.isPhone) ...[
        FluentMenuItem(
          label: tile.transparent ? '恢复磁贴底色' : '透明磁贴（露出壁纸）',
          onTap: () {
            _preferences.change((prefs) {
              final index = prefs.tiles.indexWhere(
                (item) => item.appId == tile.appId,
              );
              if (index >= 0) {
                prefs.tiles[index] = tile.copyWith(
                  transparent: !tile.transparent,
                );
              }
            });
            _closeMenu();
          },
        ),
        FluentMenuItem(
          label: '磁贴颜色',
          onTap: () {
            _closeMenu();
            _showMenu(
              context,
              placement: FluentMenuPlacement.nearPointer,
              children: [
                for (var i = 0; i < phoneAccentArgb.length; i++)
                  FluentMenuItem(
                    label: phoneAccentLabels[i],
                    onTap: () {
                      _preferences.change((prefs) {
                        final index = prefs.tiles.indexWhere(
                          (item) => item.appId == tile.appId,
                        );
                        if (index >= 0) {
                          prefs.tiles[index] = tile.copyWith(
                            colorArgb: phoneAccentArgb[i],
                            transparent: false,
                          );
                        }
                      });
                      _closeMenu();
                    },
                  ),
                FluentMenuItem(
                  label: '跟随主题强调色',
                  onTap: () {
                    _preferences.change((prefs) {
                      final index = prefs.tiles.indexWhere(
                        (item) => item.appId == tile.appId,
                      );
                      if (index >= 0) {
                        prefs.tiles[index] = tile.copyWith(clearColor: true);
                      }
                    });
                    _closeMenu();
                  },
                ),
              ],
            );
          },
        ),
      ],
      for (final group in _preferences.value.groups)
        if (group.id != tile.groupId)
          FluentMenuItem(
            label: '移动到 ${group.title}',
            onTap: () {
              _preferences.change((prefs) {
                final index = prefs.tiles.indexWhere(
                  (item) => item.appId == tile.appId,
                );
                if (index >= 0) {
                  prefs.tiles[index] = tile.copyWith(groupId: group.id);
                }
              });
              _closeMenu();
            },
          ),
      FluentMenuItem(
        label: '新建组',
        onTap: () {
          _preferences.change((prefs) {
            final id =
                prefs.groups.fold(
                  0,
                  (value, group) => math.max(value, group.id),
                ) +
                1;
            prefs.groups.add(TileGroup(id, '新建组'));
            final index = prefs.tiles.indexWhere(
              (item) => item.appId == tile.appId,
            );
            if (index >= 0) {
              prefs.tiles[index] = tile.copyWith(groupId: id);
            }
          });
          _closeMenu();
        },
      ),
      const FluentSeparator(),
      FluentMenuItem(
        label: '应用信息',
        onTap: () {
          _closeMenu();
          final app = _app(tile.appId);
          if (app != null) {
            unawaited(AppActions.showInfo(ref, app));
          } else {
            WpToast.show(ref, tile.appId);
          }
        },
      ),
      if (_app(tile.appId) case final app?)
        FluentMenuItem(
          label: '卸载',
          danger: true,
          onTap: () {
            _closeMenu();
            unawaited(AppActions.uninstall(ref, app));
          },
        ),
      FluentMenuItem(
        label: '取消固定',
        danger: true,
        onTap: () {
          _preferences.change((prefs) {
            prefs.tiles.removeWhere((item) => item.appId == tile.appId);
            if (prefs.isPhone) {
              final packed = packTilesTopLeft(
                prefs.tiles,
                tileGridColumns(prefs, _layoutWidth),
              );
              prefs.tiles
                ..clear()
                ..addAll(packed);
            }
          });
          _closeMenu();
          WpToast.show(ref, '已从开始屏幕取消固定');
        },
      ),
    ],
  );

  void _maybePromptNotificationAccess() {
    final prefs = _preferences.value;
    if (!prefs.isPhone || prefs.notificationAccessPrompted) return;
    _preferences.change((p) => p.notificationAccessPrompted = true);
    WpToast.show(
      ref,
      'Live Tile 需要桌面通知。可在设置中管理通知权限。',
      hold: const Duration(milliseconds: 4000),
      actions: [
        (
          label: '打开设置',
          onTap: () {
            unawaited(SettingsLaunch.open(page: SettingsLaunch.permissions));
          },
        ),
      ],
    );
  }

  void _showMenu(
    BuildContext context, {
    required FluentMenuPlacement placement,
    required List<Widget> children,
    List<Widget> icons = const [],
  }) {
    final anchor = _menuAnchor;
    final prefs = _preferences.value;
    _closeMenu();
    if (prefs.isPhone) {
      if (placement == FluentMenuPlacement.nearPointer) {
        _menuHandle = WpPopups.contextMenu(
          ref,
          anchor:
              anchor ??
              Offset(
                MediaQuery.sizeOf(context).width / 2,
                MediaQuery.sizeOf(context).height / 2,
              ),
          children: children,
        );
        return;
      }
      _menuHandle = WpPopups.applicationBarMenu(
        ref,
        children: children,
        icons: icons,
      );
      return;
    }
    _menuHandle = ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X Start menu',
          builder: (context, handle) {
            return Win10xFluent.wrap(
              FluentMenuSurface(
                placement: placement,
                anchor: anchor,
                children: children,
              ),
            );
          },
        );
  }

  List<Widget> _resizeChrome(List<TilePlacement> placements) {
    final appId = _resizeId;
    if (appId == null) return const [];
    TilePlacement? placement;
    for (final item in placements) {
      if (item.tile.appId == appId) {
        placement = item;
        break;
      }
    }
    if (placement == null) return const [];
    final rect = placement.rect;
    final tile = placement.tile;
    return [
      Positioned(
        left: rect.x,
        top: rect.y,
        width: rect.width,
        height: rect.height,
        child: IgnorePointer(
          child: CustomPaint(
            painter: _TileSizeGuidePainter(
              widthCells: tile.width,
              heightCells: tile.height,
            ),
          ),
        ),
      ),
      Positioned(
        left: rect.x + rect.width - _resizeHandle,
        top: rect.y + rect.height - _resizeHandle,
        width: _resizeHandle,
        height: _resizeHandle,
        child: Listener(
          behavior: HitTestBehavior.opaque,
          onPointerDown: (event) {
            _pointer = event.pointer;
            _draggingHandle = true;
            _attachPointerRoute();
            _updateResize(event.position);
          },
          child: DecoratedBox(
            decoration: BoxDecoration(
              color: const Color(0xe6ffffff),
              border: Border.all(color: Win10xFluent.accent),
            ),
            child: const Center(
              child: Text(
                '⋰',
                style: TextStyle(
                  color: Win10xFluent.accent,
                  fontSize: 14,
                  height: 1,
                ),
              ),
            ),
          ),
        ),
      ),
    ];
  }

  static const _startLabels = [
    'Windows 10 Mobile',
    'Disco turnstile',
    'Metro rise',
    'Factor slide/scale',
    'flutter_metro_ui',
  ];
  static const _launchLabels = [
    'Windows 10 Mobile',
    'Windows 8 flying tile',
    'Disco turnstile',
    'Factor clip reveal',
    '系统默认',
  ];

  void _settings(BuildContext context) {
    final prefs = _preferences.value;
    _showMenu(
      context,
      placement: FluentMenuPlacement.bottomBar,
      children: [
        const ShellModeSettings(),
        const FluentSeparator(),
        for (final layout in StartLayout.values)
          FluentRadio<StartLayout>(
            value: layout,
            groupValue: prefs.layout,
            label: ['Windows 10', 'Windows Phone', 'Windows 8'][layout.index],
            onChanged: (value) {
              _preferences.change((p) => p.layout = value);
              _closeMenu();
            },
          ),
        const FluentSeparator(),
        FluentMenuItem(
          label: '开始屏幕动画',
          subtitle: _startLabels[prefs.startAnimation.index],
          trailing: const MarkUiIcon(
            Icons.chevron_right,
            size: 18,
            color: Win10xFluent.textSecondary,
          ),
          onTap: () {
            final current = _preferences.value.startAnimation;
            _showMenu(
              context,
              placement: FluentMenuPlacement.bottomBar,
              children: [
                for (final style in StartAnimation.values)
                  FluentRadio<StartAnimation>(
                    value: style,
                    groupValue: current,
                    label: _startLabels[style.index],
                    onChanged: (value) {
                      _preferences.change((p) => p.startAnimation = value);
                      _closeMenu();
                    },
                  ),
              ],
            );
          },
        ),
        FluentMenuItem(
          label: '应用启动动画',
          subtitle: _launchLabels[prefs.launchAnimation.index],
          trailing: const MarkUiIcon(
            Icons.chevron_right,
            size: 18,
            color: Win10xFluent.textSecondary,
          ),
          onTap: () {
            final current = _preferences.value.launchAnimation;
            _showMenu(
              context,
              placement: FluentMenuPlacement.bottomBar,
              children: [
                for (final style in LaunchAnimation.values)
                  FluentRadio<LaunchAnimation>(
                    value: style,
                    groupValue: current,
                    label: _launchLabels[style.index],
                    onChanged: (value) {
                      _preferences.change((p) => p.launchAnimation = value);
                      _closeMenu();
                    },
                  ),
              ],
            );
          },
        ),
        FluentToggle(
          label: '应用窗口切换动画',
          value: prefs.switchAnimation,
          onChanged: (enabled) {
            _preferences.change((p) => p.switchAnimation = enabled);
            _closeMenu();
          },
        ),
      ],
    );
  }
}

/// Width along the top edge and height along the left edge, matching Rust
/// `paint_size_guides_inside`.
/// Settle curve for Start↔apps page push (legacy Rust easeOutExpo / 340 ms).
class _EaseOutExpoCurve extends Curve {
  const _EaseOutExpoCurve();

  @override
  double transformInternal(double t) => easeOutExpo(t);
}

class _TileSizeGuidePainter extends CustomPainter {
  const _TileSizeGuidePainter({
    required this.widthCells,
    required this.heightCells,
  });
  final int widthCells;
  final int heightCells;

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()
      ..color = const Color(0xe6ffffff)
      ..strokeWidth = 1.5
      ..style = PaintingStyle.stroke;
    const pad = 8.0;
    final topY = pad + 6;
    canvas.drawLine(Offset(pad, topY), Offset(size.width - pad, topY), paint);
    final leftX = pad + 6;
    canvas.drawLine(
      Offset(leftX, pad),
      Offset(leftX, size.height - pad),
      paint,
    );
    final text = TextPainter(
      textAlign: TextAlign.center,
      textDirection: TextDirection.ltr,
    );
    text.text = TextSpan(
      text: '$widthCells',
      style: const TextStyle(color: Color(0xe6ffffff), fontSize: 12),
    );
    text.layout();
    text.paint(
      canvas,
      Offset((size.width - text.width) / 2, topY - text.height - 1),
    );
    text.text = TextSpan(
      text: '$heightCells',
      style: const TextStyle(color: Color(0xe6ffffff), fontSize: 12),
    );
    text.layout();
    text.paint(
      canvas,
      Offset(leftX - text.width - 1, (size.height - text.height) / 2),
    );
  }

  @override
  bool shouldRepaint(covariant _TileSizeGuidePainter oldDelegate) =>
      oldDelegate.widthCells != widthCells ||
      oldDelegate.heightCells != heightCells;
}

class _DismissStartIntent extends Intent {
  const _DismissStartIntent();
}
