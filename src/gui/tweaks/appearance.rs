// Appearance tweaks

use super::{Tweak, TweakEffect};

pub static APPEARANCE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "dark_mode",
                category: "appearance",
                name: "Enable Dark Mode",
                description: "Enables dark mode for apps and system UI.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "AppsUseLightTheme", 0, 1),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "SystemUsesLightTheme", 0, 1),
                ],
                },
        crate::tweak! {
                id: "colored_titlebars",
                category: "appearance",
                name: "Colored Title Bars",
                description: "Shows accent color on window title bars.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\DWM", "ColorPrevalence", 1, 0),
                ],
                },
        crate::tweak! {
                id: "disable_transparency",
                category: "appearance",
                name: "Disable Transparency",
                description: "Disables transparency effects in Windows (taskbar, Start menu, etc.).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency", 0, 0),
                ],
                },
        crate::tweak! {
                id: "menu_show_delay",
                category: "appearance",
                name: "Reduce Menu Show Delay",
                description: "Reduces the delay when hovering over menus to make UI feel snappier.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_str!("HKCU", r"Control Panel\Desktop", "MenuShowDelay", "0", "400"),
                ],
                        },
];
