// Context Menu tweaks

use super::Tweak;

pub const REMOVE_CONTEXT_PINS: Tweak = crate::tweak! {
id: "remove_context_menu_pins",
category: "context_menu",
name: "Remove 'Pin to Start' and 'Pin to Quick Access'",
description: "Removes pinning options from the context menu to reduce clutter.",
enabled_ops: &[
        // Remove Pin to Home (Quick Access)
        crate::reg_del!("HKCR", r"AllFilesystemObjects\shell\pintohome", ""),
        crate::reg_del!("HKCR", r"Drive\shell\pintohome", ""),
        crate::reg_del!("HKCR", r"Folder\shell\pintohome", ""),
        crate::reg_del!("HKCR", r"Network\shell\pintohome", ""),
        // Remove Pin to Start
        crate::reg_del!("HKCR", r"exefile\shellex\ContextMenuHandlers\PintoStartScreen", ""),
        crate::reg_del!("HKCR", r"Folder\ShellEx\ContextMenuHandlers\PintoStartScreen", ""),
        crate::reg_del!("HKCR", r"Microsoft.Website\shellex\ContextMenuHandlers\PintoStartScreen", ""),
        crate::reg_del!("HKCR", r"mscfile\shellex\ContextMenuHandlers\PintoStartScreen", ""),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{470C0EBD-5D73-4d58-9CED-E91E22E23282}", ""),
],
};

pub const REMOVE_TERMINAL: Tweak = crate::tweak! {
id: "remove_open_in_terminal",
category: "context_menu",
name: "Remove 'Open in Terminal'",
description: "Removes 'Open in Terminal' from the context menu.",
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{9F156763-7844-4DC4-B2B1-901F640F5155}", ""),
],
};

pub const REMOVE_EDIT_NOTEPAD: Tweak = crate::tweak! {
id: "remove_edit_in_notepad",
category: "context_menu",
name: "Remove 'Edit in Notepad'",
description: "Removes 'Edit in Notepad' from the file context menu.",
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{CA6CC9F1-867A-481E-951E-A28C5E4F01EA}", ""),
],
};

pub const REMOVE_MOVE_TO_ONEDRIVE: Tweak = crate::tweak! {
id: "remove_move_to_onedrive",
category: "context_menu",
name: "Remove 'Move to OneDrive'",
description: "Removes 'Move to OneDrive' from the context menu.",
enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", "OneDrive"),
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{1FA0E654-C9F2-4A1F-9800-B9A75D744B00}", "OneDrive"),
],
};

pub const REMOVE_PHOTOS_MENU: Tweak = crate::tweak! {
id: "remove_photos_menu",
category: "context_menu",
name: "Remove 'Photos' Menu Items",
description: "Removes 'Edit with Photos' and other Photos app items from the context menu.",
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
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{7AD84985-87B4-4a16-BE58-8B72A5B390F7}", "Play to Menu"),
],
};

pub const REMOVE_ASK_COPILOT: Tweak = crate::tweak! {
id: "remove_ask_copilot",
category: "context_menu",
name: "Remove 'Ask Copilot' from Context Menu",
description: "Removes default 'Ask Copilot' entry from context menu on files.",
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
enabled_ops: &[
        crate::reg_del_key!("HKCR", r"Directory\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", ""),
        crate::reg_del_key!("HKCR", r"Drive\shellex\PropertySheetHandlers\{ef43ecfe-2ab9-4632-bf21-58909dd177f0}", ""),
        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCustomizeThisFolder", 1),
],
};

pub const REMOVE_CHANGE_BITLOCKER_PASSWORD: Tweak = crate::tweak! {
id: "remove_change_bitlocker_password",
category: "context_menu",
name: "Remove 'Change BitLocker password'",
description: "Removes the 'Change BitLocker password' context menu option.",
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Classes\Drive\shell\change-passphrase", "LegacyDisable", ""),
],
};

pub const REMOVE_COPY_AS_PATH: Tweak = crate::tweak! {
id: "remove_copy_as_path",
category: "context_menu",
name: "Remove 'Copy as path'",
description: "Removes the 'Copy as path' context menu option.",
enabled_ops: &[
        crate::reg_del_key!("HKLM", r"SOFTWARE\Classes\AllFilesystemObjects\shellex\ContextMenuHandlers\CopyAsPathMenu", ""),
],
};

pub const REMOVE_TROUBLESHOOT_COMPATIBILITY: Tweak = crate::tweak! {
id: "remove_troubleshoot_compatibility",
category: "context_menu",
name: "Remove 'Troubleshoot compatibility'",
description: "Removes 'Troubleshoot compatibility' from the context menu of executables.",
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Classes\exefile\shell\compatibility", "LegacyDisable", ""),
],
};

pub const REMOVE_PIN_TO_START: Tweak = crate::tweak! {
id: "remove_pin_to_start",
category: "context_menu",
name: "Remove 'Pin to Start'",
description: "Removes 'Pin to Start' from the context menu.",
enabled_ops: &[
        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{470C0EBD-5D73-4d58-9CED-E91E22E23282}", ""),
],
};

pub const REMOVE_NVIDIA_CONTROL_PANEL: Tweak = crate::tweak! {
id: "remove_nvidia_context",
category: "context_menu",
name: "Remove 'NVIDIA Control Panel'",
description: "Removes the NVIDIA Control Panel from the desktop context menu.",
enabled_ops: &[
        crate::reg_dword!("HKCU", r"Software\NVIDIA Corporation\Global\NvCplApi\Policies", "ContextUIPolicy", 0),
],
};

pub const REMOVE_PERSONALIZE_DISPLAY: Tweak = crate::tweak! {
        id: "remove_personalize_display",
        category: "context_menu",
        name: "Remove 'Personalize' and 'Display Settings'",
        description: "Removes 'Personalize' and 'Display settings' from the desktop context menu.",
        enabled_ops: &[
                crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\Personalize", ""),
                crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\Display", ""),
        ],
};

pub const ADD_ACCOUNTS_SETTINGS_MENU: Tweak = crate::tweak! {
    id: "add_accounts_settings_menu",
    category: "context_menu",
    name: "Add 'Accounts Settings' Menu",
    description: "Adds a menu to the desktop context menu for quick access to various Account settings.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "MUIVerb", "Accounts Settings"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "Icon", "imageres.dll,-88"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\001menu", "MUIVerb", "Accounts"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\001menu\command", "", "explorer ms-settings:accounts"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\002menu", "MUIVerb", "Your info"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\002menu\command", "", "explorer ms-settings:yourinfo"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\003menu", "MUIVerb", "Sign-in options"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AccountsSettings\shell\003menu\command", "", "explorer ms-settings:signinoptions"),
    ],
};

pub const ADD_NEW_BAT: Tweak = crate::tweak! {
    id: "add_new_bat",
    category: "context_menu",
    name: "Add 'Batch File' to New Menu",
    description: "Adds 'Windows Batch File' to the 'New' context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r".bat\ShellNew", "NullFile", ""),
    ],
};

