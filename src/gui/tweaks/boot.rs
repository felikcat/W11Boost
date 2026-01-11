// Boot & Logon tweaks

use super::Tweak;

pub static BOOT_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "disable_auto_reboot",
        category: "boot",
        name: "Disable Auto Reboot after Updates",
        description: "Prevents Windows from automatically rebooting after installing updates.",
        enabled_ops: &[],
                        custom_apply: Some(|ctx| {
                ctx.post_status("Disabling Reboot task...");
                let _ = crate::run_system_command("schtasks.exe", &["/change", "/tn", r"Microsoft\Windows\UpdateOrchestrator\Reboot", "/disable"]);
                // We skip icacls here as it might be too aggressive for a general tool, sc.exe change is usually enough for most.
                Ok(())
        }),
        },
        crate::tweak! {
                id: "disable_lock_screen",
                category: "boot",
                name: "Disable Lock Screen",
                description: "Disables the lock screen, going directly to login.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Personalization", "NoLockScreen", 1),
                ],
        },
        crate::tweak! {
        id: "disable_login_blur",
        category: "boot",
        name: "Disable Login Screen Blur",
        description: "Disables the acrylic blur effect on the login screen background.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "DisableAcrylicBackgroundOnLogon", 1),
        ],
        },
        crate::tweak! {
        id: "disable_logon_animation",
        category: "boot",
        name: "Disable Logon Animation",
        description: "Disables the animation on the logon screen.",
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\Authentication\LogonUI", "AnimationDisabled", 1),
        ],
                },
        crate::tweak! {
        id: "disable_scoobe",
        category: "boot",
        name: "Disable Windows Welcome Experience",
        description: "Disables the 'Let's finish setting up your device' (SCOOBE) screen.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement", "ScoobeSystemSettingEnabled", 0),
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-310093Enabled", 0),
        ],
        },
        crate::tweak! {
        id: "dont_display_last_user",
        category: "boot",
        name: "Don't Display Last Username",
        description: "Hides the last logged-in username on the login screen for better privacy.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "DontDisplayLastUserName", 1),
        ],
        },
        crate::tweak! {
                id: "enable_startup_sound",
                category: "boot",
                name: "Enable Startup Sound",
                description: "Enables the Windows startup sound.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\BootAnimation", "DisableStartupSound", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\EditionOverrides", "UserSetting_DisableStartupSound", 0),
                ],
        },
        crate::tweak! {
        id: "hide_network_icon_signin",
        category: "boot",
        name: "Hide Network Icon on Sign-in",
        description: "Hides the network connectivity icon from the sign-in screen.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "DontDisplayNetworkSelectionUI", 1),
        ],
        },
        crate::tweak! {
        id: "numlock_on_login",
        category: "boot",
        name: "NumLock On at Login",
        description: "Enables NumLock by default on the login screen.",
        enabled_ops: &[
                crate::reg_str!("HKCU", r".DEFAULT\Control Panel\Keyboard", "InitialKeyboardIndicators", "2147483650"),
        ],
        },
        crate::tweak! {
                id: "reduce_startup_delay",
                category: "boot",
                name: "Reduce Startup Delay",
                description: "Reduces the delay before desktop apps start loading after login.",
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize", "StartupDelayInMSec", 0),
                ],
        },
        crate::tweak! {
        id: "disable_power_button_signin",
        category: "boot",
        name: "Remove Power Button from Sign-in Screen",
        description: "Hides the power/shutdown button from the Windows sign-in screen.",
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "shutdownwithoutlogon", 0),
        ],
                },
        crate::tweak! {
        id: "verbose_logon",
        category: "boot",
        name: "Verbose Logon Messages",
        description: "Shows detailed status messages during startup and shutdown.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "VerboseStatus", 1),
        ],
        },
];
