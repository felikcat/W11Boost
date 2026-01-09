// Behavior tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static BEHAVIOR_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_aero_snap",
                category: "behavior",
                name: "Disable Aero Snap",
                description: "Disables window snapping when dragging windows to screen edges.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "WindowArrangementActive",
                        value: RegistryValue::String("0"),
                        stock_value: RegistryValue::String("1")
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "WindowArrangementActive",
                        value: RegistryValue::String("1"),
                        stock_value: RegistryValue::String("1")
        }])
        },
        crate::tweak! {
                id: "balloon_tooltips",
                category: "behavior",
                name: "Enable Balloon Notifications",
                description: "Uses legacy balloon-style notifications instead of modern toast notifications.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "EnableLegacyBalloonNotifications",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                        value_name: "EnableLegacyBalloonNotifications",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_scroll_inactive_windows",
                category: "behavior",
                name: "Disable Scroll Inactive Windows",
                description: "Prevents scrolling windows without clicking to focus them first.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "MouseWheelRouting",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(2)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "MouseWheelRouting",
                        value: RegistryValue::Dword(2),
                        stock_value: RegistryValue::Dword(2)
        }])
        },
        crate::tweak! {
                id: "disable_tablet_mode",
                category: "behavior",
                name: "Disable Tablet Mode",
                description: "Disables tablet mode and touch gestures on non-tablet PCs.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                        value_name: "TabletMode",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(0)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                        value_name: "TabletMode",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(0)
        }])
        },
        crate::tweak! {
                id: "drag_sensitivity",
                category: "behavior",
                name: "Increase Drag & Drop Sensitivity",
                description: "Increases the distance required to start a drag operation (prevents accidental drags).",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "DragWidth",
                                value: RegistryValue::String("20"),
                                stock_value: RegistryValue::String("4")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "DragHeight",
                                value: RegistryValue::String("20"),
                                stock_value: RegistryValue::String("4")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "DragWidth",
                                value: RegistryValue::String("4"),
                                stock_value: RegistryValue::String("4")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "DragHeight",
                                value: RegistryValue::String("4"),
                                stock_value: RegistryValue::String("4")
        },
                ])
        },
        crate::tweak! {
                id: "focus_follows_mouse",
                category: "behavior",
                name: "Enable Focus Follows Mouse",
                description: "Automatically activates windows when hovering over them with the mouse.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "UserPreferencesMask",
                                value: RegistryValue::Dword(0x00000001),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Control Panel\Desktop",
                                value_name: "ActiveWndTrkTimeout",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "ActiveWndTrkTimeout",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_snap_assist",
                category: "behavior",
                name: "Disable Snap Assist",
                description: "Disables the Snap Assist pop-up when snapping windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "WindowArrangementActive",
                        value: RegistryValue::String("0"),
                        stock_value: RegistryValue::String("1")
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "WindowArrangementActive",
                        value: RegistryValue::String("1"),
                        stock_value: RegistryValue::String("1")
                }])
        },
        crate::tweak! {
                    id: "disable_notification_sounds",
                    category: "behavior",
                    name: "Disable Notification Sounds",
                    description: "Disables the sound played when a notification arrives.",
                    effect: TweakEffect::Immediate,
                    enabled_ops: &[RegistryOp {
                            hkey: "HKCU",
                            subkey: r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
                            value_name: "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND",
                            value: RegistryValue::Dword(0),
                            stock_value: RegistryValue::Dword(1)
                    }],
                    disabled_ops: Some(&[RegistryOp {
                            hkey: "HKCU",
                            subkey: r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings",
                            value_name: "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND",
                            value: RegistryValue::Dword(1),
                            stock_value: RegistryValue::Dword(1)
                    }])
        },
        crate::tweak! {
                    id: "enable_balloon_tooltips",
                    category: "behavior",
                    name: "Enable Balloon Tooltips",
                    description: "Restores legacy balloon-style notifications instead of toasts.",
                    effect: TweakEffect::Logoff,
                    enabled_ops: &[RegistryOp {
                            hkey: "HKCU",
                            subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                            value_name: "EnableLegacyBalloonNotifications",
                            value: RegistryValue::Dword(1),
                            stock_value: RegistryValue::Delete
            }],
                    disabled_ops: Some(&[RegistryOp {
                            hkey: "HKCU",
                            subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                            value_name: "EnableLegacyBalloonNotifications",
                            value: RegistryValue::Delete,
                            stock_value: RegistryValue::Delete
            }])
        },
        crate::tweak! {
            id: "optimize_mouse_no_hover_time",
            category: "behavior",
            name: "Reduce Mouse Hover Time",
            description: "Reduces the time required to hover over an item before a tooltip or menu appears.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseHoverTime",
                    value: RegistryValue::String("10"), // 10ms
                    stock_value: RegistryValue::String("400"), // Default 400ms
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseHoverTime",
                    value: RegistryValue::String("400"),
                    stock_value: RegistryValue::String("400"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_mouse_acceleration",
            category: "behavior",
            name: "Disable Mouse Acceleration",
            description: "Disables mouse acceleration for more consistent pointer movement.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseSpeed",
                    value: RegistryValue::String("0"),
                    stock_value: RegistryValue::String("1")
            },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseThreshold1",
                    value: RegistryValue::String("0"),
                    stock_value: RegistryValue::String("6")
            },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseThreshold2",
                    value: RegistryValue::String("0"),
                    stock_value: RegistryValue::String("10")
            },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseSpeed",
                    value: RegistryValue::String("1"),
                    stock_value: RegistryValue::String("1")
            },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseThreshold1",
                    value: RegistryValue::String("6"),
                    stock_value: RegistryValue::String("6")
            },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Mouse",
                    value_name: "MouseThreshold2",
                    value: RegistryValue::String("10"),
                    stock_value: RegistryValue::String("10")
            },
            ])
        },
        crate::tweak! {
            id: "remove_sign_out_alt_f4",
            category: "behavior",
            name: "Remove Sign Out from Alt+F4",
            description: "Removes the 'Sign out' option from the Alt+F4 dialog and Ctrl+Alt+Del screen.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoLogoff",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoLogoff",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                }
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoLogoff",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                    value_name: "NoLogoff",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                }
            ]),
            requires_restart: true,
        },
        crate::tweak! {
            id: "classic_alt_tab",
            category: "behavior",
            name: "Classic Alt+Tab Dialog",
            description: "Enables the Windows XP-style Alt+Tab dialog.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                value_name: "AltTabSettings",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKCU",
                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                value_name: "AltTabSettings",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }])
        },
];
