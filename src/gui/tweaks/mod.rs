// Tweak definitions and categories for W11Boost
// Each tweak is a toggleable option with registry/command implementation

mod accessibility;
mod appearance;
mod behavior;
mod boot;
mod context_menu;
mod debloat;
pub mod defaults;
mod desktop;
mod edge;
mod explorer;
mod forensics;
mod network;
mod online_data;
mod performance;
mod power;
mod privacy;
mod remove_ai;
mod security;
mod software;
mod sync;

pub mod bulk;
pub use bulk::BULK_OPERATIONS_TWEAKS;
mod tools;
mod updates;

use super::shared_state::WorkerContext;
use crate::common::{delete_value, remove_subkey, set_binary, set_dword, set_expand_sz, set_string};
use anyhow::Result;
use serde::Serialize;
use std::sync::Arc;
use winsafe::HKEY;

pub use accessibility::ACCESSIBILITY_TWEAKS;
pub use appearance::APPEARANCE_TWEAKS;
pub use behavior::BEHAVIOR_TWEAKS;
pub use boot::BOOT_TWEAKS;
pub use context_menu::CONTEXT_MENU_TWEAKS;
pub use debloat::DEBLOAT_TWEAKS;
pub use desktop::DESKTOP_TWEAKS;
pub use edge::EDGE_TWEAKS;
pub use explorer::EXPLORER_TWEAKS;
pub use forensics::FORENSICS_TWEAKS;
pub use network::NETWORK_TWEAKS;
pub use online_data::ONLINE_DATA_TWEAKS;
pub use performance::PERFORMANCE_TWEAKS;
pub use power::POWER_TWEAKS;
pub use privacy::PRIVACY_TWEAKS;
pub use remove_ai::REMOVE_AI_TWEAKS;

pub use security::SECURITY_TWEAKS;
pub use software::SOFTWARE_TWEAKS;
pub use sync::SYNC_TWEAKS;

pub use tools::TOOLS_TWEAKS;
pub use updates::UPDATE_TWEAKS;

/// Category for organizing tweaks
#[derive(Clone, Debug, Serialize)]
pub struct TweakCategory
{
        pub id: &'static str,
        pub name: &'static str,
        pub description: &'static str,
}

/// A single registry operation
#[derive(Clone, Debug, Serialize)]
pub struct RegistryOp
{
        pub hkey: &'static str, // "HKLM" or "HKCU"
        pub subkey: &'static str,
        pub value_name: &'static str,
        pub value: RegistryValue,
}

/// A single Group Policy operation (Machine config)
#[derive(Clone, Debug, Serialize)]
pub struct GpoOp
{
        pub subkey: &'static str,
        pub value_name: &'static str,
        pub value: RegistryValue,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum RegistryValue
{
        Dword(u32),
        String(&'static str),
        ExpandSz(&'static str),
        Binary(&'static [u8]),
        Delete,    // Value should not exist
        DeleteKey, // Entire subkey should not exist
}

impl RegistryValue
{
        pub fn to_string(&self) -> String
        {
                match self {
                        Self::Dword(val) => format!("DWORD:{val}"),
                        Self::String(val) => format!("SZ:\"{val}\""),
                        Self::ExpandSz(val) => format!("EXPAND_SZ:\"{val}\""),
                        Self::Binary(val) => format!("BINARY:{:?}", val),
                        Self::Delete => "DELETE_VALUE".to_string(),
                        Self::DeleteKey => "DELETE_KEY".to_string(),
                }
        }
}

/// Type alias for custom apply/revert functions
pub type TweakFn = fn(&Arc<WorkerContext>) -> Result<()>;

/// A single tweak that can be enabled/disabled
#[derive(Clone, Serialize)]
#[allow(dead_code)]
pub struct Tweak
{
        pub id: &'static str,
        pub category: &'static str,
        pub name: &'static str,
        pub description: &'static str,
        pub enabled_ops: &'static [RegistryOp],
        #[serde(skip)]
        pub custom_apply: Option<TweakFn>,
        pub sub_tweaks: &'static [&'static Tweak],
        pub has_custom_input: bool,
        pub default_text: Option<&'static str>,
        pub command: Option<&'static str>,
        pub gpo_ops: Option<&'static [GpoOp]>,
}

impl Tweak
{
        pub const DEFAULT: Self = Self {
                id: "",
                category: "",
                name: "",
                description: "",
                enabled_ops: &[],
                custom_apply: None,
                sub_tweaks: &[],
                has_custom_input: false,
                default_text: None,
                command: None,
                gpo_ops: None,
        };

        #[allow(clippy::cast_possible_truncation)]
        pub const fn op_count(&self) -> u32
        {
                self.enabled_ops.len() as u32
        }
}

#[macro_export]
macro_rules! tweak {
    ($($field:ident: $value:expr),* $(,)?) => {
        $crate::gui::tweaks::Tweak {
            $($field: $value,)*
            ..$crate::gui::tweaks::Tweak::DEFAULT
        }
    };
}

#[macro_export]
macro_rules! reg_dword {
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr) => {
                $crate::reg_dword!(
                        $hkey,
                        $subkey,
                        $name,
                        $val,
                        $crate::gui::tweaks::RegistryValue::Delete
                )
        };
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr, $stock:literal) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Dword($val),
                }
        };
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr, $stock:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Dword($val),
                }
        };
}

