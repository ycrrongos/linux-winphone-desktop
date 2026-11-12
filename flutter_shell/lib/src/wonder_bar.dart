// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

// Wonder Bar: keyboard-cover remainder — trackpad / media / PiP affordances
// (research/windows10x/dual-screen-hardware/notes/ui-and-interaction.md).
import 'dart:async';

import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'dual_screen.dart';
import 'fluent.dart';
import 'preferences.dart';

class Win10xWonderBar extends ConsumerStatefulWidget {
  const Win10xWonderBar({super.key, required this.services});
  final ShellServices services;

  @override
  ConsumerState<Win10xWonderBar> createState() => _Win10xWonderBarState();
}

class _Win10xWonderBarState extends ConsumerState<Win10xWonderBar> {
  @override
  void initState() {
    super.initState();
    DualScreenController.instance.addListener(_onDual);
  }

  @override
  void dispose() {
    DualScreenController.instance.removeListener(_onDual);
    super.dispose();
  }

  void _onDual() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final geo = DualScreenController.instance.geometry;
    if (geo == null || !geo.hasWonderBar) {
      return const SizedBox.shrink();
    }
    final media = ref.watch(widget.services.media).asData?.value;
    final commands = ref.read(widget.services.mediaCommands);
    final accent = Color(
      PreferenceController.instance.value.phoneAccentArgbValue,
    );
    final bar = geo.wonderBar;
    return Positioned(
      left: bar.left,
      top: bar.top,
      width: bar.width,
      height: bar.height,
      child: Material(
        color: Win10xFluent.chrome,
        child: Row(
          children: [
            Container(width: 4, color: accent),
            const SizedBox(width: 12),
            const MarkUiIcon(Icons.apps, size: 28),
            const SizedBox(width: 8),
            const Text('Wonder Bar', style: TextStyle(fontSize: 16)),
            const Spacer(),
            if (media != null && media.available) ...[
              IconButton(
                onPressed: () => unawaited(commands.previous()),
                icon: const MarkUiIcon(Icons.skip_previous),
              ),
              IconButton(
                onPressed: () => unawaited(commands.playPause()),
                icon: MarkUiIcon(
                  media.playing ? Icons.pause : Icons.play_arrow,
                ),
              ),
              IconButton(
                onPressed: () => unawaited(commands.next()),
                icon: const MarkUiIcon(Icons.skip_next),
              ),
              const SizedBox(width: 8),
              Flexible(
                child: Text(
                  media.title,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
              ),
            ],
            const SizedBox(width: 12),
          ],
        ),
      ),
    );
  }
}
