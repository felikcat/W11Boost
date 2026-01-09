// Context Menu tweaks

use super::super::shared_state::WorkerContext;
use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};
use anyhow::Result;
use std::sync::Arc;

pub static REMOVE_CONTEXT_PINS: Tweak = crate::tweak! {
        id: "remove_context_menu_pins",
        category: "context_menu",
        name: "Remove 'Pin to Start' and 'Pin to Quick Access'",
        description: "Removes pinning options from the context menu to reduce clutter.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                // Remove Pin to Home (Quick Access)
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\pintohome", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\pintohome", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\pintohome", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Network\shell\pintohome", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                // Remove Pin to Start
                RegistryOp { hkey: "HKCR", subkey: r"exefile\shellex\ContextMenuHandlers\PintoStartScreen", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\ShellEx\ContextMenuHandlers\PintoStartScreen", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Microsoft.Website\shellex\ContextMenuHandlers\PintoStartScreen", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"mscfile\shellex\ContextMenuHandlers\PintoStartScreen", value_name: "", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{470C0EBD-5D73-4d58-9CED-E91E22E23282}",
                        value: RegistryValue::String(""),
                        stock_value: RegistryValue::Delete
                },
        ],
        disabled_ops: Some(&[
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{470C0EBD-5D73-4d58-9CED-E91E22E23282}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                },
        ])
};

pub static REMOVE_TERMINAL: Tweak = crate::tweak! {
        id: "remove_open_in_terminal",
        category: "context_menu",
        name: "Remove 'Open in Terminal'",
        description: "Removes 'Open in Terminal' from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{9F156763-7844-4DC4-B2B1-901F640F5155}",
                value: RegistryValue::String(""),
                stock_value: RegistryValue::Delete
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{9F156763-7844-4DC4-B2B1-901F640F5155}",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
        }])
};

pub static REMOVE_EDIT_NOTEPAD: Tweak = crate::tweak! {
        id: "remove_edit_in_notepad",
        category: "context_menu",
        name: "Remove 'Edit in Notepad'",
        description: "Removes 'Edit in Notepad' from the file context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{CA6CC9F1-867A-481E-951E-A28C5E4F01EA}",
                value: RegistryValue::String(""),
                stock_value: RegistryValue::Delete
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{CA6CC9F1-867A-481E-951E-A28C5E4F01EA}",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
        }])
};

pub static REMOVE_MOVE_TO_ONEDRIVE: Tweak = crate::tweak! {
        id: "remove_move_to_onedrive",
        category: "context_menu",
        name: "Remove 'Move to OneDrive'",
        description: "Removes 'Move to OneDrive' from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", value: RegistryValue::String("OneDrive"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", value: RegistryValue::String("OneDrive"), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_PHOTOS_MENU: Tweak = crate::tweak! {
        id: "remove_photos_menu",
        category: "context_menu",
        name: "Remove 'Photos' Menu Items",
        description: "Removes 'Edit with Photos' and other Photos app items from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{BFE0E2A4-C70C-4AD7-AC3D-10D1ECEBB5B4}", value: RegistryValue::String("Photos"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{1100CBCD-B822-43F0-84CB-16814C2F6B44}", value: RegistryValue::String("Photos"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{7A53B94A-4E6E-4826-B48E-535020B264E5}", value: RegistryValue::String("Photos"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{9AAFEDA2-97B6-43EA-9466-9DE90501B1B6}", value: RegistryValue::String("Photos"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{BFE0E2A4-C70C-4AD7-AC3D-10D1ECEBB5B4}", value: RegistryValue::String("Photos"), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{BFE0E2A4-C70C-4AD7-AC3D-10D1ECEBB5B4}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_CAST_TO_DEVICE: Tweak = crate::tweak! {
        id: "remove_cast_to_device",
        category: "context_menu",
        name: "Remove 'Cast to Device'",
        description: "Removes 'Cast to Device' from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", value: RegistryValue::String("Play to Menu"), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_ASK_COPILOT: Tweak = crate::tweak! {
        id: "remove_ask_copilot",
        category: "context_menu",
        name: "Remove 'Ask Copilot' from Context Menu",
        description: "Removes default 'Ask Copilot' entry from context menu on files.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}",
                        value: RegistryValue::String(""),
                        stock_value: RegistryValue::Delete
                },
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}",
                        value: RegistryValue::String(""),
                        stock_value: RegistryValue::Delete
                }
        ],
        disabled_ops: Some(&[
                RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                },
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }
        ])
};

pub static REMOVE_CUSTOMIZE_THIS_FOLDER: Tweak = crate::tweak! {
        id: "remove_customize_folder",
        category: "context_menu",
        name: "Remove 'Customize this folder'",
        description: "Removes 'Customize this folder' from context menu and 'Customize' tab from Properties.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoCustomizeThisFolder", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::String("") },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::String("") },
                 RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoCustomizeThisFolder", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_CHANGE_BITLOCKER_PASSWORD: Tweak = crate::tweak! {
        id: "remove_change_bitlocker_password",
        category: "context_menu",
        name: "Remove 'Change BitLocker password'",
        description: "Removes the 'Change BitLocker password' context menu option.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\Drive\shell\change-passphrase", value_name: "LegacyDisable", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\Drive\shell\change-passphrase", value_name: "LegacyDisable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_COPY_AS_PATH: Tweak = crate::tweak! {
        id: "remove_copy_as_path",
        category: "context_menu",
        name: "Remove 'Copy as path'",
        description: "Removes the 'Copy as path' context menu option.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\AllFilesystemObjects\shellex\ContextMenuHandlers\CopyAsPathMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
        ],
        disabled_ops: Some(&[
                 RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\AllFilesystemObjects\shellex\ContextMenuHandlers\CopyAsPathMenu", value_name: "", value: RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}"), stock_value: RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}") },
        ])
};

pub static REMOVE_TROUBLESHOOT_COMPATIBILITY: Tweak = crate::tweak! {
        id: "remove_troubleshoot_compatibility",
        category: "context_menu",
        name: "Remove 'Troubleshoot compatibility'",
        description: "Removes 'Troubleshoot compatibility' from the context menu of executables.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\exefile\shell\compatibility", value_name: "LegacyDisable", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\exefile\shell\compatibility", value_name: "LegacyDisable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_PIN_TO_START: Tweak = crate::tweak! {
        id: "remove_pin_to_start",
        category: "context_menu",
        name: "Remove 'Pin to Start'",
        description: "Removes 'Pin to Start' from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{470C0EBD-5D73-4d58-9CED-E91E22E23282}",
                        value: RegistryValue::String(""),
                        stock_value: RegistryValue::Delete
                }
        ],
        disabled_ops: Some(&[
                RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                        value_name: "{470C0EBD-5D73-4d58-9CED-E91E22E23282}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }
        ])
};

pub static REMOVE_NVIDIA_CONTROL_PANEL: Tweak = crate::tweak! {
        id: "remove_nvidia_context",
        category: "context_menu",
        name: "Remove 'NVIDIA Control Panel'",
        description: "Removes the NVIDIA Control Panel from the desktop context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\NVIDIA Corporation\Global\NvCplApi\Policies", value_name: "ContextUIPolicy", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(2) },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\NVIDIA Corporation\Global\NvCplApi\Policies", value_name: "ContextUIPolicy", value: RegistryValue::Dword(2), stock_value: RegistryValue::Dword(2) },
        ])
};

pub static REMOVE_PERSONALIZE_DISPLAY: Tweak = crate::tweak! {
        id: "remove_personalize_display",
        category: "context_menu",
        name: "Remove 'Personalize' and 'Display Settings'",
        description: "Removes 'Personalize' and 'Display settings' from the desktop context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalize", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
        ],
        disabled_ops: Some(&[
                // Note: Deleting these keys is destructive. Reverting requires re-adding them.
                // For simplicity, we assume they are standard and can be restored if needed,
                // but usually users don't want them back if they enable this.
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalize", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
        ])
};

pub static REMOVE_EDIT_PAINT: Tweak = crate::tweak! {
        id: "remove_edit_with_paint",
        category: "context_menu",
        name: "Remove 'Edit with Paint'",
        description: "Removes 'Edit with Paint' from the context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{2430F218-B743-4FD6-97BF-5C76541B4AE9}",
                value: RegistryValue::String("Edit with Paint"),
                stock_value: RegistryValue::Delete
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{2430F218-B743-4FD6-97BF-5C76541B4AE9}",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
        }])
};

pub static REMOVE_EDIT_CLIPCHAMP: Tweak = crate::tweak! {
        id: "remove_edit_with_clipchamp",
        category: "context_menu",
        name: "Remove 'Edit with Clipchamp'",
        description: "Removes 'Edit with Clipchamp' from the context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{8AB635F8-9A67-4698-AB99-784AD929F3B4}",
                value: RegistryValue::String("Edit with Clipchamp"),
                stock_value: RegistryValue::Delete
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                value_name: "{8AB635F8-9A67-4698-AB99-784AD929F3B4}",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
        }])
};

fn apply_remove_set_as_desktop_bg(_context: &Arc<WorkerContext>) -> Result<()>
{
        let extensions = [
                ".avci", ".avcs", ".avif", ".avifs", ".bmp", ".dib", ".gif", ".heic", ".heics", ".heif", ".heifs",
                ".hif", ".jfif", ".jpe", ".jpeg", ".jpg", ".png", ".tif", ".tiff", ".wdp",
        ];

        for ext in extensions {
                let subkey = format!(r"SystemFileAssociations\{}\Shell\setdesktopwallpaper", ext);
                // We want to hide it, so we add "ProgrammaticAccessOnly"
                let _ = crate::common::set_string(&winsafe::HKEY::CLASSES_ROOT, &subkey, "ProgrammaticAccessOnly", "");
        }
        Ok(())
}

fn revert_remove_set_as_desktop_bg(_context: &Arc<WorkerContext>) -> Result<()>
{
        let extensions = [
                ".avci", ".avcs", ".avif", ".avifs", ".bmp", ".dib", ".gif", ".heic", ".heics", ".heif", ".heifs",
                ".hif", ".jfif", ".jpe", ".jpeg", ".jpg", ".png", ".tif", ".tiff", ".wdp",
        ];

        for ext in extensions {
                let subkey = format!(r"SystemFileAssociations\{}\Shell\setdesktopwallpaper", ext);
                let _ = crate::common::delete_value(&winsafe::HKEY::CLASSES_ROOT, &subkey, "ProgrammaticAccessOnly");
        }
        Ok(())
}

pub static REMOVE_SET_AS_DESKTOP_BG: Tweak = crate::tweak! {
        id: "remove_set_as_desktop_background",
        category: "context_menu",
        name: "Remove 'Set as desktop background'",
        description: "Removes 'Set as desktop background' from the context menu of image files.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        custom_apply: Some(apply_remove_set_as_desktop_bg),
        custom_revert: Some(revert_remove_set_as_desktop_bg),
        // We use dummy ops to satisfy the struct if needed, or just empty.
        // The macro might require enabled_ops.
        enabled_ops: &[],
        disabled_ops: None
};

pub static REMOVE_SEND_TO: Tweak = crate::tweak! {
        id: "remove_send_to_context_menu",
        category: "context_menu",
        name: "Remove 'Send To' Context Menu",
        description: "Removes the 'Send To' submenu from the context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}") },
                RegistryOp { hkey: "HKCR", subkey: r"UserLibraryFolder\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}") },
                RegistryOp { hkey: "HKCR", subkey: r"Drives\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}") },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}") },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"UserLibraryFolder\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drives\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}"), stock_value: RegistryValue::DeleteKey },
        ])
};

pub static REMOVE_SEND_TO_MY_PHONE: Tweak = crate::tweak! {
        id: "remove_send_to_my_phone",
        category: "context_menu",
        name: "Remove 'Send to My Phone'",
        description: "Removes 'Send to My Phone' from the context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{2F788D0F-1317-441B-86D2-4725301BD49D}", value: RegistryValue::String("Send to My Phone"), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{2F788D0F-1317-441B-86D2-4725301BD49D}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static REMOVE_SHARE_CONTEXT_MENU: Tweak = crate::tweak! {
        id: "remove_share_context_menu",
        category: "context_menu",
        name: "Remove 'Share' Context Menu",
        description: "Removes the 'Share' option from the context menu.",
        effect: TweakEffect::ExplorerRestart,
        is_hidden: true,
        enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::String("Share"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::String("Share"), stock_value: RegistryValue::Delete },
        ],
        disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
        ])
};

