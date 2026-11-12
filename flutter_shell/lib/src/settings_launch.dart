// SPDX-License-Identifier: GPL-3.0-only
// Launch Denial Settings with optional --page= (SettingsPageId.name).
import 'dart:io';

/// Page names match Denial `SettingsPageId` exactly (see
/// `settings_navigation.dart` in the pinned kit).
abstract final class SettingsLaunch {
  static const you = 'you';
  static const appearance = 'appearance';
  static const language = 'language';
  static const keyboard = 'keyboard';
  static const touchpad = 'touchpad';
  static const shortcuts = 'shortcuts';
  static const environment = 'environment';
  static const animations = 'animations';
  static const layout = 'layout';
  static const overlays = 'overlays';
  static const lockScreen = 'lockScreen';
  static const fingerprint = 'fingerprint';
  static const permissions = 'permissions';
  static const audio = 'audio';
  static const displays = 'displays';
  static const network = 'network';
  static const bluetooth = 'bluetooth';
  static const power = 'power';
  static const developer = 'developer';
  static const about = 'about';

  /// Every Denial Settings page, in host navigation order (DS1–DS20).
  static const allPages = <(String id, String label)>[
    (you, '你'),
    (appearance, '外观'),
    (language, '语言'),
    (keyboard, '键盘'),
    (touchpad, '触控板'),
    (shortcuts, '快捷键'),
    (environment, '环境'),
    (animations, '动画'),
    (layout, '布局'),
    (overlays, '叠加层'),
    (lockScreen, '锁屏'),
    (fingerprint, '指纹'),
    (permissions, '权限'),
    (audio, '音频'),
    (displays, '显示器'),
    (network, '网络'),
    (bluetooth, '蓝牙'),
    (power, '电源'),
    (developer, '开发者'),
    (about, '关于'),
  ];

  static Future<bool> open({String? page}) async {
    final binary = Platform.environment['DENIAL_SETTINGS_BINARY']?.trim();
    final executable = (binary == null || binary.isEmpty)
        ? 'denial-settings'
        : binary;
    try {
      final process = await Process.start(executable, [
        if (page != null && page.isNotEmpty) '--page=$page',
      ], mode: ProcessStartMode.detached);
      return process.pid > 0;
    } on Object {
      try {
        final process = await Process.start('denial-settings', [
          if (page != null && page.isNotEmpty) '--page=$page',
        ], mode: ProcessStartMode.detached);
        return process.pid > 0;
      } on Object {
        return false;
      }
    }
  }
}
