// SPDX-License-Identifier: GPL-3.0-only
import 'markui_icons.dart';

// Disco Launcher live-tile flip (tileList.scss hide-flip-direction-0):
// 1s; 0→20% ease-in rotateY 0→-90; 21% swap to +90; 21→100% ease-out →0.
import 'dart:async';
import 'dart:io';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/models.dart'
    show MprisPlaybackState, MprisPlaybackStatus;
import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/material.dart';

import 'motion_math.dart';

/// Disco `--tile-list-flip-animation-duration`.
const liveTileFlipDuration = Duration(milliseconds: 1000);

/// Hold each live face before the next Disco flip (icon ↔ content).
const liveTileHoldDuration = Duration(milliseconds: 4500);

double liveTileFlipDegrees(double t) {
  final x = t.clamp(0.0, 1.0);
  if (x <= 0.20) {
    final u = Curves.easeIn.transform(x / 0.20);
    return -90.0 * u;
  }
  final u = Curves.easeOut.transform(((x - 0.21) / 0.79).clamp(0.0, 1.0));
  return 90.0 * (1.0 - u);
}

bool liveTileShowsFront(double t) => t.clamp(0.0, 1.0) < 0.205;

/// Disco `--tile-list-flip-animation-easing`.
@visibleForTesting
double liveTileCubicOvershoot(double t) => cubicBezier(t, .2, 1.25, .5, 1);

/// Nearest WP accent ARGB to [color] (MetroLauncher auto-accent intent).
int nearestPhoneAccentArgb(Color color, List<int> palette) {
  var best = palette.first;
  var bestDist = double.infinity;
  final r = (color.r * 255.0).round();
  final g = (color.g * 255.0).round();
  final b = (color.b * 255.0).round();
  for (final argb in palette) {
    final pr = (argb >> 16) & 0xff;
    final pg = (argb >> 8) & 0xff;
    final pb = argb & 0xff;
    final d = (r - pr) * (r - pr) + (g - pg) * (g - pg) + (b - pb) * (b - pb);
    if (d < bestDist) {
      bestDist = d.toDouble();
      best = argb;
    }
  }
  return best;
}

bool mediaMatchesApp(MprisPlaybackState media, String appId) {
  if (!media.available) return false;
  final norm = appId.toLowerCase().replaceAll(RegExp(r'[^a-z0-9]+'), '');
  if (norm.isEmpty) return false;
  bool hit(String raw) {
    final n = raw.toLowerCase().replaceAll(RegExp(r'[^a-z0-9]+'), '');
    return n.isNotEmpty && (n.contains(norm) || norm.contains(n));
  }

  return hit(media.identity) || hit(media.serviceName);
}

/// Wide/Large live face: notification text, optional media controls, Disco flip.
class LiveTileFace extends StatefulWidget {
  const LiveTileFace({
    super.key,
    required this.iconFace,
    required this.contentFace,
    required this.enabled,
    this.media,
    this.mediaCommands,
    this.accent = const Color(0xff0078d4),
  });

  final Widget iconFace;
  final Widget contentFace;
  final bool enabled;
  final MprisPlaybackState? media;
  final MediaCommands? mediaCommands;
  final Color accent;

  @override
  State<LiveTileFace> createState() => _LiveTileFaceState();
}