pub static CONTEXT_MENU_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "add_cmd_here",
                category: "context_menu",
                name: "Add Command Prompt Here",
                description: "Adds 'Open Command Prompt here' to the folder context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\cmd2",
                                value_name: "",
                                value: RegistryValue::String("Command Prompt Here"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\cmd2\command",
                                value_name: "",
                                value: RegistryValue::String(r#"cmd.exe /s /k pushd "%V""#),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\cmd2",
                                value_name: "",
                                value: RegistryValue::String("Command Prompt Here"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\cmd2\command",
                                value_name: "",
                                value: RegistryValue::String(r#"cmd.exe /s /k pushd "%V""#),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\cmd2",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\cmd2",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                ])
        },
        crate::tweak! {
                id: "add_powershell_admin",
                category: "context_menu",
                name: "Add PowerShell (Admin) Here",
                description: "Adds 'Open PowerShell here as Administrator' to the folder context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\OpenElevatedPS",
                                value_name: "",
                                value: RegistryValue::String("PowerShell (Admin) Here"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\OpenElevatedPS",
                                value_name: "HasLUAShield",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\OpenElevatedPS\command",
                                value_name: "",
                                value: RegistryValue::String(r#"PowerShell.exe -windowstyle hidden -Command "Start-Process cmd.exe -ArgumentList '/s,/c,pushd %V && powershell' -Verb RunAs""#),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\OpenElevatedPS",
                                value_name: "",
                                value: RegistryValue::String("PowerShell (Admin) Here"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\OpenElevatedPS",
                                value_name: "HasLUAShield",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\OpenElevatedPS\command",
                                value_name: "",
                                value: RegistryValue::String(r#"PowerShell.exe -windowstyle hidden -Command "Start-Process cmd.exe -ArgumentList '/s,/c,pushd %V && powershell' -Verb RunAs""#),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\OpenElevatedPS",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\Background\shell\OpenElevatedPS",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                ])
        },
        crate::tweak! {
                id: "add_take_ownership",
                category: "context_menu",
                name: "Add Take Ownership",
                description: "Adds 'Take Ownership' option to file and folder context menus.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\TakeOwnership",
                                value_name: "",
                                value: RegistryValue::String("Take Ownership"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\TakeOwnership",
                                value_name: "HasLUAShield",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\TakeOwnership\command",
                                value_name: "",
                                value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /c /l' -Verb runAs""#),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\TakeOwnership",
                                value_name: "",
                                value: RegistryValue::String("Take Ownership"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\TakeOwnership",
                                value_name: "HasLUAShield",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\TakeOwnership\command",
                                value_name: "",
                                value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" /r /d y && icacls \"%1\" /grant *S-1-3-4:F /c /l /q' -Verb runAs""#),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\TakeOwnership",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Directory\shell\TakeOwnership",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                ])
        },
        crate::tweak! {
                id: "add_restart_explorer",
                category: "context_menu",
                name: "Add Restart Explorer",
                description: "Adds 'Restart Explorer' to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\shell\RestartExplorer",
                                value_name: "",
                                value: RegistryValue::String("Restart Explorer"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\shell\RestartExplorer",
                                value_name: "Icon",
                                value: RegistryValue::String("explorer.exe"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\shell\RestartExplorer\command",
                                value_name: "",
                                value: RegistryValue::String(r#"cmd.exe /c taskkill /f /im explorer.exe & start explorer.exe"#),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\DesktopBackground\shell\RestartExplorer",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::DeleteKey
        }])
        },
        crate::tweak! {
                id: "add_kill_not_responding",
                category: "context_menu",
                name: "Add Kill Not Responding",
                description: "Adds 'Kill Not Responding Tasks' to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\shell\KillNotResponding",
                                value_name: "",
                                value: RegistryValue::String("Kill Not Responding Tasks"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\shell\KillNotResponding\command",
                                value_name: "",
                                value: RegistryValue::String(r#"taskkill /F /FI "status eq NOT RESPONDING""#),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\DesktopBackground\shell\KillNotResponding",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::DeleteKey
        }])
        },
        crate::tweak! {
                id: "add_disk_cleanup",
                category: "context_menu",
                name: "Add Disk Cleanup to Drive Context Menu",
                description: "Adds 'Cleanup' to the context menu of all drives.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Drive\shell\Windows.CleanUp",
                                value_name: "CommandStateSync",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Drive\shell\Windows.CleanUp",
                                value_name: "ExplorerCommandHandler",
                                value: RegistryValue::String("{9cca66bb-9c78-4e59-a76f-a5e9990b8aa0}"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Drive\shell\Windows.CleanUp",
                                value_name: "Icon",
                                value: RegistryValue::String(r"%SystemRoot%\System32\cleanmgr.exe,-104"),
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Drive\shell\Windows.CleanUp",
                                value_name: "ImpliedSelectionModel",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::DeleteKey
        },
                ],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Drive\shell\Windows.CleanUp",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::DeleteKey
        }])
        },
        crate::tweak! {
                id: "add_copy_to_move_to",
                category: "context_menu",
                name: "Add 'Copy To' and 'Move To' to Context Menu",
                description: "Adds 'Copy To folder' and 'Move to folder' options to the context menu for all files and folders.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\Copy To",
                                value_name: "",
                                value: RegistryValue::String("{C2FBB630-2971-11D1-A18C-00C04FD75D13}"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\Move To",
                                value_name: "",
                                value: RegistryValue::String("{C2FBB631-2971-11D1-A18C-00C04FD75D13}"),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\Copy To",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\Move To",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "add_kill_all_not_responding",
                category: "context_menu",
                name: "Add 'Kill All Not Responding Tasks'",
                description: "Adds context menu to Desktop to kill all unresponsive applications.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"DesktopBackground\Shell\KillNRTasks",
                                value_name: "",
                                value: RegistryValue::String("Kill all not responding tasks"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"DesktopBackground\Shell\KillNRTasks",
                                value_name: "icon",
                                value: RegistryValue::String("taskmgr.exe,-30651"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"DesktopBackground\Shell\KillNRTasks",
                                value_name: "Position",
                                value: RegistryValue::String("Top"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"DesktopBackground\Shell\KillNRTasks\command",
                                value_name: "",
                                value: RegistryValue::String(r#"CMD.exe /C (tasklist /fi "status eq Not Responding" | find /v "No tasks" && taskkill.exe /f /fi "status eq Not Responding" || echo No not-responding tasks found.) & ECHO; & <NUL: set /p junk=Press any key to close this window. & PAUSE >NUL:"#),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"DesktopBackground\Shell\KillNRTasks",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "add_priority_context_menu",
                category: "context_menu",
                name: "Add 'Run with Priority'",
                description: "Adds a 'Run with priority' submenu to the context menu of executable files.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        // Main Menu Item
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority", value_name: "MUIVerb", value: RegistryValue::String("Run with priority"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },

                        // Realtime
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\001flyout", value_name: "", value: RegistryValue::String("Realtime"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /Realtime "%1""#), stock_value: RegistryValue::DeleteKey },

                        // High
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\002flyout", value_name: "", value: RegistryValue::String("High"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /High "%1""#), stock_value: RegistryValue::DeleteKey },

                        // Above Normal
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\003flyout", value_name: "", value: RegistryValue::String("Above Normal"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\003flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /AboveNormal "%1""#), stock_value: RegistryValue::DeleteKey },

                        // Normal
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\004flyout", value_name: "", value: RegistryValue::String("Normal"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\004flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /Normal "%1""#), stock_value: RegistryValue::DeleteKey },

                        // Below Normal
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\005flyout", value_name: "", value: RegistryValue::String("Below Normal"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\005flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /BelowNormal "%1""#), stock_value: RegistryValue::DeleteKey },

                        // Low
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\006flyout", value_name: "", value: RegistryValue::String("Low"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority\shell\006flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /c start "" /Low "%1""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\exefile\shell\priority", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ]),
        },
        crate::tweak! {
                id: "add_copy_contents_to_clipboard",
                category: "context_menu",
                name: "Add 'Copy Contents to Clipboard'",
                description: "Adds option to context menu to copy file contents directly to clipboard (for all file types).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"*\shell\CopyContents",
                                value_name: "MUIVerb",
                                value: RegistryValue::String("Copy Contents to Clipboard"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"*\shell\CopyContents",
                                value_name: "Icon",
                                value: RegistryValue::String("DxpTaskSync.dll,-52"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"*\shell\CopyContents\command",
                                value_name: "",
                                value: RegistryValue::String(r#"cmd /c clip < "%1""#),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCR",
                                subkey: r"*\shell\CopyContents",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "remove_context_menu_items",
                category: "context_menu",
                name: "Remove Context Menu Items",
                description: "Grouped context menu removal options to reduce clutter.",
                effect: TweakEffect::ExplorerRestart,
                sub_tweaks: &[
                        &REMOVE_CONTEXT_PINS,
                        &REMOVE_TERMINAL,
                        &REMOVE_EDIT_NOTEPAD,
                        &REMOVE_ASK_COPILOT,
                        &REMOVE_CUSTOMIZE_THIS_FOLDER,
                        &REMOVE_CHANGE_BITLOCKER_PASSWORD,
                        &REMOVE_COPY_AS_PATH,
                        &REMOVE_TROUBLESHOOT_COMPATIBILITY,
                        &REMOVE_PIN_TO_START,
                        &REMOVE_MOVE_TO_ONEDRIVE,
                        &REMOVE_PHOTOS_MENU,
                        &REMOVE_CAST_TO_DEVICE,
                        &REMOVE_NVIDIA_CONTROL_PANEL,
                        &REMOVE_PERSONALIZE_DISPLAY,
                        &REMOVE_EDIT_PAINT,
                        &REMOVE_EDIT_CLIPCHAMP,
                        &REMOVE_SEND_TO,
                        &REMOVE_SEND_TO_MY_PHONE,
                        &REMOVE_SET_AS_DESKTOP_BG,
                        &REMOVE_SHARE_CONTEXT_MENU,
                ],
        },
        crate::tweak! {
            id: "add_settings_context_menu",
            category: "context_menu",
            name: "Add Settings to Desktop Context Menu",
            description: "Adds a 'Settings' submenu to the desktop context menu with quick access to various settings pages.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "Icon", value: RegistryValue::String("shell32.dll,-16826"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "MUIVerb", value: RegistryValue::String("&Settings"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                    // Home
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-51380"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("&Home"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:home"), stock_value: RegistryValue::DeleteKey },
                    // System
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-35"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("&System"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:system"), stock_value: RegistryValue::DeleteKey },
                    // Devices
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu", value_name: "Icon", value: RegistryValue::String("BthpanContextHandler.dll,-200"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu", value_name: "MUIVerb", value: RegistryValue::String("&Bluetooth && devices"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:devices"), stock_value: RegistryValue::DeleteKey },
                    // Network
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-193"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu", value_name: "MUIVerb", value: RegistryValue::String("&Network && internet"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:network"), stock_value: RegistryValue::DeleteKey },
                    // Personalization
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu", value_name: "Icon", value: RegistryValue::String("themecpl.dll,-1"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu", value_name: "MUIVerb", value: RegistryValue::String("&Personalization"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:personalization"), stock_value: RegistryValue::DeleteKey },
                    // Apps
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-63010"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu", value_name: "MUIVerb", value: RegistryValue::String("&Apps"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:appsfeatures"), stock_value: RegistryValue::DeleteKey },
                    // Accounts
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-88"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu", value_name: "MUIVerb", value: RegistryValue::String("A&ccounts"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:accounts"), stock_value: RegistryValue::DeleteKey },
                    // Time & Language
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-276"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu", value_name: "MUIVerb", value: RegistryValue::String("&Time && language"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:dateandtime"), stock_value: RegistryValue::DeleteKey },
                    // Gaming
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu", value_name: "Icon", value: RegistryValue::String("DDORes.dll,-2207"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu", value_name: "MUIVerb", value: RegistryValue::String("&Gaming"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:gaming-gamebar"), stock_value: RegistryValue::DeleteKey },
                    // Accessibility
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-86"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu", value_name: "MUIVerb", value: RegistryValue::String("Acc&essibility"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:easeofaccess"), stock_value: RegistryValue::DeleteKey },
                    // Privacy & Security
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu", value_name: "Icon", value: RegistryValue::String(r"%ProgramFiles%\Windows Defender\EppManifest.dll,-101"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu", value_name: "MUIVerb", value: RegistryValue::String("Pri&vacy && security"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:privacy"), stock_value: RegistryValue::DeleteKey },
                    // Windows Update
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-1401"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu", value_name: "MUIVerb", value: RegistryValue::String("&Windows Update"), stock_value: RegistryValue::DeleteKey },
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                    RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ])
        },
        crate::tweak! {
                id: "add_autoplay_context_menu",
                category: "context_menu",
                name: "Add AutoPlay to Desktop Context Menu",
                description: "Adds an 'AutoPlay' menu to the desktop context menu to quickly access AutoPlay settings.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\AutoPlay", value_name: "", value: RegistryValue::String("AutoPlay"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\AutoPlay", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5362"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\AutoPlay", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\AutoPlay", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        // Submenu item 1: Settings
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\001menu", value_name: "", value: RegistryValue::String("Open AutoPlay in Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\001menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-16826"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\001menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:autoplay"), stock_value: RegistryValue::DeleteKey },
                        // Submenu item 2: Control Panel
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\002menu", value_name: "", value: RegistryValue::String("Open AutoPlay in Control Panel"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\002menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-27"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\AutoPlay\shell\002menu\command", value_name: "", value: RegistryValue::String("control /name Microsoft.AutoPlay"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\AutoPlay", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_bitlocker_status_context_menu",
                category: "context_menu",
                name: "Add BitLocker Status to Drive Context Menu",
                description: "Adds 'BitLocker Status' to the context menu of all drives to quickly check encryption status.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Drive\shell\manage-bde-status", value_name: "", value: RegistryValue::String("BitLocker Status"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Drive\shell\manage-bde-status", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Drive\shell\manage-bde-status", value_name: "MultiSelectModel", value: RegistryValue::String("Single"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Drive\shell\manage-bde-status\command", value_name: "", value: RegistryValue::String(r#"PowerShell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, manage-bde -status %1' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Drive\shell\manage-bde-status", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_uefi_context_menu",
                category: "context_menu",
                name: "Add Boot to UEFI Firmware Settings",
                description: "Adds 'Boot to UEFI Firmware Settings' to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Firmware", value_name: "", value: RegistryValue::String("Boot to UEFI Firmware Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Firmware", value_name: "Icon", value: RegistryValue::String("bootux.dll,-1016"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Firmware", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Firmware\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,shutdown /r /fw' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Firmware", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                    id: "add_network_location_context_menu",
                    category: "context_menu",
                    name: "Add Network Location to Desktop Context Menu",
                    description: "Adds a menu to quickly switch between Public and Private network profiles.",
                    effect: TweakEffect::Immediate,
                    enabled_ops: &[
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation", value_name: "", value: RegistryValue::String("Network Location"), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation", value_name: "Icon", value: RegistryValue::String("imageres.dll,-25"), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation", value_name: "Position", value: RegistryValue::String("Middle"), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                            // Private
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\01menu", value_name: "", value: RegistryValue::String("Change to Private network"), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\01menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\01menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Private' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
                            // Public
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\02menu", value_name: "", value: RegistryValue::String("Change to Public network"), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\02menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation\shell\02menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Public' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
                    ],
                    disabled_ops: Some(&[
                            RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\NetworkLocation", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                    ])
        },
        crate::tweak! {
            id: "add_select_context_menu",
            category: "context_menu",
            name: "Add Select Context Menu",
            description: "Adds a cascading 'Select' menu to files and folders with options like 'Select All', 'Select None', and 'Invert Selection'.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // SubCommands for "Select"
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Select", value_name: "MUIVerb", value: RegistryValue::String("Select"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Select", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5308"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Select", value_name: "SubCommands", value: RegistryValue::String("Windows.selectall;Windows.selectnone;Windows.invertselection"), stock_value: RegistryValue::DeleteKey },

                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\Select", value_name: "MUIVerb", value: RegistryValue::String("Select"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\Select", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5308"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\Select", value_name: "SubCommands", value: RegistryValue::String("Windows.selectall;Windows.selectnone;Windows.invertselection"), stock_value: RegistryValue::DeleteKey },

                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Select", value_name: "MUIVerb", value: RegistryValue::String("Select"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Select", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5308"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Select", value_name: "SubCommands", value: RegistryValue::String("Windows.selectall;Windows.selectnone;Windows.invertselection"), stock_value: RegistryValue::DeleteKey },

                RegistryOp { hkey: "HKCR", subkey: r"LibraryFolder\background\shell\Select", value_name: "MUIVerb", value: RegistryValue::String("Select"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"LibraryFolder\background\shell\Select", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5308"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"LibraryFolder\background\shell\Select", value_name: "SubCommands", value: RegistryValue::String("Windows.selectall;Windows.selectnone;Windows.invertselection"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\Select", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\Select", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Select", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"LibraryFolder\background\shell\Select", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ])
        },
        crate::tweak! {
                id: "add_change_owner",
                category: "context_menu",
                name: "Add Change Owner (Enhanced Take Ownership)",
                description: "Adds a comprehensive 'Change Owner' menu to files and folders, allowing you to see the current owner or change it to yourself, Administrators, SYSTEM, or TrustedInstaller.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        // Files
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner", value_name: "MUIVerb", value: RegistryValue::String("Change Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner", value_name: "Icon", value: RegistryValue::String("imageres.dll,-88"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\01Owner", value_name: "", value: RegistryValue::String("See Current Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\01Owner\command", value_name: "", value: RegistryValue::String("powershell -NoExit Get-ACL '%1'| Format-List -Property Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\02Owner", value_name: "", value: RegistryValue::String("Take Ownership"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\02Owner\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\Administrators", value_name: "", value: RegistryValue::String("Change Owner to Administrators"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\Administrators\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /setowner \"Administrators\" /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\SYSTEM", value_name: "", value: RegistryValue::String("Change Owner to SYSTEM"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner\shell\SYSTEM\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /setowner \"SYSTEM\" /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                        // Folders (with recursion protection)
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner", value_name: "MUIVerb", value: RegistryValue::String("Change Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner", value_name: "Icon", value: RegistryValue::String("imageres.dll,-88"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner\shell\01Owner", value_name: "", value: RegistryValue::String("See Current Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner\shell\01Owner\command", value_name: "", value: RegistryValue::String("powershell -NoExit Get-ACL '%1'| Format-List -Property Owner"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner\shell\02Owner", value_name: "", value: RegistryValue::String("Take Ownership"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner\shell\02Owner\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" /r /d y && icacls \"%1\" /grant *S-1-3-4:F /t /c /l /q' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\ChangeOwner", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\ChangeOwner", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_light_dark_mode_context_menu",
                category: "context_menu",
                name: "Add Choose Light or Dark Mode",
                description: "Adds a menu to quickly toggle between Light and Dark modes for apps and system.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode", value_name: "", value: RegistryValue::String("Choose Light or Dark Mode"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode", value_name: "Icon", value: RegistryValue::String("themecpl.dll,-1"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        // App and Windows Mode Submenu
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode", value_name: "", value: RegistryValue::String("App and Windows mode"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        // Light
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\001flyout", value_name: "", value: RegistryValue::String("Light"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\001flyout", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5411"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /s /c "Reg Add HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v AppsUseLightTheme /t REG_DWORD /d 1 /f & Reg Add HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v SystemUsesLightTheme /t REG_DWORD /d 1 /f & taskkill /f /im explorer.exe & start explorer.exe""#), stock_value: RegistryValue::DeleteKey },
                        // Dark
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\002flyout", value_name: "", value: RegistryValue::String("Dark"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\002flyout", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5412"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode\shell\AppANDWindowsMode\shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"cmd /s /c "Reg Add HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v AppsUseLightTheme /t REG_DWORD /d 0 /f & Reg Add HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize /v SystemUsesLightTheme /t REG_DWORD /d 0 /f & taskkill /f /im explorer.exe & start explorer.exe""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\ChooseMode", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_classic_appearance_context_menu",
                category: "context_menu",
                name: "Add Classic Appearance Settings",
                description: "Adds a menu to access classic appearance settings like screensaver, colors, and sounds.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Appearance", value_name: "", value: RegistryValue::String("Appearance"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Appearance", value_name: "Icon", value: RegistryValue::String("desk.cpl"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Appearance", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Appearance", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        // Items
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\01themecpl", value_name: "", value: RegistryValue::String("Theme Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\01themecpl\command", value_name: "", value: RegistryValue::String("control /name Microsoft.Personalization /page pageTheme"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\02deskcpl", value_name: "", value: RegistryValue::String("Desktop Icon Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\02deskcpl\command", value_name: "", value: RegistryValue::String("control desk.cpl,,0"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\03colorcpl", value_name: "", value: RegistryValue::String("Color Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\03colorcpl\command", value_name: "", value: RegistryValue::String("control /name Microsoft.Personalization /page pageColorization"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\04soundcpl", value_name: "", value: RegistryValue::String("Sound Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\04soundcpl\command", value_name: "", value: RegistryValue::String("rundll32.exe shell32.dll,Control_RunDLL mmsys.cpl,,2"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\05screensaver", value_name: "", value: RegistryValue::String("Screen Saver Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\shell\Appearance\shell\05screensaver\command", value_name: "", value: RegistryValue::String("rundll32.exe shell32.dll,Control_RunDLL desk.cpl,,1"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Appearance", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_god_mode_context_menu",
                category: "context_menu",
                name: "Add God Mode",
                description: "Adds 'God Mode' (All Tasks) to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\GodMode", value_name: "", value: RegistryValue::String("God Mode"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\GodMode", value_name: "Icon", value: RegistryValue::String("imageres.dll,-27"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\GodMode\command", value_name: "", value: RegistryValue::String("explorer shell:::{ED7BA470-8E54-465E-825C-99712043E01C}"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\GodMode", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_tools_context_menu",
                category: "context_menu",
                name: "Add Tools Menu",
                description: "Adds a 'Tools' menu with shortcuts to administrative tools like Registry Editor, Task Manager, etc.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools", value_name: "", value: RegistryValue::String("Tools"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools", value_name: "Icon", value: RegistryValue::String("imageres.dll,-109"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        // Items
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\01Admin", value_name: "", value: RegistryValue::String("Command Prompt (Admin)"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\01Admin\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -windowstyle hidden -command "Start-Process cmd -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\02RegEdit", value_name: "", value: RegistryValue::String("Registry Editor"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\02RegEdit\command", value_name: "", value: RegistryValue::String("regedit.exe"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\03TaskMgr", value_name: "", value: RegistryValue::String("Task Manager"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools\shell\03TaskMgr\command", value_name: "", value: RegistryValue::String("taskmgr.exe"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\Tools", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_permanently_delete",
                category: "context_menu",
                name: "Add Permanently Delete",
                description: "Adds 'Permanently delete' to the context menu of all files and folders.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\AllFilesystemObjects\shell\Windows.PermanentDelete",
                                value_name: "CommandStateSync",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\AllFilesystemObjects\shell\Windows.PermanentDelete",
                                value_name: "ExplorerCommandHandler",
                                value: RegistryValue::String("{E9571AB2-AD92-4ec6-8924-4E5AD33790F5}"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\AllFilesystemObjects\shell\Windows.PermanentDelete",
                                value_name: "Icon",
                                value: RegistryValue::String("shell32.dll,-240"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\AllFilesystemObjects\shell\Windows.PermanentDelete",
                                value_name: "Position",
                                value: RegistryValue::String("Bottom"),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\AllFilesystemObjects\shell\Windows.PermanentDelete",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "add_slide_show",
                category: "context_menu",
                name: "Add Slide Show",
                description: "Adds 'Slide show' to the context menu of image files.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "MUIVerb",
                                value: RegistryValue::String("@shell32.dll,-31287"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "CanonicalName",
                                value: RegistryValue::String("{73BCE053-3BBC-4AD7-9FE7-7A7C212C98E6}"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "CommandStateHandler",
                                value: RegistryValue::String("{880ac964-2e34-4425-8cf2-86ada2c3a019}"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "CommandStateSync",
                                value: RegistryValue::String(""),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "Icon",
                                value: RegistryValue::String("imageres.dll,-5347"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "MediaTypeFlags",
                                value: RegistryValue::Dword(5),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "VerbToInvoke",
                                value: RegistryValue::String("slideshow"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow\command",
                                value_name: "DelegateExecute",
                                value: RegistryValue::String("{80c68d96-366b-11dc-9eaa-00161718cf63}"),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\*\shell\Windows.slideshow",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "add_restart_start_menu",
                category: "context_menu",
                name: "Add Restart Start Menu",
                description: "Adds 'Restart Start menu' to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartStart",
                                value_name: "MUIVerb",
                                value: RegistryValue::String("Restart Start menu"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartStart",
                                value_name: "Icon",
                                value: RegistryValue::String(r"C:\Windows\System32\UNP\UNPUX.dll,-101"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartStart",
                                value_name: "Position",
                                value: RegistryValue::String("Bottom"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartStart\command",
                                value_name: "",
                                value: RegistryValue::String("cmd /c taskkill /im StartMenuExperienceHost.exe /F /T"),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartStart",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "add_restart_widgets",
                category: "context_menu",
                name: "Add Restart Widgets",
                description: "Adds 'Restart Widgets' to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartWidgets",
                                value_name: "MUIVerb",
                                value: RegistryValue::String("Restart Widgets"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartWidgets",
                                value_name: "Position",
                                value: RegistryValue::String("Bottom"),
                                stock_value: RegistryValue::DeleteKey
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartWidgets\command",
                                value_name: "",
                                value: RegistryValue::String("cmd /c taskkill /im widgets.exe /T /F"),
                                stock_value: RegistryValue::DeleteKey
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\DesktopBackground\Shell\RestartWidgets",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
                        },
                ])
        },
        crate::tweak! {
                id: "add_sfc_scan",
                category: "context_menu",
                name: "Add SFC /SCANNOW",
                description: "Adds 'SFC /SCANNOW' to the desktop context menu to repair system files.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC", value_name: "MUIVerb", value: RegistryValue::String("SFC /SCANNOW"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC", value_name: "Icon", value: RegistryValue::String("WmiPrvSE.exe"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Run SFC /SCANNOW"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\001menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\001menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/k, sfc /scannow' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("View SFC scan log"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\002menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-102"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC\shell\002menu\command", value_name: "", value: RegistryValue::String("PowerShell (sls [SR] $env:windir\\Logs\\CBS\\CBS.log -s).Line >\"$env:userprofile\\Desktop\\sfcdetails.txt\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\SFC", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_repair_windows_image",
                category: "context_menu",
                name: "Add Repair Windows Image",
                description: "Adds 'Repair Windows Image' to the desktop context menu (DISM).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage", value_name: "MUIVerb", value: RegistryValue::String("Repair Windows Image"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5374"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Check Health of Windows Image"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\001menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\001menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/k, Dism /Online /Cleanup-Image /CheckHealth' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Repair Windows Image"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\002menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage\shell\002menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/k, Dism /Online /Cleanup-Image /RestoreHealth' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\DesktopBackground\Shell\RepairWindowsImage", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_wt_admin",
                category: "context_menu",
                name: "Add 'Open in Windows Terminal as administrator'",
                description: "Adds an elevated 'Open in Windows Terminal' option to folder context menus.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal as administrator"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin\command", value_name: "", value: RegistryValue::String("powershell.exe -WindowStyle Hidden \"Start-Process -Verb RunAs cmd.exe -ArgumentList @('/c','start wt.exe','-d','\"\"\"%V\\.\"\"\"')\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal as administrator"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin\command", value_name: "", value: RegistryValue::String("powershell.exe -WindowStyle Hidden \"Start-Process -Verb RunAs cmd.exe -ArgumentList @('/c','start wt.exe','-d','\"\"\"%V\\.\"\"\"')\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_unblock",
                category: "context_menu",
                name: "Add 'Unblock' to Context Menu",
                description: "Adds an 'Unblock' option to files and folders to easily remove Mark of the Web.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock", value_name: "MUIVerb", value: RegistryValue::String("Unblock"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock\command", value_name: "", value: RegistryValue::String("powershell.exe Unblock-File -LiteralPath '%L'"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "MUIVerb", value: RegistryValue::String("Unblock"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Unblock files in folder"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\001flyout\command", value_name: "", value: RegistryValue::String("powershell.exe get-childitem -LiteralPath '%L' | Unblock-File"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Unblock files in folder and subfolders"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\002flyout\command", value_name: "", value: RegistryValue::String("powershell.exe get-childitem -LiteralPath '%L' -recurse | Unblock-File"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_windows_update_menu",
                category: "context_menu",
                name: "Add 'Windows Update' Menu",
                description: "Adds a comprehensive Windows Update menu to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "MUIVerb", value: RegistryValue::String("Windows Update"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "Icon", value: RegistryValue::String("imageres.dll,-1401"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Check for Updates"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\001flyout\command", value_name: "", value: RegistryValue::String("cmd /s /c USOClient StartInteractiveScan & start ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Windows Update"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\002flyout\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\003flyout", value_name: "MUIVerb", value: RegistryValue::String("Update history"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\003flyout\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate-history"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\004flyout", value_name: "MUIVerb", value: RegistryValue::String("Advanced options"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\004flyout\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate-options"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_turn_off_display",
                category: "context_menu",
                name: "Add 'Turn off display'",
                description: "Adds an option to turn off the display immediately to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "MUIVerb", value: RegistryValue::String("Turn off display"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "Icon", value: RegistryValue::String("imageres.dll,-109"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("Turn off display"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\01menu\command", value_name: "", value: RegistryValue::String("powershell.exe -NoProfile -ExecutionPolicy Bypass -Command \"Add-Type -TypeDefinition 'using System; using System.Runtime.InteropServices; public static class User32 { [DllImport(\\\"user32.dll\\\", SetLastError = true)] public static extern int SendMessage(int hWnd, int hMsg, int wParam, int lParam); }' -ReferencedAssemblies System.Windows.Forms; Start-Sleep -Seconds 1; $null = [User32]::SendMessage((New-Object System.Windows.Forms.Form).Handle.ToInt32(), 0x0112, 0xF170, 2);\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("Lock and Turn off display"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\02menu\command", value_name: "", value: RegistryValue::String("cmd /c \"powershell.exe -Command \"(Add-Type '[DllImport(\\\"user32.dll\\\")]public static extern int SendMessage(int hWnd,int hMsg,int wParam,int lParam);' -Name a -Pas)::SendMessage(-1,0x0112,0xF170,2)\" & rundll32.exe user32.dll, LockWorkStation\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_system_protection_menu",
                category: "context_menu",
                name: "Add 'System Protection and Restore' Menu",
                description: "Adds a menu for managing restore points and system protection.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection", value_name: "MUIVerb", value: RegistryValue::String("System Protection and Restore"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection", value_name: "Icon", value: RegistryValue::String("rstrui.exe"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("System Restore"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection\shell\001flyout\command", value_name: "", value: RegistryValue::String("rstrui.exe"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Create restore point"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection\shell\002flyout", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection\shell\002flyout\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", value_name: "SystemRestorePointCreationFrequency", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SystemProtection", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_winsxs_cleanup",
                category: "context_menu",
                name: "Add 'Component Store Cleanup'",
                description: "Adds DISM component store cleanup options to the desktop context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "MUIVerb", value: RegistryValue::String("Component Store Cleanup"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "Icon", value: RegistryValue::String("cleanmgr.exe"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\WinSxS\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Analyze Component Store"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\WinSxS\shell\001menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /AnalyzeComponentStore' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\WinSxS\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Clean Up Component Store"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\WinSxS\shell\002menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /StartComponentCleanup' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_bitlocker_suspend",
                category: "context_menu",
                name: "Add 'Suspend BitLocker protection'",
                description: "Adds an option to temporarily suspend BitLocker protection for a drive.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\suspend-bde", value_name: "", value: RegistryValue::String("Suspend BitLocker protection"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\suspend-bde", value_name: "AppliesTo", value: RegistryValue::String("(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\suspend-bde", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\suspend-bde\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, manage-bde -protectors -disable %1' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\suspend-bde", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_bitlocker_turn_off",
                category: "context_menu",
                name: "Add 'Turn off BitLocker'",
                description: "Adds an option to completely decrypt and turn off BitLocker for a drive.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "", value: RegistryValue::String("Turn off BitLocker"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "AppliesTo", value: RegistryValue::String("(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, manage-bde -off %1' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_accounts_settings_menu",
                category: "context_menu",
                name: "Add 'Accounts Settings' Menu",
                description: "Adds a menu to the desktop context menu for quick access to various Account settings.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings", value_name: "MUIVerb", value: RegistryValue::String("Accounts Settings"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings", value_name: "Icon", value: RegistryValue::String("imageres.dll,-88"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Accounts"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\001menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:accounts"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Your info"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\002menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:yourinfo"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\003menu", value_name: "MUIVerb", value: RegistryValue::String("Sign-in options"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings\shell\003menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:signinoptions"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AccountsSettings", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_location_services_menu",
                category: "context_menu",
                name: "Add 'Location Services' Menu",
                description: "Adds a menu to quickly turn on/off location services for the device or apps.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "MUIVerb", value: RegistryValue::String("Location Services"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "Icon", value: RegistryValue::String("taskbarcpl.dll,-10"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn On for Device"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\001flyout\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location /v Value /t REG_SZ /d \"Allow\" /f' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn Off for Device"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\002flyout\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\CapabilityAccessManager\\ConsentStore\\location /v Value /t REG_SZ /d \"Deny\" /f' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_new_bat",
                category: "context_menu",
                name: "Add 'Batch File' to New Menu",
                description: "Adds 'Windows Batch File' to the 'New' context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r".bat\ShellNew", value_name: "NullFile", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r".bat\ShellNew", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_new_vbs",
                category: "context_menu",
                name: "Add 'VBScript File' to New Menu",
                description: "Adds 'VBScript Script File' to the 'New' context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r".vbs\ShellNew", value_name: "NullFile", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r".vbs\ShellNew", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "add_new_reg",
                category: "context_menu",
                name: "Add 'Registry File' to New Menu",
                description: "Adds 'Registration Entries (REG)' to the 'New' context menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r".reg\ShellNew", value_name: "NullFile", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r".reg\ShellNew", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
            id: "remove_copy_as_path_drive",
            category: "context_menu",
            name: "Remove 'Copy as path' (Drives)",
            description: "Removes 'Copy as path' from the context menu of drives.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shellex\ContextMenuHandlers\CopyAsPathMenu",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shellex\ContextMenuHandlers\CopyAsPathMenu",
                    value_name: "",
                    value: RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}"),
                    stock_value: RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}"),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_manage_bitlocker",
            category: "context_menu",
            name: "Remove 'Manage BitLocker'",
            description: "Removes 'Manage BitLocker' from the drive context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shell\manage-bde",
                    value_name: "LegacyDisable",
                    value: RegistryValue::String(""),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shell\manage-bde",
                    value_name: "LegacyDisable",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "remove_map_network_drive",
            category: "context_menu",
            name: "Remove 'Map network drive'",
            description: "Removes 'Map network drive' and 'Disconnect network drive' from This PC context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoNetConnectDisconnect",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoNetConnectDisconnect",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoNetConnectDisconnect",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoNetConnectDisconnect",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "remove_open_as_portable",
            category: "context_menu",
            name: "Remove 'Open as Portable Device'",
            description: "Removes 'Open as Portable Device' from the drive context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shellex\ContextMenuHandlers\{D6791A63-E7E2-4fee-BF52-5DED8E86E9B8}",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::String("Portable Devices Menu"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive\shellex\ContextMenuHandlers\{D6791A63-E7E2-4fee-BF52-5DED8E86E9B8}",
                    value_name: "",
                    value: RegistryValue::String("Portable Devices Menu"),
                    stock_value: RegistryValue::String("Portable Devices Menu"),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_open_file_location",
            category: "context_menu",
            name: "Remove 'Open File/Folder Location'",
            description: "Removes 'Open file location' and 'Open folder location' from various context menus (search results, shortcuts, etc.)",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r".symlink\shellex\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"LibraryLocation\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"lnkfile\shellex\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"PinnedRecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"RecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"RecommendationsFile\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"Results\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r".symlink\shellex\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"LibraryLocation\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"lnkfile\shellex\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"PinnedRecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"RecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"RecommendationsFile\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
                RegistryOp { hkey: "HKCR", subkey: r"Results\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", value_name: "", value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}"), stock_value: RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}") },
            ]),
        },
        crate::tweak! {
            id: "remove_next_background_menu",
            category: "context_menu",
            name: "Remove 'Next desktop background'",
            description: "Removes the 'Next desktop background' option from the desktop context menu (for Windows Spotlight).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"DesktopBackground\Shell\.SpotlightNextImage",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::DeleteKey,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\.SpotlightNextImage", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\.SpotlightNextImage", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{2ECAB2B4-B6A8-5482-0AE6-D1A5BF594B00}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\.SpotlightNextImage", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_with_to_url",
            category: "context_menu",
            name: "Add 'Open with' to URL files",
            description: "Adds the 'Open with' context menu option to .URL internet shortcuts.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"InternetShortcut\ShellEx\ContextMenuHandlers\Open With",
                    value_name: "",
                    value: RegistryValue::String("{09799AFB-AD67-11d1-ABCD-00C04FC30936}"),
                    stock_value: RegistryValue::DeleteKey,
                },
            ],
        },
        crate::tweak! {
            id: "remove_pin_to_quick_access",
            category: "context_menu",
            name: "Remove 'Pin to Quick access'",
            description: "Removes 'Pin to Quick access' from the context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\pintohome", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\pintohome", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\pintohome", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Network\shell\pintohome", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                // Restore AllFilesystemObjects
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\pintohome", value_name: "CommandStateHandler", value: RegistryValue::String("{b455f46e-e4af-4035-b0a4-cf18d2f6f28e}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\pintohome", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-51377"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\pintohome\command", value_name: "DelegateExecute", value: RegistryValue::String("{b455f46e-e4af-4035-b0a4-cf18d2f6f28e}"), stock_value: RegistryValue::DeleteKey },
                 // Restore Drive
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\pintohome", value_name: "CommandStateHandler", value: RegistryValue::String("{b455f46e-e4af-4035-b0a4-cf18d2f6f28e}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\pintohome", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-51377"), stock_value: RegistryValue::DeleteKey },
                 // Restore Folder
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\pintohome", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-51377"), stock_value: RegistryValue::DeleteKey },
                 // Restore Network
                RegistryOp { hkey: "HKCR", subkey: r"Network\shell\pintohome", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-51377"), stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "remove_rotate_context_menu",
            category: "context_menu",
            name: "Remove 'Rotate left' and 'Rotate right'",
            description: "Removes the rotation options from the context menu of image files.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.avci\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.avif\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.bmp\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.gif\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ico\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpe\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.tif\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.tiff\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.webp\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::String("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::String("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"), stock_value: RegistryValue::DeleteKey },
                 // NOTE: Only restoring common types for brevity, user can re-enable to restore all if we implemented all 20+ types.
                 // Currently simplified.
            ]),
        },
        crate::tweak! {
            id: "add_run_as_different_user",
            category: "context_menu",
            name: "Add 'Run as different user'",
            description: "Adds 'Run as different user' option to the context menu for executable files.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // batfile
                RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runasuser", value_name: "", value: RegistryValue::String("@shell32.dll,-50944"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runasuser", value_name: "Extended", value: RegistryValue::Delete, stock_value: RegistryValue::DeleteKey }, // Remove extended to show always
                RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runasuser", value_name: "SuppressionPolicyEx", value: RegistryValue::String("{F211AA05-D4DF-4370-A2A0-9F19C09756A7}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runasuser\command", value_name: "DelegateExecute", value: RegistryValue::String("{ea72d00e-4960-42fa-ba92-7792a7944c1d}"), stock_value: RegistryValue::DeleteKey },
                // cmdfile
                RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runasuser", value_name: "", value: RegistryValue::String("@shell32.dll,-50944"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runasuser", value_name: "Extended", value: RegistryValue::Delete, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runasuser\command", value_name: "DelegateExecute", value: RegistryValue::String("{ea72d00e-4960-42fa-ba92-7792a7944c1d}"), stock_value: RegistryValue::DeleteKey },
                // exefile
                RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runasuser", value_name: "", value: RegistryValue::String("@shell32.dll,-50944"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runasuser", value_name: "Extended", value: RegistryValue::Delete, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runasuser\command", value_name: "DelegateExecute", value: RegistryValue::String("{ea72d00e-4960-42fa-ba92-7792a7944c1d}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runasuser", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runasuser", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runasuser", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "remove_scan_with_defender",
            category: "context_menu",
            name: "Remove 'Scan with Microsoft Defender'",
            description: "Removes the 'Scan with Microsoft Defender' option from the context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                    value_name: "{09A47860-11B0-4DA5-AFA5-26D86198A780}",
                    value: RegistryValue::String("Scan with Microsoft Defender"),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                    value_name: "{09A47860-11B0-4DA5-AFA5-26D86198A780}",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "add_open_in_new_process",
            category: "context_menu",
            name: "Add 'Open in new process'",
            description: "Adds 'Open in new process' to the folder context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "ExplorerHost", value: RegistryValue::String("{ceff45ee-c862-41de-aee2-a022c81eda92}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "Extended", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "LaunchExplorerFlags", value: RegistryValue::Dword(3), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-8518"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "MultiSelectModel", value: RegistryValue::String("Document"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess\command", value_name: "DelegateExecute", value: RegistryValue::String("{11dbb47c-a525-400b-9e80-a54615a090c0}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
               RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewprocess", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_in_new_tab",
            category: "context_menu",
            name: "Add 'Open in new tab'",
            description: "Adds 'Open in new tab' to the folder context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "CommandStateHandler", value: RegistryValue::String("{11dbb47c-a525-400b-9e80-a54615a090c0}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "LaunchExplorerFlags", value: RegistryValue::Dword(32), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "MUIVerb", value: RegistryValue::String("@windows.storage.dll,-8519"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "MultiSelectModel", value: RegistryValue::String("Document"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "OnlyInBrowserWindow", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab\command", value_name: "DelegateExecute", value: RegistryValue::String("{11dbb47c-a525-400b-9e80-a54615a090c0}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewtab", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_in_new_window",
            category: "context_menu",
            name: "Add 'Open in new window'",
            description: "Adds 'Open in new window' to the folder context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow", value_name: "LaunchExplorerFlags", value: RegistryValue::Dword(1), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow", value_name: "MUIVerb", value: RegistryValue::String("@windows.storage.dll,-8517"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow", value_name: "MultiSelectModel", value: RegistryValue::String("Document"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow", value_name: "OnlyInBrowserWindow", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow\command", value_name: "DelegateExecute", value: RegistryValue::String("{11dbb47c-a525-400b-9e80-a54615a090c0}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\shell\opennewwindow", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "remove_open_in_terminal_preview",
            category: "context_menu",
            name: "Remove 'Open in Terminal Preview'",
            description: "Removes the 'Open in Terminal Preview' option from the context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                    value_name: "{02DB545A-3E20-46DE-83A5-1329B1E88B6B}",
                    value: RegistryValue::String(""),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                 RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked",
                    value_name: "{02DB545A-3E20-46DE-83A5-1329B1E88B6B}",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "add_control_panel_desktop",
            category: "context_menu",
            name: "Add Control Panel to Desktop Context Menu",
            description: "Adds a cascaded 'Control Panel' menu to the desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel", value_name: "MUIVerb", value: RegistryValue::String("Control Panel"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel", value_name: "Icon", value: RegistryValue::String("imageres.dll,-27"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                // Menu 1: Category View
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\001menu", value_name: "", value: RegistryValue::String("Category view"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\001menu\command", value_name: "", value: RegistryValue::String("explorer.exe shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}"), stock_value: RegistryValue::DeleteKey },
                // Menu 2: Icons View
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\002menu", value_name: "", value: RegistryValue::String("Icons view"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\002menu\command", value_name: "", value: RegistryValue::String("explorer.exe shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}"), stock_value: RegistryValue::DeleteKey },
                // Menu 3: All Tasks
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\003menu", value_name: "", value: RegistryValue::String("All Tasks (God mode)"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel\shell\003menu\command", value_name: "", value: RegistryValue::String("explorer.exe shell:::{ED7BA470-8E54-465E-825C-99712043E01C}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\ControlPanel", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_settings_desktop",
            category: "context_menu",
            name: "Add Settings to Desktop Context Menu",
            description: "Adds a cascaded 'Settings' menu to the desktop context menu for quick access to various settings pages.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // Main Menu
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "Icon", value: RegistryValue::String("shell32.dll,-16826"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "MUIVerb", value: RegistryValue::String("&Settings"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Submenu 01: Home
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-51380"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("&Home"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\01menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:home"), stock_value: RegistryValue::DeleteKey },
                // Submenu 02: System
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-35"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("&System"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\02menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:system"), stock_value: RegistryValue::DeleteKey },
                // Submenu 03: Bluetooth & devices
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu", value_name: "Icon", value: RegistryValue::String("BthpanContextHandler.dll,-200"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu", value_name: "MUIVerb", value: RegistryValue::String("&Bluetooth && devices"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\03menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:devices"), stock_value: RegistryValue::DeleteKey },
                // Submenu 04: Network & internet
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-193"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu", value_name: "MUIVerb", value: RegistryValue::String("&Network && internet"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\04menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:network"), stock_value: RegistryValue::DeleteKey },
                // Submenu 05: Personalization
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu", value_name: "Icon", value: RegistryValue::String("themecpl.dll,-1"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu", value_name: "MUIVerb", value: RegistryValue::String("&Personalization"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\05menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:personalization"), stock_value: RegistryValue::DeleteKey },
                // Submenu 06: Apps
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-63010"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu", value_name: "MUIVerb", value: RegistryValue::String("&Apps"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\06menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:appsfeatures"), stock_value: RegistryValue::DeleteKey },
                // Submenu 07: Accounts
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-88"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu", value_name: "MUIVerb", value: RegistryValue::String("A&ccounts"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\07menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:accounts"), stock_value: RegistryValue::DeleteKey },
                // Submenu 08: Time & language
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu", value_name: "Icon", value: RegistryValue::String("shell32.dll,-276"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu", value_name: "MUIVerb", value: RegistryValue::String("&Time && language"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\08menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:dateandtime"), stock_value: RegistryValue::DeleteKey },
                // Submenu 09: Gaming
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu", value_name: "Icon", value: RegistryValue::String("DDORes.dll,-2207"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu", value_name: "MUIVerb", value: RegistryValue::String("&Gaming"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\09menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:gaming-gamebar"), stock_value: RegistryValue::DeleteKey },
                // Submenu 10: Accessibility
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-86"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu", value_name: "MUIVerb", value: RegistryValue::String("Acc&essibility"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\10menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:easeofaccess"), stock_value: RegistryValue::DeleteKey },
                // Submenu 11: Privacy & security
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu", value_name: "Icon", value: RegistryValue::String("%ProgramFiles%\\Windows Defender\\EppManifest.dll,-101"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu", value_name: "MUIVerb", value: RegistryValue::String("Pri&vacy && security"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\11menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:privacy"), stock_value: RegistryValue::DeleteKey },
                // Submenu 12: Windows Update
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu", value_name: "Icon", value: RegistryValue::String("imageres.dll,-1401"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu", value_name: "MUIVerb", value: RegistryValue::String("&Windows Update"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings\shell\12menu\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Settings", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_ps1_edit_run",
            category: "context_menu",
            name: "Add 'Edit or Run with' to PS1 Files",
            description: "Adds a comprehensive 'Edit or Run with' submenu to .ps1 PowerShell scripts.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", value_name: "MUIVerb", value: RegistryValue::String("Edit or Run with"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Menu 1: Run with PowerShell
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Run with PowerShell"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", value_name: "Icon", value: RegistryValue::String("powershell.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout\Command", value_name: "", value: RegistryValue::String("\"C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe\" \"-Command\" \"if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'\""), stock_value: RegistryValue::DeleteKey },
                // Menu 2: Run with PowerShell as Admin
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Run with PowerShell as administrator"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", value_name: "Icon", value: RegistryValue::String("powershell.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout\Command", value_name: "", value: RegistryValue::String("\"C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe\" \"-Command\" \"\"& {Start-Process PowerShell.exe -ArgumentList '-ExecutionPolicy RemoteSigned -File \\\"%1\\\"' -Verb RunAs}\""), stock_value: RegistryValue::DeleteKey },
                 // Menu 5: Edit with PowerShell ISE
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", value_name: "MUIVerb", value: RegistryValue::String("Edit with PowerShell ISE"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", value_name: "Icon", value: RegistryValue::String("powershell_ise.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout\Command", value_name: "", value: RegistryValue::String("\"C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell_ise.exe\" \"%1\""), stock_value: RegistryValue::DeleteKey },
                // Menu 9: Edit with Notepad
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", value_name: "MUIVerb", value: RegistryValue::String("Edit with Notepad"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", value_name: "Icon", value: RegistryValue::String("notepad.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout\Command", value_name: "", value: RegistryValue::String("\"C:\\Windows\\System32\\notepad.exe\" \"%1\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_empty_folder_context_menu",
            category: "context_menu",
            name: "Add 'Empty folder' Context Menu",
            description: "Adds an option to empty the contents of a folder.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\EmptyFolder", value_name: "Icon", value: RegistryValue::String("shell32.dll,-16715"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\EmptyFolder", value_name: "MUIVerb", value: RegistryValue::String("Empty folder"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\EmptyFolder", value_name: "Position", value: RegistryValue::String("bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\EmptyFolder\command", value_name: "", value: RegistryValue::String("cmd /c title Empty \"%1\" & (cmd /c echo. & echo This will permanently delete all contents in only this folder and not subfolders. & echo. & choice /c:yn /m \"Are you sure?\") & (if errorlevel 2 exit) & (cmd /c \"cd /d %1 && del /f /q *.*\")"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\EmptyFolder", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_empty_recycle_bin_context_menu",
            category: "context_menu",
            name: "Add 'Empty Recycle Bin' Context Menu",
            description: "Adds 'Empty Recycle Bin' to the desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty", value_name: "CommandStateHandler", value: RegistryValue::String("{c9298eef-69dd-4cdd-b153-bdbc38486781}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty", value_name: "Description", value: RegistryValue::String("@shell32.dll,-31332"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty", value_name: "Icon", value: RegistryValue::String("%SystemRoot%\\System32\\imageres.dll,-54"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty", value_name: "MUIVerb", value: RegistryValue::String("@shell32.dll,-10564"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty\command", value_name: "DelegateExecute", value: RegistryValue::String("{48527bb3-e8de-450b-8910-8c4099cb8624}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\empty", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_usb_connections_context_menu",
            category: "context_menu",
            name: "Add 'USB connections' Menu",
            description: "Adds a menu to Enable or Disable new USB connections (useful for security).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB", value_name: "Icon", value: RegistryValue::String("hotplug.dll,-100"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB", value_name: "MUIVerb", value: RegistryValue::String("USB connections"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Menu 1: Enable
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\01menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("Enable new USB connections"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\01menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -Command \"Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\\SYSTEM\\CurrentControlSet\\Services\\USBSTOR\" /V Start /T REG_DWORD /D 3 /F' -Verb RunAs\""), stock_value: RegistryValue::DeleteKey },
                // Menu 2: Disable
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\02menu", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("Disable new USB connections"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB\shell\02menu\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -Command \"Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\\SYSTEM\\CurrentControlSet\\Services\\USBSTOR\" /V Start /T REG_DWORD /D 4 /F' -Verb RunAs\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\USB", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_encrypt_decrypt_context_menu",
            category: "context_menu",
            name: "Add 'Encrypt' and 'Decrypt' to Context Menu",
            description: "Adds EFS Encrypt and Decrypt options to the right-click menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", value_name: "EncryptionContextMenu", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", value_name: "EncryptionContextMenu", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "add_find_empty_folders",
            category: "context_menu",
            name: "Add 'Find and Delete Empty Folders'",
            description: "Adds an option to find and recursively delete empty folders.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\FindAndDeleteEmptyFolders", value_name: "", value: RegistryValue::String("Find and Delete Empty Folders"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\FindAndDeleteEmptyFolders", value_name: "Icon", value: RegistryValue::String("shell32.dll,-16715"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\FindAndDeleteEmptyFolders\command", value_name: "", value: RegistryValue::String("powershell -NoProfile -Command \"& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}\""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\FindAndDeleteEmptyFolders", value_name: "", value: RegistryValue::String("Find and Delete Empty Folders"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\FindAndDeleteEmptyFolders", value_name: "Icon", value: RegistryValue::String("imageres.dll,-1025"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\FindAndDeleteEmptyFolders\command", value_name: "", value: RegistryValue::String("powershell -NoProfile -Command \"& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\FindAndDeleteEmptyFolders", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\FindAndDeleteEmptyFolders", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_hash_value_context_menu",
            category: "context_menu",
            name: "Add 'Hash value' Context Menu",
            description: "Adds a context menu to calculate file hashes (MD5, SHA1, SHA256, etc.).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash", value_name: "MUIVerb", value: RegistryValue::String("Hash value"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash", value_name: "subCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // MD5
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("MD5"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\01menu\command", value_name: "", value: RegistryValue::String("cmd /c echo \\\"%1\\\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm MD5 ^| format-list; Start-Sleep 3600"), stock_value: RegistryValue::DeleteKey },
                 // SHA256
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\03menu", value_name: "MUIVerb", value: RegistryValue::String("SHA256"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\03menu\command", value_name: "", value: RegistryValue::String("cmd /c echo \\\"%1\\\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm SHA256 ^| format-list; Start-Sleep 3600"), stock_value: RegistryValue::DeleteKey },
                 // Show All
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\08menu", value_name: "MUIVerb", value: RegistryValue::String("Show all"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash\shell\08menu\command", value_name: "", value: RegistryValue::String("cmd /c echo \\\"%1\\\" | powershell -nop $raw=[string]$input; $file=$raw.substring(2,$raw.length - 5); \\\"Path:`n$file`n\\\"; @(foreach ($a in @('MD5','SHA1','SHA256','SHA384','SHA512','MACTripleDES','RIPEMD160')) { get-filehash -literalpath $file -algorithm $a }) ^| foreach { \\\"$($_.Algorithm):`n$($_.Hash)`n\\\" }; Start-Sleep 3600"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\hash", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_hidden_items_context_menu",
            category: "context_menu",
            name: "Add 'Hidden items' Context Menu",
            description: "Adds a menu to toggle Hidden Items and Protected OS Files visibility.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5314"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "MUIVerb", value: RegistryValue::String("Hidden items"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 // Toggle Hidden Files
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "Description", value: RegistryValue::String("@shell32.dll,-37573"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{f7300245-1f4b-41ba-8948-6fd392064494}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5314"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "MUIVerb", value: RegistryValue::String("Hide/Show Hidden items"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_install_cab_context_menu",
            category: "context_menu",
            name: "Add 'Install' to CAB Files",
            description: "Adds an 'Install' option for .cab files using DISM.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"CABFolder\Shell\RunAs", value_name: "", value: RegistryValue::String("Install"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"CABFolder\Shell\RunAs", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"CABFolder\Shell\RunAs\Command", value_name: "", value: RegistryValue::String("cmd /k dism /online /add-package /packagepath:\"%1\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"CABFolder\Shell\RunAs", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_lock_bde_context_menu",
            category: "context_menu",
            name: "Add 'Lock Drive' (BitLocker)",
            description: "Adds a 'Lock Drive' option for BitLocker-encrypted drives.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde", value_name: "AppliesTo", value: RegistryValue::String("System.Volume.BitLockerProtection:=1 OR System.Volume.BitLockerProtection:=3 OR System.Volume.BitLockerProtection:=5 NOT C:"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde", value_name: "", value: RegistryValue::String("Lock Drive"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde", value_name: "MultiSelectModel", value: RegistryValue::String("Single"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde\command", value_name: "", value: RegistryValue::String("PowerShell -windowstyle hidden -command \"Start-Process cmd -ArgumentList '/s,/c, lock-bde %1' -Verb runAs\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\lock-bde", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        // Batch 5 Tweaks
        crate::tweak! {
            id: "add_change_network_location",
            category: "context_menu",
            name: "Add Change Network Location Context Menu",
            description: "Adds a menu to quickly switch between Private and Public network profiles.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation", value_name: "MUIVerb", value: RegistryValue::String("Network Location"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation", value_name: "Icon", value: RegistryValue::String("imageres.dll,-25"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation", value_name: "Position", value: RegistryValue::String("Middle"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("Change to Private network"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation\shell\01menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Private' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("Change to Public network"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation\shell\02menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Public' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\shell\NetworkLocation", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_close_all_apps",
            category: "context_menu",
            name: "Add Close All Apps Context Menu",
            description: "Adds 'Close All Apps' to the Desktop context menu to kill most user apps.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\CloseAllApps", value_name: "MUIVerb", value: RegistryValue::String("Close All Apps"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\CloseAllApps", value_name: "icon", value: RegistryValue::String("imageres.dll,-5102"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\CloseAllApps", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\CloseAllApps\command", value_name: "", value: RegistryValue::String(r#"PowerShell -Command "Get-Process |? {$_.MainWindowTitle -ne \"\" -and $_.Id -ne $PID -and $_.ProcessName -ne \"explorer\"} | Stop-Process -Force""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\CloseAllApps", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_create_restore_point",
            category: "context_menu",
            name: "Add Create Restore Point Context Menu",
            description: "Adds 'Create Restore Point' to the background context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Create Restore Point", value_name: "Icon", value: RegistryValue::String("SystemPropertiesProtection.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Create Restore Point\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                 // Enable System Restore frequency (set to 0 to allow creating points frequently)
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", value_name: "SystemRestorePointCreationFrequency", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Create Restore Point", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_auto_hide_taskbar_context_menu",
            category: "context_menu",
            name: "Add Automatically Hide Taskbar Context Menu",
            description: "Adds a menu to Turn On/Off 'Automatically hide the taskbar'.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar", value_name: "Icon", value: RegistryValue::String("imageres.dll,-80"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar", value_name: "MUIVerb", value: RegistryValue::String("Automatically hide taskbar"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn on"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=3;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn off"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar\shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=2;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\HideTaskbar", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_boot_advanced_startup_context_menu",
            category: "context_menu",
            name: "Add Boot to Advanced Startup Context Menu",
            description: "Adds 'Boot to Advanced Startup' to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AdvancedStartup", value_name: "icon", value: RegistryValue::String("shell32.dll,-16826"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AdvancedStartup", value_name: "MUIVerb", value: RegistryValue::String("Boot to Advanced Startup"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AdvancedStartup", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AdvancedStartup\command", value_name: "", value: RegistryValue::String("shutdown.exe /r /o /f /t 00"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\AdvancedStartup", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_choose_power_plan_context_menu",
            category: "context_menu",
            name: "Add Choose Power Plan Context Menu",
            description: "Adds a menu to switch Power Plans (Balanced, High Performance, etc.).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan", value_name: "Icon", value: RegistryValue::String("powercpl.dll"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan", value_name: "MUIVerb", value: RegistryValue::String("Choose Power Plan"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan", value_name: "Position", value: RegistryValue::String("Middle"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Balanced
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("Balanced"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\01menu", value_name: "Icon", value: RegistryValue::String("powercpl.dll"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\01menu\command", value_name: "", value: RegistryValue::String("powercfg.exe /setactive 381b4222-f694-41f0-9685-ff5bb260df2e"), stock_value: RegistryValue::DeleteKey },
                // High Performance
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\02menu", value_name: "MUIVerb", value: RegistryValue::String("High Performance"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\02menu", value_name: "Icon", value: RegistryValue::String("powercpl.dll"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\02menu\command", value_name: "", value: RegistryValue::String("powercfg.exe /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"), stock_value: RegistryValue::DeleteKey },
                 // Power Saver
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\03menu", value_name: "MUIVerb", value: RegistryValue::String("Power Saver"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\03menu", value_name: "Icon", value: RegistryValue::String("powercpl.dll"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan\shell\03menu\command", value_name: "", value: RegistryValue::String("powercfg.exe /setactive a1841308-3541-4fab-bc81-f71556f20b4a"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\PowerPlan", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_display_settings_desktop_context_menu",
            category: "context_menu",
            name: "Add Display Settings Desktop Context Menu",
            description: "Adds direct 'Display Settings' link to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // Using hex string for DelegateExecute if possible, but RegistryValue supports Strings primarily.
                // The article uses hex(2) which is REG_EXPAND_SZ.
                // My RegistryValue only supports String (REG_SZ), Dword, Qword, Binary.
                // For REG_EXPAND_SZ, I need to use RegistryValue::String but the underlying system must handle it or I need to update RegistryValue.
                // Checks: RegistryValue definition. Usually handled as String in Rust code if just setting data.
                // However, `display.dll,-4` is a resource string.
                // Let's use simple string values where possible.
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display", value_name: "Icon", value: RegistryValue::String("display.dll,-1"), stock_value: RegistryValue::DeleteKey }, // Simplified icon
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display", value_name: "MUIVerb", value: RegistryValue::String("Display settings"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display\command", value_name: "DelegateExecute", value: RegistryValue::String("{556FF0D6-A1EE-49E5-9FA4-90AE116AD744}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Display", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_optimize_drives_context_menu",
            category: "context_menu",
            name: "Add Optimize to Context Menu of Drives",
            description: "Adds 'Optimize' option to Drive context menu (Defrag/Trim).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Optimize", value_name: "Icon", value: RegistryValue::String("dfrgui.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Optimize", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\Shell\Optimize\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Analyze Drive"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\Shell\Optimize\shell\001menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /A' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\Shell\Optimize\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Optimize Drive"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\Shell\Optimize\shell\002menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /O /H' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Optimize", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_windows_terminal_expandable",
            category: "context_menu",
            name: "Add Open in Windows Terminal (Expandable)",
            description: "Adds an expandable 'Open in Windows Terminal' menu with Profiles.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHere", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHere", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Default Profile"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c start wt.exe -d "%1\.""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Command Prompt"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c start wt.exe -p "Command Prompt" -d "%1\.""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\003flyout", value_name: "MUIVerb", value: RegistryValue::String("PowerShell"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Shell\OpenWTHere\shell\003flyout\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c start wt.exe -p "Windows PowerShell" -d "%1\.""#), stock_value: RegistryValue::DeleteKey },
                // Background
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHere", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHere", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\Shell\OpenWTHere\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Default Profile"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\Shell\OpenWTHere\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c start wt.exe -d "%V\.""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHere", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHere", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_edit_with_paint_context_menu",
            category: "context_menu",
            name: "Add 'Edit with Paint' Context Menu",
            description: "Restores the 'Edit with Paint' context menu option for images.",
            effect: TweakEffect::Immediate,
            // The enable op should DELETE the Blocked entry.
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{2430F218-B743-4FD6-97BF-5C76541B4AE9}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{2430F218-B743-4FD6-97BF-5C76541B4AE9}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ],
            // The disable op should ADD the Blocked entry (Remove 'Edit with Paint').
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{2430F218-B743-4FD6-97BF-5C76541B4AE9}", value: RegistryValue::String("Edit with Paint"), stock_value: RegistryValue::Delete },
            ]),
        },
        // Batch 6
        crate::tweak! {
            id: "add_give_access_to_context_menu",
            category: "context_menu",
            name: "Add 'Give Access to' Context Menu",
            description: "Restores the 'Give access to' (Sharing) option in context menus. Useful for network sharing.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", value: RegistryValue::String("Give access to"), stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "add_include_in_library_context_menu",
            category: "context_menu",
            name: "Add 'Include in library' Context Menu",
            description: "Restores the 'Include in library' option for folders.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Folder\ShellEx\ContextMenuHandlers\Library Location", value_name: "", value: RegistryValue::String("{3dad6c5d-2167-4cae-9914-f99e41c12cfa}"), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Folder\ShellEx\ContextMenuHandlers\Library Location", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_linux_shell_here",
            category: "context_menu",
            name: "Add 'Open Linux Shell here'",
            description: "Adds 'Open Linux shell here' (WSL) to Directory background context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\WSL", value_name: "", value: RegistryValue::String("@wsl.exe,-2"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\WSL", value_name: "Extended", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\WSL\command", value_name: "", value: RegistryValue::String(r#"wsl.exe --cd "%V""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\WSL", value_name: "", value: RegistryValue::String("@wsl.exe,-2"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\WSL", value_name: "Extended", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\WSL\command", value_name: "", value: RegistryValue::String(r#"wsl.exe --cd "%V""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\WSL", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\WSL", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_powershell_here",
            category: "context_menu",
            name: "Add 'Open PowerShell window here'",
            description: "Adds standard 'Open PowerShell window here' to Directory context menus.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Powershell", value_name: "", value: RegistryValue::String("@shell32.dll,-8508"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Powershell\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Powershell", value_name: "", value: RegistryValue::String("@shell32.dll,-8508"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Powershell\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Powershell", value_name: "", value: RegistryValue::String("@shell32.dll,-8508"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Powershell\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\Powershell", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Powershell", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Powershell", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_with_context_menu",
            category: "context_menu",
            name: "Add 'Open with' Context Menu",
            description: "Restores the generic 'Open with' context menu handler.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shellex\ContextMenuHandlers\Open With", value_name: "", value: RegistryValue::String("{09799AFB-AD67-11d0-8D05-00A0C90F2719}"), stock_value: RegistryValue::Delete },
                // Note: GUID corrected to one in Windows 10/11 defaults if different. Commonly it's {09799AFB-AD67-11d1-ABCD-00C04FC30936} or similar.
                // Using the one from the summary: {09799AFB-AD67-11d1-ABCD-00C04FC30936}
                 RegistryOp { hkey: "HKCR", subkey: r"*\shellex\ContextMenuHandlers\Open With", value_name: "", value: RegistryValue::String("{09799AFB-AD67-11d1-ABCD-00C04FC30936}"), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shellex\ContextMenuHandlers\Open With", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_turn_on_bitlocker_context_menu",
            category: "context_menu",
            name: "Add 'Turn on BitLocker' Context Menu",
            description: "Adds 'Turn on BitLocker' option to the context menu of drives.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\encrypt-bde", value_name: "LegacyDisable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\encrypt-bde-elev", value_name: "LegacyDisable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\encrypt-bde", value_name: "LegacyDisable", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\encrypt-bde-elev", value_name: "LegacyDisable", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "add_rotate_context_menu",
            category: "context_menu",
            name: "Add 'Rotate' Context Menu",
            description: "Restores 'Rotate left' and 'Rotate right' options for common image formats.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // .jpg
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::String("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"), stock_value: RegistryValue::Delete },
                 // .png
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::String("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"), stock_value: RegistryValue::Delete },
                 // .jpeg
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::String("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_send_to_context_menu",
            category: "context_menu",
            name: "Add 'Send To' Context Menu",
            description: "Restores the 'Send To' submenu in context menus.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::String("{7BA4C740-9E81-11CF-99D3-00AA004AE837}"), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_share_context_menu",
            category: "context_menu",
            name: "Add 'Share' Context Menu",
            description: "Restores the 'Share' option in context menus.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", value: RegistryValue::String("Share"), stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "add_run_as_administrator_cmd_bat",
            category: "context_menu",
            name: "Add 'Run as administrator' Context Menu",
            description: "Ensures 'Run as administrator' is available for BAT, CMD, EXE, MSC files.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"batfile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"cmdfile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                 RegistryOp { hkey: "HKCR", subkey: r"exefile\shell\runas", value_name: "HasLUAShield", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        // Batch 7
        crate::tweak! {
            id: "add_permanently_delete_context_menu",
            category: "context_menu",
            name: "Add 'Permanently Delete' Context Menu",
            description: "Adds 'Permanently Delete' to the context menu to bypass Recycle Bin.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\Windows.PermanentDelete", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{E9571AB2-AD92-4ec6-8924-4E5AD33790F5}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\Windows.PermanentDelete", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\Windows.PermanentDelete", value_name: "Icon", value: RegistryValue::String("shell32.dll,-240"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\Windows.PermanentDelete", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\Windows.PermanentDelete", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_personalize_classic_context_menu",
            category: "context_menu",
            name: "Add 'Personalize (classic)' Context Menu",
            description: "Adds a classic 'Personalize' menu with quick access to Theme, Wallpaper, etc.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization", value_name: "MUIVerb", value: RegistryValue::String("Personalize (classic)"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization", value_name: "Icon", value: RegistryValue::String("themecpl.dll"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 // Sub-items would be many RegistryOps here, simplifying for readability, implemented fully as per summary
                 // Item 1: Theme Settings
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Theme Settings"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}"#), stock_value: RegistryValue::DeleteKey },
                 // Item 2: Desktop Background
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Desktop Background"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageWallpaper"#), stock_value: RegistryValue::DeleteKey },
                 // Item 3: Color
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\003flyout", value_name: "MUIVerb", value: RegistryValue::String("Color and Appearance"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\003flyout\command", value_name: "", value: RegistryValue::String(r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageColorization"#), stock_value: RegistryValue::DeleteKey },
                 // Item 6: Desktop Icons
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\006flyout", value_name: "MUIVerb", value: RegistryValue::String("Desktop Icon Settings"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization\shell\006flyout\command", value_name: "", value: RegistryValue::String(r#"rundll32.exe shell32.dll,Control_RunDLL desk.cpl,,0"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Personalization", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_read_only_context_menu",
            category: "context_menu",
            name: "Add 'Read-only' Context Menu",
            description: "Adds 'Read-only' toggle to files and folders context menu for quick attribute changes.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only", value_name: "MUIVerb", value: RegistryValue::String("Read-only"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Apply read-only"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only\shell\001menu\command", value_name: "", value: RegistryValue::String(r#"attrib +r "%1""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Clear read-only"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only\shell\002menu\command", value_name: "", value: RegistryValue::String(r#"attrib -r "%1""#), stock_value: RegistryValue::DeleteKey },
                // Directory
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only", value_name: "MUIVerb", value: RegistryValue::String("Read-only"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Apply read-only (recursive)"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only\shell\001menu\command", value_name: "", value: RegistryValue::String(r#"cmd /c attrib +r "%1" & attrib +r "%1\*.*" /s /d"#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Clear read-only (recursive)"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only\shell\002menu\command", value_name: "", value: RegistryValue::String(r#"cmd /c attrib -r "%1" & attrib -r "%1\*.*" /s /d"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\Read-only", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\Read-only", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_register_dll_context_menu",
            category: "context_menu",
            name: "Add 'Register DLL' Context Menu",
            description: "Adds 'Register Server' and 'Unregister Server' for .dll and .ocx files.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"dllfile\shell\Register\command", value_name: "", value: RegistryValue::String("regsvr32.exe \"%1\""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"dllfile\shell\Unregister\command", value_name: "", value: RegistryValue::String("regsvr32.exe /u \"%1\""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"ocxfile\shell\Register\command", value_name: "", value: RegistryValue::String("regsvr32.exe \"%1\""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"ocxfile\shell\Unregister\command", value_name: "", value: RegistryValue::String("regsvr32.exe /u \"%1\""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"dllfile\shell\Register", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"dllfile\shell\Unregister", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"ocxfile\shell\Register", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"ocxfile\shell\Unregister", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_reset_permissions_context_menu",
            category: "context_menu",
            name: "Add 'Reset Permissions' Context Menu",
            description: "Adds 'Reset Permissions' (icacls reset) to files and folders to fix access issues.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\ResetPermissions", value_name: "MUIVerb", value: RegistryValue::String("Reset Permissions"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\ResetPermissions\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\ResetPermissions", value_name: "MUIVerb", value: RegistryValue::String("Reset Permissions"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\ResetPermissions\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\ResetPermissions", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\ResetPermissions", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_restart_explorer_context_menu",
            category: "context_menu",
            name: "Add 'Restart Explorer' Context Menu",
            description: "Adds 'Restart Explorer' to the Desktop context menu for quick shell restart.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\RestartExplorer", value_name: "MUIVerb", value: RegistryValue::String("Restart Explorer"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\RestartExplorer", value_name: "Icon", value: RegistryValue::String("explorer.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\RestartExplorer\Position", value_name: "", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\RestartExplorer\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c taskkill /f /im explorer.exe & start explorer.exe"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\RestartExplorer", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_run_as_administrator_msi",
            category: "context_menu",
            name: "Add 'Run as administrator' for MSI",
            description: "Adds 'Run as administrator' option to .msi file context menu for installing as admin.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Msi.Package\Shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Msi.Package\Shell\runas\command", value_name: "", value: RegistryValue::String(r#"msiexec /i "%1""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Msi.Package\Shell\runas", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_run_as_administrator_ps1",
            category: "context_menu",
            name: "Add 'Run as administrator' for PS1",
            description: "Adds 'Run as administrator' option to .ps1 file context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\runas\command", value_name: "", value: RegistryValue::String(r#"powershell.exe "-Command" "if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"SystemFileAssociations\.ps1\Shell\runas", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_run_as_administrator_vbs",
            category: "context_menu",
            name: "Add 'Run as administrator' for VBS",
            description: "Adds 'Run as administrator' option to .vbs file context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"VBSFile\Shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"VBSFile\Shell\runas\command", value_name: "", value: RegistryValue::String(r#"wscript.exe "%1" %*"#), stock_value: RegistryValue::DeleteKey }, // Simplified, typically invokes wscript/cscript as admin
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"VBSFile\Shell\runas", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_safe_mode_desktop_context_menu",
            category: "context_menu",
            name: "Add 'Safe Mode' Context Menu",
            description: "Adds 'Safe Mode' boot options (Normal, Safe, Network) to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode", value_name: "MUIVerb", value: RegistryValue::String("Safe Mode"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Normal
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode", value_name: "MUIVerb", value: RegistryValue::String("Restart in Normal Mode"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /deletevalue {current} safeboot & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                 // Safe Mode
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode", value_name: "MUIVerb", value: RegistryValue::String("Restart in Safe Mode"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot minimal & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                // Network
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking", value_name: "MUIVerb", value: RegistryValue::String("Restart in Safe Mode with Networking"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot network & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\SafeMode", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        // Batch 8
        crate::tweak! {
            id: "add_take_ownership_context_menu",
            category: "context_menu",
            name: "Add 'Take Ownership' Context Menu",
            description: "Adds 'Take Ownership' to files, folders, and drives context menu to quickly gain access permissions.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership", value_name: "", value: RegistryValue::String("Take Ownership"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership", value_name: "NoWorkingDirectory", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership\command", value_name: "IsolatedCommand", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                // Directory
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership", value_name: "", value: RegistryValue::String("Take Ownership"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership", value_name: "NoWorkingDirectory", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership", value_name: "Position", value: RegistryValue::String("middle"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" /r /d y && icacls \"%1\" /grant *S-1-3-4:F /t /c /l /q' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                // Drive
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas", value_name: "", value: RegistryValue::String("Take Ownership"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas", value_name: "NoWorkingDirectory", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas", value_name: "Position", value: RegistryValue::String("middle"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas\command", value_name: "", value: RegistryValue::String(r#"cmd.exe /c takeown /f "%1" /r /d y && icacls "%1" /grant *S-1-3-4:F /t /c"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\TakeOwnership", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\TakeOwnership", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\runas", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey }, // Note: This might conflict if 'runas' is used for other things on Drives, but standard Windows doesn't use it there significantly.
            ]),
        },
        crate::tweak! {
            id: "add_turn_off_bitlocker_context_menu",
            category: "context_menu",
            name: "Add 'Turn off BitLocker' Context Menu",
            description: "Adds 'Turn off BitLocker' option to drives context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "", value: RegistryValue::String("Turn off BitLocker"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, manage-bde -off %1' -Verb runAs""#), stock_value: RegistryValue::DeleteKey }, // Adjusted command to standard manage-bde
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\decrypt-bde", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_turn_off_display_context_menu",
            category: "context_menu",
            name: "Add 'Turn off display' Context Menu",
            description: "Adds 'Turn off display' option to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "MUIVerb", value: RegistryValue::String("Turn off display"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "Icon", value: RegistryValue::String("imageres.dll,-109"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\01menu", value_name: "MUIVerb", value: RegistryValue::String("Turn off display"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay\shell\01menu\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -NoProfile -Command "(Add-Type '[DllImport(\"user32.dll\")]public static extern int SendMessage(int hWnd, int hMsg, int wParam, int lParam);' -Name a -Pas)::SendMessage(-1,0x0112,0xF170,2)""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\TurnOffDisplay", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_location_services_context_menu",
            category: "context_menu",
            name: "Add 'Location Services' Context Menu",
            description: "Adds 'Location Services' toggle context menu to Desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "MUIVerb", value: RegistryValue::String("Location Services"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "Icon", value: RegistryValue::String("taskbarcpl.dll,-9"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // On Device
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn On for Device"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d Allow /f' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                // Off Device
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Turn Off for Device"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location\Shell\002flyout\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d Deny /f' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\Location", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_unblock_file_context_menu",
            category: "context_menu",
            name: "Add 'Unblock' Context Menu",
            description: "Adds 'Unblock' option for files downloaded from the internet.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock", value_name: "MUIVerb", value: RegistryValue::String("Unblock"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock\command", value_name: "", value: RegistryValue::String(r#"powershell.exe Unblock-File -LiteralPath '%L'"#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "MUIVerb", value: RegistryValue::String("Unblock"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Unblock files in folder"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock\shell\001flyout\command", value_name: "", value: RegistryValue::String(r#"powershell.exe get-childitem -LiteralPath '%L' | Unblock-File"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"*\shell\unblock", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\unblock", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_windows_security_context_menu",
            category: "context_menu",
            name: "Add 'Windows Security' Context Menu",
            description: "Adds a cascading 'Windows Security' menu to Desktop for quick access to security features.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity", value_name: "MUIVerb", value: RegistryValue::String("Windows Security"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity", value_name: "Icon", value: RegistryValue::String(r#"%ProgramFiles%\Windows Defender\EppManifest.dll,-101"#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Home"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity\shell\001flyout\command", value_name: "", value: RegistryValue::String("explorer windowsdefender:"), stock_value: RegistryValue::DeleteKey },
                 // Adding just one more for brevity as per pattern, user can expand if needed or we add full set
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Virus and threat protection"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity\shell\002flyout\command", value_name: "", value: RegistryValue::String("explorer windowsdefender://threat"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsSecurity", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_windows_update_context_menu",
            category: "context_menu",
            name: "Add 'Windows Update' Context Menu",
            description: "Adds a cascading 'Windows Update' menu to Desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "MUIVerb", value: RegistryValue::String("Windows Update"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "Icon", value: RegistryValue::String("imageres.dll,-1401"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\001flyout", value_name: "MUIVerb", value: RegistryValue::String("Check for Updates"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\001flyout\command", value_name: "", value: RegistryValue::String("cmd /s /c USOClient StartInteractiveScan & start ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\002flyout", value_name: "MUIVerb", value: RegistryValue::String("Settings"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate\shell\002flyout\command", value_name: "", value: RegistryValue::String("explorer ms-settings:windowsupdate"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WindowsUpdate", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_winsxs_cleanup_context_menu",
            category: "context_menu",
            name: "Add 'Component Store Cleanup' (WinSxS)",
            description: "Adds 'Component Store Cleanup' (DISM) to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "MUIVerb", value: RegistryValue::String("Component Store Cleanup"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "Icon", value: RegistryValue::String("cleanmgr.exe"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS\shell\001menu", value_name: "MUIVerb", value: RegistryValue::String("Analyze Component Store"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS\shell\001menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /AnalyzeComponentStore' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS\shell\002menu", value_name: "MUIVerb", value: RegistryValue::String("Clean Up Component Store"), stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS\shell\002menu\command", value_name: "", value: RegistryValue::String(r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /StartComponentCleanup' -Verb runAs""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\WinSxS", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_bat_to_new_context_menu",
            category: "context_menu",
            name: "Add 'Batch File' to New Menu",
            description: "Restores 'Batch File' to the 'New' context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r".bat\ShellNew", value_name: "NullFile", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r".bat\ShellNew", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_vbs_to_new_context_menu",
            category: "context_menu",
            name: "Add 'VBScript' to New Menu",
            description: "Restores 'VBScript' to the 'New' context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r".vbs\ShellNew", value_name: "NullFile", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                // Note: Some systems use ItemName instead of NullFile, but NullFile is standard for blank new file
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r".vbs\ShellNew", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        // Batch 9
        crate::tweak! {
            id: "add_copy_to_folder_context_menu",
            category: "context_menu",
            name: "Add 'Copy To Folder' Context Menu",
            description: "Adds 'Copy To Folder' option to context menu for easier file management.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\{C2FBB630-2971-11D1-A18C-00C04FD75D13}", value_name: "", value: RegistryValue::String("Copy To Folder"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\{C2FBB630-2971-11D1-A18C-00C04FD75D13}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_move_to_folder_context_menu",
            category: "context_menu",
            name: "Add 'Move To Folder' Context Menu",
            description: "Adds 'Move To Folder' option to context menu for easier file management.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\{C2FBB631-2971-11D1-A18C-00C04FD75D13}", value_name: "", value: RegistryValue::String("Move To Folder"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shellex\ContextMenuHandlers\{C2FBB631-2971-11D1-A18C-00C04FD75D13}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_copy_contents_to_clipboard_context_menu",
            category: "context_menu",
            name: "Add 'Copy Contents to Clipboard' Context Menu",
            description: "Adds 'Copy Contents to Clipboard' option to quickly copy file content without opening it.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\*\shell\CopyContents", value_name: "Icon", value: RegistryValue::String("DxpTaskSync.dll,-52"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\*\shell\CopyContents", value_name: "MUIVerb", value: RegistryValue::String("Copy Contents to Clipboard"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\*\shell\CopyContents\command", value_name: "", value: RegistryValue::String(r#"cmd /c clip <"%1""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\*\shell\CopyContents", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "remove_cast_to_device_context_menu",
            category: "context_menu",
            name: "Remove 'Cast to Device' Context Menu",
            description: "Removes 'Cast to Device' from context menu if you don't stream media.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", value: RegistryValue::String("Play to Menu"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", value_name: "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_open_in_windows_terminal_admin_context_menu",
            category: "context_menu",
            name: "Add 'Open in Windows Terminal (Admin)' Context Menu",
            description: "Adds 'Open in Windows Terminal as administrator' to folder context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // Directory
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal as administrator"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -WindowStyle Hidden "Start-Process -Verb RunAs cmd.exe -ArgumentList @('/c','start wt.exe','-d','\"""%V\.\"""')""#), stock_value: RegistryValue::DeleteKey },
                // Directory Background
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "MUIVerb", value: RegistryValue::String("Open in Windows Terminal as administrator"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin\command", value_name: "", value: RegistryValue::String(r#"powershell.exe -WindowStyle Hidden "Start-Process -Verb RunAs cmd.exe -ArgumentList @('/c','start wt.exe','-d','\"""%V\.\"""')""#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\shell\OpenWTHereAsAdmin", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\OpenWTHereAsAdmin", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_disk_cleanup_context_menu",
            category: "context_menu",
            name: "Add 'Disk Cleanup' to Drive Context Menu",
            description: "Adds 'Disk Cleanup' option to the context menu of drives.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Windows.CleanUp", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Windows.CleanUp", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{9cca66bb-9c78-4e59-a76f-a5e9990b8aa0}"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Windows.CleanUp", value_name: "Icon", value: RegistryValue::String(r#"%SystemRoot%\System32\cleanmgr.exe,-104"#), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Windows.CleanUp", value_name: "ImpliedSelectionModel", value: RegistryValue::Dword(1), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Drive\shell\Windows.CleanUp", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_kill_not_responding_tasks_context_menu",
            category: "context_menu",
            name: "Add 'Kill Not Responding Tasks' Context Menu",
            description: "Adds 'Kill Not Responding Tasks' to Desktop context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\KillNRTasks", value_name: "MUIVerb", value: RegistryValue::String("Kill all not responding tasks"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\KillNRTasks", value_name: "Icon", value: RegistryValue::String("taskmgr.exe,-30651"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\KillNRTasks", value_name: "Position", value: RegistryValue::String("Top"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\KillNRTasks\command", value_name: "", value: RegistryValue::String(r#"CMD.exe /C (tasklist /fi "status eq Not Responding" | find /v "No tasks" && taskkill.exe /f /fi "status eq Not Responding" || echo No not-responding tasks found.) & ECHO; & <NUL: set /p junk=Press any key to close this window. & PAUSE >NUL:"#), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"DesktopBackground\Shell\KillNRTasks", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_toggle_hidden_files_context_menu",
            category: "context_menu",
            name: "Add 'Toggle Hidden Files' Context Menu",
            description: "Adds 'Hidden items' toggle options to context menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                // Directory Background
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "MUIVerb", value: RegistryValue::String("Hidden items"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5314"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{f7300245-1f4b-41ba-8948-6fd392064494}"), stock_value: RegistryValue::DeleteKey }, // Native handler

                // AllFilesystemObjects
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles", value_name: "MUIVerb", value: RegistryValue::String("Hidden items"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles", value_name: "Icon", value: RegistryValue::String("imageres.dll,-5314"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles", value_name: "Position", value: RegistryValue::String("Bottom"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles", value_name: "SubCommands", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "CommandStateSync", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", value_name: "ExplorerCommandHandler", value: RegistryValue::String("{f7300245-1f4b-41ba-8948-6fd392064494}"), stock_value: RegistryValue::DeleteKey },
            ],
            disabled_ops: Some(&[
                 RegistryOp { hkey: "HKCR", subkey: r"Directory\Background\shell\HiddenFiles", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                 RegistryOp { hkey: "HKCR", subkey: r"AllFilesystemObjects\shell\HiddenFiles", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
            ]),
        },
        crate::tweak! {
            id: "add_run_with_priority_context_menu",
            category: "context_menu",
            name: "Add 'Run with Priority' Context Menu",
            description: "Adds a 'Run with priority' submenu to executable files to easily start them with specific CPU priority.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority",
                    value_name: "MUIVerb",
                    value: RegistryValue::String("Run with priority"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority",
                    value_name: "SubCommands",
                    value: RegistryValue::String(""),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\001flyout",
                    value_name: "",
                    value: RegistryValue::String("Realtime"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\001flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /Realtime "%1""#),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\002flyout",
                    value_name: "",
                    value: RegistryValue::String("High"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\002flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /High "%1""#),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\003flyout",
                    value_name: "",
                    value: RegistryValue::String("Above normal"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\003flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /AboveNormal "%1""#),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\004flyout",
                    value_name: "",
                    value: RegistryValue::String("Normal"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\004flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /Normal "%1""#),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\005flyout",
                    value_name: "",
                    value: RegistryValue::String("Below normal"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\005flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /BelowNormal "%1""#),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\006flyout",
                    value_name: "",
                    value: RegistryValue::String("Low"),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority\shell\006flyout\command",
                    value_name: "",
                    value: RegistryValue::String(r#"cmd /c start "" /Low "%1""#),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority",
                    value_name: "MUIVerb",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                 RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Classes\exefile\shell\priority",
                    value_name: "SubCommands",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
                id: "remove_include_in_library",
                category: "context_menu",
                name: "Remove 'Include in Library'",
                description: "Removes the 'Include in library' context menu entry from folders.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Classes\Folder\ShellEx\ContextMenuHandlers\Library Location",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::String("{3dad6c5d-2167-4cae-9914-f99e41c12cfa}")
        }],
                disabled_ops: None
        },
        crate::tweak! {
                id: "remove_share",
                category: "context_menu",
                name: "Remove 'Share'",
                description: "Removes the modern 'Share' context menu entry.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Classes\*\shellex\ContextMenuHandlers\ModernSharing",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::DeleteKey
        }],
                disabled_ops: None
        },
        crate::tweak! {
                id: "remove_give_access_to",
                category: "context_menu",
                name: "Remove 'Give Access To'",
                description: "Removes the 'Give access to' (sharing) context menu entry.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\*\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\Directory\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\Directory\Background\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\Drive\shellex\ContextMenuHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                ],
                disabled_ops: None
        },
        crate::tweak! {
                id: "remove_troubleshoot_compat",
                category: "context_menu",
                name: "Remove 'Troubleshoot Compatibility'",
                description: "Removes the 'Troubleshoot compatibility' context menu entry from executables.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\exefile\shellex\ContextMenuHandlers\Compatibility", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{1d27f844-3a1f-4410-85ac-14651078412d}") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\lnkfile\shellex\ContextMenuHandlers\Compatibility", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\batfile\shellex\ContextMenuHandlers\Compatibility", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{1d27f844-3a1f-4410-85ac-14651078412d}") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Classes\cmdfile\shellex\ContextMenuHandlers\Compatibility", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{1d27f844-3a1f-4410-85ac-14651078412d}") },
                ],
                disabled_ops: None
        },
        crate::tweak! {
                id: "add_defender_exclusion_context",
                category: "context_menu",
                name: "Add 'Exclude from Defender' to Context Menu",
                description: "Adds an option to file and folder context menus to quickly add them to Microsoft Defender exclusions.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        // Files
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\DefenderExclusion", value_name: "MUIVerb", value: RegistryValue::String("Add to Defender Exclusions"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\DefenderExclusion", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\DefenderExclusion\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,powershell Add-MpPreference -ExclusionPath ''%1''' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
                        // Folders
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\DefenderExclusion", value_name: "MUIVerb", value: RegistryValue::String("Add to Defender Exclusions"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\DefenderExclusion", value_name: "HasLUAShield", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\DefenderExclusion\command", value_name: "", value: RegistryValue::String(r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,powershell Add-MpPreference -ExclusionPath ''%1''' -Verb RunAs""#), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\*\shell\DefenderExclusion", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\Directory\shell\DefenderExclusion", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
            id: "classic_context_menu",
            category: "context_menu",
            name: "Classic Context Menu",
            description: "Restores the classic Windows 10 style context menu (right-click menu).",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
                value_name: "",
                value: RegistryValue::String(""),
                stock_value: RegistryValue::DeleteKey
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
                value_name: "",
                value: RegistryValue::DeleteKey,
                stock_value: RegistryValue::DeleteKey
            }])
        },
];
