// SPDX-License-Identifier: GPL-3.0-or-later
// Fluent presentation of Denial's existing PAM reply controller.
import 'package:flutter/material.dart';

/// A standard secure editor retains Flutter's pointer→requestKeyboard path.
/// Keep the upstream conversation-facing constructor; authentication stays Rust's.
class SecretField extends StatelessWidget {
  const SecretField({
    super.key,
    required this.controller,
    required this.focusNode,
    required this.placeholder,
    required this.echo,
    required this.enabled,
    required this.busy,
    required this.locked,
    required this.error,
    required this.pendingLength,
    required this.capsLock,
    required this.durationScale,
    required this.onSubmitted,
  });
  final TextEditingController controller;
  final FocusNode focusNode;
  final String placeholder;
  final bool echo, enabled, busy, locked, error, capsLock;
  final int pendingLength;
  final double durationScale;
  final VoidCallback onSubmitted;
  @override
  Widget build(BuildContext context) {
    final border = OutlineInputBorder(
      borderRadius: BorderRadius.zero,
      borderSide: BorderSide(
        color: error ? const Color(0xffc42b1c) : const Color(0xff8a8a8a),
      ),
    );
    return SizedBox(
      height: 48,
      child: TextField(
        controller: controller,
        focusNode: focusNode,
        enabled: enabled && !busy && !locked,
        obscureText: !echo,
        obscuringCharacter: '●',
        keyboardType: echo ? TextInputType.text : TextInputType.visiblePassword,
        autocorrect: false,
        enableSuggestions: false,
        enableIMEPersonalizedLearning: false,
        autofillHints: null,
        enableInteractiveSelection: false,
        textInputAction: TextInputAction.done,
        style: const TextStyle(fontSize: 15, color: Color(0xff1a1a1a)),
        cursorColor: const Color(0xff0078d4),
        cursorWidth: 2,
        decoration: InputDecoration(
          hintText: placeholder,
          filled: true,
          fillColor: Colors.white,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 12,
            vertical: 12,
          ),
          border: border,
          enabledBorder: border,
          disabledBorder: border,
          focusedBorder: const OutlineInputBorder(
            borderRadius: BorderRadius.zero,
            borderSide: BorderSide(color: Color(0xff0078d4), width: 2),
          ),
          suffixIcon: busy
              ? const Padding(
                  padding: EdgeInsets.all(14),
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : capsLock
              ? const Tooltip(
                  message: 'Caps Lock',
                  child: Center(widthFactor: 1, child: Text('⇪')),
                )
              : null,
        ),
        onSubmitted: (_) => onSubmitted(),
      ),
    );
  }
}
