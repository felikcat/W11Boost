// File Explorer tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static EXPLORER_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "remove_quick_access_home",
                category: "explorer",
                name: "Remove Quick Access from Home",
                description: "Hides the 'Quick access' (frequent places) section in File Explorer Home.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                    RegistryOp {
                        hkey: "HKLM",
                        subkey: "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HomeFolderMSGraph\\NameSpace\\DelegateFolders\\{3936E9E4-D92C-4EEE-A85A-BC16D5EA0819}",
                        value_name: "",
                        value: RegistryValue::DeleteKey,
                        stock_value: RegistryValue::String("Quick Access"),
                    }
                ],
        },
        crate::tweak! {
                id: "open_to_this_pc",
                category: "explorer",
                name: "Open to This PC",
                description: "File Explorer opens to 'This PC' instead of Quick Access.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "LaunchTo",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "LaunchTo",
                        value: RegistryValue::Dword(2),
                        stock_value: RegistryValue::Dword(2)
        }])
        },
        crate::tweak! {
                id: "show_hidden_files",
                category: "explorer",
                name: "Show Hidden Files",
                description: "Shows hidden files and folders in File Explorer.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "Hidden",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(2)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "Hidden",
                        value: RegistryValue::Dword(2),
                        stock_value: RegistryValue::Dword(2)
        }])
        },
        crate::tweak! {
                id: "show_file_extensions",
                category: "explorer",
                name: "Show File Extensions",
                description: "Shows file extensions for known file types.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "HideFileExt",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "HideFileExt",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "compact_mode",
                category: "explorer",
                name: "Compact View Mode",
                description: "Uses compact spacing in File Explorer (less padding between items).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "UseCompactMode",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "UseCompactMode",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(0)
        }])
        },
        crate::tweak! {
                id: "hide_gallery",
                category: "explorer",
                name: "Hide Gallery",
                description: "Hides the Gallery folder from File Explorer navigation pane.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowGallery",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowGallery",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "hide_recent_frequent",
                category: "explorer",
                name: "Hide Recent/Frequent Items",
                description: "Hides recently used files and frequently used folders from Quick Access.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowFrequent",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowRecent",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowFrequent",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowRecent",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ])
        },
        crate::tweak! {
                id: "disable_folder_discovery",
                category: "explorer",
                name: "Disable Folder Type Discovery",
                description: "Prevents Windows from auto-detecting folder types (can cause slowdowns).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags\AllFolders\Shell",
                        value_name: "FolderType",
                        value: RegistryValue::String("NotSpecified"),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\Shell\Bags\AllFolders\Shell",
                        value_name: "FolderType",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "drive_letters_first",
                category: "explorer",
                name: "Drive Letters Before Labels",
                description: "Shows drive letters before volume labels (e.g., 'C: Local Disk' instead of 'Local Disk (C:)').",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                        value_name: "ShowDriveLettersFirst",
                        value: RegistryValue::Dword(4),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                        value_name: "ShowDriveLettersFirst",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "classic_search",
                category: "explorer",
                name: "Classic Explorer Search",
                description: "Restores the classic search behavior in File Explorer.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\WOW6432Node\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\WOW6432Node\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\WOW6432Node\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\WOW6432Node\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "folder_view_memory",
                category: "explorer",
                name: "Increase Folder View Memory",
                description: "Increases the number of folder view settings Windows remembers (BagMRU Size).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\Shell",
                        value_name: "BagMRU Size",
                        value: RegistryValue::Dword(10000),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\Shell",
                        value_name: "BagMRU Size",
                        value: RegistryValue::Dword(5000),
                        stock_value: RegistryValue::Dword(5000)
        }])
        },
        crate::tweak! {
                id: "hide_wsl_icon",
                category: "explorer",
                name: "Hide Linux (WSL) Icon",
                description: "Hides the Linux (WSL) icon from the File Explorer navigation pane.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                        value_name: "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                        value_name: "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "hide_drives_in_send_to",
                category: "explorer",
                name: "Hide Drives in 'Send To' Menu",
                description: "Removes removable and network drives from the 'Send To' context menu.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                        value_name: "NoDrivesInSendToMenu",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                        value_name: "NoDrivesInSendToMenu",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "remove_libraries_nav",
                category: "explorer",
                name: "Remove Libraries from Navigation Pane",
                description: "Hides the Libraries folder from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{031E4825-7B94-4dc3-B131-E946B44C8DD5}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum",
                                value_name: "{031E4825-7B94-4dc3-B131-E946B44C8DD5}",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{031E4825-7B94-4dc3-B131-E946B44C8DD5}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum",
                                value_name: "{031E4825-7B94-4dc3-B131-E946B44C8DD5}",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "remove_recycle_bin_nav",
                category: "explorer",
                name: "Remove Recycle Bin from Navigation Pane",
                description: "Hides the Recycle Bin from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{645FF040-5081-101B-9F08-00AA002F954E}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ])
        },
        crate::tweak! {
                id: "hide_this_pc_nav",
                category: "explorer",
                name: "Hide 'This PC' from Navigation Pane",
                description: "Hides 'This PC' from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                         RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                ],
                disabled_ops: Some(&[
                         RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\CLSID\{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
                                value_name: "System.IsPinnedToNameSpaceTree",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ])
        },
        crate::tweak! {
                id: "remove_duplicate_drives",
                category: "explorer",
                name: "Remove Duplicate Drives",
                description: "Removers duplicate removable drives from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("Removable Drives") },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("Removable Drives") },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", value_name: "", value: RegistryValue::String("Removable Drives"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\DelegateFolders\{F5FB2C77-0E2F-4A16-A381-3E560C68BC83}", value_name: "", value: RegistryValue::String("Removable Drives"), stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "hide_3d_objects_this_pc",
                category: "explorer",
                name: "Hide 3D Objects in This PC",
                description: "Hides the 3D Objects folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{0DB7E03F-FC29-4DC6-9020-FF41B59E513A}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{0DB7E03F-FC29-4DC6-9020-FF41B59E513A}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "hide_desktop_this_pc",
                category: "explorer",
                name: "Hide Desktop in This PC",
                description: "Hides the Desktop folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "hide_documents_this_pc",
                category: "explorer",
                name: "Hide Documents in This PC",
                description: "Hides the Documents folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{d3162b92-9365-467a-956b-92703aca08af}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "hide_downloads_this_pc",
                category: "explorer",
                name: "Hide Downloads in This PC",
                description: "Hides the Downloads folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{088e3905-0323-4b02-9826-5d99428e115f}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "hide_music_this_pc",
                category: "explorer",
                name: "Hide Music in This PC",
                description: "Hides the Music folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{3dfdf296-dbec-4fb4-81d1-6a3438bcf4de}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "hide_pictures_this_pc",
                category: "explorer",
                name: "Hide Pictures in This PC",
                description: "Hides the Pictures folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{24ad3ad4-a569-4530-98e1-ab02f9417aa8}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "hide_videos_this_pc",
                category: "explorer",
                name: "Hide Videos in This PC",
                description: "Hides the Videos folder from the 'This PC' view.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", value_name: "HideIfEnabled", value: RegistryValue::Dword(0x022ab9b9), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", value_name: "HideIfEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0x022ab9b9) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{f86fa3ab-70d2-4fc7-9c99-fcbf05467f3a}", value_name: "HiddenByDefault", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "add_devices_and_printers_this_pc",
                category: "explorer",
                name: "Add Devices and Printers to This PC",
                description: "Adds 'Devices and Printers' to the 'This PC' view in File Explorer.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::String("Devices and Printers"), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MyComputer\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "remove_home_nav",
                category: "explorer",
                name: "Remove 'Home' from Navigation Pane",
                description: "Hides the 'Home' icon from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{f874310e-b6b7-47dc-bc84-b9e6b38f5903}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "remove_desktop_nav",
                category: "explorer",
                name: "Remove Desktop from Navigation Pane",
                description: "Hides the Desktop folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_documents_nav",
                category: "explorer",
                name: "Remove Documents from Navigation Pane",
                description: "Hides the Documents folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8CDFF1C-4878-43be-B5FD-F8091C1C60D0}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_downloads_nav",
                category: "explorer",
                name: "Remove Downloads from Navigation Pane",
                description: "Hides the Downloads folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{374DE290-123F-4565-9164-39C4925E467B}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{374DE290-123F-4565-9164-39C4925E467B}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{374DE290-123F-4565-9164-39C4925E467B}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{374DE290-123F-4565-9164-39C4925E467B}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_music_nav",
                category: "explorer",
                name: "Remove Music from Navigation Pane",
                description: "Hides the Music folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{1CF1260C-4DD0-4ebb-811F-33C572699FDE}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_pictures_nav",
                category: "explorer",
                name: "Remove Pictures from Navigation Pane",
                description: "Hides the Pictures folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{3ADD1653-EB32-4cb0-BBD7-DFA0ABB5ACCA}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_videos_nav",
                category: "explorer",
                name: "Remove Videos from Navigation Pane",
                description: "Hides the Videos folder from the File Explorer navigation pane (Show all folders mode).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{A0953C92-50DC-43bf-BE83-3742FED03C9C}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A0953C92-50DC-43bf-BE83-3742FED03C9C}", value_name: "HiddenByDefault", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\NonEnum", value_name: "{A0953C92-50DC-43bf-BE83-3742FED03C9C}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A0953C92-50DC-43bf-BE83-3742FED03C9C}", value_name: "HiddenByDefault", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_onedrive_nav",
                category: "explorer",
                name: "Remove OneDrive from Navigation Pane",
                description: "Hides the OneDrive icon from the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\WOW6432Node\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\WOW6432Node\CLSID\{018D5C66-4533-4307-9B53-224DE2ED1FE6}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "max_jump_list_items",
                category: "explorer",
                name: "Change Maximum Number of Items in Jump Lists",
                description: "Increases the number of recent items shown in Taskbar Jump Lists (defaults to 10).",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "JumpListItems_Maximum",
                        value: RegistryValue::Dword(20),
                        stock_value: RegistryValue::Dword(10)
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "JumpListItems_Maximum",
                        value: RegistryValue::Dword(10),
                        stock_value: RegistryValue::Dword(10)
                }])
        },
        crate::tweak! {
                id: "show_printers_nav",
                category: "explorer",
                name: "Show Printers in Navigation Pane",
                description: "Adds the Printers folder to the File Explorer navigation pane.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{2227A280-3AEA-1069-A2DE-08002B30309D}", value_name: "System.IsPinnedToNameSpaceTree", value: RegistryValue::Dword(1), stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{2227A280-3AEA-1069-A2DE-08002B30309D}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "hide_recent_files_home",
                category: "explorer",
                name: "Hide Recent Files in Home",
                description: "Hides the Recent files section in File Explorer Home.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer", value_name: "ShowRecent", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer", value_name: "ShowRecent", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "remove_sharing_tab",
                category: "explorer",
                name: "Remove Sharing Tab",
                description: "Removes the 'Sharing' tab from drive and folder properties.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\Sharing", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}") },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\Sharing", value_name: "", value: RegistryValue::String("{f81e9010-6ea4-11ce-a7ff-00aa003ca9f6}"), stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "remove_compatibility_tab",
                category: "explorer",
                name: "Remove Compatibility Tab",
                description: "Removes the 'Compatibility' tab from app properties.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", value_name: "DisablePropPage", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", value_name: "DisablePropPage", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_security_tab",
                category: "explorer",
                name: "Remove Security Tab",
                description: "Removes the 'Security' tab from file, folder, and drive properties.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCR", subkey: r"*\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCR", subkey: r"*\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Directory\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                        RegistryOp { hkey: "HKCR", subkey: r"Drive\shellex\PropertySheetHandlers\{1f2e5c40-9550-11ce-99d2-00aa006e086c}", value_name: "", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                ])
        },
        crate::tweak! {
                id: "remove_this_pc_properties",
                category: "explorer",
                name: "Remove 'Properties' from This PC",
                description: "Removes the 'Properties' option from the 'This PC' context menu.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesMyComputer", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesMyComputer", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesMyComputer", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesMyComputer", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_recycle_bin_properties",
                category: "explorer",
                name: "Remove 'Properties' from Recycle Bin",
                description: "Removes the 'Properties' option from the Recycle Bin context menu.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesRecycleBin", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesRecycleBin", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesRecycleBin", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", value_name: "NoPropertiesRecycleBin", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "remove_previous_versions_tab",
                category: "explorer",
                name: "Remove Previous Versions Tab",
                description: "Removes the 'Previous Versions' tab from file, folder, and drive properties.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer", value_name: "NoPreviousVersionsPage", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", value_name: "NoPreviousVersionsPage", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer", value_name: "NoPreviousVersionsPage", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", value_name: "NoPreviousVersionsPage", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "hide_cloud_files_quick_access",
                category: "explorer",
                name: "Hide Cloud Files in Quick Access",
                description: "Hides cloud-based files from showing up in Quick Access recent files.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowCloudFilesInQuickAccess",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowCloudFilesInQuickAccess",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
                id: "hide_explorer_recommendations",
                category: "explorer",
                name: "Hide Explorer Recommendations",
                description: "Disables recommendations and tips in File Explorer and Start Menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowRecommendations",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_IrisRecommendations",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                                value_name: "ShowRecommendations",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_IrisRecommendations",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
                id: "disable_network_crawling",
                category: "explorer",
                name: "Disable Network Crawling",
                description: "Prevents Windows from automatically searching specifically for network printers and shares.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "NoNetCrawling",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "NoNetCrawling",
                                value: RegistryValue::Delete, // Default is usually off (0) or not present (0)
                                stock_value: RegistryValue::Delete
                        }
                ])
        },
        crate::tweak! {
                id: "disable_balloon_tips",
                category: "explorer",
                name: "Disable Balloon Tips",
                description: "Disables legacy balloon tips in the notification area.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "EnableBalloonTips",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1) // Assuming enabled by default if not present? Actually 0 is disable.
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "EnableBalloonTips",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
                id: "disable_sync_provider_notifications",
                category: "explorer",
                name: "Disable Sync Provider Notifications",
                description: "Disables notifications from sync providers like OneDrive in File Explorer.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowSyncProviderNotifications",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowSyncProviderNotifications",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
                id: "hide_copilot_button",
                category: "explorer",
                name: "Hide Copilot Button",
                description: "Hides the Copilot button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowCopilotButton",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowCopilotButton",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
                id: "hide_people_band",
                category: "explorer",
                name: "Hide People Band",
                description: "Hides the 'People' icon/band from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\people",
                                value_name: "PeopleBand",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\people",
                                value_name: "PeopleBand",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        }
                ])
        },
        crate::tweak! {
            id: "remove_control_panel_nav",
            category: "explorer",
            name: "Remove Control Panel from Navigation Pane",
            description: "Hides the Control Panel from the File Explorer navigation pane.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Classes\CLSID\{26EE0668-A00A-44D7-9371-BEB064C98683}",
                    value_name: "System.IsPinnedToNameSpaceTree",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1)
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Classes\CLSID\{26EE0668-A00A-44D7-9371-BEB064C98683}",
                    value_name: "System.IsPinnedToNameSpaceTree",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1)
                },
            ])
        },
        crate::tweak! {
            id: "remove_details_tab",
            category: "explorer",
            name: "Remove Details Tab",
            description: "Removes the 'Details' tab from file properties.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"*\shellex\PropertySheetHandlers\{883373C3-BF89-11D1-BE35-080036B11A03}",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::String("Summary Properties Page")
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"*\shellex\PropertySheetHandlers\{883373C3-BF89-11D1-BE35-080036B11A03}",
                    value_name: "",
                    value: RegistryValue::String("Summary Properties Page"),
                    stock_value: RegistryValue::DeleteKey
                },
            ])
        },
        crate::tweak! {
            id: "add_devices_printers_nav",
            category: "explorer",
            name: "Add Devices and Printers to Navigation Pane",
            description: "Adds 'Devices and Printers' to the File Explorer navigation pane.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::String("Device Center"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "System.IsPinnedToNamespaceTree", value: RegistryValue::Dword(1), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "SortOrderIndex", value: RegistryValue::Dword(0x3c), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "System.ApplicationName", value: RegistryValue::String("Microsoft.DevicesAndPrinters"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\DefaultIcon", value_name: "", value: RegistryValue::String(r"%systemroot%\system32\DeviceCenter.dll,-1"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", value_name: "", value: RegistryValue::String(r"%systemroot%\system32\DeviceCenter.dll"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", value_name: "LoadWithoutCOM", value: RegistryValue::String(""), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}\InProcServer32", value_name: "ThreadingModel", value: RegistryValue::String("Both"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::String("Device Center"), stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", value_name: "{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Classes\CLSID\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value_name: "", value: RegistryValue::DeleteKey, stock_value: RegistryValue::DeleteKey },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", value_name: "{A8A91A66-3A7D-4424-8D24-04E180695C7A}", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ])
        },
        crate::tweak! {
            id: "remove_drive_space_indicator",
            category: "explorer",
            name: "Remove Drive Space Indicator",
            description: "Removes the drive space indicator bar from drives in This PC.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive",
                    value_name: "TileInfo",
                    value: RegistryValue::String("prop:*System.Computer.DecoratedFreeSpace;System.Volume.FileSystem"),
                    stock_value: RegistryValue::String("prop:*System.PercentFull;System.Computer.DecoratedFreeSpace;System.Volume.FileSystem"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Drive",
                    value_name: "TileInfo",
                    value: RegistryValue::String("prop:*System.PercentFull;System.Computer.DecoratedFreeSpace;System.Volume.FileSystem"),
                    stock_value: RegistryValue::String("prop:*System.PercentFull;System.Computer.DecoratedFreeSpace;System.Volume.FileSystem"),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_location_tab",
            category: "explorer",
            name: "Remove Location Tab",
            description: "Removes the 'Location' tab from folder properties.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Directory\shellex\PropertySheetHandlers\{4a7ded0a-ad25-11d0-98a8-0800361b1103}",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::String("Location Tab"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCR",
                    subkey: r"Directory\shellex\PropertySheetHandlers\{4a7ded0a-ad25-11d0-98a8-0800361b1103}",
                    value_name: "",
                    value: RegistryValue::String(""),
                    stock_value: RegistryValue::String(""),
                },
            ]),
        },
        crate::tweak! {
            id: "add_explorer_menu_bar",
            category: "explorer",
            name: "Always Show Menu Bar",
            description: "Forces the classic menu bar (File, Edit, View, Tools) to be always visible in File Explorer.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "AlwaysShowMenus",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "AlwaysShowMenus",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(0),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_network_nav_pane",
            category: "explorer",
            name: "Remove Network from Navigation Pane",
            description: "Removes the Network icon from the File Explorer navigation pane.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Classes\CLSID\{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
                    value_name: "System.IsPinnedToNameSpaceTree",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Classes\CLSID\{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
                    value_name: "System.IsPinnedToNameSpaceTree",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_shortcut_extension",
            category: "explorer",
            name: "Remove '- Shortcut' Extension",
            description: "Prevents Windows from adding '- Shortcut' text to newly created shortcuts.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                    value_name: "link",
                    value: RegistryValue::Binary(&[0, 0, 0, 0]),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\NamingTemplates",
                    value_name: "ShortcutNameTemplate",
                    value: RegistryValue::String("ShortcutNameTemplate"),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                    value_name: "link",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\NamingTemplates",
                    value_name: "ShortcutNameTemplate",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
                id: "disable_start_menu_ads",
                category: "explorer",
                name: "Disable Start Menu Ads",
                description: "Removes 'Recommended' ads and tips (Iris Recommendations) from the Start Menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", value_name: "Start_IrisRecommendations", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", value_name: "Start_IrisRecommendations", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "disable_autoplay",
                category: "explorer",
                name: "Disable Autoplay",
                description: "Disables Autoplay for all drives.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
                        value_name: "DisableAutoplay",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers",
                        value_name: "DisableAutoplay",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }]),
        },
        crate::tweak! {
                id: "classic_explorer_search",
                category: "explorer",
                name: "Classic Explorer Search",
                description: "Restores the older, faster File Explorer search behavior.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\WOW6432Node\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\WOW6432Node\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}\TreatAs",
                                value_name: "",
                                value: RegistryValue::String("{64bc32b5-4eec-4de7-972d-bd8bd0324537}"),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Classes\WOW6432Node\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\WOW6432Node\Classes\CLSID\{1d64637d-31e9-4b06-9124-e83fb178ac6e}",
                                value_name: "",
                                value: RegistryValue::DeleteKey,
                                stock_value: RegistryValue::DeleteKey
        },
                ])
        },
        crate::tweak! {
                id: "disable_item_checkboxes",
                category: "explorer",
                name: "Disable Item Checkboxes",
                description: "Removes checkboxes from file/folder items in File Explorer.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "AutoCheckSelect",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(0), // Default is 0 on most systems, but can be 1
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "AutoCheckSelect",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(0)
        }])
        },
        crate::tweak! {
                id: "disable_thumbnail_caching",
                category: "explorer",
                name: "Disable Thumbnail Caching",
                description: "Prevents Windows from caching folder thumbnails (reduces disk writes and artifacts).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\DWM",
                                value_name: "AlwaysHibernateThumbnails",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DisableThumbnailCache",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DisableThumbsDBOnNetworkFolders",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\DWM",
                                value_name: "AlwaysHibernateThumbnails",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(0)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DisableThumbnailCache",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DisableThumbsDBOnNetworkFolders",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_sync_notifications",
                category: "explorer",
                name: "Disable Sync Provider Notifications",
                description: "Disables notifications from sync providers like OneDrive in File Explorer.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowSyncProviderNotifications",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowSyncProviderNotifications",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
            id: "large_icon_cache",
            category: "explorer",
            name: "Increase Icon Cache Size",
            description: "Increases the icon cache size to prevent icon rebuilding.",
            effect: TweakEffect::Restart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                value_name: "Max Cached Icons",
                value: RegistryValue::String("4096"),
                stock_value: RegistryValue::Delete // Often not present
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer",
                value_name: "Max Cached Icons",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_low_disk_warning",
            category: "explorer",
            name: "Disable Low Disk Space Warning",
            description: "Disables the warning notification when disk space is low.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                value_name: "NoLowDiskSpaceChecks",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                value_name: "NoLowDiskSpaceChecks",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }]),
        },
        crate::tweak! {
            id: "disable_shortcut_text",
            category: "explorer",
            name: "Remove '- Shortcut' Suffix",
            description: "Removes '- Shortcut' text from newly created shortcuts.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\NamingTemplates",
                value_name: "ShortcutNameTemplate",
                value: RegistryValue::String("%s.lnk"),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\NamingTemplates",
                value_name: "ShortcutNameTemplate",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }])
        },
        crate::tweak! {
            id: "long_paths",
            category: "explorer",
            name: "Enable Long Paths",
            description: "Enables support for file paths longer than 260 characters.",
            effect: TweakEffect::Restart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\FileSystem",
                value_name: "LongPathsEnabled",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\FileSystem",
                value_name: "LongPathsEnabled",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_storage_sense",
            category: "explorer",
            name: "Disable Storage Sense",
            description: "Disables automatic disk cleanup (Storage Sense).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
                value_name: "01",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0) // Default off usually? Or on?
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\StorageSense\Parameters\StoragePolicy",
                value_name: "01",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }]),
        },
];
