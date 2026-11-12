// SPDX-License-Identifier: GPL-3.0-only
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:denial_flutter_sdk/popups.dart';

import 'wallpaper_picker.dart';
import 'fluent.dart';

import 'fluent_controls.dart';
import 'preferences.dart';

class ScreenSettings extends ConsumerStatefulWidget {
  const ScreenSettings({super.key});
  @override
  ConsumerState<ScreenSettings> createState() => _ScreenSettingsState();
}

class _ScreenSettingsState extends ConsumerState<ScreenSettings> {
  late final off = TextEditingController(
    text: PreferenceController.instance.value.screenOffCommand,
  );
  late final on = TextEditingController(
    text: PreferenceController.instance.value.screenOnCommand,
  );
  String? status;
  @override
  void dispose() {
    off.dispose();
    on.dispose();
    super.dispose();
  }

  void wallpaper() {
    ref
        .read(shellPopupControllerProvider.notifier)
        .show(
          debugLabel: 'Win10X wallpaper picker',
          barrierColor: Colors.transparent,
          builder: (context, handle) => Win10xFluent.wrapPhone(
            WallpaperPicker(
              onClose: handle.close,
              onPicked: (path) {
                PreferenceController.instance.change(
                  (p) => p.desktopWallpaperPath = path,
                );
                handle.close();
              },
            ),
          ),
        );
  }

  Future<void> authorize() async {
    final c = PreferenceController.instance;
    await c.flush();
    try {
      final result = await Process.run('pkexec', [
        '/usr/local/libexec/win10x-screen-hooks',
        'install',
        c.file.path,
      ]);
      if (mounted) {
        setState(
          () => status = result.exitCode == 0
              ? '已授权保存的两条命令；修改命令后需重新授权'
              : '授权未完成：${result.stderr.toString().trim()}',
        );
      }
    } on Object catch (e) {
      if (mounted) setState(() => status = '授权工具不可用：$e');
    }
  }

  @override
  Widget build(BuildContext context) {
    final c = PreferenceController.instance, p = c.value;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text('屏幕与壁纸', style: Theme.of(context).textTheme.titleMedium),
        FluentMenuItem(
          label: '更换桌面与锁屏壁纸…',
          subtitle: p.desktopWallpaperPath ?? '使用系统壁纸',
          onTap: wallpaper,
        ),
        if (p.desktopWallpaperPath != null)
          FluentMenuItem(
            label: '恢复系统壁纸',
            onTap: () => c.change((p) => p.desktopWallpaperPath = null),
          ),
        FluentToggle(
          label: '自动熄屏',
          value: p.autoScreenOff,
          onChanged: (v) => c.change((p) => p.autoScreenOff = v),
        ),
        if (p.autoScreenOff)
          Wrap(
            children: [
              for (final seconds in [30, 60, 120, 300, 600, 900, 1800, 3600])
                TextButton(
                  onPressed: () =>
                      c.change((p) => p.screenOffSeconds = seconds),
                  child: Text(
                    '${seconds < 60 ? '$seconds 秒' : '${seconds ~/ 60} 分钟'}${p.screenOffSeconds == seconds ? ' ✓' : ''}',
                  ),
                ),
            ],
          ),
        Text('熄屏时执行', style: Theme.of(context).textTheme.bodyMedium),
        TextField(
          controller: off,
          maxLines: 2,
          autocorrect: false,
          onChanged: (v) => c.change((p) => p.screenOffCommand = v),
          decoration: const InputDecoration(hintText: '留空不执行；Shell 命令'),
        ),
        const SizedBox(height: 8),
        Text('亮屏时执行', style: Theme.of(context).textTheme.bodyMedium),
        TextField(
          controller: on,
          maxLines: 2,
          autocorrect: false,
          onChanged: (v) => c.change((p) => p.screenOnCommand = v),
          decoration: const InputDecoration(hintText: '留空不执行；Shell 命令'),
        ),
        FluentToggle(
          label: '以管理员身份执行这两条命令',
          value: p.screenHooksRoot,
          onChanged: (v) => c.change((p) => p.screenHooksRoot = v),
        ),
        if (p.screenHooksRoot) ...[
          const Text('授权只保存当前两条命令。运行时免密，修改命令后需重新授权。'),
          TextButton(onPressed: authorize, child: const Text('授权当前亮灭屏命令…')),
        ],
        if (status != null) Text(status!),
      ],
    );
  }
}
