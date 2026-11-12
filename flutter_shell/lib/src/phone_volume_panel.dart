// SPDX-License-Identifier: GPL-3.0-only
import 'package:flutter/material.dart';

import 'markui_icons.dart';
import 'wp_motion.dart';

/// Lumia-style top strip: channel, compact slider/value, expandable media row.
/// The supplied channel is real Linux output audio, never a fabricated ringer.
class PhoneVolumePanel extends StatefulWidget {
  const PhoneVolumePanel({
    super.key,
    required this.volume,
    required this.onChanged,
    required this.onBegin,
    required this.onEnd,
    required this.onToggleMute,
    this.title,
    this.artist,
    this.playing = false,
    this.onPrevious,
    this.onPlayPause,
    this.onNext,
    this.onInteraction,
  });
  final double volume;
  final ValueChanged<double> onChanged, onEnd;
  final VoidCallback onBegin, onToggleMute;
  final String? title, artist;
  final bool playing;
  final VoidCallback? onPrevious, onPlayPause, onNext;
  final ValueChanged<bool>? onInteraction;
  @override
  State<PhoneVolumePanel> createState() => _PhoneVolumePanelState();
}

class _PhoneVolumePanelState extends State<PhoneVolumePanel> {
  bool _expanded = true;
  @override
  Widget build(BuildContext context) => Material(
    color: const Color(0xff282c34),
    child: Padding(
      padding: const EdgeInsets.fromLTRB(12, 2, 12, 6),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            children: [
              const Expanded(
                child: Text(
                  '媒体音量',
                  style: TextStyle(color: Colors.white, fontSize: 11),
                ),
              ),
              SizedBox(
                width: 40,
                height: 28,
                child: IconButton(
                  padding: EdgeInsets.zero,
                  tooltip: _expanded ? '收起媒体控制' : '展开媒体控制',
                  onPressed: () {
                    setState(() => _expanded = !_expanded);
                    widget.onInteraction?.call(false);
                  },
                  icon: MarkUiIcon(
                    _expanded ? Icons.expand_more : Icons.keyboard_arrow_up,
                    color: Colors.white,
                    size: 14,
                  ),
                ),
              ),
            ],
          ),
          SizedBox(
            height: 36,
            child: Row(
              children: [
                SizedBox(
                  width: 32,
                  height: 36,
                  child: IconButton(
                    padding: EdgeInsets.zero,
                    tooltip: widget.volume == 0 ? '取消静音' : '静音',
                    onPressed: widget.onToggleMute,
                    icon: MarkUiIcon(
                      widget.volume == 0
                          ? Icons.do_not_disturb_on_outlined
                          : Icons.volume_up_outlined,
                      size: 18,
                      color: Colors.white,
                    ),
                  ),
                ),
                Expanded(
                  child: SliderTheme(
                    data: SliderTheme.of(context).copyWith(
                      trackHeight: 2,
                      tickMarkShape: SliderTickMarkShape.noTickMark,
                    ),
                    child: Slider(
                      value: widget.volume.clamp(0, 1),
                      label:
                          '媒体音量 ${(widget.volume.clamp(0, 1) * 30).round()}/30',
                      divisions: 30,
                      onChangeStart: (_) {
                        widget.onBegin();
                        widget.onInteraction?.call(true);
                      },
                      onChanged: widget.onChanged,
                      onChangeEnd: (v) {
                        widget.onEnd(v);
                        widget.onInteraction?.call(false);
                      },
                    ),
                  ),
                ),
                SizedBox(
                  width: 32,
                  child: Text(
                    '${(widget.volume.clamp(0, 1) * 30).round()}'.padLeft(
                      2,
                      '0',
                    ),
                    textAlign: TextAlign.right,
                    style: const TextStyle(color: Colors.white, fontSize: 13),
                  ),
                ),
              ],
            ),
          ),
          AnimatedSize(
            duration: MediaQuery.disableAnimationsOf(context)
                ? Duration.zero
                : WpMotion.barEnter,
            curve: WpMotion.enter,
            alignment: Alignment.topCenter,
            child: _expanded && widget.onPlayPause != null
                ? Padding(
                    padding: const EdgeInsets.only(top: 8),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        Text(
                          widget.title ?? '正在播放',
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: const TextStyle(
                            color: Colors.white,
                            fontSize: 14,
                          ),
                        ),
                        if (widget.artist?.isNotEmpty == true)
                          Text(
                            widget.artist!,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: const TextStyle(
                              color: Colors.white70,
                              fontSize: 10,
                            ),
                          ),
                        Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            IconButton(
                              tooltip: '上一首',
                              onPressed: widget.onPrevious,
                              icon: const MarkUiIcon(
                                Icons.skip_previous,
                                size: 18,
                                color: Colors.white,
                              ),
                            ),
                            IconButton(
                              tooltip: widget.playing ? '暂停' : '播放',
                              onPressed: widget.onPlayPause,
                              icon: MarkUiIcon(
                                widget.playing ? Icons.pause : Icons.play_arrow,
                                size: 18,
                                color: Colors.white,
                              ),
                            ),
                            IconButton(
                              tooltip: '下一首',
                              onPressed: widget.onNext,
                              icon: const MarkUiIcon(
                                Icons.skip_next,
                                size: 18,
                                color: Colors.white,
                              ),
                            ),
                          ],
                        ),
                      ],
                    ),
                  )
                : const SizedBox.shrink(),
          ),
        ],
      ),
    ),
  );
}
