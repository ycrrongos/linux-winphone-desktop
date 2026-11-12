// SPDX-License-Identifier: GPL-3.0-only
// Copied into the pinned host by prepare; uses the host's actual OSK key intents.
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:denial_flutter_sdk/platform.dart';

import 'shell_osk_panel.dart';
import 'pinyin_engine.dart';
import 'pinyin_session.dart';
import '../../state/win10x_wp_motion.dart';

class Win10xPinyinKeyboard extends StatefulWidget {
  const Win10xPinyinKeyboard({
    super.key,
    this.onKey,
    this.onKeyDown,
    this.textInputStates,
    this.predictions,
    this.onPredict,
  });
  final ValueChanged<ShellOskKeyIntent>? onKey;
  final VoidCallback? onKeyDown;
  final Stream<DenialTextInputState>? textInputStates;
  final Stream<DenialTextPrediction>? predictions;
  final ValueChanged<ShellOskWordRequest>? onPredict;
  @override
  State<Win10xPinyinKeyboard> createState() => _Win10xPinyinKeyboardState();
}

class _Win10xPinyinKeyboardState extends State<Win10xPinyinKeyboard> {
  final session = PinyinSession();
  StreamSubscription<DenialTextInputState>? input;
  DenialTextInputState? editor;
  String raw = '';
  List<PinyinChoice> choices = [];
  bool expanded = false, loaded = false;
  Timer? debounce;
  int revision = 0;
  bool get sensitive =>
      editor != null &&
      ((editor!.contentHint & 0xc0) != 0 ||
          {8, 9}.contains(editor!.contentPurpose));
  bool get directOnly =>
      sensitive || {2, 3, 4, 5, 6}.contains(editor?.contentPurpose);
  bool get composing =>
      session.chinese && !directOnly && loaded && (editor?.active ?? false);
  @override
  void initState() {
    super.initState();
    listen();
    unawaited(load());
  }

  Future<void> load() async {
    try {
      await session.initialize(
        await rootBundle.loadString(
          'packages/win10x_shell/assets/ime/pinyin_simp.dict.yaml',
        ),
      );
      await session.ready.future;
      if (mounted) setState(() => loaded = true);
    } on Object {
      if (mounted) setState(() => loaded = false);
    }
  }

  void listen() {
    input = widget.textInputStates?.listen((next) {
      if (editor?.activationSerial != next.activationSerial ||
          editor?.active != next.active ||
          editor?.contentHint != next.contentHint ||
          editor?.contentPurpose != next.contentPurpose) {
        reset();
      }
      editor = next;
      if (mounted) setState(() {});
    });
  }

  @override
  void didUpdateWidget(covariant Win10xPinyinKeyboard old) {
    super.didUpdateWidget(old);
    if (old.textInputStates != widget.textInputStates) {
      input?.cancel();
      listen();
    }
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (!TickerMode.of(context)) reset();
  }

  void reset() {
    debounce?.cancel();
    revision++;
    raw = '';
    choices = [];
    expanded = false;
  }

  void changed() {
    choices = [];
    final key = ++revision;
    debounce?.cancel();
    if (raw.isNotEmpty) {
      debounce = Timer(const Duration(milliseconds: 25), () async {
        final result = await session.query(raw);
        if (!mounted || key != revision) return;
        setState(() => choices = result);
      });
    }
    setState(() {});
  }

  void pick(PinyinChoice choice) {
    if (!composing || raw.isEmpty || !choices.contains(choice)) return;
    widget.onKey?.call(ShellOskKeyIntent.text(choice.text));
    session.learn(choice);
    var count = 0, index = 0;
    while (index < raw.length && count < choice.consumed) {
      if (raw[index] != "'") count++;
      index++;
    }
    raw = raw.substring(index).replaceFirst(RegExp("^'+"), '');
    expanded = false;
    changed();
  }

  void direct() {
    if (raw.isNotEmpty) {
      widget.onKey?.call(ShellOskKeyIntent.text(raw));
      raw = '';
      changed();
    }
  }

  Future<void> _actions = Future.value();
  void key(ShellOskKeyIntent intent) {
    final owner = (
      editor?.activationSerial,
      editor?.active,
      editor?.contentPurpose,
      editor?.contentHint,
    );
    _actions = _actions.then((_) async {
      if (mounted &&
          owner ==
              (
                editor?.activationSerial,
                editor?.active,
                editor?.contentPurpose,
                editor?.contentHint,
              )) {
        await processKey(intent);
      }
    });
  }

  Future<bool> commitDefault() async {
    if (choices.isEmpty && raw.isNotEmpty) {
      debounce?.cancel();
      final expected = revision;
      final result = await session.query(raw);
      if (!mounted || expected != revision || !composing) return false;
      choices = result;
    }
    if (choices.isNotEmpty) {
      pick(choices.first);
    } else {
      direct();
    }
    return true;
  }