pub const ADD_WINSXS_CLEANUP: Tweak = crate::tweak! {
    id: "add_winsxs_cleanup",
    category: "context_menu",
    name: "Add 'Component Store Cleanup'",
    description: "Adds DISM component store cleanup options to the desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "MUIVerb", "Component Store Cleanup"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "Icon", "cleanmgr.exe"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\WinSxS", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\001menu", "MUIVerb", "Analyze Component Store"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\001menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /AnalyzeComponentStore' -Verb runAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\002menu", "MUIVerb", "Clean Up Component Store"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\WinSxS\shell\002menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, DISM /Online /Cleanup-Image /StartComponentCleanup' -Verb runAs""#),
    ],
};

pub const ADD_PS1_EDIT_RUN: Tweak = crate::tweak! {
    id: "add_ps1_edit_run",
    category: "context_menu",
    name: "Add 'Edit or Run with' to PS1 Files",
    description: "Adds a comprehensive 'Edit or Run with' submenu to .ps1 PowerShell scripts.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", "MUIVerb", "Edit or Run with"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with", "SubCommands", ""),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", "MUIVerb", "Run with PowerShell"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout", "Icon", "powershell.exe"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\001flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe" "-Command" "if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'""#),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "MUIVerb", "Run with PowerShell as administrator"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout", "Icon", "powershell.exe"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\002flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe" "-Command" ""& {Start-Process PowerShell.exe -ArgumentList '-ExecutionPolicy RemoteSigned -File \"%1\"' -Verb RunAs}""#),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", "MUIVerb", "Edit with PowerShell ISE"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout", "Icon", "powershell_ise.exe"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\005flyout\Command", "", r#""C:\Windows\System32\WindowsPowerShell\v1.0\powershell_ise.exe" "%1""#),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", "MUIVerb", "Edit with Notepad"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout", "Icon", "notepad.exe"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\Edit-Run-with\shell\009flyout\Command", "", r#""C:\Windows\System32\notepad.exe" "%1""#),
    ],
};

pub const ADD_EDIT_WITH_PAINT: Tweak = crate::tweak! {
    id: "add_edit_with_paint_context_menu",
    category: "context_menu",
    name: "Add 'Edit with Paint' Context Menu",
    description: "Restores the 'Edit with Paint' context menu option for images.",
    enabled_ops: &[
        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{2430F218-B743-4FD6-97BF-5C76541B4AE9}"),
        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{2430F218-B743-4FD6-97BF-5C76541B4AE9}"),
    ],
};

pub const ADD_EMPTY_FOLDER: Tweak = crate::tweak! {
    id: "add_empty_folder_context_menu",
    category: "context_menu",
    name: "Add 'Empty folder' Context Menu",
    description: "Adds an option to empty the contents of a folder.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "Icon", "shell32.dll,-16715"),
        crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "MUIVerb", "Empty folder"),
        crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder", "Position", "bottom"),
        crate::reg_str!("HKCR", r"Directory\shell\EmptyFolder\command", "", r#"cmd /c title Empty "%1" & (cmd /c echo. & echo This will permanently delete all contents in only this folder and not subfolders. & echo. & choice /c:yn /m "Are you sure?") & (if errorlevel 2 exit) & (cmd /c "cd /d %1 && del /f /q *.*")"#),
    ],
};

pub const ADD_EMPTY_RECYCLE_BIN: Tweak = crate::tweak! {
    id: "add_empty_recycle_bin_context_menu",
    category: "context_menu",
    name: "Add 'Empty Recycle Bin' Context Menu",
    description: "Adds 'Empty Recycle Bin' to the desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "CommandStateHandler", "{c9298eef-69dd-4cdd-b153-bdbc38486781}"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "Description", "@shell32.dll,-31332"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "Icon", "%SystemRoot%\\System32\\imageres.dll,-54"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\empty", "MUIVerb", "@shell32.dll,-10564"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\empty\command", "DelegateExecute", "{48527bb3-e8de-450b-8910-8c4099cb8624}"),
    ],
};

pub const ADD_ENCRYPT_DECRYPT: Tweak = crate::tweak! {
    id: "add_encrypt_decrypt_context_menu",
    category: "context_menu",
    name: "Add 'Encrypt' and 'Decrypt' to Context Menu",
    description: "Adds EFS Encrypt and Decrypt options to the right-click menu.",
    enabled_ops: &[
        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "EncryptionContextMenu", 1),
    ],
};

pub const ADD_FIND_EMPTY_FOLDERS: Tweak = crate::tweak! {
    id: "add_find_empty_folders",
    category: "context_menu",
    name: "Add 'Find and Delete Empty Folders'",
    description: "Adds an option to find and recursively delete empty folders.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders", "", "Find and Delete Empty Folders"),
        crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders", "Icon", "shell32.dll,-16715"),
        crate::reg_str!("HKCR", r"Directory\shell\FindAndDeleteEmptyFolders\command", "", r#"powershell -NoProfile -Command "& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}"#),
        crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders", "", "Find and Delete Empty Folders"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders", "Icon", "imageres.dll,-1025"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\FindAndDeleteEmptyFolders\command", "", r#"powershell -NoProfile -Command "& {Get-ChildItem -Path '%1' -Recurse -Directory | Where-Object {!(Get-ChildItem -Path $_.FullName)} | ForEach-Object {Write-Host 'Empty folder found:' $_.FullName; $response = Read-Host 'Do you want to delete this folder (Y/N)?'; If ($response -eq 'Y') {Remove-Item -Path $_.FullName -Force}}}"#),
    ],
};

pub const ADD_GIVE_ACCESS_TO_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_give_access_to_context_menu",
    category: "context_menu",
    name: "Add 'Give Access to' Context Menu",
    description: "Restores the 'Give access to' (Sharing) option in context menus. Useful for network sharing.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
        crate::reg_str!("HKCR", r"Directory\Background\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
        crate::reg_str!("HKCR", r"Directory\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
        crate::reg_str!("HKCR", r"Drive\shellex\ContextMenuHandlers\Sharing", "", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"),
    ],
};

pub const ADD_HASH_VALUE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_hash_value_context_menu",
    category: "context_menu",
    name: "Add 'Hash value' Context Menu",
    description: "Adds a context menu to calculate file hashes (MD5, SHA1, SHA256, etc.).",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shell\hash", "MUIVerb", "Hash value"),
        crate::reg_str!("HKCR", r"*\shell\hash", "subCommands", ""),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\01menu", "MUIVerb", "MD5"),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\01menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm MD5 ^| format-list; Start-Sleep 3600"#),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\03menu", "MUIVerb", "SHA256"),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\03menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $file=[string]$input; get-filehash -literalpath $file.substring(2,$file.length - 5) -algorithm SHA256 ^| format-list; Start-Sleep 3600"#),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\08menu", "MUIVerb", "Show all"),
        crate::reg_str!("HKCR", r"*\shell\hash\shell\08menu\command", "", r#"cmd /c echo \"%1\" | powershell -nop $raw=[string]$input; $file=$raw.substring(2,$raw.length - 5); \"Path:`n$file`n\"; @(foreach ($a in @('MD5','SHA1','SHA256','SHA384','SHA512','MACTripleDES','RIPEMD160')) { get-filehash -literalpath $file -algorithm $a }) ^| foreach { \"$($_.Algorithm):`n$($_.Hash)`n\" }; Start-Sleep 3600"#),
    ],
};

