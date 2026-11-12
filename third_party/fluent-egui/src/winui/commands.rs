use eframe::egui::{
    self, Align, Align2, Color32, InnerResponse, Layout, Margin, Response, Sense, Stroke, Ui, Vec2,
    Widget, pos2, vec2,
};

use super::{
    FluentButton, FluentIcon, FluentTheme, IconButton, icon_font_id, navigation_item, palette,
};

fn current_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Available `StandardUICommandKind` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum StandardUICommandKind {
    /// The `Copy` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Copy,
    /// The `Cut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Cut,
    /// The `Paste` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Paste,
    /// The `Undo` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Undo,
    /// The `Redo` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Redo,
    /// The `Delete` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Delete,
    /// The `Save` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Save,
    /// The `Open` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Open,
}

impl StandardUICommandKind {
    /// Configures or reads the `label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn label(self) -> &'static str {
        match self {
            Self::Copy => "Copy",
            Self::Cut => "Cut",
            Self::Paste => "Paste",
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Delete => "Delete",
            Self::Save => "Save",
            Self::Open => "Open",
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(self) -> FluentIcon {
        match self {
            Self::Copy => FluentIcon::Copy,
            Self::Cut => FluentIcon::Cut,
            Self::Paste => FluentIcon::Paste,
            Self::Undo => FluentIcon::Undo,
            Self::Redo => FluentIcon::Redo,
            Self::Delete => FluentIcon::Delete,
            Self::Save => FluentIcon::Save,
            Self::Open => FluentIcon::OpenFile,
        }
    }
}

/// A standard semantic command rendered as a Fluent command button.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct StandardUICommand {
    kind: StandardUICommandKind,
    show_label: bool,
}

impl StandardUICommand {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(kind: StandardUICommandKind) -> Self {
        Self {
            kind,
            show_label: true,
        }
    }

    /// Configures or reads the `show_label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }
}

impl Widget for StandardUICommand {
    fn ui(self, ui: &mut Ui) -> Response {
        if self.show_label {
            ui.add(FluentButton::subtle(self.kind.label()).icon(self.kind.icon()))
        } else {
            ui.add(IconButton::new(self.kind.icon()))
                .on_hover_text(self.kind.label())
        }
    }
}

/// A WinUI ListView-style command target used to demonstrate a shared command.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CommandList<'a> {
    items: &'a [String],
    selected: &'a mut Option<usize>,
    command: StandardUICommandKind,
    row_height: f32,
}

impl<'a> CommandList<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        items: &'a [String],
        selected: &'a mut Option<usize>,
        command: StandardUICommandKind,
    ) -> Self {
        Self {
            items,
            selected,
            command,
            row_height: 60.0,
        }
    }

    /// Configures or reads the `row_height` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn row_height(mut self, row_height: f32) -> Self {
        self.row_height = row_height;
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
    pub fn show(self, ui: &mut Ui) -> CommandListResponse {
        let p = current_palette(ui);
        let mut invoked = None;
        let mut selection_changed = false;
        let mut combined_response: Option<Response> = None;

        ui.spacing_mut().item_spacing.y = 0.0;
        for (index, item) in self.items.iter().enumerate() {
            let (rect, row_response) = ui.allocate_exact_size(
                vec2(ui.available_width(), self.row_height),
                Sense::click_and_drag(),
            );
            let selected = *self.selected == Some(index);
            let fill = if selected {
                p.selection
            } else if row_response.hovered() {
                p.subtle_hover
            } else {
                Color32::TRANSPARENT
            };
            ui.painter().rect_filled(rect, 0, fill);
            ui.painter().line_segment(
                [
                    pos2(rect.left(), rect.bottom()),
                    pos2(rect.right(), rect.bottom()),
                ],
                Stroke::new(1.0, p.card_border),
            );

            if row_response.secondary_clicked() {
                *self.selected = Some(index);
                selection_changed = true;
            }
            row_response.context_menu(|ui| {
                if ui.add(StandardUICommand::new(self.command)).clicked() {
                    *self.selected = Some(index);
                    invoked = Some(index);
                    ui.close();
                }
            });

            let mut command_clicked = false;
            ui.scope_builder(
                egui::UiBuilder::new()
                    .id_salt(("command_list_row", index))
                    .max_rect(rect)
                    .layout(Layout::left_to_right(Align::Center)),
                |ui| {
                    ui.set_clip_rect(rect);
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(item).size(14.0));
                    if row_response.hovered() || selected {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            let command_response = ui.add(StandardUICommand::new(self.command));
                            if command_response.clicked() {
                                command_clicked = true;
                                invoked = Some(index);
                            }
                        });
                    }
                },
            );

            if row_response.clicked() && !command_clicked {
                *self.selected = Some(index);
                selection_changed = true;
            }
            if row_response.drag_stopped() && row_response.drag_delta().x < -72.0 {
                *self.selected = Some(index);
                invoked = Some(index);
            }
            combined_response = Some(match combined_response {
                Some(response) => response.union(row_response),
                None => row_response,
            });
        }

        let mut response =
            combined_response.unwrap_or_else(|| ui.allocate_response(Vec2::ZERO, Sense::hover()));
        if selection_changed || invoked.is_some() {
            response.mark_changed();
        }
        CommandListResponse {
            response,
            invoked,
            selection_changed,
        }
    }
}

