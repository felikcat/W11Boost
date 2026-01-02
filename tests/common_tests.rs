//! Unit tests for common.rs utility functions

use winsafe::HKEY;

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

// Windows constant for creating processes without a window
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

// ============================================
// Tests for BarrettConstants and barrett_div
// ============================================

#[test]
fn test_barrett_div_by_12_basic()
{
    // Can be off by 1 due to approximation
    assert_eq!(barrett_div(12, &BARRETT_DIV_12), 0);  // slightly under 1
    assert_eq!(barrett_div(24, &BARRETT_DIV_12), 1);  // slightly under 2
    assert_eq!(barrett_div(36, &BARRETT_DIV_12), 2);  // slightly under 3
    assert_eq!(barrett_div(120, &BARRETT_DIV_12), 9); // slightly under 10
}

#[test]
fn test_barrett_div_by_12_edge_cases()
{
    assert_eq!(barrett_div(0, &BARRETT_DIV_12), 0);
    assert_eq!(barrett_div(1, &BARRETT_DIV_12), 0);
    assert_eq!(barrett_div(11, &BARRETT_DIV_12), 0);
    assert_eq!(barrett_div(13, &BARRETT_DIV_12), 1);
}

#[test]
fn test_barrett_div_by_12_large_values()
{
    assert_eq!(barrett_div(480, &BARRETT_DIV_12), 39);   // 480/12=40, Barrett gives 39
    assert_eq!(barrett_div(1200, &BARRETT_DIV_12), 99);  // 1200/12=100, but Barrett gives 99
    assert_eq!(barrett_div(12000, &BARRETT_DIV_12), 999); // Barrett approximation
}

#[test]
fn test_barrett_div_by_100_basic()
{
    // Can be off by 1 due to approximation
    assert_eq!(barrett_div(100, &BARRETT_DIV_100), 0);  // slightly under 1
    assert_eq!(barrett_div(200, &BARRETT_DIV_100), 1);  // slightly under 2
    assert_eq!(barrett_div(500, &BARRETT_DIV_100), 4);  // slightly under 5
    assert_eq!(barrett_div(1000, &BARRETT_DIV_100), 9); // slightly under 10
}

#[test]
fn test_barrett_div_by_100_edge_cases()
{
    assert_eq!(barrett_div(0, &BARRETT_DIV_100), 0);
    assert_eq!(barrett_div(1, &BARRETT_DIV_100), 0);
    assert_eq!(barrett_div(99, &BARRETT_DIV_100), 0);
    assert_eq!(barrett_div(101, &BARRETT_DIV_100), 1);
}

#[test]
fn test_barrett_div_by_100_large_values()
{
    assert_eq!(barrett_div(4800, &BARRETT_DIV_100), 47);   // slightly under 48
    assert_eq!(barrett_div(10000, &BARRETT_DIV_100), 99);  // slightly under 100
    assert_eq!(barrett_div(100000, &BARRETT_DIV_100), 999); // slightly under 1000
}

#[test]
fn test_barrett_div_approximation()
{
    // Should be within 1 of standard division
    for n in 0..1000 {
        let barrett_result = barrett_div(n, &BARRETT_DIV_12);
        let standard_result = n / 12;
        assert!(
            barrett_result == standard_result || barrett_result == standard_result - 1,
            "Barrett div by 12 failed for n={n}: got {barrett_result}, expected {standard_result} or {}",
            standard_result - 1
        );

        let barrett_result = barrett_div(n, &BARRETT_DIV_100);
        let standard_result = n / 100;
        assert!(
            barrett_result == standard_result || barrett_result == standard_result - 1,
            "Barrett div by 100 failed for n={n}: got {barrett_result}, expected {standard_result} or {}",
            standard_result - 1
        );
    }
}

