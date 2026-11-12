// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'dart:async';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'dashboard_reveal.dart';
import 'fluent.dart';
import 'fluent_motion.dart';
import 'controls.dart';
import 'mode_settings.dart';
import 'model.dart';
import 'notification_feed.dart';
import 'preferences.dart';
import 'shell_navigation.dart';
import 'system_details.dart';
import 'settings_launch.dart';
import 'wp_popups.dart';

class Win10xActionCenter extends ConsumerStatefulWidget {
  const Win10xActionCenter({
    super.key,
    required this.services,
    required this.onClose,
    this.embedded = false,
    this.onOpenSettings,
  });
  final ShellServices services;
  final VoidCallback onClose;
  final bool embedded;
  final VoidCallback? onOpenSettings;
  @override
  ConsumerState<Win10xActionCenter> createState() => _ActionCenterState();
}

class _ActionCenterState extends ConsumerState<Win10xActionCenter> {
  String? _detail;
  bool _expanded = false;
  int _detailRevision = -1;
  double _dismissTravel = 0, _dismissOrigin = 1;
  bool _dismissActive = false, _dismissCancelled = false;
  double _dismissExtent(bool phone) {
    final size = MediaQuery.sizeOf(context),
        p = PreferenceController.instance.value;
    final geometry = actionCenterGeometry(p, size.width, size.height);
    return math.max(1, phone ? geometry.height : geometry.width);
  }

  void _cancelDismiss(bool phone) {
    if (!_dismissActive) return;
    _dismissActive = false;
    final open = (_dismissOrigin - _dismissTravel / _dismissExtent(phone))
        .clamp(0.0, 1.0);
    unawaited(
      DashboardReveal.settleClose(
        services: widget.services,
        from: open,
        closeVelocity: 0,
        cancelled: true,
      ),
    );
  }

  @override
  void initState() {
    super.initState();
    ShellNavigation.instance.addListener(_requestedDetail);
    _detail = ShellNavigation.instance.actionDetail;
    Future.microtask(() {
      if (mounted) {
        unawaited(
          ref.read(desktopPowerModesProvider.notifier).refreshIfStale(),
        );
      }
    });
  }

  void _requestedDetail() {
    final nav = ShellNavigation.instance;
    if (nav.detailRevision == _detailRevision) return;
    _detailRevision = nav.detailRevision;
    if (mounted) setState(() => _detail = nav.actionDetail);
  }

