// System Tools tweaks

use super::Tweak;

pub static TOOLS_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "disable_auto_maintenance",
        category: "tools",
        name: "Disable Automatic Maintenance",
        description: "Disables Windows automatic maintenance tasks.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance", "MaintenanceDisabled", 1),
        ],
        },
        crate::tweak! {
                id: "enable_registry_backup",
                category: "tools",
                name: "Enable Registry Backup",
                description: "Enables automatic periodic registry backup (disabled by default in Win10 1803+).",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager", "EnablePeriodicBackup", 1),
                ],
        },
        crate::tweak! {
        id: "restore_point_frequency",
        category: "tools",
        name: "Remove Restore Point Limit",
        description: "Removes the 24-hour limit between manual restore point creation.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0),
        ],
        },
        crate::tweak! {
                id: "reset_windows_store",
                category: "tools",
                name: "Reset Windows Store",
                description: "Resets the Microsoft Store cache and re-registers the app.",
                enabled_ops: &[],
                                custom_apply: Some(|ctx| {
                        ctx.post_status("Running wsreset.exe -i...");
                        crate::run_system_command("wsreset.exe", &["-i"])?;
                        ctx.report_progress();
                        Ok(())
                })
        },
        crate::tweak! {
        id: "restore_photo_viewer",
        category: "tools",
        name: "Restore Windows Photo Viewer",
        description: "Restores Windows Photo Viewer as an option for opening images.",
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".bmp", "PhotoViewer.FileAssoc.Tiff"),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".gif", "PhotoViewer.FileAssoc.Tiff"),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".jpeg", "PhotoViewer.FileAssoc.Tiff"),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".jpg", "PhotoViewer.FileAssoc.Tiff"),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows Photo Viewer\Capabilities\FileAssociations", ".png", "PhotoViewer.FileAssoc.Tiff"),
        ],
        },
        crate::tweak! {
        id: "clear_oem_info",
        category: "tools",
        name: "Clear OEM Information",
        description: "Clears OEM information in System properties.",
        enabled_ops: &[
                crate::reg_del_key!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\OEMInformation", ""),
        ],
        },
];