pub const ADD_HIDDEN_ITEMS_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_hidden_items_context_menu",
    category: "context_menu",
    name: "Add 'Hidden items' Context Menu",
    description: "Adds a menu to toggle Hidden Items and Protected OS Files visibility.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "Icon", "imageres.dll,-5314"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "MUIVerb", "Hidden items"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles", "SubCommands", ""),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "CommandStateSync", ""),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "Description", "@shell32.dll,-37573"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "ExplorerCommandHandler", "{f7300245-1f4b-41ba-8948-6fd392064494}"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "Icon", "imageres.dll,-5314"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\HiddenFiles\shell\Windows.ShowHiddenFiles", "MUIVerb", "Hide/Show Hidden items"),
    ],
};

pub const ADD_INCLUDE_IN_LIBRARY_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_include_in_library_context_menu",
    category: "context_menu",
    name: "Add 'Include in library' Context Menu",
    description: "Restores the 'Include in library' option for folders.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Folder\ShellEx\ContextMenuHandlers\Library Location", "", "{3dad6c5d-2167-4cae-9914-f99e41c12cfa}"),
    ],
};

pub const ADD_INSTALL_CAB_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_install_cab_context_menu",
    category: "context_menu",
    name: "Add 'Install' to CAB Files",
    description: "Adds an 'Install' option for .cab files using DISM.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs", "", "Install"),
        crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"CABFolder\Shell\RunAs\Command", "", r#"cmd /k dism /online /add-package /packagepath:"%1""#),
    ],
};

pub const ADD_LOCATION_SERVICES_MENU: Tweak = crate::tweak! {
    id: "add_location_services_menu",
    category: "context_menu",
    name: "Add 'Location Services' Menu",
    description: "Adds a menu to quickly turn on/off location services for the device or apps.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "MUIVerb", "Location Services"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "Icon", "taskbarcpl.dll,-10"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\001flyout", "MUIVerb", "Turn On for Device"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\001flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d \"Allow\" /f' -Verb runAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\002flyout", "MUIVerb", "Turn Off for Device"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Location\Shell\002flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, Reg Add HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location /v Value /t REG_SZ /d \"Deny\" /f' -Verb runAs""#),
    ],
};

pub const ADD_LOCK_BDE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_lock_bde_context_menu",
    category: "context_menu",
    name: "Add 'Lock Drive' (BitLocker)",
    description: "Adds a 'Lock Drive' option for BitLocker-encrypted drives.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "AppliesTo", "System.Volume.BitLockerProtection:=1 OR System.Volume.BitLockerProtection:=3 OR System.Volume.BitLockerProtection:=5 NOT C:"),
        crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "", "Lock Drive"),
        crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Drive\shell\lock-bde", "MultiSelectModel", "Single"),
        crate::reg_str!("HKCR", r"Drive\shell\lock-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, lock-bde %1' -Verb runAs""#),
    ],
};

pub const ADD_OPEN_IN_NEW_PROCESS: Tweak = crate::tweak! {
    id: "add_open_in_new_process",
    category: "context_menu",
    name: "Add 'Open in new process'",
    description: "Adds 'Open in new process' to the folder context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "ExplorerHost", "{ceff45ee-c862-41de-aee2-a022c81eda92}"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "Extended", ""),
        crate::reg_dword!("HKCR", r"Folder\shell\opennewprocess", "LaunchExplorerFlags", 3),
        crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "MUIVerb", "@shell32.dll,-8518"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewprocess", "MultiSelectModel", "Document"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewprocess\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}"),
    ],
};

pub const ADD_OPEN_IN_NEW_TAB: Tweak = crate::tweak! {
    id: "add_open_in_new_tab",
    category: "context_menu",
    name: "Add 'Open in new tab'",
    description: "Adds 'Open in new tab' to the folder context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "CommandStateHandler", "{11dbb47c-a525-400b-9e80-a54615a090c0}"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "CommandStateSync", ""),
        crate::reg_dword!("HKCR", r"Folder\shell\opennewtab", "LaunchExplorerFlags", 32),
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "MUIVerb", "@windows.storage.dll,-8519"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "MultiSelectModel", "Document"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab", "OnlyInBrowserWindow", ""),
        crate::reg_str!("HKCR", r"Folder\shell\opennewtab\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}"),
    ],
};

pub const ADD_OPEN_IN_NEW_WINDOW: Tweak = crate::tweak! {
    id: "add_open_in_new_window",
    category: "context_menu",
    name: "Add 'Open in new window'",
    description: "Adds 'Open in new window' to the folder context menu.",
    enabled_ops: &[
        crate::reg_dword!("HKCR", r"Folder\shell\opennewwindow", "LaunchExplorerFlags", 1),
        crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "MUIVerb", "@windows.storage.dll,-8517"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "MultiSelectModel", "Document"),
        crate::reg_str!("HKCR", r"Folder\shell\opennewwindow", "OnlyInBrowserWindow", ""),
        crate::reg_str!("HKCR", r"Folder\shell\opennewwindow\command", "DelegateExecute", "{11dbb47c-a525-400b-9e80-a54615a090c0}"),
    ],
};

pub const ADD_OPEN_LINUX_SHELL_HERE: Tweak = crate::tweak! {
    id: "add_open_linux_shell_here",
    category: "context_menu",
    name: "Add 'Open Linux Shell here'",
    description: "Adds 'Open Linux shell here' (WSL) to Directory background context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\Background\shell\WSL", "", "@wsl.exe,-2"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\WSL", "Extended", ""),
        crate::reg_str!("HKCR", r"Directory\Background\shell\WSL\command", "", r#"wsl.exe --cd "%V""#),
        crate::reg_str!("HKCR", r"Directory\shell\WSL", "", "@wsl.exe,-2"),
        crate::reg_str!("HKCR", r"Directory\shell\WSL", "Extended", ""),
        crate::reg_str!("HKCR", r"Directory\shell\WSL\command", "", r#"wsl.exe --cd "%V""#),
    ],
};

pub const ADD_POWERSHELL_HERE: Tweak = crate::tweak! {
    id: "add_powershell_here",
    category: "context_menu",
    name: "Add 'Open PowerShell window here'",
    description: "Adds standard 'Open PowerShell window here' to Directory context menus.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\Background\shell\Powershell", "", "@shell32.dll,-8508"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#),
        crate::reg_str!("HKCR", r"Directory\shell\Powershell", "", "@shell32.dll,-8508"),
        crate::reg_str!("HKCR", r"Directory\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#),
        crate::reg_str!("HKCR", r"Drive\shell\Powershell", "", "@shell32.dll,-8508"),
        crate::reg_str!("HKCR", r"Drive\shell\Powershell\command", "", r#"powershell.exe -noexit -command Set-Location -literalPath "%V""#),
    ],
};

pub const ADD_OPEN_WITH_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_open_with_context_menu",
    category: "context_menu",
    name: "Add 'Open with' Context Menu",
    description: "Restores the generic 'Open with' context menu handler.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shellex\ContextMenuHandlers\Open With", "", "{09799AFB-AD67-11d1-ABCD-00C04FC30936}"),
    ],
};

pub const ADD_OPEN_WITH_TO_URL: Tweak = crate::tweak! {
    id: "add_open_with_to_url",
    category: "context_menu",
    name: "Add 'Open with' to URL files",
    description: "Adds the 'Open with' context menu option to .URL internet shortcuts.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"InternetShortcut\ShellEx\ContextMenuHandlers\Open With", "", "{09799AFB-AD67-11d1-ABCD-00C04FC30936}"),
    ],
};

