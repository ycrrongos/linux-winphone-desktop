// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/popups.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/settings.dart';
import 'package:denial_flutter_sdk/environment.dart';

import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app_switch.dart';
import 'controls.dart';
import 'calendar.dart';
import 'phone_chrome.dart';
import 'shell_navigation.dart';
import 'symbols.dart';
import 'fluent.dart';
import 'fluent_controls.dart';
import 'model.dart';
import 'preferences.dart';

class Win10xTaskbar extends ConsumerStatefulWidget {
  const Win10xTaskbar({
    super.key,
    required this.services,
    required this.monitorId,
  });
  final ShellServices services;
  final int monitorId;

  @override
  ConsumerState<Win10xTaskbar> createState() => _Win10xTaskbarState();
}

class _Win10xTaskbarState extends ConsumerState<Win10xTaskbar> {
  ShellServices get services => widget.services;
  int get monitorId => widget.monitorId;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final panel = ref.read(
        startupEnvironmentProvider,
      )['WIN10X_PREVIEW_PANEL'];
      if (panel == 'action-center') services.openDashboard();
      if (panel == 'task-view') {
        _showWindows(context, ref, ref.read(services.windows(monitorId)));
      }
      if (mounted &&
          (ref.read(startupEnvironmentProvider).flag('WIN10X_PREVIEW_START') ||
              (PreferenceController.instance.value.mode != ShellMode.desktop &&
                  (panel == null || panel.isEmpty)))) {
        if (!ShellNavigation.instance.launcherVisible) {
          ShellNavigation.instance.home(services);
        }
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final controller = PreferenceController.instance;
    final catalog = ref.watch(services.applications);
    final windows = ref.watch(services.windows(monitorId));
    ref.listen(services.windows(monitorId), (previous, next) {
      if (PreferenceController.instance.value.mode != ShellMode.desktop &&
          previous?.any((w) => !w.minimized) == true &&
          !next.any((w) => !w.minimized)) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted && !ShellNavigation.instance.launcherVisible) {
            ShellNavigation.instance.home(services);
          }
        });
      }
    });
    final clock = ref.watch(services.clock).asData?.value;
    return ListenableBuilder(
      listenable: controller,
      builder: (context, _) {
        final prefs = controller.value;
        final entries =
            <
              ({
                String id,
                String appId,
                String label,
                List<ApplicationWindow> windows,
              })
            >[];
        final used = <int>{};
        for (final pin in prefs.pins) {
          final matches = catalog.where(
            (app) => normalizedAppId(app.id) == normalizedAppId(pin),
          );
          if (matches.isEmpty) continue;
          final app = matches.first;
          final running = windows
              .where(
                (window) => app.windowAppIds.any(
                  (id) => normalizedAppId(id) == normalizedAppId(window.appId),
                ),
              )
              .toList();
          used.addAll(running.map((window) => window.id));
          entries.add((
            id: app.id,
            appId: app.appId,
            label: app.name,
            windows: running,
          ));
        }

        for (final window in windows.where(
          (window) => !used.contains(window.id),
        )) {
          final previous = entries.indexWhere(
            (entry) => entry.appId == window.appId,
          );
          if (previous >= 0) {
            entries[previous].windows.add(window);
          } else {
            final apps = catalog.where(
              (app) => app.windowAppIds.any(
                (id) => normalizedAppId(id) == normalizedAppId(window.appId),
              ),
            );
            entries.add((
              id: apps.isEmpty ? '' : apps.first.id,
              appId: window.appId,
              label: apps.isEmpty ? window.title : apps.first.name,
              windows: [window],
            ));
          }
        }
        if (prefs.isPhone) {
          return PhoneNavigationBar(services: services, monitorId: monitorId);
        }
        final tablet = prefs.isTablet;
        final commandWidth = MediaQuery.sizeOf(context).width < 500
            ? 40.0
            : 48.0;
        void tasks() {
          if (ShellNavigation.instance.nativeTasks != null) {
            ShellNavigation.instance.nativeTasks!();
          } else {
            _showWindows(context, ref, windows);
          }
        }

        return Material(
          color: Win10xFluent.chrome,
          child: Row(
            children: [
              ShellButton(
                key: const ValueKey('win10x.start'),
                label: '开始',
                width: commandWidth,
                onPressed: () => ShellNavigation.instance.home(services),
                child: const ShellGlyph(ShellSymbol.start),
              ),
              if (tablet)
                ShellButton(
                  label: '返回',
                  width: commandWidth,
                  onPressed: () => ShellNavigation.instance.back(services),
                  child: const ShellGlyph(ShellSymbol.back),
                ),
              ShellButton(
                label: '搜索',
                width: commandWidth,
                onPressed: () => ShellNavigation.instance.search(services),
                child: const ShellGlyph(ShellSymbol.search),
              ),
              ShellButton(
                key: const ValueKey('win10x.task-view'),
                label: '任务视图',
                width: commandWidth,
                onPressed: tasks,
                child: const ShellGlyph(ShellSymbol.tasks),
              ),
              Expanded(
                child: tablet
                    ? const SizedBox()
                    : SingleChildScrollView(
                        scrollDirection: Axis.horizontal,
                        child: Row(
                          children: [
                            for (final entry in entries)
                              Builder(
                                builder: (buttonContext) => ShellButton(
                                  key: ValueKey('win10x.app.${entry.appId}'),
                                  label: entry.label,
                                  running: entry.windows.isNotEmpty,
                                  active: entry.windows.any((w) => w.active),
                                  width: commandWidth,
                                  onPressed: () {
                                    if (entry.windows.isEmpty) {
                                      unawaited(
                                        services.launchApplication(
                                          entry.id,
                                          monitorId: monitorId,
                                        ),
                                      );
                                    } else {
                                      final target =
                                          entry.windows
                                              .where((w) => w.active)
                                              .firstOrNull ??
                                          entry.windows.first;
                                      if (target.active && !target.minimized) {
                                        services.minimizeWindow(target.id);
                                      } else {
                                        final box =
                                            buttonContext.findRenderObject()
                                                as RenderBox?;
                                        final origin = box == null
                                            ? Rect.zero
                                            : box.localToGlobal(Offset.zero) &
                                                  box.size;
                                        final outgoing = entries
                                            .expand((e) => e.windows)
                                            .where(
                                              (w) =>
                                                  w.active &&
                                                  !w.minimized &&
                                                  w.id != target.id,
                                            )
                                            .firstOrNull;
                                        unawaited(
                                          AppSwitchHandoff.activate(
                                            ref: ref,
                                            services: services,
                                            target: target,
                                            origin: origin,
                                            outgoing: outgoing,
                                            output: MediaQuery.sizeOf(
                                              buttonContext,
                                            ),
                                          ),
                                        );
                                      }
                                    }
                                  },
                                  onSecondaryTap: () => _showAppMenu(
                                    context,
                                    ref,
                                    entry.id,
                                    entry.windows,
                                  ),
                                  child: buildMarkUiApplicationIcon(
                                    context,
                                    entry.appId,
                                    services.buildApplicationIcon,
                                  ),
                                ),
                              ),
                          ],
                        ),
                      ),
              ),
              if (MediaQuery.sizeOf(context).width > 700)
                SizedBox(
                  width: 72,
                  child: ClipRect(
                    child: services.buildSystemTray(
                      context,
                      horizontal: true,
                      itemIds: ref.watch(services.trayItemIds).take(3).toList(),
                      foregroundColor: Win10xFluent.text,
                    ),
                  ),
                ),
              if (MediaQuery.sizeOf(context).width > 500)
                ShellButton(
                  label: '音量',
                  width: 40,
                  onPressed: () => _volume(context, ref),
                  child: const MarkUiIcon(Icons.volume_up_outlined, size: 20),
                ),
              InkWell(
                onTap: () => _calendar(context, ref),
                child: SizedBox(
                  width: MediaQuery.sizeOf(context).width < 500 ? 68 : 86,
                  child: Center(
                    child: Text(
                      clock == null
                          ? ''
                          : '${clock.hour.toString().padLeft(2, '0')}:${clock.minute.toString().padLeft(2, '0')}\n${clock.year}/${clock.month}/${clock.day}',
                      textAlign: TextAlign.center,
                      style: const TextStyle(fontSize: 12),
                    ),
                  ),
                ),
              ),
              ShellButton(
                label: '操作中心',
                width: commandWidth,
                onPressed: services.toggleDashboard,
                child: const ShellGlyph(ShellSymbol.notifications),
              ),
              SizedBox(
                width: 5,
                child: InkWell(
                  onTap: services.toggleDesktop,
                  child: const DecoratedBox(
                    decoration: BoxDecoration(
                      border: Border(
                        left: BorderSide(color: Win10xFluent.strokeCard),
                      ),
                    ),
                    child: SizedBox.expand(),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  void _calendar(BuildContext context, WidgetRef ref) {
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X calendar',
          keyName: 'win10x.calendar',
          builder: (context, handle) => Win10xFluent.wrap(
            Align(
              alignment: Alignment.bottomRight,
              child: Padding(
                padding: EdgeInsets.only(
                  bottom: PreferenceController.instance.value.bottomInset,
                ),
                child: SingleChildScrollView(child: const Win10xCalendar()),
              ),
            ),
          ),
        );
  }

  void _volume(BuildContext context, WidgetRef ref) {
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X volume',
          keyName: 'win10x.volume',
          builder: (context, handle) => Win10xFluent.wrap(
            Align(
              alignment: Alignment.bottomRight,
              child: Padding(
                padding: EdgeInsets.only(
                  bottom: PreferenceController.instance.value.bottomInset,
                ),
                child: Material(
                  color: Win10xFluent.chrome,
                  child: SizedBox(
                    width: 320,
                    child: Consumer(
                      builder: (context, ref, _) => Padding(
                        padding: const EdgeInsets.all(16),
                        child: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            const Align(
                              alignment: Alignment.centerLeft,
                              child: Text('音量'),
                            ),
                            Slider(
                              value: ref.watch(services.volume).clamp(0, 1),
                              onChangeStart: (_) =>
                                  services.beginVolumeInteraction(),
                              onChanged: services.setVolume,
                              onChangeEnd: services.commitVolume,
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
  }

  void _showWindows(
    BuildContext context,
    WidgetRef ref,
    List<ApplicationWindow> windows,
  ) {
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X Task View',
          keyName: 'win10x.task-view',
          builder: (context, handle) => Win10xFluent.wrap(
            Padding(
              padding: EdgeInsets.only(
                top: PreferenceController.instance.value.topInset,
                bottom: PreferenceController.instance.value.bottomInset,
              ),
              child: Center(
                child: Material(
                  color: Win10xFluent.chrome,
                  shape: Win10xFluent.square,
                  child: ConstrainedBox(
                    constraints: BoxConstraints(
                      maxWidth: 760,
                      maxHeight: math.min(
                        560,
                        MediaQuery.sizeOf(context).height -
                            PreferenceController.instance.value.topInset -
                            PreferenceController.instance.value.bottomInset -
                            24,
                      ),
                    ),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        ListTile(
                          title: const Text('任务视图'),
                          trailing: FluentIconButton(
                            tooltip: '关闭',
                            onPressed: handle.close,
                            icon: const MarkUiIcon(Icons.close),
                          ),
                        ),
                        Flexible(
                          child: windows.isEmpty
                              ? const Padding(
                                  padding: EdgeInsets.all(32),
                                  child: Text('没有打开的窗口'),
                                )
                              : GridView.builder(
                                  shrinkWrap: true,
                                  padding: const EdgeInsets.all(20),
                                  itemCount: windows.length,
                                  gridDelegate:
                                      const SliverGridDelegateWithMaxCrossAxisExtent(
                                        maxCrossAxisExtent: 320,
                                        childAspectRatio: 1.45,
                                        mainAxisSpacing: 16,
                                        crossAxisSpacing: 16,
                                      ),
                                  itemBuilder: (context, index) {
                                    final window = windows[index];
                                    return InkWell(
                                      onTap: () {
                                        handle.close();
                                        final box =
                                            context.findRenderObject()
                                                as RenderBox?;
                                        final origin = box == null
                                            ? Rect.zero
                                            : box.localToGlobal(Offset.zero) &
                                                  box.size;
                                        final outgoing = windows
                                            .where(
                                              (w) =>
                                                  w.active &&
                                                  !w.minimized &&
                                                  w.id != window.id,
                                            )
                                            .firstOrNull;
                                        unawaited(
                                          AppSwitchHandoff.activate(
                                            ref: ref,
                                            services: services,
                                            target: window,
                                            origin: origin,
                                            outgoing: outgoing,
                                            output: MediaQuery.sizeOf(context),
                                          ),
                                        );
                                      },
                                      child: Column(
                                        children: [
                                          Expanded(
                                            child: services.buildWindowPreview(
                                              context,
                                              window.id,
                                            ),
                                          ),
                                          Padding(
                                            padding: const EdgeInsets.all(8),
                                            child: Text(
                                              window.title,
                                              maxLines: 1,
                                              overflow: TextOverflow.ellipsis,
                                            ),
                                          ),
                                        ],
                                      ),
                                    );
                                  },
                                ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        );
  }

  void _showAppMenu(
    BuildContext context,
    WidgetRef ref,
    String id,
    List<ApplicationWindow> windows,
  ) {
    final controller = PreferenceController.instance;
    final pinned = controller.value.pins.any(
      (pin) => normalizedAppId(pin) == normalizedAppId(id),
    );
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X taskbar commands',
          builder: (context, handle) => Win10xFluent.wrap(
            FluentMenuSurface(
              placement: FluentMenuPlacement.bottomBar,
              children: [
                if (windows.isNotEmpty) ...[
                  FluentMenuItem(
                    label: '选择窗口',
                    onTap: () {
                      handle.close();
                      _showWindows(context, ref, windows);
                    },
                  ),
                  FluentMenuItem(
                    label: '关闭应用窗口',
                    danger: true,
                    onTap: () {
                      handle.close();
                      final ids = windows.map((window) => window.id).toSet();
                      for (final window
                          in ref.read(shellControllerProvider).openAppWindows) {
                        if (ids.contains(window.objectId)) {
                          ref
                              .read(shellControllerProvider.notifier)
                              .closeWindow(window);
                        }
                      }
                    },
                  ),
                ],
                if (id.isNotEmpty) ...[
                  FluentMenuItem(
                    label: '新建窗口',
                    onTap: () {
                      handle.close();
                      unawaited(
                        services.launchApplication(id, monitorId: monitorId),
                      );
                    },
                  ),
                  FluentMenuItem(
                    label: pinned ? '从任务栏取消固定' : '固定到任务栏',
                    onTap: () {
                      controller.change((prefs) {
                        if (pinned) {
                          prefs.pins.removeWhere(
                            (pin) =>
                                normalizedAppId(pin) == normalizedAppId(id),
                          );
                        } else {
                          prefs.pins.add(id);
                        }
                      });
                      handle.close();
                    },
                  ),
                ],
                FluentMenuItem(
                  label: '窗口布局',
                  trailing: const MarkUiIcon(
                    Icons.chevron_right,
                    size: 18,
                    color: Win10xFluent.textSecondary,
                  ),
                  onTap: () {
                    handle.close();
                    _layoutMenu(ref);
                  },
                ),
              ],
            ),
          ),
        );
  }

  void _layoutMenu(WidgetRef ref) {
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X window layout',
          builder: (context, handle) => Win10xFluent.wrap(
            FluentMenuSurface(
              placement: FluentMenuPlacement.bottomBar,
              children: [
                for (final entry in [
                  (DesktopWindowLayout.stacking, '堆叠窗口'),
                  (DesktopWindowLayout.dwindle, '自动分屏'),
                  (DesktopWindowLayout.scrolling, '滚动平铺'),
                ])
                  FluentMenuItem(
                    label: entry.$2,
                    onTap: () {
                      ref
                          .read(shellSettingsProvider.notifier)
                          .setDesktopWindowLayout(entry.$1);
                      handle.close();
                    },
                  ),
              ],
            ),
          ),
        );
  }
}
