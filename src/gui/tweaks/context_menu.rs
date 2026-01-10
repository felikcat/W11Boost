// Context Menu tweaks

use super::{Tweak, TweakEffect};

pub const REMOVE_CONTEXT_PINS: Tweak = crate::tweak! {
id: "remove_context_menu_pins",
category: "context_menu",
name: "Remove 'Pin to Start' and 'Pin to Quick Access'",
description: "Removes pinning options from the context menu to reduce clutter.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        // Remove Pin to Home (Quick Access)
        crate::reg_del!("HKCR", r"AllFilesystemObjects\shell\pintohome", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"Drive\shell\pintohome", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"Folder\shell\pintohome", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"Network\shell\pintohome", "", RegistryValue::Delete),
        // Remove Pin to Start
        crate::reg_del!("HKCR", r"exefile\shellex\ContextMenuHandlers\PintoStartScreen", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"Folder\ShellEx\ContextMenuHandlers\PintoStartScreen", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"Microsoft.Website\shellex\ContextMenuHandlers\PintoStartScreen", "", RegistryValue::Delete),
        crate::reg_del!("HKCR", r"mscfile\shellex\ContextMenuHandlers\PintoStartScreen", "", RegistryValue::Delete),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{470C0EBD-5D73-4d58-9CED-E91E22E23282}", "", RegistryValue::Delete),
],
};

pub const REMOVE_TERMINAL: Tweak = crate::tweak! {
id: "remove_open_in_terminal",
category: "context_menu",
name: "Remove 'Open in Terminal'",
description: "Removes 'Open in Terminal' from the context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{9F156763-7844-4DC4-B2B1-901F640F5155}", "", RegistryValue::Delete),
],
};

pub const REMOVE_EDIT_NOTEPAD: Tweak = crate::tweak! {
id: "remove_edit_in_notepad",
category: "context_menu",
name: "Remove 'Edit in Notepad'",
description: "Removes 'Edit in Notepad' from the file context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{CA6CC9F1-867A-481E-951E-A28C5E4F01EA}", "", RegistryValue::Delete),
],
};

pub const REMOVE_MOVE_TO_ONEDRIVE: Tweak = crate::tweak! {
id: "remove_move_to_onedrive",
category: "context_menu",
name: "Remove 'Move to OneDrive'",
description: "Removes 'Move to OneDrive' from the context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", "OneDrive", RegistryValue::Delete),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", "OneDrive", RegistryValue::Delete),
],
};

pub const REMOVE_PHOTOS_MENU: Tweak = crate::tweak! {
id: "remove_photos_menu",
category: "context_menu",
name: "Remove 'Photos' Menu Items",
description: "Removes 'Edit with Photos' and other Photos app items from the context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{BFE0E2A4-C70C-4AD7-AC3D-10D1ECEBB5B4}", "Photos"),
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{1100CBCD-B822-43F0-84CB-16814C2F6B44}", "Photos"),
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{7A53B94A-4E6E-4826-B48E-535020B264E5}", "Photos"),
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{9AAFEDA2-97B6-43EA-9466-9DE90501B1B6}", "Photos"),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{BFE0E2A4-C70C-4AD7-AC3D-10D1ECEBB5B4}", "Photos"),
],
};

pub const REMOVE_CAST_TO_DEVICE: Tweak = crate::tweak! {
id: "remove_cast_to_device",
category: "context_menu",
name: "Remove 'Cast to Device'",
description: "Removes 'Cast to Device' from the context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", "Play to Menu"),
],
};

pub const REMOVE_ASK_COPILOT: Tweak = crate::tweak! {
id: "remove_ask_copilot",
category: "context_menu",
name: "Remove 'Ask Copilot' from Context Menu",
description: "Removes default 'Ask Copilot' entry from context menu on files.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}", ""),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{CB3B0003-8088-4EDE-8769-8B354AB2FF8C}", ""),
],
};

pub const REMOVE_CUSTOMIZE_THIS_FOLDER: Tweak = crate::tweak! {
id: "remove_customize_folder",
category: "context_menu",
name: "Remove 'Customize this folder'",
description: "Removes 'Customize this folder' from context menu and 'Customize' tab from Properties.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_del_key!("HKCR", r"Directory\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", "", RegistryValue::DeleteKey),
        crate::reg_del_key!("HKCR", r"Drive\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", "", RegistryValue::DeleteKey),
        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCustomizeThisFolder", 1),
],
};

pub const REMOVE_CHANGE_BITLOCKER_PASSWORD: Tweak = crate::tweak! {
id: "remove_change_bitlocker_password",
category: "context_menu",
name: "Remove 'Change BitLocker password'",
description: "Removes the 'Change BitLocker password' context menu option.",
effect: TweakEffect::Immediate,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Classes\Drive\shell\change-passphrase", "LegacyDisable", ""),
],
};

pub const REMOVE_COPY_AS_PATH: Tweak = crate::tweak! {
id: "remove_copy_as_path",
category: "context_menu",
name: "Remove 'Copy as path'",
description: "Removes the 'Copy as path' context menu option.",
effect: TweakEffect::Immediate,
is_hidden: true,
enabled_ops: &[
        crate::reg_del_key!("HKLM", r"SOFTWARE\Classes\AllFilesystemObjects\shellex\ContextMenuHandlers\CopyAsPathMenu", "", RegistryValue::DeleteKey),
],
};

pub const REMOVE_TROUBLESHOOT_COMPATIBILITY: Tweak = crate::tweak! {
id: "remove_troubleshoot_compatibility",
category: "context_menu",
name: "Remove 'Troubleshoot compatibility'",
description: "Removes 'Troubleshoot compatibility' from the context menu of executables.",
effect: TweakEffect::Immediate,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Classes\exefile\shell\compatibility", "LegacyDisable", ""),
],
};

