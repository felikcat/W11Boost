// Accessibility tweaks

use super::Tweak;

pub static ACCESSIBILITY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "disable_accessibility_sounds",
        category: "accessibility",
        name: "Disable Accessibility Sounds",
        description: "Disables sounds when turning sticky keys, toggle keys, or filter keys on/off.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Sound on Activation", 0),
            crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Warning Sounds", 0),
        ],
                },
        crate::tweak! {
        id: "disable_slate_launch",
        category: "accessibility",
        name: "Disable Slate Launch",
        description: "Disables SlateLaunch (Accessibility on tablet).",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Control Panel\Accessibility\SlateLaunch", "LaunchAT", 0),
            crate::reg_str!("HKCU", r"Control Panel\Accessibility\SlateLaunch", "ATapp", ""),
        ],
                },
        crate::tweak! {
            id: "accessibility_high_contrast_flags",
            category: "accessibility",
            name: "Disable High Contrast Shortcut",
            description: "Disables the High Contrast keyboard shortcut (Left Alt + Left Shift + Print Screen).",
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\HighContrast", "Flags", "4194"),
            ],
             // Hard to revert to unknown default without capturing it
        },
        crate::tweak! {
        id: "accessibility_keyboard_response_flags",
        category: "accessibility",
        name: "Disable Filter Keys Shortcut",
        description: "Disables the Filter Keys keyboard shortcut (Hold Right Shift for 8 seconds).",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Accessibility\Keyboard Response", "Flags", "2"),
        ],
                },
        crate::tweak! {
        id: "accessibility_mouse_keys_flags",
        category: "accessibility",
        name: "Disable Mouse Keys Shortcut",
        description: "Disables the Mouse Keys keyboard shortcut (Left Alt + Left Shift + Num Lock).",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Accessibility\MouseKeys", "Flags", "2"),
        ],
                },
        crate::tweak! {
        id: "accessibility_sticky_keys_flags",
        category: "accessibility",
        name: "Disable Sticky Keys Shortcut",
        description: "Disables the Sticky Keys keyboard shortcut (Press Shift 5 times).",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Accessibility\StickyKeys", "Flags", "2"),
        ],
                },
        crate::tweak! {
        id: "accessibility_toggle_keys_flags",
        category: "accessibility",
        name: "Disable Toggle Keys Shortcut",
        description: "Disables the Toggle Keys keyboard shortcut (Hold Num Lock for 5 seconds).",
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Control Panel\Accessibility\ToggleKeys", "Flags", "34"),
        ],
                },
];
