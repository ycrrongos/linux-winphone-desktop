use eframe::egui::{
    self, Align, Align2, Color32, FontId, InnerResponse, Layout, Margin, Response, Sense, Stroke,
    StrokeKind, Ui, Vec2, Widget, pos2, vec2,
};

use super::{FluentIcon, FluentTheme, IconButton, SegmentedControl, palette};

fn current_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// A Fluent API value representing `CollectionItem`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CollectionItem {
    /// The `title` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub title: String,
    /// The `subtitle` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub subtitle: String,
}

impl CollectionItem {
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
            subtitle: String::new(),
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
        self.subtitle = subtitle.into();
        self
    }
}

fn collection_row(ui: &mut Ui, item: &CollectionItem, selected: bool, height: f32) -> Response {
    let p = current_palette(ui);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let fill = if selected {
        p.selection
    } else if response.hovered() {
        p.subtle_hover
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 0, fill);
    if selected {
        let marker = egui::Rect::from_min_size(
            pos2(rect.left() + 2.0, rect.center().y - 8.0),
            vec2(3.0, 16.0),
        );
        ui.painter().rect_filled(marker, 2, p.accent);
    }
    let text_x = rect.left() + 12.0;
    if item.subtitle.is_empty() {
        ui.painter().text(
            pos2(text_x, rect.center().y - 0.5),
            Align2::LEFT_CENTER,
            &item.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
    } else {
        ui.painter().text(
            pos2(text_x, rect.center().y - 9.0),
            Align2::LEFT_CENTER,
            &item.title,
            FontId::proportional(14.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(text_x, rect.center().y + 10.0),
            Align2::LEFT_CENTER,
            &item.subtitle,
            FontId::proportional(12.0),
            p.text_secondary,
        );
    }
    response
}

/// A compact single-selection list corresponding to ListBox.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ListBox<'a> {
    selected: &'a mut usize,
    items: &'a [CollectionItem],
    width: f32,
}

impl<'a> ListBox<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, items: &'a [CollectionItem]) -> Self {
        Self {
            selected,
            items,
            width: 240.0,
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
        let response = egui::Frame::new()
            .stroke(Stroke::new(1.0, current_palette(ui).control_border))
            .corner_radius(0)
            .inner_margin(Margin::same(4))
            .show(ui, |ui| {
                ui.set_width(self.width - 10.0);
                ui.spacing_mut().item_spacing.y = 2.0;
                for (index, item) in self.items.iter().enumerate() {
                    if collection_row(ui, item, *self.selected == index, 32.0).clicked() {
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

/// A richer two-line list corresponding to ListView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ListView<'a> {
    selected: &'a mut usize,
    items: &'a [CollectionItem],
    max_height: f32,
}

impl<'a> ListView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, items: &'a [CollectionItem]) -> Self {
        Self {
            selected,
            items,
            max_height: 260.0,
        }
    }

    /// Configures or reads the `max_height` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
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
            .scope(|ui| {
                egui::ScrollArea::vertical()
                    .max_height(self.max_height)
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 1.0;
                        for (index, item) in self.items.iter().enumerate() {
                            if collection_row(ui, item, *self.selected == index, 48.0).clicked() {
                                *self.selected = index;
                                changed = true;
                            }
                        }
                    });
            })
            .response;
        let mut response = response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// A selectable tile collection corresponding to GridView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridSelectionMode {
    /// The `None` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    None,
    #[default]
    /// The `Single` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Single,
    /// The `Multiple` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Multiple,
    /// The `Extended` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Extended,
}

/// A Fluent API value representing `GridView`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GridView<'a> {
    selected: &'a mut usize,
    items: &'a [CollectionItem],
    tile_size: Vec2,
    selection_mode: GridSelectionMode,
    selected_indices: Option<&'a mut Vec<usize>>,
    reverse: bool,
    spacing: Vec2,
    max_columns: Option<usize>,
}

