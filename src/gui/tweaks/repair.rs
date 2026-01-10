// System Repair Tweaks
// These tweaks are designed to revert dangerous or harmful optimizations and restore default Windows behavior.

use super::{Tweak, TweakEffect};

pub static REPAIR_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "repair_svc_split_threshold",
            category: "repair",
            name: "Repair Service Split Threshold",
            description: "Restores default service hosting behavior. Fixes potential stability issues caused by forcing all services into a single svchost process.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control", "SvcHostSplitThresholdInKB", 3800000, 3800000), // 3.8GB is default
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Control", "SvcHostSplitThresholdInKB", 3800000, 3800000),
            ],

        },
        crate::tweak! {
            id: "repair_cleanup_timer_resolution",
            category: "repair",
            name: "Remove Timer Resolution Tools",
            description: "Removes third-party timer resolution tools (ISLC, TimerResolution, SetTimerResolutionService) that can cause higher power consumption.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],

            custom_apply: Some(|ctx| {
                ctx.post_status("Cleaning up timer resolution tools...");
                let script = r#"
                # Stop and remove TimerResolution
                Get-Process -Name TimerResolution -ErrorAction SilentlyContinue | Stop-Process -Force
                # Clean up known paths if possible, but mainly stop processes
                
                # Stop and remove ISLC
                Get-Process -Name "Intelligent standby list cleaner ISLC" -ErrorAction SilentlyContinue | Stop-Process -Force

                # Stop and remove Set Timer Resolution Service
                $service = Get-Service -Name "Set Timer Resolution Service", "STR" -ErrorAction SilentlyContinue
                if ($service) {
                    Stop-Service -Name $service.Name -Force -ErrorAction SilentlyContinue
                    sc.exe delete $service.Name *>$null
                }
                Get-Process -Name SetTimerResolutionService -ErrorAction SilentlyContinue | Stop-Process -Force
            "#;
                let _ = crate::run_powershell_command(script);
                Ok(())
            }),
        },
        crate::tweak! {
            id: "repair_win32_priority_separation",
            category: "repair",
            name: "Reset Processor Scheduling",
            description: "Resets Win32PrioritySeparation to default (26 or 2, usually 2 for best bg/fg balance).",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\PriorityControl", "Win32PrioritySeparation", 2, 2), // Default is usually 2 (0x2) or 26 (0x1a)
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Control\PriorityControl", "Win32PrioritySeparation", 2, 2),
            ],

        },
        crate::tweak! {
            id: "repair_tcp_autotuning",
            category: "repair",
            name: "Reset TCP Auto-Tuning",
            description: "Resets TCP Receive Window Auto-Tuning Level to 'normal'. Fixes network throughput issues.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],

            custom_apply: Some(|ctx| {
                ctx.post_status("Resetting TCP Auto-Tuning...");
                let _ = crate::run_system_command("netsh", &["interface", "tcp", "set", "global", "autotuninglevel=normal"]);
                Ok(())
            }),
            command: Some("netsh interface tcp set global autotuninglevel=normal"),
        },
        crate::tweak! {
            id: "repair_sysmain",
            category: "repair",
            name: "Enable SysMain (Superfetch)",
            description: "Enables SysMain service. Improves app launch times and system responsiveness over time.",
            effect: TweakEffect::Restart,
            enabled_ops: &[], // Logic is complex, use custom apply

            custom_apply: Some(|ctx| {
                ctx.post_status("Enabling SysMain service...");
                let _ = crate::run_system_command("sc", &["config", "SysMain", "start=", "auto"]);
                let _ = crate::run_system_command("sc", &["start", "SysMain"]);
                Ok(())
            }),
            command: Some("sc config SysMain start= auto\nsc start SysMain"),
        },
        crate::tweak! {
            id: "repair_ordinary_dpcs",
            category: "repair",
            name: "Repair DPC Settings",
            description: "Removes regular/ordinary DPC overrides that can cause audio stuttering and input lag.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel", "ThreadDpcEnable", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\ControlSet001\Control\Session Manager\kernel", "ThreadDpcEnable", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_spectre_meltdown",
            category: "repair",
            name: "Restore CPU Security Mitigations",
            description: "Re-enables Spectre/Meltdown mitigations. Important for system security.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "FeatureSettingsOverride", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "FeatureSettingsOverrideMask", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management", "FeatureSettingsOverride", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management", "FeatureSettingsOverrideMask", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_hpet",
            category: "repair",
            name: "Enable HPET (High Precision Event Timer)",
            description: "Enables HPET in Device Manager. Required for some software and timing accuracy.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],

            custom_apply: Some(|ctx| {
                ctx.post_status("Enabling HPET device...");
                let cmd = r#"Get-PnpDevice -FriendlyName 'High precision event timer' -ErrorAction SilentlyContinue | Enable-PnpDevice -Confirm:$false"#;
                let _ = crate::run_powershell_command(cmd);
                Ok(())
            }),
        },
        crate::tweak! {
            id: "repair_queue_sizes",
            category: "repair",
            name: "Reset Input Queue Sizes",
            description: "Resets mouse and keyboard data queue sizes to default (100). Fixes input consistency.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters", "KeyboardDataQueueSize", 100, 100),
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\kbdclass\Parameters", "KeyboardDataQueueSize", 100, 100),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters", "MouseDataQueueSize", 100, 100),
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\mouclass\Parameters", "MouseDataQueueSize", 100, 100),
            ],

        },
        crate::tweak! {
            id: "repair_csrss_priority",
            category: "repair",
            name: "Reset CSRSS Priority",
            description: "Removes custom performance options for csrss.exe which can cause instability.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_del_key!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe", "PerfOptions", RegistryValue::DeleteKey),
            ],

        },
        crate::tweak! {
            id: "repair_mpo",
            category: "repair",
            name: "Reset Multi-Plane Overlay (MPO)",
            description: "Restores default MPO behavior. Delete OverlayTestMode key.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\Dwm", "OverlayTestMode", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_memory_management",
            category: "repair",
            name: "Reset Memory Management",
            description: "Resets various memory management tweaks (LargeSystemCache, NonPagedPoolSize, etc.) to defaults.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "ClearPageFileAtShutdown", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "LargeSystemCache", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "NonPagedPoolQuota", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "NonPagedPoolSize", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "PagedPoolQuota", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "PagedPoolSize", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SecondLevelDataCache", 0, 0),
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SystemPages", 0, 0),
                // Delete potentially harmful keys
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "IoPageLockLimit", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "CacheUnmapBehindLengthInMB", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SimulateCommitSavings", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "TrackLockedPages", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "TrackPtes", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DynamicMemory", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "EnforceWriteProtection", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "MakeLowMemory", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SystemCacheLimit", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SessionSpaceLimit", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "WriteWatch", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SnapUnloads", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "MapAllocationFragment", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "Mirroring", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DontVerifyRandomDrivers", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "EnableLowVaAccess", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_raw_mouse_throttle",
            category: "repair",
            name: "Remove Raw Mouse Throttle Tweaks",
            description: "Removes registry values related to mouse throttling that are not effective or harmful.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleEnabled", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleForced", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleDuration", RegistryValue::Delete),
                crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleLeeway", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_prefetch",
            category: "repair",
            name: "Enable Prefetcher",
            description: "Sets EnablePrefetcher to 3 (App launch and Boot enabled). Default is optimal for most systems.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 3, 3),
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 3, 3),
            ],

        },
        crate::tweak! {
            id: "repair_nvme_driver_hacks",
            category: "repair",
            name: "Remove NVMe Driver Hacks",
            description: "Removes unsupported NVMe driver overrides that can cause Inaccessible Boot Device BSODs.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides", "735209102", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides", "1853569164", RegistryValue::Delete),
                crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides", "156965516", RegistryValue::Delete),
            ],

        },
        crate::tweak! {
            id: "repair_appx_service",
            category: "repair",
            name: "Enable AppX Deployment Service",
            description: "Restores AppXSvc. Disabling this breaks Start Menu, Taskbar, and Store apps.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\AppXSvc", "Start", 3, 3), // 3 = Manual (Trigger Start)
            ],

        },
        crate::tweak! {
            id: "repair_uac",
            category: "repair",
            name: "Enable User Account Control (UAC)",
            description: "Restores UAC (EnableLUA). Disabling this breaks UWP apps and causes security risks.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "EnableLUA", 1, 1),
            ],

        },
        crate::tweak! {
            id: "repair_background_apps_global",
            category: "repair",
            name: "Enable Background Apps Globally",
            description: "Restores global background app execution. Required for Notifications, Phone Link, etc. on Windows 11.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications", "GlobalUserDisabled", 0, 0),
            ],

        },
        crate::tweak! {
            id: "repair_tablet_input_service",
            category: "repair",
            name: "Enable Tablet Input Service",
            description: "Restores TabletInputService. Fixes keyboard entry in Start Menu, Settings, and UWP apps.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                 // 3 = Manual (Demand Start)
                 // Using custom command to be safe across different service names if needed, but registry start type is standard
                 crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\TabletInputService", "Start", 3, 3),
            ],

        },
        crate::tweak! {
            id: "repair_ci_env_var",
            category: "repair",
            name: "Remove CI Environment Variable",
            description: "Removes 'CI' environment variable which can interfere with some CLI tools (like gemini-cli).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],

            custom_apply: Some(|ctx| {
                ctx.post_status("Removing CI environment variable...");
                let script = r#"
                [Environment]::SetEnvironmentVariable("CI", $null, "User")
                [Environment]::SetEnvironmentVariable("CI", $null, "Machine")
            "#;
                let _ = crate::run_powershell_command(script);
                Ok(())
            }),
        },
        crate::tweak! {
            id: "repair_error_reporting",
            category: "repair",
            name: "Repair Error Reporting",
            description: "Enables Windows Error Reporting to ensure BSOD mini dumps are created.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "Disabled", 0, 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DontSendAdditionalData", 0, 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DontShowUI", 0, 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", 0, 0),
                crate::reg_dword!("HKLM", r"Software\Microsoft\pchealth\errorreporting", "DoReport", 1, 1),
            ],

        },
];