/// Result from [`CommandList::show`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CommandListResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `invoked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub invoked: Option<usize>,
    /// The `selection_changed` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub selection_changed: bool,
}

/// Application-defined command metadata corresponding to XamlUICommand.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct XamlUICommand {
    label: String,
    description: String,
    icon: Option<FluentIcon>,
    shortcut: Option<String>,
}

impl XamlUICommand {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            description: String::new(),
            icon: None,
            shortcut: None,
        }
    }

    /// Configures or reads the `description` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures or reads the `shortcut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Configures or reads the `show_app_bar` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_app_bar(self, ui: &mut Ui) -> Response {
        let icon = self.icon.unwrap_or(FluentIcon::Favorite);
        let mut tooltip = self.description;
        if let Some(shortcut) = self.shortcut {
            if !tooltip.is_empty() {
                tooltip.push('\n');
            }
            tooltip.push_str(&shortcut);
        }
        let response = ui.add(AppBarButton::new(self.label, icon));
        if tooltip.is_empty() {
            response
        } else {
            response.on_hover_text(tooltip)
        }
    }
}

impl Widget for XamlUICommand {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text = self.label;
        if let Some(shortcut) = self.shortcut {
            text.push_str("   ");
            text.push_str(&shortcut);
        }
        let response = ui.add(match self.icon {
            Some(icon) => FluentButton::subtle(text).icon(icon),
            None => FluentButton::subtle(text),
        });
        if self.description.is_empty() {
            response
        } else {
            response.on_hover_text(self.description)
        }
    }
}

#[derive(Clone, Debug)]
/// A Fluent API value representing `MenuItem`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MenuItem {
    /// The `text` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub text: String,
    /// The `icon` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub icon: Option<FluentIcon>,
    /// The `enabled` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub enabled: bool,
    /// The `checked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub checked: bool,
    /// The `separator_after` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub separator_after: bool,
    /// The `shortcut` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub shortcut: Option<String>,
    /// The `radio_group` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub radio_group: Option<String>,
    /// The `children` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            enabled: true,
            checked: false,
            separator_after: false,
            shortcut: None,
            radio_group: None,
            children: Vec::new(),
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures or reads the `enabled` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Configures or reads the `checked` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Configures or reads the `separator_after` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn separator_after(mut self, separator: bool) -> Self {
        self.separator_after = separator;
        self
    }

    /// Configures or reads the `shortcut` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Configures or reads the `radio_group` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn radio_group(mut self, group: impl Into<String>) -> Self {
        self.radio_group = Some(group.into());
        self
    }

    /// Configures or reads the `children` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn children(mut self, children: Vec<MenuItem>) -> Self {
        self.children = children;
        self
    }
}

