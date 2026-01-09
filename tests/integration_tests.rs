//! Integration tests for W11Boost registry operations.
//! Requires elevation to run.

use anyhow::Result;
use winsafe::co::{self, ERROR};
use winsafe::{HKEY, RegistryValue};

// Test subkey - isolated from real app data
const TEST_SUBKEY: &str = r"SOFTWARE\W11BoostIntegrationTests";

// Helper: read a DWORD from registry
fn read_dword(hkey: &HKEY, subkey: &str, value_name: &str) -> Result<Option<u32>>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(RegistryValue::Dword(v)) => Ok(Some(v)),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(None),
                Err(e) => Err(anyhow::anyhow!("Failed to read DWORD: {e}")),
                _ => Err(anyhow::anyhow!("Unexpected registry value type")),
        }
}

// Helper: read a string from registry
fn read_string(hkey: &HKEY, subkey: &str, value_name: &str) -> Result<Option<String>>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_SZ) {
                Ok(RegistryValue::Sz(v)) => Ok(Some(v)),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(None),
                Err(e) => Err(anyhow::anyhow!("Failed to read string: {e}")),
                _ => Err(anyhow::anyhow!("Unexpected registry value type")),
        }
}

// Helper: check if a subkey exists
fn subkey_exists(hkey: &HKEY, subkey: &str) -> bool
{
        hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::READ)
                .is_ok()
}

// Helper: create a test subkey with a value
fn create_test_key(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<()>
{
        let (key, _) = hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)?;
        key.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))?;
        Ok(())
}

// Helper: delete test subkey tree
fn cleanup_test_key(hkey: &HKEY, subkey: &str)
{
        let _ = hkey.RegDeleteTree(Some(subkey));
}

// ============================================
// set_dword tests
// ============================================

#[test]
fn test_set_dword_creates_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetDwordCreate");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let result = w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "TestValue", 42);
        assert!(result.is_ok(), "set_dword failed: {:?}", result.err());

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "TestValue").unwrap();
        assert_eq!(value, Some(42), "Value should be 42");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_dword_overwrites_existing()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetDwordOverwrite");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        // Create initial value
        create_test_key(&HKEY::CURRENT_USER, &subkey, "TestValue", 100).unwrap();

        // Overwrite with new value
        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "TestValue", 200).unwrap();

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "TestValue").unwrap();
        assert_eq!(value, Some(200), "Value should be updated to 200");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_dword_creates_nested_subkey()
{
        let subkey = format!(r"{TEST_SUBKEY}\Deep\Nested\Path\SetDword");
        cleanup_test_key(&HKEY::CURRENT_USER, TEST_SUBKEY);

        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "DeepValue", 999).unwrap();

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "DeepValue").unwrap();
        assert_eq!(value, Some(999));

        cleanup_test_key(&HKEY::CURRENT_USER, TEST_SUBKEY);
}

// ============================================
// set_string tests
// ============================================

#[test]
fn test_set_string_creates_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetStringCreate");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let result = w11boost::set_string(&HKEY::CURRENT_USER, &subkey, "TestString", "Hello World");
        assert!(result.is_ok(), "set_string failed: {:?}", result.err());

        let value = read_string(&HKEY::CURRENT_USER, &subkey, "TestString").unwrap();
        assert_eq!(value, Some("Hello World".to_string()));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_string_with_special_chars()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetStringSpecial");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let special = r"C:\Path\With\Backslashes & Special <chars>";
        w11boost::set_string(&HKEY::CURRENT_USER, &subkey, "SpecialPath", special).unwrap();

        let value = read_string(&HKEY::CURRENT_USER, &subkey, "SpecialPath").unwrap();
        assert_eq!(value, Some(special.to_string()));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_string_empty_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetStringEmpty");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        w11boost::set_string(&HKEY::CURRENT_USER, &subkey, "EmptyString", "").unwrap();

        let value = read_string(&HKEY::CURRENT_USER, &subkey, "EmptyString").unwrap();
        assert_eq!(value, Some(String::new()));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

// Helper: read an expandable string from registry
fn read_expand_sz(hkey: &HKEY, subkey: &str, value_name: &str) -> Result<Option<String>>
{
        match hkey.RegGetValue(
                Some(subkey),
                Some(value_name),
                co::RRF::RT_REG_EXPAND_SZ | co::RRF::NOEXPAND,
        ) {
                Ok(RegistryValue::ExpandSz(v)) => Ok(Some(v)),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(None),
                Err(e) => Err(anyhow::anyhow!("Failed to read EXPAND_SZ: {e}")),
                _ => Err(anyhow::anyhow!("Unexpected registry value type")),
        }
}

// Helper: read binary from registry
fn read_binary(hkey: &HKEY, subkey: &str, value_name: &str) -> Result<Option<Vec<u8>>>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_BINARY) {
                Ok(RegistryValue::Binary(v)) => Ok(Some(v)),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(None),
                Err(e) => Err(anyhow::anyhow!("Failed to read binary: {e}")),
                _ => Err(anyhow::anyhow!("Unexpected registry value type")),
        }
}