#[macro_export]
macro_rules! reg_str {
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr) => {
                $crate::reg_str!(
                        $hkey,
                        $subkey,
                        $name,
                        $val,
                        $crate::gui::tweaks::RegistryValue::Delete
                )
        };
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr, $stock:literal) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::String($val),
                }
        };
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr, $stock:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::String($val),
                }
        };
}

#[macro_export]
macro_rules! reg_binary {
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr) => {
                $crate::reg_binary!(
                        $hkey,
                        $subkey,
                        $name,
                        $val,
                        $crate::gui::tweaks::RegistryValue::Delete
                )
        };
        ($hkey:expr, $subkey:expr, $name:expr, $val:expr, $stock:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Binary($val),
                }
        };
}

#[macro_export]
macro_rules! reg_del {
        ($hkey:expr, $subkey:expr, $name:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Delete,
                }
        };
        ($hkey:expr, $subkey:expr, $name:expr, $stock:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Delete,
                }
        };
}

#[macro_export]
macro_rules! reg_del_key {
        ($hkey:expr, $subkey:expr, $name:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::DeleteKey,
                }
        };
        ($hkey:expr, $subkey:expr, $name:expr, $stock:expr) => {
                $crate::gui::tweaks::RegistryOp {
                        hkey: $hkey,
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::DeleteKey,
                }
        };
}

#[macro_export]
macro_rules! gpo_dword {
        ($subkey:expr, $name:expr, $val:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Dword($val),
                }
        };
        ($subkey:expr, $name:expr, $val:expr, $stock:literal) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Dword($val),
                }
        };
        ($subkey:expr, $name:expr, $val:expr, $stock:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Dword($val),
                }
        };
}

#[macro_export]
macro_rules! gpo_str {
        ($subkey:expr, $name:expr, $val:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::String($val),
                }
        };
        ($subkey:expr, $name:expr, $val:expr, $stock:literal) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::String($val),
                }
        };
        ($subkey:expr, $name:expr, $val:expr, $stock:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::String($val),
                }
        };
}

#[macro_export]
macro_rules! gpo_del {
        ($subkey:expr, $name:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Delete,
                }
        };
        ($subkey:expr, $name:expr, $stock:expr) => {
                $crate::gui::tweaks::GpoOp {
                        subkey: $subkey,
                        value_name: $name,
                        value: $crate::gui::tweaks::RegistryValue::Delete,
                }
        };
}

// Helper to get HKEY from string
fn get_hkey(hkey_str: &str) -> HKEY
{
        if hkey_str == "HKLM" {
                HKEY::LOCAL_MACHINE
        } else {
                HKEY::CURRENT_USER
        }
}

pub fn execute_registry_op(op: &RegistryOp, ctx: &Arc<WorkerContext>, prefix: &str) -> Result<()>
{
        let hkey = get_hkey(op.hkey);
        ctx.post_status(&format!("{} {}\\{}\\{}", prefix, op.hkey, op.subkey, op.value_name));

        match &op.value {
                RegistryValue::Dword(val) => set_dword(&hkey, op.subkey, op.value_name, *val)?,
                RegistryValue::String(val) => set_string(&hkey, op.subkey, op.value_name, val)?,
                RegistryValue::ExpandSz(val) => set_expand_sz(&hkey, op.subkey, op.value_name, val)?,
                RegistryValue::Binary(val) => set_binary(&hkey, op.subkey, op.value_name, val)?,
                RegistryValue::Delete => delete_value(&hkey, op.subkey, op.value_name)?,
                RegistryValue::DeleteKey => {
                        let _ = remove_subkey(&hkey, op.subkey);
                }
        }
        ctx.report_progress();
        Ok(())
}

/// Apply a single tweak
pub fn apply_tweak(tweak: &Tweak, ctx: &Arc<WorkerContext>) -> Result<()>
{
        if let Some(custom_fn) = tweak.custom_apply {
                return custom_fn(ctx);
        }

        for op in tweak.enabled_ops {
                execute_registry_op(op, ctx, "Applying")?;
        }

        Ok(())
}

/// Revert a single tweak
#[allow(dead_code)]

// ============================================================================
// CATEGORY DEFINITIONS
// ============================================================================