fn show_menu_items(
    ui: &mut Ui,
    items: &[MenuItem],
    path: &mut Vec<usize>,
    selected: &mut Option<Vec<usize>>,
) {
    ui.set_min_width(190.0);
    for (index, item) in items.iter().enumerate() {
        path.push(index);
        if item.children.is_empty() {
            let mut label = item.text.clone();
            if let Some(shortcut) = &item.shortcut {
                label.push_str("    ");
                label.push_str(shortcut);
            }
            let response = ui.add_enabled(
                item.enabled,
                if item.checked {
                    FluentButton::subtle(label).icon(FluentIcon::Check)
                } else if let Some(icon) = item.icon {
                    FluentButton::subtle(label).icon(icon)
                } else {
                    FluentButton::subtle(label)
                },
            );
            if response.clicked() {
                *selected = Some(path.clone());
                ui.close();
            }
        } else {
            let mut label = item.text.clone();
            if item.checked {
                label = format!("On  {label}");
            }
            ui.add_enabled_ui(item.enabled, |ui| {
                ui.menu_button(label, |ui| {
                    show_menu_items(ui, &item.children, path, selected);
                });
            });
        }
        path.pop();
        if item.separator_after {
            ui.separator();
        }
    }
}

#[derive(Clone, Debug)]
/// A Fluent API value representing `MenuDefinition`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MenuDefinition {
    /// The `title` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub title: String,
    /// The `items` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub items: Vec<MenuItem>,
}

impl MenuDefinition {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

/// A top-level menu row corresponding to MenuBar.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MenuBar<'a> {
    menus: &'a [MenuDefinition],
}

impl<'a> MenuBar<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(menus: &'a [MenuDefinition]) -> Self {
        Self { menus }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) -> Option<(usize, usize)> {
        self.show_path(ui)
            .and_then(|(menu, path)| path.first().copied().map(|item| (menu, item)))
    }

    /// Configures or reads the `show_path` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_path(self, ui: &mut Ui) -> Option<(usize, Vec<usize>)> {
        let mut selected_path = None;
        ui.scope(|ui| {
            ui.visuals_mut().button_frame = false;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                for (menu_index, menu) in self.menus.iter().enumerate() {
                    ui.menu_button(&menu.title, |ui| {
                        let mut selected = None;
                        show_menu_items(ui, &menu.items, &mut Vec::new(), &mut selected);
                        if let Some(path) = selected {
                            selected_path = Some((menu_index, path));
                        }
                    });
                }
            });
        });
        selected_path
    }
}

/// A button-anchored MenuFlyout.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MenuFlyout {
    label: String,
    items: Vec<MenuItem>,
    icon: Option<FluentIcon>,
    compact: bool,
}

impl MenuFlyout {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            label: label.into(),
            items,
            icon: None,
            compact: false,
        }
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures or reads the `compact` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
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
    pub fn show(self, ui: &mut Ui) -> MenuFlyoutResponse {
        let button = if self.compact {
            ui.add(IconButton::new(self.icon.unwrap_or(FluentIcon::More)))
                .on_hover_text(&self.label)
        } else {
            ui.add(match self.icon {
                Some(icon) => FluentButton::new(&self.label).icon(icon),
                None => FluentButton::new(&self.label),
            })
        };
        let mut selected_path = None;
        let popup = egui::Popup::from_toggle_button_response(&button)
            .width(190.0)
            .gap(4.0)
            .frame(egui::Frame::popup(ui.style()).inner_margin(Margin::same(4)))
            .show(|ui| {
                show_menu_items(ui, &self.items, &mut Vec::new(), &mut selected_path);
            });
        MenuFlyoutResponse {
            response: button,
            selected: selected_path
                .as_ref()
                .and_then(|path| path.first().copied()),
            selected_path,
            open: popup.is_some(),
        }
    }
}

/// A Fluent API value representing `MenuFlyoutResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct MenuFlyoutResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `selected` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub selected: Option<usize>,
    /// The `selected_path` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub selected_path: Option<Vec<usize>>,
    /// The `open` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub open: bool,
}

/// A Fluent command strip corresponding to CommandBar.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CommandBar;

