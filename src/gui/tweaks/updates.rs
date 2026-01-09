// Windows Update tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static UPDATE_TWEAKS: &[Tweak] = &[
        crate::tweak! {

                id: "disable_updates_aggressive",

                category: "updates",

                name: "Disable Windows Updates (Aggressive)",

                description: "Aggressively disables Windows Updates by redirecting to a non-existent WSUS server.",

                effect: TweakEffect::Restart,

                enabled_ops: &[

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DoNotConnectToWindowsUpdateInternetLocations",

                                value: RegistryValue::Dword(1),

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "WUServer",

                                value: RegistryValue::String("localserver.localdomain.wsus"),

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "WUStatusServer",

                                value: RegistryValue::String("localserver.localdomain.wsus"),

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate\AU",

                                value_name: "UseWUServer",

                                value: RegistryValue::Dword(1),

                                stock_value: RegistryValue::Delete
        },

                ],

                disabled_ops: Some(&[

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DoNotConnectToWindowsUpdateInternetLocations",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "WUServer",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "WUStatusServer",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"Software\Policies\Microsoft\Windows\WindowsUpdate\AU",

                                value_name: "UseWUServer",

                                value: RegistryValue::Dword(0),

                                stock_value: RegistryValue::Delete
        },

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

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpgrade",

                                value: RegistryValue::Dword(1),

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpgradePeriod",

                                value: RegistryValue::Dword(1),

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpdatePeriod",

                                value: RegistryValue::Dword(0),

                                stock_value: RegistryValue::Delete
        },

                ],

                disabled_ops: Some(&[

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpgrade",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpgradePeriod",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate",

                                value_name: "DeferUpdatePeriod",

                                value: RegistryValue::Delete,

                                stock_value: RegistryValue::Delete
        },

                ]),

                requires_restart: true
        },
        crate::tweak! {

                id: "disable_driver_searching",

                category: "updates",

                name: "Disable Driver Searching",

                description: "Disables automatic searching for driver updates on the internet.",

                effect: TweakEffect::Restart,

                enabled_ops: &[RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching",

                        value_name: "SearchOrderConfig",

                        value: RegistryValue::Dword(0),

                        stock_value: RegistryValue::Dword(1)
        }],

                disabled_ops: Some(&[RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching",

                        value_name: "SearchOrderConfig",

                        value: RegistryValue::Dword(1),

                        stock_value: RegistryValue::Dword(1)
        }]),

                requires_restart: true
        },
        crate::tweak! {

                id: "prevent_device_metadata",

                category: "updates",

                name: "Prevent Device Metadata from Network",

                description: "Prevents Windows from downloading device metadata from the internet.",

                effect: TweakEffect::Restart,

                enabled_ops: &[RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata",

                        value_name: "PreventDeviceMetadataFromNetwork",

                        value: RegistryValue::Dword(1),

                        stock_value: RegistryValue::Dword(0)
        }],

                disabled_ops: Some(&[RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Device Metadata",

                        value_name: "PreventDeviceMetadataFromNetwork",

                        value: RegistryValue::Dword(0),

                        stock_value: RegistryValue::Dword(0)
        }]),

                requires_restart: true
        },
        crate::tweak! {

                id: "disable_wuauserv",

                category: "updates",

                name: "Disable Windows Update Service",

                description: "Completely disables the Windows Update service (wuauserv).",

                effect: TweakEffect::Restart,

                enabled_ops: &[

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SYSTEM\ControlSet001\Services\wuauserv",

                                value_name: "Start",

                                value: RegistryValue::Dword(4),

                                stock_value: RegistryValue::Dword(3)
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Services\7971f918-a847-4430-9279-4a52d1efe18d",

                                value_name: "RegisteredWithAU",

                                value: RegistryValue::Dword(0),

                                stock_value: RegistryValue::Delete
        },

                ],

                disabled_ops: Some(&[

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SYSTEM\ControlSet001\Services\wuauserv",

                                value_name: "Start",

                                value: RegistryValue::Dword(2),

                                stock_value: RegistryValue::Dword(3)
        },

                        RegistryOp {

                                hkey: "HKLM",

                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsUpdate\Services\7971f918-a847-4430-9279-4a52d1efe18d",

                                value_name: "RegisteredWithAU",

                                value: RegistryValue::Dword(1),

                                stock_value: RegistryValue::Delete
        },

                ]),

                requires_restart: true
        },
        crate::tweak! {

        id: "disable_auto_update",

        category: "updates",

        name: "Disable Automatic Updates",

        description: "Disables automatic Windows updates via Automatic Updates (AU) policy.",

        effect: TweakEffect::Restart,

        enabled_ops: &[RegistryOp {

                hkey: "HKLM",

                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",

                value_name: "NoAutoUpdate",

                value: RegistryValue::Dword(1),

                stock_value: RegistryValue::Delete
        }],

        disabled_ops: Some(&[RegistryOp {

                hkey: "HKLM",

                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU",

                value_name: "NoAutoUpdate",

                value: RegistryValue::Delete,

                stock_value: RegistryValue::Delete
        }]),

        requires_restart: true
        },
        crate::tweak! {

        id: "disable_delivery_optimization",

        category: "updates",

        name: "Disable Delivery Optimization",

        description: "Disables Windows Update Delivery Optimization (P2P updates).",

        effect: TweakEffect::Restart,

        enabled_ops: &[

                RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config",

                        value_name: "DODownloadMode",

                        value: RegistryValue::Dword(0),

                        stock_value: RegistryValue::Delete
        },

                RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",

                        value_name: "DODownloadMode",

                        value: RegistryValue::Dword(0),

                        stock_value: RegistryValue::Delete
        },

                RegistryOp {

                        hkey: "HKCU",

                        subkey: r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization",

                        value_name: "SystemSettingsDownloadMode",

                        value: RegistryValue::Dword(0),

                        stock_value: RegistryValue::Delete
        },

        ],

        disabled_ops: Some(&[

                RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeliveryOptimization\Config",

                        value_name: "DODownloadMode",

                        value: RegistryValue::Delete,

                        stock_value: RegistryValue::Delete
        },

                RegistryOp {

                        hkey: "HKLM",

                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",

                        value_name: "DODownloadMode",

                        value: RegistryValue::Delete,

                        stock_value: RegistryValue::Delete
        },

                RegistryOp {

                        hkey: "HKCU",

                        subkey: r"Software\Microsoft\Windows\CurrentVersion\DeliveryOptimization",

                        value_name: "SystemSettingsDownloadMode",

                        value: RegistryValue::Delete,

                        stock_value: RegistryValue::Delete
        },

        ]),

        requires_restart: true
        },
        crate::tweak! {

        id: "disable_store_auto_update",

        category: "updates",

        name: "Disable Store Auto-Update",

        description: "Disables automatic updates for Windows Store apps.",

        effect: TweakEffect::Immediate,

        enabled_ops: &[RegistryOp {

                hkey: "HKLM",

                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate",

                value_name: "AutoDownload",

                value: RegistryValue::Dword(2),

                stock_value: RegistryValue::Delete
        }],

        disabled_ops: Some(&[RegistryOp {

                hkey: "HKLM",

                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\WindowsStore\WindowsUpdate",

                value_name: "AutoDownload",

                value: RegistryValue::Delete,

                stock_value: RegistryValue::Delete
        }])
        },
];