class _LiveTileFaceState extends State<LiveTileFace>
    with SingleTickerProviderStateMixin {
  late final AnimationController _flip;
  Timer? _hold;
  bool _showingContent = false;

  @override
  void initState() {
    super.initState();
    _flip = AnimationController(vsync: this, duration: liveTileFlipDuration);
    if (widget.enabled) _scheduleHold();
  }

  @override
  void didUpdateWidget(covariant LiveTileFace oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.enabled != oldWidget.enabled) {
      _hold?.cancel();
      if (widget.enabled) {
        _scheduleHold();
      } else {
        _flip.stop();
        _flip.value = 0;
        _showingContent = false;
      }
    }
  }

  void _scheduleHold() {
    _hold?.cancel();
    _hold = Timer(liveTileHoldDuration, _runFlip);
  }

  Future<void> _runFlip() async {
    if (!mounted || !widget.enabled) return;
    await _flip.forward(from: 0);
    if (!mounted) return;
    setState(() => _showingContent = !_showingContent);
    _flip.value = 0;
    _scheduleHold();
  }

  @override
  void dispose() {
    _hold?.cancel();
    _flip.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final media = widget.media;
    final mediaFace = media != null && media.available
        ? _MediaMiniPlayer(
            media: media,
            commands: widget.mediaCommands,
            accent: widget.accent,
          )
        : null;
    final back = mediaFace ?? widget.contentFace;
    if (!widget.enabled) {
      return mediaFace ?? widget.iconFace;
    }
    return AnimatedBuilder(
      animation: _flip,
      builder: (context, _) {
        final t = _flip.isAnimating ? _flip.value : 0.0;
        final degrees = _flip.isAnimating ? liveTileFlipDegrees(t) : 0.0;
        final front = liveTileShowsFront(t);
        final face = _flip.isAnimating
            ? (front
                  ? (_showingContent ? back : widget.iconFace)
                  : (_showingContent ? widget.iconFace : back))
            : (_showingContent ? back : widget.iconFace);
        final matrix = Matrix4.identity()
          ..setEntry(3, 2, 0.001)
          ..rotateY(degrees * math.pi / 180);
        return Transform(
          alignment: Alignment.center,
          transform: matrix,
          child: face,
        );
      },
    );
  }
}

class _MediaMiniPlayer extends StatelessWidget {
  const _MediaMiniPlayer({
    required this.media,
    required this.commands,
    required this.accent,
  });
  final MprisPlaybackState media;
  final MediaCommands? commands;
  final Color accent;

  @override
  Widget build(BuildContext context) {
    final progress = media.length.inMilliseconds <= 0
        ? 0.0
        : (media.positionAt(DateTime.now()).inMilliseconds /
                  media.length.inMilliseconds)
              .clamp(0.0, 1.0);
    final url = media.artUrl.trim();
    Widget? art;
    if (url.startsWith('http://') || url.startsWith('https://')) {
      art = Image.network(
        url,
        fit: BoxFit.cover,
        errorBuilder: (_, error, stack) => const SizedBox.shrink(),
      );
    } else if (url.startsWith('file://') || url.startsWith('/')) {
      final path = url.startsWith('file://')
          ? Uri.parse(url).toFilePath()
          : url;
      art = Image.file(
        File(path),
        fit: BoxFit.cover,
        errorBuilder: (_, error, stack) => const SizedBox.shrink(),
      );
    }
    final title = media.title.isEmpty ? media.identity : media.title;
    return Padding(
      padding: const EdgeInsets.fromLTRB(10, 10, 10, 28),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (art != null)
            Expanded(child: ClipRect(child: art))
          else ...[
            Text(
              title,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 14,
                fontWeight: FontWeight.w600,
              ),
            ),
            if (media.artistLabel.isNotEmpty) ...[
              const SizedBox(height: 4),
              Text(
                media.artistLabel,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(color: Colors.white70, fontSize: 12),
              ),
            ],
            const Spacer(),
          ],
          if (art != null) ...[
            const SizedBox(height: 4),
            Text(
              title,
              maxLines: 1,
              overflow: TextOverflow.ellipsis,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 12,
                fontWeight: FontWeight.w600,
              ),
            ),
          ],
          ClipRect(
            child: LinearProgressIndicator(
              value: progress,
              minHeight: 2,
              backgroundColor: Colors.white24,
              color: accent,
            ),
          ),
          const SizedBox(height: 6),
          Row(
            children: [
              IconButton(
                visualDensity: VisualDensity.compact,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                onPressed: media.canGoPrevious && commands != null
                    ? () => unawaited(commands!.previous())
                    : null,
                icon: const MarkUiIcon(
                  Icons.skip_previous,
                  color: Colors.white,
                ),
              ),
              IconButton(
                visualDensity: VisualDensity.compact,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                onPressed: (media.canPlay || media.canPause) && commands != null
                    ? () => unawaited(commands!.playPause())
                    : null,
                icon: MarkUiIcon(
                  media.status == MprisPlaybackStatus.playing
                      ? Icons.pause
                      : Icons.play_arrow,
                  color: Colors.white,
                ),
              ),
              IconButton(
                visualDensity: VisualDensity.compact,
                padding: EdgeInsets.zero,
                constraints: const BoxConstraints(minWidth: 36, minHeight: 36),
                onPressed: media.canGoNext && commands != null
                    ? () => unawaited(commands!.next())
                    : null,
                icon: const MarkUiIcon(Icons.skip_next, color: Colors.white),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
