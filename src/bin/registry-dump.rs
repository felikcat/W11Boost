//! Registry verification tool for W11Boost
//! Dumps all registry values W11Boost modifies and compares against Windows defaults.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use winsafe::co::{self, KNOWNFOLDERID};
use winsafe::{GetLocalTime, HKEY, RegistryValue, SHGetKnownFolderPath};

use w11boost::gui::tweaks::{RegistryValue as TweakValue, get_all_tweaks};

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

fn main()
{
        let mut results = Results::default();

        println!("W11Boost Registry Verification\nChecking registry entries...\n");

        let tweaks = get_all_tweaks();

        for tweak in tweaks {
                for op in tweak.enabled_ops {
                        if op.subkey.is_empty() {
                                continue;
                        }

                        let hkey = get_hkey(op.hkey);
                        let path = format!("{}\\{}\\{}", op.hkey, op.subkey, op.value_name);

                        match &op.stock_value {
                                TweakValue::DeleteKey => {
                                        if subkey_exists(&hkey, op.subkey) {
                                                results.add(
                                                        tweak.category,
                                                        false,
                                                        format!("FAIL {}  |  Expected: Subkey not present", path),
                                                );
                                        } else {
                                                results.add(
                                                        tweak.category,
                                                        true,
                                                        format!("OK   {}  |  Not present (expected)", path),
                                                );
                                        }
                                }
                                TweakValue::Delete => {
                                        if value_exists(&hkey, op.subkey, op.value_name) {
                                                let current = read_dword(&hkey, op.subkey, op.value_name)
                                                        .map(|v| v.to_string())
                                                        .or_else(|| {
                                                                read_string(&hkey, op.subkey, op.value_name)
                                                                        .map(|s| format!("\"{}\"", s))
                                                        })
                                                        .unwrap_or_else(|| "exists".to_string());
                                                results.add(
                                                        tweak.category,
                                                        false,
                                                        format!("FAIL {}  |  Current: {}  |  Expected: Value not present", path, current),
                                                );
                                        } else {
                                                results.add(
                                                        tweak.category,
                                                        true,
                                                        format!("OK   {}  |  Not present (expected)", path),
                                                );
                                        }
                                }
                                TweakValue::Dword(expected) => {
                                        match read_dword(&hkey, op.subkey, op.value_name) {
                                                Some(current) if current == *expected => {
                                                        results.add(
                                                                tweak.category,
                                                                true,
                                                                format!(
                                                                        "OK   {}  |  Value: {} (matches stock)",
                                                                        path, current
                                                                ),
                                                        );
                                                }
                                                Some(current) => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Current: {}  |  Expected: {} (stock)", path, current, expected),
                                                        );
                                                }
                                                None => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Not found  |  Expected: {} (stock)", path, expected),
                                                        );
                                                }
                                        }
                                }
                                TweakValue::String(expected) => {
                                        match read_string(&hkey, op.subkey, op.value_name) {
                                                Some(current) if current == *expected => {
                                                        results.add(
                                                                tweak.category,
                                                                true,
                                                                format!(
                                                                        "OK   {}  |  Value: \"{}\" (matches stock)",
                                                                        path, current
                                                                ),
                                                        );
                                                }
                                                Some(current) => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Current: \"{}\"  |  Expected: \"{}\" (stock)", path, current, expected),
                                                        );
                                                }
                                                None => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Not found  |  Expected: \"{}\" (stock)", path, expected),
                                                        );
                                                }
                                        }
                                }
                                TweakValue::ExpandSz(expected) => {
                                        match read_expand_sz(&hkey, op.subkey, op.value_name) {
                                                Some(current) if current == *expected => {
                                                        results.add(
                                                                tweak.category,
                                                                true,
                                                                format!(
                                                                        "OK   {}  |  Value: \"{}\" (matches stock)",
                                                                        path, current
                                                                ),
                                                        );
                                                }
                                                Some(current) => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Current: \"{}\"  |  Expected: \"{}\" (stock)", path, current, expected),
                                                        );
                                                }
                                                None => {
                                                        results.add(
                                                                tweak.category,
                                                                false,
                                                                format!("FAIL {}  |  Not found  |  Expected: \"{}\" (stock)", path, expected),
                                                        );
                                                }
                                        }
                                }
                                TweakValue::Binary(expected) => {
                                        match read_binary(&hkey, op.subkey, op.value_name) {
                                                Some(current) if current == *expected => {
                                                        results.add(
                                                                tweak.category,
                                                                true,
                                                                format!(
                                                                        "OK   {}  |  Value: {:?} (matches stock)",
                                                                        path, current
                                                                ),
                                                        );
                                                }
                                                Some(current) => {
                                                        results.add(
                                                        tweak.category,
                                                        false,
                                                        format!("FAIL {}  |  Current: {:?}  |  Expected: {:?} (stock)", path, current, expected),
                                                );
                                                }
                                                None => {
                                                        results.add(
                                                        tweak.category,
                                                        false,
                                                        format!("FAIL {}  |  Not found  |  Expected: {:?} (stock)", path, expected),
                                                );
                                                }
                                        }
                                }
                        }
                }
        }

        // Generate report
        let documents = get_documents_path();
        let timestamp = get_timestamp();
        let log_path = documents.join(format!("registry-dump-{}.log", timestamp));

        let total = results.ok + results.fail;

        // Build report content
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

        // Group by category
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

        // Write to file
        match File::create(&log_path) {
                Ok(mut file) => {
                        if let Err(e) = file.write_all(report.as_bytes()) {
                                eprintln!("Failed to write report: {}", e);
                        }
                }
                Err(e) => {
                        eprintln!("Failed to create log file: {}", e);
                }
        }

        // Print summary
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
