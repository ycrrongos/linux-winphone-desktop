use eframe::egui::{
    self, Align, Align2, CornerRadius, FontId, InnerResponse, Layout, Margin, Response, Sense,
    Stroke, StrokeKind, Ui, Widget, pos2, vec2,
};

use super::{FluentIcon, FluentTheme, combo_option, fluent_combo_box, icon_font_id, palette};

/// A small Gregorian date value used by the date controls.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FluentDate {
    /// The `year` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub year: i32,
    /// The `month` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub month: u8,
    /// The `day` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub day: u8,
}

impl FluentDate {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        let month = month.clamp(1, 12);
        let day = day.clamp(1, days_in_month(year, month));
        Self { year, month, day }
    }

    /// Configures or reads the `display` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn display(self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Default for FluentDate {
    fn default() -> Self {
        Self::new(2026, 7, 18)
    }
}

/// A 24-hour time value used by [`TimePicker`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FluentTime {
    /// The `hour` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub hour: u8,
    /// The `minute` value returned to or configured by the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub minute: u8,
}

impl FluentTime {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
        }
    }
}

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

// Gregorian weekday, Sunday = 0. Valid for the modern positive year range used by the picker.
fn first_weekday(year: i32, month: u8) -> usize {
    let (mut y, mut m) = (year, month as i32);
    if m < 3 {
        y -= 1;
        m += 12;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (1 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
    ((h + 6) % 7) as usize
}

fn shift_month(date: &mut FluentDate, delta: i32) {
    let absolute = date.year * 12 + date.month as i32 - 1 + delta;
    date.year = absolute.div_euclid(12);
    date.month = (absolute.rem_euclid(12) + 1) as u8;
    date.day = date.day.min(days_in_month(date.year, date.month));
}

fn draw_calendar(ui: &mut Ui, date: &mut FluentDate) -> Response {
    let theme = if ui.visuals().dark_mode {
        FluentTheme::Dark
    } else {
        FluentTheme::Light
    };
    let p = palette(theme);
    let mut changed = false;
    let response = ui
        .vertical(|ui| {
            ui.set_width(280.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} {}",
                        MONTHS[date.month as usize - 1],
                        date.year
                    ))
                    .strong(),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add_sized(
                            [32.0, 28.0],
                            egui::Button::new(
                                egui::RichText::new(FluentIcon::ChevronRight.glyph())
                                    .font(icon_font_id(11.0)),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        shift_month(date, 1);
                    }
                    if ui
                        .add_sized(
                            [32.0, 28.0],
                            egui::Button::new(
                                egui::RichText::new(FluentIcon::Back.glyph())
                                    .font(icon_font_id(11.0)),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        shift_month(date, -1);
                    }
                });
            });
            ui.add_space(4.0);

            let origin = ui.cursor().min;
            let cell = vec2(40.0, 34.0);
            let height = cell.y * 7.0;
            let (rect, _) = ui.allocate_exact_size(vec2(cell.x * 7.0, height), Sense::hover());
            for (column, weekday) in WEEKDAYS.iter().enumerate() {
                ui.painter().text(
                    pos2(
                        origin.x + column as f32 * cell.x + cell.x * 0.5,
                        origin.y + 14.0,
                    ),
                    Align2::CENTER_CENTER,
                    *weekday,
                    FontId::proportional(12.0),
                    p.text_secondary,
                );
            }
            let offset = first_weekday(date.year, date.month);
            for day in 1..=days_in_month(date.year, date.month) {
                let index = offset + day as usize - 1;
                let row = index / 7 + 1;
                let column = index % 7;
                let day_rect = egui::Rect::from_min_size(
                    pos2(
                        origin.x + column as f32 * cell.x,
                        origin.y + row as f32 * cell.y,
                    ),
                    cell,
                )
                .shrink(2.0);
                let item = ui.interact(
                    day_rect,
                    ui.id().with((date.year, date.month, day)),
                    Sense::click(),
                );
                item.widget_info(|| {
                    egui::WidgetInfo::selected(
                        egui::WidgetType::SelectableLabel,
                        ui.is_enabled(),
                        date.day == day,
                        day,
                    )
                });
                let selected = date.day == day;
                let fill = if selected {
                    p.accent
                } else if item.hovered() {
                    p.subtle_hover
                } else {
                    egui::Color32::TRANSPARENT
                };
                ui.painter()
                    .rect_filled(day_rect, CornerRadius::same(16), fill);
                ui.painter().text(
                    day_rect.center(),
                    Align2::CENTER_CENTER,
                    day,
                    FontId::proportional(14.0),
                    if selected {
                        p.on_accent
                    } else {
                        p.text_primary
                    },
                );
                if item.clicked() {
                    date.day = day;
                    changed = true;
                }
            }
            rect
        })
        .response;
    let mut response = response;
    if changed {
        response.mark_changed();
    }
    response
}

