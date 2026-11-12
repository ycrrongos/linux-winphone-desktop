// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/system_services.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Lumia-era status order: connectivity left, clock centered, battery right.
class Win10xStatusBar extends ConsumerWidget {
  const Win10xStatusBar({super.key, required this.services});
  static const height = 24.0;
  final ShellServices services;
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final clock = ref.watch(services.clock).asData?.value;
    final battery = ref.watch(services.battery);
    final network = ref.watch(networkConnectivityProvider);
    final bluetooth = ref.watch(bluetoothProvider);
    final mobile = ref.watch(mobileNetworkProvider).asData?.value;
    final notices = ref.watch(desktopNotificationsProvider);
    final time = clock == null
        ? ''
        : '${clock.hour.toString().padLeft(2, '0')}:${clock.minute.toString().padLeft(2, '0')}';
    const iconStyle = IconThemeData(size: 14, color: Colors.white);
    const labelStyle = TextStyle(fontSize: 10, color: Colors.white);
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: services.openDashboard,
      child: ColoredBox(
        color: Colors.black,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 10),
          child: IconTheme(
            data: iconStyle,
            child: Row(
              children: [
                Expanded(
                  child: Row(
                    children: [
                      if (mobile != null &&
                          mobile.managerAvailable &&
                          mobile.modemPath != null) ...[
                        MarkUiIcon(
                          mobile.registered
                              ? Icons.signal_cellular_4_bar
                              : Icons.signal_cellular_0_bar,
                        ),
                        if (mobile.technology.isNotEmpty)
                          Text(mobile.technology, style: labelStyle),
                        const SizedBox(width: 6),
                      ],
                      if (network.snapshot.connectedNetwork != null)
                        const MarkUiAsset('ui/status_wifi', size: 14),
                      if (bluetooth.powered)
                        const Padding(
                          padding: EdgeInsets.only(left: 6),
                          child: MarkUiAsset('ui/status_bluetooth', size: 14),
                        ),
                      if (notices.doNotDisturb)
                        const Padding(
                          padding: EdgeInsets.only(left: 6),
                          child: MarkUiIcon(Icons.do_not_disturb_on_outlined),
                        ),
                    ],
                  ),
                ),
                Text(
                  time,
                  style: const TextStyle(color: Colors.white, fontSize: 12),
                ),
                Expanded(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.end,
                    children: [
                      MarkUiAsset(
                        battery.charging
                            ? 'ui/charging'
                            : 'ui/battery_${((battery.capacity ?? 0).clamp(0, 100) * 17 / 100).round()}',
                        size: 18,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        battery.capacity == null
                            ? '--'
                            : '${battery.capacity}%',
                        style: labelStyle,
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
