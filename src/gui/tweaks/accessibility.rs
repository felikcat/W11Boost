// Accessibility tweaks

use super::{Tweak, TweakEffect};

pub static ACCESSIBILITY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_accessibility_sounds",
            category: "accessibility",
            name: "Disable Accessibility Sounds",
            description: "Disables sounds when turning sticky keys, toggle keys, or filter keys on/off.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Sound on Activation", 0, 1),
                crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Warning Sounds", 0, 1),
            ],
            disabled_ops: Some(&[
                crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Sound on Activation", 1, 1),
                crate::reg_dword!("HKCU", r"Control Panel\Accessibility", "Warning Sounds", 1, 1),
            ]),
        },
        crate::tweak! {
            id: "accessibility_high_contrast_flags",
            category: "accessibility",
            name: "Set High Contrast Flags",
            description: "Sets High Contrast flags to 4194 (Custom optimized value).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\HighContrast", "Flags", "4194", "4222"),
            ],
            disabled_ops: None, // Hard to revert to unknown default without capturing it
        },
        crate::tweak! {
            id: "accessibility_keyboard_response_flags",
            category: "accessibility",
            name: "Set Keyboard Response Flags",
            description: "Sets Keyboard Response flags to 2.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\Keyboard Response", "Flags", "2", "126"),
            ],
            disabled_ops: None,
        },
        crate::tweak! {
            id: "accessibility_mouse_keys_flags",
            category: "accessibility",
            name: "Set Mouse Keys Flags",
            description: "Sets Mouse Keys flags to 2.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\MouseKeys", "Flags", "2", "62"),
            ],
            disabled_ops: None,
        },
        crate::tweak! {
            id: "accessibility_sticky_keys_flags",
            category: "accessibility",
            name: "Set Sticky Keys Flags",
            description: "Sets Sticky Keys flags to 2 (Disabled).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\StickyKeys", "Flags", "2", "510"),
            ],
            disabled_ops: None,
        },
        crate::tweak! {
            id: "accessibility_toggle_keys_flags",
            category: "accessibility",
            name: "Set Toggle Keys Flags",
            description: "Sets Toggle Keys flags to 34.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\ToggleKeys", "Flags", "34", "62"),
            ],
            disabled_ops: None,
        },
        crate::tweak! {
            id: "disable_slate_launch",
            category: "accessibility",
            name: "Disable Slate Launch",
            description: "Disables SlateLaunch (Accessibility on tablet).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\Accessibility\SlateLaunch", "LaunchAT", 0, 1),
                crate::reg_str!("HKCU", r"Control Panel\Accessibility\SlateLaunch", "ATapp", "", "narrator.exe"),
            ],
            disabled_ops: None,
        },
];