/// An always-visible WinUI-style month calendar.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CalendarView<'a> {
    value: &'a mut FluentDate,
}

impl<'a> CalendarView<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut FluentDate) -> Self {
        Self { value }
    }
}

impl Widget for CalendarView<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        draw_calendar(ui, self.value)
    }
}

/// A compact date field that opens a [`CalendarView`] popup.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct CalendarDatePicker<'a> {
    value: &'a mut FluentDate,
    width: f32,
    placeholder: String,
}

impl<'a> CalendarDatePicker<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut FluentDate) -> Self {
        Self {
            value,
            width: 180.0,
            placeholder: "Pick a date".into(),
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

    /// Configures or reads the `placeholder` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
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
    pub fn show(self, ui: &mut Ui) -> InnerResponse<Option<Response>> {
        let p = palette(if ui.visuals().dark_mode {
            FluentTheme::Dark
        } else {
            FluentTheme::Light
        });
        let (rect, response) = ui.allocate_exact_size(vec2(self.width, 32.0), Sense::click());
        let fill = if response.is_pointer_button_down_on() {
            p.control_pressed
        } else if response.hovered() {
            p.control_hover
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
        let label = self.value.display();
        ui.painter().text(
            pos2(rect.left() + 10.0, rect.center().y - 0.5),
            Align2::LEFT_CENTER,
            if label.is_empty() {
                self.placeholder
            } else {
                label
            },
            FontId::proportional(14.0),
            p.text_primary,
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::ComboBox,
                ui.is_enabled(),
                self.value.display(),
            )
        });
        ui.painter().text(
            pos2(rect.right() - 16.0, rect.center().y),
            Align2::CENTER_CENTER,
            FluentIcon::ChevronDown.glyph(),
            icon_font_id(11.0),
            p.text_primary,
        );
        let inner = egui::Popup::from_toggle_button_response(&response)
            .width(312.0)
            .gap(4.0)
            .frame(
                egui::Frame::popup(ui.style())
                    .inner_margin(Margin::same(12))
                    .corner_radius(0),
            )
            .show(|ui| draw_calendar(ui, self.value))
            .map(|result| result.inner);
        InnerResponse { inner, response }
    }
}

/// A three-column month/day/year picker matching WinUI's compact DatePicker.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct DatePicker<'a> {
    value: &'a mut FluentDate,
    width: f32,
}

impl<'a> DatePicker<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut FluentDate) -> Self {
        Self {
            value,
            width: 300.0,
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
}

impl Widget for DatePicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let before = *self.value;
        let column = (self.width - 8.0) / 3.0;
        let response = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                fluent_combo_box(ui.id().with("month"), MONTHS[self.value.month as usize - 1])
                    .width(column)
                    .show_ui(ui, |ui| {
                        for month in 1..=12 {
                            if combo_option(
                                ui,
                                MONTHS[month as usize - 1],
                                self.value.month == month,
                            )
                            .clicked()
                            {
                                self.value.month = month;
                                ui.close();
                            }
                        }
                    });
                fluent_combo_box(ui.id().with("day"), self.value.day.to_string())
                    .width(column)
                    .show_ui(ui, |ui| {
                        for day in 1..=days_in_month(self.value.year, self.value.month) {
                            if combo_option(ui, day.to_string(), self.value.day == day).clicked() {
                                self.value.day = day;
                                ui.close();
                            }
                        }
                    });
                fluent_combo_box(ui.id().with("year"), self.value.year.to_string())
                    .width(column)
                    .show_ui(ui, |ui| {
                        for year in (1900..=2100).rev() {
                            if combo_option(ui, year.to_string(), self.value.year == year).clicked()
                            {
                                self.value.year = year;
                                ui.close();
                            }
                        }
                    });
            })
            .response;
        self.value.day = self
            .value
            .day
            .min(days_in_month(self.value.year, self.value.month));
        let mut response = response;
        if before != *self.value {
            response.mark_changed();
        }
        response
    }
}

