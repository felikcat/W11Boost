// DPI-aware layout system for W11Boost GUI
// All base values are at 96 DPI (100% scaling)
#![allow(dead_code)]

/// Base layout values at 96 DPI
pub mod base
{
        // Margins
        pub const MARGIN: i32 = 20;
        pub const APP_INDENT: i32 = 30;
        pub const COLUMN_GAP: i32 = 10;

        // Spacing
        pub const ROW_SPACING: i32 = 28;
        pub const APP_ROW_SPACING: i32 = 20;
        pub const SECTION_GAP: i32 = 28;
        pub const BUTTON_GAP: i32 = 8;
        pub const STATUS_GAP: i32 = 40;
        pub const STATUS_TO_BUTTON: i32 = 50;
        pub const BOTTOM_PAD: i32 = 60;

        // Component sizes
        pub const CHECKBOX_H: i32 = 24;
        pub const APP_CHECKBOX_H: i32 = 20;
        pub const BUTTON_W_MAIN: i32 = 180;
        pub const BUTTON_H_MAIN: i32 = 36;
        pub const BUTTON_W_CONFIRM: i32 = 100;
        pub const BUTTON_W_SELECT: i32 = 90;
        pub const BUTTON_H_SELECT: i32 = 24;
        pub const STATUS_H: i32 = 60;
        pub const COL_MIN_W: i32 = 150;

        // Collapsed layout offsets from bottom
        pub const STATUS_FROM_BOTTOM: i32 = 150;
        pub const BUTTON_FROM_BOTTOM: i32 = 60;

        // Progress bar
        pub const PROGRESS_H: i32 = 20;
        pub const PROGRESS_MARGIN: i32 = 10;
}

/// Scroll behavior (not DPI-scaled)
pub mod scroll
{
        pub const LINE: i32 = 20;
        pub const PAGE: i32 = 100;
        pub const WHEEL_DIVISOR: i32 = 40;
}

/// Window dimensions at 96 DPI
pub mod window
{
        pub const WIDTH: i32 = 630;
        pub const HEIGHT: i32 = 340;
}

/// Pre-computed layout metrics scaled for current DPI
#[allow(dead_code)]
pub struct LayoutMetrics
{
        pub scale: f32,

        // Margins
        pub margin: i32,
        pub app_indent: i32,
        pub column_gap: i32,

        // Spacing
        pub row_spacing: i32,
        pub app_row_spacing: i32,
        pub section_gap: i32,
        pub button_gap: i32,
        pub status_gap: i32,
        pub status_to_button: i32,
        pub bottom_pad: i32,

        // Component sizes
        pub checkbox_h: i32,
        pub app_checkbox_h: i32,
        pub button_w_main: i32,
        pub button_h_main: i32,
        pub button_w_confirm: i32,
        pub button_w_select: i32,
        pub button_h_select: i32,
        pub status_h: i32,
        pub col_min_w: i32,

        // Collapsed layout
        pub status_from_bottom: i32,
        pub button_from_bottom: i32,

        // Progress bar
        pub progress_h: i32,
        pub progress_margin: i32,
}

impl LayoutMetrics
{
        /// Create metrics scaled for given DPI
        #[allow(clippy::cast_precision_loss)]
        pub fn for_dpi(dpi: u32) -> Self
        {
                let scale = dpi as f32 / 96.0;

                Self {
                        scale,

                        margin: Self::scaled(base::MARGIN, scale),
                        app_indent: Self::scaled(base::APP_INDENT, scale),
                        column_gap: Self::scaled(base::COLUMN_GAP, scale),

                        row_spacing: Self::scaled(base::ROW_SPACING, scale),
                        app_row_spacing: Self::scaled(base::APP_ROW_SPACING, scale),
                        section_gap: Self::scaled(base::SECTION_GAP, scale),
                        button_gap: Self::scaled(base::BUTTON_GAP, scale),
                        status_gap: Self::scaled(base::STATUS_GAP, scale),
                        status_to_button: Self::scaled(base::STATUS_TO_BUTTON, scale),
                        bottom_pad: Self::scaled(base::BOTTOM_PAD, scale),

                        checkbox_h: Self::scaled(base::CHECKBOX_H, scale),
                        app_checkbox_h: Self::scaled(base::APP_CHECKBOX_H, scale),
                        button_w_main: Self::scaled(base::BUTTON_W_MAIN, scale),
                        button_h_main: Self::scaled(base::BUTTON_H_MAIN, scale),
                        button_w_confirm: Self::scaled(base::BUTTON_W_CONFIRM, scale),
                        button_w_select: Self::scaled(base::BUTTON_W_SELECT, scale),
                        button_h_select: Self::scaled(base::BUTTON_H_SELECT, scale),
                        status_h: Self::scaled(base::STATUS_H, scale),
                        col_min_w: Self::scaled(base::COL_MIN_W, scale),

                        status_from_bottom: Self::scaled(base::STATUS_FROM_BOTTOM, scale),
                        button_from_bottom: Self::scaled(base::BUTTON_FROM_BOTTOM, scale),

                        progress_h: Self::scaled(base::PROGRESS_H, scale),
                        progress_margin: Self::scaled(base::PROGRESS_MARGIN, scale),
                }
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        #[inline]
        fn scaled(base: i32, scale: f32) -> i32
        {
                (base as f32 * scale) as i32
        }

        /// Left column X position
        #[inline]
        pub const fn x_left(&self) -> i32
        {
                self.margin
        }

        /// Right column X position
        #[inline]
        pub const fn x_right(&self, client_width: i32) -> i32
        {
                client_width / 2 + self.column_gap
        }

        /// Checkbox width for two-column layout
        #[inline]
        pub const fn cb_width(&self, client_width: i32) -> i32
        {
                client_width / 2 - self.margin - self.column_gap
        }

        /// Calculate app checkbox column layout
        pub fn app_columns(&self, client_width: i32, num_apps: i32) -> (i32, i32, i32)
        {
                let area_width = client_width - 2 * self.margin;
                let num_cols = (area_width / self.col_min_w).clamp(2, 4);
                let col_width = area_width / num_cols;
                let apps_per_col = (num_apps + num_cols - 1) / num_cols;
                (num_cols, col_width, apps_per_col)
        }
}

impl Default for LayoutMetrics
{
        fn default() -> Self
        {
                Self::for_dpi(96)
        }
}
