// Windows Update tweaks

use super::Tweak;

pub static UPDATE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "defer_windows_upgrades",
                category: "updates",
                name: "Defer Windows Upgrades",
                description: "Defers Windows feature updates and quality updates.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgrade", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgradePeriod", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpdatePeriod", 0),
                ],
        },
        crate::tweak! {
                id: "disable_auto_update",
                category: "updates",
                name: "Disable Automatic Updates",
                description: "Disables automatic Windows updates via Automatic Updates (AU) policy.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU", "NoAutoUpdate", 1),
                ],
        },
        crate::tweak! {
                id: "disable_delivery_optimization",
                category: "updates",
                name: "Disable Delivery Optimization",
                description: "Disables Windows Update Delivery Optimization (P2P updates).",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config", "DODownloadMode", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization", "DODownloadMode", 0),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization", "SystemSettingsDownloadMode", 0),
                ],
        },
        crate::tweak! {
                id: "disable_driver_searching",
                category: "updates",
                name: "Disable Driver Searching",
                description: "Disables automatic searching for driver updates on the internet.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching", "SearchOrderConfig", 0),
                ],
        },
        crate::tweak! {
        id: "disable_store_auto_update",
        category: "updates",
        name: "Disable Store Auto-Update",
        description: "Disables automatic updates for Windows Store apps.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate", "AutoDownload", 2),
        ],
        },
        crate::tweak! {
                id: "prevent_device_metadata",
                category: "updates",
                name: "Prevent Device Metadata from Network",
                description: "Prevents Windows from downloading device metadata from the internet.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata", "PreventDeviceMetadataFromNetwork", 1),
                ],
        },
        crate::tweak! {
                id: "hide_insider_page",
                category: "updates",
                name: "Hide Windows Insider Program Page",
                description: "Hides the Windows Insider Program page from Windows Update settings.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\WindowsSelfHost\UI\Visibility", "HideInsiderPage", 1),
                ],
        },
];