impl<'a> GridView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, items: &'a [CollectionItem]) -> Self {
        Self {
            selected,
            items,
            tile_size: vec2(132.0, 76.0),
            selection_mode: GridSelectionMode::Single,
            selected_indices: None,
            reverse: false,
            spacing: vec2(6.0, 6.0),
            max_columns: None,
        }
    }

    /// Configures or reads the `tile_size` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn tile_size(mut self, size: Vec2) -> Self {
        self.tile_size = size;
        self
    }

    /// Configures or reads the `selection_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selection_mode(mut self, mode: GridSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Configures or reads the `selected_indices` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn selected_indices(mut self, selected: &'a mut Vec<usize>) -> Self {
        self.selected_indices = Some(selected);
        self
    }

    /// Configures or reads the `reverse` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Configures or reads the `spacing` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = spacing;
        self
    }

    /// Configures or reads the `max_columns` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn max_columns(mut self, columns: usize) -> Self {
        self.max_columns = Some(columns.max(1));
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
    pub fn show(mut self, ui: &mut Ui) -> Response {
        let columns = ((ui.available_width() + self.spacing.x)
            / (self.tile_size.x + self.spacing.x))
            .floor()
            .max(1.0) as usize;
        let columns = self.max_columns.map_or(columns, |max| columns.min(max));
        let p = current_palette(ui);
        let mut changed = false;
        let response = egui::Grid::new(ui.id().with("grid_view"))
            .num_columns(columns)
            .spacing(self.spacing)
            .show(ui, |ui| {
                let order: Box<dyn Iterator<Item = usize>> = if self.reverse {
                    Box::new((0..self.items.len()).rev())
                } else {
                    Box::new(0..self.items.len())
                };
                for (display_index, index) in order.enumerate() {
                    let item = &self.items[index];
                    let (rect, item_response) =
                        ui.allocate_exact_size(self.tile_size, Sense::click());
                    let selected = match self.selection_mode {
                        GridSelectionMode::None => false,
                        GridSelectionMode::Single => *self.selected == index,
                        GridSelectionMode::Multiple | GridSelectionMode::Extended => self
                            .selected_indices
                            .as_deref()
                            .is_some_and(|indices| indices.contains(&index)),
                    };
                    let fill = if selected {
                        p.selection
                    } else if item_response.hovered() {
                        p.subtle_hover
                    } else {
                        p.control_bg
                    };
                    ui.painter().rect(
                        rect,
                        4,
                        fill,
                        Stroke::new(1.0, p.control_border),
                        StrokeKind::Inside,
                    );
                    ui.painter().text(
                        rect.center() - vec2(0.0, 7.0),
                        Align2::CENTER_CENTER,
                        &item.title,
                        FontId::proportional(14.0),
                        p.text_primary,
                    );
                    if !item.subtitle.is_empty() {
                        ui.painter().text(
                            rect.center() + vec2(0.0, 13.0),
                            Align2::CENTER_CENTER,
                            &item.subtitle,
                            FontId::proportional(12.0),
                            p.text_secondary,
                        );
                    }
                    if item_response.clicked() {
                        match self.selection_mode {
                            GridSelectionMode::None => {}
                            GridSelectionMode::Single => {
                                *self.selected = index;
                                changed = true;
                            }
                            GridSelectionMode::Multiple | GridSelectionMode::Extended => {
                                if let Some(indices) = self.selected_indices.as_deref_mut() {
                                    if let Some(position) =
                                        indices.iter().position(|selected| *selected == index)
                                    {
                                        indices.remove(position);
                                    } else {
                                        indices.push(index);
                                    }
                                    *self.selected = index;
                                    changed = true;
                                }
                            }
                        }
                    }
                    if (display_index + 1) % columns == 0 {
                        ui.end_row();
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

/// A generic repeated-item presenter with no retained item widgets.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ItemsRepeater;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Available `ItemsRepeaterLayout` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum ItemsRepeaterLayout {
    #[default]
    /// The `VerticalStack` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    VerticalStack,
    /// The `HorizontalStack` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    HorizontalStack,
    /// The `UniformGrid` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    UniformGrid {
        /// The `columns` value returned to or configured by the caller.
        ///
        /// # Examples
        ///
        /// ```
        /// let theme = fluent_egui::winui::FluentTheme::Light;
        /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
        /// ```
        columns: usize,
    },
}

impl ItemsRepeater {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<T>(
        ui: &mut Ui,
        items: impl IntoIterator<Item = T>,
        mut add_item: impl FnMut(&mut Ui, T),
    ) -> Response {
        ui.vertical(|ui| {
            for item in items {
                add_item(ui, item);
            }
        })
        .response
    }

    /// Creates the `show_with_layout` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show_with_layout<T>(
        ui: &mut Ui,
        items: impl IntoIterator<Item = T>,
        layout: ItemsRepeaterLayout,
        mut add_item: impl FnMut(&mut Ui, T),
    ) -> Response {
        match layout {
            ItemsRepeaterLayout::VerticalStack => Self::show(ui, items, add_item),
            ItemsRepeaterLayout::HorizontalStack => {
                ui.horizontal(|ui| {
                    for item in items {
                        add_item(ui, item);
                    }
                })
                .response
            }
            ItemsRepeaterLayout::UniformGrid { columns } => {
                egui::Grid::new(ui.id().with("items_repeater_uniform_grid"))
                    .num_columns(columns.max(1))
                    .spacing(vec2(6.0, 6.0))
                    .show(ui, |ui| {
                        for (index, item) in items.into_iter().enumerate() {
                            add_item(ui, item);
                            if (index + 1) % columns.max(1) == 0 {
                                ui.end_row();
                            }
                        }
                    })
                    .response
            }
        }
    }
}

/// ItemsView currently shares the WinUI tile presentation with GridView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ItemsView<'a>(GridView<'a>);

impl<'a> ItemsView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, items: &'a [CollectionItem]) -> Self {
        Self(GridView::new(selected, items))
    }

    /// Configures or reads the `tile_size` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn tile_size(mut self, size: Vec2) -> Self {
        self.0 = self.0.tile_size(size);
        self
    }

    /// Configures or reads the `selection_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selection_mode(mut self, mode: GridSelectionMode) -> Self {
        self.0 = self.0.selection_mode(mode);
        self
    }

    /// Configures or reads the `selected_indices` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn selected_indices(mut self, selected: &'a mut Vec<usize>) -> Self {
        self.0 = self.0.selected_indices(selected);
        self
    }

    /// Configures or reads the `reverse` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn reverse(mut self, reverse: bool) -> Self {
        self.0 = self.0.reverse(reverse);
        self
    }

    /// Configures or reads the `spacing` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn spacing(mut self, spacing: Vec2) -> Self {
        self.0 = self.0.spacing(spacing);
        self
    }

    /// Configures or reads the `max_columns` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn max_columns(mut self, columns: usize) -> Self {
        self.0 = self.0.max_columns(columns);
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
        self.0.show(ui)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A Fluent API value representing `TreeNode`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TreeNode {
    /// The `label` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub label: String,
    /// The `children` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// Creates the `leaf` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn leaf(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            children: Vec::new(),
        }
    }

    /// Creates the `branch` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn branch(label: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self {
            label: label.into(),
            children,
        }
    }
}

