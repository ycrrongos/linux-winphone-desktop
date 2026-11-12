import 'dart:async';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/rendering.dart';
// SPDX-License-Identifier: GPL-3.0-or-later
// Presentation fixture only; no agent connection or privileged operation.
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'agent.dart';
import 'main.dart';
import 'prompt.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  final session = AgentSession()
    ..phase = AgentPhase.prompting
    ..message = '界面预览：需要管理员授权以更改系统设置'
    ..action = 'win10x.preview.only'
    ..identities = const [AgentIdentity(uid: 1000, name: '预览账户')]
    ..selectedUid = 1000
    ..promptLabel = '密码（仅样式预览）';
  runApp(
    _Capture(
      child: ProviderScope(
        child: DialogApp(
          title: 'Fluent authorization — UI preview only',
          builder: (_, duration) => AuthenticationDialog(
            session: session,
            appearanceReady: true,
            durationScale: duration,
          ),
        ),
      ),
    ),
  );
}

class _Capture extends StatefulWidget {
  const _Capture({required this.child});
  final Widget child;
  @override
  State<_Capture> createState() => _CaptureState();
}

class _CaptureState extends State<_Capture> {
  final key = GlobalKey();
  Timer? timer;
  @override
  void initState() {
    super.initState();
    timer = Timer(const Duration(seconds: 3), () async {
      final path = Platform.environment['WIN10X_AUTH_PREVIEW_CAPTURE'];
      if (path == null || !mounted) return;
      final image =
          await (key.currentContext!.findRenderObject()!
                  as RenderRepaintBoundary)
              .toImage(pixelRatio: 1.5);
      final bytes = await image.toByteData(format: ui.ImageByteFormat.png);
      if (bytes != null)
        await File(path).writeAsBytes(bytes.buffer.asUint8List());
      image.dispose();
    });
  }

  @override
  void dispose() {
    timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      RepaintBoundary(key: key, child: widget.child);
}