  @override
  void dispose() {
    ShellNavigation.instance.removeListener(_requestedDetail);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final prefs = PreferenceController.instance.value;
    final phone = prefs.isPhone;
    final color = phone ? const Color(0xff101010) : Win10xFluent.chrome;
    Widget panel = Material(
      color: color,
      child: _detail == null
          ? _main(context, phone)
          : Column(
              children: [
                ListTile(
                  leading: FluentIconButton(
                    tooltip: '返回',
                    onPressed: () => setState(() => _detail = null),
                    icon: const MarkUiIcon(Icons.arrow_back),
                  ),
                  title: Text(
                    {
                          'wifi': 'Wi-Fi',
                          'bluetooth': '蓝牙',
                          'audio': '声音',
                          'performance': '性能模式',
                          'modes': '模式与个性化',
                          'power': '电源',
                        }[_detail] ??
                        '操作中心',
                  ),
                  trailing: FluentIconButton(
                    tooltip: '关闭',
                    onPressed: widget.onClose,
                    icon: const MarkUiIcon(Icons.close),
                  ),
                ),
                Expanded(
                  child: switch (_detail) {
                    'wifi' => _wifi(),
                    'bluetooth' => _bluetooth(),
                    'audio' => Column(
                      children: [
                        _volume(),
                        const Expanded(child: Win10xAudioSettings()),
                      ],
                    ),
                    'performance' => const Win10xPerformanceSettings(),
                    'modes' => const SingleChildScrollView(
                      child: ShellModeSettings(),
                    ),
                    _ => _power(),
                  },
                ),
              ],
            ),
    );
    // Finger-follow dismiss: phone pulls up, desktop/tablet pulls right.
    panel = Listener(
      onPointerDown: (_) => _dismissCancelled = false,
      onPointerCancel: (_) {
        if (_dismissActive) {
          _dismissCancelled = true;
          _cancelDismiss(phone);
        }
      },
      child: GestureDetector(
        onVerticalDragStart: phone
            ? (_) {
                _dismissActive = true;
                _dismissTravel = 0;
                _dismissOrigin =
                    ShellNavigation.instance.dashboardReveal.value ?? 1;
                DashboardReveal.setProgress(_dismissOrigin);
              }
            : null,
        onVerticalDragUpdate: phone
            ? (d) {
                _dismissTravel += -(d.primaryDelta ?? 0);
                final extent = _dismissExtent(true);
                final open = (_dismissOrigin - _dismissTravel / extent).clamp(
                  0.0,
                  1.0,
                );
                DashboardReveal.setProgress(open);
              }
            : null,
        onVerticalDragCancel: phone ? () => _cancelDismiss(true) : null,
        onVerticalDragEnd: phone
            ? (d) {
                if (_dismissCancelled) return;
                final extent = _dismissExtent(true);
                final open = (_dismissOrigin - _dismissTravel / extent).clamp(
                  0.0,
                  1.0,
                );
                _dismissActive = false;
                final closeVelocity = -(d.primaryVelocity ?? 0);
                unawaited(
                  DashboardReveal.settleClose(
                    services: widget.services,
                    from: open,
                    closeVelocity: closeVelocity,
                  ),
                );
              }
            : null,
        onHorizontalDragStart: phone
            ? null
            : (_) {
                _dismissActive = true;
                _dismissTravel = 0;
                _dismissOrigin =
                    ShellNavigation.instance.dashboardReveal.value ?? 1;
                DashboardReveal.setProgress(_dismissOrigin);
              },
        onHorizontalDragUpdate: phone
            ? null
            : (d) {
                _dismissTravel += d.primaryDelta ?? 0;
                final extent = _dismissExtent(false);
                final open = (_dismissOrigin - _dismissTravel / extent).clamp(
                  0.0,
                  1.0,
                );
                DashboardReveal.setProgress(open);
              },
        onHorizontalDragCancel: phone ? null : () => _cancelDismiss(false),
        onHorizontalDragEnd: phone
            ? null
            : (d) {
                if (_dismissCancelled) return;
                _dismissActive = false;
                final extent = _dismissExtent(false);
                final open = (_dismissOrigin - _dismissTravel / extent).clamp(
                  0.0,
                  1.0,
                );
                unawaited(
                  DashboardReveal.settleClose(
                    services: widget.services,
                    from: open,
                    closeVelocity: d.primaryVelocity ?? 0,
                  ),
                );
              },
        child: panel,
      ),
    );
    if (widget.embedded) return panel;
    final keyboard = MediaQuery.viewInsetsOf(context).bottom;
    return Align(
      alignment: phone ? Alignment.topCenter : Alignment.bottomRight,
      child: Padding(
        padding: EdgeInsets.only(
          top: prefs.topInset,
          bottom: prefs.bottomInset + keyboard,
        ),
        child: SizedBox(
          width: phone
              ? MediaQuery.sizeOf(context).width
              : math.min(380, MediaQuery.sizeOf(context).width),
          height:
              MediaQuery.sizeOf(context).height -
              prefs.topInset -
              prefs.bottomInset -
              keyboard,
          child: panel,
        ),
      ),
    );
  }