/// A hierarchical collection corresponding to TreeView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TreeSelectionMode {
    /// The `None` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    None,
    #[default]
    /// The `Single` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Single,
    /// The `Multiple` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Multiple,
}

/// A Fluent API value representing `TreeView`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TreeView<'a> {
    nodes: &'a [TreeNode],
    selection_mode: TreeSelectionMode,
    selected_paths: Option<&'a mut Vec<String>>,
    template_icons: bool,
}

impl<'a> TreeView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(nodes: &'a [TreeNode]) -> Self {
        Self {
            nodes,
            selection_mode: TreeSelectionMode::None,
            selected_paths: None,
            template_icons: false,
        }
    }

    /// Configures or reads the `selection_mode` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn selection_mode(mut self, mode: TreeSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Configures or reads the `selected_paths` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn selected_paths(mut self, selected: &'a mut Vec<String>) -> Self {
        self.selected_paths = Some(selected);
        self
    }

    /// Configures or reads the `template_icons` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn template_icons(mut self, enabled: bool) -> Self {
        self.template_icons = enabled;
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
    pub fn show(mut self, ui: &mut Ui) -> Response {
        fn toggle_path(selected: &mut Vec<String>, path: &str, mode: TreeSelectionMode) -> bool {
            match mode {
                TreeSelectionMode::None => false,
                TreeSelectionMode::Single => {
                    if selected.len() == 1 && selected[0] == path {
                        false
                    } else {
                        selected.clear();
                        selected.push(path.to_owned());
                        true
                    }
                }
                TreeSelectionMode::Multiple => {
                    if let Some(index) = selected.iter().position(|item| item == path) {
                        selected.remove(index);
                    } else {
                        selected.push(path.to_owned());
                    }
                    true
                }
            }
        }

        fn show_node(
            ui: &mut Ui,
            node: &TreeNode,
            path: &str,
            mode: TreeSelectionMode,
            selected: &mut Vec<String>,
            template_icons: bool,
            changed: &mut bool,
        ) {
            let id = ui.id().with(path);
            let icon = if template_icons {
                if node.children.is_empty() {
                    format!("{} ", FluentIcon::OpenFile.glyph())
                } else {
                    format!("{} ", FluentIcon::Folder.glyph())
                }
            } else {
                String::new()
            };
            let label = format!("{icon}{}", node.label);
            if node.children.is_empty() {
                let is_selected = selected.iter().any(|item| item == path);
                let response = if mode == TreeSelectionMode::None {
                    ui.label(label)
                } else {
                    ui.selectable_label(is_selected, label)
                };
                if response.clicked() && toggle_path(selected, path, mode) {
                    *changed = true;
                }
            } else {
                let result = egui::CollapsingHeader::new(label)
                    .id_salt(id)
                    .default_open(true)
                    .show(ui, |ui| {
                        for (index, child) in node.children.iter().enumerate() {
                            show_node(
                                ui,
                                child,
                                &format!("{path}/{index}"),
                                mode,
                                selected,
                                template_icons,
                                changed,
                            );
                        }
                    });
                if result.header_response.clicked() && toggle_path(selected, path, mode) {
                    *changed = true;
                }
            }
        }
        let mut scratch = Vec::new();
        let selected = self.selected_paths.as_deref_mut().unwrap_or(&mut scratch);
        let mut changed = false;
        let mut response = ui
            .vertical(|ui| {
                for (index, node) in self.nodes.iter().enumerate() {
                    show_node(
                        ui,
                        node,
                        &index.to_string(),
                        self.selection_mode,
                        selected,
                        self.template_icons,
                        &mut changed,
                    );
                }
            })
            .response;
        if changed {
            response.mark_changed();
        }
        response
    }
}

