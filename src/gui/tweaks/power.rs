// Power & Sleep tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static POWER_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_hibernation",
                category: "power",
                name: "Disable Hibernation",
                description: "Globally disables hibernation and removes hiberfil.sys.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Power",
                        value_name: "HibernateEnabledDefault",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Power",
                        value_name: "HibernateEnabledDefault",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_hybrid_sleep",
                category: "power",
                name: "Disable Hybrid Sleep",
                description: "Disables hybrid sleep mode on both battery and AC power.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_idle_sleep",
                category: "power",
                name: "Disable Idle Sleep",
                description: "Prevents system from sleeping when idle (both battery and AC).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_idle_hibernate",
                category: "power",
                name: "Disable Idle Hibernate",
                description: "Prevents system from hibernating when idle (both battery and AC).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_unattended_sleep",
                category: "power",
                name: "Disable Unattended Sleep",
                description: "Prevents sleep during unattended system sessions.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0",
                                value_name: "DCSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0",
                                value_name: "ACSettingIndex",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "hide_hibernate_option",
                category: "power",
                name: "Hide Hibernate in Power Menu",
                description: "Removes the Hibernate option from the Start Menu power options.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings",
                        value_name: "ShowHibernateOption",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings",
                        value_name: "ShowHibernateOption",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "hide_sleep_option",
                category: "power",
                name: "Hide Sleep in Power Menu",
                description: "Removes the Sleep option from the Start Menu power options.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings",
                        value_name: "ShowSleepOption",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings",
                        value_name: "ShowSleepOption",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_fast_startup",
                category: "power",
                name: "Disable Fast Startup",
                description: "Disables Fast Startup (hybrid boot) for cleaner shutdowns.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                                value_name: "HiberbootEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "HiberbootEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                                value_name: "HiberbootEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "HiberbootEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "hide_lock_option",
                category: "power",
                name: "Hide Lock in Account/Power Menu",
                description: "Removes the Lock option from the account picture and power menus.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideLock", value_name: "value", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", value_name: "ShowLockOption", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer", value_name: "ShowLockOption", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideLock", value_name: "value", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", value_name: "ShowLockOption", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer", value_name: "ShowLockOption", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
            id: "fast_shutdown",
            category: "power",
            name: "Enable Fast Shutdown",
            description: "Reduces the time Windows waits for services to stop during shutdown.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control",
                value_name: "WaitToKillServiceTimeout",
                value: RegistryValue::String("2000"), // 2 seconds
                stock_value: RegistryValue::String("5000") // Default is usually 5000 (5s)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control",
                value_name: "WaitToKillServiceTimeout",
                value: RegistryValue::String("5000"),
                stock_value: RegistryValue::String("5000")
            }]),
        },
        crate::tweak! {
            id: "toggle_screensaver",
            category: "power",
            name: "Turn On/Off Screen Saver",
            description: "Enables or disables the screen saver and sign-in on resume.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Control Panel\Desktop", value_name: "SCRNSAVE.EXE", value: RegistryValue::Delete, stock_value: RegistryValue::String(r"C:\Windows\System32\scrnsave.scr") },
                RegistryOp { hkey: "HKCU", subkey: r"Control Panel\Desktop", value_name: "ScreenSaverIsSecure", value: RegistryValue::String("0"), stock_value: RegistryValue::String("1") },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Control Panel\Desktop", value_name: "SCRNSAVE.EXE", value: RegistryValue::String(r"C:\Windows\System32\scrnsave.scr"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Control Panel\Desktop", value_name: "ScreenSaverIsSecure", value: RegistryValue::String("1"), stock_value: RegistryValue::String("0") },
            ]),
        },
];
