// SPDX-License-Identifier: GPL-3.0-only
import 'dart:math' as math;
import 'dart:async';

import 'catalog_index.dart';
import 'phone_chrome.dart';
import 'shell_navigation.dart';

import 'package:denial_flutter_sdk/state.dart';

import 'package:denial_flutter_sdk/launcher.dart';
import 'package:denial_flutter_sdk/popups.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'fluent.dart';
import 'fluent_controls.dart';
import 'model.dart';
import 'preferences.dart';
import 'fluent_motion.dart';
import 'start_motion.dart';

class AppCatalogView extends ConsumerStatefulWidget {
  const AppCatalogView({
    super.key,
    required this.apps,
    required this.buildIcon,
    required this.onLaunch,
    required this.onMenu,
    this.columns = false,
    this.discoListExit,
    this.launchingAppId,
    this.launchElapsed = 0,
  });
  final List<LauncherApplication> apps;
  final Widget Function(BuildContext, String) buildIcon;
  final ValueChanged<LauncherApplication> onLaunch;
  final void Function(LauncherApplication, Offset) onMenu;
  final bool columns;

  /// When true, apply Disco app-list exit (rotateY toward −90°, slide left).
  final bool? discoListExit;
  final String? launchingAppId;
  final double launchElapsed;
  @override
  ConsumerState<AppCatalogView> createState() => _AppCatalogState();
}

class _AppCatalogState extends ConsumerState<AppCatalogView> {
  final _keys = <String, GlobalKey>{};
  final _scroll = ScrollController();
  CatalogIndex _index = CatalogIndex({}, rowHeight: 56);
  String? _activeLetter;
  bool _vertical = true, _scrubbing = false, _indexing = false;
  double _railY = 0;
  int _jumpGeneration = 0;
  Timer? _badgeTimer;
  bool _showBadge = false;

  @override
  void initState() {
    super.initState();
    _scroll.addListener(() {
      if (!_indexing && !_scrubbing && mounted && _scroll.hasClients) {
        final letter = _index.letterAt(_scroll.offset);
        if (letter != _activeLetter) setState(() => _activeLetter = letter);
      }
    });
  }

  @override
  void dispose() {
    _badgeTimer?.cancel();
    _jumpGeneration++;
    _scroll.dispose();
    super.dispose();
  }

  Future<void> _goLetter(String letter, {bool animate = true}) async {
    final offset = _index.offsets[letter];
    if (!mounted || offset == null) return;
    final generation = ++_jumpGeneration;
    setState(() {
      _activeLetter = letter;
      _indexing = true;
    });
    if (_vertical && _scroll.hasClients) {
      final target = offset.clamp(0.0, _scroll.position.maxScrollExtent);
      if (animate) {
        await _scroll.animateTo(
          target,
          duration: FluentMotion.slow,
          curve: FluentMotion.standard,
        );
      } else {
        _scroll.jumpTo(target);
      }
    } else {
      final target = _keys[letter]?.currentContext;
      if (target != null) {
        await Scrollable.ensureVisible(
          target,
          duration: animate ? FluentMotion.slow : Duration.zero,
          curve: FluentMotion.standard,
        );
      }
    }
    if (mounted && generation == _jumpGeneration) _indexing = false;
  }

  void _railSelect(double y, double height, {required bool jump}) {
    final letter = _index.letterAtRail(y, height);
    if (letter == null) return;
    _badgeTimer?.cancel();
    final changed = letter != _activeLetter;
    setState(() {
      _railY = y.clamp(0, height);
      _showBadge = true;
      _scrubbing = jump;
      _activeLetter = letter;
    });
    final target = _scroll.hasClients
        ? _index.offsets[letter]!.clamp(0.0, _scroll.position.maxScrollExtent)
        : 0.0;
    if (jump &&
        (changed ||
            !_scroll.hasClients ||
            (_scroll.offset - target).abs() > .5)) {
      unawaited(_goLetter(letter, animate: false));
    }
  }

