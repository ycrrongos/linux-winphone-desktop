use eframe::egui::{
    self, Align, Color32, CornerRadius, Layout, Margin, RichText, Sense, Stroke, StrokeKind, Ui,
    Vec2, pos2, vec2,
};

use super::{FluentPalette, FluentTheme, palette};

const SECTIONS: [&str; 6] = [
    "Text",
    "Fill",
    "Stroke",
    "Background",
    "Signal",
    "High Contrast",
];

#[derive(Clone, Copy)]
struct BrushTile {
    name: &'static str,
    usage: &'static str,
    resource: &'static str,
    color: Color32,
    foreground: Color32,
}

/// Complete WinUI Gallery-style color guidance surface.
///
/// The selected section is owned by the application so it remains stable across frames.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct ColorGuidance<'a> {
    selected: &'a mut usize,
    theme: FluentTheme,
}

impl<'a> ColorGuidance<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn new(selected: &'a mut usize, theme: FluentTheme) -> Self {
        Self { selected, theme }
    }

    /// Renders the control in the supplied egui UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn show(self, ui: &mut Ui) {
        let p = palette(self.theme);
        ui.label("The brushes below are part of WinUI 3 and can be referenced by widgets in an application. For example:");
        ui.add_space(8.0);
        code_sample(ui, p);
        ui.add_space(4.0);
        selector(ui, self.selected, p);
        ui.add_space(18.0);

        let groups = brush_groups(p, self.theme);
        for group in &groups[*self.selected % groups.len()] {
            brush_group(ui, group.0, group.1, &group.2, p);
            ui.add_space(12.0);
        }
    }
}

fn code_sample(ui: &mut Ui, p: FluentPalette) {
    egui::Frame::new()
        .fill(p.control_bg)
        .stroke(Stroke::new(1.0, p.control_border))
        .corner_radius(0)
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.label(
                RichText::new(
                    r#"<TextBlock Text="..." Foreground="{ThemeResource TextFillColorPrimaryBrush}" />"#,
                )
                .monospace()
                .size(12.0),
            );
        });
}

fn selector(ui: &mut Ui, selected: &mut usize, p: FluentPalette) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for (index, label) in SECTIONS.iter().enumerate() {
            let text_width = ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(
                        (*label).into(),
                        egui::TextStyle::Button.resolve(ui.style()),
                        p.text_primary,
                    )
                    .size()
                    .x
            });
            let (rect, response) =
                ui.allocate_exact_size(vec2(text_width + 24.0, 38.0), Sense::click());
            let hovered = response.hovered();
            if hovered {
                ui.painter()
                    .rect_filled(rect.shrink2(vec2(2.0, 4.0)), 4, p.subtle_hover);
            }
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                *label,
                egui::TextStyle::Button.resolve(ui.style()),
                p.text_primary,
            );
            if index == *selected {
                let underline = egui::Rect::from_center_size(
                    pos2(rect.center().x, rect.bottom() - 2.0),
                    vec2(18.0, 3.0),
                );
                ui.painter().rect_filled(underline, 2, p.accent);
            }
            if response.clicked() {
                *selected = index;
            }
        }
    });
}