/// A previous/current/next content presenter corresponding to FlipView.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FlipView<'a> {
    selected: &'a mut usize,
    count: usize,
    size: Vec2,
}

impl<'a> FlipView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(selected: &'a mut usize, count: usize) -> Self {
        Self {
            selected,
            count: count.max(1),
            size: vec2(360.0, 180.0),
        }
    }

    /// Configures or reads the `size` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn size(mut self, size: Vec2) -> Self {
        self.size = size;
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
        ui: &mut Ui,
        add_page: impl FnOnce(&mut Ui, usize) -> R,
    ) -> InnerResponse<R> {
        *self.selected = (*self.selected).min(self.count - 1);
        let p = current_palette(ui);
        let response = egui::Frame::new()
            .fill(p.control_bg)
            .stroke(Stroke::new(1.0, p.control_border))
            .corner_radius(0)
            .show(ui, |ui| {
                ui.set_min_size(self.size);
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if ui
                            .add_enabled(*self.selected > 0, IconButton::new(FluentIcon::Back))
                            .clicked()
                        {
                            *self.selected -= 1;
                        }
                        let result = ui
                            .allocate_ui_with_layout(
                                vec2(self.size.x - 84.0, self.size.y - 8.0),
                                Layout::top_down(Align::Center),
                                |ui| add_page(ui, *self.selected),
                            )
                            .inner;
                        if ui
                            .add_enabled(
                                *self.selected + 1 < self.count,
                                IconButton::new(FluentIcon::ChevronRight),
                            )
                            .clicked()
                        {
                            *self.selected += 1;
                        }
                        result
                    })
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

/// A Fluent-configured ScrollViewer/ScrollView wrapper.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ScrollViewer {
    max_height: f32,
    horizontal: bool,
}

