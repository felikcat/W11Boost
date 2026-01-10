// Power & Sleep tweaks

use super::{Tweak, TweakEffect};

pub static POWER_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_hibernation",
                category: "power",
                name: "Disable Hibernation",
                description: "Globally disables hibernation and removes hiberfil.sys.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Power", "HibernateEnabledDefault", 0, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_hybrid_sleep",
                category: "power",
                name: "Disable Hybrid Sleep",
                description: "Disables hybrid sleep mode on both battery and AC power.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "ACSettingIndex", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_idle_sleep",
                category: "power",
                name: "Disable Idle Sleep",
                description: "Prevents system from sleeping when idle (both battery and AC).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "ACSettingIndex", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_idle_hibernate",
                category: "power",
                name: "Disable Idle Hibernate",
                description: "Prevents system from hibernating when idle (both battery and AC).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "ACSettingIndex", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_unattended_sleep",
                category: "power",
                name: "Disable Unattended Sleep",
                description: "Prevents sleep during unattended system sessions.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "ACSettingIndex", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "hide_hibernate_option",
                category: "power",
                name: "Hide Hibernate in Power Menu",
                description: "Removes the Hibernate option from the Start Menu power options.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowHibernateOption", 0),
                ],
                },
        crate::tweak! {
                id: "hide_sleep_option",
                category: "power",
                name: "Hide Sleep in Power Menu",
                description: "Removes the Sleep option from the Start Menu power options.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowSleepOption", 0),
                ],
                },
        crate::tweak! {
                id: "disable_fast_startup",
                category: "power",
                name: "Disable Fast Startup",
                description: "Disables Fast Startup (hybrid boot) for cleaner shutdowns.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 0, 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "HiberbootEnabled", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "hide_lock_option",
                category: "power",
                name: "Hide Lock in Account/Power Menu",
                description: "Removes the Lock option from the account picture and power menus.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideLock", "value", 1, 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowLockOption", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "ShowLockOption", 0),
                ],
                },
        crate::tweak! {
            id: "fast_shutdown",
            category: "power",
            name: "Enable Fast Shutdown",
            description: "Reduces the time Windows waits for services to stop during shutdown.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SYSTEM\CurrentControlSet\Control", "WaitToKillServiceTimeout", "2000", "5000"),
            ],
                    },
        crate::tweak! {
            id: "toggle_screensaver",
            category: "power",
            name: "Turn On/Off Screen Saver",
            description: "Enables or disables the screen saver and sign-in on resume.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_del!("HKCU", r"Control Panel\Desktop", "SCRNSAVE.EXE", RegistryValue::String(r"C:\Windows\System32\scrnsave.scr")),
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "ScreenSaverIsSecure", "0", "1"),
            ],
                    },
];