  Future<void> processKey(ShellOskKeyIntent intent) async {
    if (!composing || intent.ctrl) {
      if (raw.isNotEmpty) reset();
      widget.onKey?.call(intent);
      return;
    }
    switch (intent.action) {
      case ShellOskKeyAction.text:
        final text = intent.text ?? '';
        if (RegExp("^[a-zA-Zv']+\$").hasMatch(text)) {
          if (raw.length < 64) {
            raw += text.toLowerCase();
            changed();
          }
          return;
        }
        if (raw.isNotEmpty) {
          if (!await commitDefault()) return;
        }
        if (raw.isNotEmpty) direct();
        const punctuation = {
          ',': '，',
          '.': '。',
          '?': '？',
          '!': '！',
          ';': '；',
          ':': '：',
        };
        widget.onKey?.call(
          punctuation.containsKey(text)
              ? ShellOskKeyIntent.text(punctuation[text]!)
              : intent,
        );
      case ShellOskKeyAction.backspace:
        if (raw.isNotEmpty) {
          raw = raw.substring(0, raw.length - 1);
          changed();
        } else {
          widget.onKey?.call(intent);
        }
      case ShellOskKeyAction.space:
        if (raw.isNotEmpty) {
          if (!await commitDefault()) return;
        } else {
          widget.onKey?.call(intent);
        }
      case ShellOskKeyAction.enter:
        if (raw.isNotEmpty) {
          direct();
        } else {
          widget.onKey?.call(intent);
        }
      case ShellOskKeyAction.key:
        // Cursor movement cannot carry an old editor's composition forward.
        reset();
        setState(() {});
        widget.onKey?.call(intent);
      case ShellOskKeyAction.replace:
        widget.onKey?.call(intent);
    }
  }

  @override
  void dispose() {
    debounce?.cancel();
    input?.cancel();
    session.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Widget button(String label, VoidCallback? tap) => TextButton(
      onPressed: tap,
      style: TextButton.styleFrom(
        foregroundColor: Colors.white,
        padding: const EdgeInsets.symmetric(horizontal: 10),
        shape: const RoundedRectangleBorder(),
      ),
      child: Text(label),
    );
    final header = ColoredBox(
      color: const Color(0xff171717),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            height: 34,
            child: Row(
              children: [
                button(
                  session.chinese && !directOnly ? '中' : '英',
                  loaded && !directOnly
                      ? () {
                          setState(() {
                            reset();
                            session.chinese = !session.chinese;
                            session.save();
                          });
                        }
                      : null,
                ),
                Expanded(
                  child: Text(
                    sensitive
                        ? '安全输入'
                        : directOnly
                        ? '英文／数字输入'
                        : raw.isEmpty
                        ? (loaded ? '拼音 · 点击候选词上屏' : '正在加载拼音词库…')
                        : raw,
                    style: const TextStyle(color: Colors.white, fontSize: 17),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
                if (raw.isNotEmpty)
                  button('取消', () {
                    setState(reset);
                  }),
                if (composing)
                  button(expanded ? '收起' : '展开', () {
                    setState(() => expanded = !expanded);
                  }),
              ],
            ),
          ),
          if (composing)
            SizedBox(
              height: 40,
              child: ListView(
                scrollDirection: Axis.horizontal,
                children: [
                  for (final choice in choices)
                    Semantics(
                      label: '候选 ${choice.text}',
                      button: true,
                      child: button(choice.text, () => pick(choice)),
                    ),
                ],
              ),
            ),
        ],
      ),
    );
    return Column(
      children: [
        header,
        Expanded(
          child: AnimatedSwitcher(
            duration: WpMotion.barExit,
            switchInCurve: WpMotion.enter,
            switchOutCurve: WpMotion.exit,
            transitionBuilder: (child, animation) => FadeTransition(
              opacity: animation,
              child: IgnorePointer(
                ignoring: animation.status == AnimationStatus.reverse,
                child: child,
              ),
            ),
            child: expanded && composing
                ? ColoredBox(
                    color: const Color(0xff171717),
                    child: SingleChildScrollView(
                      child: Wrap(
                        children: [
                          for (final choice in choices)
                            Semantics(
                              label: '候选 ${choice.text}',
                              button: true,
                              child: button(choice.text, () => pick(choice)),
                            ),
                        ],
                      ),
                    ),
                  )
                : ShellOskPanel(
                    key: ValueKey(
                      'pinyin:$composing:${editor?.activationSerial}',
                    ),
                    onKey: key,
                    onKeyDown: widget.onKeyDown,
                    textInputStates: widget.textInputStates,
                    predictions: composing || sensitive
                        ? null
                        : widget.predictions,
                    onPredict: composing ? null : widget.onPredict,
                  ),
          ),
        ),
      ],
    );
  }
}
