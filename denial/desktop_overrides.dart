// SPDX-License-Identifier: GPL-3.0-only
import 'package:denial_flutter_sdk/services.dart';
import 'package:flutter/widgets.dart';

/// Composition hooks in the pinned reference host. The native dashboard keeps
/// its existing focus, dismissal and shortcut lifecycle; only its content
/// changes. The decorator can wrap the scene with gesture chrome and service
/// scope without replacing the host tree.
Widget Function(BuildContext, VoidCallback, VoidCallback)?
win10xDashboardBuilder;
Widget Function(BuildContext, ShellServices, Widget)? win10xDesktopDecorator;

/// Lets dashboard content and gesture chrome read the host [ShellServices]
/// without threading it through every Denial panel constructor.
class Win10xShellServicesScope extends InheritedWidget {
  const Win10xShellServicesScope({
    super.key,
    required this.services,
    required super.child,
  });

  final ShellServices services;

  static ShellServices of(BuildContext context) {
    final scope = context
        .dependOnInheritedWidgetOfExactType<Win10xShellServicesScope>();
    assert(scope != null, 'Win10xShellServicesScope is missing');
    return scope!.services;
  }

  static ShellServices? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<Win10xShellServicesScope>()
      ?.services;

  @override
  bool updateShouldNotify(Win10xShellServicesScope oldWidget) =>
      services != oldWidget.services;
}

// Evaluated before each close so user-selected stagger durations stay intact.
Duration Function()? win10xLauncherCloseDuration;

// Mode is a composition policy, separate from the user's tile-layout choice.
// 0 = Windows 10 tablet, 1 = desktop, 2 = Windows 10 Mobile.
final win10xShellMode = ValueNotifier<int>(0);
final win10xReachability = ValueNotifier<double>(0);

/// When non-null, Action Center translation tracks this 0..1 progress instead
/// of the host panel controller (finger-follow open/close).
final win10xDashboardReveal = ValueNotifier<double?>(null);
Rect Function(Rect)? win10xLauncherBounds;
Rect Function(Rect)? win10xDashboardBounds;
void Function(bool, bool)? win10xPanelObserver;
VoidCallback? win10xDismissKeyboard;
void Function(
  VoidCallback,
  VoidCallback,
  VoidCallback,
  VoidCallback,
  VoidCallback,
  VoidCallback,
)?
win10xBindCommands;

// Presentation only, hosted inside the native secure stage. Never hide the lock
// based on a local gesture: only Rust's authentication result unlocks the seat.
Widget Function(BuildContext, Animation<double>)? win10xLockBuilder;

final win10xAudioHudRevision = ValueNotifier<int>(0);

// Only set by a native session with the custom ScreenPowerHost installed.
bool win10xManagedScreenPower = false;

/// Native audio HUD presentation: one root above both desktop and secure lock.
Widget Function(BuildContext, int, bool, VoidCallback)?
win10xPhoneAudioHudBuilder;