impl CommandBar {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<R>(ui: &mut Ui, add_commands: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let p = current_palette(ui);
        egui::Frame::new()
            .fill(p.control_bg)
            .stroke(Stroke::new(1.0, p.control_border))
            .corner_radius(0)
            .inner_margin(Margin::symmetric(6, 4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    add_commands(ui)
                })
                .inner
            })
    }
}

/// A popup command surface corresponding to CommandBarFlyout.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CommandBarFlyout {
    id: egui::Id,
}

impl CommandBarFlyout {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Self {
            id: egui::Id::new(id_source),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        visible: bool,
        add_commands: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        if !visible {
            return None;
        }
        Some(
            egui::Area::new(self.id)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| CommandBar::show(ui, add_commands).inner)
                .inner,
        )
    }
}

/// A labeled command used in app bars.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AppBarButton {
    label: String,
    icon: FluentIcon,
}

impl AppBarButton {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(label: impl Into<String>, icon: FluentIcon) -> Self {
        Self {
            label: label.into(),
            icon,
        }
    }
}

impl Widget for AppBarButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = current_palette(ui);
        let font = egui::TextStyle::Button.resolve(ui.style());
        let galley = ui
            .painter()
            .layout(self.label.clone(), font, p.text_primary, 72.0);
        let height = (42.0 + galley.size().y).max(64.0);
        let (rect, response) = ui.allocate_exact_size(vec2(80.0, height), Sense::click());
        let fill = if response.is_pointer_button_down_on() {
            p.control_pressed
        } else if response.hovered() {
            p.subtle_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, 0, fill);
        ui.painter().text(
            pos2(rect.center().x, rect.top() + 18.0),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(16.0),
            p.text_primary,
        );
        ui.painter().galley(
            pos2(rect.center().x - galley.size().x * 0.5, rect.top() + 34.0),
            galley,
            p.text_primary,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &self.label)
        });
        response
    }
}

/// A binary command used in app bars.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AppBarToggleButton<'a> {
    value: &'a mut bool,
    label: String,
    icon: FluentIcon,
}

impl<'a> AppBarToggleButton<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut bool, label: impl Into<String>, icon: FluentIcon) -> Self {
        Self {
            value,
            label: label.into(),
            icon,
        }
    }
}

impl Widget for AppBarToggleButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = current_palette(ui);
        let font = egui::TextStyle::Button.resolve(ui.style());
        let galley = ui
            .painter()
            .layout_no_wrap(self.label, font, p.text_primary);
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(galley.size().x + 50.0, 32.0), Sense::click());
        let fill = if *self.value {
            p.selection
        } else if response.hovered() {
            p.subtle_hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, 0, fill);
        ui.painter().text(
            pos2(rect.left() + 18.0, rect.center().y),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(14.0),
            p.text_primary,
        );
        ui.painter().galley(
            pos2(
                rect.left() + 34.0,
                rect.center().y - galley.size().y * 0.5 - 0.5,
            ),
            galley,
            p.text_primary,
        );
        if response.clicked() {
            *self.value = !*self.value;
            response.mark_changed();
        }
        response
    }
}

/// A Fluent API value representing `AppBarSeparator`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AppBarSeparator;

impl Widget for AppBarSeparator {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(vec2(9.0, 28.0), Sense::hover());
        ui.painter().line_segment(
            [
                pos2(rect.center().x, rect.top() + 4.0),
                pos2(rect.center().x, rect.bottom() - 4.0),
            ],
            Stroke::new(1.0, current_palette(ui).control_border),
        );
        response
    }
}

/// A selected-state SplitButton corresponding to ToggleSplitButton.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ToggleSplitButton<'a> {
    selected: &'a mut bool,
    label: String,
}

impl<'a> ToggleSplitButton<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut bool, label: impl Into<String>) -> Self {
        Self {
            selected,
            label: label.into(),
        }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_menu: impl FnOnce(&mut Ui) -> R,
    ) -> super::SplitButtonResponse<R> {
        let prefix = if *self.selected { "✓  " } else { "" };
        let mut result =
            super::SplitButton::new(format!("{prefix}{}", self.label)).show(ui, add_menu);
        if result.response.clicked() {
            *self.selected = !*self.selected;
            result.response.mark_changed();
        }
        result
    }
}

