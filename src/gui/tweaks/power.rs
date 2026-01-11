// Power & Sleep tweaks

use super::Tweak;

pub static POWER_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_fast_startup",
                category: "power",
                name: "Disable Fast Startup",
                description: "Disables Fast Startup (hybrid boot) for cleaner shutdowns.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "HiberbootEnabled", 0),
                ],
        },
        crate::tweak! {
                id: "disable_hibernation",
                category: "power",
                name: "Disable Hibernation",
                description: "Globally disables hibernation and removes hiberfil.sys.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Power", "HibernateEnabledDefault", 0),
                ],
        },
        crate::tweak! {
                id: "disable_hybrid_sleep",
                category: "power",
                name: "Disable Hybrid Sleep",
                description: "Disables hybrid sleep mode on both battery and AC power.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "ACSettingIndex", 0),
                ],
        },
        crate::tweak! {
                id: "disable_idle_hibernate",
                category: "power",
                name: "Disable Idle Hibernate",
                description: "Prevents system from hibernating when idle (both battery and AC).",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "ACSettingIndex", 0),
                ],
        },
        crate::tweak! {
                id: "disable_idle_sleep",
                category: "power",
                name: "Disable Idle Sleep",
                description: "Prevents system from sleeping when idle (both battery and AC).",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "ACSettingIndex", 0),
                ],
        },
        crate::tweak! {
                id: "disable_unattended_sleep",
                category: "power",
                name: "Disable Unattended Sleep",
                description: "Prevents sleep during unattended system sessions.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "DCSettingIndex", 0),
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "ACSettingIndex", 0),
                ],
        },
        crate::tweak! {
        id: "fast_shutdown",
        category: "power",
        name: "Enable Fast Shutdown",
        description: "Reduces the time Windows waits for services to stop during shutdown.",
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SYSTEM\CurrentControlSet\Control", "WaitToKillServiceTimeout", "2000"),
        ],
                },
        crate::tweak! {
        id: "hide_hibernate_option",
        category: "power",
        name: "Hide Hibernate in Power Menu",
        description: "Removes the Hibernate option from the Start Menu power options.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowHibernateOption", 0),
        ],
        },
        crate::tweak! {
        id: "hide_sleep_option",
        category: "power",
        name: "Hide Sleep in Power Menu",
        description: "Removes the Sleep option from the Start Menu power options.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowSleepOption", 0),
        ],
        },
        crate::tweak! {
        id: "toggle_screensaver",
        category: "power",
        name: "Turn On/Off Screen Saver",
        description: "Enables or disables the screen saver and sign-in on resume.",
        enabled_ops: &[
            crate::reg_del!("HKCU", r"Control Panel\Desktop", "SCRNSAVE.EXE"),
            crate::reg_str!("HKCU", r"Control Panel\Desktop", "ScreenSaverIsSecure", "0"),
        ],
                },
];