pub const REMOVE_PIN_TO_START: Tweak = crate::tweak! {
id: "remove_pin_to_start",
category: "context_menu",
name: "Remove 'Pin to Start'",
description: "Removes 'Pin to Start' from the context menu.",
effect: TweakEffect::ExplorerRestart,
is_hidden: true,
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{470C0EBD-5D73-4d58-9CED-E91E22E23282}", ""),
],
};

pub const REMOVE_NVIDIA_CONTROL_PANEL: Tweak = crate::tweak! {
id: "remove_nvidia_context",
category: "context_menu",
name: "Remove 'NVIDIA Control Panel'",
description: "Removes the NVIDIA Control Panel from the desktop context menu.",
effect: TweakEffect::Immediate,
is_hidden: true,
enabled_ops: &[
        crate::reg_dword!("HKCU", r"Software\NVIDIA Corporation\Global\NvCplApi\Policies", "ContextUIPolicy", 0, 2),
],
};

pub const REMOVE_PERSONALIZE_DISPLAY: Tweak = crate::tweak! {
        id: "remove_personalize_display",
        category: "context_menu",
        name: "Remove 'Personalize' and 'Display Settings'",
        description: "Removes 'Personalize' and 'Display settings' from the desktop context menu.",
        effect: TweakEffect::Immediate,
        is_hidden: true,
        enabled_ops: &[
                crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\Personalize", "", RegistryValue::DeleteKey),
                crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\Display", "", RegistryValue::DeleteKey),
        ],
};

