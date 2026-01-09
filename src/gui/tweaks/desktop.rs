// Desktop & Taskbar tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static DESKTOP_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_aero_shake",
                category: "desktop",
                name: "Disable Aero Shake",
                description: "Prevents minimizing all windows when shaking a window titlebar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "DisallowShaking",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "DisallowShaking",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "show_seconds_clock",
                category: "desktop",
                name: "Show Seconds in Clock",
                description: "Displays seconds in the taskbar clock.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowSecondsInSystemClock",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowSecondsInSystemClock",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "taskbar_end_task",
                category: "desktop",
                name: "Enable End Task in Taskbar",
                description: "Adds an 'End task' option when right-clicking taskbar items. Useful for unresponsive apps.",
                effect: TweakEffect::Immediate, // Actually might be immediate or require explorer restart, tutorial says nothing about restart but usually these need one. Let's assume ExplorerRestart to be safe.
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
                        value_name: "TaskbarEndTask",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\TaskbarDeveloperSettings",
                        value_name: "TaskbarEndTask",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "taskbar_never_combine",
                category: "desktop",
                name: "Never Combine Taskbar Buttons",
                description: "Shows labels and never combines buttons on the taskbar (including multi-monitor taskbars).",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "TaskbarGlomLevel",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Dword(0), // Default is Always Combine (0)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "MMTaskbarGlomLevel",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Dword(0)
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "TaskbarGlomLevel",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "MMTaskbarGlomLevel",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                ]), // Explorer restart is handled by effect
        },
        crate::tweak! {
                id: "hide_task_view",
                category: "desktop",
                name: "Hide Task View Button",
                description: "Hides the Task View button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowTaskViewButton",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "ShowTaskViewButton",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "hide_task_view_policy",
                category: "desktop",
                name: "Hide Task View Button (Policy)",
                description: "Hides the Task View button from the taskbar using Group Policy for stricter enforcement.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "HideTaskViewButton",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "HideTaskViewButton",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }])
        },
        crate::tweak! {
                id: "hide_search_box",
                category: "desktop",
                name: "Hide Search Box",
                description: "Removes the search box/icon from the taskbar and disables search suggestions.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
                                value_name: "SearchboxTaskbarMode",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchBoxSuggestions",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Search",
                                value_name: "SearchboxTaskbarMode",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchBoxSuggestions",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "hide_chat_icon",
                category: "desktop",
                name: "Hide Teams Chat Icon",
                description: "Hides the Microsoft Teams chat icon from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "TaskbarMn",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "TaskbarMn",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_chat_icon_policy",
                category: "desktop",
                name: "Disable Chat Icon (Policy)",
                description: "Disables the Chat icon on the Taskbar using Group Policy for stricter enforcement.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Chat",
                        value_name: "ChatIcon",
                        value: RegistryValue::Dword(3),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Chat",
                        value_name: "ChatIcon",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }])
        },
        crate::tweak! {
                id: "last_active_click",
                category: "desktop",
                name: "Last Active Click",
                description: "Clicking grouped taskbar button opens last active window instead of thumbnail preview.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "LastActiveClick",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "LastActiveClick",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_notification_center",
                category: "desktop",
                name: "Disable Notification Center",
                description: "Removes the Action Center / Notification Center from the taskbar.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "DisableNotificationCenter",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "DisableNotificationCenter",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "remove_shortcut_arrow",
                category: "desktop",
                name: "Remove Shortcut Arrow",
                description: "Removes the arrow icon from shortcuts on the desktop and explorer.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons",
                        value_name: "29",
                        value: RegistryValue::String(r"%windir%\System32\shell32.dll,51"),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Icons",
                        value_name: "29",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "taskbar_button_width",
                category: "desktop",
                name: "Minimize Taskbar Button Width",
                description: "Sets the minimum width for taskbar buttons.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop\WindowMetrics",
                        value_name: "MinWidth",
                        value: RegistryValue::String("38"),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop\WindowMetrics",
                        value_name: "MinWidth",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "taskbar_flash_count",
                category: "desktop",
                name: "Disable Taskbar Flashing",
                description: "Stops taskbar buttons from flashing for attention.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "ForegroundFlashCount",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(7)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "ForegroundFlashCount",
                        value: RegistryValue::Dword(7),
                        stock_value: RegistryValue::Dword(7)
        }])
        },
        crate::tweak! {
                id: "hide_meet_now",
                category: "desktop",
                name: "Hide Meet Now Icon",
                description: "Hides the 'Meet Now' icon from the taskbar notification area.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "HideSCAMeetNow",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "HideSCAMeetNow",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "HideSCAMeetNow",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "HideSCAMeetNow",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_news_interests",
                category: "desktop",
                name: "Disable News and Interests (Widgets)",
                description: "Disables the 'News and Interests' (Widgets) feature on the taskbar and lock screen.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Dsh",
                                value_name: "AllowNewsAndInterests",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\NewsAndInterests\AllowNewsAndInterests",
                                value_name: "value",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Dsh",
                                value_name: "AllowNewsAndInterests",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\NewsAndInterests\AllowNewsAndInterests",
                                value_name: "value",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                ])
        },
        crate::tweak! {
                id: "hide_people_button",
                category: "desktop",
                name: "Hide People Button",
                description: "Hides the People button from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People",
                        value_name: "PeopleBand",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People",
                        value_name: "PeopleBand",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_search_highlights",
                category: "desktop",
                name: "Disable Search Highlights",
                description: "Disables dynamic content and highlights in the search box.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                        value_name: "IsDynamicSearchBoxEnabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                        value_name: "IsDynamicSearchBoxEnabled",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_start_recommendations",
                category: "desktop",
                name: "Disable Start Recommendations",
                description: "Disables tracking of recently opened programs and documents for the Start menu.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackDocs",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackDocs",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ])
        },
        crate::tweak! {
                id: "add_desktop_search_box",
                category: "desktop",
                name: "Show Desktop Search Box",
                description: "Shows a search box on the desktop for quick access to Windows Search. (Windows 11 Build 25120+)",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "DesktopSearchBox",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "DesktopSearchBox",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
                }])
        },
        crate::tweak! {
                id: "remove_bluetooth_icon",
                category: "desktop",
                name: "Remove Bluetooth Icon",
                description: "Removes the Bluetooth icon from the taskbar notification area.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Bluetooth",
                        value_name: "Notification Area Icon",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Bluetooth",
                        value_name: "Notification Area Icon",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
                }])
        },
        crate::tweak! {
                id: "remove_ask_copilot_taskbar",
                category: "desktop",
                name: "Remove 'Ask Copilot' from Taskbar",
                description: "Removes the 'Ask Copilot' button/icon from the taskbar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "TaskbarCompanion",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                        value_name: "TaskbarCompanion",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
                }])
        },
        crate::tweak! {
                id: "disable_web_search",
                category: "desktop",
                name: "Disable Web Search Results",
                description: "Prevents Windows Search from showing web results and Copilot suggestions.",
                effect: TweakEffect::ExplorerRestart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Search",
                                value_name: "BingSearchEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchBoxSuggestions",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        }
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Search",
                                value_name: "BingSearchEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchBoxSuggestions",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        }
                ])
        },
        crate::tweak! {
            id: "remove_desktop_search_box",
            category: "desktop",
            name: "Remove Desktop Search Box",
            description: "Removes the search box from the desktop (if present).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "DesktopSearchBox",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "DesktopSearchBox",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_spotlight_icon",
            category: "desktop",
            name: "Remove 'Learn about this picture' Icon",
            description: "Removes the 'Learn about this picture' icon from the desktop when using Windows Spotlight.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                    value_name: "",
                    value: RegistryValue::DeleteKey,
                    stock_value: RegistryValue::String("Windows Spotlight"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Desktop\NameSpace\{2cc5ca98-6485-489a-920e-b3e88a6ccce3}",
                    value_name: "",
                    value: RegistryValue::String("Windows Spotlight"),
                    stock_value: RegistryValue::String("Windows Spotlight"),
                },
            ]),
        },
        crate::tweak! {
            id: "remove_this_pc_desktop_icon",
            category: "desktop",
            name: "Remove 'This PC' Desktop Icon",
            description: "Hides the 'This PC' icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu",
                    value_name: "{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
        },
        crate::tweak! {
            id: "remove_user_files_desktop_icon",
            category: "desktop",
            name: "Remove 'User Files' Desktop Icon",
            description: "Hides the User Files folder icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{59031a47-3f72-44a7-89c5-5595fe6b30ee}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu",
                    value_name: "{59031a47-3f72-44a7-89c5-5595fe6b30ee}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
        },
        crate::tweak! {
            id: "remove_network_desktop_icon",
            category: "desktop",
            name: "Remove 'Network' Desktop Icon",
            description: "Hides the Network icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu",
                    value_name: "{F02C1A0D-BE21-4350-88B0-7367FC96EF3C}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
        },
        crate::tweak! {
            id: "remove_recycle_bin_desktop_icon",
            category: "desktop",
            name: "Remove 'Recycle Bin' Desktop Icon",
            description: "Hides the Recycle Bin icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{645FF040-5081-101B-9F08-00AA002F954E}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu",
                    value_name: "{645FF040-5081-101B-9F08-00AA002F954E}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
        },
        crate::tweak! {
            id: "remove_control_panel_desktop_icon",
            category: "desktop",
            name: "Remove 'Control Panel' Desktop Icon",
            description: "Hides the Control Panel icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\ClassicStartMenu",
                    value_name: "{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
        },
        crate::tweak! {
            id: "remove_onedrive_desktop_icon",
            category: "desktop",
            name: "Remove 'OneDrive' Desktop Icon",
            description: "Hides the OneDrive icon from the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                    value_name: "{018D5C66-4533-4307-9B53-224DE2ED1FE6}",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(0), // Default is usually shown if installed
                },
            ]),
        },
        crate::tweak! {
            id: "drag_full_windows",
            category: "desktop",
            name: "Disable Drag Full Windows",
            description: "Shows only window outline while dragging to improve performance.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "DragFullWindows",
                    value: RegistryValue::String("0"),
                    stock_value: RegistryValue::String("1"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "DragFullWindows",
                    value: RegistryValue::String("1"),
                    stock_value: RegistryValue::String("1"),
                },
            ]),
        },
        crate::tweak! {
            id: "foreground_lock_timeout",
            category: "desktop",
            name: "Reduce Foreground Lock Timeout",
            description: "Reduces the time before a window can steal focus (set to 0).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "ForegroundLockTimeout",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(200000), // Default 200s
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "ForegroundLockTimeout",
                    value: RegistryValue::Dword(200000),
                    stock_value: RegistryValue::Dword(200000),
                },
            ]),
        },
        crate::tweak! {
            id: "jpeg_import_quality",
            category: "desktop",
            name: "Increase Wallpaper JPEG Quality",
            description: "Set wallpaper import quality to 100% (disable compression artifacts).",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "JPEGImportQuality",
                    value: RegistryValue::Dword(100),
                    stock_value: RegistryValue::Delete, // Doesn't exist by default (85%)
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Desktop",
                    value_name: "JPEGImportQuality",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "hide_recommended_section",
            category: "desktop",
            name: "Hide Recommended Section in Start Menu",
            description: "Hides the 'Recommended' section (files/apps) in the Windows 11 Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\PolicyManager\current\device\start",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\PolicyManager\current\device\start",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "show_libraries_desktop",
            category: "desktop",
            name: "Show Libraries on Desktop",
            description: "Shows the Libraries icon on the desktop.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                value_name: "{031E4825-7B94-4DC3-B131-E946B44C8DD5}",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(1)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\HideDesktopIcons\NewStartPanel",
                value_name: "{031E4825-7B94-4DC3-B131-E946B44C8DD5}",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }])
        },
        crate::tweak! {
            id: "disable_copilot_taskbar",
            category: "desktop",
            name: "Disable Copilot Taskbar Button",
            description: "Hides the Copilot button from the taskbar.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "ShowCopilotButton",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "ShowCopilotButton",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
            }])
        },
        crate::tweak! {
            id: "remove_notification_bell",
            category: "desktop",
            name: "Remove Notification Bell Icon",
            description: "Removes the notification bell icon from the taskbar system tray.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "ShowNotificationIcon",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                    value_name: "ShowNotificationIcon",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
            ]),
        },
        crate::tweak! {
            id: "hide_security_notification_icon",
            category: "desktop",
            name: "Hide Windows Security Icon",
            description: "Hides the Windows Security notification icon from the taskbar system tray.",
            effect: TweakEffect::Restart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray",
                value_name: "HideSystray",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray",
                value_name: "HideSystray",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }]),
            requires_restart: true
        },
        crate::tweak! {
            id: "taskbar_left_align",
            category: "desktop",
            name: "Left-Align Taskbar",
            description: "Aligns taskbar icons to the left instead of center (Windows 11 default).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                value_name: "TaskbarAl",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                value_name: "TaskbarAl",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }])
        },
        crate::tweak! {
            id: "small_taskbar",
            category: "desktop",
            name: "Small Taskbar",
            description: "Uses smaller taskbar size in Windows 11.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                value_name: "TaskbarSi",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                value_name: "TaskbarSi",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }])
        },
        crate::tweak! {
            id: "hide_recommended",
            category: "desktop",
            name: "Hide Start Menu Recommended",
            description: "Hides the 'Recommended' section in Windows 11 Start Menu.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Education",
                    value_name: "IsEducationEnvironment",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedSection",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Education",
                    value_name: "IsEducationEnvironment",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
            ]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_phone_link_start",
            category: "desktop",
            name: "Disable Phone Link in Start Menu",
            description: "Hides the Phone Link (mobile device) companion in the Windows 11 Start menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Start\Companions\Microsoft.YourPhone_8wekyb3d8bbwe",
                value_name: "IsEnabled",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Start\Companions\Microsoft.YourPhone_8wekyb3d8bbwe",
                value_name: "IsEnabled",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }])
        },
        crate::tweak! {
            id: "remove_account_picture_start",
            category: "desktop",
            name: "Remove Account Picture from Start",
            description: "Removes the user account picture and menu from the Windows 11 Start menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideUserTile",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideUserTile",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }])
        },
        crate::tweak! {
            id: "remove_change_account_settings",
            category: "desktop",
            name: "Remove 'Change account settings' from Start",
            description: "Removes default 'Change account settings' option from the account picture menu on Start.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideChangeAccountSettings",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideChangeAccountSettings",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }])
        },
        crate::tweak! {
            id: "remove_all_apps_start",
            category: "desktop",
            name: "Remove 'All apps' from Start Menu",
            description: "Removes the 'All apps' list from the Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableAllApps",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableAllApps",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableAllApps",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableAllApps",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "remove_category_view_start",
            category: "desktop",
            name: "Remove 'Category View' from All Apps",
            description: "Forces 'All apps' to use Grid/List view by removing Category View option.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideCategoryView",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideCategoryView",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideCategoryView",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideCategoryView",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "remove_common_program_groups",
            category: "desktop",
            name: "Remove Common Program Groups",
            description: "Removes 'All Users' shortcuts from the Start Menu.",
            effect: TweakEffect::ExplorerRestart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoCommonGroups",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoCommonGroups",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoCommonGroups",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoCommonGroups",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "add_run_as_different_user_start",
            category: "desktop",
            name: "Add 'Run as different user' to Start",
            description: "Adds the 'Run as different user' option to context menus in the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "ShowRunAsDifferentUserInStart",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "ShowRunAsDifferentUserInStart",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "ShowRunAsDifferentUserInStart",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "ShowRunAsDifferentUserInStart",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "hide_recently_added_apps",
            category: "desktop",
            name: "Hide 'Recently added' apps",
            description: "Hides the 'Recently added' apps list from the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecentlyAddedApps",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecentlyAddedApps",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecentlyAddedApps",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecentlyAddedApps",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "hide_recommended_websites",
            category: "desktop",
            name: "Hide Recommended Websites",
            description: "Hides recommended websites from the Start Menu Recommended section.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedPersonalizedSites",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedPersonalizedSites",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedPersonalizedSites",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                    value_name: "HideRecommendedPersonalizedSites",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "hide_power_button_start_menu",
            category: "desktop",
            name: "Hide Power Button on Start Menu",
            description: "Hides the power button from the Windows 11 Start menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HidePowerButton",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HidePowerButton",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }]),
        },
        crate::tweak! {
            id: "hide_restart_in_power_menu",
            category: "desktop",
            name: "Hide Restart in Power Menu",
            description: "Removes the 'Restart' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideRestart",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideRestart",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }]),
        },
        crate::tweak! {
            id: "hide_shut_down_in_power_menu",
            category: "desktop",
            name: "Hide Shut Down in Power Menu",
            description: "Removes the 'Shut Down' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideShutDown",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideShutDown",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }]),
        },
        crate::tweak! {
            id: "remove_sleep_power_menu",
            category: "desktop",
            name: "Hide Sleep in Power Menu",
            description: "Removes the 'Sleep' option from the Start Menu power options.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowPowerSleep",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(1)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Settings\AllowPowerSleep",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }]),
        },
        crate::tweak! {
            id: "remove_sign_out_start_menu",
            category: "desktop",
            name: "Hide Sign Out in Start Menu",
            description: "Removes the 'Sign out' option from the account picture menu in the Start Menu.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideSignOut",
                value_name: "value",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\PolicyManager\default\Start\HideSignOut",
                value_name: "value",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }]),
        },
        crate::tweak! {
            id: "paint_desktop_version",
            category: "desktop",
            name: "Show Windows Version on Desktop",
            description: "Shows Windows version and build number in the bottom-right corner of the desktop.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Control Panel\Desktop",
                value_name: "PaintDesktopVersion",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Control Panel\Desktop",
                value_name: "PaintDesktopVersion",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
            }])
        },
];