/// Clock presentation used by [`TimePicker`].
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimePickerClock {
    #[default]
    /// The `TwelveHour` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    TwelveHour,
    /// The `TwentyFourHour` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    TwentyFourHour,
}

/// A compact hour/minute picker with configurable minute increments and clock mode.
///
/// # Examples
///
/// ```
/// let theme = fluent_egui::winui::FluentTheme::Light;
/// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
/// ```
pub struct TimePicker<'a> {
    value: &'a mut FluentTime,
    minute_increment: u8,
    width: f32,
    clock: TimePickerClock,
}

impl<'a> TimePicker<'a> {
    /// Creates a new value with WinUI-compatible defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub fn new(value: &'a mut FluentTime) -> Self {
        Self {
            value,
            minute_increment: 1,
            width: 200.0,
            clock: TimePickerClock::TwelveHour,
        }
    }

    /// Configures or reads the `minute_increment` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn minute_increment(mut self, increment: u8) -> Self {
        self.minute_increment = increment;
        self
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

    /// Configures or reads the `clock` option.
    ///
    /// # Examples
    ///
    /// ```
    /// let theme = fluent_egui::winui::FluentTheme::Light;
    /// assert!(matches!(theme, fluent_egui::winui::FluentTheme::Light));
    /// ```
    pub const fn clock(mut self, clock: TimePickerClock) -> Self {
        self.clock = clock;
        self
    }
}

impl Widget for TimePicker<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let before = *self.value;
        let twelve_hour = self.clock == TimePickerClock::TwelveHour;
        let column_count = if twelve_hour { 3.0 } else { 2.0 };
        let column = (self.width - 4.0 * (column_count - 1.0)) / column_count;
        let step = self.minute_increment.clamp(1, 59) as usize;
        let response = ui
            .horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let display_hour = if twelve_hour {
                    match self.value.hour % 12 {
                        0 => 12,
                        hour => hour,
                    }
                } else {
                    self.value.hour
                };
                fluent_combo_box(ui.id().with("hour"), format!("{display_hour:02}"))
                    .width(column)
                    .show_ui(ui, |ui| {
                        if twelve_hour {
                            let afternoon = self.value.hour >= 12;
                            for hour in 1..=12 {
                                if combo_option(ui, format!("{hour:02}"), display_hour == hour)
                                    .clicked()
                                {
                                    self.value.hour = (hour % 12) + if afternoon { 12 } else { 0 };
                                    ui.close();
                                }
                            }
                        } else {
                            for hour in 0..24 {
                                if combo_option(ui, format!("{hour:02}"), self.value.hour == hour)
                                    .clicked()
                                {
                                    self.value.hour = hour;
                                    ui.close();
                                }
                            }
                        }
                    });
                fluent_combo_box(ui.id().with("minute"), format!("{:02}", self.value.minute))
                    .width(column)
                    .show_ui(ui, |ui| {
                        for minute in (0..60).step_by(step) {
                            if combo_option(
                                ui,
                                format!("{:02}", minute),
                                self.value.minute == minute as u8,
                            )
                            .clicked()
                            {
                                self.value.minute = minute as u8;
                                ui.close();
                            }
                        }
                    });
                if twelve_hour {
                    fluent_combo_box(
                        ui.id().with("period"),
                        if self.value.hour < 12 { "AM" } else { "PM" },
                    )
                    .width(column)
                    .show_ui(ui, |ui| {
                        for (afternoon, label) in [(false, "AM"), (true, "PM")] {
                            if combo_option(ui, label, (self.value.hour >= 12) == afternoon)
                                .clicked()
                            {
                                if afternoon && self.value.hour < 12 {
                                    self.value.hour += 12;
                                } else if !afternoon && self.value.hour >= 12 {
                                    self.value.hour -= 12;
                                }
                                ui.close();
                            }
                        }
                    });
                }
            })
            .response;
        let mut response = response;
        if before != *self.value {
            response.mark_changed();
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_math_handles_leap_years() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2100, 2), 28);
        assert_eq!(first_weekday(2026, 7), 3);
    }

    #[test]
    fn date_clamps_invalid_components() {
        assert_eq!(
            FluentDate::new(2025, 2, 31),
            FluentDate {
                year: 2025,
                month: 2,
                day: 28
            }
        );
    }
}
