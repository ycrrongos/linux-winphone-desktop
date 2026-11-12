// SPDX-License-Identifier: GPL-3.0-only
import 'dart:io';

import 'package:flutter/widgets.dart';
import 'package:denial_flutter_sdk/rendering.dart';

import 'preferences.dart';

/// Same selection for desktop, Start and the secure picture cover.
class UserWallpaper extends StatelessWidget {
  const UserWallpaper({super.key});
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: PreferenceController.instance,
    builder: (context, _) {
      final path = PreferenceController.instance.value.desktopWallpaperPath;
      return path == null
          ? const ShellWallpaper()
          : Image.file(
              File(path),
              fit: BoxFit.cover,
              width: double.infinity,
              height: double.infinity,
              gaplessPlayback: true,
              errorBuilder: (_, error, stack) => const ShellWallpaper(),
            );
    },
  );
}
