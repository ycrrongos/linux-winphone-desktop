// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';
import 'phone_tasks.dart';
import 'fluent_motion.dart';
import 'model.dart';

import 'dart:async';

import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/popups.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app_switch.dart';
import 'fluent.dart';
import 'preferences.dart';
import 'symbols.dart';
import 'shell_navigation.dart';
import 'status_bar.dart';
import 'wp_popups.dart';

class PhoneNavigationBar extends ConsumerWidget {
  const PhoneNavigationBar({
    super.key,
    required this.services,
    required this.monitorId,
  });
  final ShellServices services;
  final int monitorId;
  @override
  Widget build(BuildContext context, WidgetRef ref) => ListenableBuilder(
    listenable: PreferenceController.instance,
    builder: (context, _) => _build(context, ref),
  );

  Widget _build(BuildContext context, WidgetRef ref) {
    Widget key(
      ShellSymbol glyph,
      String label,
      VoidCallback tap,
      VoidCallback? hold,
    ) => Expanded(
      child: Semantics(
        button: true,
        label: label,
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: tap,
          onLongPress: hold,
          child: SizedBox.expand(
            child: Center(
              child: ShellGlyph(glyph, color: Colors.white, size: 22),
            ),
          ),
        ),
      ),
    );
    return Material(
      color: Colors.black,
      child: Row(
        children: [
          key(ShellSymbol.back, '返回；长按切换应用', () {
            final popups = ref.read(shellPopupControllerProvider);
            if (popups.isNotEmpty) {
              if (WpPopups.dismiss(popups.last.id)) return;
              ref
                  .read(shellPopupControllerProvider.notifier)
                  .close(popups.last.id);
              return;
            }
            ShellNavigation.instance.back(services);
          }, () => _tasks(ref)),
          key(
            ShellSymbol.start,
            '开始；长按单手模式',
            () {
              ref
                  .read(shellPopupControllerProvider.notifier)
                  .dismissAllImmediately();
              ShellNavigation.instance.home(services);
            },
            () => ShellNavigation.instance.setReachability(
              !ShellNavigation.instance.oneHanded,
            ),
          ),
          key(
            PreferenceController.instance.value.phoneTaskButton
                ? ShellSymbol.tasks
                : ShellSymbol.search,
            PreferenceController.instance.value.phoneTaskButton
                ? '任务列表'
                : '搜索应用',
            () {
              if (PreferenceController.instance.value.phoneTaskButton) {
                _tasks(ref);
                return;
              }
              ref
                  .read(shellPopupControllerProvider.notifier)
                  .dismissAllImmediately();
              ShellNavigation.instance.search(services);
            },
            null,
          ),
        ],
      ),
    );
  }

  void _tasks(WidgetRef ref) {
    if (ref
        .read(shellPopupControllerProvider)
        .any((surface) => surface.debugLabel == 'Win10X phone task switcher')) {
      return;
    }
    ref.read(shellPopupControllerProvider.notifier).dismissAllImmediately();
    ShellNavigation.instance.setReachability(false);
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X phone task switcher',
          keyName: 'win10x.phone-tasks',
          transitionDuration: FluentMotion.slow,
          builder: (context, handle) {
            final prefs = PreferenceController.instance.value;
            return Win10xFluent.wrapPhone(
              Stack(
                fit: StackFit.expand,
                children: [
                  PhoneTaskSwitcher(
                    services: services,
                    monitorId: monitorId,
                    onClose: handle.close,
                  ),
                  Positioned(
                    left: 0,
                    right: 0,
                    top: 0,
                    height: 24,
                    child: Win10xStatusBar(services: services),
                  ),
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
              dark: prefs.phoneThemeDark,
              accentColor: Color(prefs.phoneAccentArgbValue),
            );
          },
        );
  }
}

class PhoneTaskSwitcher extends ConsumerWidget {
  const PhoneTaskSwitcher({
    super.key,
    required this.services,
    required this.monitorId,
    required this.onClose,
  });
  final ShellServices services;
  final int monitorId;
  final VoidCallback onClose;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final windows = ref.watch(services.windows(monitorId));
    final catalog = ref.watch(services.applications);
    return PhoneTaskBoard(
      windows: windows,
      accent: Color(PreferenceController.instance.value.phoneAccentArgbValue),
      labelFor: (window) =>
          catalog
              .where(
                (app) => app.windowAppIds.any(
                  (id) => normalizedAppId(id) == normalizedAppId(window.appId),
                ),
              )
              .firstOrNull
              ?.name ??
          (window.title.isNotEmpty ? window.title : window.appId),
      buildIcon: (context, id) => buildMarkUiApplicationIcon(
        context,
        id,
        services.buildApplicationIcon,
        color:
            Color(PreferenceController.instance.value.phoneAccentArgbValue)
                    .computeLuminance() >
                .4
            ? Colors.black
            : Colors.white,
      ),
      buildPreview: services.buildWindowPreview,
      onHome: () {
        onClose();
        ShellNavigation.instance.revealHome(services);
      },
      onClose: (id) {
        final window = ref.read(shellControllerProvider).windowByObjectId(id);
        if (window != null) {
          ref.read(shellControllerProvider.notifier).closeWindow(window);
        }
      },
      onActivate: (window, origin) {
        onClose();
        ShellNavigation.instance.closePanels?.call();
        unawaited(
          AppSwitchHandoff.activate(
            ref: ref,
            services: services,
            target: window,
            origin: origin,
            outgoing: windows
                .where((w) => w.active && !w.minimized && w.id != window.id)
                .firstOrNull,
            output: MediaQuery.sizeOf(context),
          ),
        );
      },
    );
  }
}
