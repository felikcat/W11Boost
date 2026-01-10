// Windows Update tweaks

use super::{RegistryValue, Tweak, TweakEffect};

pub static UPDATE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_updates_aggressive",
                category: "updates",
                name: "Disable Windows Updates (Aggressive)",
                description: "Aggressively disables Windows Updates by redirecting to a non-existent WSUS server.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "DoNotConnectToWindowsUpdateInternetLocations", 1),
                        crate::reg_str!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "WUServer", "localserver.localdomain.wsus", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "WUStatusServer", "localserver.localdomain.wsus", RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate\AU", "UseWUServer", 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "DoNotConnectToWindowsUpdateInternetLocations", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "WUServer", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate", "WUStatusServer", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"Software\Policies\Microsoft\Windows\WindowsUpdate\AU", "UseWUServer", RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "defer_windows_upgrades",
                category: "updates",
                name: "Defer Windows Upgrades",
                description: "Defers Windows feature updates and quality updates.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgrade", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgradePeriod", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpdatePeriod", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgrade", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpgradePeriod", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "DeferUpdatePeriod", RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_driver_searching",
                category: "updates",
                name: "Disable Driver Searching",
                description: "Disables automatic searching for driver updates on the internet.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching", "SearchOrderConfig", 0, 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching", "SearchOrderConfig", 1, 1),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "prevent_device_metadata",
                category: "updates",
                name: "Prevent Device Metadata from Network",
                description: "Prevents Windows from downloading device metadata from the internet.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata", "PreventDeviceMetadataFromNetwork", 1, 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata", "PreventDeviceMetadataFromNetwork", 0, 0),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_wuauserv",
                category: "updates",
                name: "Disable Windows Update Service",
                description: "Completely disables the Windows Update service (wuauserv).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\wuauserv", "Start", 4, 3),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Services\7971f918-a847-4430-9279-4a52d1efe18d", "RegisteredWithAU", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\wuauserv", "Start", 2, 3),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Services\7971f918-a847-4430-9279-4a52d1efe18d", "RegisteredWithAU", 1, RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_auto_update",
                category: "updates",
                name: "Disable Automatic Updates",
                description: "Disables automatic Windows updates via Automatic Updates (AU) policy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU", "NoAutoUpdate", 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU", "NoAutoUpdate", RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_delivery_optimization",
                category: "updates",
                name: "Disable Delivery Optimization",
                description: "Disables Windows Update Delivery Optimization (P2P updates).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config", "DODownloadMode", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization", "DODownloadMode", 0),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization", "SystemSettingsDownloadMode", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config", "DODownloadMode", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization", "DODownloadMode", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization", "SystemSettingsDownloadMode", RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_store_auto_update",
                category: "updates",
                name: "Disable Store Auto-Update",
                description: "Disables automatic updates for Windows Store apps.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate", "AutoDownload", 2),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate", "AutoDownload", RegistryValue::Delete),
                ])
        },
];
