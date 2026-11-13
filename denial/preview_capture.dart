// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';

import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:denial_flutter_sdk/launcher.dart';
import 'package:win10x_shell/src/app_catalog.dart';
import 'package:win10x_shell/src/catalog_index.dart';
import 'package:win10x_shell/src/phone_tasks.dart';
import 'package:win10x_shell/src/preferences.dart';
import 'package:win10x_shell/src/fluent.dart';
import 'package:win10x_shell/src/markui_icons.dart';
import 'package:win10x_shell/src/launch.dart';
import 'package:win10x_shell/src/phone_lock_pane.dart';
import 'package:win10x_shell/src/screen_motion.dart';
import 'package:win10x_shell/src/mode_settings.dart';
import 'package:win10x_shell/src/wallpaper_picker.dart';
import 'package:win10x_shell/src/phone_volume_panel.dart';

import 'package:denial_desktop/src/state/win10x_client_surface.dart';
import 'package:denial_desktop/src/state/reference_shell_controller.dart';
import 'package:denial_desktop/src/state/system_level_hud.dart';
import 'package:denial_desktop/src/widgets/system_level_hud.dart';
import 'package:denial_desktop/src/widgets/osk/pinyin_keyboard.dart';
import 'package:denial_desktop/src/widgets/osk/shell_osk_panel.dart';
import 'package:denial_desktop/src/widgets/osk/keyboard_tools.dart';
import 'package:denial_desktop/src/widgets/osk/keyboard_tools_panel.dart';
import 'package:denial_flutter_sdk/platform.dart';
import 'package:denial_desktop/src/desktop/retained_animated_positioned.dart';

import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/rendering.dart';
import 'package:flutter/gestures.dart';
import 'package:win10x_shell/src/controls.dart';
import 'package:win10x_shell/src/fluent_controls.dart';
import 'package:flutter/services.dart';
import 'package:win10x_shell/src/shell_navigation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter/material.dart'
    show TextButton, IconButton, Material, TextField, InputDecoration;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:denial_flutter_sdk/popups.dart';
import 'package:denial_flutter_sdk/input.dart';
import 'package:win10x_shell/src/wp_popups.dart';
import 'package:win10x_shell/src/wp_toast.dart';
import 'package:win10x_shell/src/dashboard_reveal.dart';

/// Opt-in self-capture of this preview only; never capture the host desktop.
class PreviewCapture extends ConsumerStatefulWidget {
  const PreviewCapture({super.key, required this.path, required this.child});
  final String path;
  final Widget child;
  @override
  ConsumerState<PreviewCapture> createState() => _PreviewCaptureState();
}

