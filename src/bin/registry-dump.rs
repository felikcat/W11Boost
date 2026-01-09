//! Registry verification tool for W11Boost
//! Dumps all registry values W11Boost modifies and compares against Windows defaults.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use winsafe::co::{self, KNOWNFOLDERID};
use winsafe::{GetLocalTime, HKEY, RegistryValue, SHGetKnownFolderPath};

use w11boost::gui::tweaks::{RegistryOp, RegistryValue as TweakValue, get_all_tweaks};

#[derive(Default)]
struct Results
{
        ok: u32,
        fail: u32,
        entries: Vec<(String, bool, String)>, // (module, is_ok, message)
}

impl Results
{
        fn add(&mut self, module: &str, is_ok: bool, message: String)
        {
                if is_ok {
                        self.ok += 1;
                } else {
                        self.fail += 1;
                }
                self.entries.push((module.to_string(), is_ok, message));
        }
}

fn get_hkey(hkey_str: &str) -> HKEY
{
        if hkey_str == "HKLM" {
                HKEY::LOCAL_MACHINE
        } else {
                HKEY::CURRENT_USER
        }
}

fn read_dword(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<u32>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(RegistryValue::Dword(v)) => Some(v),
                _ => None,
        }
}

fn value_exists(hkey: &HKEY, subkey: &str, value_name: &str) -> bool
{
        // Try to read as DWORD first
        if hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD)
                .is_ok()
        {
                return true;
        }
        // Try as string
        if hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_SZ)
                .is_ok()
        {
                return true;
        }
        // Try as ExpandSz
        if hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_EXPAND_SZ)
                .is_ok()
        {
                return true;
        }
        false
}

fn subkey_exists(hkey: &HKEY, subkey: &str) -> bool
{
        hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::READ)
                .is_ok()
}

fn read_string(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<String>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_SZ) {
                Ok(RegistryValue::Sz(s)) => Some(s),
                _ => None,
        }
}

fn read_expand_sz(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<String>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_EXPAND_SZ) {
                Ok(RegistryValue::ExpandSz(s)) => Some(s),
                _ => None,
        }
}

fn read_binary(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<Vec<u8>>
{
        match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_BINARY) {
                Ok(RegistryValue::Binary(v)) => Some(v),
                _ => None,
        }
}

fn get_documents_path() -> PathBuf
{
        SHGetKnownFolderPath(&KNOWNFOLDERID::Documents, co::KF::DEFAULT, None)
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
}

fn check_delete_key(category: &str, hkey: &HKEY, subkey: &str, path: &str, results: &mut Results)
{
        if subkey_exists(hkey, subkey) {
                results.add(
                        category,
                        false,
                        format!("FAIL {}  |  Expected: Subkey not present", path),
                );
        } else {
                results.add(category, true, format!("OK   {}  |  Not present (expected)", path));
        }
}

fn check_delete_value(category: &str, hkey: &HKEY, subkey: &str, value_name: &str, path: &str, results: &mut Results)
{
        if value_exists(hkey, subkey, value_name) {
                let current = read_dword(hkey, subkey, value_name)
                        .map(|v| v.to_string())
                        .or_else(|| read_string(hkey, subkey, value_name).map(|s| format!("\"{}\"", s)))
                        .unwrap_or_else(|| "exists".to_string());
                results.add(
                        category,
                        false,
                        format!("FAIL {}  |  Current: {}  |  Expected: Value not present", path, current),
                );
        } else {
                results.add(category, true, format!("OK   {}  |  Not present (expected)", path));
        }
}

fn check_dword(
        category: &str,
        hkey: &HKEY,
        subkey: &str,
        value_name: &str,
        expected: u32,
        path: &str,
        results: &mut Results,
)
{
        match read_dword(hkey, subkey, value_name) {
                Some(current) if current == expected => {
                        results.add(
                                category,
                                true,
                                format!("OK   {}  |  Value: {} (matches stock)", path, current),
                        );
                }
                Some(current) => {
                        results.add(
                                category,
                                false,
                                format!(
                                        "FAIL {}  |  Current: {}  |  Expected: {} (stock)",
                                        path, current, expected
                                ),
                        );
                }
                None => {
                        results.add(
                                category,
                                false,
                                format!("FAIL {}  |  Not found  |  Expected: {} (stock)", path, expected),
                        );
                }
        }
}

fn check_string(
        category: &str,
        hkey: &HKEY,
        subkey: &str,
        value_name: &str,
        expected: &str,
        path: &str,
        results: &mut Results,
)
{
        match read_string(hkey, subkey, value_name) {
                Some(current) if current == expected => {
                        results.add(
                                category,
                                true,
                                format!("OK   {}  |  Value: \"{}\" (matches stock)", path, current),
                        );
                }
                Some(current) => {
                        results.add(
                                category,
                                false,
                                format!(
                                        "FAIL {}  |  Current: \"{}\"  |  Expected: \"{}\" (stock)",
                                        path, current, expected
                                ),
                        );
                }
                None => {
                        results.add(
                                category,
                                false,
                                format!("FAIL {}  |  Not found  |  Expected: \"{}\" (stock)", path, expected),
                        );
                }
        }
}

