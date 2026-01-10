// Cloud Sync tweaks

use super::{RegistryValue, Tweak, TweakEffect};

pub static SYNC_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_settings_sync",
                category: "sync",
                name: "Disable Settings Sync",
                description: "Disables Windows Settings Sync (Windows Backup) completely.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSettingSync", 2),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSettingSyncUserOverride", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSyncOnPaidNetwork", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableWindowsSettingSync", 2),
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync", "SyncPolicy", 5),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSettingSync", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSettingSyncUserOverride", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableSyncOnPaidNetwork", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\SettingSync", "DisableWindowsSettingSync", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync", "SyncPolicy", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_personalization_sync",
                category: "sync",
                name: "Disable Personalization Sync",
                description: "Disables syncing of personalization settings (themes, colors).",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Personalization", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Personalization", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_browser_sync",
                category: "sync",
                name: "Disable Browser Settings Sync",
                description: "Disables syncing of browser settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\BrowserSettings", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\BrowserSettings", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_credentials_sync",
                category: "sync",
                name: "Disable Credentials Sync",
                description: "Disables syncing of passwords and credentials.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_accessibility_sync",
                category: "sync",
                name: "Disable Accessibility Sync",
                description: "Disables syncing of accessibility settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Accessibility", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Accessibility", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_windows_sync",
                category: "sync",
                name: "Disable Windows Settings Sync",
                description: "Disables syncing of Windows-specific settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Windows", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Windows", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_language_sync",
                category: "sync",
                name: "Disable Language Sync",
                description: "Disables syncing of language preferences.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Language", "Enabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Language", "Enabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_cross_device_resume",
                category: "sync",
                name: "Disable Cross-Device Resume",
                description: "Disables resuming activities across devices.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration", "IsResumeAllowed", 0, 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration", "IsResumeAllowed", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
            id: "disable_cross_device_clipboard",
            category: "sync",
            name: "Disable Cross-Device Clipboard",
            description: "Prevents clipboard content from syncing across devices.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowCrossDeviceClipboard", 0),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowCrossDeviceClipboard", RegistryValue::Delete),
            ])
        },
        crate::tweak! {
            id: "disable_message_sync",
            category: "sync",
            name: "Disable Messaging Sync",
            description: "Disables cloud synchronization of text messages.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync", 0),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync", RegistryValue::Delete),
            ])
        },
];