class _PreviewCaptureState extends ConsumerState<PreviewCapture> {
  final _boundary = GlobalKey();
  Timer? _timer;
  Timer? _exerciseTimer;
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      final delay =
          int.tryParse(
            Platform.environment['WIN10X_PREVIEW_CAPTURE_MS'] ?? '',
          ) ??
          3000;
      _timer = Timer(
        Duration(milliseconds: delay),
        () => unawaited(_capture()),
      );
      if (Platform.environment['WIN10X_PREVIEW_EXERCISE']
          case final scenario?) {
        _exerciseTimer = Timer(
          const Duration(milliseconds: 2200),
          () => unawaited(
            _exercise(scenario).catchError((Object error, StackTrace stack) {
              debugPrint('Win10X preview exercise failed: $error');
              debugPrint('$stack');
            }),
          ),
        );
      }
    });
  }

  int _testPointer = 100;
  Element? _findElement(bool Function(Element) test, [Element? root]) {
    Element? result;
    void visit(Element e) {
      if (result != null) return;
      if (test(e)) {
        result = e;
        return;
      }
      e.visitChildren(visit);
    }

    visit(root ?? _boundary.currentContext! as Element);
    return result;
  }

  RenderBox _semanticBox(String label, [Element? root]) {
    final element = _findElement(
      (e) =>
          e.widget is Semantics &&
          (e.widget as Semantics).properties.label == label,
      root,
    );
    if (element == null) throw StateError('Missing semantic target $label');
    return element.findRenderObject()! as RenderBox;
  }

  Future<void> _tapAt(Offset point) async {
    final pointer = _testPointer++;
    GestureBinding.instance.handlePointerEvent(
      PointerDownEvent(
        pointer: pointer,
        position: point,
        kind: PointerDeviceKind.touch,
      ),
    );
    GestureBinding.instance.handlePointerEvent(
      PointerUpEvent(
        pointer: pointer,
        position: point,
        kind: PointerDeviceKind.touch,
        timeStamp: const Duration(milliseconds: 80),
      ),
    );
    await _paintFrame();
  }

  Future<void> _dragBetween(Offset start, Offset end) async {
    final pointer = _testPointer++;
    final binding = GestureBinding.instance;
    binding.handlePointerEvent(
      PointerDownEvent(
        pointer: pointer,
        position: start,
        kind: PointerDeviceKind.touch,
      ),
    );
    for (var i = 1; i <= 12; i++) {
      await Future<void>.delayed(const Duration(milliseconds: 50));
      binding.handlePointerEvent(
        PointerMoveEvent(
          pointer: pointer,
          position: Offset.lerp(start, end, i / 12)!,
          delta: (end - start) / 12,
          kind: PointerDeviceKind.touch,
          timeStamp: Duration(milliseconds: i * 50),
        ),
      );
    }
    binding.handlePointerEvent(
      PointerUpEvent(
        pointer: pointer,
        position: end,
        kind: PointerDeviceKind.touch,
        timeStamp: const Duration(milliseconds: 650),
      ),
    );
    await _paintFrame();
  }

  // Opt-in checks run inside this preview's Flutter tree, never on the host.
  Future<void> _exercise(String scenario) async {
    if (!mounted) return;
    await _paintFrame();
    if (!mounted) return;
    if (scenario == 'toast-follow') {
      WpToast.show(ref, '通知跟手检查', hold: const Duration(seconds: 20));
      await Future<void>.delayed(const Duration(milliseconds: 400));
      final text = _findElement((e) {
        if (e.widget is! Text || (e.widget as Text).data != '通知跟手检查')
          return false;
        var visible = true;
        e.visitAncestorElements((a) {
          if (a.widget is Offstage && (a.widget as Offstage).offstage)
            visible = false;
          return true;
        });
        final box = e.findRenderObject();
        return visible && box is RenderBox && box.hasSize && box.attached;
      });
      if (text == null) throw StateError('Toast missing');
      final box = text.findRenderObject()! as RenderBox;
      final oldY = box.localToGlobal(Offset.zero).dy;
      final point = box.localToGlobal(box.size.center(Offset.zero));
      final pointer = _testPointer++, binding = GestureBinding.instance;
      binding.handlePointerEvent(
        PointerDownEvent(
          pointer: pointer,
          position: point,
          kind: PointerDeviceKind.touch,
        ),
      );
      await _paintFrame();
      binding.handlePointerEvent(
        PointerMoveEvent(
          pointer: pointer,
          position: point - const Offset(0, 24),
          delta: const Offset(0, -24),
          kind: PointerDeviceKind.touch,
          timeStamp: const Duration(milliseconds: 100),
        ),
      );
      await _paintFrame();
      binding.handlePointerEvent(
        PointerMoveEvent(
          pointer: pointer,
          position: point - const Offset(0, 44),
          delta: const Offset(0, -20),
          kind: PointerDeviceKind.touch,
          timeStamp: const Duration(milliseconds: 200),
        ),
      );
      await _paintFrame();
      final delta = oldY - box.localToGlobal(Offset.zero).dy;
      if (delta < 10) throw StateError('Toast did not follow active finger');
      binding.handlePointerEvent(
        PointerCancelEvent(
          pointer: pointer,
          position: point - const Offset(0, 44),
          kind: PointerDeviceKind.touch,
        ),
      );
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if ((box.localToGlobal(Offset.zero).dy - oldY).abs() > 2)
        throw StateError('Toast cancel did not ease back');
      debugPrint('Win10X QA: toast follows drag and cancels continuously');
      return;
    }
    if (scenario == 'edge-cancel') {
      final services = ShellNavigation.instance.services;
      if (services == null) throw StateError('Shell services missing');
      DashboardReveal.followOpen(services: services, progress: .45);
      final settle = DashboardReveal.settleOpen(
        services: services,
        from: .45,
        velocity: 0,
        cancelled: true,
      );
      if ((ShellNavigation.instance.dashboardReveal.value ?? 0) < .4)
        throw StateError('Cancel jumped to zero before easing');
      await settle;
      if (ShellNavigation.instance.dashboardVisible)
        throw StateError('Cancelled panel stayed open');
      debugPrint(
        'Win10X QA: edge cancel preserves drag position until nonlinear settle',
      );
      return;
    }
    if (scenario == 'native-volume-route') {
      final audio = StreamController<DenialAudioState>.broadcast();
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X native volume routing fixture',
            barrierColor: const Color(0x00000000),
            builder: (context, handle) => ProviderScope(
              overrides: [
                systemLevelHudProvider.overrideWith(
                  SystemLevelHudController.new,
                ),
                systemLevelHudSignalsProvider.overrideWithValue((
                  audio: audio.stream,
                  brightness: const Stream<DenialBrightnessState>.empty(),
                )),
                systemLevelHudVisibleDurationProvider.overrideWithValue(
                  const Duration(seconds: 20),
                ),
              ],
              child: const Stack(
                fit: StackFit.expand,
                children: [SystemLevelHudLayer()],
              ),
            ),
          );
      await Future<void>.delayed(const Duration(milliseconds: 400));
      audio.add(
        const DenialAudioState(level: 1, requestSerial: 0, limitReached: true),
      );
      await Future<void>.delayed(const Duration(milliseconds: 650));
      if (_findElement((e) => e.widget is PhoneVolumePanel) == null)
        throw StateError('Native audio event did not reach WP top panel');
      if (_findElement(
            (e) => e.widget.runtimeType.toString() == '_SystemLevelHudCard',
          ) !=
          null)
        throw StateError('Legacy bottom volume card still rendered');
      final panel =
          _findElement((e) => e.widget is PhoneVolumePanel)!.findRenderObject()!
              as RenderBox;
      if (panel.localToGlobal(Offset.zero).dy > 60)
        throw StateError('Phone volume did not anchor at top');
      debugPrint(
        'Win10X QA: native audio/limit event routes exclusively to top WP volume',
      );
      return;
    }
    if (scenario == 'ime-chinese' ||
        scenario == 'ime-secret' ||
        scenario == 'ime-candidates' ||
        scenario == 'ime-expanded' ||
        scenario.startsWith('keyboard-tools-')) {
      final input = StreamController<DenialTextInputState>.broadcast();
      final output = ValueNotifier<String>('');
      final shortcuts = <PcKeyStroke>[];
      final keyboardTargetFocus = FocusNode(
        debugLabel: 'Keyboard QA editor target',
      );
      final arrowKeys = <String>[];
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X Chinese keyboard fixture',
            barrierColor: const Color(0x00000000),
            builder: (context, handle) => Win10xFluent.wrapPhone(
              Material(
                color: const Color(0xff171717),
                child: Column(
                  children: [
                    const SizedBox(height: 50),
                    Focus(
                      autofocus: scenario == 'keyboard-tools-focus',
                      focusNode: keyboardTargetFocus,
                      child: ValueListenableBuilder<String>(
                        valueListenable: output,
                        builder: (context, text, _) => Text(
                          '输入结果：$text',
                          style: const TextStyle(
                            color: Color(0xffffffff),
                            fontSize: 28,
                          ),
                        ),
                      ),
                    ),
                    const Spacer(),
                    SizedBox(
                      height: (MediaQuery.sizeOf(context).height - 180)
                          .clamp(100, 330)
                          .toDouble(),
                      child: Win10xPinyinKeyboard(
                        textInputStates: input.stream,
                        onShortcut: shortcuts.add,
                        onKey: (intent) {
                          if (intent.action == ShellOskKeyAction.key)
                            arrowKeys.add(intent.key ?? '');
                          if (intent.action == ShellOskKeyAction.text)
                            output.value += intent.text ?? '';
                        },
                      ),
                    ),
                    const SizedBox(height: 48),
                  ],
                ),
              ),
            ),
          );
      await Future<void>.delayed(const Duration(milliseconds: 700));
      input.add(
        const DenialTextInputState(
          active: true,
          inputPanelVisible: true,
          legacy: false,
          contentHint: 0,
          contentPurpose: 0,
          activationSerial: 100,
        ),
      );
      Element? toggle;
      for (var i = 0; i < 40; i++) {
        toggle = _findElement(
          (e) =>
              e.widget is TextButton &&
              (e.widget as TextButton).onPressed != null &&
              (e.widget as TextButton).child is Text &&
              {
                '英',
                '中',
              }.contains(((e.widget as TextButton).child as Text).data),
        );
        if (toggle != null) break;
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (toggle == null) throw StateError('Pinyin dictionary did not load');
      if (((toggle.widget as TextButton).child as Text).data == '英') {
        // The stream/dictionary can enable this widget before its new layout
        // is painted. Do not inject a setup tap into the preceding frame.
        await _paintFrame();
        final b = toggle.findRenderObject()! as RenderBox;
        await _tapAt(b.localToGlobal(b.size.center(Offset.zero)));
      }
      // AnimatedSwitcher keeps the outgoing English keys alive (and ignores
      // their pointers). Wait for it before locating the actual current keys.
      await Future<void>.delayed(const Duration(milliseconds: 300));
      final modeReady = _findElement(
        (e) =>
            e.widget is TextButton &&
            (e.widget as TextButton).child is Text &&
            ((e.widget as TextButton).child as Text).data == '中',
      );
      if (modeReady == null)
        throw StateError(
          'Chinese mode setup tap did not activate its rendered button',
        );
      if (scenario.startsWith('keyboard-tools-')) {
        Future<void> tapButton(String label) async {
          final element = _findElement(
            (e) =>
                e.widget is TextButton &&
                (e.widget as TextButton).child is Text &&
                ((e.widget as TextButton).child as Text).data == label,
          );
          if (element == null) throw StateError('Missing toolbar $label');
          await Scrollable.ensureVisible(element);
          await _paintFrame();
          final box = element.findRenderObject()! as RenderBox;
          await _tapAt(box.localToGlobal(box.size.center(Offset.zero)));
          await Future<void>.delayed(const Duration(milliseconds: 350));
        }

        Future<void> tapSemantic(String label) async {
          final element = _findElement(
            (e) =>
                e.widget is Semantics &&
                (e.widget as Semantics).properties.label == label,
          );
          if (element == null) throw StateError('Missing key $label');
          await Scrollable.ensureVisible(element);
          await _paintFrame();
          final box = element.findRenderObject()! as RenderBox;
          final point = box.localToGlobal(box.size.center(Offset.zero));
          final hits = HitTestResult();
          GestureBinding.instance.hitTestInView(
            hits,
            point,
            View.of(element).viewId,
          );
          if (label == '电脑键 数字区')
            debugPrint(
              'Win10X QA numpad tap=$point size=${box.size} hit=${hits.path.any((entry) => identical(entry.target, box))}',
            );
          await _tapAt(point);
          await Future<void>.delayed(const Duration(milliseconds: 300));
        }

        if (scenario == 'keyboard-tools-focus') {
          await tapButton('编辑');
          await tapButton('撤销');
          if (!keyboardTargetFocus.hasFocus)
            throw StateError('Keyboard controls stole editor focus');
        } else if (scenario == 'keyboard-tools-editing') {
          await tapButton('编辑');
          await tapSemantic('选择');
          await tapSemantic('光标 ←');
          if (shortcuts.last.key != 'Left' ||
              !shortcuts.last.modifiers.contains('Shift_L'))
            throw StateError('Selection did not send Shift+Left');
          await tapButton('复制');
          await tapButton('粘贴');
          await tapButton('撤销');
          await tapButton('重做');
          if (shortcuts.map((s) => s.key).join(',') != 'Left,c,v,z,z' ||
              shortcuts
                  .skip(1)
                  .take(3)
                  .any(
                    (s) =>
                        s.modifiers.length != 1 ||
                        s.modifiers.first != 'Control_L',
                  ))
            throw StateError('Editing shortcut mapping wrong');
          if (!shortcuts.last.modifiers.contains('Shift_L'))
            throw StateError('Redo lacks Linux Ctrl+Shift+Z');
        } else if (scenario == 'keyboard-tools-space') {
          final element = _findElement(
            (e) =>
                e.widget is Semantics &&
                const {
                  'Space',
                  '空格',
                }.contains((e.widget as Semantics).properties.label),
          );
          if (element == null) throw StateError('Space semantic missing');
          final box = element.findRenderObject()! as RenderBox;
          final start = box.localToGlobal(box.size.center(Offset.zero));
          await _dragBetween(start, start + const Offset(64, 0));
          if (arrowKeys.where((k) => k == 'Right').length < 3 ||
              output.value.contains(' '))
            throw StateError(
              'Space drag arrows=$arrowKeys output=${output.value}',
            );
        } else {
          await tapButton('电脑');
          await tapSemantic('电脑键 F12');
          if (shortcuts.last.key != 'F12') throw StateError('F12 missing');
          await tapSemantic('电脑键 Fn');
          await tapSemantic('电脑键 1');
          if (shortcuts.last.key != 'F1')
            throw StateError('Fn number layer failed');
          await tapSemantic('电脑键 Ctrl');
          await tapSemantic('电脑键 z');
          if (shortcuts.last.key != 'z' ||
              !shortcuts.last.modifiers.contains('Control_L'))
            throw StateError('PC Ctrl chord missing');
          if (scenario == 'keyboard-tools-numpad') {
            await tapButton('数字区');
            final pane = _findElement((e) => e.widget is KeyboardToolsPanel);
            debugPrint(
              'Win10X QA numpad view=${(pane?.widget as KeyboardToolsPanel?)?.tools.view}',
            );
            await tapSemantic('电脑键 7');
            if (shortcuts.last.key != 'KP_7')
              throw StateError('Numpad mapping ${shortcuts.last.key}');
          }
        }
        debugPrint('Win10X QA: $scenario actual toolbar/keys passed');
        return;
      }
      if (scenario == 'ime-secret') {
        input.add(
          const DenialTextInputState(
            active: true,
            inputPanelVisible: true,
            legacy: false,
            contentHint: 0xc0,
            contentPurpose: 8,
            activationSerial: 101,
          ),
        );
        await Future<void>.delayed(const Duration(milliseconds: 200));
      }
      for (final letter in 'nihao'.split('')) {
        final element = _findElement(
          (e) =>
              e.widget is Semantics &&
              ((e.widget as Semantics).properties.label ?? '').toLowerCase() ==
                  letter,
        );
        if (element == null) throw StateError('Keyboard letter missing');
        final b = element.findRenderObject()! as RenderBox;
        await _tapAt(b.localToGlobal(b.size.center(Offset.zero)));
        await Future<void>.delayed(const Duration(milliseconds: 65));
      }
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (scenario == 'ime-secret') {
        if (output.value.toLowerCase() != 'nihao' ||
            _findElement(
                  (e) =>
                      e.widget is Semantics &&
                      (e.widget as Semantics).properties.label == '候选 你好',
                ) !=
                null)
          throw StateError('Secure editor used Chinese composition');
      } else {
        if (output.value.isNotEmpty)
          throw StateError('Unselected pinyin leaked to editor');
        _semanticBox('候选 你好');
        if (scenario == 'ime-candidates') {
          debugPrint(
            'Win10X QA: nihao candidates present, no premature commit',
          );
          return;
        }
        if (scenario == 'ime-expanded') {
          final element = _findElement(
            (e) =>
                e.widget is TextButton &&
                (e.widget as TextButton).child is Text &&
                ((e.widget as TextButton).child as Text).data == '展开',
          );
          if (element == null) throw StateError('Candidate expansion missing');
          final box = element.findRenderObject()! as RenderBox;
          await _tapAt(box.localToGlobal(box.size.center(Offset.zero)));
          await Future<void>.delayed(const Duration(milliseconds: 300));
          _semanticBox('候选 你好');
          debugPrint('Win10X QA: expanded keyboard candidates present');
          return;
        }
        final choice = _semanticBox('候选 你好');
        await _tapAt(choice.localToGlobal(choice.size.center(Offset.zero)));
        await Future<void>.delayed(const Duration(milliseconds: 200));
        if (output.value != '你好')
          throw StateError('Chinese candidate did not commit');
      }
      debugPrint(
        'Win10X QA: Chinese keyboard candidate/secure-field flow passed',
      );
      return;
    }
    if (scenario.startsWith('wake-')) {
      await _exercise('lock-cover');
      ScreenMotion.progress.value = double.parse(scenario.substring(5));
      return;
    }
    if (scenario == 'wallpaper-select') {
      final file = Platform.environment['WIN10X_TEST_WALLPAPER'];
      if (file == null)
        throw StateError('Wallpaper selection needs test image');
      var selected = false;
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X wallpaper selection fixture',
            barrierColor: const Color(0x00000000),
            builder: (context, handle) => Win10xFluent.wrapPhone(
              WallpaperPicker(
                onClose: handle.close,
                onPicked: (path) {
                  PreferenceController.instance.change(
                    (p) => p.desktopWallpaperPath = path,
                  );
                  selected = true;
                  handle.close();
                },
              ),
            ),
          );
      await Future<void>.delayed(const Duration(milliseconds: 300));
      final field = _findElement(
        (e) =>
            e.widget is TextField &&
            (e.widget as TextField).decoration?.hintText == '输入目录或图片的完整路径',
      );
      if (field == null) throw StateError('Wallpaper path editor missing');
      (field.widget as TextField).onSubmitted!(file);
      for (var i = 0; i < 30 && !selected; i++) {
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (!selected ||
          PreferenceController.instance.value.desktopWallpaperPath != file)
        throw StateError('Wallpaper picker did not validate/persist selection');
      await _exercise('lock-cover');
      debugPrint(
        'Win10X QA: in-shell picker decoded and persisted test wallpaper',
      );
      return;
    }
    if (scenario == 'wallpaper-picker') {
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X wallpaper picker fixture',
            barrierColor: const Color(0x00000000),
            builder: (context, handle) => Win10xFluent.wrapPhone(
              WallpaperPicker(
                onClose: handle.close,
                onPicked: (_) => handle.close(),
              ),
            ),
          );
      return;
    }
    if (scenario == 'screen-settings') {
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X screen settings fixture',
            barrierColor: const Color(0x00000000),
            builder: (context, handle) => Win10xFluent.wrapPhone(
              Material(
                color: const Color(0xff171717),
                child: const Padding(
                  padding: EdgeInsets.fromLTRB(12, 24, 12, 48),
                  child: SingleChildScrollView(child: ShellModeSettings()),
                ),
              ),
            ),
          );
      return;
    }
    if (scenario == 'nav-tasks') {
      PreferenceController.instance.change((p) => p.phoneTaskButton = true);
      await Future<void>.delayed(const Duration(milliseconds: 300));
      final nav = _semanticBox('任务列表');
      await _tapAt(nav.localToGlobal(nav.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 600));
      if (!ref
          .read(shellPopupControllerProvider)
          .any((s) => s.debugLabel == 'Win10X phone task switcher')) {
        throw StateError('Task navigation button did not open task list');
      }
      return;
    }
    if (scenario == 'screen-start-close') {
      ScreenMotion.startMotion.value = true;
      return;
    }
    if (scenario == 'native-gtk-input') {
      if (Platform.environment['WIN10X_NATIVE_ISOLATED_TEST'] != '1') {
        throw StateError('GTK check requires isolated test session');
      }
      ShellNavigation.instance.closePanels?.call();
      final launched = ref.read(denialBridgeProvider).launchDesktopApplication(
        'win10x-gtk-probe.desktop',
        [
          '/usr/bin/env',
          'GDK_BACKEND=wayland',
          'GTK_IM_MODULE=wayland',
          '/usr/bin/python3',
          '/tmp/win10x-gtk-input-probe.py',
        ],
      );
      if (!launched) throw StateError('Native GTK fixture launch rejected');
      for (var i = 0; i < 40; i++) {
        if (ref.read(referenceShellProvider).edgePanelVisible) break;
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (!ref.read(referenceShellProvider).edgePanelVisible) {
        throw StateError('Native GTK editor did not summon keyboard');
      }
      await Future<void>.delayed(const Duration(milliseconds: 400));
      final letter = _findElement(
        (e) =>
            e.widget is Semantics &&
            ((e.widget as Semantics).properties.label ?? '').toLowerCase() ==
                'a',
      );
      if (letter == null)
        throw StateError('Native GTK keyboard has no letter A');
      final key = letter.findRenderObject()! as RenderBox;
      await _tapAt(key.localToGlobal(key.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 500));
      debugPrint(
        'Win10X QA: native GTK active editor summoned OSK; verify GTK test-key receipt separately',
      );
      return;
    }
    if (scenario == 'native-lock-input') {
      // Only the bounded separate test seat may trigger a native lock here.
      if (Platform.environment['WIN10X_NATIVE_ISOLATED_TEST'] != '1' ||
          !(Platform.environment['XDG_RUNTIME_DIR'] ?? '').endsWith(
            '/win10x-lock-test',
          )) {
        throw StateError('Native lock exercise requires isolated test runtime');
      }
      ref.read(authenticationProvider.notifier).lock();
      for (var i = 0; i < 30; i++) {
        if (_findElement((e) => e.widget is PhoneLockPane) != null) break;
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      final pane = _findElement((e) => e.widget is PhoneLockPane);
      if (pane == null) throw StateError('Native secure lock pane missing');
      final box = pane.findRenderObject()! as RenderBox;
      await _paintFrame();
      final origin = box.localToGlobal(Offset.zero);
      final viewport =
          View.of(pane).physicalSize / View.of(pane).devicePixelRatio;
      final start = Offset(viewport.width * .5, viewport.height * .8);
      final hits = HitTestResult();
      GestureBinding.instance.hitTestInView(hits, start, View.of(pane).viewId);
      debugPrint(
        'Win10X QA native lock: size=${box.size.width},${box.size.height} origin=${origin.dx},${origin.dy} viewport=${viewport.width},${viewport.height} touch=${start.dx},${start.dy} hits=${hits.path.map((e) => e.target.runtimeType).join(',')}',
      );
      await _dragBetween(start, start - Offset(0, viewport.height * .65));
      final afterSwipe = _findElement((e) => e.widget is PhoneLockPane);
      final native = ref.read(authenticationProvider);
      debugPrint(
        'Win10X QA native lock: revealed=${afterSwipe == null ? null : (afterSwipe.widget as PhoneLockPane).credentialsVisible} synchronized=${native.synchronized} locked=${native.locked} available=${native.available} busy=${native.busy} prompt=${native.prompt?.requiresResponse}',
      );

      await Future<void>.delayed(const Duration(milliseconds: 1100));
      if (ref.read(authenticationProvider).prompt?.requiresResponse != true) {
        throw StateError('Native PAM credential prompt missing');
      }
      final field = _findElement(
        (e) => e.widget is TextField && (e.widget as TextField).obscureText,
      );
      if (field == null)
        throw StateError('Native secure password editor missing');
      final fieldBox = field.findRenderObject()! as RenderBox;
      await _tapAt(fieldBox.localToGlobal(fieldBox.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 700));
      final key = _findElement(
        (e) =>
            e.widget is Semantics &&
            (e.widget as Semantics).properties.label == 'a',
      );
      if (key == null || !ref.read(referenceShellProvider).edgePanelVisible) {
        throw StateError('Native password keyboard did not open automatically');
      }
      final keyBox = key.findRenderObject()! as RenderBox;
      await _tapAt(keyBox.localToGlobal(keyBox.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 200));
      final text = (field.widget as TextField).controller!;
      final delivered = text.text.toLowerCase() == 'a';
      text.clear(); // Never submit a credential in this presentation/input check.
      if (!delivered)
        throw StateError('Native password keyboard did not deliver text');
      debugPrint(
        'Win10X QA: native secure swipe, PAM prompt, automatic keyboard and key delivery passed; no authentication submitted',
      );
      return;
    }
    if (scenario.startsWith('lock-')) {
      var revealed = false;
      final pane = GlobalKey();
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X lock picture layout fixture',
            transitionDuration: Duration.zero,
            keyboardPolicy: ShellKeyboardPolicy.none,
            builder: (context, handle) => StatefulBuilder(
              builder: (context, update) => Win10xFluent.wrapPhone(
                PhoneLockPane(
                  key: pane,
                  time: DateTime(2015, 1, 14, 3, 14),
                  notificationCount: 1,
                  batteryLabel: '87%',
                  credentialsVisible: revealed,
                  onCredentialsChanged: (v) => update(() => revealed = v),
                  wallpaper: const DecoratedBox(
                    decoration: BoxDecoration(
                      gradient: LinearGradient(
                        begin: Alignment.topLeft,
                        end: Alignment.bottomRight,
                        colors: [
                          Color(0xff1b323a),
                          Color(0xff26352b),
                          Color(0xff101820),
                        ],
                      ),
                    ),
                  ),
                  connectivity: const Text(
                    '5G  Wi-Fi',
                    style: TextStyle(color: Color(0xffffffff), fontSize: 11),
                  ),
                  credentials: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      const Text(
                        '解锁',
                        style: TextStyle(
                          color: Color(0xffffffff),
                          fontSize: 32,
                        ),
                      ),
                      const SizedBox(height: 24),
                      const Text(
                        '仅界面测试，不进行认证',
                        style: TextStyle(color: Color(0xffffffff)),
                      ),
                      const TextField(
                        decoration: InputDecoration(labelText: '密码（测试）'),
                        obscureText: true,
                      ),
                      TextButton(
                        onPressed: () => update(() => revealed = false),
                        child: const Text('返回锁屏'),
                      ),
                    ],
                  ),
                ),
                dark: true,
              ),
            ),
          );
      await _paintFrame();
      await _paintFrame();
      if (scenario == 'lock-cover') return;
      final size = MediaQuery.sizeOf(context);
      final start = Offset(size.width * .5, size.height * .8);
      if (scenario == 'lock-mid') {
        final binding = GestureBinding.instance;
        binding.handlePointerEvent(
          PointerDownEvent(
            pointer: 411,
            position: start,
            kind: PointerDeviceKind.touch,
          ),
        );
        for (var i = 1; i <= 8; i++) {
          await Future<void>.delayed(const Duration(milliseconds: 30));
          binding.handlePointerEvent(
            PointerMoveEvent(
              pointer: 411,
              position: start - Offset(0, size.height * .04 * i),
              delta: Offset(0, -size.height * .04),
              kind: PointerDeviceKind.touch,
              timeStamp: Duration(milliseconds: 30 * i),
            ),
          );
        }
        await _paintFrame();
        await _capture();
        binding.handlePointerEvent(
          PointerCancelEvent(
            pointer: 411,
            position: start - Offset(0, size.height * .32),
            kind: PointerDeviceKind.touch,
          ),
        );
        return;
      }
      await _dragBetween(
        start,
        start - Offset(0, size.height * (scenario == 'lock-short' ? .08 : .48)),
      );
      await Future<void>.delayed(const Duration(milliseconds: 500));
      if (revealed != (scenario != 'lock-short'))
        throw StateError('Lock swipe settle chose wrong page');
      if (scenario == 'lock-return') {
        final back = _findElement(
          (e) =>
              e.widget is TextButton &&
              (e.widget as TextButton).child is Text &&
              ((e.widget as TextButton).child as Text).data == '返回锁屏',
        )!;
        final box = back.findRenderObject()! as RenderBox;
        await _tapAt(box.localToGlobal(box.size.center(Offset.zero)));
        await Future<void>.delayed(const Duration(milliseconds: 400));
        if (revealed) throw StateError('Password-page return failed');
      }
      debugPrint(
        'Win10X QA: lock picture finger-follow/reveal/cancel/return fixture passed; no native authentication bypass',
      );
      return;
    }
    if (scenario == 'volume-panel') {
      var volume = .43;
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X Lumia volume layout fixture',
            barrierColor: const Color(0x00000000),
            transitionDuration: Duration.zero,
            builder: (context, handle) => StatefulBuilder(
              builder: (context, update) => Win10xFluent.wrapPhone(
                Align(
                  alignment: Alignment.topCenter,
                  child: Padding(
                    padding: const EdgeInsets.only(top: 24),
                    child: PhoneVolumePanel(
                      volume: volume,
                      onBegin: () {},
                      onChanged: (v) => update(() => volume = v),
                      onEnd: (_) {},
                      onToggleMute: () =>
                          update(() => volume = volume == 0 ? .43 : 0),
                      title: '正在播放的音乐',
                      artist: '媒体控制 · UI 测试',
                      onPrevious: () {},
                      onPlayPause: () {},
                      onNext: () {},
                    ),
                  ),
                ),
              ),
            ),
          );
      await _paintFrame();
      return;
    }
    if (scenario == 'context-menu') {
      WpPopups.contextMenu(
        ref,
        anchor: const Offset(180, 250),
        children: [
          FluentMenuItem(label: '固定到开始屏幕', onTap: () {}),
          FluentMenuItem(label: '应用信息', onTap: () {}),
          FluentMenuItem(label: '卸载', onTap: () {}),
        ],
      );
      await Future<void>.delayed(const Duration(milliseconds: 650));
      return;
    }
    if (scenario.startsWith('win8-motion:')) {
      final p = double.parse(scenario.split(':').last);
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X Win8 flight frame fixture',
            transitionDuration: Duration.zero,
            builder: (context, handle) => Material(
              color: const Color(0xff202020),
              child: LayoutBuilder(
                builder: (context, c) => Stack(
                  children: [
                    Windows8FlyingTile(
                      source: const Rect.fromLTWH(40, 100, 120, 120),
                      output: Rect.fromLTWH(0, 0, c.maxWidth, c.maxHeight),
                      progress: p,
                      face: const MarkUiAsset(
                        'apps/browser',
                        color: Color(0xffffffff),
                      ),
                      color: const Color(0xff0078d7),
                    ),
                    Positioned(
                      top: 24,
                      left: 20,
                      child: Text(
                        'Win8 flight frame $p (UI fixture)',
                        style: const TextStyle(color: Color(0xffffffff)),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          );
      await _paintFrame();
      return;
    }
    if (scenario == 'icon-scales') {
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X icon optical scale fixture',
            transitionDuration: Duration.zero,
            builder: (context, handle) => Win10xFluent.wrapPhone(
              Material(
                color: const Color(0xff171717),
                child: Padding(
                  padding: const EdgeInsets.all(24),
                  child: Column(
                    children: [
                      const Text(
                        'MarkUI optical size / DPR',
                        style: TextStyle(color: Color(0xffffffff)),
                      ),
                      for (final dpr in [1.0, 1.25, 1.5, 2.5])
                        Expanded(
                          child: MediaQuery(
                            data: MediaQuery.of(context)
                                .copyWith(devicePixelRatio: dpr),
                            child: Row(
                              mainAxisAlignment: MainAxisAlignment.spaceAround,
                              children: [
                                Text(
                                  'DPR $dpr',
                                  style: const TextStyle(
                                    color: Color(0xffffffff),
                                  ),
                                ),
                                for (final size in [
                                  16.0,
                                  24.0,
                                  32.0,
                                  48.0,
                                  64.0,
                                ])
                                  Column(
                                    mainAxisAlignment: MainAxisAlignment.center,
                                    children: [
                                      Text(
                                        '$size',
                                        style: const TextStyle(
                                          color: Color(0xffffffff),
                                        ),
                                      ),
                                      MarkUiAsset(
                                        'nav/start',
                                        size: size,
                                        color: const Color(0xffffffff),
                                      ),
                                      MarkUiAsset(
                                        'ui/apps',
                                        size: size,
                                        color: const Color(0xffffffff),
                                      ),
                                      MarkUiAsset(
                                        'apps/browser',
                                        size: size,
                                        color: const Color(0xffffffff),
                                      ),
                                      SizedBox.square(
                                        dimension: size,
                                        child: const MarkUiAsset(
                                          'apps/files',
                                          color: Color(0xffffffff),
                                        ),
                                      ),
                                    ],
                                  ),
                              ],
                            ),
                          ),
                        ),
                    ],
                  ),
                ),
              ),
            ),
          );
      await _paintFrame();
      debugPrint(
        'Win10X QA: icon fixture renders16/24/32/48/64 slots at DPR1/1.25/1.5/2.5; inspect visible bounds',
      );
      return;
    }
    if (scenario == 'keyboard-layout' || scenario == 'keyboard-nav') {
      ref.read(referenceShellProvider.notifier).openEdgePanel();
      await Future<void>.delayed(const Duration(milliseconds: 500));
      final key = _findElement(
        (e) =>
            e.widget is Semantics &&
            (e.widget as Semantics).properties.label == 'a',
      );
      if (key == null) throw StateError('Keyboard layout has no letter a');
      var hidden = false;
      key.visitAncestorElements((e) {
        if (e.widget is Offstage && (e.widget as Offstage).offstage)
          hidden = true;
        return true;
      });
      if (hidden) throw StateError('Forced keyboard layout is offstage');
      if (scenario == 'keyboard-nav') {
        final nav = _semanticBox('开始；长按单手模式');
        await _tapAt(nav.localToGlobal(nav.size.center(Offset.zero)));
        await Future<void>.delayed(const Duration(milliseconds: 400));
        if (ref.read(referenceShellProvider).edgePanelVisible)
          throw StateError('Keyboard swallowed navigation Start');
        debugPrint(
          'Win10X QA: visible keyboard leaves navigation Start reachable and dismisses on Home',
        );
      } else {
        debugPrint(
          'Win10X QA: forced real keyboard layout; input activation not claimed',
        );
      }
      return;
    }
    if (scenario == 'keyboard-entry') {
      final text = TextEditingController();
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X keyboard input fixture',
            keyboardPolicy: ShellKeyboardPolicy.none,
            transitionDuration: Duration.zero,
            builder: (context, handle) => Win10xFluent.wrapPhone(
              Material(
                color: const Color(0xff171717),
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(20, 60, 20, 60),
                  child: Align(
                    alignment: Alignment.topCenter,
                    child: TextField(
                      controller: text,
                      autofocus: true,
                      decoration: const InputDecoration(
                        labelText: 'Keyboard input verification',
                      ),
                    ),
                  ),
                ),
              ),
            ),
          );
      await Future<void>.delayed(const Duration(milliseconds: 900));
      final field = _findElement(
        (e) =>
            e.widget is TextField && (e.widget as TextField).controller == text,
      )!;
      final fieldBox = field.findRenderObject()! as RenderBox;
      await _tapAt(fieldBox.localToGlobal(fieldBox.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 600));
      final key = _findElement(
        (e) =>
            e.widget is Semantics &&
            (e.widget as Semantics).properties.label == 'a',
      );
      if (key == null)
        throw StateError('Automatic keyboard did not show letter a');
      var hidden = false;
      key.visitAncestorElements((e) {
        if (e.widget is Offstage && (e.widget as Offstage).offstage)
          hidden = true;
        return true;
      });
      if (hidden) throw StateError('Keyboard keys exist only offstage');
      final box = key.findRenderObject()! as RenderBox;
      await _tapAt(box.localToGlobal(box.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 200));
      if (!text.text.toLowerCase().contains('a'))
        throw StateError(
          'Visible keyboard did not deliver text to focused editor',
        );
      debugPrint(
        'Win10X QA: automatic keyboard opened and typed into focused Flutter editor',
      );
      return;
    }
    if (scenario.startsWith('alphabet-')) {
      final key = GlobalKey();
      var launched = 0;
      final apps = <LauncherApplication>[
        const LauncherApplication(
          id: 'number',
          name: '1 numeric app',
          searchableText: '1',
        ),
        for (var i = 0; i < 26; i++)
          for (var n = 0; n < 12; n++)
            LauncherApplication(
              id: 'fixture-$i-$n',
              name:
                  '${String.fromCharCode(65 + i)} app ${n.toString().padLeft(2, '0')}',
              searchableText: 'fixture',
            ),
      ];
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X alphabet fixture',
            transitionDuration: Duration.zero,
            builder: (context, handle) => Win10xFluent.wrapPhone(
              Material(
                color: PreferenceController.instance.value.phoneThemeDark
                    ? const Color(0xff101010)
                    : const Color(0xffffffff),
                child: Padding(
                  padding: const EdgeInsets.only(top: 24, bottom: 48),
                  child: AppCatalogView(
                    key: key,
                    apps: apps,
                    buildIcon: (context, id) => const MarkUiAsset('ui/apps'),
                    onLaunch: (_) => launched++,
                    onMenu: (_, _) {},
                  ),
                ),
              ),
              dark: PreferenceController.instance.value.phoneThemeDark,
              accentColor: Color(
                PreferenceController.instance.value.phoneAccentArgbValue,
              ),
            ),
          );
      await _paintFrame();
      await _paintFrame();
      ScrollPosition position() {
        final element =
            _findElement(
                  (e) => e is StatefulElement && e.state is ScrollableState,
                  key.currentContext! as Element,
                )!
                as StatefulElement;
        return (element.state as ScrollableState).position;
      }

      final rail = _semanticBox('快速字母索引', key.currentContext! as Element);
      final origin = rail.localToGlobal(Offset.zero);
      Offset railPoint(double fraction) =>
          origin + Offset(rail.size.width / 2, rail.size.height * fraction);
      final metrics = CatalogIndex({
        '#': 1,
        for (var i = 0; i < 26; i++) String.fromCharCode(65 + i): 12,
      }, rowHeight: 56);
      await _tapAt(railPoint(.5));
      await Future<void>.delayed(const Duration(milliseconds: 300));
      if ((position().pixels - metrics.offsets['M']!).abs() > 1)
        throw StateError('Rail tap did not reach M: ${position().pixels}');
      await _dragBetween(railPoint(.5), railPoint(.999));
      final end = metrics.offsets['Z']!.clamp(0.0, position().maxScrollExtent);
      if ((position().pixels - end).abs() > 1 || launched != 0)
        throw StateError(
          'Rail scrub missed Z or launched an app: actual=${position().pixels}, expected=$end, max=${position().maxScrollExtent}, launched=$launched',
        );
      await _dragBetween(railPoint(.999), railPoint(.04));
      if ((position().pixels - metrics.offsets['A']!).abs() > 1)
        throw StateError(
          'Rail reversal did not reach A: actual=${position().pixels}, expected=${metrics.offsets['A']}',
        );
      final header = _findElement(
        (e) =>
            e.widget is TextButton &&
            (e.widget as TextButton).child is Container &&
            ((e.widget as TextButton).child as Container).child is Text &&
            (((e.widget as TextButton).child as Container).child as Text)
                    .data ==
                'A',
        key.currentContext! as Element,
      );
      if (header == null)
        throw StateError('Visible A header absent after jump');
      final headerBox = header.findRenderObject()! as RenderBox;
      await _tapAt(headerBox.localToGlobal(headerBox.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 300));
      if (scenario == 'alphabet-picker') {
        if (_findElement(
              (e) => e.widget.runtimeType.toString() == 'PhoneNavigationBar',
            ) ==
            null)
          throw StateError('Alphabet picker lost navigation');
        debugPrint(
          'Win10X QA: rail tap/scrub/reversal and alphabet picker navigation passed',
        );
        return;
      }
      final pick = _findElement(
        (e) =>
            e.widget is TextButton &&
            (e.widget as TextButton).child is Text &&
            ((e.widget as TextButton).child as Text).data == 'M' &&
            (e.widget as TextButton).onPressed != null,
      )!;
      final pickBox = pick.findRenderObject()! as RenderBox;
      await _tapAt(pickBox.localToGlobal(pickBox.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 350));
      if ((position().pixels - metrics.offsets['M']!).abs() > 1) {
        debugPrint(
          'Win10X QA: grid scroll still progressing: ${position().pixels} -> ${metrics.offsets['M']}',
        );
        final deadline = DateTime.now().add(const Duration(seconds: 2));
        while ((position().pixels - metrics.offsets['M']!).abs() > 1 &&
            DateTime.now().isBefore(deadline)) {
          await Future<void>.delayed(const Duration(milliseconds: 32));
        }
      }
      if ((position().pixels - metrics.offsets['M']!).abs() > 1)
        throw StateError(
          'Alphabet grid did not use the same list geometry: actual=${position().pixels}, expected=${metrics.offsets['M']}',
        );
      await _dragBetween(railPoint(.5), railPoint(.999));
      if ((position().pixels - end).abs() > 1 || launched != 0)
        throw StateError('Final index position or app input was corrupted');
      debugPrint(
        'Win10X QA: rail tap, slow scrub, reverse, lazy far-section jump and grid jump passed; no app launched',
      );
      await _capture();
      return;
    }
    if (scenario == 'tasks-native-entry') {
      await _exercise('hold:返回；长按切换应用');
      await Future<void>.delayed(const Duration(milliseconds: 350));
      if (!ref
          .read(shellPopupControllerProvider)
          .any((s) => s.debugLabel == 'Win10X phone task switcher'))
        throw StateError('Hold Back did not open tasks');
      final start = _semanticBox('切换到 开始');
      await _tapAt(start.localToGlobal(start.size.center(Offset.zero)));
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (!ShellNavigation.instance.launcherVisible)
        throw StateError('Start card toggled Start closed');
      debugPrint(
        'Win10X QA: actual Back long-press entry and explicit Start-card destination passed (empty native preview)',
      );
      return;
    }
    if (scenario.startsWith('tasks-')) {
      var windows = <ApplicationWindow>[
        for (var i = 0; i < 4; i++)
          ApplicationWindow(
            id: 11 + i,
            appId: [
              'chromium',
              'org.kde.dolphin',
              'org.kde.gwenview',
              'org.kde.kate',
            ][i],
            title: ['Chromium', 'Dolphin', '照片', '笔记'][i],
            active: i == 0,
            minimized: false,
            previewSize: const Size(390, 708),
          ),
      ];
      final requested = <int>[];
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X tasks fixture',
            transitionDuration: Duration.zero,
            builder: (context, handle) => StatefulBuilder(
              builder: (context, update) => Win10xFluent.wrapPhone(
                PhoneTaskBoard(
                  windows: windows,
                  accent: Color(
                    PreferenceController.instance.value.phoneAccentArgbValue,
                  ),
                  labelFor: (w) => w.title,
                  buildIcon: (context, id) => MarkUiAsset(
                    id == 'chromium' ? 'apps/browser' : 'apps/files',
                    color: const Color(0xffffffff),
                  ),
                  buildPreview: (context, id) => Center(
                    child: Text(
                      '窗口 $id\nUI 布局测试',
                      textAlign: TextAlign.center,
                      style: const TextStyle(color: Color(0xffffffff)),
                    ),
                  ),
                  onActivate: (window, rect) {
                    debugPrint(
                      'Win10X QA: fixture selected exact window ${window.id}',
                    );
                  },
                  onHome: handle.close,
                  onClose: (id) {
                    requested.add(id);
                    if (scenario != 'tasks-refused')
                      update(
                        () =>
                            windows = windows.where((w) => w.id != id).toList(),
                      );
                  },
                ),
              ),
            ),
          );
      await _paintFrame();
      await _paintFrame();
      if (scenario == 'tasks-board') {
        debugPrint(
          'Win10X QA: task layout fixture; no native windows fabricated in session bridge',
        );
        return;
      }
      var box = _semanticBox('切换到 Chromium');
      final start = box.localToGlobal(
        Offset(box.size.width / 2, box.size.height * .3),
      );
      await _dragBetween(start, start + Offset(0, box.size.height * .4));
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (requested.length != 1 || requested.single != 11)
        throw StateError(
          'Slow distance dismissal did not request exact window11',
        );
      if (scenario == 'tasks-refused') {
        await Future<void>.delayed(const Duration(milliseconds: 900));
        final button = _findElement(
          (e) =>
              e.widget is IconButton &&
              (e.widget as IconButton).tooltip == '关闭 Chromium',
        )!;
        if ((button.widget as IconButton).onPressed == null)
          throw StateError('Unclosed native card stayed hidden/disabled');
        debugPrint(
          'Win10X QA: refused/confirmation-pending close restored the still-live card',
        );
      } else {
        if (windows.any((w) => w.id == 11))
          throw StateError('Fixture close did not remove confirmed window');
        debugPrint(
          'Win10X QA: distance-based downward close targeted stable window11; no premature native removal',
        );
      }
      return;
    }
    if (scenario == 'client-proportions') {
      final checked = Completer<void>();
      ref
          .read(shellPopupControllerProvider.notifier)
          .show(
            debugLabel: 'Win10X client geometry fixture',
            transitionDuration: Duration.zero,
            builder: (context, handle) => _ClientGeometryFixture(
              onChecked: (error) {
                if (checked.isCompleted) return;
                if (error == null) {
                  checked.complete();
                } else {
                  checked.completeError(error);
                }
              },
            ),
          );
      await checked.future.timeout(const Duration(seconds: 5));
      return;
    }
    if (scenario == 'panel-interrupt') {
      final services = ShellNavigation.instance.services;
      if (services == null) throw StateError('Shell services not ready');
      DashboardReveal.followOpen(services: services, progress: .7);
      final settle = DashboardReveal.settleOpen(
        services: services,
        from: .7,
        velocity: 0,
      );
      await Future<void>.delayed(const Duration(milliseconds: 50));
      DashboardReveal.setProgress(.25);
      await settle;
      if (ShellNavigation.instance.dashboardReveal.value != .25) {
        throw StateError('Old settle overwrote new finger progress');
      }
      await DashboardReveal.settleOpen(
        services: services,
        from: .25,
        velocity: 0,
      );
      if (ShellNavigation.instance.dashboardVisible)
        throw StateError('Cancelled panel stayed open');
      debugPrint(
        'Win10X QA: interrupted settle respected new drag; cancelled panel closed',
      );
      return;
    }
    if (scenario == 'start-toggle') {
      final nav = ShellNavigation.instance;
      final services = nav.services;
      if (services == null || !nav.launcherVisible)
        throw StateError('Start not ready');
      nav.home(services);
      await Future<void>.delayed(const Duration(milliseconds: 1800));
      if (nav.launcherVisible)
        throw StateError('Second Start did not close tiles');
      nav.home(services);
      await Future<void>.delayed(const Duration(milliseconds: 1200));
      if (!nav.launcherVisible) throw StateError('Start did not reopen');
      nav.send(StartRequest.apps);
      await Future<void>.delayed(const Duration(milliseconds: 400));
      nav.home(services);
      await Future<void>.delayed(const Duration(milliseconds: 400));
      if (!nav.launcherVisible)
        throw StateError('Start closed instead of returning from apps');
      debugPrint('Win10X QA: Start close/reopen and All Apps to tiles passed');
      return;
    }
    if (scenario == 'popup-lifecycle' || scenario == 'phone-menu') {
      final menu = WpPopups.applicationBarMenu(
        ref,
        children: [
          const FluentMenuItem(label: '应用信息'),
          const FluentMenuItem(label: '取消固定'),
        ],
      );
      await Future<void>.delayed(const Duration(milliseconds: 500));
      if (!mounted) return;
      if (ref.read(shellPopupControllerProvider).isEmpty) {
        throw StateError('ApplicationBar failed to open');
      }
      if (scenario == 'phone-menu') {
        debugPrint('Win10X QA: ApplicationBar opened with stable controller');
        return;
      }
      menu.close();
      await Future<void>.delayed(const Duration(milliseconds: 650));
      if (!mounted) return;
      if (ref.read(shellPopupControllerProvider).isNotEmpty) {
        throw StateError('ApplicationBar did not finish closing');
      }
      final result = WpPopups.messageBox(ref, title: '应用信息', body: 'QA dialog');
      await _paintFrame();
      if (!mounted) return;
      final dialog = ref.read(shellPopupControllerProvider).last;
      if (!WpPopups.dismiss(dialog.id)) throw StateError('Unowned dialog');
      await result.timeout(const Duration(seconds: 2));
      if (!mounted) return;
      final interrupted = WpPopups.messageBox(ref, title: '取消测试', body: 'QA');
      await _paintFrame();
      if (!mounted) return;
      ref.read(shellPopupControllerProvider.notifier).dismissAllImmediately();
      await interrupted.timeout(const Duration(seconds: 2));
      final unbuilt = WpPopups.messageBox(ref, title: '立即取消', body: 'QA');
      ref.read(shellPopupControllerProvider.notifier).dismissAllImmediately();
      await unbuilt.timeout(const Duration(seconds: 2));
      debugPrint(
        'Win10X QA: menu close, dialog Back and forced disposal completed',
      );
      return;
    }
    if (scenario == 'dialog-overflow') {
      unawaited(
        WpPopups.messageBox(
          ref,
          title: '应用信息',
          body: List.filled(
            20,
            '长应用名称 /usr/share/applications/example.desktop',
          ).join('\n'),
        ),
      );
      debugPrint('Win10X QA: long dialog opened');
      return;
    }
    if (scenario == 'toast-feedback') {
      WpToast.show(
        ref,
        '已固定到开始屏幕',
        hold: const Duration(seconds: 8),
        actions: [(label: '撤销', onTap: () {})],
      );
      await _paintFrame();
      if (!mounted) return;
      final interactions = ref.read(shellInteractionRegistryProvider);
      if (interactions.capturesFullScene || interactions.capturesKeyboard) {
        throw StateError('Feedback toast captured modal input');
      }
      debugPrint('Win10X QA: toast is non-modal and leaves keyboard available');
      return;
    }
    if (scenario == 'all-apps') {
      ShellNavigation.instance.send(StartRequest.apps);
      debugPrint('Win10X preview exercise: all-apps');
      return;
    }
    if (scenario.startsWith('tap:') || scenario.startsWith('hold:')) {
      final label = scenario.substring(scenario.indexOf(':') + 1);
      Offset? point;
      void find(Element element) {
        if (element.widget is Semantics &&
            (element.widget as Semantics).properties.label == label) {
          final box = element.findRenderObject();
          if (box is RenderBox && box.hasSize)
            point = box.localToGlobal(box.size.center(Offset.zero));
        }
        element.visitChildren(find);
      }

      (_boundary.currentContext! as Element).visitChildren(find);
      if (point == null) throw StateError('No target for $label');
      GestureBinding.instance.handlePointerEvent(
        PointerDownEvent(
          pointer: 80,
          position: point!,
          kind: PointerDeviceKind.touch,
        ),
      );
      if (scenario.startsWith('hold:'))
        await Future<void>.delayed(const Duration(milliseconds: 700));
      if (!mounted) return;
      GestureBinding.instance.handlePointerEvent(
        PointerUpEvent(
          pointer: 80,
          position: point!,
          kind: PointerDeviceKind.touch,
          timeStamp: const Duration(milliseconds: 750),
        ),
      );
      debugPrint('Win10X preview exercise: $scenario');
      return;
    }
    if (scenario == 'settings' || scenario == 'audio') {
      Element? target;
      void find(Element element) {
        final widget = element.widget;
        if ((scenario == 'settings' &&
                widget is FluentIconButton &&
                widget.tooltip == '开始设置') ||
            (scenario == 'audio' &&
                widget is FluentMenuItem &&
                widget.label == '声音输出与应用音量'))
          target = element;
        element.visitChildren(find);
      }

      (_boundary.currentContext! as Element).visitChildren(find);
      if (target == null) return;
      await Scrollable.ensureVisible(
        target!,
        duration: const Duration(milliseconds: 100),
      );
      if (!mounted) return;
      final box = target!.findRenderObject()! as RenderBox;
      final point = box.localToGlobal(box.size.center(Offset.zero));
      GestureBinding.instance.handlePointerEvent(
        PointerDownEvent(
          pointer: 78,
          position: point,
          kind: PointerDeviceKind.touch,
        ),
      );
      GestureBinding.instance.handlePointerEvent(
        PointerUpEvent(
          pointer: 78,
          position: point,
          timeStamp: const Duration(milliseconds: 40),
          kind: PointerDeviceKind.touch,
        ),
      );
      debugPrint('Win10X preview exercise: $scenario');
      return;
    }
    if (scenario == 'escape-close') {
      HardwareKeyboard.instance.handleKeyEvent(
        const KeyDownEvent(
          physicalKey: PhysicalKeyboardKey.escape,
          logicalKey: LogicalKeyboardKey.escape,
          timeStamp: Duration.zero,
        ),
      );
      HardwareKeyboard.instance.handleKeyEvent(
        const KeyUpEvent(
          physicalKey: PhysicalKeyboardKey.escape,
          logicalKey: LogicalKeyboardKey.escape,
          timeStamp: Duration(milliseconds: 1),
        ),
      );
      debugPrint('Win10X preview exercise: Escape close');
      return;
    }
    if (scenario == 'start-close') {
      Offset? point;
      void find(Element element) {
        if (element.widget case ShellButton(label: '开始')) {
          final box = element.findRenderObject();
          if (box is RenderBox && box.hasSize) {
            point = box.localToGlobal(box.size.center(Offset.zero));
          }
        }
        element.visitChildren(find);
      }

      (_boundary.currentContext! as Element).visitChildren(find);
      if (point != null) {
        GestureBinding.instance.handlePointerEvent(
          PointerDownEvent(
            pointer: 79,
            position: point!,
            kind: PointerDeviceKind.touch,
          ),
        );
        GestureBinding.instance.handlePointerEvent(
          PointerUpEvent(
            pointer: 79,
            position: point!,
            timeStamp: const Duration(milliseconds: 40),
            kind: PointerDeviceKind.touch,
          ),
        );
      }
      debugPrint(
        'Win10X preview exercise: Start button close (${point != null})',
      );
      return;
    }
    final box = _boundary.currentContext!.findRenderObject()! as RenderBox;
    final size = box.size;
    final start = scenario == 'edge-left'
        ? Offset(2, size.height * .4)
        : scenario == 'edge-top'
        ? Offset(size.width * .25, 2)
        : Offset(size.width - 2, size.height * .4);
    final travel = scenario == 'edge-left'
        ? const Offset(120, 0)
        : scenario == 'edge-top'
        ? const Offset(0, 120)
        : const Offset(-120, 0);
    final binding = GestureBinding.instance;
    binding.handlePointerEvent(
      PointerDownEvent(
        pointer: 77,
        position: start,
        kind: PointerDeviceKind.touch,
      ),
    );
    for (var i = 1; i <= 12; i++) {
      await Future<void>.delayed(const Duration(milliseconds: 12));
      if (!mounted) return;
      binding.handlePointerEvent(
        PointerMoveEvent(
          pointer: 77,
          position: start + travel * (i / 12),
          delta: travel / 12,
          timeStamp: Duration(milliseconds: i * 12),
          kind: PointerDeviceKind.touch,
        ),
      );
    }
    binding.handlePointerEvent(
      PointerUpEvent(
        pointer: 77,
        position: start + travel,
        timeStamp: const Duration(milliseconds: 160),
        kind: PointerDeviceKind.touch,
      ),
    );
    debugPrint('Win10X preview exercise: $scenario');
  }

  Future<void> _paintFrame() async {
    final binding = WidgetsBinding.instance;
    binding.scheduleForcedFrame();
    await binding.endOfFrame.timeout(const Duration(seconds: 12));
  }

  Future<void> _capture() async {
    if (!mounted) return;
    try {
      // The native Wayland host may withhold frames while occluded. Reading
      // the old boundary layer then returns the initial black/empty scene.
      await _paintFrame();
      await _paintFrame();
      if (!mounted) return;
      final boundary =
          _boundary.currentContext?.findRenderObject()
              as RenderRepaintBoundary?;
      if (boundary == null) return;
      void inspect(Element e) {
        if (e.widget.runtimeType.toString() == 'PhoneNavigationBar') {
          final box = e.findRenderObject();
          final fades = <double>[];
          e.visitAncestorElements((a) {
            if (a.widget is FadeTransition)
              fades.add((a.widget as FadeTransition).opacity.value);
            return true;
          });
          if (box is RenderBox && box.hasSize) {
            final p = box.localToGlobal(Offset.zero);
            debugPrint(
              'Win10X nav bounds: ${p.dx},${p.dy},${box.size.width},${box.size.height}; fades=$fades',
            );
          }
        }
        e.visitChildren(inspect);
      }

      (_boundary.currentContext! as Element).visitChildren(inspect);
      final bitmap = await boundary.toImage(pixelRatio: 1.5);
      try {
        final png = await bitmap.toByteData(format: ui.ImageByteFormat.png);
        if (png == null) throw StateError('PNG encoder returned no data');
        final file = File(widget.path);
        await file.parent.create(recursive: true);
        await file.writeAsBytes(
          png.buffer.asUint8List(png.offsetInBytes, png.lengthInBytes),
        );
        debugPrint('Win10X preview self-capture: ${widget.path}');
      } finally {
        bitmap.dispose();
      }
    } catch (error) {
      debugPrint('Win10X preview self-capture failed: $error');
    }
  }

  @override
  void dispose() {
    _timer?.cancel();
    _exerciseTimer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      RepaintBoundary(key: _boundary, child: widget.child);
}