impl ScrollViewer {
    /// Creates the `vertical` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn vertical(max_height: f32) -> Self {
        Self {
            max_height,
            horizontal: false,
        }
    }

    /// Creates the `horizontal` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn horizontal(max_height: f32) -> Self {
        Self {
            max_height,
            horizontal: true,
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        if self.horizontal {
            egui::ScrollArea::horizontal()
                .max_height(self.max_height)
                .show(ui, add_contents)
                .inner
        } else {
            egui::ScrollArea::vertical()
                .max_height(self.max_height)
                .show(ui, add_contents)
                .inner
        }
    }
}

/// Public alias for the `ScrollView` API.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub type ScrollView = ScrollViewer;

/// A semantic label positioned along an [`AnnotatedScrollBar`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedScrollBarLabel {
    /// The `text` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub text: String,
    /// The `offset` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub offset: f32,
}

impl AnnotatedScrollBarLabel {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>, offset: f32) -> Self {
        Self {
            text: text.into(),
            offset: offset.clamp(0.0, 1.0),
        }
    }
}

/// A scrollbar rail with semantic annotation markers and optional detail labels.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AnnotatedScrollBar<'a> {
    value: &'a mut f32,
    markers: &'a [f32],
    labels: Option<&'a [AnnotatedScrollBarLabel]>,
    detail_label: Option<&'a dyn Fn(f32) -> String>,
    height: f32,
}

impl<'a> AnnotatedScrollBar<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(value: &'a mut f32, markers: &'a [f32]) -> Self {
        Self {
            value,
            markers,
            labels: None,
            detail_label: None,
            height: 180.0,
        }
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
        self.height = height;
        self
    }

    /// Configures or reads the `labels` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn labels(mut self, labels: &'a [AnnotatedScrollBarLabel]) -> Self {
        self.labels = Some(labels);
        self
    }

    /// Configures or reads the `detail_label` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn detail_label(mut self, labeler: &'a dyn Fn(f32) -> String) -> Self {
        self.detail_label = Some(labeler);
        self
    }
}

impl Widget for AnnotatedScrollBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let p = current_palette(ui);
        let has_labels = self.labels.is_some_and(|labels| !labels.is_empty());
        let width = if has_labels { 90.0 } else { 24.0 };
        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, self.height), Sense::click_and_drag());
        let rail_x = rect.right() - 12.0;
        let track = egui::Rect::from_min_max(
            pos2(rail_x - 8.0, rect.top() + 16.0),
            pos2(rail_x + 8.0, rect.bottom() - 16.0),
        );
        if let Some(pointer) = response.interact_pointer_pos() {
            *self.value = ((pointer.y - track.top()) / track.height()).clamp(0.0, 1.0);
            response.mark_changed();
        }
        ui.painter().rect_filled(
            egui::Rect::from_center_size(pos2(rail_x, track.center().y), vec2(2.0, track.height())),
            2,
            p.progress_track,
        );
        ui.painter().text(
            pos2(rail_x, rect.top() + 7.0),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronUp.glyph(),
            super::icon_font_id(10.0),
            p.text_primary,
        );
        ui.painter().text(
            pos2(rail_x, rect.bottom() - 7.0),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronDown.glyph(),
            super::icon_font_id(10.0),
            p.text_primary,
        );
        for marker in self.markers {
            let y = egui::lerp(track.y_range(), marker.clamp(0.0, 1.0));
            ui.painter().line_segment(
                [pos2(rail_x - 6.0, y), pos2(rail_x + 6.0, y)],
                Stroke::new(1.5, p.text_secondary),
            );
        }
        if let Some(labels) = self.labels {
            let mut last_label_y = f32::NEG_INFINITY;
            for label in labels {
                let y = egui::lerp(track.y_range(), label.offset.clamp(0.0, 1.0));
                if y - last_label_y >= 12.0 {
                    ui.painter().text(
                        pos2(rail_x - 12.0, y),
                        Align2::RIGHT_CENTER,
                        &label.text,
                        FontId::proportional(12.0),
                        p.text_primary,
                    );
                    last_label_y = y;
                }
            }
        }
        let y = egui::lerp(track.y_range(), self.value.clamp(0.0, 1.0));
        ui.painter().rect_filled(
            egui::Rect::from_center_size(pos2(rail_x, y), vec2(6.0, 24.0)),
            3,
            p.accent,
        );
        if let (Some(pointer), Some(labeler)) = (response.hover_pos(), self.detail_label) {
            let offset = ((pointer.y - track.top()) / track.height()).clamp(0.0, 1.0);
            response = response.on_hover_text_at_pointer(labeler(offset));
        }
        response
    }
}

