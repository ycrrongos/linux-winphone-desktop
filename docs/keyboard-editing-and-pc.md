# Editing tools and full computer keyboard

Interaction reference: Fcitx5 Android,
revisioneb5b50216e70484a009933d9519db76005f1faf3:
- input/bar/ui/idle/ButtonsBarUi.kt (editing/undo/redo toolbar entry)
- input/editing/TextEditingWindow.kt and TextEditingUi.kt (directional selection,
  Home/End and copy/cut/paste/select-all)
- input/keyboard/BaseKeyboard.kt (space swipe to move cursor)
Repository: https://github.com/fcitx5-android/fcitx5-android . Its LGPL Kotlin code
and Android assets were studied for behavior, not copied into the implementation.

The WP keyboard has persistent Edit and Computer entries. The middle command
strip scrolls independently. Editing replaces the key area with arrows, line
start/end, page navigation, word movement, selection, select-all/copy/cut/paste,
undo/redo and deletion. Selection maps to Shift+direction, word movement to Ctrl.
Clipboard actions go to the active application as real shortcuts; no clipboard
history is collected or displayed. Password/PIN/sensitive editors disable
copy/cut/select-all and do not retain tools mode across a focus change.

Space swipe keeps fractional travel: one arrow per12 logical pixels after the
threshold. It claims the horizontal keyboard gesture, continues after its first
step and consumes release rather than inserting Space. Cancel stops immediately.
Pending pinyin is committed before navigation/shortcuts rather than discarded.

Computer mode has F1–F12, Escape/Tab, numbers/punctuation, navigation, modifiers,
software Caps, Fn and a separate keypad. It scrolls horizontally on a phone and
vertically when the viewport is too short; no stretched/squashed keys. Unmodified
letters and Space still use the Chinese composition/candidate path when Chinese
mode is active. Modified combinations go directly to the application.

Modifier taps arm one stroke, long press locks the UI state, tapping again
unlocks. Each actual chord presses its modifiers, taps its key and releases the
modifiers in reverse order; nothing is held natively between user strokes. Focus,
hide and input-purpose changes discard armed/locked state and cancel repeaters.
Fn is a software layer (digits→F1–F12, arrows→Home/End/PageUp/PageDown,
Backspace→Delete), not a fictitious hardware Fn scancode. Arrow/delete repeaters
stop on cancellation/disposal; switches/pressed colors use WP nonlinear easing.

The pinned upstream native named-key table lacked PC navigation, function and
modifier keys. denial/native/pc-keyboard.patch and tools/win10x-denial-native
supply them using the existing SDK protocol. Packaging requires a verified
patched native broker. See denial/native/README.md for x64/ARM64 reproduction.
Native secure shortcut policy is unchanged (including the existing locked
hardware-volume restriction). Application shortcut support varies; this layer
does not invent application undo/selection behavior when the application lacks it.