// Deterministic retained/native-buffer composition fixture; never a fake native
// session acceptance result. Its real render transforms must preserve circles.
class _ClientGeometryFixture extends StatefulWidget {
  const _ClientGeometryFixture({required this.onChecked});
  final void Function(Object?) onChecked;
  @override
  State<_ClientGeometryFixture> createState() => _ClientGeometryFixtureState();
}

class _ClientGeometryFixtureState extends State<_ClientGeometryFixture> {
  final _source = GlobalKey();
  final _timers = <Timer>[];
  bool _portrait = false;
  @override
  void initState() {
    super.initState();
    _timers.add(
      Timer(const Duration(milliseconds: 200), () {
        if (mounted) setState(() => _portrait = true);
      }),
    );
    _timers.add(Timer(const Duration(milliseconds: 600), () => _check(false)));
    _timers.add(Timer(const Duration(milliseconds: 1300), () => _check(true)));
  }

  Future<void> _check(bool complete) async {
    if (!mounted) return;
    WidgetsBinding.instance.scheduleFrame();
    await WidgetsBinding.instance.endOfFrame;
    if (!mounted) return;
    try {
      final box = _source.currentContext!.findRenderObject()! as RenderBox;
      final zero = box.localToGlobal(Offset.zero);
      final x = box.localToGlobal(const Offset(100, 0));
      final y = box.localToGlobal(const Offset(0, 100));
      if (((x - zero).distance - (y - zero).distance).abs() > .01) {
        throw StateError('Client pixels were stretched differently on X/Y');
      }
      final point = box.globalToLocal(
        box.localToGlobal(const Offset(400, 300)),
      );
      if ((point - const Offset(400, 300)).distance > .01) {
        throw StateError('Client pointer inverse differs from paint transform');
      }
      debugPrint(
        'Win10X QA: client proportions and inverse hit transform passed ${complete ? "settled" : "in animation"}',
      );
      if (complete) widget.onChecked(null);
    } catch (error) {
      widget.onChecked(error);
    }
  }

