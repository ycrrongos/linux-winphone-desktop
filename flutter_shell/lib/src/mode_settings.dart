// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';

import 'dual_screen.dart';
import 'fluent_controls.dart';
import 'model.dart';
import 'preferences.dart';
import 'settings_launch.dart';
import 'screen_settings.dart';

class ShellModeSettings extends StatelessWidget {
  const ShellModeSettings({super.key});
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: PreferenceController.instance,
    builder: (context, _) {
      final controller = PreferenceController.instance;
      final prefs = controller.value;
      return Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const ScreenSettings(),
          const SizedBox(height: 16),
          for (final mode in ShellMode.values)
            FluentRadio<ShellMode>(
              value: mode,
              groupValue: prefs.mode,
              label: shellModeLabels[mode.index],
              onChanged: (value) => controller.change((p) => p.mode = value),
            ),
          const SizedBox(height: 12),
          if (!prefs.isPhone) ...[
            Text('开始布局', style: Theme.of(context).textTheme.titleMedium),
            for (final layout in StartLayout.values)
              FluentRadio<StartLayout>(
                value: layout,
                groupValue: prefs.layout,
                label: layoutNames[layout.index],
                onChanged: (value) =>
                    controller.change((p) => p.layout = value),
              ),
          ],
          const SizedBox(height: 8),
          Text('开始磁贴动画', style: Theme.of(context).textTheme.titleSmall),
          for (final anim in StartAnimation.values)
            FluentRadio<StartAnimation>(
              value: anim,
              groupValue: prefs.startAnimation,
              label: startAnimationNames[anim.index],
              onChanged: (value) =>
                  controller.change((p) => p.startAnimation = value),
            ),
          const SizedBox(height: 8),
          Text('启动动画', style: Theme.of(context).textTheme.titleSmall),
          for (final anim in LaunchAnimation.values)
            FluentRadio<LaunchAnimation>(
              value: anim,
              groupValue: prefs.launchAnimation,
              label: launchAnimationNames[anim.index],
              onChanged: (value) =>
                  controller.change((p) => p.launchAnimation = value),
            ),
          FluentToggle(
            label: '应用切换动画',
            value: prefs.switchAnimation,
            onChanged: (value) =>
                controller.change((p) => p.switchAnimation = value),
          ),
          FluentToggle(
            label: '新亚克力帷幕（Denial 背景模糊）',
            value: prefs.acrylicEnabled,
            onChanged: (value) =>
                controller.change((p) => p.acrylicEnabled = value),
          ),
          if (prefs.isPhone) ...[
            FluentToggle(
              label: '用任务列表替换搜索按钮',
              value: prefs.phoneTaskButton,
              onChanged: (value) =>
                  controller.change((p) => p.phoneTaskButton = value),
            ),
            FluentToggle(
              label: '显示更多磁贴（4 → 6 列）',
              value: prefs.phoneMoreTiles,
              onChanged: (value) =>
                  controller.change((p) => p.phoneMoreTiles = value),
            ),
            FluentToggle(
              label: '固定时使用系统强调色',
              value: prefs.autoAccentFromIcon,
              onChanged: (value) =>
                  controller.change((p) => p.autoAccentFromIcon = value),
            ),
            const SizedBox(height: 12),
            Text('开始屏幕 + 主题', style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 4),
            Text(
              '开始屏幕使用当前桌面壁纸（Windows Phone 8.1 Start Background）。'
              '背景主题影响文字与系统界面深浅；强调色用于磁贴与链接。',
              style: Theme.of(context).textTheme.bodySmall,
            ),
            const SizedBox(height: 8),
            FluentRadio<bool>(
              value: true,
              groupValue: prefs.phoneThemeDark,
              label: '深色背景',
              onChanged: (value) =>
                  controller.change((p) => p.phoneThemeDark = true),
            ),
            FluentRadio<bool>(
              value: false,
              groupValue: prefs.phoneThemeDark,
              label: '浅色背景',
              onChanged: (value) =>
                  controller.change((p) => p.phoneThemeDark = false),
            ),
            const SizedBox(height: 8),
            Text('强调色', style: Theme.of(context).textTheme.titleSmall),
            const SizedBox(height: 8),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                for (var i = 0; i < phoneAccentArgb.length; i++)
                  GestureDetector(
                    onTap: () =>
                        controller.change((p) => p.phoneAccentIndex = i),
                    child: Container(
                      width: 36,
                      height: 36,
                      decoration: BoxDecoration(
                        color: Color(phoneAccentArgb[i]),
                        border: Border.all(
                          color: prefs.phoneAccentIndex == i
                              ? Colors.white
                              : Colors.black26,
                          width: prefs.phoneAccentIndex == i ? 3 : 1,
                        ),
                      ),
                      child: prefs.phoneAccentIndex == i
                          ? const MarkUiIcon(
                              Icons.check,
                              color: Colors.white,
                              size: 18,
                            )
                          : null,
                    ),
                  ),
              ],
            ),
            const SizedBox(height: 4),
            Text(
              phoneAccentLabels[prefs.phoneAccentIndex],
              style: TextStyle(color: Color(prefs.phoneAccentArgbValue)),
            ),
            const SizedBox(height: 12),
            Text('应用列表字号', style: Theme.of(context).textTheme.titleSmall),
            for (final size in UiFontSize.values)
              FluentRadio<UiFontSize>(
                value: size,
                groupValue: prefs.drawerFontSize,
                label: uiFontSizeLabels[size.index],
                onChanged: (value) =>
                    controller.change((p) => p.drawerFontSize = value),
              ),
            Text('磁贴标题字号', style: Theme.of(context).textTheme.titleSmall),
            for (final size in UiFontSize.values)
              FluentRadio<UiFontSize>(
                value: size,
                groupValue: prefs.tileFontSize,
                label: uiFontSizeLabels[size.index],
                onChanged: (value) =>
                    controller.change((p) => p.tileFontSize = value),
              ),
            const SizedBox(height: 8),
            FluentMenuItem(
              label: prefs.startWallpaperPath == null
                  ? '选择开始屏幕图片…'
                  : '更换开始屏幕图片…',
              subtitle: prefs.startWallpaperPath ?? '使用桌面壁纸',
              onTap: () => _pickStartWallpaper(controller),
            ),
            if (prefs.startWallpaperPath != null)
              FluentMenuItem(
                label: '恢复桌面壁纸',
                onTap: () =>
                    controller.change((p) => p.startWallpaperPath = null),
              ),
          ],
          const SizedBox(height: 16),
          FluentMenuItem(
            label: '重置开始屏幕',
            subtitle: '恢复默认「开始」分组，并按任务栏固定项重新铺磁贴',
            danger: true,
            onTap: () {
              controller.change(
                (p) => resetStartBoard(
                  p,
                  columns: p.isPhone
                      ? (p.phoneMoreTiles ? 8 : 6)
                      : (p.isTablet ? 8 : 6),
                ),
              );
            },
          ),
          if (DualScreenController.instance.active) ...[
            const SizedBox(height: 12),
            Text('双屏布局', style: Theme.of(context).textTheme.titleMedium),
            Text(
              '姿态：${DualScreenController.instance.geometry?.posture.name ?? '—'}'
              '（无铰链传感器时由输出几何推断）',
              style: Theme.of(context).textTheme.bodySmall,
            ),
            for (final mode in DualScreenLayoutMode.values)
              FluentRadio<DualScreenLayoutMode>(
                value: mode,
                groupValue: DualScreenController.instance.mode,
                label: switch (mode) {
                  DualScreenLayoutMode.extendedCanvas => '扩展画布',
                  DualScreenLayoutMode.listDetail => '列表–详情',
                  DualScreenLayoutMode.twoPage => '双页',
                  DualScreenLayoutMode.companionPane => '伴侣窗格',
                  DualScreenLayoutMode.dualView => '双视图',
                },
                onChanged: (value) =>
                    DualScreenController.instance.setMode(value),
              ),
          ],
          const SizedBox(height: 12),
          Text('Denial 系统设置', style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 4),
          Text(
            '打开 Denial Settings 对应页面（SettingsPageId）。'
            '外壳模式、开始/启动/切换动画与亚克力仍由上方 Win10X 项控制。',
            style: Theme.of(context).textTheme.bodySmall,
          ),
          const SizedBox(height: 8),
          for (final page in SettingsLaunch.allPages)
            FluentMenuItem(
              label: page.$2,
              subtitle: '--page=${page.$1}',
              onTap: () => unawaited(SettingsLaunch.open(page: page.$1)),
            ),
        ],
      );
    },
  );
}

Future<void> _pickStartWallpaper(PreferenceController controller) async {
  try {
    final result = await Process.run('zenity', [
      '--file-selection',
      '--title=选择开始屏幕图片',
      '--file-filter=图片 | *.png *.jpg *.jpeg *.webp *.bmp',
    ]);
    if (result.exitCode == 0) {
      final path = (result.stdout as String).trim();
      if (path.isNotEmpty) {
        controller.change((p) => p.startWallpaperPath = path);
      }
    }
  } on Object {
    try {
      final result = await Process.run('kdialog', [
        '--getopenfilename',
        '.',
        '*.png *.jpg *.jpeg *.webp *.bmp',
      ]);
      if (result.exitCode == 0) {
        final path = (result.stdout as String).trim();
        if (path.isNotEmpty) {
          controller.change((p) => p.startWallpaperPath = path);
        }
      }
    } on Object {
      // Picker unavailable — leave path unchanged.
    }
  }
}
