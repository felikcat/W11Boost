// Behavior tweaks

use super::Tweak;

pub static BEHAVIOR_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "classic_alt_tab",
        category: "behavior",
        name: "Classic Alt+Tab Dialog",
        description: "Enables the Windows XP-style Alt+Tab dialog.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer", "AltTabSettings", 1),
        ],
        },
        crate::tweak! {
        id: "disable_aero_snap",
        category: "behavior",
        name: "Disable Aero Snap",
        description: "Disables window snapping when dragging windows to screen edges.",
        enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "WindowArrangementActive", "0"),
        ],
        },
        crate::tweak! {
        id: "disable_mouse_acceleration",
        category: "behavior",
        name: "Disable Mouse Acceleration",
        description: "Disables mouse acceleration for more consistent pointer movement.",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseSpeed", "0"),
            crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseThreshold1", "0"),
            crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseThreshold2", "0"),
        ],
        },
        crate::tweak! {
        id: "disable_notification_sounds",
        category: "behavior",
        name: "Disable Notification Sounds",
        description: "Disables the sound played when a notification arrives.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_ALLOW_NOTIFICATION_SOUND", 0),
        ],
        },
        crate::tweak! {
        id: "disable_scroll_inactive_windows",
        category: "behavior",
        name: "Disable Scroll Inactive Windows",
        description: "Prevents scrolling windows without clicking to focus them first.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "MouseWheelRouting", 0),
        ],
        },
        crate::tweak! {
        id: "disable_tablet_mode",
        category: "behavior",
        name: "Disable Tablet Mode",
        description: "Disables tablet mode and touch gestures on non-tablet PCs.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ImmersiveShell", "TabletMode", 0),
        ],
        },
        crate::tweak! {
        id: "balloon_tooltips",
        category: "behavior",
        name: "Enable Balloon Notifications",
        description: "Uses legacy balloon-style notifications instead of modern toast notifications.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "EnableLegacyBalloonNotifications", 1),
        ],
        },
        crate::tweak! {
        id: "focus_follows_mouse",
        category: "behavior",
        name: "Enable Focus Follows Mouse",
        description: "Automatically activates windows when hovering over them with the mouse.",
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "UserPreferencesMask", 0x0000_0001),
                crate::reg_dword!("HKCU", r"Control Panel\Desktop", "ActiveWndTrkTimeout", 0),
        ],
        },
        crate::tweak! {
        id: "drag_sensitivity",
        category: "behavior",
        name: "Increase Drag & Drop Sensitivity",
        description: "Increases the distance required to start a drag operation (prevents accidental drags).",
        enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragWidth", "20"),
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "DragHeight", "20"),
        ],
        },
        crate::tweak! {
        id: "optimize_mouse_no_hover_time",
        category: "behavior",
        name: "Reduce Mouse Hover Time",
        description: "Reduces the time required to hover over an item before a tooltip or menu appears.",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Mouse", "MouseHoverTime", "10"),
        ],
                },
];