  @override
  void dispose() {
    for (final timer in _timers) {
      timer.cancel();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xff202020),
    child: Stack(
      children: [
        RetainedAnimatedPositioned(
          rect: Rect.fromLTWH(24, 60, 320, _portrait ? 600 : 240),
          duration: const Duration(milliseconds: 900),
          curve: Curves.easeOutCubic,
          child: Win10xClientSurface(
            sourceSize: const Size(800, 600),
            frameWidth: 0,
            child: FittedBox(
              fit: BoxFit.fill,
              child: SizedBox(
                key: _source,
                width: 800,
                height: 600,
                child: CustomPaint(painter: _ClientGridPainter()),
              ),
            ),
          ),
        ),
      ],
    ),
  );
}

class _ClientGridPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawRect(
      Offset.zero & size,
      Paint()..color = const Color(0xfff0f0f0),
    );
    final line = Paint()
      ..color = const Color(0xffaaaaaa)
      ..strokeWidth = 2;
    for (var x = 0.0; x <= size.width; x += 50) {
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), line);
    }
    for (var y = 0.0; y <= size.height; y += 50) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), line);
    }
    canvas.drawCircle(
      size.center(Offset.zero),
      100,
      Paint()
        ..color = const Color(0xff0078d4)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 12,
    );
  }

  @override
  bool shouldRepaint(_ClientGridPainter oldDelegate) => false;
}