pub static CATEGORIES: &[TweakCategory] = &[
        TweakCategory {
                id: "accessibility",
                name: "Accessibility",
                description: "Accessibility features and settings",
        },
        TweakCategory {
                id: "appearance",
                name: "Appearance",
                description: "Visual customization and theme settings",
        },
        TweakCategory {
                id: "behavior",
                name: "Behavior",
                description: "Window behavior, Aero features, and input settings",
        },
        TweakCategory {
                id: "boot",
                name: "Boot & Logon",
                description: "Startup, login, and boot configuration",
        },
        TweakCategory {
                id: "bulk",
                name: "Bulk Operations",
                description: "Comprehensive telemetry blocking and system repair operations",
        },
        TweakCategory {
                id: "sync",
                name: "Cloud Sync",
                description: "Windows cloud sync, backup, and cross-device settings",
        },
        TweakCategory {
                id: "context_menu",
                name: "Context Menu",
                description: "Add or remove context menu items",
        },
        TweakCategory {
                id: "debloat",
                name: "Debloat Apps",
                description: "Remove pre-installed Windows apps and bloatware",
        },
        TweakCategory {
                id: "desktop",
                name: "Desktop & Taskbar",
                description: "Customize desktop behavior and taskbar appearance",
        },
        TweakCategory {
                id: "explorer",
                name: "File Explorer",
                description: "File Explorer behavior and view settings",
        },
        TweakCategory {
                id: "forensics",
                name: "Forensics & Local Data",
                description: "Minimize local forensic artifacts and usage tracking data stored on disk",
        },
        TweakCategory {
                id: "software",
                name: "Install Software",
                description: "Install recommended software via winget",
        },
        TweakCategory {
                id: "edge",
                name: "Microsoft Edge",
                description: "Microsoft Edge browser tweaks",
        },
        TweakCategory {
                id: "network",
                name: "Network",
                description: "Network and connectivity settings",
        },
        TweakCategory {
                id: "online_data",
                name: "Online Data Collection",
                description: "Control Microsoft online services and cloud data collection",
        },
        TweakCategory {
                id: "performance",
                name: "Performance",
                description: "Performance optimizations and power settings",
        },
        TweakCategory {
                id: "power",
                name: "Power & Sleep",
                description: "Sleep, hibernate, and power management settings",
        },
        TweakCategory {
                id: "privacy",
                name: "Privacy & Telemetry",
                description: "Control Windows data collection, telemetry, and privacy settings",
        },
        TweakCategory {
                id: "remove_ai",
                name: "Remove AI",
                description: "Disable Copilot, Recall, and other AI/NPU features",
        },
        TweakCategory {
                id: "security",
                name: "Security",
                description: "Windows Defender, SmartScreen, and UAC settings",
        },
        TweakCategory {
                id: "tools",
                name: "System Tools",
                description: "System utilities and maintenance settings",
        },
        TweakCategory {
                id: "updates",
                name: "Windows Update",
                description: "Control Windows Update behavior",
        },
];

const ALL_TWEAK_LISTS: &[&[Tweak]] = &[
        ACCESSIBILITY_TWEAKS,
        APPEARANCE_TWEAKS,
        BEHAVIOR_TWEAKS,
        BOOT_TWEAKS,
        BULK_OPERATIONS_TWEAKS,
        CONTEXT_MENU_TWEAKS,
        DEBLOAT_TWEAKS,
        DESKTOP_TWEAKS,
        EDGE_TWEAKS,
        EXPLORER_TWEAKS,
        FORENSICS_TWEAKS,
        NETWORK_TWEAKS,
        ONLINE_DATA_TWEAKS,
        PERFORMANCE_TWEAKS,
        POWER_TWEAKS,
        PRIVACY_TWEAKS,
        REMOVE_AI_TWEAKS,
        SECURITY_TWEAKS,
        SOFTWARE_TWEAKS,
        SYNC_TWEAKS,
        TOOLS_TWEAKS,
        UPDATE_TWEAKS,
];

/// Get all tweaks for a given category
pub fn get_tweaks_for_category(category_id: &str) -> Vec<&'static Tweak>
{
        ALL_TWEAK_LISTS
                .iter()
                .flat_map(|list| list.iter())
                .filter(|tweak| tweak.category == category_id)
                .collect()
}

fn collect_recursive(tweak: &'static Tweak, out: &mut Vec<&'static Tweak>)
{
        out.push(tweak);
        for sub in tweak.sub_tweaks {
                collect_recursive(sub, out);
        }
}

/// Get all tweaks (recursively including sub-tweaks)
pub fn get_all_tweaks() -> Vec<&'static Tweak>
{
        let mut all = Vec::with_capacity(500);
        for list in ALL_TWEAK_LISTS {
                for tweak in *list {
                        collect_recursive(tweak, &mut all);
                }
        }
        all
}
