// System Tools tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static TOOLS_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "restore_photo_viewer",
                category: "tools",
                name: "Restore Windows Photo Viewer",
                description: "Restores Windows Photo Viewer as an option for opening images.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".bmp",
                                value: RegistryValue::String("PhotoViewer.FileAssoc.Tiff"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".gif",
                                value: RegistryValue::String("PhotoViewer.FileAssoc.Tiff"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".jpeg",
                                value: RegistryValue::String("PhotoViewer.FileAssoc.Tiff"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".jpg",
                                value: RegistryValue::String("PhotoViewer.FileAssoc.Tiff"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".png",
                                value: RegistryValue::String("PhotoViewer.FileAssoc.Tiff"),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".bmp",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".gif",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".jpeg",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".jpg",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations",
                                value_name: ".png",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "restore_point_frequency",
                category: "tools",
                name: "Remove Restore Point Limit",
                description: "Removes the 24-hour limit between manual restore point creation.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore",
                        value_name: "SystemRestorePointCreationFrequency",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore",
                        value_name: "SystemRestorePointCreationFrequency",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "enable_registry_backup",
                category: "tools",
                name: "Enable Registry Backup",
                description: "Enables automatic periodic registry backup (disabled by default in Win10 1803+).",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager",
                        value_name: "EnablePeriodicBackup",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager",
                        value_name: "EnablePeriodicBackup",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_auto_maintenance",
                category: "tools",
                name: "Disable Automatic Maintenance",
                description: "Disables Windows automatic maintenance tasks.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance",
                        value_name: "MaintenanceDisabled",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance",
                        value_name: "MaintenanceDisabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "reset_windows_store",
                category: "tools",
                name: "Reset Windows Store",
                description: "Resets the Microsoft Store cache and re-registers the app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| {
                        ctx.post_status("Running wsreset.exe -i...");
                        crate::run_system_command("wsreset.exe", &["-i"])?;
                        ctx.report_progress();
                        Ok(())
                })
        },
        crate::tweak! {
                id: "set_oem_info",
                category: "tools",
                name: "Set OEM Information",
                description: "Sets custom OEM information in System properties.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation",
                                value_name: "Manufacturer",
                                value: RegistryValue::String("W11Boost"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation",
                                value_name: "SupportURL",
                                value: RegistryValue::String("https://github.com/Admin/W11Boost"),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation",
                                value_name: "Manufacturer",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation",
                                value_name: "SupportURL",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "set_registered_owner",
                category: "tools",
                name: "Set Registered Owner",
                description: "Changes the registered owner name in Windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                        value_name: "RegisteredOwner",
                        value: RegistryValue::String("W11Boost User"),
                        stock_value: RegistryValue::String("epic")
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
                        value_name: "RegisteredOwner",
                        value: RegistryValue::String("Windows User"),
                        stock_value: RegistryValue::String("Windows User")
        }])
        },
];
