// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';

import 'keyboard_tools.dart';
import '../../state/win10x_wp_motion.dart';

class KeyboardToolsPanel extends StatelessWidget {
  const KeyboardToolsPanel({
    super.key,
    required this.tools,
    required this.onStroke,
    required this.onChange,
    required this.onCommand,
    this.sensitive = false,
  });
  final KeyboardTools tools;
  final ValueChanged<PcKeyStroke> onStroke;
  final VoidCallback onChange;
  final ValueChanged<String> onCommand;
  final bool sensitive;
  void activateKey(PcKey spec) {
    if (const {
      'Shift_L',
      'Shift_R',
      'Control_L',
      'Control_R',
      'Alt_L',
      'Alt_R',
      'Super_L',
    }.contains(spec.key)) {
      tools.modifier(spec.key);
      onChange();
      return;
    }
    switch (spec.key) {
      case 'fn':
        tools.fn = !tools.fn;
      case 'caps':
        tools.caps = !tools.caps;
      case 'numpad':
        tools.view = KeyboardView.numpad;
      case 'computer':
        tools.view = KeyboardView.computer;
      default:
        onStroke(
          tools.stroke(
            spec.key,
            direction: const {
              'Left',
              'Right',
              'Up',
              'Down',
              'Home',
              'End',
              'Page_Up',
              'Page_Down',
            }.contains(spec.key),
          ),
        );
    }
    onChange();
  }