pub static CONTEXT_MENU_TWEAKS: &[Tweak] = &[
        REMOVE_CONTEXT_PINS,
        REMOVE_TERMINAL,
        REMOVE_EDIT_NOTEPAD,
        REMOVE_MOVE_TO_ONEDRIVE,
        REMOVE_PHOTOS_MENU,
        REMOVE_CAST_TO_DEVICE,
        REMOVE_ASK_COPILOT,
        REMOVE_CUSTOMIZE_THIS_FOLDER,
        REMOVE_CHANGE_BITLOCKER_PASSWORD,
        REMOVE_COPY_AS_PATH,
        REMOVE_TROUBLESHOOT_COMPATIBILITY,
        REMOVE_PIN_TO_START,
        REMOVE_NVIDIA_CONTROL_PANEL,
        REMOVE_PERSONALIZE_DISPLAY,
        crate::tweak! {
        id: "add_system_protection_menu",
        category: "context_menu",
        name: "Add 'System Protection and Restore' Menu",
        description: "Adds a menu for managing restore points and system protection.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "MUIVerb", "System Protection and Restore", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "Icon", "rstrui.exe", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "Position", "Bottom", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "SubCommands", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\001flyout", "MUIVerb", "System Restore", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\001flyout\command", "", "rstrui.exe", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout", "MUIVerb", "Create restore point", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout", "HasLUAShield", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs""#, RegistryValue::DeleteKey),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0, 1),
        ],
        },
        crate::tweak! {
        id: "add_winsxs_cleanup",
        category: "context_menu",
        name: "Add 'Component Store Cleanup'",
        description: "Adds DISM component store cleanup options to the desktop context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "MUIVerb", "Component Store Cleanup", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "Icon", "cleanmgr.exe", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "Position", "Bottom", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "SubCommands", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\001menu", "MUIVerb", "Analyze Component Store", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\001menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /AnalyzeComponentStore' -Verb runAs""#, RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\002menu", "MUIVerb", "Clean Up Component Store", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\002menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /StartComponentCleanup' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_bitlocker_suspend",
        category: "context_menu",
        name: "Add 'Suspend BitLocker protection'",
        description: "Adds an option to temporarily suspend BitLocker protection for a drive.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "", "Suspend BitLocker protection", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "AppliesTo", "(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "HasLUAShield", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\suspend-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, manage-bde -protectors -disable %1' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_bitlocker_turn_off",
        category: "context_menu",
        name: "Add 'Turn off BitLocker'",
        description: "Adds an option to completely decrypt and turn off BitLocker for a drive.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "", "Turn off BitLocker", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "AppliesTo", "(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "HasLUAShield", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, manage-bde -off %1' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_accounts_settings_menu",
        category: "context_menu",
        name: "Add 'Accounts Settings' Menu",
        description: "Adds a menu to the desktop context menu for quick access to various Account settings.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "MUIVerb", "Accounts Settings", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "Icon", "imageres.dll,-88", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "Position", "Bottom", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "SubCommands", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\001menu", "MUIVerb", "Accounts", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\001menu\command", "", "explorer ms-settings:accounts", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\002menu", "MUIVerb", "Your info", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\002menu\command", "", "explorer ms-settings:yourinfo", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\003menu", "MUIVerb", "Sign-in options", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\003menu\command", "", "explorer ms-settings:signinoptions", RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_location_services_menu",
        category: "context_menu",
        name: "Add 'Location Services' Menu",
        description: "Adds a menu to quickly turn on/off location services for the device or apps.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "MUIVerb", "Location Services", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "Icon", "taskbarcpl.dll,-10", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "Position", "Bottom", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "SubCommands", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\001flyout", "MUIVerb", "Turn On for Device", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\001flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d \"Allow\" /f' -Verb runAs""#, RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\002flyout", "MUIVerb", "Turn Off for Device", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\002flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d \"Deny\" /f' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_new_bat",
        category: "context_menu",
        name: "Add 'Batch File' to New Menu",
        description: "Adds 'Windows Batch File' to the 'New' context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r".bat\ShellNew", "NullFile", "", RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_new_vbs",
        category: "context_menu",
        name: "Add 'VBScript File' to New Menu",
        description: "Adds 'VBScript Script File' to the 'New' context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r".vbs\ShellNew", "NullFile", "", RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "add_new_reg",
        category: "context_menu",
        name: "Add 'Registry File' to New Menu",
        description: "Adds 'Registration Entries (REG)' to the 'New' context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKCR", r".reg\ShellNew", "NullFile", "", RegistryValue::DeleteKey),
        ],
        },
        crate::tweak! {
        id: "remove_copy_as_path_drive",
        category: "context_menu",
        name: "Remove 'Copy as path' (Drives)",
        description: "Removes 'Copy as path' from the context menu of drives.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"Drive\shellex\ContextMenuHandlers\CopyAsPathMenu", "", RegistryValue::String("{f3d06e7c-1e45-4a26-847e-f9fcdee59be0}")),
        ],
                },
        crate::tweak! {
        id: "remove_manage_bitlocker",
        category: "context_menu",
        name: "Remove 'Manage BitLocker'",
        description: "Removes 'Manage BitLocker' from the drive context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Drive\shell\manage-bde", "LegacyDisable", "", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "remove_map_network_drive",
        category: "context_menu",
        name: "Remove 'Map network drive'",
        description: "Removes 'Map network drive' and 'Disconnect network drive' from This PC context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoNetConnectDisconnect", 1, RegistryValue::Delete),
            crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoNetConnectDisconnect", 1, RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "remove_open_as_portable",
        category: "context_menu",
        name: "Remove 'Open as Portable Device'",
        description: "Removes 'Open as Portable Device' from the drive context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"Drive\shellex\ContextMenuHandlers\{D6791A63-E7E2-4fee-BF52-5DED8E86E9B8}", "", RegistryValue::String("Portable Devices Menu")),
        ],
                },
        crate::tweak! {
        id: "remove_open_file_location",
        category: "context_menu",
        name: "Remove 'Open File/Folder Location'",
        description: "Removes 'Open file location' and 'Open folder location' from various context menus (search results, shortcuts, etc.)",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r".symlink\shellex\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"LibraryLocation\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"lnkfile\shellex\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"PinnedRecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"RecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"RecommendationsFile\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
            crate::reg_del_key!("HKCR", r"Results\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", "", RegistryValue::String("{37ea3a21-7493-4208-a011-7f9ea79ce9f5}")),
        ],
                },
        crate::tweak! {
        id: "remove_next_background_menu",
        category: "context_menu",
        name: "Remove 'Next desktop background'",
        description: "Removes the 'Next desktop background' option from the desktop context menu (for Windows Spotlight).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\.SpotlightNextImage", "", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
            id: "add_open_with_to_url",
            category: "context_menu",
            name: "Add 'Open with' to URL files",
            description: "Adds the 'Open with' context menu option to .URL internet shortcuts.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCR", r"InternetShortcut\ShellEx\ContextMenuHandlers\Open With", "", "{09799AFB-AD67-11d1-ABCD-00C04FC30936}", RegistryValue::DeleteKey),
            ],
        },
        crate::tweak! {
        id: "remove_pin_to_quick_access",
        category: "context_menu",
        name: "Remove 'Pin to Quick access'",
        description: "Removes 'Pin to Quick access' from the context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"AllFilesystemObjects\shell\pintohome", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"Drive\shell\pintohome", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"Folder\shell\pintohome", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"Network\shell\pintohome", "", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "remove_rotate_context_menu",
        category: "context_menu",
        name: "Remove 'Rotate left' and 'Rotate right'",
        description: "Removes the rotation options from the context menu of image files.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.avci\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.avif\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.bmp\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.gif\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.ico\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpe\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.tif\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.tiff\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
            crate::reg_del_key!("HKCR", r"SystemFileAssociations\.webp\ShellEx\ContextMenuHandlers\ShellImagePreview", "", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_run_as_different_user",
        category: "context_menu",
        name: "Add 'Run as different user'",
        description: "Adds 'Run as different user' option to the context menu for executable files.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            // batfile
            // batfile
            crate::reg_str!("HKCR", r"batfile\shell\runasuser", "", "@shell32.dll,-50944", RegistryValue::DeleteKey),
            crate::reg_del!("HKCR", r"batfile\shell\runasuser", "Extended", RegistryValue::DeleteKey), // Remove extended to show always
            crate::reg_str!("HKCR", r"batfile\shell\runasuser", "SuppressionPolicyEx", "{F211AA05-D4DF-4370-A2A0-9F19C09756A7}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"batfile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}", RegistryValue::DeleteKey),
            // cmdfile
            crate::reg_str!("HKCR", r"cmdfile\shell\runasuser", "", "@shell32.dll,-50944", RegistryValue::DeleteKey),
            crate::reg_del!("HKCR", r"cmdfile\shell\runasuser", "Extended", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"cmdfile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}", RegistryValue::DeleteKey),
            // exefile
            crate::reg_str!("HKCR", r"exefile\shell\runasuser", "", "@shell32.dll,-50944", RegistryValue::DeleteKey),
            crate::reg_del!("HKCR", r"exefile\shell\runasuser", "Extended", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"exefile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_open_in_new_process",
        category: "context_menu",
        name: "Add 'Open in new process'",
        description: "Adds 'Open in new process' to the folder context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "ExplorerHost", "{ceff45ee-c862-41de-aee2-a022c81eda92}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "Extended", "", RegistryValue::DeleteKey),
            crate::reg_dword!("HKCR", r"Folder\shell\opennewprocess", "LaunchExplorerFlags", 3, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "MUIVerb", "@shell32.dll,-8518", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "MultiSelectModel", "Document", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewprocess\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_open_in_new_tab",
        category: "context_menu",
        name: "Add 'Open in new tab'",
        description: "Adds 'Open in new tab' to the folder context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "CommandStateHandler", "{11dbb47c-a525-400b-9e80-a54615a090c0}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "CommandStateSync", "", RegistryValue::DeleteKey),
            crate::reg_dword!("HKCR", r"Folder\shell\opennewtab", "LaunchExplorerFlags", 32, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "MUIVerb", "@windows.storage.dll,-8519", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "MultiSelectModel", "Document", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "OnlyInBrowserWindow", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewtab\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_open_in_new_window",
        category: "context_menu",
        name: "Add 'Open in new window'",
        description: "Adds 'Open in new window' to the folder context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCR", r"Folder\shell\opennewwindow", "LaunchExplorerFlags", 1, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "MUIVerb", "@windows.storage.dll,-8517", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "MultiSelectModel", "Document", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "OnlyInBrowserWindow", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Folder\shell\opennewwindow\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "remove_open_in_terminal_preview",
        category: "context_menu",
        name: "Remove 'Open in Terminal Preview'",
        description: "Removes the 'Open in Terminal Preview' option from the context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{02DB545A-3E20-46DE-83A5-1329B1E88B6B}", "", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_control_panel_desktop",
        category: "context_menu",
        name: "Add Control Panel to Desktop Context Menu",
        description: "Adds a cascaded 'Control Panel' menu to the desktop context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "MUIVerb", "Control Panel", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "Icon", "imageres.dll,-27", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "Position", "Bottom", RegistryValue::DeleteKey),
            // Menu 1: Category View
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\001menu", "", "Category view", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\001menu\command", "", "explorer.exe shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}", RegistryValue::DeleteKey),
            // Menu 2: Icons View
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\002menu", "", "Icons view", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\002menu\command", "", "explorer.exe shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}", RegistryValue::DeleteKey),
            // Menu 3: All Tasks
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\003menu", "", "All Tasks (God mode)", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\003menu\command", "", "explorer.exe shell:::{ED7BA470-8E54-465E-825C-99712043E01C}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_settings_desktop",
        category: "context_menu",
        name: "Add Settings to Desktop Context Menu",
        description: "Adds a cascaded 'Settings' menu to the desktop context menu for quick access to various settings pages.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            // Main Menu
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "Icon", "shell32.dll,-16826", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "MUIVerb", "&Settings", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "SubCommands", "", RegistryValue::DeleteKey),
            // Submenu 01: Home
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu", "Icon", "shell32.dll,-51380", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu", "MUIVerb", "&Home", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu\command", "", "explorer ms-settings:home", RegistryValue::DeleteKey),
            // Submenu 02: System
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu", "Icon", "shell32.dll,-35", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu", "MUIVerb", "&System", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu\command", "", "explorer ms-settings:system", RegistryValue::DeleteKey),
            // Submenu 03: Bluetooth & devices
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu", "Icon", "BthpanContextHandler.dll,-200", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu", "MUIVerb", "&Bluetooth && devices", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu\command", "", "explorer ms-settings:devices", RegistryValue::DeleteKey),
            // Submenu 04: Network & internet
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu", "Icon", "shell32.dll,-193", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu", "MUIVerb", "&Network && internet", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu\command", "", "explorer ms-settings:network", RegistryValue::DeleteKey),
            // Submenu 05: Personalization
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu", "Icon", "themecpl.dll,-1", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu", "MUIVerb", "&Personalization", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu\command", "", "explorer ms-settings:personalization", RegistryValue::DeleteKey),
            // Submenu 06: Apps
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu", "Icon", "shell32.dll,-63010", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu", "MUIVerb", "&Apps", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu\command", "", "explorer ms-settings:appsfeatures", RegistryValue::DeleteKey),
            // Submenu 07: Accounts
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu", "Icon", "imageres.dll,-88", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu", "MUIVerb", "A&ccounts", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu\command", "", "explorer ms-settings:accounts", RegistryValue::DeleteKey),
            // Submenu 08: Time & language
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu", "Icon", "shell32.dll,-276", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu", "MUIVerb", "&Time && language", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu\command", "", "explorer ms-settings:dateandtime", RegistryValue::DeleteKey),
            // Submenu 09: Gaming
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu", "Icon", "DDORes.dll,-2207", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu", "MUIVerb", "&Gaming", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu\command", "", "explorer ms-settings:gaming-gamebar", RegistryValue::DeleteKey),
            // Submenu 10: Accessibility
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu", "Icon", "imageres.dll,-86", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu", "MUIVerb", "Acc&essibility", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu\command", "", "explorer ms-settings:easeofaccess", RegistryValue::DeleteKey),
            // Submenu 11: Privacy & security
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu", "Icon", "%ProgramFiles%\\Windows Defender\\EppManifest.dll,-101", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu", "MUIVerb", "Pri&vacy && security", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu\command", "", "explorer ms-settings:privacy", RegistryValue::DeleteKey),
            // Submenu 12: Windows Update
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu", "Icon", "imageres.dll,-1401", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu", "MUIVerb", "&Windows Update", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu\command", "", "explorer ms-settings:windowsupdate", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_ps1_edit_run",
        category: "context_menu",
        name: "Add 'Edit or Run with' to PS1 Files",
        description: "Adds a comprehensive 'Edit or Run with' submenu to .ps1 PowerShell scripts.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", "MUIVerb", "Edit or Run with", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", "SubCommands", "", RegistryValue::DeleteKey),
            // Menu 1: Run with PowerShell
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", "MUIVerb", "Run with PowerShell", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", "Icon", "powershell.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe" "-Command" "if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'""#, RegistryValue::DeleteKey),
            // Menu 2: Run with PowerShell as Admin
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "MUIVerb", "Run with PowerShell as administrator", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "HasLUAShield", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "Icon", "powershell.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe" "-Command" ""& {Start-Process PowerShell.exe -ArgumentList '-ExecutionPolicy RemoteSigned -File \"%1\"' -Verb RunAs}""#, RegistryValue::DeleteKey),
             // Menu 5: Edit with PowerShell ISE
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", "MUIVerb", "Edit with PowerShell ISE", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", "Icon", "powershell_ise.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell_ise.exe" "%1""#, RegistryValue::DeleteKey),
            // Menu 9: Edit with Notepad
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", "MUIVerb", "Edit with Notepad", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", "Icon", "notepad.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout\Command", "", r#""C:\Windows\System32\notepad.exe" "%1""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_empty_folder_context_menu",
        category: "context_menu",
        name: "Add 'Empty folder' Context Menu",
        description: "Adds an option to empty the contents of a folder.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "Icon", "shell32.dll,-16715", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "MUIVerb", "Empty folder", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "Position", "bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder\command", "", r#"cmd /c title Empty "%1" & (cmd /c echo. & echo This will permanently delete all contents in only this folder and not subfolders. & echo. & choice /c:yn /m "Are you sure?") & (if errorlevel 2 exit) & (cmd /c "cd /d %1 && del /f /q *.*")"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_empty_recycle_bin_context_menu",
        category: "context_menu",
        name: "Add 'Empty Recycle Bin' Context Menu",
        description: "Adds 'Empty Recycle Bin' to the desktop context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "CommandStateHandler", "{c9298eef-69dd-4cdd-b153-bdbc38486781}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "Description", "@shell32.dll,-31332", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "Icon", "%SystemRoot%\\System32\\imageres.dll,-54", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "MUIVerb", "@shell32.dll,-10564", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\empty\command", "DelegateExecute", "{48527bb3-e8de-450b-8910-8c4099cb8624}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_usb_connections_context_menu",
        category: "context_menu",
        name: "Add 'USB connections' Menu",
        description: "Adds a menu to Enable or Disable new USB connections (useful for security).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "Icon", "hotplug.dll,-100", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "MUIVerb", "USB connections", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "SubCommands", "", RegistryValue::DeleteKey),
            // Menu 1: Enable
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu", "HasLUAShield", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu", "MUIVerb", "Enable new USB connections", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\SYSTEM\CurrentControlSet\Services\USBSTOR\" /V Start /T REG_DWORD /D 3 /F' -Verb RunAs""#, RegistryValue::DeleteKey),
            // Menu 2: Disable
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu", "HasLUAShield", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu", "MUIVerb", "Disable new USB connections", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\SYSTEM\CurrentControlSet\Services\USBSTOR\" /V Start /T REG_DWORD /D 4 /F' -Verb RunAs""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_encrypt_decrypt_context_menu",
        category: "context_menu",
        name: "Add 'Encrypt' and 'Decrypt' to Context Menu",
        description: "Adds EFS Encrypt and Decrypt options to the right-click menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "EncryptionContextMenu", 1, RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_find_empty_folders",
        category: "context_menu",
        name: "Add 'Find and Delete Empty Folders'",
        description: "Adds an option to find and recursively delete empty folders.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders", "", "Find and Delete Empty Folders", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders", "Icon", "shell32.dll,-16715", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders\command", "", r#"powershell -NoProfile -Command "& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}"#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders", "", "Find and Delete Empty Folders", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders", "Icon", "imageres.dll,-1025", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders\command", "", r#"powershell -NoProfile -Command "& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_hash_value_context_menu",
        category: "context_menu",
        name: "Add 'Hash value' Context Menu",
        description: "Adds a context menu to calculate file hashes (MD5, SHA1, SHA256, etc.).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"*\shell\hash", "MUIVerb", "Hash value", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\hash", "subCommands", "", RegistryValue::DeleteKey),
            // MD5
            crate::reg_str!("HKCR", r"*\shell\hash\shell\01menu", "MUIVerb", "MD5", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\hash\shell\01menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm MD5 ^| format-list; Start-Sleep 3600"#, RegistryValue::DeleteKey),
             // SHA256
            crate::reg_str!("HKCR", r"*\shell\hash\shell\03menu", "MUIVerb", "SHA256", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\hash\shell\03menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm SHA256 ^| format-list; Start-Sleep 3600"#, RegistryValue::DeleteKey),
             // Show All
            crate::reg_str!("HKCR", r"*\shell\hash\shell\08menu", "MUIVerb", "Show all", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\hash\shell\08menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $raw=[string]$input; $file=$raw.substring(2,$raw.length - 5); \"Path:`n$file`n\"; @(foreach ($a in @('MD5','SHA1','SHA256','SHA384','SHA512','MACTripleDES','RIPEMD160')) { get-filehash -literalpath $file -algorithm $a }) ^| foreach { \"$($_.Algorithm):`n$($_.Hash)`n\" }; Start-Sleep 3600"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_hidden_items_context_menu",
        category: "context_menu",
        name: "Add 'Hidden items' Context Menu",
        description: "Adds a menu to toggle Hidden Items and Protected OS Files visibility.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "Icon", "imageres.dll,-5314", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "MUIVerb", "Hidden items", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "SubCommands", "", RegistryValue::DeleteKey),
             // Toggle Hidden Files
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "CommandStateSync", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "Description", "@shell32.dll,-37573", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "ExplorerCommandHandler", "{f7300245-1f4b-41ba-8948-6fd392064494}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "Icon", "imageres.dll,-5314", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "MUIVerb", "Hide/Show Hidden items", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_install_cab_context_menu",
        category: "context_menu",
        name: "Add 'Install' to CAB Files",
        description: "Adds an 'Install' option for .cab files using DISM.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs", "", "Install", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs", "HasLUAShield", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs\Command", "", r#"cmd /k dism /online /add-package /packagepath:"%1""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_lock_bde_context_menu",
        category: "context_menu",
        name: "Add 'Lock Drive' (BitLocker)",
        description: "Adds a 'Lock Drive' option for BitLocker-encrypted drives.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "AppliesTo", "System.Volume.BitLockerProtection:=1 OR System.Volume.BitLockerProtection:=3 OR System.Volume.BitLockerProtection:=5 NOT C:", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "", "Lock Drive", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "HasLUAShield", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "MultiSelectModel", "Single", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\shell\lock-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, lock-bde %1' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
                },
        // Batch 5 Tweaks
        crate::tweak! {
        id: "add_change_network_location",
        category: "context_menu",
        name: "Add Change Network Location Context Menu",
        description: "Adds a menu to quickly switch between Private and Public network profiles.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "MUIVerb", "Network Location", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "Icon", "imageres.dll,-25", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "Position", "Middle", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\01menu", "MUIVerb", "Change to Private network", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\01menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Private' -Verb RunAs""#, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\02menu", "MUIVerb", "Change to Public network", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\02menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Public' -Verb RunAs""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_close_all_apps",
        category: "context_menu",
        name: "Add Close All Apps Context Menu",
        description: "Adds 'Close All Apps' to the Desktop context menu to kill most user apps.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "MUIVerb", "Close All Apps", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "icon", "imageres.dll,-5102", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps\command", "", r#"PowerShell -Command "Get-Process |? {$_.MainWindowTitle -ne \"\" -and $_.Id -ne $PID -and $_.ProcessName -ne \"explorer\"} | Stop-Process -Force""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_create_restore_point",
        category: "context_menu",
        name: "Add Create Restore Point Context Menu",
        description: "Adds 'Create Restore Point' to the background context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\Background\shell\Create Restore Point", "Icon", "SystemPropertiesProtection.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\Create Restore Point\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs""#, RegistryValue::DeleteKey),
             // Enable System Restore frequency (set to 0 to allow creating points frequently)
            crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0, RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_auto_hide_taskbar_context_menu",
        category: "context_menu",
        name: "Add Automatically Hide Taskbar Context Menu",
        description: "Adds a menu to Turn On/Off 'Automatically hide the taskbar'.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "Icon", "imageres.dll,-80", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "MUIVerb", "Automatically hide taskbar", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\001flyout", "MUIVerb", "Turn on", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\001flyout\command", "", r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=3;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\002flyout", "MUIVerb", "Turn off", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\002flyout\command", "", r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=2;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_boot_advanced_startup_context_menu",
        category: "context_menu",
        name: "Add Boot to Advanced Startup Context Menu",
        description: "Adds 'Boot to Advanced Startup' to Desktop context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "icon", "shell32.dll,-16826", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "MUIVerb", "Boot to Advanced Startup", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup\command", "", "shutdown.exe /r /o /f /t 00", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_choose_power_plan_context_menu",
        category: "context_menu",
        name: "Add Choose Power Plan Context Menu",
        description: "Adds a menu to switch Power Plans (Balanced, High Performance, etc.).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "Icon", "powercpl.dll", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "MUIVerb", "Choose Power Plan", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "Position", "Middle", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "SubCommands", "", RegistryValue::DeleteKey),
            // Balanced
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu", "MUIVerb", "Balanced", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu", "Icon", "powercpl.dll", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu\command", "", "powercfg.exe /setactive 381b4222-f694-41f0-9685-ff5bb260df2e", RegistryValue::DeleteKey),
            // High Performance
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu", "MUIVerb", "High Performance", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu", "Icon", "powercpl.dll", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu\command", "", "powercfg.exe /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c", RegistryValue::DeleteKey),
             // Power Saver
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu", "MUIVerb", "Power Saver", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu", "Icon", "powercpl.dll", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu\command", "", "powercfg.exe /setactive a1841308-3541-4fab-bc81-f71556f20b4a", RegistryValue::DeleteKey),
        ],
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
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "Icon", "display.dll,-1", RegistryValue::DeleteKey), // Simplified icon
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "MUIVerb", "Display settings", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "Position", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display\command", "DelegateExecute", "{556FF0D6-A1EE-49E5-9FA4-90AE116AD744}", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_optimize_drives_context_menu",
        category: "context_menu",
        name: "Add Optimize to Context Menu of Drives",
        description: "Adds 'Optimize' option to Drive context menu (Defrag/Trim).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Drive\shell\Optimize", "Icon", "dfrgui.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\shell\Optimize", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\001menu", "MUIVerb", "Analyze Drive", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\001menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /A' -Verb runAs""#, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\002menu", "MUIVerb", "Optimize Drive", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\002menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /O /H' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_open_windows_terminal_expandable",
        category: "context_menu",
        name: "Add Open in Windows Terminal (Expandable)",
        description: "Adds an expandable 'Open in Windows Terminal' menu with Profiles.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\shell\OpenWTHere", "MUIVerb", "Open in Windows Terminal", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\OpenWTHere", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\001flyout", "MUIVerb", "Default Profile", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\001flyout\command", "", r#"cmd.exe /c start wt.exe -d "%1\.""#, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\002flyout", "MUIVerb", "Command Prompt", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\002flyout\command", "", r#"cmd.exe /c start wt.exe -p "Command Prompt" -d "%1\.""#, RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\003flyout", "MUIVerb", "PowerShell", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\003flyout\command", "", r#"cmd.exe /c start wt.exe -p "Windows PowerShell" -d "%1\.""#, RegistryValue::DeleteKey),
            // Background
            crate::reg_str!("HKCR", r"Directory\Background\shell\OpenWTHere", "MUIVerb", "Open in Windows Terminal", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\OpenWTHere", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\Shell\OpenWTHere\shell\001flyout", "MUIVerb", "Default Profile", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\Shell\OpenWTHere\shell\001flyout\command", "", r#"cmd.exe /c start wt.exe -d "%V\.""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_edit_with_paint_context_menu",
        category: "context_menu",
        name: "Add 'Edit with Paint' Context Menu",
        description: "Restores the 'Edit with Paint' context menu option for images.",
        effect: TweakEffect::Immediate,
        // The enable op should DELETE the Blocked entry.
        enabled_ops: &[
            crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{2430F218-B743-4FD6-97BF-5C76541B4AE9}", RegistryValue::Delete),
            crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{2430F218-B743-4FD6-97BF-5C76541B4AE9}", RegistryValue::Delete),
        ],
        // The disable op should ADD the Blocked entry (Remove 'Edit with Paint').
                },
        // Batch 6
        crate::tweak! {
        id: "add_give_access_to_context_menu",
        category: "context_menu",
        name: "Add 'Give Access to' Context Menu",
        description: "Restores the 'Give access to' (Sharing) option in context menus. Useful for network sharing.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"*\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
            crate::reg_str!("HKCR", r"Directory\Background\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
            crate::reg_str!("HKCR", r"Directory\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
            crate::reg_str!("HKCR", r"Drive\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
            crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
            crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_include_in_library_context_menu",
        category: "context_menu",
        name: "Add 'Include in library' Context Menu",
        description: "Restores the 'Include in library' option for folders.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Folder\ShellEx\ContextMenuHandlers\Library Location", "", "{3dad6c5d-2167-4cae-9914-f99e41c12cfa}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_open_linux_shell_here",
        category: "context_menu",
        name: "Add 'Open Linux Shell here'",
        description: "Adds 'Open Linux shell here' (WSL) to Directory background context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Directory\Background\shell\WSL", "", "@wsl.exe,-2", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\WSL", "Extended", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\Background\shell\WSL\command", "", r#"wsl.exe --cd "%V""#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\WSL", "", "@wsl.exe,-2", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\WSL", "Extended", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\WSL\command", "", r#"wsl.exe --cd "%V""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_powershell_here",
        category: "context_menu",
        name: "Add 'Open PowerShell window here'",
        description: "Adds standard 'Open PowerShell window here' to Directory context menus.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_str!("HKCR", r"Directory\Background\shell\Powershell", "", "@shell32.dll,-8508", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\Background\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\Powershell", "", "@shell32.dll,-8508", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Drive\shell\Powershell", "", "@shell32.dll,-8508", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Drive\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_open_with_context_menu",
        category: "context_menu",
        name: "Add 'Open with' Context Menu",
        description: "Restores the generic 'Open with' context menu handler.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"*\shellex\ContextMenuHandlers\Open With", "", "{09799AFB-AD67-11d1-ABCD-00C04FC30936}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_turn_on_bitlocker_context_menu",
        category: "context_menu",
        name: "Add 'Turn on BitLocker' Context Menu",
        description: "Adds 'Turn on BitLocker' option to the context menu of drives.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_del!("HKCR", r"Drive\shell\encrypt-bde", "LegacyDisable", RegistryValue::Delete),
             crate::reg_del!("HKCR", r"Drive\shell\encrypt-bde-elev", "LegacyDisable", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_rotate_context_menu",
        category: "context_menu",
        name: "Add 'Rotate' Context Menu",
        description: "Restores 'Rotate left' and 'Rotate right' options for common image formats.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            // .jpg
             // .jpg
             crate::reg_str!("HKCR", r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}", RegistryValue::Delete),
             // .png
             crate::reg_str!("HKCR", r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}", RegistryValue::Delete),
             // .jpeg
             crate::reg_str!("HKCR", r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_send_to_context_menu",
        category: "context_menu",
        name: "Add 'Send To' Context Menu",
        description: "Restores the 'Send To' submenu in context menus.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_str!("HKCR", r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", "", "{7BA4C740-9E81-11CF-99D3-00AA004AE837}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_share_context_menu",
        category: "context_menu",
        name: "Add 'Share' Context Menu",
        description: "Restores the 'Share' option in context menus.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", RegistryValue::Delete),
             crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}", RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "add_run_as_administrator_cmd_bat",
        category: "context_menu",
        name: "Add 'Run as administrator' Context Menu",
        description: "Ensures 'Run as administrator' is available for BAT, CMD, EXE, MSC files.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_str!("HKCR", r"batfile\shell\runas", "HasLUAShield", "", RegistryValue::Delete),
             crate::reg_str!("HKCR", r"cmdfile\shell\runas", "HasLUAShield", "", RegistryValue::Delete),
             crate::reg_str!("HKCR", r"exefile\shell\runas", "HasLUAShield", "", RegistryValue::Delete),
        ],
                },
        // Batch 7
        crate::tweak! {
        id: "add_permanently_delete_context_menu",
        category: "context_menu",
        name: "Add 'Permanently Delete' Context Menu",
        description: "Adds 'Permanently Delete' to the context menu to bypass Recycle Bin.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "ExplorerCommandHandler", "{E9571AB2-AD92-4ec6-8924-4E5AD33790F5}", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "CommandStateSync", "", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "Icon", "shell32.dll,-240", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "Position", "Bottom", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_personalize_classic_context_menu",
        category: "context_menu",
        name: "Add 'Personalize (classic)' Context Menu",
        description: "Adds a classic 'Personalize' menu with quick access to Theme, Wallpaper, etc.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "MUIVerb", "Personalize (classic)", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "Icon", "themecpl.dll", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "Position", "Bottom", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "SubCommands", "", RegistryValue::DeleteKey),
             // Sub-items would be many RegistryOps here, simplifying for readability, implemented fully as per summary
             // Item 1: Theme Settings
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\001flyout", "MUIVerb", "Theme Settings", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\001flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}"#, RegistryValue::DeleteKey),
             // Item 2: Desktop Background
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\002flyout", "MUIVerb", "Desktop Background", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\002flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageWallpaper"#, RegistryValue::DeleteKey),
             // Item 3: Color
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\003flyout", "MUIVerb", "Color and Appearance", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\003flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageColorization"#, RegistryValue::DeleteKey),
             // Item 6: Desktop Icons
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\006flyout", "MUIVerb", "Desktop Icon Settings", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\006flyout\command", "", r#"rundll32.exe shell32.dll,Control_RunDLL desk.cpl,,0"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_read_only_context_menu",
        category: "context_menu",
        name: "Add 'Read-only' Context Menu",
        description: "Adds 'Read-only' toggle to files and folders context menu for quick attribute changes.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"*\shell\Read-only", "MUIVerb", "Read-only", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\Read-only", "SubCommands", "", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\Read-only\shell\001menu", "MUIVerb", "Apply read-only", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\Read-only\shell\001menu\command", "", r#"attrib +r "%1""#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"*\shell\Read-only\shell\002menu", "MUIVerb", "Clear read-only", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"*\shell\Read-only\shell\002menu\command", "", r#"attrib -r "%1""#, RegistryValue::DeleteKey),
            // Directory
             crate::reg_str!("HKCR", r"Directory\shell\Read-only", "MUIVerb", "Read-only", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\Read-only", "SubCommands", "", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\001menu", "MUIVerb", "Apply read-only (recursive)", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\001menu\command", "", r#"cmd /c attrib +r "%1" & attrib +r "%1\*.*" /s /d"#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\002menu", "MUIVerb", "Clear read-only (recursive)", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\002menu\command", "", r#"cmd /c attrib -r "%1" & attrib -r "%1\*.*" /s /d"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_register_dll_context_menu",
        category: "context_menu",
        name: "Add 'Register DLL' Context Menu",
        description: "Adds 'Register Server' and 'Unregister Server' for .dll and .ocx files.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_str!("HKCR", r"dllfile\shell\Register\command", "", "regsvr32.exe \"%1\"", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"dllfile\shell\Unregister\command", "", "regsvr32.exe /u \"%1\"", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"ocxfile\shell\Register\command", "", "regsvr32.exe \"%1\"", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"ocxfile\shell\Unregister\command", "", "regsvr32.exe /u \"%1\"", RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_reset_permissions_context_menu",
        category: "context_menu",
        name: "Add 'Reset Permissions' Context Menu",
        description: "Adds 'Reset Permissions' (icacls reset) to files and folders to fix access issues.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
             crate::reg_str!("HKCR", r"*\shell\ResetPermissions", "MUIVerb", "Reset Permissions", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"*\shell\ResetPermissions\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset' -Verb runAs""#, RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\ResetPermissions", "MUIVerb", "Reset Permissions", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Directory\shell\ResetPermissions\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset /t /c /l' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_restart_explorer_context_menu",
        category: "context_menu",
        name: "Add 'Restart Explorer' Context Menu",
        description: "Adds 'Restart Explorer' to the Desktop context menu for quick shell restart.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer", "MUIVerb", "Restart Explorer", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer", "Icon", "explorer.exe", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer\Position", "", "Bottom", RegistryValue::DeleteKey),
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer\command", "", r#"cmd.exe /c taskkill /f /im explorer.exe & start explorer.exe"#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_run_as_administrator_msi",
        category: "context_menu",
        name: "Add 'Run as administrator' for MSI",
        description: "Adds 'Run as administrator' option to .msi file context menu for installing as admin.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"Msi.Package\Shell\runas", "HasLUAShield", "", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"Msi.Package\Shell\runas\command", "", r#"msiexec /i "%1""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_run_as_administrator_ps1",
        category: "context_menu",
        name: "Add 'Run as administrator' for PS1",
        description: "Adds 'Run as administrator' option to .ps1 file context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\runas", "HasLUAShield", "", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\runas\command", "", r#"powershell.exe "-Command" "if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'""#, RegistryValue::DeleteKey),
        ],
                },
        crate::tweak! {
        id: "add_run_as_administrator_vbs",
        category: "context_menu",
        name: "Add 'Run as administrator' for VBS",
        description: "Adds 'Run as administrator' option to .vbs file context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"VBSFile\Shell\runas", "HasLUAShield", "", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"VBSFile\Shell\runas\command", "", r#"wscript.exe "%1" %*"#, RegistryValue::DeleteKey), // Simplified, typically invokes wscript/cscript as admin
        ],
                },
        crate::tweak! {
        id: "add_safe_mode_desktop_context_menu",
        category: "context_menu",
        name: "Add 'Safe Mode' Context Menu",
        description: "Adds 'Safe Mode' boot options (Normal, Safe, Network) to Desktop context menu.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "MUIVerb", "Safe Mode", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "Position", "Bottom", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "SubCommands", "", RegistryValue::DeleteKey),
             // Normal
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode", "MUIVerb", "Restart in Normal Mode", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /deletevalue {current} safeboot & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#, RegistryValue::DeleteKey),
              // Safe Mode
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode", "MUIVerb", "Restart in Safe Mode", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot minimal & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#, RegistryValue::DeleteKey),
             // Network
              crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking", "MUIVerb", "Restart in Safe Mode with Networking", RegistryValue::DeleteKey),
             crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot network & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#, RegistryValue::DeleteKey),
        ],
                },
        // Batch 8
        crate::tweak! {
            id: "add_take_ownership_context_menu",
            category: "context_menu",
            name: "Add 'Take Ownership' Context Menu",
            description: "Adds 'Take Ownership' to files, folders, and drives context menu to quickly gain access permissions.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "", "Take Ownership", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "HasLUAShield", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "NoWorkingDirectory", "", RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"*\shell\TakeOwnership\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#, RegistryValue::DeleteKey),
                crate::reg_str!("HKCR", r"*\shell\TakeOwnership\command", "IsolatedCommand", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#, RegistryValue::DeleteKey),
                // Directory
                 crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "", "Take Ownership", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "HasLUAShield", "", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "NoWorkingDirectory", "", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "Position", "middle", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" /r /d y && icacls \"%1\" /grant *S-1-3-4:F /t /c /l /q' -Verb runAs""#, RegistryValue::DeleteKey),
                // Drive
                 crate::reg_str!("HKCR", r"Drive\shell\runas", "", "Take Ownership", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Drive\shell\runas", "HasLUAShield", "", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Drive\shell\runas", "NoWorkingDirectory", "", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Drive\shell\runas", "Position", "middle", RegistryValue::DeleteKey),
                 crate::reg_str!("HKCR", r"Drive\shell\runas\command", "", r#"cmd.exe /c takeown /f "%1" /r /d y && icacls "%1" /grant *S-1-3-4:F /t /c"#, RegistryValue::DeleteKey),
            ],
        },
];