/// Side on which a [`SplitView`] pane is shown.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitViewPanePlacement {
    #[default]
    /// The `Left` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Left,
    /// The `Right` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Right,
}

/// Layout behavior of the pane relative to SplitView content.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitViewDisplayMode {
    /// The `Inline` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Inline,
    /// The `CompactInline` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CompactInline,
    /// The `Overlay` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Overlay,
    #[default]
    /// The `CompactOverlay` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    CompactOverlay,
}

/// A pane/content layout corresponding to SplitView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SplitView {
    pane_width: f32,
    compact_pane_width: f32,
    open: bool,
    placement: SplitViewPanePlacement,
    display_mode: SplitViewDisplayMode,
    pane_fill: Option<Color32>,
    height: Option<f32>,
}

impl SplitView {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(open: bool) -> Self {
        Self {
            pane_width: 220.0,
            compact_pane_width: 48.0,
            open,
            placement: SplitViewPanePlacement::Left,
            display_mode: SplitViewDisplayMode::CompactOverlay,
            pane_fill: None,
            height: None,
        }
    }

    /// Configures or reads the `pane_width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn pane_width(mut self, width: f32) -> Self {
        self.pane_width = width;
        self
    }

    /// Configures or reads the `compact_pane_width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn compact_pane_width(mut self, width: f32) -> Self {
        self.compact_pane_width = width;
        self
    }

    /// Configures or reads the `placement` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn placement(mut self, placement: SplitViewPanePlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Configures or reads the `display_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn display_mode(mut self, display_mode: SplitViewDisplayMode) -> Self {
        self.display_mode = display_mode;
        self
    }

    /// Configures or reads the `pane_fill` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn pane_fill(mut self, fill: Color32) -> Self {
        self.pane_fill = Some(fill);
        self
    }

    /// Configures or reads the `height` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
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
    pub fn show<P, C>(
        self,
        ui: &mut Ui,
        add_pane: impl FnOnce(&mut Ui) -> P,
        add_content: impl FnOnce(&mut Ui) -> C,
    ) -> SplitViewResponse<P, C> {
        let p = current_palette(ui);
        let mut pane_result = None;
        let mut content_result = None;
        let pane_visible_width = if self.open {
            Some(self.pane_width)
        } else if matches!(
            self.display_mode,
            SplitViewDisplayMode::CompactInline | SplitViewDisplayMode::CompactOverlay
        ) {
            Some(self.compact_pane_width)
        } else {
            None
        };
        let mut add_pane = Some(add_pane);
        let mut add_content = Some(add_content);
        let response = ui
            .horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                if let Some(height) = self.height {
                    ui.set_min_height(height);
                }
                let mut show_pane = |ui: &mut Ui| {
                    let Some(width) = pane_visible_width else {
                        return;
                    };
                    egui::Frame::new()
                        .fill(self.pane_fill.unwrap_or(p.control_bg))
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.set_width(width);
                            if let Some(height) = self.height {
                                ui.set_min_height((height - 16.0).max(0.0));
                            }
                            pane_result = Some(add_pane
                                .take()
                                .expect("SplitView pane closure must run once")(
                                ui
                            ));
                        });
                };
                if self.placement == SplitViewPanePlacement::Left {
                    show_pane(ui);
                    if pane_visible_width.is_some() {
                        ui.separator();
                    }
                }
                content_result = Some(add_content
                    .take()
                    .expect("SplitView content closure must run once")(
                    ui
                ));
                if self.placement == SplitViewPanePlacement::Right {
                    if pane_visible_width.is_some() {
                        ui.separator();
                    }
                    show_pane(ui);
                }
            })
            .response;
        SplitViewResponse {
            response,
            pane: pane_result,
            content: content_result.expect("SplitView content closure must run"),
        }
    }
}

/// A Fluent API value representing `SplitViewResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SplitViewResponse<P, C> {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `pane` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub pane: Option<P>,
    /// The `content` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub content: C,
}