pub const ADD_PERMANENTLY_DELETE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_permanently_delete_context_menu",
    category: "context_menu",
    name: "Add 'Permanently Delete' Context Menu",
    description: "Adds 'Permanently Delete' to the context menu to bypass Recycle Bin.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "ExplorerCommandHandler", "{E9571AB2-AD92-4ec6-8924-4E5AD33790F5}"),
        crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "CommandStateSync", ""),
        crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "Icon", "shell32.dll,-240"),
        crate::reg_str!("HKCR", r"AllFilesystemObjects\shell\Windows.PermanentDelete", "Position", "Bottom"),
    ],
};

pub const ADD_PERSONALIZE_CLASSIC_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_personalize_classic_context_menu",
    category: "context_menu",
    name: "Add 'Personalize (classic)' Context Menu",
    description: "Adds a classic 'Personalize' menu with quick access to Theme, Wallpaper, etc.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "MUIVerb", "Personalize (classic)"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "Icon", "themecpl.dll"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\001flyout", "MUIVerb", "Theme Settings"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\001flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921}"#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\002flyout", "MUIVerb", "Desktop Background"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\002flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageWallpaper"#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\003flyout", "MUIVerb", "Color and Appearance"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\003flyout\command", "", r#"explorer shell:::{ED834ED6-4B5A-4bfe-8F11-A626DCB6A921} -Microsoft.Personalization\pageColorization"#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\006flyout", "MUIVerb", "Desktop Icon Settings"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Personalization\shell\006flyout\command", "", r#"rundll32.exe shell32.dll,Control_RunDLL desk.cpl,,0"#),
    ],
};

pub const ADD_READ_ONLY_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_read_only_context_menu",
    category: "context_menu",
    name: "Add 'Read-only' Context Menu",
    description: "Adds 'Read-only' toggle to files and folders context menu for quick attribute changes.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shell\Read-only", "MUIVerb", "Read-only"),
        crate::reg_str!("HKCR", r"*\shell\Read-only", "SubCommands", ""),
        crate::reg_str!("HKCR", r"*\shell\Read-only\shell\001menu", "MUIVerb", "Apply read-only"),
        crate::reg_str!("HKCR", r"*\shell\Read-only\shell\001menu\command", "", r#"attrib +r "%1""#),
        crate::reg_str!("HKCR", r"*\shell\Read-only\shell\002menu", "MUIVerb", "Clear read-only"),
        crate::reg_str!("HKCR", r"*\shell\Read-only\shell\002menu\command", "", r#"attrib -r "%1""#),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only", "MUIVerb", "Read-only"),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only", "SubCommands", ""),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\001menu", "MUIVerb", "Apply read-only (recursive)"),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\001menu\command", "", r#"cmd /c attrib +r "%1" & attrib +r "%1\*.*" /s /d"#),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\002menu", "MUIVerb", "Clear read-only (recursive)"),
        crate::reg_str!("HKCR", r"Directory\shell\Read-only\shell\002menu\command", "", r#"cmd /c attrib -r "%1" & attrib -r "%1\*.*" /s /d"#),
    ],
};

pub const ADD_REGISTER_DLL_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_register_dll_context_menu",
    category: "context_menu",
    name: "Add 'Register DLL' Context Menu",
    description: "Adds 'Register Server' and 'Unregister Server' for .dll and .ocx files.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"dllfile\shell\Register\command", "", "regsvr32.exe \"%1\""),
        crate::reg_str!("HKCR", r"dllfile\shell\Unregister\command", "", "regsvr32.exe /u \"%1\""),
        crate::reg_str!("HKCR", r"ocxfile\shell\Register\command", "", "regsvr32.exe \"%1\""),
        crate::reg_str!("HKCR", r"ocxfile\shell\Unregister\command", "", "regsvr32.exe /u \"%1\""),
    ],
};

pub const ADD_NEW_REG: Tweak = crate::tweak! {
    id: "add_new_reg",
    category: "context_menu",
    name: "Add 'Registry File' to New Menu",
    description: "Adds 'Registration Entries (REG)' to the 'New' context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r".reg\ShellNew", "NullFile", ""),
    ],
};

pub const ADD_RESET_PERMISSIONS_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_reset_permissions_context_menu",
    category: "context_menu",
    name: "Add 'Reset Permissions' Context Menu",
    description: "Adds 'Reset Permissions' (icacls reset) to files and folders to fix access issues.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shell\ResetPermissions", "MUIVerb", "Reset Permissions"),
        crate::reg_str!("HKCR", r"*\shell\ResetPermissions\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset' -Verb runAs""#),
        crate::reg_str!("HKCR", r"Directory\shell\ResetPermissions", "MUIVerb", "Reset Permissions"),
        crate::reg_str!("HKCR", r"Directory\shell\ResetPermissions\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c icacls \"%1\" /reset /t /c /l' -Verb runAs""#),
    ],
};

pub const ADD_RESTART_EXPLORER_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_restart_explorer_context_menu",
    category: "context_menu",
    name: "Add 'Restart Explorer' Context Menu",
    description: "Adds 'Restart Explorer' to the Desktop context menu for quick shell restart.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer", "MUIVerb", "Restart Explorer"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer", "Icon", "explorer.exe"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer\Position", "", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\RestartExplorer\command", "", r#"cmd.exe /c taskkill /f /im explorer.exe & start explorer.exe"#),
    ],
};

pub const ADD_ROTATE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_rotate_context_menu",
    category: "context_menu",
    name: "Add 'Rotate' Context Menu",
    description: "Restores 'Rotate left' and 'Rotate right' options for common image formats.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", "", "{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}"),
    ],
};

pub const ADD_RUN_AS_ADMINISTRATOR_CMD_BAT: Tweak = crate::tweak! {
    id: "add_run_as_administrator_cmd_bat",
    category: "context_menu",
    name: "Add 'Run as administrator' Context Menu",
    description: "Ensures 'Run as administrator' is available for BAT, CMD, EXE, MSC files.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"batfile\shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"cmdfile\shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"exefile\shell\runas", "HasLUAShield", ""),
    ],
};

pub const ADD_RUN_AS_ADMINISTRATOR_MSI: Tweak = crate::tweak! {
    id: "add_run_as_administrator_msi",
    category: "context_menu",
    name: "Add 'Run as administrator' for MSI",
    description: "Adds 'Run as administrator' option to .msi file context menu for installing as admin.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Msi.Package\Shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Msi.Package\Shell\runas\command", "", r#"msiexec /i "%1""#),
    ],
};

pub const ADD_RUN_AS_ADMINISTRATOR_PS1: Tweak = crate::tweak! {
    id: "add_run_as_administrator_ps1",
    category: "context_menu",
    name: "Add 'Run as administrator' for PS1",
    description: "Adds 'Run as administrator' option to .ps1 file context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"SystemFileAssociations\.ps1\Shell\runas\command", "", r#"powershell.exe "-Command" "if((Get-ExecutionPolicy ) -ne 'AllSigned') { Set-ExecutionPolicy -Scope Process Bypass }; & '%1'""#),
    ],
};