fn brush_group(ui: &mut Ui, title: &str, subtitle: &str, tiles: &[BrushTile], p: FluentPalette) {
    egui::Frame::new()
        .fill(p.card_bg)
        .stroke(Stroke::new(1.0, p.card_border))
        .corner_radius(0)
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), 138.0),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        ui.add_space(14.0);
                        ui.vertical(|ui| {
                            ui.label(RichText::new(title).size(18.0).strong());
                            ui.label(RichText::new(subtitle).size(12.0).color(p.text_secondary));
                        });
                    });
                    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new("Amino acid").size(38.0));
                    });
                },
            );
            for row in tiles.chunks(4) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    let width = ui.available_width() / row.len().max(1) as f32;
                    for tile in row {
                        let (rect, response) =
                            ui.allocate_exact_size(vec2(width, 126.0), Sense::click());
                        ui.painter().rect(
                            rect,
                            CornerRadius::ZERO,
                            tile.color,
                            Stroke::NONE,
                            StrokeKind::Inside,
                        );
                        let clip = rect.shrink(12.0);
                        let painter = ui.painter().with_clip_rect(clip);
                        painter.text(
                            pos2(rect.left() + 12.0, rect.top() + 14.0),
                            egui::Align2::LEFT_TOP,
                            tile.name,
                            egui::FontId::proportional(13.0),
                            tile.foreground,
                        );
                        painter.text(
                            pos2(rect.left() + 12.0, rect.top() + 36.0),
                            egui::Align2::LEFT_TOP,
                            tile.usage,
                            egui::FontId::proportional(11.0),
                            tile.foreground,
                        );
                        painter.text(
                            pos2(rect.left() + 12.0, rect.bottom() - 16.0),
                            egui::Align2::LEFT_BOTTOM,
                            tile.resource,
                            egui::FontId::proportional(11.0),
                            tile.foreground,
                        );
                        painter.text(
                            pos2(rect.right() - 13.0, rect.top() + 14.0),
                            egui::Align2::RIGHT_TOP,
                            "Copy",
                            egui::FontId::proportional(10.0),
                            tile.foreground,
                        );
                        if response.hovered() {
                            ui.painter().rect_stroke(
                                rect.shrink(1.0),
                                CornerRadius::ZERO,
                                Stroke::new(1.0, p.accent),
                                StrokeKind::Inside,
                            );
                        }
                        if response.clicked() {
                            ui.ctx().copy_text(tile.resource.to_owned());
                        }
                    }
                });
            }
        });
}

type BrushGroup = (&'static str, &'static str, Vec<BrushTile>);

#[derive(Clone, Copy)]
struct TileSpec {
    name: &'static str,
    usage: &'static str,
    resource: &'static str,
}

#[derive(Clone, Copy)]
struct GroupSpec {
    title: &'static str,
    subtitle: &'static str,
    tiles: &'static [TileSpec],
}

const fn tile(name: &'static str, usage: &'static str, resource: &'static str) -> TileSpec {
    TileSpec {
        name,
        usage,
        resource,
    }
}

const fn group(
    title: &'static str,
    subtitle: &'static str,
    tiles: &'static [TileSpec],
) -> GroupSpec {
    GroupSpec {
        title,
        subtitle,
        tiles,
    }
}