#[derive(Clone, Debug)]
/// A Fluent API value representing `NavigationViewItem`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct NavigationViewItem {
    /// The `label` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub label: String,
    /// The `icon` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub icon: FluentIcon,
}

impl NavigationViewItem {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(label: impl Into<String>, icon: FluentIcon) -> Self {
        Self {
            label: label.into(),
            icon,
        }
    }
}

/// A compact navigation pane corresponding to NavigationView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct NavigationView<'a> {
    selected: &'a mut usize,
    items: &'a [NavigationViewItem],
    width: f32,
}

impl<'a> NavigationView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, items: &'a [NavigationViewItem]) -> Self {
        Self {
            selected,
            items,
            width: 220.0,
        }
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
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
    pub fn show(self, ui: &mut Ui) -> Response {
        let mut changed = false;
        let response = ui
            .vertical(|ui| {
                ui.set_width(self.width);
                ui.spacing_mut().item_spacing.y = 2.0;
                for (index, item) in self.items.iter().enumerate() {
                    if navigation_item(ui, item.icon, &item.label, *self.selected == index)
                        .clicked()
                    {
                        *self.selected = index;
                        changed = true;
                    }
                }
            })
            .response;
        let mut response = response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// An in-client-area title bar. Window dragging is handled by the shell adapter.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TitleBar {
    title: String,
    subtitle: Option<String>,
    icon: Option<FluentIcon>,
    back_button_visible: bool,
    pane_toggle_button_visible: bool,
}

impl TitleBar {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            icon: None,
            back_button_visible: false,
            pane_toggle_button_visible: false,
        }
    }

    /// Configures or reads the `subtitle` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Configures or reads the `icon` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn icon(mut self, icon: FluentIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Configures or reads the `back_button_visible` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn back_button_visible(mut self, visible: bool) -> Self {
        self.back_button_visible = visible;
        self
    }

    /// Configures or reads the `pane_toggle_button_visible` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn pane_toggle_button_visible(mut self, visible: bool) -> Self {
        self.pane_toggle_button_visible = visible;
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
    pub fn show<R>(self, ui: &mut Ui, trailing: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let p = current_palette(ui);
        let response = egui::Frame::new()
            .fill(p.nav_bg)
            .inner_margin(Margin::symmetric(12, 6))
            .show(ui, |ui| {
                ui.set_min_height(36.0);
                ui.horizontal(|ui| {
                    if self.pane_toggle_button_visible {
                        ui.add(IconButton::new(FluentIcon::List))
                            .on_hover_text("Toggle pane");
                    }
                    if self.back_button_visible {
                        ui.add(IconButton::new(FluentIcon::Back))
                            .on_hover_text("Back");
                    }
                    if let Some(icon) = self.icon {
                        ui.add(IconButton::new(icon)).on_hover_text(icon.name());
                    }
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(self.title).strong());
                        if let Some(subtitle) = self.subtitle {
                            ui.label(
                                egui::RichText::new(subtitle)
                                    .size(11.0)
                                    .color(p.text_secondary),
                            );
                        }
                    });
                    ui.with_layout(Layout::right_to_left(Align::Center), trailing)
                        .inner
                })
                .inner
            });
        InnerResponse {
            inner: response.inner,
            response: response.response,
        }
    }
}

/// Forces an always-open Fluent popup area at a caller-provided position.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentPopup {
    id: egui::Id,
    position: egui::Pos2,
    width: f32,
}

impl FluentPopup {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, position: egui::Pos2) -> Self {
        Self {
            id: egui::Id::new(id_source),
            position,
            width: 220.0,
        }
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn width(mut self, width: f32) -> Self {
        self.width = width;
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
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        visible: bool,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        if !visible {
            return None;
        }
        Some(
            egui::Area::new(self.id)
                .fixed_pos(self.position)
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style())
                        .inner_margin(Margin::same(8))
                        .show(ui, |ui| {
                            ui.set_width(self.width);
                            add_contents(ui)
                        })
                        .inner
                })
                .inner,
        )
    }
}
