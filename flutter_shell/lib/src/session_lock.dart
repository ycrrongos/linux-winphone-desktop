// SPDX-License-Identifier: GPL-3.0-only
import 'dart:async';
import 'dart:math' as math;

import 'package:denial_flutter_sdk/services.dart';
import 'package:denial_flutter_sdk/state.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'markui_icons.dart';
import 'model.dart';
import 'preferences.dart';
import 'user_wallpaper.dart';
import 'phone_lock_pane.dart';

/// Runs only inside Denial's secure lock layer. Sliding reveals credentials;
/// it never changes native lock state. Rust/PAM alone accepts authentication.
class Win10xLockScreen extends ConsumerStatefulWidget {
  const Win10xLockScreen({super.key, this.services});
  final ShellServices? services;
  @override
  ConsumerState<Win10xLockScreen> createState() => _Win10xLockScreenState();
}

class _Win10xLockScreenState extends ConsumerState<Win10xLockScreen> {
  final _password = TextEditingController();
  final _focus = FocusNode(debugLabel: 'WinPhone native unlock password');
  bool _credentials = false, _beginPending = false, _passwordMode = false;
  Timer? _clock;
  @override
  void initState() {
    super.initState();
    _clock = Timer.periodic(const Duration(seconds: 1), (_) {
      if (mounted) {
        setState(() {});
      }
    });
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        ref.read(authenticationProvider.notifier).offerFingerprint(true);
      }
    });
  }

  @override
  void dispose() {
    _clock?.cancel();
    _password.clear();
    _password.dispose();
    _focus.dispose();
    super.dispose();
  }

  void _begin(AuthenticationState auth) {
    if (!_credentials ||
        !_beginPending ||
        !auth.synchronized ||
        !auth.locked ||
        !auth.available ||
        auth.busy ||
        auth.rateLimited) {
      return;
    }
    _beginPending = false;
    ref
        .read(authenticationProvider.notifier)
        .begin(password: _passwordMode || !auth.pinConfigured);
  }

  void _reveal(bool value) {
    _password.clear();
    setState(() {
      _credentials = value;
      _beginPending = value;
    });
    ref.read(authenticationProvider.notifier).offerFingerprint(!value);
    if (value) {
      final auth = ref.read(authenticationProvider);
      _begin(auth);
      if (auth.prompt?.requiresResponse == true) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted && _credentials) {
            _focus.requestFocus();
          }
        });
      }
    } else {
      _focus.unfocus();
      ref.read(authenticationProvider.notifier).cancel();
    }
  }

  void _submit() {
    final auth = ref.read(authenticationProvider);
    if (auth.prompt?.requiresResponse == true && !auth.rateLimited) {
      final response = _password.text;
      _password.clear();
      ref.read(authenticationProvider.notifier).respond(response);
    } else {
      _beginPending = true;
      _begin(auth);
    }
  }

  @override
  Widget build(BuildContext context) {
    final auth = ref.watch(authenticationProvider);
    ref.listen<AuthenticationState>(authenticationProvider, (previous, next) {
      if (_credentials &&
          next.prompt?.requiresResponse == true &&
          previous?.prompt?.sequence != next.prompt?.sequence) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted && _credentials) _focus.requestFocus();
        });
      }
      if (_beginPending) {
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (mounted) {
            _begin(next);
          }
        });
      }
    });
    final time = widget.services == null
        ? DateTime.now()
        : ref.watch(widget.services!.clock).asData?.value ?? DateTime.now();
    final notices = ref.watch(desktopNotificationsProvider);
    final count = notices.groups.fold<int>(0, (n, g) => n + g.members.length);
    final battery = widget.services == null
        ? null
        : ref.watch(widget.services!.battery);
    final network = ref.watch(networkConnectivityProvider);
    final bluetooth = ref.watch(bluetoothProvider);
    return PhoneLockPane(
      connectivity: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (network.snapshot.connectedNetwork != null)
            const MarkUiAsset('ui/status_wifi', size: 14, color: Colors.white),
          if (bluetooth.powered)
            const Padding(
              padding: EdgeInsets.only(left: 8),
              child: MarkUiAsset(
                'ui/status_bluetooth',
                size: 14,
                color: Colors.white,
              ),
            ),
        ],
      ),
      time: time,
      notificationCount: count,
      batteryLabel: battery == null
          ? null
          : battery.capacity == null
          ? null
          : '${battery.capacity}%',
      wallpaper: const UserWallpaper(),
      credentialsVisible: _credentials,
      onCredentialsChanged: _reveal,
      onRevealSettled: () {
        if (_credentials &&
            ref.read(authenticationProvider).prompt?.requiresResponse == true) {
          _focus.requestFocus();
        }
      },
      credentials: Focus(
        onKeyEvent: (_, event) {
          if (event is KeyDownEvent &&
              event.logicalKey == LogicalKeyboardKey.escape) {
            _reveal(false);
            return KeyEventResult.handled;
          }
          return KeyEventResult.ignored;
        },
        child: LayoutBuilder(
          builder: (context, c) => SingleChildScrollView(
            padding: EdgeInsets.fromLTRB(
              24,
              math.min(140, c.maxHeight * .2),
              24,
              32,
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const Text(
                  '解锁',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 32,
                    fontWeight: FontWeight.w300,
                  ),
                ),
                const SizedBox(height: 28),
                Text(
                  auth.prompt?.message ??
                      (auth.pinConfigured && !_passwordMode
                          ? '输入 PIN'
                          : '输入账户密码'),
                  style: const TextStyle(color: Colors.white, fontSize: 18),
                ),
                const SizedBox(height: 12),
                TextField(
                  controller: _password,
                  focusNode: _focus,
                  obscureText: auth.prompt?.obscure ?? true,
                  keyboardType: auth.prompt?.isPin == true
                      ? TextInputType.number
                      : TextInputType.visiblePassword,
                  autocorrect: false,
                  enableSuggestions: false,
                  enableIMEPersonalizedLearning: false,
                  enabled:
                      _credentials &&
                      auth.prompt?.requiresResponse == true &&
                      !auth.rateLimited,
                  style: const TextStyle(color: Colors.white, fontSize: 22),
                  decoration: const InputDecoration(
                    hintText: '密码',
                    hintStyle: TextStyle(color: Colors.white54),
                    filled: false,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.zero,
                      borderSide: BorderSide(color: Colors.white),
                    ),
                  ),
                  onSubmitted: (_) => _submit(),
                ),
                if (auth.resultMessage case final message?)
                  Padding(
                    padding: const EdgeInsets.only(top: 12),
                    child: Text(
                      message,
                      style: TextStyle(
                        color: auth.resultIsError
                            ? const Color(0xffff8c8c)
                            : Colors.white,
                      ),
                    ),
                  ),
                if (auth.statusMessage case final message?)
                  Padding(
                    padding: const EdgeInsets.only(top: 12),
                    child: Text(
                      message,
                      style: const TextStyle(color: Colors.white70),
                    ),
                  ),
                const SizedBox(height: 20),
                Row(
                  children: [
                    TextButton(
                      onPressed: () => _reveal(false),
                      child: const Text(
                        '返回',
                        style: TextStyle(color: Colors.white),
                      ),
                    ),
                    if (auth.pinConfigured)
                      TextButton(
                        onPressed: () {
                          ref.read(authenticationProvider.notifier).cancel();
                          setState(() {
                            _passwordMode = !_passwordMode;
                            _beginPending = true;
                          });
                          _begin(ref.read(authenticationProvider));
                        },
                        child: Text(
                          _passwordMode ? '使用 PIN' : '使用密码',
                          style: const TextStyle(color: Colors.white),
                        ),
                      ),
                    const Spacer(),
                    FilledButton(
                      onPressed:
                          auth.available &&
                              !auth.rateLimited &&
                              (!auth.busy ||
                                  auth.prompt?.requiresResponse == true)
                          ? _submit
                          : null,
                      child: Text(
                        auth.busy && auth.prompt == null ? '正在验证…' : '解锁',
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// First-run OOBE: one step per page, large type (SS3).
class Win10xOobe extends StatefulWidget {
  const Win10xOobe({super.key});

  @override
  State<Win10xOobe> createState() => _Win10xOobeState();
}

class _Win10xOobeState extends State<Win10xOobe> {
  int _step = 0;

  void _next() {
    if (_step >= 2) {
      PreferenceController.instance.change((p) => p.oobeCompleted = true);
      return;
    }
    setState(() => _step++);
  }

  @override
  Widget build(BuildContext context) {
    final prefs = PreferenceController.instance.value;
    final titles = ['欢迎使用 Win10X', '选择外壳模式', '选择强调色'];
    return Material(
      color: const Color(0xff0078d4),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(28),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                titles[_step],
                style: const TextStyle(
                  color: Colors.white,
                  fontSize: 36,
                  fontFamily: 'Segoe UI',
                  fontWeight: FontWeight.w300,
                ),
              ),
              const SizedBox(height: 24),
              Expanded(
                child: switch (_step) {
                  0 => const Text(
                    '触控优先的 Linux 桌面。后续步骤可随时在设置中更改。',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 20,
                      height: 1.4,
                    ),
                  ),
                  1 => ListView(
                    children: [
                      for (final mode in ShellMode.values)
                        ListTile(
                          title: Text(
                            shellModeLabels[mode.index],
                            style: const TextStyle(color: Colors.white),
                          ),
                          trailing: prefs.mode == mode
                              ? const MarkUiIcon(
                                  Icons.check,
                                  color: Colors.white,
                                )
                              : null,
                          onTap: () {
                            PreferenceController.instance.change(
                              (p) => p.mode = mode,
                            );
                            setState(() {});
                          },
                        ),
                    ],
                  ),
                  _ => Wrap(
                    spacing: 12,
                    runSpacing: 12,
                    children: [
                      for (var i = 0; i < phoneAccentArgb.length; i++)
                        GestureDetector(
                          onTap: () {
                            PreferenceController.instance.change(
                              (p) => p.phoneAccentIndex = i,
                            );
                            setState(() {});
                          },
                          child: Container(
                            width: 48,
                            height: 48,
                            color: Color(phoneAccentArgb[i]),
                            child: prefs.phoneAccentIndex == i
                                ? const MarkUiIcon(
                                    Icons.check,
                                    color: Colors.white,
                                  )
                                : null,
                          ),
                        ),
                    ],
                  ),
                },
              ),
              Align(
                alignment: Alignment.centerRight,
                child: FilledButton(
                  style: FilledButton.styleFrom(
                    backgroundColor: Colors.white,
                    foregroundColor: const Color(0xff0078d4),
                  ),
                  onPressed: _next,
                  child: Text(_step >= 2 ? '完成' : '下一步'),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
