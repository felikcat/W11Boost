// Accessibility tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static ACCESSIBILITY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_accessibility_sounds",
            category: "accessibility",
            name: "Disable Accessibility Sounds",
            description: "Disables sounds when turning sticky keys, toggle keys, or filter keys on/off.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility",
                    value_name: "Sound on Activation",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1), // Default is usually ON (1) or system dependent
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility",
                    value_name: "Warning Sounds",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility",
                    value_name: "Sound on Activation",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility",
                    value_name: "Warning Sounds",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
            ]),
        },
        crate::tweak! {
            id: "accessibility_high_contrast_flags",
            category: "accessibility",
            name: "Set High Contrast Flags",
            description: "Sets High Contrast flags to 4194 (Custom optimized value).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\HighContrast",
                    value_name: "Flags",
                    value: RegistryValue::String("4194"),
                    stock_value: RegistryValue::String("4222"), // Common default, verify?
                },
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
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\Keyboard Response",
                    value_name: "Flags",
                    value: RegistryValue::String("2"),
                    stock_value: RegistryValue::String("126"), // Common default
                },
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
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\MouseKeys",
                    value_name: "Flags",
                    value: RegistryValue::String("2"),
                    stock_value: RegistryValue::String("62"), // Common default
                },
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
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\StickyKeys",
                    value_name: "Flags",
                    value: RegistryValue::String("2"),
                    stock_value: RegistryValue::String("510"), // Common default
                },
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
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\ToggleKeys",
                    value_name: "Flags",
                    value: RegistryValue::String("34"),
                    stock_value: RegistryValue::String("62"), // Common default
                },
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
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\SlateLaunch",
                    value_name: "LaunchAT",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\Accessibility\SlateLaunch",
                    value_name: "ATapp",
                    value: RegistryValue::String(""),
                    stock_value: RegistryValue::String("narrator.exe"), // Often narrator
                },
            ],
            disabled_ops: None,
        },
];