// ============================================
// set_expand_sz tests
// ============================================

#[test]
fn test_set_expand_sz_creates_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetExpandSzCreate");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let result = w11boost::set_expand_sz(&HKEY::CURRENT_USER, &subkey, "TestExpand", "%SystemRoot%\\System32");
        assert!(result.is_ok(), "set_expand_sz failed: {:?}", result.err());

        let value = read_expand_sz(&HKEY::CURRENT_USER, &subkey, "TestExpand").unwrap();
        assert_eq!(value, Some("%SystemRoot%\\System32".to_string()));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_expand_sz_creates_nested_subkey()
{
        let subkey = format!(r"{TEST_SUBKEY}\Nested\ExpandSz");
        cleanup_test_key(&HKEY::CURRENT_USER, TEST_SUBKEY);

        w11boost::set_expand_sz(&HKEY::CURRENT_USER, &subkey, "DeepExpand", "%ProgramFiles%").unwrap();

        let value = read_expand_sz(&HKEY::CURRENT_USER, &subkey, "DeepExpand").unwrap();
        assert_eq!(value, Some("%ProgramFiles%".to_string()));

        cleanup_test_key(&HKEY::CURRENT_USER, TEST_SUBKEY);
}

// ============================================
// set_binary tests
// ============================================

#[test]
fn test_set_binary_creates_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetBinaryCreate");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let data = vec![0x1, 0x2, 0x3, 0x4, 0x0, 0xFF];
        let result = w11boost::set_binary(&HKEY::CURRENT_USER, &subkey, "TestBinary", &data);
        assert!(result.is_ok(), "set_binary failed: {:?}", result.err());

        let value = read_binary(&HKEY::CURRENT_USER, &subkey, "TestBinary").unwrap();
        assert_eq!(value, Some(data));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_binary_empty()
{
        let subkey = format!(r"{TEST_SUBKEY}\SetBinaryEmpty");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let data = vec![];
        w11boost::set_binary(&HKEY::CURRENT_USER, &subkey, "EmptyBinary", &data).unwrap();

        let value = read_binary(&HKEY::CURRENT_USER, &subkey, "EmptyBinary").unwrap();
        assert_eq!(value, Some(data));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

// ============================================
// check_dword tests
// ============================================

#[test]
fn test_check_dword_returns_true_when_matches()
{
        let subkey = format!(r"{TEST_SUBKEY}\CheckDwordMatch");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        create_test_key(&HKEY::CURRENT_USER, &subkey, "CheckMe", 123).unwrap();

        let result = w11boost::check_dword(&HKEY::CURRENT_USER, &subkey, "CheckMe", 123).unwrap();
        assert!(result, "Should return true when value matches");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_check_dword_returns_false_when_different()
{
        let subkey = format!(r"{TEST_SUBKEY}\CheckDwordDiff");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        create_test_key(&HKEY::CURRENT_USER, &subkey, "CheckMe", 123).unwrap();

        let result = w11boost::check_dword(&HKEY::CURRENT_USER, &subkey, "CheckMe", 456).unwrap();
        assert!(!result, "Should return false when value differs");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_check_dword_returns_false_when_missing()
{
        let subkey = format!(r"{TEST_SUBKEY}\CheckDwordMissing");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let result = w11boost::check_dword(&HKEY::CURRENT_USER, &subkey, "NonExistent", 0).unwrap();
        assert!(!result, "Should return false when key doesn't exist");
}

// ============================================
// delete_value tests
// ============================================

#[test]
fn test_delete_value_removes_existing()
{
        let subkey = format!(r"{TEST_SUBKEY}\DeleteValue");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        // Create value then delete it
        create_test_key(&HKEY::CURRENT_USER, &subkey, "ToDelete", 1).unwrap();
        assert!(read_dword(&HKEY::CURRENT_USER, &subkey, "ToDelete").unwrap().is_some());

        w11boost::delete_value(&HKEY::CURRENT_USER, &subkey, "ToDelete").unwrap();

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "ToDelete").unwrap();
        assert!(value.is_none(), "Value should be deleted");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_delete_value_ok_when_missing()
{
        let subkey = format!(r"{TEST_SUBKEY}\DeleteValueMissing");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        // Should not error when value doesn't exist
        let result = w11boost::delete_value(&HKEY::CURRENT_USER, &subkey, "NonExistent");
        assert!(result.is_ok(), "Should succeed when value doesn't exist");
}

#[test]
fn test_delete_value_preserves_other_values()
{
        let subkey = format!(r"{TEST_SUBKEY}\DeleteValuePreserve");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        // Create two values
        create_test_key(&HKEY::CURRENT_USER, &subkey, "Keep", 1).unwrap();
        let (key, _) = HKEY::CURRENT_USER
                .RegCreateKeyEx(&subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .unwrap();
        key.RegSetValueEx(Some("Delete"), RegistryValue::Dword(2)).unwrap();

        // Delete one
        w11boost::delete_value(&HKEY::CURRENT_USER, &subkey, "Delete").unwrap();

        // Other should remain
        let kept = read_dword(&HKEY::CURRENT_USER, &subkey, "Keep").unwrap();
        assert_eq!(kept, Some(1), "Other value should be preserved");

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

// ============================================
// remove_subkey tests
// ============================================

#[test]
fn test_remove_subkey_deletes_key()
{
        let subkey = format!(r"{TEST_SUBKEY}\RemoveSubkey");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        create_test_key(&HKEY::CURRENT_USER, &subkey, "Value", 1).unwrap();
        assert!(subkey_exists(&HKEY::CURRENT_USER, &subkey));

        w11boost::remove_subkey(&HKEY::CURRENT_USER, &subkey).unwrap();

        assert!(!subkey_exists(&HKEY::CURRENT_USER, &subkey), "Subkey should be deleted");
}

#[test]
fn test_remove_subkey_deletes_nested()
{
        let parent = format!(r"{TEST_SUBKEY}\RemoveNested");
        let child = format!(r"{parent}\Child");
        let grandchild = format!(r"{child}\Grandchild");
        cleanup_test_key(&HKEY::CURRENT_USER, &parent);

        // Create nested structure
        create_test_key(&HKEY::CURRENT_USER, &grandchild, "Deep", 1).unwrap();
        assert!(subkey_exists(&HKEY::CURRENT_USER, &grandchild));

        // Remove parent deletes all descendants
        w11boost::remove_subkey(&HKEY::CURRENT_USER, &parent).unwrap();

        assert!(!subkey_exists(&HKEY::CURRENT_USER, &parent));
        assert!(!subkey_exists(&HKEY::CURRENT_USER, &child));
        assert!(!subkey_exists(&HKEY::CURRENT_USER, &grandchild));
}

#[test]
fn test_remove_subkey_ok_when_missing()
{
        let subkey = format!(r"{TEST_SUBKEY}\RemoveNonExistent");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        let result = w11boost::remove_subkey(&HKEY::CURRENT_USER, &subkey);
        assert!(result.is_ok(), "Should succeed when subkey doesn't exist");
}

// ============================================
// Edge cases and error handling
// ============================================

#[test]
fn test_set_dword_zero_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\ZeroValue");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "Zero", 0).unwrap();

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "Zero").unwrap();
        assert_eq!(value, Some(0));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_set_dword_max_value()
{
        let subkey = format!(r"{TEST_SUBKEY}\MaxValue");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "Max", u32::MAX).unwrap();

        let value = read_dword(&HKEY::CURRENT_USER, &subkey, "Max").unwrap();
        assert_eq!(value, Some(u32::MAX));

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

#[test]
fn test_multiple_values_same_key()
{
        let subkey = format!(r"{TEST_SUBKEY}\MultipleValues");
        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);

        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "Value1", 1).unwrap();
        w11boost::set_dword(&HKEY::CURRENT_USER, &subkey, "Value2", 2).unwrap();
        w11boost::set_string(&HKEY::CURRENT_USER, &subkey, "Value3", "three").unwrap();

        assert_eq!(read_dword(&HKEY::CURRENT_USER, &subkey, "Value1").unwrap(), Some(1));
        assert_eq!(read_dword(&HKEY::CURRENT_USER, &subkey, "Value2").unwrap(), Some(2));
        assert_eq!(
                read_string(&HKEY::CURRENT_USER, &subkey, "Value3").unwrap(),
                Some("three".to_string())
        );

        cleanup_test_key(&HKEY::CURRENT_USER, &subkey);
}

// ============================================
// HKLM tests (require admin)
// ============================================

#[test]
fn test_set_dword_hklm()
{
        let subkey = format!(r"{TEST_SUBKEY}\HKLM");
        cleanup_test_key(&HKEY::LOCAL_MACHINE, &subkey);

        let result = w11boost::set_dword(&HKEY::LOCAL_MACHINE, &subkey, "AdminValue", 777);
        assert!(result.is_ok(), "HKLM write failed (requires admin): {:?}", result.err());

        let value = read_dword(&HKEY::LOCAL_MACHINE, &subkey, "AdminValue").unwrap();
        assert_eq!(value, Some(777));

        cleanup_test_key(&HKEY::LOCAL_MACHINE, &subkey);
}

#[test]
fn test_remove_subkey_hklm()
{
        let subkey = format!(r"{TEST_SUBKEY}\HKLMRemove");
        cleanup_test_key(&HKEY::LOCAL_MACHINE, &subkey);

        w11boost::set_dword(&HKEY::LOCAL_MACHINE, &subkey, "ToRemove", 1).unwrap();
        w11boost::remove_subkey(&HKEY::LOCAL_MACHINE, &subkey).unwrap();

        assert!(!subkey_exists(&HKEY::LOCAL_MACHINE, &subkey));
}