pub const ADD_RUN_AS_ADMINISTRATOR_VBS: Tweak = crate::tweak! {
    id: "add_run_as_administrator_vbs",
    category: "context_menu",
    name: "Add 'Run as administrator' for VBS",
    description: "Adds 'Run as administrator' option to .vbs file context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"VBSFile\Shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"VBSFile\Shell\runas\command", "", r#"wscript.exe "%1" %*"#),
    ],
};

pub const ADD_RUN_AS_DIFFERENT_USER: Tweak = crate::tweak! {
    id: "add_run_as_different_user",
    category: "context_menu",
    name: "Add 'Run as different user'",
    description: "Adds 'Run as different user' option to the context menu for executable files.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"batfile\shell\runasuser", "", "@shell32.dll,-50944"),
        crate::reg_del!("HKCR", r"batfile\shell\runasuser", "Extended"),
        crate::reg_str!("HKCR", r"batfile\shell\runasuser", "SuppressionPolicyEx", "{F211AA05-D4DF-4370-A2A0-9F19C09756A7}"),
        crate::reg_str!("HKCR", r"batfile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}"),
        crate::reg_str!("HKCR", r"cmdfile\shell\runasuser", "", "@shell32.dll,-50944"),
        crate::reg_del!("HKCR", r"cmdfile\shell\runasuser", "Extended"),
        crate::reg_str!("HKCR", r"cmdfile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}"),
        crate::reg_str!("HKCR", r"exefile\shell\runasuser", "", "@shell32.dll,-50944"),
        crate::reg_del!("HKCR", r"exefile\shell\runasuser", "Extended"),
        crate::reg_str!("HKCR", r"exefile\shell\runasuser\command", "DelegateExecute", "{ea72d00e-4960-42fa-ba92-7792a7944c1d}"),
    ],
};

pub const ADD_SAFE_MODE_DESKTOP_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_safe_mode_desktop_context_menu",
    category: "context_menu",
    name: "Add 'Safe Mode' Context Menu",
    description: "Adds 'Safe Mode' boot options (Normal, Safe, Network) to Desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "MUIVerb", "Safe Mode"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode", "MUIVerb", "Restart in Normal Mode"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\001-NormalMode\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /deletevalue {current} safeboot & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode", "MUIVerb", "Restart in Safe Mode"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\002-SafeMode\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot minimal & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking", "MUIVerb", "Restart in Safe Mode with Networking"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SafeMode\shell\003-SafeModeNetworking\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c,bcdedit /set {current} safeboot network & bcdedit /deletevalue {current} safebootalternateshell & shutdown -r -t 00 -f' -Verb runAs""#),
    ],
};

pub const ADD_SEND_TO_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_send_to_context_menu",
    category: "context_menu",
    name: "Add 'Send To' Context Menu",
    description: "Restores the 'Send To' submenu in context menus.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"AllFilesystemObjects\shellex\ContextMenuHandlers\SendTo", "", "{7BA4C740-9E81-11CF-99D3-00AA004AE837}"),
    ],
};

pub const ADD_SHARE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_share_context_menu",
    category: "context_menu",
    name: "Add 'Share' Context Menu",
    description: "Restores the 'Share' option in context menus.",
    enabled_ops: &[
        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}"),
        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{e2bf9676-5f8f-435c-97eb-11607a5bedf7}"),
    ],
};

pub const ADD_BITLOCKER_SUSPEND: Tweak = crate::tweak! {
    id: "add_bitlocker_suspend",
    category: "context_menu",
    name: "Add 'Suspend BitLocker protection'",
    description: "Adds an option to temporarily suspend BitLocker protection for a drive.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "", "Suspend BitLocker protection"),
        crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "AppliesTo", "(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On"),
        crate::reg_str!("HKCR", r"Drive\shell\suspend-bde", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Drive\shell\suspend-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, manage-bde -protectors -disable %1' -Verb runAs""#),
    ],
};

pub const ADD_SYSTEM_PROTECTION_MENU: Tweak = crate::tweak! {
    id: "add_system_protection_menu",
    category: "context_menu",
    name: "Add 'System Protection and Restore' Menu",
    description: "Adds a menu for managing restore points and system protection.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "MUIVerb", "System Protection and Restore"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "Icon", "rstrui.exe"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\001flyout", "MUIVerb", "System Restore"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\001flyout\command", "", "rstrui.exe"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout", "MUIVerb", "Create restore point"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\SystemProtection\shell\002flyout\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs""#),
        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0),
    ],
};

pub const ADD_TAKE_OWNERSHIP_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_take_ownership_context_menu",
    category: "context_menu",
    name: "Add 'Take Ownership' Context Menu",
    description: "Adds 'Take Ownership' to files, folders, and drives context menu to quickly gain access permissions.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "", "Take Ownership"),
        crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"*\shell\TakeOwnership", "NoWorkingDirectory", ""),
        crate::reg_str!("HKCR", r"*\shell\TakeOwnership\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#),
        crate::reg_str!("HKCR", r"*\shell\TakeOwnership\command", "IsolatedCommand", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" && icacls \"%1\" /grant *S-1-3-4:F /t /c /l' -Verb runAs""#),
        crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "", "Take Ownership"),
        crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "NoWorkingDirectory", ""),
        crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership", "Position", "middle"),
        crate::reg_str!("HKCR", r"Directory\shell\TakeOwnership\command", "", r#"powershell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/c takeown /f \"%1\" /r /d y && icacls \"%1\" /grant *S-1-3-4:F /t /c /l /q' -Verb runAs""#),
        crate::reg_str!("HKCR", r"Drive\shell\runas", "", "Take Ownership"),
        crate::reg_str!("HKCR", r"Drive\shell\runas", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Drive\shell\runas", "NoWorkingDirectory", ""),
        crate::reg_str!("HKCR", r"Drive\shell\runas", "Position", "middle"),
        crate::reg_str!("HKCR", r"Drive\shell\runas\command", "", r#"cmd.exe /c takeown /f "%1" /r /d y && icacls "%1" /grant *S-1-3-4:F /t /c"#),
    ],
};

pub const ADD_BITLOCKER_TURN_OFF: Tweak = crate::tweak! {
    id: "add_bitlocker_turn_off",
    category: "context_menu",
    name: "Add 'Turn off BitLocker'",
    description: "Adds an option to completely decrypt and turn off BitLocker for a drive.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "", "Turn off BitLocker"),
        crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "AppliesTo", "(System.Volume.BitLockerProtection:=System.Volume.BitLockerProtection#On"),
        crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"Drive\shell\decrypt-bde\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, manage-bde -off %1' -Verb runAs""#),
    ],
};

pub const ADD_TURN_ON_BITLOCKER_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_turn_on_bitlocker_context_menu",
    category: "context_menu",
    name: "Add 'Turn on BitLocker' Context Menu",
    description: "Adds 'Turn on BitLocker' option to the context menu of drives.",
    enabled_ops: &[
        crate::reg_del!("HKCR", r"Drive\shell\encrypt-bde", "LegacyDisable"),
        crate::reg_del!("HKCR", r"Drive\shell\encrypt-bde-elev", "LegacyDisable"),
    ],
};

