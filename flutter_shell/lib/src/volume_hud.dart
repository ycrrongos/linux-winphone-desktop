// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:flutter/foundation.dart' show ValueListenable;

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/input.dart';

import 'fluent.dart';
import 'wp_motion.dart';
import 'phone_volume_panel.dart';
import 'preferences.dart';
import 'shell_navigation.dart';

/// WP-style top volume strip, backed by the actual output-volume service.
/// Linux has no universal independent ringer channel, so do not invent one.
class Win10xVolumeHud extends ConsumerStatefulWidget {
  const Win10xVolumeHud({
    super.key,
    required this.services,
    this.events,
    this.nativeRevision,
    this.nativeVisible = true,
    this.onNativeDismissed,
  });
  final ShellServices services;
  final ValueListenable<int>? events;
  final int? nativeRevision;
  final bool nativeVisible;
  final VoidCallback? onNativeDismissed;
  @override
  ConsumerState<Win10xVolumeHud> createState() => _VolumeHudState();
}

class _VolumeHudState extends ConsumerState<Win10xVolumeHud>
    with SingleTickerProviderStateMixin {
  late final _motion = AnimationController(
    vsync: this,
    duration: WpMotion.barEnter,
    reverseDuration: WpMotion.barExit,
  );
  Timer? _hide;
  bool _shown = false;
  bool _dragging = false;
  double _beforeMute = .5;
  @override
  void initState() {
    super.initState();
    widget.events?.addListener(_present);
    if (widget.nativeRevision != null && widget.nativeVisible) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _present();
      });
    }
  }

  @override
  void didUpdateWidget(covariant Win10xVolumeHud old) {
    super.didUpdateWidget(old);
    if (widget.nativeRevision != null) {
      if (widget.nativeVisible &&
          (old.nativeRevision != widget.nativeRevision || !old.nativeVisible)) {
        _present();
      }
      if (!widget.nativeVisible && old.nativeVisible) {
        unawaited(_dismiss());
      }
    }
    if (old.events != widget.events) {
      old.events?.removeListener(_present);
      widget.events?.addListener(_present);
    }
  }

  void _present() {
    if (!PreferenceController.instance.value.isPhone ||
        ShellNavigation.instance.dashboardVisible) {
      return;
    }
    _hide?.cancel();
    setState(() => _shown = true);
    _motion.animateTo(1, duration: WpMotion.barEnter, curve: WpMotion.enter);
    if (!_dragging && widget.nativeRevision == null) _armHide();
  }

  void _armHide() {
    _hide?.cancel();
    _hide = Timer(const Duration(seconds: 3), () => unawaited(_dismiss()));
  }

  Future<void> _dismiss() async {
    if (_dragging) {
      _armHide();
      return;
    }
    try {
      await _motion
          .animateBack(0, duration: WpMotion.barExit, curve: WpMotion.exit)
          .orCancel;
    } on TickerCanceled {
      return;
    }
    if (mounted && _motion.isDismissed) {
      setState(() => _shown = false);
      widget.onNativeDismissed?.call();
    }
  }

  @override
  void dispose() {
    widget.events?.removeListener(_present);
    _hide?.cancel();
    _motion.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (widget.nativeRevision == null) {
      ref.listen<double>(widget.services.volume, (previous, next) {
        if (previous != null && (next - previous).abs() > .005) _present();
      });
    }
    if (!_shown) return const SizedBox.shrink();
    final volume = ref.watch(widget.services.volume).clamp(0.0, 1.0);
    final media = ref.watch(widget.services.media).asData?.value;
    return Positioned(
      left: 0,
      right: 0,
      top: 24,
      child: Win10xFluent.wrapPhone(
        AnimatedBuilder(
          animation: _motion,
          builder: (context, child) {
            final t = _motion.value;
            return FractionalTranslation(
              translation: Offset(0, -(1 - t)),
              child: IgnorePointer(
                ignoring: _motion.status == AnimationStatus.reverse,
                child: child,
              ),
            );
          },
          child: ShellInputRegion(
            debugLabel: 'Win10X phone volume controls',
            child: PhoneVolumePanel(
              volume: volume,
              onBegin: widget.services.beginVolumeInteraction,
              onChanged: widget.services.setVolume,
              onEnd: widget.services.commitVolume,
              onToggleMute: () {
                if (volume > 0) _beforeMute = volume;
                widget.services.setVolume(volume > 0 ? 0 : _beforeMute);
                widget.services.commitVolume(volume > 0 ? 0 : _beforeMute);
              },
              onInteraction: (dragging) {
                _dragging = dragging;
                _hide?.cancel();
                if (!dragging) _armHide();
              },
              title: media?.title,
              artist: media?.artistLabel,
              playing: media?.playing ?? false,
              onPrevious: media?.available == true
                  ? () => unawaited(
                      ref.read(widget.services.mediaCommands).previous(),
                    )
                  : null,
              onPlayPause: media?.available == true
                  ? () => unawaited(
                      ref.read(widget.services.mediaCommands).playPause(),
                    )
                  : null,
              onNext: media?.available == true
                  ? () => unawaited(
                      ref.read(widget.services.mediaCommands).next(),
                    )
                  : null,
            ),
          ),
        ),
      ),
    );
  }
}
