// File Explorer tweaks

use super::Tweak;

pub static EXPLORER_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "add_devices_printers_nav",
        category: "explorer",
        name: "Add Devices and Printers to Navigation Pane",
        description: "Adds 'Devices and Printers' to the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "", "Device Center"),
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "System.IsPinnedToNamespaceTree", 1),
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "SortOrderIndex", 0x3c),
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "System.ApplicationName", "Microsoft.DevicesAndPrinters"),
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\DefaultIcon", "", r"%systemroot%\system32\DeviceCenter.dll,-1"),
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", "", r"%systemroot%\system32\DeviceCenter.dll"),
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", "LoadWithoutCOM", ""),
            crate::reg_str!("HKCU", r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", "ThreadingModel", "Both"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "", "Device Center"),
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{A8A91A66-3A7D-4424-8D24-04E180695C7A}", 1),
        ],
        },
        crate::tweak! {
        id: "add_devices_and_printers_this_pc",
        category: "explorer",
        name: "Add Devices and Printers to This PC",
        description: "Adds 'Devices and Printers' to the 'This PC' view in File Explorer.",
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", "", "Devices and Printers"),
        ],
        },
        crate::tweak! {
        id: "add_explorer_menu_bar",
        category: "explorer",
        name: "Always Show Menu Bar",
        description: "Forces the classic menu bar (File, Edit, View, Tools) to be always visible in File Explorer.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "AlwaysShowMenus", 1),
        ],
                },
        crate::tweak! {
        id: "max_jump_list_items",
        category: "explorer",
        name: "Change Maximum Number of Items in Jump Lists",
        description: "Increases the number of recent items shown in Taskbar Jump Lists (defaults to 10).",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "JumpListItems_Maximum", 20),
        ],
        },
        crate::tweak! {
        id: "classic_explorer_search",
        category: "explorer",
        name: "Classic Explorer Search",
        description: "Restores the older, faster File Explorer search behavior; using Everything for search is better than this.",
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs", "", "{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                crate::reg_str!("HKLM", r"SOFTWARE\Classes\WOW6432Node\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs", "", "{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                crate::reg_str!("HKLM", r"SOFTWARE\WOW6432Node\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs", "", "{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
        ],
        },
        crate::tweak! {
        id: "compact_mode",
        category: "explorer",
        name: "Compact View Mode",
        description: "Uses compact spacing in File Explorer (less padding between items).",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "UseCompactMode", 1),
        ],
        },
        crate::tweak! {
        id: "disable_balloon_tips",
        category: "explorer",
        name: "Disable Balloon Tips",
        description: "Disables legacy balloon tips in the notification area.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "EnableBalloonTips", 0),
        ],
        },
        crate::tweak! {
        id: "disable_folder_discovery",
        category: "explorer",
        name: "Disable Folder Type Discovery",
        description: "Prevents Windows from auto-detecting folder types (can cause slowdowns).",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags\AllFolders\Shell", "FolderType", "NotSpecified"),
        ],
        },
        crate::tweak! {
        id: "disable_item_checkboxes",
        category: "explorer",
        name: "Disable Item Checkboxes",
        description: "Removes checkboxes from file/folder items in File Explorer.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "AutoCheckSelect", 0),
        ],
        },
        crate::tweak! {
        id: "disable_low_disk_warning",
        category: "explorer",
        name: "Disable Low Disk Space Warning",
        description: "Disables the warning notification when disk space is low.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoLowDiskSpaceChecks", 1),
        ],
                },
        crate::tweak! {
        id: "disable_network_crawling",
        category: "explorer",
        name: "Disable Network Crawling",
        description: "Prevents Windows from automatically searching specifically for network printers and shares.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "NoNetCrawling", 1),
        ],
        },
        crate::tweak! {
        id: "disable_storage_sense",
        category: "explorer",
        name: "Disable Storage Sense",
        description: "Disables automatic disk cleanup (Storage Sense).",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy", "01", 0),
        ],
                },
        crate::tweak! {
        id: "disable_sync_provider_notifications",
        category: "explorer",
        name: "Disable Sync Provider Notifications",
        description: "Disables notifications from sync providers like OneDrive in File Explorer.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSyncProviderNotifications", 0),
        ],
        },
        crate::tweak! {
        id: "drive_letters_first",
        category: "explorer",
        name: "Drive Letters Before Labels",
        description: "Shows drive letters before volume labels (e.g., 'C: Local Disk' instead of 'Local Disk (C:)').",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "ShowDriveLettersFirst", 4),
        ],
        },
        crate::tweak! {
            id: "long_paths",
            category: "explorer",
            name: "Enable Long Paths",
            description: "Enables support for file paths longer than 260 characters.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\FileSystem", "LongPathsEnabled", 1),
            ],
        },
        crate::tweak! {
        id: "hide_this_pc_nav",
        category: "explorer",
        name: "Hide 'This PC' from Navigation Pane",
        description: "Hides 'This PC' from the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{20D04FE0-3AEA-1069-A2D8-08002B30309D}", "System.IsPinnedToNameSpaceTree", 0),
        ],
        },
        crate::tweak! {
        id: "hide_3d_objects_this_pc",
        category: "explorer",
        name: "Hide 3D Objects in This PC",
        description: "Hides the 3D Objects folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_del_key!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{0DB7E03F-FC29-4DC6-9020-FF41B59E513A}", ""),
        ],
        },
        crate::tweak! {
        id: "hide_cloud_files_quick_access",
        category: "explorer",
        name: "Hide Cloud Files in Quick Access",
        description: "Hides cloud-based files from showing up in Quick Access recent files.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "ShowCloudFilesInQuickAccess", 0),
        ],
        },
        crate::tweak! {
        id: "hide_desktop_this_pc",
        category: "explorer",
        name: "Hide Desktop in This PC",
        description: "Hides the Desktop folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "hide_documents_this_pc",
        category: "explorer",
        name: "Hide Documents in This PC",
        description: "Hides the Documents folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "hide_downloads_this_pc",
        category: "explorer",
        name: "Hide Downloads in This PC",
        description: "Hides the Downloads folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "hide_drives_in_send_to",
        category: "explorer",
        name: "Hide Drives in 'Send To' Menu",
        description: "Removes removable and network drives from the 'Send To' context menu.",
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoDrivesInSendToMenu", 1),
        ],
        },
        crate::tweak! {
        id: "hide_explorer_recommendations",
        category: "explorer",
        name: "Hide Explorer Recommendations",
        description: "Disables recommendations and tips in File Explorer and Start Menu.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "ShowRecommendations", 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_IrisRecommendations", 0),
        ],
        },
        crate::tweak! {
        id: "hide_gallery",
        category: "explorer",
        name: "Hide Gallery",
        description: "Hides the Gallery folder from File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowGallery", 0),
        ],
        },
        crate::tweak! {
        id: "hide_wsl_icon",
        category: "explorer",
        name: "Hide Linux (WSL) Icon",
        description: "Hides the Linux (WSL) icon from the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}", 1),
        ],
        },
        crate::tweak! {
        id: "hide_music_this_pc",
        category: "explorer",
        name: "Hide Music in This PC",
        description: "Hides the Music folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "hide_pictures_this_pc",
        category: "explorer",
        name: "Hide Pictures in This PC",
        description: "Hides the Pictures folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "hide_recent_files_home",
        category: "explorer",
        name: "Hide Recent Files in Home",
        description: "Hides the Recent files section in File Explorer Home.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "ShowRecent", 0),
        ],
        },
        crate::tweak! {
        id: "hide_recent_frequent",
        category: "explorer",
        name: "Hide Recent/Frequent Items",
        description: "Hides recently used files and frequently used folders from Quick Access.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "ShowFrequent", 0),
                crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "ShowRecent", 0),
        ],
        },
        crate::tweak! {
        id: "hide_videos_this_pc",
        category: "explorer",
        name: "Hide Videos in This PC",
        description: "Hides the Videos folder from the 'This PC' view.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", "HideIfEnabled", 0x022ab9b9),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "folder_view_memory",
        category: "explorer",
        name: "Increase Folder View Memory",
        description: "Increases the number of folder view settings Windows remembers (BagMRU Size).",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\Shell", "BagMRU Size", 10000),
        ],
        },
        crate::tweak! {
            id: "large_icon_cache",
            category: "explorer",
            name: "Increase Icon Cache Size",
            description: "Increases the icon cache size to prevent icon rebuilding.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "Max Cached Icons", "4096"),
            ],
        },
        crate::tweak! {
        id: "open_to_this_pc",
        category: "explorer",
        name: "Open to This PC",
        description: "File Explorer opens to 'This PC' instead of Quick Access.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "LaunchTo", 1),
        ],
        },
        crate::tweak! {
        id: "disable_shortcut_text",
        category: "explorer",
        name: "Remove '- Shortcut' Suffix",
        description: "Removes '- Shortcut' text from newly created shortcuts.",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\NamingTemplates", "ShortcutNameTemplate", "%s.lnk"),
        ],
        },
        crate::tweak! {
        id: "remove_home_nav",
        category: "explorer",
        name: "Remove 'Home' from Navigation Pane",
        description: "Hides the 'Home' icon from the File Explorer navigation pane.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}", "System.IsPinnedToNameSpaceTree", 0),
        ],
        },
        crate::tweak! {
        id: "remove_recycle_bin_properties",
        category: "explorer",
        name: "Remove 'Properties' from Recycle Bin",
        description: "Removes the 'Properties' option from the Recycle Bin context menu.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoPropertiesRecycleBin", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoPropertiesRecycleBin", 1),
        ],
        },
        crate::tweak! {
        id: "remove_this_pc_properties",
        category: "explorer",
        name: "Remove 'Properties' from This PC",
        description: "Removes the 'Properties' option from the 'This PC' context menu.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoPropertiesMyComputer", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoPropertiesMyComputer", 1),
        ],
        },
        crate::tweak! {
        id: "remove_control_panel_nav",
        category: "explorer",
        name: "Remove Control Panel from Navigation Pane",
        description: "Hides the Control Panel from the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{26EE0668-A00A-44D7-9371-BEB064C98683}", "System.IsPinnedToNameSpaceTree", 0),
        ],
        },
        crate::tweak! {
        id: "remove_desktop_nav",
        category: "explorer",
        name: "Remove Desktop from Navigation Pane",
        description: "Hides the Desktop folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "remove_details_tab",
        category: "explorer",
        name: "Remove Details Tab",
        description: "Removes the 'Details' tab from file properties.",
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"*\shellex\PropertySheetHandlers\{883373C3-BF89-11D1-BE35-080036B11A03}", ""),
        ],
        },
        crate::tweak! {
        id: "remove_documents_nav",
        category: "explorer",
        name: "Remove Documents from Navigation Pane",
        description: "Hides the Documents folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "remove_downloads_nav",
        category: "explorer",
        name: "Remove Downloads from Navigation Pane",
        description: "Hides the Downloads folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{374DE290-123F-4565-9164-39C4925E467B}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{374DE290-123F-4565-9164-39C4925E467B}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "remove_duplicate_drives",
        category: "explorer",
        name: "Remove Duplicate Drives",
        description: "Removers duplicate removable drives from the File Explorer navigation pane.",
        enabled_ops: &[
                crate::reg_del_key!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", ""),
                crate::reg_del_key!("HKLM", r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", ""),
        ],
        },
        crate::tweak! {
        id: "remove_libraries_nav",
        category: "explorer",
        name: "Remove Libraries from Navigation Pane",
        description: "Hides the Libraries folder from the File Explorer navigation pane.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{031E4825-7B94-4dc3-B131-E946B44C8DD5}", "System.IsPinnedToNameSpaceTree", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{031E4825-7B94-4dc3-B131-E946B44C8DD5}", 1),
        ],
        },
        crate::tweak! {
        id: "remove_location_tab",
        category: "explorer",
        name: "Remove Location Tab",
        description: "Removes the 'Location' tab from folder properties.",
        enabled_ops: &[
            crate::reg_del_key!("HKCR", r"Directory\shellex\PropertySheetHandlers\{4a7ded0a-ad25-11d0-98a8-0800361b1103}", ""),
        ],
                },
        crate::tweak! {
        id: "remove_music_nav",
        category: "explorer",
        name: "Remove Music from Navigation Pane",
        description: "Hides the Music folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "remove_network_nav_pane",
        category: "explorer",
        name: "Remove Network from Navigation Pane",
        description: "Removes the Network icon from the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}", "System.IsPinnedToNameSpaceTree", 0),
        ],
                },
        crate::tweak! {
        id: "remove_onedrive_nav",
        category: "explorer",
        name: "Remove OneDrive from Navigation Pane",
        description: "Hides the OneDrive icon from the File Explorer navigation pane.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", "System.IsPinnedToNameSpaceTree", 0),
                crate::reg_dword!("HKCU", r"Software\Classes\WOW6432Node\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", "System.IsPinnedToNameSpaceTree", 0),
        ],
        },
        crate::tweak! {
        id: "remove_pictures_nav",
        category: "explorer",
        name: "Remove Pictures from Navigation Pane",
        description: "Hides the Pictures folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "remove_previous_versions_tab",
        category: "explorer",
        name: "Remove Previous Versions Tab",
        description: "Removes the 'Previous Versions' tab from file, folder, and drive properties.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "NoPreviousVersionsPage", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "NoPreviousVersionsPage", 1),
        ],
        },
        crate::tweak! {
                id: "remove_quick_access_home",
                category: "explorer",
                name: "Remove Quick Access from Home",
                description: "Hides the 'Quick access' (frequent places) section in File Explorer Home.",
                enabled_ops: &[
                    crate::reg_del_key!("HKLM", "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HomeFolderMSGraph\\NameSpace\\DelegateFolders\\{3936E9E4-D92C-4EEE-A85A-BC16D5EA0819}", ""),
                ],
        },
        crate::tweak! {
        id: "remove_recycle_bin_nav",
        category: "explorer",
        name: "Remove Recycle Bin from Navigation Pane",
        description: "Hides the Recycle Bin from the File Explorer navigation pane.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}", "System.IsPinnedToNameSpaceTree", 0),
        ],
        },
        crate::tweak! {
        id: "remove_security_tab",
        category: "explorer",
        name: "Remove Security Tab",
        description: "Removes the 'Security' tab from file, folder, and drive properties.",
        enabled_ops: &[
                crate::reg_del_key!("HKCR", r"*\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", ""),
                crate::reg_del_key!("HKCR", r"Directory\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", ""),
                crate::reg_del_key!("HKCR", r"Drive\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", ""),
        ],
        },
        crate::tweak! {
        id: "remove_sharing_tab",
        category: "explorer",
        name: "Remove Sharing Tab",
        description: "Removes the 'Sharing' tab from drive and folder properties.",
        enabled_ops: &[
                crate::reg_del_key!("HKCR", r"Directory\shellex\PropertySheetHandlers\Sharing", ""),
                crate::reg_del_key!("HKCR", r"Drive\shellex\PropertySheetHandlers\Sharing", ""),
        ],
        },
        crate::tweak! {
        id: "remove_videos_nav",
        category: "explorer",
        name: "Remove Videos from Navigation Pane",
        description: "Hides the Videos folder from the File Explorer navigation pane (Show all folders mode).",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", "{A0953C92-50DC-43bf-BE83-3742FED03C9C}", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A0953C92-50DC-43bf-BE83-3742FED03C9C}", "HiddenByDefault", 1),
        ],
        },
        crate::tweak! {
        id: "show_file_extensions",
        category: "explorer",
        name: "Show File Extensions",
        description: "Shows file extensions for known file types.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "HideFileExt", 0),
        ],
        },
        crate::tweak! {
        id: "show_hidden_files",
        category: "explorer",
        name: "Show Hidden Files",
        description: "Shows hidden files and folders in File Explorer.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Hidden", 1),
        ],
        },
        crate::tweak! {
        id: "show_printers_nav",
        category: "explorer",
        name: "Show Printers in Navigation Pane",
        description: "Adds the Printers folder to the File Explorer navigation pane.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Classes\CLSID\{2227A280-3AEA-1069-A2DE-08002B30309D}", "System.IsPinnedToNameSpaceTree", 1),
        ],
        },
];
