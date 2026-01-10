// System Tools tweaks

use super::{Tweak, TweakEffect};

pub static TOOLS_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "restore_photo_viewer",
                category: "tools",
                name: "Restore Windows Photo Viewer",
                description: "Restores Windows Photo Viewer as an option for opening images.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".bmp", "PhotoViewer.FileAssoc.Tiff", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".gif", "PhotoViewer.FileAssoc.Tiff", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".jpeg", "PhotoViewer.FileAssoc.Tiff", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".jpg", "PhotoViewer.FileAssoc.Tiff", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".png", "PhotoViewer.FileAssoc.Tiff", RegistryValue::Delete),
                ],
                },
        crate::tweak! {
                id: "restore_point_frequency",
                category: "tools",
                name: "Remove Restore Point Limit",
                description: "Removes the 24-hour limit between manual restore point creation.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0),
                ],
                },
        crate::tweak! {
                id: "enable_registry_backup",
                category: "tools",
                name: "Enable Registry Backup",
                description: "Enables automatic periodic registry backup (disabled by default in Win10 1803+).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager", "EnablePeriodicBackup", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_auto_maintenance",
                category: "tools",
                name: "Disable Automatic Maintenance",
                description: "Disables Windows automatic maintenance tasks.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance", "MaintenanceDisabled", 1),
                ],
                },
        crate::tweak! {
                id: "reset_windows_store",
                category: "tools",
                name: "Reset Windows Store",
                description: "Resets the Microsoft Store cache and re-registers the app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
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
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation", "Manufacturer", "W11Boost", RegistryValue::Delete),
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation", "SupportURL", "https://github.com/Admin/W11Boost", RegistryValue::Delete),
                ],
                },
        crate::tweak! {
                id: "set_registered_owner",
                category: "tools",
                name: "Set Registered Owner",
                description: "Changes the registered owner name in Windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", "RegisteredOwner", "W11Boost User", "epic"),
                ],
                },
];
