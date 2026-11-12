// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';

import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/system_services.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'fluent_controls.dart';

class Win10xAudioSettings extends ConsumerStatefulWidget {
  const Win10xAudioSettings({super.key});
  @override
  ConsumerState<Win10xAudioSettings> createState() => _AudioSettingsState();
}

class _AudioSettingsState extends ConsumerState<Win10xAudioSettings> {
  @override
  void initState() {
    super.initState();
    Future.microtask(() {
      if (!mounted) return;
      ref.read(audioDevicesProvider.notifier).refresh();
      ref.read(appAudioProvider.notifier).refresh();
    });
  }

  @override
  Widget build(BuildContext context) {
    final devices = ref.watch(audioDevicesProvider);
    final apps = ref.watch(appAudioProvider);
    return ListView(
      children: [
        const Padding(padding: EdgeInsets.all(16), child: Text('声音输出')),
        if (devices.loading) const LinearProgressIndicator(),
        for (final device in devices.devices)
          FluentRadio<String>(
            value: device.name,
            groupValue: devices.activeDevice?.name,
            label: device.description,
            onChanged: device.available && !devices.changing
                ? ref.read(audioDevicesProvider.notifier).select
                : null,
          ),
        if (!devices.loading && devices.devices.isEmpty)
          const Padding(padding: EdgeInsets.all(16), child: Text('没有可用的输出设备')),
        const FluentSeparator(),
        const Padding(padding: EdgeInsets.all(16), child: Text('应用音量')),
        for (final app in apps.streams)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(app.name, maxLines: 1, overflow: TextOverflow.ellipsis),
                Slider(
                  value: app.level.clamp(0, 1),
                  label: app.name,
                  semanticFormatterCallback: (v) => '${(v * 100).round()}%',
                  onChanged: (v) =>
                      ref.read(appAudioProvider.notifier).setVolume(app.id, v),
                  onChangeEnd: (v) => ref
                      .read(appAudioProvider.notifier)
                      .commitVolume(app.id, v),
                ),
              ],
            ),
          ),
        if (!apps.loading && apps.streams.isEmpty)
          const Padding(padding: EdgeInsets.all(16), child: Text('没有正在播放的应用')),
        if (devices.error ?? apps.error case final error?)
          Padding(padding: const EdgeInsets.all(16), child: Text(error)),
      ],
    );
  }
}

class Win10xPerformanceSettings extends ConsumerWidget {
  const Win10xPerformanceSettings({super.key});
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final modes = ref.watch(desktopPowerModesProvider);
    final controller = ref.read(desktopPowerModesProvider.notifier);
    return ListView(
      children: [
        FluentMenuItem(
          label: '刷新电源模式',
          leading: const MarkUiIcon(Icons.refresh),
          onTap: modes.refreshing
              ? null
              : () => unawaited(controller.refresh()),
        ),
        if (modes.refreshing) const LinearProgressIndicator(),
        if (!modes.systemAvailable &&
            !modes.pboAvailable &&
            !modes.gpuAvailable &&
            !modes.refreshing)
          const Padding(
            padding: EdgeInsets.all(16),
            child: Text('此设备未提供可切换的性能模式'),
          ),
        if (modes.systemAvailable) ...[
          const Padding(padding: EdgeInsets.all(16), child: Text('系统电源模式')),
          for (final entry in {
            PowerProfile.powerSave: '省电',
            PowerProfile.balanced: '平衡',
            PowerProfile.performance: '高性能',
          }.entries)
            FluentRadio<String>(
              value: entry.key,
              groupValue: modes.systemProfile,
              label: entry.value,
              onChanged: modes.systemChanging
                  ? null
                  : (v) => unawaited(controller.selectSystemProfile(v)),
            ),
        ],
        if (modes.pboAvailable) ...[
          const FluentSeparator(),
          const Padding(padding: EdgeInsets.all(16), child: Text('处理器性能')),
          for (final entry in {
            DesktopPboProfile.silent: '安静',
            DesktopPboProfile.balanced: '平衡',
            DesktopPboProfile.performance: '性能',
          }.entries)
            FluentRadio<String>(
              value: entry.key,
              groupValue: modes.pboProfile,
              label: entry.value,
              onChanged: modes.pboChanging
                  ? null
                  : (v) => unawaited(controller.selectPboProfile(v)),
            ),
        ],
        if (modes.gpuAvailable) ...[
          const FluentSeparator(),
          const Padding(padding: EdgeInsets.all(16), child: Text('显卡性能')),
          for (final entry in {
            LactPerformancePreset.low: '节能',
            LactPerformancePreset.automatic: '自动',
            LactPerformancePreset.high: '性能',
          }.entries)
            FluentRadio<String>(
              value: entry.key,
              groupValue: modes.gpuPerformancePreset,
              label: entry.value,
              onChanged: modes.gpuChanging
                  ? null
                  : (v) => unawaited(controller.selectGpuPerformancePreset(v)),
            ),
        ],
        if (modes.error case final error?)
          Padding(padding: const EdgeInsets.all(16), child: Text(error)),
      ],
    );
  }
}
