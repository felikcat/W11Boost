// Appearance tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static APPEARANCE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "dark_mode",
                category: "appearance",
                name: "Enable Dark Mode",
                description: "Enables dark mode for apps and system UI.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                                value_name: "AppsUseLightTheme",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                                value_name: "SystemUsesLightTheme",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                                value_name: "AppsUseLightTheme",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                                value_name: "SystemUsesLightTheme",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ])
        },
        crate::tweak! {
                id: "colored_titlebars",
                category: "appearance",
                name: "Colored Title Bars",
                description: "Shows accent color on window title bars.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\DWM",
                        value_name: "ColorPrevalence",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(0)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\DWM",
                        value_name: "ColorPrevalence",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(0)
        }])
        },
        crate::tweak! {
                id: "disable_transparency",
                category: "appearance",
                name: "Disable Transparency",
                description: "Disables transparency effects in Windows (taskbar, Start menu, etc.).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                        value_name: "EnableTransparency",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(0)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                        value_name: "EnableTransparency",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(0)
        }])
        },
        crate::tweak! {
                id: "menu_show_delay",
                category: "appearance",
                name: "Reduce Menu Show Delay",
                description: "Reduces the delay when hovering over menus to make UI feel snappier.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "MenuShowDelay",
                        value: RegistryValue::String("0"),
                        stock_value: RegistryValue::String("400")
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\Desktop",
                        value_name: "MenuShowDelay",
                        value: RegistryValue::String("400"),
                        stock_value: RegistryValue::String("400")
                }]),
        },
];