  Widget _main(BuildContext context, bool phone) {
    final notifications = ref.watch(desktopNotificationsProvider);
    final actions = ref.read(desktopNotificationsProvider.notifier);
    final clock = ref.watch(widget.services.clock).asData?.value;
    Widget commands() => Row(
      children: [
        TextButton(
          onPressed: notifications.groups.isEmpty ? null : actions.clearAll,
          child: const Text('清除所有通知'),
        ),
        const Spacer(),
        TextButton(
          onPressed: () => setState(() => _expanded = !_expanded),
          child: Text(_expanded ? '折叠' : '展开'),
        ),
      ],
    );
    final media = ref.watch(widget.services.media).asData?.value;
    Widget footer() => Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        AnimatedSize(
          duration: MediaQuery.disableAnimationsOf(context)
              ? Duration.zero
              : FluentMotion.slow,
          curve: FluentMotion.standard,
          alignment: Alignment.topCenter,
          child: _quickGrid(phone),
        ),
        _brightness(),
        _volume(),
        if (_expanded && media != null && media.available)
          ListTile(
            title: Text(
              media.title,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
            subtitle: Text(
              media.artistLabel,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
            ),
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                FluentIconButton(
                  tooltip: '上一首',
                  onPressed: () => unawaited(
                    ref.read(widget.services.mediaCommands).previous(),
                  ),
                  icon: const MarkUiIcon(Icons.skip_previous),
                ),
                FluentIconButton(
                  tooltip: media.playing ? '暂停' : '播放',
                  onPressed: () => unawaited(
                    ref.read(widget.services.mediaCommands).playPause(),
                  ),
                  icon: MarkUiIcon(
                    media.playing ? Icons.pause : Icons.play_arrow,
                  ),
                ),
                FluentIconButton(
                  tooltip: '下一首',
                  onPressed: () =>
                      unawaited(ref.read(widget.services.mediaCommands).next()),
                  icon: const MarkUiIcon(Icons.skip_next),
                ),
              ],
            ),
          ),
        if (_expanded && !phone)
          TextButton(
            onPressed: () => setState(() => _detail = 'audio'),
            child: const Text('声音输出与应用音量'),
          ),
      ],
    );
    return Column(
      children: [
        if (phone)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
            child: Row(
              children: [
                const Text('操作中心'),
                const Spacer(),
                if (clock != null) Text('${clock.month}/${clock.day}'),
              ],
            ),
          )
        else
          Row(
            children: [
              const SizedBox(width: 12),
              const Text('通知'),
              const Spacer(),
              TextButton(
                onPressed: () {
                  widget.onClose();
                  unawaited(
                    SettingsLaunch.open(page: SettingsLaunch.permissions),
                  );
                },
                child: const Text('管理通知'),
              ),
              FluentIconButton(
                tooltip: '关闭',
                onPressed: widget.onClose,
                icon: const MarkUiIcon(Icons.close),
              ),
            ],
          ),
        if (phone)
          Flexible(
            fit: FlexFit.loose,
            child: SingleChildScrollView(child: footer()),
          ),
        if (phone) commands(),
        const Expanded(flex: 2, child: NotificationFeed()),
        if (!phone) commands(),
        if (!phone)
          Flexible(
            fit: FlexFit.loose,
            flex: 2,
            child: SingleChildScrollView(child: footer()),
          ),
        if (phone)
          SizedBox(
            height: 32,
            child: TextButton(
              onPressed: widget.onClose,
              child: const MarkUiIcon(Icons.keyboard_arrow_up),
            ),
          ),
      ],
    );
  }

  Widget _quickGrid(bool phone) {
    final network = ref.watch(networkConnectivityProvider);
    final bluetooth = ref.watch(bluetoothProvider);
    final notices = ref.watch(desktopNotificationsProvider);
    final prefs = PreferenceController.instance.value;
    Widget tile(
      IconData icon,
      String label,
      bool selected,
      VoidCallback? tap, {
      VoidCallback? hold,
    }) => Semantics(
      button: true,
      label: label,
      toggled: selected,
      enabled: tap != null,
      child: Material(
        color: selected
            ? Win10xFluent.accent
            : phone
            ? const Color(0xff292929)
            : const Color(0xffdedede),
        child: InkWell(
          onTap: tap,
          onLongPress: hold,
          child: Padding(
            padding: const EdgeInsets.all(8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                MarkUiIcon(
                  icon,
                  color: phone || selected ? Colors.white : Win10xFluent.text,
                  size: 20,
                ),
                const Spacer(),
                Text(
                  label,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontSize: 11,
                    color: phone || selected ? Colors.white : Win10xFluent.text,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
    final list = [
      if (!phone)
        tile(Icons.tablet_mac, '平板模式', prefs.isTablet, () {
          widget.onClose();
          PreferenceController.instance.change(
            (p) => p.mode = p.isTablet ? ShellMode.desktop : ShellMode.tablet,
          );
        }),
      tile(
        Icons.wifi,
        'Wi-Fi',
        network.snapshot.wirelessEnabled,
        network.snapshot.wifiDeviceAvailable && !network.radioChanging
            ? () => unawaited(
                ref.read(networkConnectivityProvider.notifier).toggleWireless(),
              )
            : null,
        hold: () {
          setState(() => _detail = 'wifi');
        },
      ),
      tile(
        Icons.bluetooth,
        '蓝牙',
        bluetooth.powered,
        bluetooth.available && !bluetooth.powerChanging
            ? () =>
                  unawaited(ref.read(bluetoothProvider.notifier).togglePower())
            : null,
        hold: () {
          setState(() => _detail = 'bluetooth');
        },
      ),
      tile(Icons.desktop_windows_outlined, '显示器', false, () {
        widget.onClose();
        unawaited(SettingsLaunch.open(page: SettingsLaunch.displays));
      }),
      tile(Icons.travel_explore_rounded, '网络设置', false, () {
        widget.onClose();
        unawaited(SettingsLaunch.open(page: SettingsLaunch.network));
      }),
      tile(Icons.bluetooth_rounded, '蓝牙设置', false, () {
        widget.onClose();
        unawaited(SettingsLaunch.open(page: SettingsLaunch.bluetooth));
      }),
      tile(
        Icons.do_not_disturb_on_outlined,
        '免打扰',
        notices.doNotDisturb,
        ref.read(desktopNotificationsProvider.notifier).toggleDoNotDisturb,
      ),
      tile(
        Icons.settings_outlined,
        '所有设置',
        false,
        () {
          widget.onClose();
          widget.onOpenSettings?.call();
        },
        hold: () {
          widget.onClose();
          unawaited(SettingsLaunch.open(page: SettingsLaunch.appearance));
        },
      ),
      tile(
        Icons.volume_up_outlined,
        '声音',
        false,
        () => setState(() => _detail = 'audio'),
        hold: () {
          widget.onClose();
          unawaited(SettingsLaunch.open(page: SettingsLaunch.audio));
        },
      ),
      tile(
        Icons.speed,
        '性能模式',
        false,
        () => setState(() => _detail = 'performance'),
      ),
      tile(
        Icons.power_settings_new,
        '电源',
        false,
        () => setState(() => _detail = 'power'),
        hold: () {
          widget.onClose();
          (widget.onOpenSettings ?? widget.services.openPowerSettings)();
          unawaited(SettingsLaunch.open(page: SettingsLaunch.power));
        },
      ),
      tile(
        Icons.tablet_mac,
        '切换模式',
        false,
        () => setState(() => _detail = 'modes'),
        hold: () {
          widget.onClose();
          unawaited(SettingsLaunch.open(page: SettingsLaunch.animations));
        },
      ),
      tile(Icons.lock_outline_rounded, '锁屏', false, () {
        widget.onClose();
        unawaited(SettingsLaunch.open(page: SettingsLaunch.lockScreen));
      }),
      tile(Icons.tune, '个性化', false, () {
        widget.onClose();
        unawaited(SettingsLaunch.open(page: SettingsLaunch.appearance));
      }),
    ];
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 8, 8, 4),
      child: GridView.count(
        crossAxisCount: 4,
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        mainAxisSpacing: 4,
        crossAxisSpacing: 4,
        childAspectRatio: 1.25,
        children: _expanded ? list : list.take(4).toList(),
      ),
    );
  }

  Widget _brightness() {
    final layout = ref.watch(displayLayoutProvider);
    final output = layout?.mainOutput;
    final state = ref.watch(displayBrightnessProvider);
    final value = output == null ? null : state.levels[output.monitorId];
    if (output == null || value == null) return const SizedBox();
    return Row(
      children: [
        const Padding(
          padding: EdgeInsets.all(12),
          child: MarkUiIcon(Icons.brightness_6_outlined),
        ),
        Expanded(
          child: Slider(
            value: value.clamp(0, 1),
            label: '亮度',
            onChanged: (v) => ref
                .read(displayBrightnessProvider.notifier)
                .setLevel(output, v),
            onChangeEnd: (v) => ref
                .read(displayBrightnessProvider.notifier)
                .commitLevel(output, v),
          ),
        ),
      ],
    );
  }

  Widget _volume() => Row(
    children: [
      const Padding(
        padding: EdgeInsets.all(12),
        child: MarkUiIcon(Icons.volume_up_outlined),
      ),
      Expanded(
        child: Slider(
          value: ref.watch(widget.services.volume).clamp(0, 1),
          onChangeStart: (_) => widget.services.beginVolumeInteraction(),
          onChanged: widget.services.setVolume,
          onChangeEnd: widget.services.commitVolume,
        ),
      ),
    ],
  );
  Widget _wifi() {
    final state = ref.watch(networkConnectivityProvider);
    return ListView(
      children: [
        ListTile(
          title: const Text('搜索网络'),
          trailing: FluentIconButton(
            tooltip: '刷新',
            icon: const MarkUiIcon(Icons.refresh),
            onPressed: state.scanning
                ? null
                : () => unawaited(
                    ref.read(networkConnectivityProvider.notifier).scan(),
                  ),
          ),
        ),
        for (final network in state.snapshot.networks)
          ListTile(
            leading: MarkUiIcon(
              network.connected ? Icons.wifi : Icons.wifi_outlined,
            ),
            title: Text(network.ssid),
            subtitle: Text(network.connected ? '已连接' : network.security.label),
          ),
        if (state.error != null)
          Padding(padding: const EdgeInsets.all(16), child: Text(state.error!)),
        ListTile(
          title: const Text('更多网络设置'),
          onTap: () {
            widget.onClose();
            (widget.onOpenSettings ?? widget.services.openPowerSettings)();
          },
        ),
      ],
    );
  }

  Widget _bluetooth() {
    final state = ref.watch(bluetoothProvider);
    return ListView(
      children: [
        ListTile(
          title: const Text('搜索设备'),
          trailing: FluentIconButton(
            tooltip: '搜索',
            icon: const MarkUiIcon(Icons.refresh),
            onPressed: state.powered
                ? () => unawaited(ref.read(bluetoothProvider.notifier).scan())
                : null,
          ),
        ),
        for (final device in state.devices)
          ListTile(
            title: Text(device.name),
            subtitle: Text(
              device.connected
                  ? '已连接'
                  : device.paired
                  ? '已配对'
                  : device.address,
            ),
            trailing: device.paired
                ? TextButton(
                    onPressed: () => unawaited(
                      ref
                          .read(bluetoothProvider.notifier)
                          .toggleConnection(device),
                    ),
                    child: Text(device.connected ? '断开' : '连接'),
                  )
                : null,
          ),
        if (state.error != null)
          Padding(padding: const EdgeInsets.all(16), child: Text(state.error!)),
      ],
    );
  }

  Widget _power() {
    final power = ref.watch(sessionPowerProvider);
    final controller = ref.read(sessionPowerProvider.notifier);
    const names = {
      SessionPowerAction.lock: '锁定',
      SessionPowerAction.suspend: '睡眠',
      SessionPowerAction.logout: '退出登录',
      SessionPowerAction.reboot: '重启',
      SessionPowerAction.powerOff: '关机',
    };
    Future<void> request(SessionPowerAction action) async {
      final availability = power.availabilityFor(action);
      if (!availability.enabled) return;
      if (action.requiresConfirmation) {
        final ok = await WpPopups.messageBox(
          ref,
          title: names[action] ?? action.name,
          body: '确定要${names[action] ?? action.name}吗？',
          primary: '确定',
          secondary: '取消',
          destructive:
              action == SessionPowerAction.powerOff ||
              action == SessionPowerAction.logout,
        );
        if (ok == true) {
          await controller.request(action);
          if (ref.read(sessionPowerProvider).confirmationAction != null) {
            await controller.confirm();
          }
        }
        return;
      }
      await controller.request(action);
    }

    return ListView(
      children: [
        for (final entry in names.entries)
          ListTile(
            title: Text(entry.value),
            enabled: !power.busy && power.availabilityFor(entry.key).enabled,
            onTap: () => unawaited(request(entry.key)),
          ),
        if (power.error != null)
          Padding(padding: const EdgeInsets.all(16), child: Text(power.error!)),
      ],
    );
  }
}
