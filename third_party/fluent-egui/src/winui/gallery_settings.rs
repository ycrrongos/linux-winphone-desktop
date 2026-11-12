use eframe::egui::{RichText, Ui};

use super::{
    FluentButton, FluentIcon, FluentTheme, HyperlinkButton, SettingsExpander,
    SettingsNavigationRow, ToggleSwitch, combo_option, fluent_combo_box, section_card, setting_row,
    settings_group,
};

/// A Fluent API value representing `GallerySettings`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GallerySettings<'a> {
    theme: &'a mut FluentTheme,
    navigation_style: &'a mut usize,
    sound_expanded: &'a mut bool,
    sound_enabled: bool,
    spatial_audio: &'a mut bool,
    about_expanded: &'a mut bool,
    status_message: Option<&'a str>,
}

impl<'a> GallerySettings<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        theme: &'a mut FluentTheme,
        navigation_style: &'a mut usize,
        sound_expanded: &'a mut bool,
        sound_enabled: bool,
        spatial_audio: &'a mut bool,
        about_expanded: &'a mut bool,
    ) -> Self {
        Self {
            theme,
            navigation_style,
            sound_expanded,
            sound_enabled,
            spatial_audio,
            about_expanded,
            status_message: None,
        }
    }

    /// Configures or reads the `status_message` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn status_message(mut self, message: &'a str) -> Self {
        if !message.is_empty() {
            self.status_message = Some(message);
        }
        self
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> GallerySettingsResponse {
        let previous_theme = *self.theme;
        let mut response = GallerySettingsResponse::default();

        ui.label(RichText::new("Appearance & behavior").size(15.0).strong());
        ui.add_space(6.0);
        settings_group(ui, |ui| {
            setting_row(
                ui,
                *self.theme,
                "App theme",
                "Select which app theme to display",
                |ui| {
                    let mut selected = usize::from(self.theme.is_dark());
                    fluent_combo_box(
                        "gallery_settings_theme",
                        ["Light", "Dark", "Use system setting"][selected],
                    )
                    .width(190.0)
                    .show_ui(ui, |ui| {
                        for (index, label) in
                            ["Light", "Dark", "Use system setting"].iter().enumerate()
                        {
                            if combo_option(ui, *label, selected == index).clicked() {
                                selected = index;
                                *self.theme = if index == 1 {
                                    FluentTheme::Dark
                                } else {
                                    FluentTheme::Light
                                };
                                ui.close();
                            }
                        }
                    });
                },
            );
            setting_row(
                ui,
                *self.theme,
                "Navigation style",
                "Choose where primary navigation is displayed",
                |ui| {
                    let labels = ["Left", "Top"];
                    fluent_combo_box(
                        "gallery_settings_navigation",
                        labels[(*self.navigation_style).min(1)],
                    )
                    .width(120.0)
                    .show_ui(ui, |ui| {
                        for (index, label) in labels.iter().enumerate() {
                            if combo_option(ui, *label, *self.navigation_style == index).clicked() {
                                *self.navigation_style = index;
                                ui.close();
                            }
                        }
                    });
                },
            );
            SettingsExpander::new(
                "gallery_settings_sound",
                self.sound_expanded,
                "Sound",
                "Controls provide audible feedback",
            )
            .icon(FluentIcon::Volume)
            .show(ui, |ui| {
                setting_row(
                    ui,
                    *self.theme,
                    "Enable Spatial Audio",
                    "Learn more about enabling sounds in your app",
                    |ui| {
                        ui.add_enabled(self.sound_enabled, ToggleSwitch::new(self.spatial_audio));
                    },
                );
            });
            setting_row(
                ui,
                *self.theme,
                "Manage samples",
                "Clear your recent or favorite samples",
                |ui| {
                    ui.horizontal(|ui| {
                        response.clear_recents =
                            ui.add(FluentButton::new("Clear recents")).clicked();
                        response.remove_favorites =
                            ui.add(FluentButton::new("Remove favorites")).clicked();
                    });
                },
            );
        });
        if let Some(message) = self.status_message {
            ui.label(RichText::new(message).color(ui.visuals().weak_text_color()));
        }

        ui.add_space(22.0);
        ui.label(RichText::new("About").size(15.0).strong());
        ui.add_space(6.0);
        settings_group(ui, |ui| {
            SettingsExpander::new(
                "gallery_settings_about",
                self.about_expanded,
                "WinUI 3 Gallery",
                "Copyright 2026 Microsoft. All rights reserved.",
            )
            .icon(FluentIcon::Info)
            .show(ui, |ui| {
                SettingsNavigationRow::new(
                    "To clone this repository",
                    "git clone https://github.com/microsoft/WinUI-Gallery",
                    FluentIcon::Copy,
                )
                .show(ui);
                SettingsNavigationRow::new(
                    "File a bug or request new sample",
                    "Open the WinUI Gallery issue tracker",
                    FluentIcon::Globe,
                )
                .show(ui);
                section_card(
                    ui,
                    *self.theme,
                    "Dependencies & references",
                    "Runtime and design libraries used by the official sample",
                    |ui| {
                        for dependency in [
                            "Windows App SDK",
                            "WinUI 3",
                            "Windows Community Toolkit",
                            "ColorCode-Universal",
                            "Win2D",
                            "CsWin32",
                        ] {
                            ui.add(HyperlinkButton::new(dependency));
                        }
                    },
                );
            });
        });

        response.theme_changed = previous_theme != *self.theme;
        response
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// A Fluent API value representing `GallerySettingsResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GallerySettingsResponse {
    /// The `theme_changed` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub theme_changed: bool,
    /// The `clear_recents` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub clear_recents: bool,
    /// The `remove_favorites` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub remove_favorites: bool,
}