pub const ADD_USB_CONNECTIONS_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_usb_connections_context_menu",
    category: "context_menu",
    name: "Add 'USB connections' Menu",
    description: "Adds a menu to Enable or Disable new USB connections (useful for security).",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "Icon", "hotplug.dll,-100"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "MUIVerb", "USB connections"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu", "MUIVerb", "Enable new USB connections"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\01menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\SYSTEM\CurrentControlSet\Services\USBSTOR\" /V Start /T REG_DWORD /D 3 /F' -Verb RunAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu", "HasLUAShield", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu", "MUIVerb", "Disable new USB connections"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\USB\shell\02menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,REG ADD \"HKLM\SYSTEM\CurrentControlSet\Services\USBSTOR\" /V Start /T REG_DWORD /D 4 /F' -Verb RunAs""#),
    ],
};

pub const ADD_NEW_VBS: Tweak = crate::tweak! {
    id: "add_new_vbs",
    category: "context_menu",
    name: "Add 'VBScript File' to New Menu",
    description: "Adds 'VBScript Script File' to the 'New' context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r".vbs\ShellNew", "NullFile", ""),
    ],
};

pub const ADD_AUTO_HIDE_TASKBAR_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_auto_hide_taskbar_context_menu",
    category: "context_menu",
    name: "Add Automatically Hide Taskbar Context Menu",
    description: "Adds a menu to Turn On/Off 'Automatically hide the taskbar'.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "Icon", "imageres.dll,-80"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "MUIVerb", "Automatically hide taskbar"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\001flyout", "MUIVerb", "Turn on"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\001flyout\command", "", r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=3;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\002flyout", "MUIVerb", "Turn off"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\HideTaskbar\shell\002flyout\command", "", r#"powershell -command "&{$p='HKCU:SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3';$v=(Get-ItemProperty -Path $p).Settings;$v[8]=2;&Set-ItemProperty -Path $p -Name Settings -Value $v;&Stop-Process -f -ProcessName explorer}""#),
    ],
};

pub const ADD_BOOT_ADVANCED_STARTUP_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_boot_advanced_startup_context_menu",
    category: "context_menu",
    name: "Add Boot to Advanced Startup Context Menu",
    description: "Adds 'Boot to Advanced Startup' to Desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "icon", "shell32.dll,-16826"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "MUIVerb", "Boot to Advanced Startup"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\AdvancedStartup\command", "", "shutdown.exe /r /o /f /t 00"),
    ],
};

pub const ADD_CHANGE_NETWORK_LOCATION: Tweak = crate::tweak! {
    id: "add_change_network_location",
    category: "context_menu",
    name: "Add Change Network Location Context Menu",
    description: "Adds a menu to quickly switch between Private and Public network profiles.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "MUIVerb", "Network Location"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "Icon", "imageres.dll,-25"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "Position", "Middle"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\01menu", "MUIVerb", "Change to Private network"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\01menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Private' -Verb RunAs""#),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\02menu", "MUIVerb", "Change to Public network"),
        crate::reg_str!("HKCR", r"DesktopBackground\shell\NetworkLocation\shell\02menu\command", "", r#"PowerShell -windowstyle hidden -Command "Start-Process cmd -ArgumentList '/s,/c,PowerShell $net = get-netconnectionprofile;Set-NetConnectionProfile -Name $net.Name -NetworkCategory Public' -Verb RunAs""#),
    ],
};

pub const ADD_CHOOSE_POWER_PLAN_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_choose_power_plan_context_menu",
    category: "context_menu",
    name: "Add Choose Power Plan Context Menu",
    description: "Adds a menu to switch Power Plans (Balanced, High Performance, etc.).",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "Icon", "powercpl.dll"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "MUIVerb", "Choose Power Plan"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "Position", "Middle"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu", "MUIVerb", "Balanced"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu", "Icon", "powercpl.dll"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\01menu\command", "", "powercfg.exe /setactive 381b4222-f694-41f0-9685-ff5bb260df2e"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu", "MUIVerb", "High Performance"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu", "Icon", "powercpl.dll"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\02menu\command", "", "powercfg.exe /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu", "MUIVerb", "Power Saver"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu", "Icon", "powercpl.dll"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\PowerPlan\shell\03menu\command", "", "powercfg.exe /setactive a1841308-3541-4fab-bc81-f71556f20b4a"),
    ],
};

pub const ADD_CLOSE_ALL_APPS: Tweak = crate::tweak! {
    id: "add_close_all_apps",
    category: "context_menu",
    name: "Add Close All Apps Context Menu",
    description: "Adds 'Close All Apps' to the Desktop context menu to kill most user apps.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "MUIVerb", "Close All Apps"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "icon", "imageres.dll,-5102"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\CloseAllApps\command", "", r#"PowerShell -Command "Get-Process |? {$_.MainWindowTitle -ne \"\" -and $_.Id -ne $PID -and $_.ProcessName -ne \"explorer\"} | Stop-Process -Force""#),
    ],
};

pub const ADD_CONTROL_PANEL_DESKTOP: Tweak = crate::tweak! {
    id: "add_control_panel_desktop",
    category: "context_menu",
    name: "Add Control Panel to Desktop Context Menu",
    description: "Adds a cascaded 'Control Panel' menu to the desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "MUIVerb", "Control Panel"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "Icon", "imageres.dll,-27"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\001menu", "", "Category view"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\001menu\command", "", "explorer.exe shell:::{26EE0668-A00A-44D7-9371-BEB064C98683}"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\002menu", "", "Icons view"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\002menu\command", "", "explorer.exe shell:::{21EC2020-3AEA-1069-A2DD-08002B30309D}"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\003menu", "", "All Tasks (God mode)"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\ControlPanel\shell\003menu\command", "", "explorer.exe shell:::{ED7BA470-8E54-465E-825C-99712043E01C}"),
    ],
};

pub const ADD_CREATE_RESTORE_POINT: Tweak = crate::tweak! {
    id: "add_create_restore_point",
    category: "context_menu",
    name: "Add Create Restore Point Context Menu",
    description: "Adds 'Create Restore Point' to the background context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\Background\shell\Create Restore Point", "Icon", "SystemPropertiesProtection.exe"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\Create Restore Point\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/c, PowerShell Checkpoint-Computer -Description \"Manual\" -RestorePointType \"MODIFY_SETTINGS\"' -Verb runAs""#),
        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore", "SystemRestorePointCreationFrequency", 0),
    ],
};

pub const ADD_DISPLAY_SETTINGS_DESKTOP_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_display_settings_desktop_context_menu",
    category: "context_menu",
    name: "Add Display Settings Desktop Context Menu",
    description: "Adds direct 'Display Settings' link to Desktop context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "Icon", "display.dll,-1"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "MUIVerb", "Display settings"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Display\command", "DelegateExecute", "{556FF0D6-A1EE-49E5-9FA4-90AE116AD744}"),
    ],
};

