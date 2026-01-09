// Cloud Sync tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static SYNC_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_settings_sync",
                category: "sync",
                name: "Disable Settings Sync",
                description: "Disables Windows Settings Sync (Windows Backup) completely.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSettingSync",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSettingSyncUserOverride",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSyncOnPaidNetwork",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableWindowsSettingSync",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync",
                                value_name: "SyncPolicy",
                                value: RegistryValue::Dword(5),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSettingSync",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSettingSyncUserOverride",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableSyncOnPaidNetwork",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\SettingSync",
                                value_name: "DisableWindowsSettingSync",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync",
                                value_name: "SyncPolicy",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_personalization_sync",
                category: "sync",
                name: "Disable Personalization Sync",
                description: "Disables syncing of personalization settings (themes, colors).",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Personalization",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Personalization",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_browser_sync",
                category: "sync",
                name: "Disable Browser Settings Sync",
                description: "Disables syncing of browser settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\BrowserSettings",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\BrowserSettings",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_credentials_sync",
                category: "sync",
                name: "Disable Credentials Sync",
                description: "Disables syncing of passwords and credentials.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Credentials",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_accessibility_sync",
                category: "sync",
                name: "Disable Accessibility Sync",
                description: "Disables syncing of accessibility settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Accessibility",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Accessibility",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_windows_sync",
                category: "sync",
                name: "Disable Windows Settings Sync",
                description: "Disables syncing of Windows-specific settings.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Windows",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Windows",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_language_sync",
                category: "sync",
                name: "Disable Language Sync",
                description: "Disables syncing of language preferences.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Language",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\SettingSync\Groups\Language",
                        value_name: "Enabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_cross_device_resume",
                category: "sync",
                name: "Disable Cross-Device Resume",
                description: "Disables resuming activities across devices.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration",
                        value_name: "IsResumeAllowed",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\CrossDeviceResume\Configuration",
                        value_name: "IsResumeAllowed",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
            id: "disable_cross_device_clipboard",
            category: "sync",
            name: "Disable Cross-Device Clipboard",
            description: "Prevents clipboard content from syncing across devices.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                value_name: "AllowCrossDeviceClipboard",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                value_name: "AllowCrossDeviceClipboard",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }])
        },
        crate::tweak! {
            id: "disable_message_sync",
            category: "sync",
            name: "Disable Messaging Sync",
            description: "Disables cloud synchronization of text messages.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Messaging",
                value_name: "AllowMessageSync",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Messaging",
                value_name: "AllowMessageSync",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }])
        },
];
