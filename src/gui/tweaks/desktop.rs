// Desktop & Taskbar tweaks

use super::{RegistryValue, Tweak, TweakEffect};

pub static DESKTOP_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_aero_shake",
                category: "desktop",
                name: "Disable Aero Shake",
                description: "Prevents minimizing all windows when shaking a window titlebar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisallowShaking", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisallowShaking", 0, RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "show_seconds_clock",
                category: "desktop",
                name: "Show Seconds in Clock",
                description: "Displays seconds in the taskbar clock.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSecondsInSystemClock", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSecondsInSystemClock", 0, RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "taskbar_end_task",
                category: "desktop",
                name: "Enable End Task in Taskbar",
                description: "Adds an 'End task' option when right-clicking taskbar items. Useful for unresponsive apps.",
                effect: TweakEffect::Immediate, // Actually might be immediate or require explorer restart, tutorial says nothing about restart but usually these need one. Let's assume ExplorerRestart to be safe.
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings", "TaskbarEndTask", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings", "TaskbarEndTask", 0, RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "taskbar_never_combine",
                category: "desktop",
                name: "Never Combine Taskbar Buttons",
                description: "Shows labels and never combines buttons on the taskbar (including multi-monitor taskbars).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarGlomLevel", 2, 0),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "MMTaskbarGlomLevel", 2, 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarGlomLevel", 0, 0),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "MMTaskbarGlomLevel", 0, 0),
                ]), // Explorer restart is handled by effect
        },
        crate::tweak! {
                id: "hide_task_view",
                category: "desktop",
                name: "Hide Task View Button",
                description: "Hides the Task View button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowTaskViewButton", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowTaskViewButton", 1, 1),
                ])
        },
        crate::tweak! {
                id: "hide_task_view",
                category: "desktop",
                name: "Hide Task View Button",
                description: "Hides the Task View button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowTaskViewButton", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowTaskViewButton", 1, 1),
                ])
        },
        crate::tweak! {
                id: "hide_task_view_policy",
                category: "desktop",
                name: "Hide Task View Button (Policy)",
                description: "Hides the Task View button from the taskbar using Group Policy for stricter enforcement.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideTaskViewButton", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideTaskViewButton", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "hide_search_box",
                category: "desktop",
                name: "Hide Search Box",
                description: "Removes the search box/icon from the taskbar and disables search suggestions.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search", "SearchboxTaskbarMode", 0, 2),
                        crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search", "SearchboxTaskbarMode", 2, 2),
                        crate::reg_del!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "hide_chat_icon",
                category: "desktop",
                name: "Hide Teams Chat Icon",
                description: "Hides the Microsoft Teams chat icon from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarMn", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarMn", 1, 1),
                ])
        },
        crate::tweak! {
                id: "disable_chat_icon_policy",
                category: "desktop",
                name: "Disable Chat Icon (Policy)",
                description: "Disables the Chat icon on the Taskbar using Group Policy for stricter enforcement.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Chat", "ChatIcon", 3, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Chat", "ChatIcon", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "last_active_click",
                category: "desktop",
                name: "Last Active Click",
                description: "Clicking grouped taskbar button opens last active window instead of thumbnail preview.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "LastActiveClick", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "LastActiveClick", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_notification_center",
                category: "desktop",
                name: "Disable Notification Center",
                description: "Removes the Action Center / Notification Center from the taskbar.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableNotificationCenter", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableNotificationCenter", RegistryValue::Delete),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "remove_shortcut_arrow",
                category: "desktop",
                name: "Remove Shortcut Arrow",
                description: "Removes the arrow icon from shortcuts on the desktop and explorer.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons", "29", r"%windir%\System32\shell32.dll,51", RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons", "29", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "taskbar_button_width",
                category: "desktop",
                name: "Minimize Taskbar Button Width",
                description: "Sets the minimum width for taskbar buttons.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_str!("HKCU", r"Control Panel\Desktop\WindowMetrics", "MinWidth", "38", RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Control Panel\Desktop\WindowMetrics", "MinWidth", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "taskbar_flash_count",
                category: "desktop",
                name: "Disable Taskbar Flashing",
                description: "Stops taskbar buttons from flashing for attention.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ForegroundFlashCount", 0, 7),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ForegroundFlashCount", 7, 7),
                ])
        },
        crate::tweak! {
                id: "hide_meet_now",
                category: "desktop",
                name: "Hide Meet Now Icon",
                description: "Hides the 'Meet Now' icon from the taskbar notification area.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "HideSCAMeetNow", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "HideSCAMeetNow", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "HideSCAMeetNow", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "HideSCAMeetNow", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_news_interests",
                category: "desktop",
                name: "Disable News and Interests (Widgets)",
                description: "Disables the 'News and Interests' (Widgets) feature on the taskbar and lock screen.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\NewsAndInterests\AllowNewsAndInterests", "value", 0, 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\NewsAndInterests\AllowNewsAndInterests", "value", 1, 1),
                ])
        },
        crate::tweak! {
                id: "hide_people_button",
                category: "desktop",
                name: "Hide People Button",
                description: "Hides the People button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People", "PeopleBand", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People", "PeopleBand", 1, 1),
                ])
        },
        crate::tweak! {
                id: "disable_search_highlights",
                category: "desktop",
                name: "Disable Search Highlights",
                description: "Disables dynamic content and highlights in the search box.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDynamicSearchBoxEnabled", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDynamicSearchBoxEnabled", 1, 1),
                ])
        },
        crate::tweak! {
                id: "disable_start_recommendations",
                category: "desktop",
                name: "Disable Start Recommendations",
                description: "Disables tracking of recently opened programs and documents for the Start menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackDocs", 0, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 1, 1),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackDocs", 1, 1),
                ])
        },
        crate::tweak! {
                id: "add_desktop_search_box",
                category: "desktop",
                name: "Show Desktop Search Box",
                description: "Shows a search box on the desktop for quick access to Windows Search. (Windows 11 Build 25120+)",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DesktopSearchBox", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DesktopSearchBox", 0, RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "remove_bluetooth_icon",
                category: "desktop",
                name: "Remove Bluetooth Icon",
                description: "Removes the Bluetooth icon from the taskbar notification area.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Control Panel\Bluetooth", "Notification Area Icon", 0, 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Control Panel\Bluetooth", "Notification Area Icon", 1, 1),
                ])
        },
        crate::tweak! {
                id: "remove_ask_copilot_taskbar",
                category: "desktop",
                name: "Remove 'Ask Copilot' from Taskbar",
                description: "Removes the 'Ask Copilot' button/icon from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarCompanion", 0, 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarCompanion", 1, 1),
                ])
        },
        crate::tweak! {
                id: "disable_web_search",
                category: "desktop",
                name: "Disable Web Search Results",
                description: "Prevents Windows Search from showing web results and Copilot suggestions.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Search", "BingSearchEnabled", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1, RegistryValue::Delete),
                ],
                disabled_ops: Some(&[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Search", "BingSearchEnabled", 1, RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
            id: "remove_desktop_search_box",
            category: "desktop",
            name: "Remove Desktop Search Box",
            description: "Removes the search box from the desktop (if present).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DesktopSearchBox", 0, 1),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DesktopSearchBox", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "remove_spotlight_icon",
            category: "desktop",
            name: "Remove 'Learn about this picture' Icon",
            description: "Removes the 'Learn about this picture' icon from the desktop when using Windows Spotlight.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}", 1, RegistryValue::Delete),
                crate::reg_del_key!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{2cc5ca98-6485-489a-920e-b3e88a6ccce3}", "", RegistryValue::String("Windows Spotlight")),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}", RegistryValue::Delete),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{2cc5ca98-6485-489a-920e-b3e88a6ccce3}", "", "Windows Spotlight", "Windows Spotlight"),
            ]),
        },
        crate::tweak! {
            id: "remove_this_pc_desktop_icon",
            category: "desktop",
            name: "Remove 'This PC' Desktop Icon",
            description: "Hides the 'This PC' icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{20D04FE0-3AEA-1069-A2D8-08002B30309D}", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu", "{20D04FE0-3AEA-1069-A2D8-08002B30309D}", 1, 0),
            ],
        },
        crate::tweak! {
            id: "remove_user_files_desktop_icon",
            category: "desktop",
            name: "Remove 'User Files' Desktop Icon",
            description: "Hides the User Files folder icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{59031a47-3f72-44a7-89c5-5595fe6b30ee}", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu", "{59031a47-3f72-44a7-89c5-5595fe6b30ee}", 1, 0),
            ],
        },
        crate::tweak! {
            id: "remove_network_desktop_icon",
            category: "desktop",
            name: "Remove 'Network' Desktop Icon",
            description: "Hides the Network icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu", "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}", 1, 0),
            ],
        },
        crate::tweak! {
            id: "remove_recycle_bin_desktop_icon",
            category: "desktop",
            name: "Remove 'Recycle Bin' Desktop Icon",
            description: "Hides the Recycle Bin icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{645FF040-5081-101B-9F08-00AA002F954E}", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu", "{645FF040-5081-101B-9F08-00AA002F954E}", 1, 0),
            ],
        },
        crate::tweak! {
            id: "remove_control_panel_desktop_icon",
            category: "desktop",
            name: "Remove 'Control Panel' Desktop Icon",
            description: "Hides the Control Panel icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu", "{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}", 1, 0),
            ],
        },
        crate::tweak! {
            id: "remove_onedrive_desktop_icon",
            category: "desktop",
            name: "Remove 'OneDrive' Desktop Icon",
            description: "Hides the OneDrive icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{018D5C66-4533-4307-9B53-224DE2ED1FE6}", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{018D5C66-4533-4307-9B53-224DE2ED1FE6}", 0, 0), // Default is usually shown if installed
            ]),
        },
        crate::tweak! {
            id: "drag_full_windows",
            category: "desktop",
            name: "Disable Drag Full Windows",
            description: "Shows only window outline while dragging to improve performance.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragFullWindows", "0", "1"),
            ],
            disabled_ops: Some(&[
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragFullWindows", "1", "1"),
            ]),
        },
        crate::tweak! {
            id: "foreground_lock_timeout",
            category: "desktop",
            name: "Reduce Foreground Lock Timeout",
            description: "Reduces the time before a window can steal focus (set to 0).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ForegroundLockTimeout", 0, 200000),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ForegroundLockTimeout", 200000, 200000),
            ]),
        },
        crate::tweak! {
            id: "jpeg_import_quality",
            category: "desktop",
            name: "Increase Wallpaper JPEG Quality",
            description: "Set wallpaper import quality to 100% (disable compression artifacts).",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "JPEGImportQuality", 100, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"Control Panel\Desktop", "JPEGImportQuality", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "hide_recommended_section",
            category: "desktop",
            name: "Hide Recommended Section in Start Menu",
            description: "Hides the 'Recommended' section (files/apps) in the Windows 11 Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"Software\Microsoft\PolicyManager\current\device\start", "HideRecommendedSection", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"Software\Microsoft\PolicyManager\current\device\start", "HideRecommendedSection", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "show_libraries_desktop",
            category: "desktop",
            name: "Show Libraries on Desktop",
            description: "Shows the Libraries icon on the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{031E4825-7B94-4DC3-B131-E946B44C8DD5}", 0, 1),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel", "{031E4825-7B94-4DC3-B131-E946B44C8DD5}", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "disable_copilot_taskbar",
            category: "desktop",
            name: "Disable Copilot Taskbar Button",
            description: "Hides the Copilot button from the taskbar.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowCopilotButton", 0, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowCopilotButton", 1, RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "remove_notification_bell",
            category: "desktop",
            name: "Remove Notification Bell Icon",
            description: "Removes the notification bell icon from the taskbar system tray.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowNotificationIcon", 0, 1),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowNotificationIcon", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "hide_security_notification_icon",
            category: "desktop",
            name: "Hide Windows Security Icon",
            description: "Hides the Windows Security notification icon from the taskbar system tray.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray", "HideSystray", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray", "HideSystray", RegistryValue::Delete),
            ]),
            requires_restart: true
        },
        crate::tweak! {
            id: "taskbar_left_align",
            category: "desktop",
            name: "Left-Align Taskbar",
            description: "Aligns taskbar icons to the left instead of center (Windows 11 default).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarAl", 0, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarAl", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "small_taskbar",
            category: "desktop",
            name: "Small Taskbar",
            description: "Uses smaller taskbar size in Windows 11.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarSi", 0, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarSi", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "hide_recommended",
            category: "desktop",
            name: "Hide Start Menu Recommended",
            description: "Hides the 'Recommended' section in Windows 11 Start Menu.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Education", "IsEducationEnvironment", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Education", "IsEducationEnvironment", RegistryValue::Delete),
            ]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_phone_link_start",
            category: "desktop",
            name: "Disable Phone Link in Start Menu",
            description: "Hides the Phone Link (mobile device) companion in the Windows 11 Start menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Start\Companions\Microsoft.YourPhone_8wekyb3d8bbwe", "IsEnabled", 0, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Start\Companions\Microsoft.YourPhone_8wekyb3d8bbwe", "IsEnabled", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "remove_account_picture_start",
            category: "desktop",
            name: "Remove Account Picture from Start",
            description: "Removes the user account picture and menu from the Windows 11 Start menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideUserTile", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideUserTile", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "remove_change_account_settings",
            category: "desktop",
            name: "Remove 'Change account settings' from Start",
            description: "Removes default 'Change account settings' option from the account picture menu on Start.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideChangeAccountSettings", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideChangeAccountSettings", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "remove_all_apps_start",
            category: "desktop",
            name: "Remove 'All apps' from Start Menu",
            description: "Removes the 'All apps' list from the Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableAllApps", 1, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableAllApps", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableAllApps", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableAllApps", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "remove_category_view_start",
            category: "desktop",
            name: "Remove 'Category View' from All Apps",
            description: "Forces 'All apps' to use Grid/List view by removing Category View option.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideCategoryView", 1, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideCategoryView", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideCategoryView", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideCategoryView", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "remove_common_program_groups",
            category: "desktop",
            name: "Remove Common Program Groups",
            description: "Removes 'All Users' shortcuts from the Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCommonGroups", 1, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCommonGroups", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCommonGroups", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoCommonGroups", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "add_run_as_different_user_start",
            category: "desktop",
            name: "Add 'Run as different user' to Start",
            description: "Adds the 'Run as different user' option to context menus in the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "ShowRunAsDifferentUserInStart", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "ShowRunAsDifferentUserInStart", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "ShowRunAsDifferentUserInStart", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "ShowRunAsDifferentUserInStart", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "hide_recently_added_apps",
            category: "desktop",
            name: "Hide 'Recently added' apps",
            description: "Hides the 'Recently added' apps list from the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecentlyAddedApps", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecentlyAddedApps", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecentlyAddedApps", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecentlyAddedApps", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "hide_recommended_websites",
            category: "desktop",
            name: "Hide Recommended Websites",
            description: "Hides recommended websites from the Start Menu Recommended section.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", 1, RegistryValue::Delete),
            ],
            disabled_ops: Some(&[
                crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", RegistryValue::Delete),
            ]),
        },
        crate::tweak! {
            id: "hide_power_button_start_menu",
            category: "desktop",
            name: "Hide Power Button on Start Menu",
            description: "Hides the power button from the Windows 11 Start menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HidePowerButton", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HidePowerButton", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "hide_restart_in_power_menu",
            category: "desktop",
            name: "Hide Restart in Power Menu",
            description: "Removes the 'Restart' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideRestart", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideRestart", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "hide_shut_down_in_power_menu",
            category: "desktop",
            name: "Hide Shut Down in Power Menu",
            description: "Removes the 'Shut Down' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideShutDown", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideShutDown", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "remove_sleep_power_menu",
            category: "desktop",
            name: "Hide Sleep in Power Menu",
            description: "Removes the 'Sleep' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowPowerSleep", "value", 0, 1),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowPowerSleep", "value", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "remove_sign_out_start_menu",
            category: "desktop",
            name: "Hide Sign Out in Start Menu",
            description: "Removes the 'Sign out' option from the account picture menu in the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideSignOut", "value", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideSignOut", "value", 0, 0),
            ]),
        },
        crate::tweak! {
            id: "paint_desktop_version",
            category: "desktop",
            name: "Show Windows Version on Desktop",
            description: "Shows Windows version and build number in the bottom-right corner of the desktop.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "PaintDesktopVersion", 1, 0),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "PaintDesktopVersion", 0, 0),
            ]),
        },
];