const TEXT_GROUPS: &[GroupSpec] = &[
    group(
        "Text",
        "For UI labels and static text",
        &[
            tile(
                "Text / Primary",
                "Rest or Hover",
                "TextFillColorPrimaryBrush",
            ),
            tile(
                "Text / Secondary",
                "Rest or Hover",
                "TextFillColorSecondaryBrush",
            ),
            tile(
                "Text / Tertiary",
                "Pressed only (not accessible)",
                "TextFillColorTertiaryBrush",
            ),
            tile(
                "Text / Disabled",
                "Disabled only (not accessible)",
                "TextFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Accent Text",
        "Recommended for links",
        &[
            tile(
                "Accent Text / Primary",
                "Rest or Hover",
                "AccentTextFillColorPrimaryBrush",
            ),
            tile(
                "Accent Text / Secondary",
                "Rest or Hover",
                "AccentTextFillColorSecondaryBrush",
            ),
            tile(
                "Accent Text / Tertiary",
                "Pressed only (not accessible)",
                "AccentTextFillColorTertiaryBrush",
            ),
            tile(
                "Accent Text / Disabled",
                "Disabled only (not accessible)",
                "AccentTextFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Text On Accent",
        "Used for text on accent colored controls or fills",
        &[
            tile(
                "Text on Accent / Primary",
                "Rest or Hover",
                "TextOnAccentFillColorPrimaryBrush",
            ),
            tile(
                "Text on Accent / Secondary",
                "Pressed only (not accessible)",
                "TextOnAccentFillColorSecondaryBrush",
            ),
            tile(
                "Text on Accent / Disabled",
                "Disabled only (not accessible)",
                "TextOnAccentFillColorDisabledBrush",
            ),
            tile(
                "Text on Accent / Selected Text",
                "For highlighted text in text entry experiences",
                "TextOnAccentFillColorSelectedTextBrush",
            ),
        ],
    ),
];

const FILL_GROUPS: &[GroupSpec] = &[
    group(
        "Control Fill",
        "Fill used for standard controls",
        &[
            tile("Control / Default", "Rest", "ControlFillColorDefaultBrush"),
            tile(
                "Control / Secondary",
                "Hover",
                "ControlFillColorSecondaryBrush",
            ),
            tile(
                "Control / Tertiary",
                "Pressed",
                "ControlFillColorTertiaryBrush",
            ),
            tile(
                "Control / Quartenary",
                "Rest (Pill Button control)",
                "ControlFillColorQuarternaryBrush",
            ),
            tile(
                "Control / Disabled",
                "Disabled",
                "ControlFillColorDisabledBrush",
            ),
            tile(
                "Control / Transparent",
                "Rest",
                "ControlFillColorTransparentBrush",
            ),
            tile(
                "Control / Input Active",
                "Active/focused text input fields",
                "ControlFillColorInputActiveBrush",
            ),
        ],
    ),
    group(
        "Control Alt Fill",
        "Fill used for the off states of toggle controls",
        &[
            tile(
                "Control Alt / Transparent",
                "",
                "ControlAltFillColorTransparentBrush",
            ),
            tile(
                "Control Alt / Secondary",
                "Rest",
                "ControlAltFillColorSecondaryBrush",
            ),
            tile(
                "Control Alt / Tertiary",
                "Hover",
                "ControlAltFillColorTertiaryBrush",
            ),
            tile(
                "Control Alt / Quarternary",
                "Pressed",
                "ControlAltFillColorQuarternaryBrush",
            ),
            tile(
                "Control Alt / Disabled",
                "Disabled",
                "ControlAltFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Neutral Solid",
        "Fills used for slider thumbs to cover the track beneath",
        &[tile(
            "Control Solid / Default",
            "Rest",
            "ControlSolidFillColorDefaultBrush",
        )],
    ),
    group(
        "Neutral Strong",
        "Controls that must meet 3:1 contrast requirements",
        &[
            tile(
                "Control Strong / Default",
                "Rest or hover",
                "ControlStrongFillColorDefaultBrush",
            ),
            tile(
                "Control Strong / Disabled",
                "Disabled only (not accessible)",
                "ControlStrongFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Subtle Fill",
        "List items transparent at rest and visible upon interaction",
        &[
            tile(
                "Subtle / Transparent",
                "Rest",
                "SubtleFillColorTransparentBrush",
            ),
            tile(
                "Subtle / Secondary",
                "Hover",
                "SubtleFillColorSecondaryBrush",
            ),
            tile(
                "Subtle / Tertiary",
                "Pressed",
                "SubtleFillColorTertiaryBrush",
            ),
            tile(
                "Subtle / Disabled",
                "Disabled only (not accessible)",
                "SubtleFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Control On Image Fill",
        "Controls living on top of imagery",
        &[
            tile(
                "Control On Image Fill Default",
                "Rest",
                "ControlOnImageFillColorDefaultBrush",
            ),
            tile(
                "Control On Image Fill Secondary",
                "Hover",
                "ControlOnImageFillColorSecondaryBrush",
            ),
            tile(
                "Control On Image Fill Tertiary",
                "Pressed",
                "ControlOnImageFillColorTertiaryBrush",
            ),
            tile(
                "Control On Image Fill Disabled",
                "Disabled only (not accessible)",
                "ControlOnImageFillColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Accent Fill",
        "Accent fills on controls",
        &[
            tile("Accent / Default", "Rest", "AccentFillColorDefaultBrush"),
            tile(
                "Accent / Secondary",
                "Hover",
                "AccentFillColorSecondaryBrush",
            ),
            tile(
                "Accent / Tertiary",
                "Pressed",
                "AccentFillColorTertiaryBrush",
            ),
            tile(
                "Accent / Disabled",
                "Disabled",
                "AccentFillColorDisabledBrush",
            ),
            tile(
                "Accent / Selected Text Background",
                "Highlighted or selected text background",
                "AccentFillColorSelectedTextBackgroundBrush",
            ),
        ],
    ),
];

const STROKE_GROUPS: &[GroupSpec] = &[
    group(
        "Card Stroke",
        "Used for card and layer colors",
        &[
            tile(
                "Card Stroke / Default",
                "Card layer and strokes",
                "CardStrokeColorDefaultBrush",
            ),
            tile(
                "Card Stroke / Default Solid",
                "Solid equivalent used in expanded command bars",
                "CardStrokeColorDefaultSolidBrush",
            ),
        ],
    ),
    group(
        "Control Elevation (gradient strokes)",
        "Standard control strokes and stroke states",
        &[
            tile("Control / Border", "Rest", "ControlElevationBorderBrush"),
            tile("Circle / Border", "Rest", "CircleElevationBorderBrush"),
            tile(
                "Text Control / Border",
                "Rest",
                "TextControlElevationBorderBrush",
            ),
            tile(
                "Text Control / Border Focused",
                "Active text fields",
                "TextControlElevationBorderFocusedBrush",
            ),
            tile(
                "Accent Control / Border",
                "Rest",
                "AccentControlElevationBorderBrush",
            ),
        ],
    ),
    group(
        "Control Stroke",
        "Gradient stops in elevation borders and control states",
        &[
            tile(
                "Control Stroke / Default",
                "Pressed or disabled elevation brushes",
                "ControlStrokeColorDefaultBrush",
            ),
            tile(
                "Control Stroke / Secondary",
                "Used in control elevation brushes",
                "ControlStrokeColorSecondaryBrush",
            ),
            tile(
                "Control Stroke / On Accent Default",
                "Pressed or disabled accent elevation",
                "ControlStrokeColorOnAccentDefaultBrush",
            ),
            tile(
                "Control Stroke / On Accent Secondary",
                "Accent elevation brushes",
                "ControlStrokeColorOnAccentSecondaryBrush",
            ),
            tile(
                "Control Stroke / On Accent Tertiary",
                "Linework on accent controls",
                "ControlStrokeColorOnAccentTertiaryBrush",
            ),
            tile(
                "Control Stroke / On Accent Disabled",
                "Disabled",
                "ControlStrokeColorOnAccentDisabledBrush",
            ),
            tile(
                "Control Stroke / For Strong Fill When On Image",
                "Ensures 3:1 contrast on any background",
                "ControlStrokeColorForStrongFillWhenOnImageBrush",
            ),
        ],
    ),
    group(
        "Control Strong Stroke",
        "Control strokes requiring 3:1 contrast",
        &[
            tile(
                "Control Strong Stroke / Default",
                "3:1 control border",
                "ControlStrongStrokeColorDefaultBrush",
            ),
            tile(
                "Control Strong Stroke / Disabled",
                "Disabled",
                "ControlStrongStrokeColorDisabledBrush",
            ),
        ],
    ),
    group(
        "Surface Stroke",
        "Strokes on flyouts, windows and dialogs",
        &[
            tile(
                "Surface Stroke / Default",
                "Window and dialog borders, theme inverse",
                "SurfaceStrokeColorDefaultBrush",
            ),
            tile(
                "Surface Stroke / Flyout",
                "Control flyouts, always dark",
                "SurfaceStrokeColorFlyoutBrush",
            ),
        ],
    ),
    group(
        "Divider Stroke",
        "Divider and graphic lines",
        &[tile(
            "Divider Stroke / Default",
            "Content dividers",
            "DividerStrokeColorDefaultBrush",
        )],
    ),
    group(
        "Focus Stroke",
        "Theme-inverse keyboard focus indication",
        &[
            tile(
                "Focus / Outer",
                "Outer stroke color",
                "FocusStrokeColorOuterBrush",
            ),
            tile(
                "Focus / Inner",
                "Inner stroke color",
                "FocusStrokeColorInnerBrush",
            ),
        ],
    ),
];

const BACKGROUND_GROUPS: &[GroupSpec] = &[
    group(
        "Card Background",
        "Content blocks on page and layer backgrounds",
        &[
            tile(
                "Card Background / Default",
                "Default card color",
                "CardBackgroundFillColorDefaultBrush",
            ),
            tile(
                "Card Background / Secondary",
                "Alternate card color",
                "CardBackgroundFillColorSecondaryBrush",
            ),
            tile(
                "Card Background / Tertiary",
                "Card hover and pressed",
                "CardBackgroundTertiaryBrush",
            ),
        ],
    ),
    group(
        "Smoke Background",
        "Blocks inaccessible content behind dialogs",
        &[tile(
            "Smoke / Default",
            "Dims the background behind dialogs",
            "SmokeFillColorDefaultBrush",
        )],
    ),
    group(
        "Layer",
        "Layering on background colors and materials",
        &[
            tile(
                "Layer / Default",
                "Content layer color",
                "LayerFillColorDefaultBrush",
            ),
            tile(
                "Layer / Alt",
                "Alternate content layer color",
                "LayerFillColorAltBrush",
            ),
        ],
    ),
    group(
        "Layer on Acrylic",
        "Layering on acrylic surfaces",
        &[tile(
            "Layer On Acrylic / Default",
            "Content layer on acrylic",
            "LayerOnAcrylicFillColorDefaultBrush",
        )],
    ),
    group(
        "Layer on Mica Base Alt",
        "Fills used on Tab controls",
        &[
            tile(
                "Layer On Mica Base Alt / Default",
                "Active tab rest and content layer",
                "LayerOnMicaBaseAltFillColorDefaultBrush",
            ),
            tile(
                "Layer On Mica Base Alt / Tertiary",
                "Active tab drag",
                "LayerOnMicaBaseAltFillColorTertiaryBrush",
            ),
            tile(
                "Layer On Mica Base Alt / Transparent",
                "Inactive tab rest",
                "LayerOnMicaBaseAltFillColorTransparentBrush",
            ),
            tile(
                "Layer On Mica Base Alt / Secondary",
                "Inactive tab hover",
                "LayerOnMicaBaseAltFillColorSecondaryBrush",
            ),
        ],
    ),
    group(
        "Solid Background",
        "Solid backgrounds for layers, cards and controls",
        &[
            tile(
                "Solid Background / Base",
                "Bottom-most layer",
                "SolidBackgroundFillColorBaseBrush",
            ),
            tile(
                "Solid Background / Base Alt",
                "Alternate bottom-most layer",
                "SolidBackgroundFillColorBaseAltBrush",
            ),
            tile(
                "Solid Background / Secondary",
                "Alternate darker base",
                "SolidBackgroundFillColorSecondaryBrush",
            ),
            tile(
                "Solid Background / Tertiary",
                "Content layer color",
                "SolidBackgroundFillColorTertiaryBrush",
            ),
            tile(
                "Solid Background / Quarternary",
                "Alternate content layer",
                "SolidBackgroundFillColorQuarternaryBrush",
            ),
            tile(
                "Solid Background / Quinary",
                "Solid default card colors",
                "SolidBackgroundFillColorQuinaryBrush",
            ),
            tile(
                "Solid Background / Senary",
                "Solid default card color",
                "SolidBackgroundFillColorSenaryBrush",
            ),
        ],
    ),
    group(
        "Mica Background",
        "Mica backgrounds for layers, cards and controls",
        &[
            tile(
                "Mica Background / Base",
                "Bottom-most experience layer",
                "MicaBackgroundFillColorBaseBrush",
            ),
            tile(
                "Mica Background / Base Alt",
                "Default tab band background",
                "MicaBackgroundFillColorBaseAltBrush",
            ),
        ],
    ),
    group(
        "Acrylic Background",
        "Acrylic backgrounds for layers, cards and controls",
        &[
            tile(
                "Acrylic Background / Base",
                "Bottom acrylic layer beneath content layers",
                "AcrylicBackgroundFillColorBaseBrush",
            ),
            tile(
                "Acrylic Background / Default",
                "Default control flyout acrylic recipe",
                "AcrylicBackgroundFillColorDefaultBrush",
            ),
        ],
    ),
    group(
        "Accent Acrylic Background",
        "Accent acrylic backgrounds",
        &[
            tile(
                "Accent Acrylic Background / Base",
                "Bottom accent acrylic layer",
                "AccentAcrylicBackgroundFillColorBaseBrush",
            ),
            tile(
                "Accent Acrylic Background / Default",
                "Default accent acrylic flyout recipe",
                "AccentAcrylicBackgroundFillColorDefaultBrush",
            ),
        ],
    ),
];

const SIGNAL_GROUPS: &[GroupSpec] = &[group(
    "System",
    "System status and notification colors",
    &[
        tile("System / Success", "Badge", "SystemFillColorSuccessBrush"),
        tile("System / Caution", "Badge", "SystemFillColorCautionBrush"),
        tile("System / Critical", "Badge", "SystemFillColorCriticalBrush"),
        tile(
            "System / Success Background",
            "InfoBar background",
            "SystemFillColorSuccessBackgroundBrush",
        ),
        tile(
            "System / Caution Background",
            "InfoBar background",
            "SystemFillColorCautionBackgroundBrush",
        ),
        tile(
            "System / Critical Background",
            "InfoBar background",
            "SystemFillColorCriticalBackgroundBrush",
        ),
        tile(
            "System / Attention",
            "Badge",
            "SystemFillColorAttentionBrush",
        ),
        tile("System / Neutral", "Badge", "SystemFillColorNeutralBrush"),
        tile(
            "System / Solid Neutral",
            "Neutral badges over content",
            "SystemFillColorSolidNeutralBrush",
        ),
        tile(
            "System / Attention Background",
            "InfoBar background",
            "SystemFillColorAttentionBackgroundBrush",
        ),
        tile(
            "System / Neutral Background",
            "InfoBar background",
            "SystemFillColorNeutralBackgroundBrush",
        ),
        tile(
            "System / Solid Neutral Background",
            "Neutral badges over content",
            "SystemFillColorSolidNeutralBackgroundBrush",
        ),
        tile(
            "System / Solid Attention Background",
            "",
            "SystemFillColorSolidAttentionBackgroundBrush",
        ),
    ],
)];

const HIGH_CONTRAST_TILES: &[TileSpec] = &[
    tile(
        "Window Text Color",
        "Headings, body, lists, placeholders and non-interactive borders",
        "SystemColorWindowTextColor",
    ),
    tile(
        "Window Color",
        "Background of pages, panes, popups and windows",
        "SystemColorWindowColor",
    ),
    tile(
        "Highlight Text Color",
        "Selected, interacted-with or in-progress foreground",
        "SystemColorHighlightTextColor",
    ),
    tile(
        "Highlight Color",
        "Selected, interacted-with or in-progress background",
        "SystemColorHighlightColor",
    ),
    tile(
        "Button Text Color",
        "Foreground for buttons and interactive UI",
        "SystemColorButtonTextColor",
    ),
    tile(
        "Button Face Color",
        "Background for buttons and interactive UI",
        "SystemColorButtonFaceColor",
    ),
    tile(
        "Hotlight Color",
        "Hyperlink foreground",
        "SystemColorHotlightColor",
    ),
    tile(
        "Gray Text Color / Disabled",
        "Inactive or disabled UI foreground",
        "SystemColorGrayTextColor",
    ),
];

const HIGH_CONTRAST_GROUPS: &[GroupSpec] = &[
    group(
        "Aquatic",
        "Default high contrast theme",
        HIGH_CONTRAST_TILES,
    ),
    group("Desert", "Default high contrast theme", HIGH_CONTRAST_TILES),
    group("Dusk", "Default high contrast theme", HIGH_CONTRAST_TILES),
    group(
        "Night Sky",
        "Default high contrast theme",
        HIGH_CONTRAST_TILES,
    ),
];

fn brush_groups(p: FluentPalette, theme: FluentTheme) -> [Vec<BrushGroup>; 6] {
    [
        resolve_groups(TEXT_GROUPS, p, theme),
        resolve_groups(FILL_GROUPS, p, theme),
        resolve_groups(STROKE_GROUPS, p, theme),
        resolve_groups(BACKGROUND_GROUPS, p, theme),
        resolve_groups(SIGNAL_GROUPS, p, theme),
        resolve_groups(HIGH_CONTRAST_GROUPS, p, theme),
    ]
}

fn resolve_groups(
    groups: &'static [GroupSpec],
    p: FluentPalette,
    theme: FluentTheme,
) -> Vec<BrushGroup> {
    groups
        .iter()
        .enumerate()
        .map(|(group_index, group)| {
            let tiles = group
                .tiles
                .iter()
                .enumerate()
                .map(|(tile_index, tile)| {
                    let color =
                        resolve_brush_color(tile.resource, p, theme, group_index * 11 + tile_index);
                    BrushTile {
                        name: tile.name,
                        usage: tile.usage,
                        resource: tile.resource,
                        color,
                        foreground: contrasting_text(color),
                    }
                })
                .collect();
            (group.title, group.subtitle, tiles)
        })
        .collect()
}

fn resolve_brush_color(
    resource: &str,
    p: FluentPalette,
    theme: FluentTheme,
    variant: usize,
) -> Color32 {
    if resource == "TextFillColorPrimaryBrush" {
        return p.text_primary;
    }
    if resource == "TextFillColorSecondaryBrush" {
        return p.text_secondary;
    }
    if resource == "TextFillColorTertiaryBrush" {
        return if theme.is_dark() {
            Color32::from_gray(160)
        } else {
            Color32::from_gray(114)
        };
    }
    if resource == "TextFillColorDisabledBrush" {
        return p.text_disabled;
    }
    if resource == "AccentTextFillColorPrimaryBrush" {
        return p.accent_hover;
    }
    if resource == "AccentTextFillColorSecondaryBrush" {
        return p.accent;
    }
    if resource == "AccentTextFillColorTertiaryBrush" {
        return p.accent_pressed;
    }
    if resource == "TextOnAccentFillColorPrimaryBrush" {
        return p.on_accent;
    }
    if resource == "TextOnAccentFillColorSecondaryBrush" {
        return if theme.is_dark() {
            Color32::from_gray(30)
        } else {
            Color32::from_gray(235)
        };
    }
    if resource == "TextOnAccentFillColorDisabledBrush" {
        return Color32::from_gray(126);
    }
    if resource == "TextOnAccentFillColorSelectedTextBrush" {
        return p.accent;
    }
    if resource.contains("Success") {
        return if resource.contains("Background") {
            Color32::from_rgb(57, 61, 27)
        } else {
            p.success
        };
    }
    if resource.contains("Caution") {
        return if resource.contains("Background") {
            Color32::from_rgb(67, 53, 25)
        } else {
            p.warning
        };
    }
    if resource.contains("Critical") {
        return if resource.contains("Background") {
            Color32::from_rgb(68, 39, 38)
        } else {
            p.danger
        };
    }
    if resource.contains("Accent") || resource.contains("Highlight") {
        return match variant % 4 {
            0 => p.accent_hover,
            1 => p.accent,
            2 => p.accent_pressed,
            _ => Color32::from_rgb(126, 126, 126),
        };
    }
    if resource.contains("WindowText") || resource.contains("ButtonText") {
        return if theme.is_dark() {
            Color32::WHITE
        } else {
            Color32::BLACK
        };
    }
    if resource.contains("WindowColor") || resource.contains("ButtonFace") {
        return if theme.is_dark() {
            Color32::from_gray(18)
        } else {
            Color32::from_gray(246)
        };
    }
    if resource.contains("Hotlight") {
        return Color32::from_rgb(0, 102, 204);
    }
    if resource.contains("GrayText") || resource.contains("Disabled") {
        return Color32::from_gray(126);
    }
    if resource.contains("Transparent") {
        return if theme.is_dark() {
            Color32::from_gray(42)
        } else {
            Color32::from_gray(248)
        };
    }

    let hash = resource
        .bytes()
        .fold(variant as u32 * 31 + 17, |value, byte| {
            value.wrapping_mul(33).wrapping_add(byte as u32)
        });
    let offset = (hash % 54) as u8;
    if theme.is_dark() {
        Color32::from_rgb(
            34_u8.saturating_add(offset / 2),
            34_u8.saturating_add(offset / 3),
            36_u8.saturating_add(offset / 4),
        )
    } else {
        Color32::from_rgb(
            248_u8.saturating_sub(offset / 2),
            248_u8.saturating_sub(offset / 3),
            250_u8.saturating_sub(offset / 4),
        )
    }
}

fn contrasting_text(color: Color32) -> Color32 {
    let luminance =
        0.2126 * color.r() as f32 + 0.7152 * color.g() as f32 + 0.0722 * color.b() as f32;
    if luminance > 145.0 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_guidance_matches_official_tile_counts() {
        let counts = [
            TEXT_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
            FILL_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
            STROKE_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
            BACKGROUND_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
            SIGNAL_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
            HIGH_CONTRAST_GROUPS
                .iter()
                .map(|group| group.tiles.len())
                .sum::<usize>(),
        ];
        assert_eq!(counts, [12, 28, 21, 24, 13, 32]);
        assert_eq!(counts.into_iter().sum::<usize>(), 130);
    }
}
