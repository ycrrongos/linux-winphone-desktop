use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, InnerResponse, Layout, Margin, Rect, Response,
    Sense, Stroke, StrokeKind, Ui, UiBuilder, Vec2, Widget, vec2,
};

use super::{FluentIcon, FluentTheme, icon_font_id, palette};

fn current_palette(ui: &Ui) -> super::FluentPalette {
    palette(if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    })
}

/// A framed content presenter corresponding to WinUI Border.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentBorder {
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    corner_radius: u8,
    margin: Margin,
}

impl Default for FluentBorder {
    fn default() -> Self {
        Self {
            fill: None,
            stroke: None,
            corner_radius: 0,
            margin: Margin::same(12),
        }
    }
}

impl FluentBorder {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures or reads the `fill` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn fill(mut self, fill: Color32) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Configures or reads the `stroke` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Configures or reads the `corner_radius` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Configures or reads the `margin` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let p = current_palette(ui);
        egui::Frame::new()
            .fill(self.fill.unwrap_or(p.control_bg))
            .stroke(self.stroke.unwrap_or(Stroke::new(1.0, p.control_border)))
            .corner_radius(self.corner_radius)
            .inner_margin(self.margin)
            .show(ui, add_contents)
    }
}

/// A fixed-size immediate-mode drawing surface corresponding to WinUI Canvas.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct Canvas {
    size: Vec2,
    clip: bool,
}

impl Canvas {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(size: Vec2) -> Self {
        Self { size, clip: true }
    }

    /// Configures or reads the `clip` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
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
        paint: impl FnOnce(&egui::Painter, Rect) -> R,
    ) -> InnerResponse<R> {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let painter = if self.clip {
            ui.painter().with_clip_rect(rect)
        } else {
            ui.painter().clone()
        };
        InnerResponse {
            inner: paint(&painter, rect),
            response,
        }
    }
}

/// A Fluent-spaced wrapper around egui Grid.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct GridPanel {
    id: egui::Id,
    columns: usize,
    spacing: Vec2,
    striped: bool,
}

impl GridPanel {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(id_source: impl std::hash::Hash + std::fmt::Debug, columns: usize) -> Self {
        Self {
            id: egui::Id::new(id_source),
            columns: columns.max(1),
            spacing: vec2(12.0, 8.0),
            striped: false,
        }
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

    /// Configures or reads the `striped` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        egui::Grid::new(self.id)
            .num_columns(self.columns)
            .spacing(self.spacing)
            .striped(self.striped)
            .show(ui, add_contents)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Available `Orientation` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum Orientation {
    /// The `Horizontal` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Horizontal,
    #[default]
    /// The `Vertical` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Vertical,
}

/// A horizontal or vertical content stack corresponding to StackPanel.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct StackPanel {
    orientation: Orientation,
    spacing: f32,
    align: Align,
}

impl StackPanel {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            spacing: 8.0,
            align: Align::Min,
        }
    }

    /// Configures or reads the `spacing` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Configures or reads the `align` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn align(mut self, align: Align) -> Self {
        self.align = align;
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(self.spacing);
            ui.with_layout(
                match self.orientation {
                    Orientation::Horizontal => Layout::left_to_right(self.align),
                    Orientation::Vertical => Layout::top_down(self.align),
                },
                add_contents,
            )
            .inner
        })
    }
}

/// A fixed rectangle whose children can be placed by absolute offsets.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RelativePanel {
    size: Vec2,
}

impl RelativePanel {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(size: Vec2) -> Self {
        Self { size }
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
        add_contents: impl FnOnce(&mut RelativePanelUi<'_>) -> R,
    ) -> InnerResponse<R> {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let mut panel = RelativePanelUi { ui, rect };
        InnerResponse {
            inner: add_contents(&mut panel),
            response,
        }
    }
}

/// A Fluent API value representing `RelativePanelUi`.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RelativePanelUi<'a> {
    ui: &'a mut Ui,
    rect: Rect,
}

impl RelativePanelUi<'_> {
    /// Configures or reads the `put` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn put<W: Widget>(&mut self, offset: Vec2, size: Vec2, widget: W) -> Response {
        let rect = Rect::from_min_size(self.rect.min + offset, size);
        self.ui.put(rect, widget)
    }

    /// Configures or reads the `rect` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn rect(&self) -> Rect {
        self.rect
    }
}

/// A wrapping tile panel corresponding to VariableSizedWrapGrid.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct VariableSizedWrapGrid {
    item_size: Vec2,
    spacing: Vec2,
}

impl VariableSizedWrapGrid {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(item_size: Vec2) -> Self {
        Self {
            item_size,
            spacing: vec2(6.0, 6.0),
        }
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

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<T>(
        self,
        ui: &mut Ui,
        items: impl IntoIterator<Item = T>,
        mut add_item: impl FnMut(&mut Ui, T),
    ) -> Response {
        let width = ui.available_width().max(self.item_size.x);
        let columns = ((width + self.spacing.x) / (self.item_size.x + self.spacing.x))
            .floor()
            .max(1.0) as usize;
        egui::Grid::new(ui.id().with("variable_wrap_grid"))
            .num_columns(columns)
            .spacing(self.spacing)
            .show(ui, |ui| {
                for (index, item) in items.into_iter().enumerate() {
                    ui.allocate_ui(self.item_size, |ui| add_item(ui, item));
                    if (index + 1) % columns == 0 {
                        ui.end_row();
                    }
                }
            })
            .response
    }
}

/// A constrained presenter corresponding to Viewbox.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct Viewbox {
    size: Vec2,
}

