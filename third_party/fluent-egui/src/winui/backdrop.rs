use super::FluentTheme;

/// Native Windows backdrop material.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BackdropKind {
    /// Let DWM select the appropriate material. The shell currently resolves this to Mica.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    #[default]
    Auto,
    /// The `Mica` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Mica,
    /// The `Acrylic` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Acrylic,
    /// The `Tabbed` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Tabbed,
    /// Legacy Aero Glass on Windows 7, with Background Acrylic fallback on newer Windows.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Aero,
    /// The `None` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    None,
}

/// Result of attempting to configure the native window material.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackdropStatus {
    /// The `Applied` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Applied,
    /// The `Unsupported` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Unsupported,
    /// The `Failed` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Failed(i32),
}

/// Synchronizes DWM dark mode, rounded corners, and backdrop material for a winit window.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn sync_window_backdrop(
    window: &winit::window::Window,
    theme: FluentTheme,
    kind: BackdropKind,
) -> BackdropStatus {
    platform::sync_window_backdrop(window, theme, kind)
}

#[cfg(windows)]
mod platform {
    use std::{cell::Cell, ffi::c_void, mem::size_of};

    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::{
        Foundation::HWND,
        Graphics::Dwm::{
            DWM_BB_ENABLE, DWM_BLURBEHIND, DWMWA_SYSTEMBACKDROP_TYPE,
            DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_WINDOW_CORNER_PREFERENCE,
            DwmEnableBlurBehindWindow, DwmExtendFrameIntoClientArea, DwmSetWindowAttribute,
        },
        UI::Controls::MARGINS,
    };

    use super::{BackdropKind, BackdropStatus, FluentTheme};

    thread_local! {
        static LAST_BACKDROP: Cell<Option<(isize, bool, i32, bool)>> = const { Cell::new(None) };
    }

    pub(super) fn sync_window_backdrop(
        window: &winit::window::Window,
        theme: FluentTheme,
        kind: BackdropKind,
    ) -> BackdropStatus {
        let Ok(window_handle) = window.window_handle() else {
            return BackdropStatus::Unsupported;
        };
        let RawWindowHandle::Win32(handle) = window_handle.as_raw() else {
            return BackdropStatus::Unsupported;
        };
        let hwnd = handle.hwnd.get() as HWND;
        let dark_mode: i32 = i32::from(theme.is_dark());
        let material_key: i32 = match kind {
            BackdropKind::Auto | BackdropKind::Mica => 2, // DWMSBT_MAINWINDOW
            BackdropKind::Acrylic => 3,                   // DWMSBT_TRANSIENTWINDOW
            BackdropKind::Tabbed => 4,                    // DWMSBT_TABBEDWINDOW
            BackdropKind::Aero => 5,
            BackdropKind::None => 1, // DWMSBT_NONE
        };

        let previous = LAST_BACKDROP.with(|last| last.get());
        let cache_key = (handle.hwnd.get(), theme.is_dark(), material_key);
        if previous.is_some_and(|state| (state.0, state.1, state.2) == cache_key) {
            return BackdropStatus::Applied;
        }

        let window_changed = previous.is_none_or(|state| state.0 != handle.hwnd.get());
        let theme_changed = previous.is_none_or(|state| state.1 != theme.is_dark());
        let material_changed = previous.is_none_or(|state| state.2 != material_key);

        let mut legacy_fallback = previous.is_some_and(|state| state.3);
        if material_changed || window_changed {
            // A negative margin makes DWM's material cover the entire OpenGL client area.
            let margin = if matches!(kind, BackdropKind::None) {
                0
            } else {
                -1
            };
            let margins = MARGINS {
                cxLeftWidth: margin,
                cxRightWidth: margin,
                cyTopHeight: margin,
                cyBottomHeight: margin,
            };
            let extend_result = unsafe { DwmExtendFrameIntoClientArea(hwnd, &margins) };
            if extend_result < 0 {
                return BackdropStatus::Failed(extend_result);
            }

            if matches!(kind, BackdropKind::Aero) {
                if let Err(error) = enable_legacy_aero(hwnd) {
                    return BackdropStatus::Failed(error);
                }
                legacy_fallback = true;
                // Windows 11 no longer has Aero Glass; use Background Acrylic when available.
                // Failure is expected on Windows 7 and keeps the legacy blur active.
                if set_attribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE, 3).is_ok() {
                    let _ = disable_legacy_aero(hwnd);
                    legacy_fallback = false;
                }
            } else if matches!(kind, BackdropKind::None) {
                // Removing a backdrop must also clear a blur enabled by an earlier fallback.
                let _ = disable_legacy_aero(hwnd);
                legacy_fallback = false;
                // Unsupported is expected before Windows 11; the zero margins and disabled
                // blur above are sufficient for the requested opaque fallback.
                let _ = set_attribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE, material_key);
            } else {
                let backdrop = material_key;
                if let Err(error) = set_attribute(hwnd, DWMWA_SYSTEMBACKDROP_TYPE, backdrop) {
                    if let Err(blur_error) = enable_legacy_aero(hwnd) {
                        return BackdropStatus::Failed(if blur_error < 0 {
                            blur_error
                        } else {
                            error
                        });
                    }
                    legacy_fallback = true;
                } else {
                    let _ = disable_legacy_aero(hwnd);
                    legacy_fallback = false;
                }
            }

            if let Err(error) = set_attribute(hwnd, DWMWA_WINDOW_CORNER_PREFERENCE, 2)
                && !legacy_fallback
                && !matches!(kind, BackdropKind::None)
            {
                return BackdropStatus::Failed(error);
            }
        }

        if (theme_changed || window_changed)
            && let Err(error) = set_attribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, dark_mode)
            && !legacy_fallback
            && !matches!(kind, BackdropKind::None)
        {
            return BackdropStatus::Failed(error);
        }
        LAST_BACKDROP.with(|last| {
            last.set(Some((
                cache_key.0,
                cache_key.1,
                cache_key.2,
                legacy_fallback,
            )))
        });
        BackdropStatus::Applied
    }

    fn set_attribute(hwnd: HWND, attribute: i32, value: i32) -> Result<(), i32> {
        let result = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                attribute as u32,
                (&value as *const i32).cast::<c_void>(),
                size_of::<i32>() as u32,
            )
        };
        if result < 0 { Err(result) } else { Ok(()) }
    }

    fn enable_legacy_aero(hwnd: HWND) -> Result<(), i32> {
        set_legacy_aero(hwnd, true)
    }

    fn disable_legacy_aero(hwnd: HWND) -> Result<(), i32> {
        set_legacy_aero(hwnd, false)
    }

    fn set_legacy_aero(hwnd: HWND, enabled: bool) -> Result<(), i32> {
        let blur = DWM_BLURBEHIND {
            dwFlags: DWM_BB_ENABLE,
            fEnable: i32::from(enabled),
            hRgnBlur: std::ptr::null_mut(),
            fTransitionOnMaximized: 0,
        };
        let result = unsafe { DwmEnableBlurBehindWindow(hwnd, &blur) };
        if result < 0 { Err(result) } else { Ok(()) }
    }
}

#[cfg(not(windows))]
mod platform {
    use super::{BackdropKind, BackdropStatus, FluentTheme};

    pub(super) fn sync_window_backdrop(
        _window: &winit::window::Window,
        _theme: FluentTheme,
        _kind: BackdropKind,
    ) -> BackdropStatus {
        BackdropStatus::Unsupported
    }
}
