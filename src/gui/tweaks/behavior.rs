// Behavior tweaks

use super::{Tweak, TweakEffect};

pub static BEHAVIOR_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_aero_snap",
                category: "behavior",
                name: "Disable Aero Snap",
                description: "Disables window snapping when dragging windows to screen edges.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_str!("HKCU", r"Control Panel\Desktop", "WindowArrangementActive", "0", "1"),
                ],
                },
        crate::tweak! {
                id: "balloon_tooltips",
                category: "behavior",
                name: "Enable Balloon Notifications",
                description: "Uses legacy balloon-style notifications instead of modern toast notifications.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "EnableLegacyBalloonNotifications", 1, RegistryValue::Delete),
                ],
                },
        crate::tweak! {
                id: "disable_scroll_inactive_windows",
                category: "behavior",
                name: "Disable Scroll Inactive Windows",
                description: "Prevents scrolling windows without clicking to focus them first.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Control Panel\Desktop", "MouseWheelRouting", 0, 2),
                ],
                },
        crate::tweak! {
                id: "disable_tablet_mode",
                category: "behavior",
                name: "Disable Tablet Mode",
                description: "Disables tablet mode and touch gestures on non-tablet PCs.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ImmersiveShell", "TabletMode", 0, 0),
                ],
                },
        crate::tweak! {
                id: "drag_sensitivity",
                category: "behavior",
                name: "Increase Drag & Drop Sensitivity",
                description: "Increases the distance required to start a drag operation (prevents accidental drags).",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragWidth", "20", "4"),
                        crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragHeight", "20", "4"),
                ],
                },
        crate::tweak! {
                id: "focus_follows_mouse",
                category: "behavior",
                name: "Enable Focus Follows Mouse",
                description: "Automatically activates windows when hovering over them with the mouse.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Control Panel\Desktop", "UserPreferencesMask", 0x0000_0001, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ActiveWndTrkTimeout", 0, RegistryValue::Delete),
                ],
                },
        crate::tweak! {
                id: "disable_notification_sounds",
                category: "behavior",
                name: "Disable Notification Sounds",
                description: "Disables the sound played when a notification arrives.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND", 0, 1),
                ],
                },
        crate::tweak! {
                id: "enable_balloon_tooltips",
                category: "behavior",
                name: "Enable Balloon Tooltips",
                description: "Restores legacy balloon-style notifications instead of toasts.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "EnableLegacyBalloonNotifications", 1, RegistryValue::Delete),
                ],
                },
        crate::tweak! {
            id: "optimize_mouse_no_hover_time",
            category: "behavior",
            name: "Reduce Mouse Hover Time",
            description: "Reduces the time required to hover over an item before a tooltip or menu appears.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseHoverTime", "10", "400"),
            ],
                    },
        crate::tweak! {
            id: "disable_mouse_acceleration",
            category: "behavior",
            name: "Disable Mouse Acceleration",
            description: "Disables mouse acceleration for more consistent pointer movement.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseSpeed", "0", "1"),
                crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseThreshold1", "0", "6"),
                crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseThreshold2", "0", "10"),
            ],
            },
        crate::tweak! {
            id: "remove_sign_out_alt_f4",
            category: "behavior",
            name: "Remove Sign Out from Alt+F4",
            description: "Removes the 'Sign out' option from the Alt+F4 dialog and Ctrl+Alt+Del screen.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoLogoff", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoLogoff", 1, RegistryValue::Delete),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "classic_alt_tab",
            category: "behavior",
            name: "Classic Alt+Tab Dialog",
            description: "Enables the Windows XP-style Alt+Tab dialog.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "AltTabSettings", 1, RegistryValue::Delete),
            ],
            },
];