pub const ADD_OPEN_WINDOWS_TERMINAL_EXPANDABLE: Tweak = crate::tweak! {
    id: "add_open_windows_terminal_expandable",
    category: "context_menu",
    name: "Add Open in Windows Terminal (Expandable)",
    description: "Adds an expandable 'Open in Windows Terminal' menu with Profiles.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Directory\shell\OpenWTHere", "MUIVerb", "Open in Windows Terminal"),
        crate::reg_str!("HKCR", r"Directory\shell\OpenWTHere", "SubCommands", ""),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\001flyout", "MUIVerb", "Default Profile"),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\001flyout\command", "", r#"cmd.exe /c start wt.exe -d "%1\.""#),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\002flyout", "MUIVerb", "Command Prompt"),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\002flyout\command", "", r#"cmd.exe /c start wt.exe -p "Command Prompt" -d "%1\.""#),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\003flyout", "MUIVerb", "PowerShell"),
        crate::reg_str!("HKCR", r"Directory\Shell\OpenWTHere\shell\003flyout\command", "", r#"cmd.exe /c start wt.exe -p "Windows PowerShell" -d "%1\.""#),
        crate::reg_str!("HKCR", r"Directory\Background\shell\OpenWTHere", "MUIVerb", "Open in Windows Terminal"),
        crate::reg_str!("HKCR", r"Directory\Background\shell\OpenWTHere", "SubCommands", ""),
        crate::reg_str!("HKCR", r"Directory\Background\Shell\OpenWTHere\shell\001flyout", "MUIVerb", "Default Profile"),
        crate::reg_str!("HKCR", r"Directory\Background\Shell\OpenWTHere\shell\001flyout\command", "", r#"cmd.exe /c start wt.exe -d "%V\.""#),
    ],
};

pub const ADD_OPTIMIZE_DRIVES_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "add_optimize_drives_context_menu",
    category: "context_menu",
    name: "Add Optimize to Context Menu of Drives",
    description: "Adds 'Optimize' option to Drive context menu (Defrag/Trim).",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Drive\shell\Optimize", "Icon", "dfrgui.exe"),
        crate::reg_str!("HKCR", r"Drive\shell\Optimize", "SubCommands", ""),
        crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\001menu", "MUIVerb", "Analyze Drive"),
        crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\001menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /A' -Verb runAs""#),
        crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\002menu", "MUIVerb", "Optimize Drive"),
        crate::reg_str!("HKCR", r"Drive\Shell\Optimize\shell\002menu\command", "", r#"PowerShell -windowstyle hidden -command "Start-Process cmd -ArgumentList '/s,/k, defrag %1 /O /H' -Verb runAs""#),
    ],
};

pub const ADD_SETTINGS_DESKTOP: Tweak = crate::tweak! {
    id: "add_settings_desktop",
    category: "context_menu",
    name: "Add Settings to Desktop Context Menu",
    description: "Adds a cascaded 'Settings' menu to the desktop context menu for quick access to various settings pages.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "Icon", "shell32.dll,-16826"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "MUIVerb", "&Settings"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "Position", "Bottom"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings", "SubCommands", ""),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu", "Icon", "shell32.dll,-51380"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu", "MUIVerb", "&Home"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\01menu\command", "", "explorer ms-settings:home"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu", "Icon", "shell32.dll,-35"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu", "MUIVerb", "&System"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\02menu\command", "", "explorer ms-settings:system"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu", "Icon", "BthpanContextHandler.dll,-200"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu", "MUIVerb", "&Bluetooth && devices"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\03menu\command", "", "explorer ms-settings:devices"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu", "Icon", "shell32.dll,-193"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu", "MUIVerb", "&Network && internet"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\04menu\command", "", "explorer ms-settings:network"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu", "Icon", "themecpl.dll,-1"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu", "MUIVerb", "&Personalization"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\05menu\command", "", "explorer ms-settings:personalization"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu", "Icon", "shell32.dll,-63010"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu", "MUIVerb", "&Apps"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\06menu\command", "", "explorer ms-settings:appsfeatures"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu", "Icon", "imageres.dll,-88"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu", "MUIVerb", "A&ccounts"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\07menu\command", "", "explorer ms-settings:accounts"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu", "Icon", "shell32.dll,-276"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu", "MUIVerb", "&Time && language"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\08menu\command", "", "explorer ms-settings:dateandtime"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu", "Icon", "DDORes.dll,-2207"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu", "MUIVerb", "&Gaming"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\09menu\command", "", "explorer ms-settings:gaming-gamebar"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu", "Icon", "imageres.dll,-86"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu", "MUIVerb", "Acc&essibility"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\10menu\command", "", "explorer ms-settings:easeofaccess"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu", "Icon", "%ProgramFiles%\\Windows Defender\\EppManifest.dll,-101"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu", "MUIVerb", "Pri&vacy && security"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\11menu\command", "", "explorer ms-settings:privacy"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu", "Icon", "imageres.dll,-1401"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu", "MUIVerb", "&Windows Update"),
        crate::reg_str!("HKCR", r"DesktopBackground\Shell\Settings\shell\12menu\command", "", "explorer ms-settings:windowsupdate"),
    ],
};

pub const REMOVE_COPY_AS_PATH_DRIVE: Tweak = crate::tweak! {
    id: "remove_copy_as_path_drive",
    category: "context_menu",
    name: "Remove 'Copy as path' (Drives)",
    description: "Removes 'Copy as path' from the context menu of drives.",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r"Drive\shellex\ContextMenuHandlers\CopyAsPathMenu", ""),
    ],
};

pub const REMOVE_MANAGE_BITLOCKER: Tweak = crate::tweak! {
    id: "remove_manage_bitlocker",
    category: "context_menu",
    name: "Remove 'Manage BitLocker'",
    description: "Removes 'Manage BitLocker' from the drive context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCR", r"Drive\shell\manage-bde", "LegacyDisable", ""),
    ],
};

pub const REMOVE_MAP_NETWORK_DRIVE: Tweak = crate::tweak! {
    id: "remove_map_network_drive",
    category: "context_menu",
    name: "Remove 'Map network drive'",
    description: "Removes 'Map network drive' and 'Disconnect network drive' from This PC context menu.",
    enabled_ops: &[
        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoNetConnectDisconnect", 1),
        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoNetConnectDisconnect", 1),
    ],
};

pub const REMOVE_NEXT_BACKGROUND_MENU: Tweak = crate::tweak! {
    id: "remove_next_background_menu",
    category: "context_menu",
    name: "Remove 'Next desktop background'",
    description: "Removes the 'Next desktop background' option from the desktop context menu (for Windows Spotlight).",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r"DesktopBackground\Shell\.SpotlightNextImage", ""),
    ],
};

pub const REMOVE_OPEN_AS_PORTABLE: Tweak = crate::tweak! {
    id: "remove_open_as_portable",
    category: "context_menu",
    name: "Remove 'Open as Portable Device'",
    description: "Removes 'Open as Portable Device' from the drive context menu.",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r"Drive\shellex\ContextMenuHandlers\{D6791A63-E7E2-4fee-BF52-5DED8E86E9B8}", ""),
    ],
};

pub const REMOVE_OPEN_FILE_LOCATION: Tweak = crate::tweak! {
    id: "remove_open_file_location",
    category: "context_menu",
    name: "Remove 'Open File/Folder Location'",
    description: "Removes 'Open file location' and 'Open folder location' from various context menus (search results, shortcuts, etc.)",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r".symlink\shellex\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"LibraryLocation\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"lnkfile\shellex\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"PinnedRecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"RecentDocument\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"RecommendationsFile\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", ""),
        crate::reg_del_key!("HKCR", r"Results\ShellEx\ContextMenuHandlers\OpenContainingFolderMenu", ""),
    ],
};

pub const REMOVE_OPEN_IN_TERMINAL_PREVIEW: Tweak = crate::tweak! {
    id: "remove_open_in_terminal_preview",
    category: "context_menu",
    name: "Remove 'Open in Terminal Preview'",
    description: "Removes the 'Open in Terminal Preview' option from the context menu.",
    enabled_ops: &[
        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Shell Extensions\Blocked", "{02DB545A-3E20-46DE-83A5-1329B1E88B6B}", ""),
    ],
};