  void _railEnd() {
    _scrubbing = false;
    _badgeTimer?.cancel();
    _badgeTimer = Timer(const Duration(milliseconds: 400), () {
      if (mounted) setState(() => _showBadge = false);
    });
  }

  void _jump() {
    final phone = PreferenceController.instance.value.isPhone;
    final services = ShellNavigation.instance.services;
    final monitorId =
        ref.read(displayLayoutProvider)?.mainOutput?.monitorId ?? 0;
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X alphabet jump',
          keyName: 'win10x.alphabet',
          transitionDuration: FluentMotion.slow,
          builder: (context, handle) {
            final prefs = PreferenceController.instance.value;
            final grid = LayoutBuilder(
              builder: (context, constraints) => GridView.count(
                shrinkWrap: true,
                padding: EdgeInsets.zero,
                crossAxisCount: 6,
                mainAxisExtent: phone
                    ? math.min(
                        constraints.maxWidth / 6,
                        math.max(44.0, constraints.maxHeight / 5),
                      )
                    : null,
                children: [
                  for (final letter in [
                    '#',
                    ...List.generate(26, (i) => String.fromCharCode(65 + i)),
                  ])
                    TextButton(
                      onPressed: _index.offsets.containsKey(letter)
                          ? () {
                              handle.close();
                              unawaited(_goLetter(letter));
                            }
                          : null,
                      child: Text(letter, style: const TextStyle(fontSize: 24)),
                    ),
                ],
              ),
            );
            if (!phone) {
              return Win10xFluent.wrap(
                FluentMenuSurface(
                  placement: FluentMenuPlacement.bottomBar,
                  children: [grid],
                ),
              );
            }
            return Win10xFluent.wrapPhone(
              Material(
                color: prefs.phoneThemeDark
                    ? const Color(0xff101010)
                    : Colors.white,
                child: Stack(
                  fit: StackFit.expand,
                  children: [
                    Padding(
                      padding: const EdgeInsets.fromLTRB(20, 56, 20, 60),
                      child: grid,
                    ),
                    if (services != null)
                      Positioned(
                        left: 0,
                        right: 0,
                        bottom: 0,
                        height: 48,
                        child: PhoneNavigationBar(
                          services: services,
                          monitorId: monitorId,
                        ),
                      ),
                  ],
                ),
              ),
              dark: prefs.phoneThemeDark,
              accentColor: Color(prefs.phoneAccentArgbValue),
            );
          },
        );
  }

  Map<String, List<LauncherApplication>> _groups() {
    final groups = <String, List<LauncherApplication>>{};
    for (final app in widget.apps) {
      groups.putIfAbsent(catalogSection(app.name), () => []).add(app);
    }
    return {
      for (final letter in [
        '#',
        ...List.generate(26, (i) => String.fromCharCode(65 + i)),
      ])
        if (groups.containsKey(letter)) letter: groups[letter]!,
    };
  }

  @override
  Widget build(BuildContext context) {
    final groups = _groups();
    final phone = PreferenceController.instance.value.isPhone;
    final color = Theme.of(context).colorScheme.onSurface;
    final flat = <LauncherApplication>[
      for (final group in groups.values) ...group,
    ];
    Widget item(LauncherApplication app) {
      final rank = widget.discoListExit == true
          ? flat.indexWhere((a) => a.id == app.id)
          : 0;
      final disco = widget.discoListExit == true
          ? StartReveal.discoAppListFrame(
              elapsed: widget.launchElapsed,
              normalRank: math.max(0, rank),
              count: math.max(1, flat.length),
              selected:
                  widget.launchingAppId != null &&
                  normalizedAppId(widget.launchingAppId!) ==
                      normalizedAppId(app.id),
              launchingApp: true,
            )
          : null;
      if (disco != null && !disco.visible) {
        return const SizedBox.shrink();
      }
      final progress = disco?.progress ?? 0.0;
      final scale = math.cos(math.pi / 2 * progress).clamp(0.05, 1.0);
      final row = GestureDetector(
        onSecondaryTapDown: (d) => widget.onMenu(app, d.globalPosition),
        onLongPressStart: (d) => widget.onMenu(app, d.globalPosition),
        child: InkWell(
          onTap: () => widget.onLaunch(app),
          child: SizedBox(
            height: phone ? 56 : 48,
            child: Row(
              children: [
                SizedBox(
                  width: phone ? 48 : 40,
                  height: phone ? 40 : 32,
                  child: widget.buildIcon(context, app.id),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Text(
                    app.name,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontSize:
                          (phone ? 20 : 14) *
                          uiFontScale(
                            PreferenceController.instance.value.drawerFontSize,
                          ),
                      color: color,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      );
      if (disco == null) return row;
      final width = MediaQuery.sizeOf(context).width;
      return Transform(
        alignment: Alignment.centerLeft,
        transform: Matrix4.identity()
          ..setEntry(3, 2, 0.001)
          ..translateByDouble(-width / 3 * progress, 0, 0, 1)
          ..rotateY(-math.pi / 2 * progress)
          ..scaleByDouble(scale, 1, 1, 1),
        child: row,
      );
    }

    _index = CatalogIndex({
      for (final entry in groups.entries) entry.key: entry.value.length,
    }, rowHeight: phone ? 56 : 48);
    if (!_index.offsets.containsKey(_activeLetter)) {
      _activeLetter = _index.letters.firstOrNull;
    }
    Widget header(String letter) => SizedBox(
      height: 48,
      child: Align(
        alignment: Alignment.centerLeft,
        child: TextButton(
          key: _keys.putIfAbsent(letter, GlobalKey.new),
          onPressed: _jump,
          child: Container(
            width: 32,
            height: 32,
            alignment: Alignment.center,
            decoration: BoxDecoration(
              border: Border.all(color: color.withValues(alpha: .6)),
            ),
            child: Text(
              letter,
              style: TextStyle(
                fontSize:
                    (phone ? 22 : 16) *
                    uiFontScale(
                      PreferenceController.instance.value.drawerFontSize,
                    ),
                color: color,
              ),
            ),
          ),
        ),
      ),
    );
    if (widget.apps.isEmpty) return const Center(child: Text('没有匹配的应用'));

    return LayoutBuilder(
      builder: (context, c) {
        final cols = widget.columns
            ? math.max(1, (c.maxWidth / 300).floor())
            : 1;
        _vertical = cols == 1;
        if (cols == 1) {
          final rows = <({String letter, LauncherApplication? app})>[
            for (final group in groups.entries) ...[
              (letter: group.key, app: null),
              for (final app in group.value) (letter: group.key, app: app),
            ],
          ];
          final list = CustomScrollView(
            controller: _scroll,
            slivers: [
              SliverPadding(
                padding: EdgeInsets.fromLTRB(16, 8, phone ? 56 : 16, 8),
                sliver: SliverVariedExtentList(
                  itemExtentBuilder: (index, dimensions) =>
                      rows[index].app == null
                      ? 48
                      : phone
                      ? 56
                      : 48,
                  delegate: _CatalogChildren(
                    (context, index) => switch (rows[index].app) {
                      final app? => item(app),
                      null => header(rows[index].letter),
                    },
                    childCount: rows.length,
                    extent: _index.contentExtent - 16,
                  ),
                ),
              ),
            ],
          );
          if (!phone) return list;
          return Stack(
            children: [
              list,
              Positioned(
                right: 0,
                top: 8,
                bottom: 8,
                width: 40,
                child: LayoutBuilder(
                  builder: (context, constraints) {
                    final height = constraints.maxHeight;
                    return Semantics(
                      label: '快速字母索引',
                      child: GestureDetector(
                        behavior: HitTestBehavior.opaque,
                        onTapDown: (d) => _railSelect(
                          d.localPosition.dy,
                          height,
                          jump: false,
                        ),
                        onTapUp: (d) {
                          final letter = _index.letterAtRail(
                            d.localPosition.dy,
                            height,
                          );
                          if (letter != null) unawaited(_goLetter(letter));
                          _railEnd();
                        },
                        onTapCancel: _railEnd,
                        onVerticalDragStart: (d) =>
                            _railSelect(d.localPosition.dy, height, jump: true),
                        onVerticalDragUpdate: (d) =>
                            _railSelect(d.localPosition.dy, height, jump: true),
                        onVerticalDragEnd: (_) => _railEnd(),
                        onVerticalDragCancel: _railEnd,
                        child: Column(
                          children: [
                            for (final letter in _index.letters)
                              Expanded(
                                child: Semantics(
                                  button: true,
                                  selected: letter == _activeLetter,
                                  label: '跳转到 $letter',
                                  onTap: () => unawaited(_goLetter(letter)),
                                  child: Center(
                                    child: FittedBox(
                                      fit: BoxFit.scaleDown,
                                      child: Text(
                                        letter,
                                        style: TextStyle(
                                          fontSize: 11,
                                          color: letter == _activeLetter
                                              ? Color(
                                                  PreferenceController
                                                      .instance
                                                      .value
                                                      .phoneAccentArgbValue,
                                                )
                                              : color.withValues(alpha: .85),
                                          fontWeight: FontWeight.w600,
                                        ),
                                      ),
                                    ),
                                  ),
                                ),
                              ),
                          ],
                        ),
                      ),
                    );
                  },
                ),
              ),
              if (_showBadge && _activeLetter != null)
                Positioned(
                  right: 52,
                  top: (_railY - 28).clamp(
                    0.0,
                    math.max(0.0, c.maxHeight - 64),
                  ),
                  child: IgnorePointer(
                    child: ExcludeSemantics(
                      child: Container(
                        width: 56,
                        height: 56,
                        alignment: Alignment.center,
                        color: Color(
                          PreferenceController
                              .instance
                              .value
                              .phoneAccentArgbValue,
                        ),
                        child: Text(
                          _activeLetter!,
                          style: TextStyle(
                            color:
                                Color(
                                      PreferenceController
                                          .instance
                                          .value
                                          .phoneAccentArgbValue,
                                    ).computeLuminance() >
                                    .4
                                ? Colors.black
                                : Colors.white,
                            fontSize: 32,
                          ),
                        ),
                      ),
                    ),
                  ),
                ),
            ],
          );
        }
        final sections = [
          for (final group in groups.entries)
            Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                header(group.key),
                for (final app in group.value) item(app),
              ],
            ),
        ];
        // Column-major scan order, as the full-screen Windows 10 All Apps view.
        final perColumn = (sections.length / cols).ceil();
        return SingleChildScrollView(
          padding: const EdgeInsets.all(24),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              for (var col = 0; col < cols; col++)
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 12),
                    child: Column(
                      children: sections
                          .skip(col * perColumn)
                          .take(perColumn)
                          .toList(),
                    ),
                  ),
                ),
            ],
          ),
        );
      },
    );
  }
}

// Fixed section/row heights make the full extent known even for a lazy list.
// Flutter's average-of-visible-children estimate otherwise clamps a far jump
// before the true final section, then changes its maximum after that jump.
class _CatalogChildren extends SliverChildBuilderDelegate {
  _CatalogChildren(
    super.builder, {
    required super.childCount,
    required this.extent,
  });
  final double extent;
  @override
  double estimateMaxScrollOffset(
    int firstIndex,
    int lastIndex,
    double leadingScrollOffset,
    double trailingScrollOffset,
  ) => extent;
}