fn check_expand_sz(
        category: &str,
        hkey: &HKEY,
        subkey: &str,
        value_name: &str,
        expected: &str,
        path: &str,
        results: &mut Results,
)
{
        match read_expand_sz(hkey, subkey, value_name) {
                Some(current) if current == expected => {
                        results.add(
                                category,
                                true,
                                format!("OK   {}  |  Value: \"{}\" (matches stock)", path, current),
                        );
                }
                Some(current) => {
                        results.add(
                                category,
                                false,
                                format!(
                                        "FAIL {}  |  Current: \"{}\"  |  Expected: \"{}\" (stock)",
                                        path, current, expected
                                ),
                        );
                }
                None => {
                        results.add(
                                category,
                                false,
                                format!("FAIL {}  |  Not found  |  Expected: \"{}\" (stock)", path, expected),
                        );
                }
        }
}

fn check_binary(
        category: &str,
        hkey: &HKEY,
        subkey: &str,
        value_name: &str,
        expected: &[u8],
        path: &str,
        results: &mut Results,
)
{
        match read_binary(hkey, subkey, value_name) {
                Some(current) if current == expected => {
                        results.add(
                                category,
                                true,
                                format!("OK   {}  |  Value: {:?} (matches stock)", path, current),
                        );
                }
                Some(current) => {
                        results.add(
                                category,
                                false,
                                format!(
                                        "FAIL {}  |  Current: {:?}  |  Expected: {:?} (stock)",
                                        path, current, expected
                                ),
                        );
                }
                None => {
                        results.add(
                                category,
                                false,
                                format!("FAIL {}  |  Not found  |  Expected: {:?} (stock)", path, expected),
                        );
                }
        }
}

fn check_operation(category: &str, op: &RegistryOp, results: &mut Results)
{
        if op.subkey.is_empty() {
                return;
        }

        let hkey = get_hkey(op.hkey);
        let path = format!("{}\\{}\\{}", op.hkey, op.subkey, op.value_name);

        match &op.stock_value {
                TweakValue::DeleteKey => check_delete_key(category, &hkey, op.subkey, &path, results),
                TweakValue::Delete => check_delete_value(category, &hkey, op.subkey, op.value_name, &path, results),
                TweakValue::Dword(expected) => {
                        check_dword(category, &hkey, op.subkey, op.value_name, *expected, &path, results)
                }
                TweakValue::String(expected) => {
                        check_string(category, &hkey, op.subkey, op.value_name, expected, &path, results)
                }
                TweakValue::ExpandSz(expected) => {
                        check_expand_sz(category, &hkey, op.subkey, op.value_name, expected, &path, results)
                }
                TweakValue::Binary(expected) => {
                        check_binary(category, &hkey, op.subkey, op.value_name, expected, &path, results)
                }
        }
}

fn generate_report(results: &Results, timestamp: &str) -> String
{
        let total = results.ok + results.fail;
        let mut report = String::new();

        report.push_str("==============================================\n");
        report.push_str("W11Boost Registry Verification Report\n");
        report.push_str(&format!("Generated: {}\n", timestamp.replace("_", " ")));
        report.push_str("==============================================\n\n");
        report.push_str(&format!(
                "SUMMARY:\n  Total Ops Checked: {}  |  Stock: {}  |  Modified: {}\n\n",
                total, results.ok, results.fail
        ));

        if results.fail == 0 {
                report.push_str("STATUS: All registry values match stock Windows defaults.\n");
                report.push_str("        W11Boost has not been applied (or was successfully removed).\n");
        } else {
                report.push_str("STATUS: Some registry values differ from stock Windows defaults.\n");
                report.push_str("        W11Boost tweaks appear to be applied.\n");
        }

        report.push_str("\n==============================================\n\n");

        let mut current_cat = String::new();
        for (cat, _, message) in &results.entries {
                if cat != &current_cat {
                        if !current_cat.is_empty() {
                                report.push('\n');
                        }
                        report.push_str(&format!("[{}]\n", cat));
                        current_cat = cat.clone();
                }
                report.push_str(&format!("  {}\n", message));
        }

        report.push_str("\n==============================================\n");
        report.push_str("END OF REPORT\n");
        report.push_str("==============================================\n");

        report
}

fn save_report(path: &std::path::Path, content: &str)
{
        match File::create(path) {
                Ok(mut file) => {
                        if let Err(e) = file.write_all(content.as_bytes()) {
                                eprintln!("Failed to write report: {}", e);
                        }
                }
                Err(e) => {
                        eprintln!("Failed to create log file: {}", e);
                }
        }
}

fn print_summary(results: &Results, log_path: &std::path::Path)
{
        let total = results.ok + results.fail;

        println!("SUMMARY:");
        println!(
                "  Total Ops Checked: {}  |  Stock: {}  |  Modified: {}",
                total, results.ok, results.fail
        );
        println!();

        if results.fail == 0 {
                println!("All registry values match stock Windows defaults.");
        } else {
                println!("{} values differ from stock Windows defaults.", results.fail);
        }

        println!();
        println!("Report saved to: {}", log_path.display());
}

fn main()
{
        println!("W11Boost Registry Verification\nChecking registry entries...\n");

        let mut results = Results::default();
        let tweaks = get_all_tweaks();

        for tweak in tweaks {
                for op in tweak.enabled_ops {
                        check_operation(tweak.category, op, &mut results);
                }
        }

        let documents = get_documents_path();
        let timestamp = get_timestamp();
        let log_path = documents.join(format!("registry-dump-{}.log", timestamp));

        let report_content = generate_report(&results, &timestamp);
        save_report(&log_path, &report_content);
        print_summary(&results, &log_path);

        println!("\nPress Enter to exit...");
        let _ = std::io::stdin().read_line(&mut String::new());
}

fn get_timestamp() -> String
{
        let t = GetLocalTime();
        format!(
                "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
                t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute, t.wSecond
        )
}