pub const REMOVE_PIN_TO_QUICK_ACCESS: Tweak = crate::tweak! {
    id: "remove_pin_to_quick_access",
    category: "context_menu",
    name: "Remove 'Pin to Quick access'",
    description: "Removes 'Pin to Quick access' from the context menu.",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r"AllFilesystemObjects\shell\pintohome", ""),
        crate::reg_del_key!("HKCR", r"Drive\shell\pintohome", ""),
        crate::reg_del_key!("HKCR", r"Folder\shell\pintohome", ""),
        crate::reg_del_key!("HKCR", r"Network\shell\pintohome", ""),
    ],
};

pub const REMOVE_ROTATE_CONTEXT_MENU: Tweak = crate::tweak! {
    id: "remove_rotate_context_menu",
    category: "context_menu",
    name: "Remove 'Rotate left' and 'Rotate right'",
    description: "Removes the rotation options from the context menu of image files.",
    enabled_ops: &[
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.avci\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.avif\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.bmp\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.gif\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.ico\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpe\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpeg\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.jpg\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.png\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.tif\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.tiff\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
        crate::reg_del_key!("HKCR", r"SystemFileAssociations\.webp\ShellEx\ContextMenuHandlers\ShellImagePreview", ""),
    ],
};

pub const ADD_MENU_ITEMS: Tweak = crate::tweak! {
    id: "add_menu_items_category",
    category: "context_menu",
    name: "Add Menu Items",
    description: "Add useful items to the context menu.",
    sub_tweaks: &[
        &ADD_ACCOUNTS_SETTINGS_MENU,
        &ADD_AUTO_HIDE_TASKBAR_CONTEXT_MENU,
        &ADD_BITLOCKER_SUSPEND,
        &ADD_BITLOCKER_TURN_OFF,
        &ADD_BOOT_ADVANCED_STARTUP_CONTEXT_MENU,
        &ADD_CHANGE_NETWORK_LOCATION,
        &ADD_CHOOSE_POWER_PLAN_CONTEXT_MENU,
        &ADD_CLOSE_ALL_APPS,
        &ADD_CONTROL_PANEL_DESKTOP,
        &ADD_CREATE_RESTORE_POINT,
        &ADD_DISPLAY_SETTINGS_DESKTOP_CONTEXT_MENU,
        &ADD_EDIT_WITH_PAINT,
        &ADD_EMPTY_FOLDER,
        &ADD_EMPTY_RECYCLE_BIN,
        &ADD_ENCRYPT_DECRYPT,
        &ADD_FIND_EMPTY_FOLDERS,
        &ADD_GIVE_ACCESS_TO_CONTEXT_MENU,
        &ADD_HASH_VALUE_CONTEXT_MENU,
        &ADD_HIDDEN_ITEMS_CONTEXT_MENU,
        &ADD_INCLUDE_IN_LIBRARY_CONTEXT_MENU,
        &ADD_INSTALL_CAB_CONTEXT_MENU,
        &ADD_LOCATION_SERVICES_MENU,
        &ADD_LOCK_BDE_CONTEXT_MENU,
        &ADD_NEW_BAT,
        &ADD_NEW_REG,
        &ADD_NEW_VBS,
        &ADD_OPEN_IN_NEW_PROCESS,
        &ADD_OPEN_IN_NEW_TAB,
        &ADD_OPEN_IN_NEW_WINDOW,
        &ADD_OPEN_LINUX_SHELL_HERE,
        &ADD_OPEN_WINDOWS_TERMINAL_EXPANDABLE,
        &ADD_OPEN_WITH_CONTEXT_MENU,
        &ADD_OPEN_WITH_TO_URL,
        &ADD_OPTIMIZE_DRIVES_CONTEXT_MENU,
        &ADD_PERMANENTLY_DELETE_CONTEXT_MENU,
        &ADD_PERSONALIZE_CLASSIC_CONTEXT_MENU,
        &ADD_POWERSHELL_HERE,
        &ADD_PS1_EDIT_RUN,
        &ADD_READ_ONLY_CONTEXT_MENU,
        &ADD_REGISTER_DLL_CONTEXT_MENU,
        &ADD_RESET_PERMISSIONS_CONTEXT_MENU,
        &ADD_RESTART_EXPLORER_CONTEXT_MENU,
        &ADD_ROTATE_CONTEXT_MENU,
        &ADD_RUN_AS_ADMINISTRATOR_CMD_BAT,
        &ADD_RUN_AS_ADMINISTRATOR_MSI,
        &ADD_RUN_AS_ADMINISTRATOR_PS1,
        &ADD_RUN_AS_ADMINISTRATOR_VBS,
        &ADD_RUN_AS_DIFFERENT_USER,
        &ADD_SAFE_MODE_DESKTOP_CONTEXT_MENU,
        &ADD_SEND_TO_CONTEXT_MENU,
        &ADD_SETTINGS_DESKTOP,
        &ADD_SHARE_CONTEXT_MENU,
        &ADD_SYSTEM_PROTECTION_MENU,
        &ADD_TAKE_OWNERSHIP_CONTEXT_MENU,
        &ADD_TURN_ON_BITLOCKER_CONTEXT_MENU,
        &ADD_USB_CONNECTIONS_CONTEXT_MENU,
        &ADD_WINSXS_CLEANUP,
    ],
};

pub const REMOVE_MENU_ITEMS: Tweak = crate::tweak! {
    id: "remove_menu_items_category",
    category: "context_menu",
    name: "Remove Menu Items",
    description: "Remove unwanted items from the context menu.",
    sub_tweaks: &[
        &REMOVE_ASK_COPILOT,
        &REMOVE_CAST_TO_DEVICE,
        &REMOVE_CHANGE_BITLOCKER_PASSWORD,
        &REMOVE_CONTEXT_PINS,
        &REMOVE_COPY_AS_PATH,
        &REMOVE_COPY_AS_PATH_DRIVE,
        &REMOVE_CUSTOMIZE_THIS_FOLDER,
        &REMOVE_EDIT_NOTEPAD,
        &REMOVE_MANAGE_BITLOCKER,
        &REMOVE_MAP_NETWORK_DRIVE,
        &REMOVE_MOVE_TO_ONEDRIVE,
        &REMOVE_NEXT_BACKGROUND_MENU,
        &REMOVE_NVIDIA_CONTROL_PANEL,
        &REMOVE_OPEN_AS_PORTABLE,
        &REMOVE_OPEN_FILE_LOCATION,
        &REMOVE_OPEN_IN_TERMINAL_PREVIEW,
        &REMOVE_PERSONALIZE_DISPLAY,
        &REMOVE_PHOTOS_MENU,
        &REMOVE_PIN_TO_QUICK_ACCESS,
        &REMOVE_PIN_TO_START,
        &REMOVE_ROTATE_CONTEXT_MENU,
        &REMOVE_TERMINAL,
        &REMOVE_TROUBLESHOOT_COMPATIBILITY,
    ],
};

pub static CONTEXT_MENU_TWEAKS: &[Tweak] = &[
    ADD_MENU_ITEMS,
    REMOVE_MENU_ITEMS,
];