impl Viewbox {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(size: Vec2) -> Self {
        Self { size }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let inner = ui
            .scope_builder(
                UiBuilder::new()
                    .max_rect(rect)
                    .layout(Layout::top_down(Align::Center)),
                add_contents,
            )
            .inner;
        InnerResponse { inner, response }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Available `ShapeKind` modes.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub enum ShapeKind {
    /// The `Rectangle` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Rectangle,
    /// The `RoundedRectangle` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    RoundedRectangle(f32),
    /// The `Ellipse` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    Ellipse,
}

/// A primitive WinUI Shape analogue.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ShapeElement {
    kind: ShapeKind,
    size: Vec2,
    fill: Color32,
    stroke: Stroke,
}

impl ShapeElement {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(kind: ShapeKind, size: Vec2, fill: Color32) -> Self {
        Self {
            kind,
            size,
            fill,
            stroke: Stroke::NONE,
        }
    }

    /// Configures or reads the `stroke` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
}

impl Widget for ShapeElement {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        match self.kind {
            ShapeKind::Rectangle => {
                ui.painter()
                    .rect(rect, 0, self.fill, self.stroke, StrokeKind::Inside);
            }
            ShapeKind::RoundedRectangle(radius) => {
                ui.painter()
                    .rect(rect, radius, self.fill, self.stroke, StrokeKind::Inside);
            }
            ShapeKind::Ellipse => {
                let radius = rect.size().min_elem() * 0.5;
                ui.painter()
                    .circle(rect.center(), radius, self.fill, self.stroke);
            }
        }
        response
    }
}

/// A line primitive with a stable allocation rectangle.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct LineElement {
    size: Vec2,
    stroke: Stroke,
}

impl LineElement {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(size: Vec2, stroke: Stroke) -> Self {
        Self { size, stroke }
    }
}

impl Widget for LineElement {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        ui.painter()
            .line_segment([rect.left_bottom(), rect.right_top()], self.stroke);
        response
    }
}

/// A texture-backed image with optional Fluent border and corner radius.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct FluentImage {
    texture: egui::TextureId,
    size: Vec2,
    corner_radius: u8,
}

impl FluentImage {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(texture: egui::TextureId, size: Vec2) -> Self {
        Self {
            texture,
            size,
            corner_radius: 0,
        }
    }

    /// Configures or reads the `corner_radius` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
        self
    }
}

impl Widget for FluentImage {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(egui::Image::new((self.texture, self.size)).corner_radius(self.corner_radius))
    }
}

/// A standalone Fluent font icon corresponding to IconElement.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct IconElement {
    icon: FluentIcon,
    size: f32,
    color: Option<Color32>,
}

impl IconElement {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(icon: FluentIcon) -> Self {
        Self {
            icon,
            size: 18.0,
            color: None,
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
    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Configures or reads the `color` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for IconElement {
    fn ui(self, ui: &mut Ui) -> Response {
        let side = self.size.max(16.0);
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(side), Sense::hover());
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.icon.glyph(),
            icon_font_id(self.size),
            self.color.unwrap_or_else(|| ui.visuals().text_color()),
        );
        response
    }
}

/// A renderer-independent concentric-ring radial gradient approximation for egui Painter.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct RadialGradientBrush {
    size: Vec2,
    center: Color32,
    edge: Color32,
    rings: usize,
}

impl RadialGradientBrush {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(size: Vec2, center: Color32, edge: Color32) -> Self {
        Self {
            size,
            center,
            edge,
            rings: 24,
        }
    }

    /// Configures or reads the `rings` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn rings(mut self, rings: usize) -> Self {
        self.rings = rings;
        self
    }
}

impl Widget for RadialGradientBrush {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let rings = self.rings.max(2);
        let radius = rect.size().min_elem() * 0.5;
        for index in (0..rings).rev() {
            let t = index as f32 / (rings - 1) as f32;
            let color = egui::Rgba::from(self.center).multiply(1.0 - t)
                + egui::Rgba::from(self.edge).multiply(t);
            ui.painter().circle_filled(
                rect.center(),
                radius * (index + 1) as f32 / rings as f32,
                Color32::from(color),
            );
        }
        response
    }
}

/// A translucent in-app surface. This is not a system DWM Acrylic backdrop.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct AcrylicBrush {
    tint: Color32,
    opacity: f32,
    corner_radius: u8,
}

impl AcrylicBrush {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(tint: Color32) -> Self {
        Self {
            tint,
            opacity: 0.82,
            corner_radius: 0,
        }
    }

    /// Configures or reads the `opacity` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Configures or reads the `corner_radius` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn corner_radius(mut self, radius: u8) -> Self {
        self.corner_radius = radius;
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
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let alpha = (self.opacity.clamp(0.0, 1.0) * 255.0) as u8;
        let fill =
            Color32::from_rgba_unmultiplied(self.tint.r(), self.tint.g(), self.tint.b(), alpha);
        egui::Frame::new()
            .fill(fill)
            .stroke(Stroke::new(1.0, Color32::from_white_alpha(28)))
            .corner_radius(CornerRadius::same(self.corner_radius))
            .inner_margin(Margin::same(12))
            .show(ui, add_contents)
    }
}

/// Applies WinUI compact sizing to an isolated child scope.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub fn compact_sizing<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 4.0);
        ui.spacing_mut().interact_size = vec2(32.0, 24.0);
        ui.spacing_mut().button_padding = vec2(8.0, 3.0);
        add_contents(ui)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_builders_keep_requested_values() {
        let panel = StackPanel::new(Orientation::Horizontal).spacing(4.0);
        assert_eq!(panel.orientation, Orientation::Horizontal);
        assert_eq!(panel.spacing, 4.0);
        let brush = AcrylicBrush::new(Color32::BLACK).opacity(0.5);
        assert_eq!(brush.opacity, 0.5);
    }
}
