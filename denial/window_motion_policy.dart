// SPDX-License-Identifier: GPL-3.0-only
// Installed only into our prepared Denial reference-plugin copy.
import 'package:flutter/widgets.dart';

/// Window motion is independent of Start's entrance and launch settings.
final win10xWindowMotionEnabled = ValueNotifier<bool>(true);

class Win10xWindowMotion extends StatelessWidget {
  const Win10xWindowMotion({super.key, required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) => ValueListenableBuilder<bool>(
    valueListenable: win10xWindowMotionEnabled,
    child: child,
    builder: (context, enabled, child) => MediaQuery(
      data: MediaQuery.of(context).copyWith(
        disableAnimations: MediaQuery.disableAnimationsOf(context) || !enabled,
      ),
      child: child!,
    ),
  );
}
