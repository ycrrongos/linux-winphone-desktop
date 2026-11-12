// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/wallpaper.dart';

import 'preferences.dart';
import 'screen_motion.dart';
import 'screen_policy.dart';
import 'shell_navigation.dart';
import 'start_motion.dart';
import 'wp_motion.dart';

/// Native idle/DPMS observation also covers input going to an application.
/// Preview never runs hooks or changes the host display's power policy.
class ScreenPowerHost extends ConsumerStatefulWidget {
  const ScreenPowerHost({super.key, required this.child});
  final Widget child;
  @override
  ConsumerState<ScreenPowerHost> createState() => _ScreenPowerHostState();
}

class _ScreenPowerHostState extends ConsumerState<ScreenPowerHost>
    with SingleTickerProviderStateMixin {
  late final AnimationController motion = AnimationController(
    vsync: this,
    duration: const Duration(milliseconds: 600),
    value: 1,
  )..addListener(() => ScreenMotion.progress.value = motion.value);
  Timer? poll;
  StreamSubscription<bool>? ambient;
  bool polling = false, transitioning = false, dark = false;
  final powerEdges = ScreenPowerEdges();
  Future<void> commands = Future.value();
  final native =
      Platform.environment['DENIAL_COMPOSITOR_BINARY'] != null &&
      Platform.environment['WIN10X_PREVIEW_CAPTURE'] == null;
  @override
  void initState() {
    super.initState();
    PreferenceController.instance.addListener(syncWallpaper);
    WidgetsBinding.instance.addPostFrameCallback((_) => syncWallpaper());
    if (native) {
      poll = Timer.periodic(
        const Duration(milliseconds: 500),
        (_) => unawaited(check()),
      );
      WidgetsBinding.instance.addPostFrameCallback((_) {
        ambient = ref.read(denialBridgeProvider).ambientDisplay.listen((off) {
          if (off) {
            unawaited(turnOff(requestNative: false));
          } else {
            wake();
          }
        });
      });
    }
  }

  String? appliedWallpaper;
  bool wallpaperSynced = false;
  void syncWallpaper() {
    if (!mounted) return;
    final path = PreferenceController.instance.value.desktopWallpaperPath;
    if (wallpaperSynced && path == appliedWallpaper) return;
    if (!wallpaperSynced && path == null) {
      wallpaperSynced = true;
      return;
    }
    appliedWallpaper = path;
    wallpaperSynced = true;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final resource = path == null
          ? WallpaperResource.defaultWallpaper
          : WallpaperResource.file(path);
      ref
          .read(wallpaperControllerProvider.notifier)
          .commitCandidate(
            WallpaperCandidate(
              id: 'win10x-local',
              providerId: 'local',
              label: '用户壁纸',
              previewUri: path == null ? Uri() : Uri.file(path),
              width: 0,
              height: 0,
              resource: resource,
            ),
            resource,
            revealOriginFraction: const Offset(.5, .5),
            target: const WallpaperTarget.all(),
          );
    });
  }

  Future<void> hook(bool off) async {
    final p = PreferenceController.instance.value;
    final command = off ? p.screenOffCommand : p.screenOnCommand;
    if (command.trim().isEmpty) return;
    final root = p.screenHooksRoot;
    // Serialize transitions; no commands in UI/build callbacks and no retries.
    commands = commands.then((_) async {
      try {
        final result = await Process.run('/usr/bin/timeout', [
          '--signal=TERM',
          '--kill-after=2s',
          '15s',
          if (root) ...[
            'sudo',
            '-n',
            '/usr/local/libexec/win10x-screen-hooks',
            off ? 'off' : 'on',
          ] else ...[
            '/bin/sh',
            '-c',
            command,
          ],
        ]);
        if (result.exitCode != 0) {
          debugPrint(
            'Win10X screen hook ${off ? 'off' : 'on'} failed: exit ${result.exitCode}',
          );
        }
      } on Object {
        debugPrint('Win10X screen hook failed or timed out');
      }
    });
  }

  Future<Map<String, dynamic>?> query(List<String> args) async {
    final tool = Platform.environment['DENIAL_CONTROL_TOOL'] ?? 'denialctl';
    final result = await Process.run(tool, [
      '--json',
      ...args,
    ]).timeout(const Duration(seconds: 2));
    if (result.exitCode != 0) return null;
    return jsonDecode(result.stdout.toString()) as Map<String, dynamic>;
  }

  Future<void> check() async {
    if (polling || !mounted) return;
    polling = true;
    try {
      final status = await query(['status']);
      final outputs = (status?['outputs'] as Map?)?['outputs'] as List?;
      if (outputs != null && outputs.isNotEmpty) {
        final panel = outputs
            .where((o) => o['connected'] == true && o['enabled'] == true)
            .toList();
        if (panel.isNotEmpty) {
          final powered = panel.any((o) => o['powered'] == true);
          if (powerEdges.observe(powered) != null) {
            if (powered) {
              wake();
            } else {
              dark = true;
              motion.value = 0;
              unawaited(hook(true));
            }
          }
          if (!powered && !dark) {
            dark = true;
            motion.value = 0;
          }
        }
      }
      final ui = status?['ui'] as Map?;
      final activity = ui?['session_activity'] as Map?;
      final p = PreferenceController.instance.value;
      if (transitioning && (activity?['input_idle_ms'] as num? ?? 0) < 500) {
        wake();
      }
      if (screenIdleDue(
        enabled: p.autoScreenOff,
        powered: !dark,
        transitioning: transitioning,
        inhibited: activity?['idle_inhibited'] == true,
        idleMs: (activity?['input_idle_ms'] as num? ?? 0).toInt(),
        seconds: p.screenOffSeconds,
      )) {
        unawaited(turnOff());
      }
    } on Object {
      /* No compositor in preview/disconnected session. */
    } finally {
      polling = false;
    }
  }

  Future<void> turnOff({bool requestNative = true}) async {
    if (transitioning || dark || !mounted) return;
    transitioning = true;
    final locked = ref.read(authenticationProvider).locked;
    final p = PreferenceController.instance.value;
    if (!locked) {
      ScreenMotion.startCloseDuration = null;
      ScreenMotion.startMotion.value = true;
    }
    final duration = locked
        ? const Duration(milliseconds: 600)
        : ScreenMotion.startCloseDuration ??
              StartReveal.closeDuration(p, rowSpan: p.tiles.length);
    try {
      await motion
          .animateTo(
            0,
            duration: (MediaQuery.maybeOf(context)?.disableAnimations ?? false)
                ? Duration.zero
                : duration,
            curve: locked ? Curves.linear : WpMotion.exit,
          )
          .orCancel;
      if (!mounted) return;
      if (requestNative) {
        ref.read(denialBridgeProvider).requestDpmsOff();
      } else {
        ref.read(denialBridgeProvider).ambientDisplayReady();
      }
    } on TickerCanceled {
      return;
    } finally {
      transitioning = false;
    }
  }

  void wake() {
    if (!mounted) return;
    final wasDark = dark;
    dark = false;
    ScreenMotion.startMotion.value = false;
    motion.animateTo(
      1,
      duration: (MediaQuery.maybeOf(context)?.disableAnimations ?? false)
          ? Duration.zero
          : const Duration(milliseconds: 600),
    );
    if (wasDark) unawaited(hook(false));
  }

  @override
  void dispose() {
    PreferenceController.instance.removeListener(syncWallpaper);
    poll?.cancel();
    ambient?.cancel();
    motion.dispose();
    ScreenMotion.progress.value = 1;
    ScreenMotion.startMotion.value = false;
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    ref.listen(authenticationProvider.select((s) => s.locked), (
      previous,
      locked,
    ) {
      if (previous == true && !locked) {
        ScreenMotion.startMotion.value = false;
        final nav = ShellNavigation.instance;
        if (nav.services case final services?) nav.revealHome(services);
        ScreenMotion.reenter.value++;
      }
    });
    return AnimatedBuilder(
      animation: motion,
      builder: (context, _) => Stack(
        alignment: Alignment.topLeft,
        fit: StackFit.expand,
        children: [
          widget.child,
          IgnorePointer(
            child: ColoredBox(
              color: const Color(0xff000000).withValues(
                alpha: ref.watch(authenticationProvider).locked
                    ? 0
                    : 1 - motion.value,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