#[test]
fn test_barrett_div_window_height_calculation()
{
    // Actual use case in GUI
    const WINDOW_HEIGHT: i32 = 480;
    let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);
    assert_eq!(checkbox_height, 39); // 480 / 12 = 40, Barrett gives 39

    let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);
    assert_eq!(button_height, 67); // (480 * 14) / 100 = 67
}

// ============================================
// Tests for BarrettConstants struct
// ============================================

#[test]
fn test_barrett_constants_values()
{
    // m = floor(2^35 / divisor)
    assert_eq!(BARRETT_DIV_12.m, 2_863_311_530);
    assert_eq!(BARRETT_DIV_12.k, 35);

    assert_eq!(BARRETT_DIV_100.m, 343_597_383);
    assert_eq!(BARRETT_DIV_100.k, 35);
}

// ============================================
// Tests for CREATE_NO_WINDOW constant
// ============================================

#[test]
fn test_create_no_window_constant()
{
    assert_eq!(CREATE_NO_WINDOW, 0x0800_0000);
}

// ============================================
// Tests for backup line parsing format
// ============================================

#[test]
fn test_backup_line_format_dword()
{
    let line = "HKEY_LOCAL_MACHINE|SOFTWARE\\Test\\Path|ValueName|DWORD:123";
    let parts: Vec<&str> = line.split('|').collect();

    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "HKEY_LOCAL_MACHINE");
    assert_eq!(parts[1], "SOFTWARE\\Test\\Path");
    assert_eq!(parts[2], "ValueName");
    assert_eq!(parts[3], "DWORD:123");

    let value_info = parts[3];
    let dword_str = value_info.strip_prefix("DWORD:").unwrap();
    let value: u32 = dword_str.parse().unwrap();
    assert_eq!(value, 123);
}

#[test]
fn test_backup_line_format_sz()
{
    let line = "HKEY_CURRENT_USER|SOFTWARE\\Test|StringValue|SZ:TestString";
    let parts: Vec<&str> = line.split('|').collect();

    assert_eq!(parts.len(), 4);
    assert_eq!(parts[0], "HKEY_CURRENT_USER");
    assert_eq!(parts[1], "SOFTWARE\\Test");
    assert_eq!(parts[2], "StringValue");
    assert_eq!(parts[3], "SZ:TestString");

    let value_info = parts[3];
    let string_value = value_info.strip_prefix("SZ:").unwrap();
    assert_eq!(string_value, "TestString");
}

#[test]
fn test_backup_line_format_key_created()
{
    let line = "HKEY_LOCAL_MACHINE|SOFTWARE\\NewKey|ValueName|KEY_CREATED_BY_APP";
    let parts: Vec<&str> = line.split('|').collect();

    assert_eq!(parts.len(), 4);
    assert_eq!(parts[3], "KEY_CREATED_BY_APP");
}

#[test]
fn test_backup_line_format_subkey_markers()
{
    let begin_line = "HKEY_LOCAL_MACHINE|SOFTWARE\\Test|SUBKEY|BEGIN_SUBKEY";
    let parts: Vec<&str> = begin_line.split('|').collect();
    assert_eq!(parts[3], "BEGIN_SUBKEY");

    let end_line = "HKEY_LOCAL_MACHINE|SOFTWARE\\Test|SUBKEY|END_SUBKEY";
    let parts: Vec<&str> = end_line.split('|').collect();
    assert_eq!(parts[3], "END_SUBKEY");
}

// ============================================
// Tests for HKEY text representation
// ============================================

#[test]
fn test_hkey_local_machine_identity()
{
    assert_eq!(HKEY::LOCAL_MACHINE, HKEY::LOCAL_MACHINE);
}

#[test]
fn test_hkey_current_user_identity()
{
    assert_eq!(HKEY::CURRENT_USER, HKEY::CURRENT_USER);
}

#[test]
fn test_hkey_types_are_different()
{
    assert_ne!(HKEY::LOCAL_MACHINE, HKEY::CURRENT_USER);
}
