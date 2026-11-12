// SPDX-License-Identifier: GPL-3.0-only
import 'dart:io';

import 'package:denial_flutter_sdk/launcher.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'wp_popups.dart';
import 'wp_toast.dart';

/// Desktop-entry helpers: app info and best-effort uninstall via system tools.
abstract final class AppActions {
  static Future<void> showInfo(WidgetRef ref, LauncherApplication app) async {
    final desktop = await _desktopPath(app.id);
    if (!ref.context.mounted) return;
    final body = StringBuffer()
      ..writeln(app.name)
      ..writeln(app.id);
    if (desktop != null) body.writeln(desktop);
    final openLocation = desktop != null;
    final result = await WpPopups.messageBox(
      ref,
      title: '应用信息',
      body: body.toString().trim(),
      primary: openLocation ? '打开文件位置' : '关闭',
      secondary: openLocation ? '关闭' : '取消',
    );
    if (result == true && desktop != null) {
      if (!await _tryDetached('xdg-open', [File(desktop).parent.path]) &&
          ref.context.mounted) {
        WpToast.show(ref, '无法打开文件位置');
      }
    }
  }

  static Future<void> uninstall(WidgetRef ref, LauncherApplication app) async {
    final confirmed = await WpPopups.messageBox(
      ref,
      title: '卸载应用',
      body:
          '确定要卸载「${app.name}」吗？将尝试打开系统软件管理器；'
          '若系统不支持，请在发行版软件中心中手动卸载。',
      primary: '卸载',
      secondary: '取消',
      destructive: true,
    );
    if (confirmed != true) return;
    final desktop = await _desktopPath(app.id);
    final launched = await _launchUninstaller(app, desktop);
    if (!ref.context.mounted) return;
    WpToast.show(ref, launched ? '已打开软件管理器，请选择卸载' : '无法自动卸载，请在系统软件中心中操作');
  }

  static Future<String?> _desktopPath(String appId) async {
    final name = appId.replaceFirst(RegExp(r'^desktop:'), '').split('/').last;
    final candidates = <String>[
      if (name.endsWith('.desktop')) name else '$name.desktop',
    ];
    final home = Platform.environment['HOME'] ?? '';
    final dirs = [
      '$home/.local/share/applications',
      '/usr/share/applications',
      '/usr/local/share/applications',
      '/var/lib/flatpak/exports/share/applications',
      '$home/.local/share/flatpak/exports/share/applications',
    ];
    for (final dir in dirs) {
      for (final file in candidates) {
        final path = '$dir/$file';
        if (await File(path).exists()) return path;
      }
    }
    return null;
  }

  static Future<bool> _launchUninstaller(
    LauncherApplication app,
    String? desktop,
  ) async {
    final id = app.id.toLowerCase();
    if (id.contains('flatpak') || (desktop?.contains('/flatpak/') ?? false)) {
      final flatpakId = _flatpakId(desktop) ?? app.id;
      return await _tryDetached('gnome-software', ['--details=$flatpakId']) ||
          await _tryDetached('plasma-discover', ['--application=$flatpakId']);
    }
    if (desktop != null) {
      if (await _tryDetached('plasma-discover', ['--application=${app.id}']) ||
          await _tryDetached('gnome-software', [])) {
        return true;
      }
    }
    return await _tryDetached('gnome-software', []) ||
        await _tryDetached('plasma-discover', []);
  }

  static String? _flatpakId(String? desktopPath) {
    if (desktopPath == null) return null;
    final base = File(desktopPath).uri.pathSegments.last;
    return base.replaceFirst(RegExp(r'\.desktop$'), '');
  }

  static Future<bool> _tryDetached(String exe, List<String> args) async {
    try {
      final result = await Process.start(
        exe,
        args,
        mode: ProcessStartMode.detached,
      );
      return result.pid > 0;
    } on Object {
      return false;
    }
  }
}