  @override
  Widget build(BuildContext context) {
    Widget button(PcKey spec) => Expanded(
      flex: (spec.width * 10).round(),
      child: Padding(
        padding: const EdgeInsets.all(2),
        child: KeyboardToolKey(
          label: spec.label,
          semanticLabel: '电脑键 ${spec.label}',
          active:
              tools.modifiers.containsKey(spec.key) ||
              (spec.key == 'fn' && tools.fn) ||
              (spec.key == 'caps' && tools.caps),
          onPressed: () => activateKey(spec),
          onLongPress: spec.key.endsWith('_L') || spec.key.endsWith('_R')
              ? () {
                  tools.modifier(spec.key, lock: true);
                  onChange();
                }
              : null,
          repeat: const {
            'Left',
            'Right',
            'Up',
            'Down',
            'BackSpace',
            'Delete',
            'Page_Up',
            'Page_Down',
          }.contains(spec.key),
        ),
      ),
    );
    if (tools.view == KeyboardView.editing) {
      Widget arrow(String label, String value) => Expanded(
        child: Padding(
          padding: const EdgeInsets.all(2),
          child: KeyboardToolKey(
            label: label,
            semanticLabel: '光标 $label',
            repeat: true,
            onPressed: () {
              onStroke(
                tools.stroke(
                  value,
                  direction: const {
                    'Left',
                    'Right',
                    'Up',
                    'Down',
                    'Home',
                    'End',
                    'Page_Up',
                    'Page_Down',
                  }.contains(value),
                ),
              );
              onChange();
            },
          ),
        ),
      );
      Widget command(String label) => Expanded(
        child: Padding(
          padding: const EdgeInsets.all(2),
          child: KeyboardToolKey(
            label: label,
            onPressed: sensitive && const {'复制', '剪切', '全选'}.contains(label)
                ? null
                : () => onCommand(label),
          ),
        ),
      );
      Widget toggle(String label, bool selected, VoidCallback action) =>
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(2),
              child: KeyboardToolKey(
                label: label,
                active: selected,
                onPressed: () {
                  action();
                  onChange();
                },
              ),
            ),
          );
      return Column(
        children: [
          Expanded(
            child: Row(
              children: [
                arrow('行首', 'Home'),
                arrow('↑', 'Up'),
                arrow('行尾', 'End'),
                command('全选'),
              ],
            ),
          ),
          Expanded(
            child: Row(
              children: [
                arrow('←', 'Left'),
                toggle(
                  '选择',
                  tools.selecting,
                  () => tools.selecting = !tools.selecting,
                ),
                arrow('→', 'Right'),
                command('复制'),
              ],
            ),
          ),
          Expanded(
            child: Row(
              children: [
                arrow('上页', 'Page_Up'),
                arrow('↓', 'Down'),
                arrow('下页', 'Page_Down'),
                command('粘贴'),
              ],
            ),
          ),
          Expanded(
            child: Row(
              children: [
                command('撤销'),
                command('重做'),
                command('剪切'),
                arrow('删除', 'Delete'),
              ],
            ),
          ),
          Expanded(
            child: Row(
              children: [
                toggle('按词', tools.byWord, () => tools.byWord = !tools.byWord),
                arrow('Tab', 'Tab'),
                arrow('Esc', 'Escape'),
                arrow('⌫', 'BackSpace'),
              ],
            ),
          ),
        ],
      );
    }
    final rows = tools.view == KeyboardView.numpad
        ? KeyboardTools.numpad
        : KeyboardTools.rows;
    return LayoutBuilder(
      builder: (context, limits) {
        final width = tools.view == KeyboardView.numpad
            ? limits.maxWidth
            : math.max(limits.maxWidth, 850.0);
        final height = math.max(limits.maxHeight - 22, rows.length * 34.0);
        return Column(
          children: [
            SizedBox(
              height: 22,
              child: Text(
                tools.fn ? 'Fn：数字→F键，方向→翻页/行首尾' : '左右滑动查看 · 修饰键点按一次，长按锁定',
                style: const TextStyle(color: Colors.white70, fontSize: 11),
              ),
            ),
            Expanded(
              child: SingleChildScrollView(
                child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: SizedBox(
                    width: width,
                    height: height,
                    child: Column(
                      children: [
                        for (final row in rows)
                          Expanded(
                            child: Row(
                              children: [for (final spec in row) button(spec)],
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}

class KeyboardToolKey extends StatefulWidget {
  const KeyboardToolKey({
    super.key,
    required this.label,
    this.semanticLabel,
    this.onPressed,
    this.onLongPress,
    this.active = false,
    this.repeat = false,
  });
  final String label;
  final String? semanticLabel;
  final VoidCallback? onPressed, onLongPress;
  final bool active, repeat;
  @override
  State<KeyboardToolKey> createState() => _KeyboardToolKeyState();
}

class _KeyboardToolKeyState extends State<KeyboardToolKey> {
  Timer? timer;
  bool pressed = false, repeated = false;
  void cancel() {
    timer?.cancel();
    timer = null;
    if (mounted) setState(() => pressed = false);
  }

  @override
  void dispose() {
    timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Semantics(
    label: widget.semanticLabel ?? widget.label,
    button: true,
    enabled: widget.onPressed != null,
    selected: widget.active,
    child: GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTapDown: widget.onPressed == null
          ? null
          : (_) {
              setState(() {
                pressed = true;
                repeated = false;
              });
              if (widget.repeat)
                timer = Timer(const Duration(milliseconds: 400), () {
                  repeated = true;
                  widget.onPressed?.call();
                  timer = Timer.periodic(
                    const Duration(milliseconds: 65),
                    (_) => widget.onPressed?.call(),
                  );
                });
            },
      onTapCancel: cancel,
      onTapUp: widget.onPressed == null
          ? null
          : (_) {
              if (!repeated) widget.onPressed?.call();
              cancel();
            },
      onLongPress: widget.repeat ? null : widget.onLongPress,
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 90),
        curve: WpMotion.enter,
        alignment: Alignment.center,
        decoration: BoxDecoration(
          color: pressed || widget.active
              ? const Color(0xff0078d7)
              : const Color(0xff303030),
          border: Border.all(color: Colors.white12),
        ),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 3),
          child: FittedBox(
            fit: BoxFit.scaleDown,
            child: Text(
              widget.label,
              style: TextStyle(
                fontSize: 15,
                color: widget.onPressed == null ? Colors.white30 : Colors.white,
              ),
            ),
          ),
        ),
      ),
    ),
  );
}
