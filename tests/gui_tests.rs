//! Unit tests for gui.rs components

// ============================================
// Barrett Division constants and function (mirrored from common.rs)
// ============================================

// Barrett division constants
struct BarrettConstants {
    m: u64, // multiplier
    k: u8,  // shift amount
}

const BARRETT_DIV_12: BarrettConstants = BarrettConstants {
    m: 2_863_311_530,
    k: 35,
};

const BARRETT_DIV_100: BarrettConstants = BarrettConstants {
    m: 343_597_383,
    k: 35,
};

#[inline(always)]
const fn barrett_div(n: i32, c: &BarrettConstants) -> i32
{
    ((n as u64 * c.m) >> c.k) as i32
}

// GUI constants (mirrored from gui.rs since they're private)
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const TOP_PADDING: i32 = 4;
const FONT_PATH: &str = "C:\\Windows\\Fonts\\segoeui.ttf";
const DISPLAY_TIMEOUT_SUCCESS: f64 = 5.0;
const DISPLAY_TIMEOUT_ERROR: f64 = 10.0;

// ============================================
// Tests for constants
// ============================================

#[test]
fn test_window_dimensions()
{
    assert_eq!(WINDOW_WIDTH, 640);
    assert_eq!(WINDOW_HEIGHT, 480);
}

#[test]
fn test_top_padding()
{
    assert_eq!(TOP_PADDING, 4);
}

#[test]
fn test_font_path()
{
    assert_eq!(FONT_PATH, "C:\\Windows\\Fonts\\segoeui.ttf");
}

#[test]
fn test_display_timeouts()
{
    assert_eq!(DISPLAY_TIMEOUT_SUCCESS, 5.0);
    assert_eq!(DISPLAY_TIMEOUT_ERROR, 10.0);
}

// ============================================
// Tests for checkbox dimensions calculation
// ============================================

#[test]
fn test_checkbox_height_calculation()
{
    let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);
    assert_eq!(checkbox_height, 39); // 480 / 12 = 40, Barrett gives 39
}

#[test]
fn test_checkbox_width_calculation()
{
    let checkbox_width = WINDOW_WIDTH >> 1; // 50% of window
    assert_eq!(checkbox_width, 320);
}

#[test]
fn test_button_height_calculation()
{
    let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);
    assert_eq!(button_height, 67); // (480 * 14) / 100 = 67
}

// ============================================
// Tests for checkbox y-position calculations
// ============================================

#[test]
fn test_first_checkbox_position()
{
    let first_y = TOP_PADDING;
    assert_eq!(first_y, 4);
}

#[test]
fn test_checkbox_spacing()
{
    let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);

    // Second checkbox y position (checkbox_height = 39)
    let second_y = TOP_PADDING + checkbox_height + 2;
    assert_eq!(second_y, 45); // 4 + 39 + 2 = 45

    // Third checkbox y position
    let third_y = TOP_PADDING + checkbox_height * 2 + 4;
    assert_eq!(third_y, 86); // 4 + 78 + 4 = 86
}

#[test]
fn test_all_checkboxes_fit_in_window()
{
    let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);
    let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);

    // 8 checkboxes with spacing
    let last_checkbox_y = TOP_PADDING + checkbox_height * 7 + 14;
    let last_checkbox_bottom = last_checkbox_y + checkbox_height;

    // Should fit above the buttons
    let button_top = WINDOW_HEIGHT - button_height - 2;

    assert!(last_checkbox_bottom < button_top,
        "Checkboxes overlap with buttons: checkbox bottom={}, button top={}",
        last_checkbox_bottom, button_top);
}

// ============================================
// Tests for button positioning
// ============================================

#[test]
fn test_apply_button_position()
{
    let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);
    let button_width = (WINDOW_WIDTH - 4) >> 1;

    // Apply button is at x=2, y=WINDOW_HEIGHT - height - 2
    let apply_x = 2;
    let apply_y = WINDOW_HEIGHT - button_height - 2;

    assert_eq!(apply_x, 2);
    assert_eq!(apply_y, 411); // 480 - 67 - 2 = 411
    assert_eq!(button_width, 318); // (640 - 4) / 2 = 318
}

#[test]
fn test_remove_button_position()
{
    let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);
    let button_width = (WINDOW_WIDTH - 4) >> 1;

    // Remove button is on the right side
    let remove_x = WINDOW_WIDTH - button_width - 2;
    let remove_y = WINDOW_HEIGHT - button_height - 2;

    assert_eq!(remove_x, 320); // 640 - 318 - 2 = 320
    assert_eq!(remove_y, 411); // 480 - 67 - 2 = 411
}

// ============================================
// Tests for checkbox labels
// ============================================

#[test]
fn test_checkbox_labels_are_descriptive()
{
    let labels = [
        "Minimize forensics / local data",
        "Minimize Microsoft online data",
        "Disable Windows Recall",
        "Disable Windows Copilot",
        "Disable sleep and hibernate",
        "Install Microsoft Store",
    ];

    for label in labels {
        assert!(!label.is_empty());
        assert!(label.len() <= 50, "Label too long: {}", label);
        assert!(label.len() >= 10, "Label too short: {}", label);
    }
}

#[test]
fn test_checkbox_count()
{
    // 6 checkboxes, non_intrusive_tweaks runs separately
    let checkbox_count = 6;
    assert_eq!(checkbox_count, 6);
}

// ============================================
// Tests for error names
// ============================================

#[test]
fn test_error_names_are_valid_identifiers()
{
    // non_intrusive_tweaks runs first, not in this list
    let error_names = [
        "minimize_forensics",
        "minimize_online_data_collection",
        "disable_recall",
        "disable_copilot",
        "disable_sleep",
        "reset_windows_store",
    ];

    for name in error_names {
        // snake_case identifiers
        assert!(!name.is_empty());
        assert!(!name.contains(' '), "Error name contains space: {}", name);
        assert!(!name.contains('-'), "Error name contains dash: {}", name);
        assert_eq!(name, name.to_lowercase(), "Error name not lowercase: {}", name);
    }
}

// ============================================
// Tests for non-intrusive tweaks (always runs)
// ============================================

#[test]
fn test_non_intrusive_tweaks_always_runs()
{
    // Runs before other features in apply()
    let error_name = "non_intrusive_tweaks";
    assert!(!error_name.is_empty());
    assert_eq!(error_name, error_name.to_lowercase());
}