/// A deterministic pull-to-refresh analogue with an explicit refresh command.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PullToRefresh;

impl PullToRefresh {
    /// Creates the `show` configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<R>(
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> PullToRefreshResponse<R> {
        let mut refresh_requested = false;
        let inner = ui
            .vertical(|ui| {
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("Refresh")
                                .font(egui::TextStyle::Button.resolve(ui.style())),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    refresh_requested = true;
                }
                add_contents(ui)
            })
            .inner;
        PullToRefreshResponse {
            inner,
            refresh_requested,
        }
    }
}

/// A Fluent API value representing `PullToRefreshResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct PullToRefreshResponse<R> {
    /// The `inner` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub inner: R,
    /// The `refresh_requested` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub refresh_requested: bool,
}

/// A list/detail scale switch corresponding to SemanticZoom.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SemanticZoom<'a> {
    zoomed_out: &'a mut bool,
}

impl<'a> SemanticZoom<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(zoomed_out: &'a mut bool) -> Self {
        Self { zoomed_out }
    }
}

impl Widget for SemanticZoom<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut selected = usize::from(*self.zoomed_out);
        let mut response = ui.add(SegmentedControl::new(
            &mut selected,
            ["Detailed", "Grouped"],
        ));
        let next = selected == 1;
        if next != *self.zoomed_out {
            *self.zoomed_out = next;
            response.mark_changed();
        }
        response
    }
}

/// A horizontal selector corresponding to SelectorBar.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SelectorBar<'a> {
    selected: &'a mut usize,
    labels: Vec<String>,
    width: f32,
}

impl<'a> SelectorBar<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        selected: &'a mut usize,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            selected,
            labels: labels.into_iter().map(Into::into).collect(),
            width: 320.0,
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
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Widget for SelectorBar<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let labels = self.labels.iter().map(String::as_str).collect::<Vec<_>>();
        ui.add(SegmentedControl::new(self.selected, labels).width(self.width))
    }
}

/// A Pivot tab header. Content remains owned by the caller.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct Pivot<'a>(SelectorBar<'a>);

impl<'a> Pivot<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(
        selected: &'a mut usize,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self(SelectorBar::new(selected, labels))
    }

    /// Configures or reads the `width` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn width(mut self, width: f32) -> Self {
        self.0 = self.0.width(width);
        self
    }
}

impl Widget for Pivot<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.0.ui(ui)
    }
}

/// A row with a revealed trailing action, corresponding to SwipeControl.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SwipeControl {
    text: String,
    action_text: String,
    reveal: bool,
}

impl SwipeControl {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(text: impl Into<String>, action_text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            action_text: action_text.into(),
            reveal: false,
        }
    }

    /// Configures or reads the `reveal` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn reveal(mut self, reveal: bool) -> Self {
        self.reveal = reveal;
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
    pub fn show(self, ui: &mut Ui) -> SwipeControlResponse {
        let p = current_palette(ui);
        let mut action_clicked = false;
        let mut reveal_requested = false;
        let response = ui
            .horizontal(|ui| {
                let (rect, row) =
                    ui.allocate_exact_size(vec2(ui.available_width() - 96.0, 44.0), Sense::drag());
                ui.painter().rect_filled(rect, 0, p.control_bg);
                ui.painter().text(
                    pos2(rect.left() + 12.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    self.text,
                    FontId::proportional(14.0),
                    p.text_primary,
                );
                if row.dragged() || row.clicked() {
                    reveal_requested = true;
                }
                if self.reveal
                    && ui
                        .add_sized(
                            [88.0, 44.0],
                            egui::Button::new(self.action_text).fill(p.danger),
                        )
                        .clicked()
                {
                    action_clicked = true;
                }
            })
            .response;
        SwipeControlResponse {
            response,
            reveal_requested,
            action_clicked,
        }
    }
}

/// A Fluent API value representing `SwipeControlResponse`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct SwipeControlResponse {
    /// The `response` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub response: Response,
    /// The `reveal_requested` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub reveal_requested: bool,
    /// The `action_clicked` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub action_clicked: bool,
}
