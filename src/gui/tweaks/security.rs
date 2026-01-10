// Security tweaks

use super::{Tweak, TweakEffect};

pub static SECURITY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_smartscreen",
                category: "security",
                name: "Disable SmartScreen",
                description: "Disables SmartScreen filter in Windows and Store apps via GPO. Warning: Reduces security.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                                custom_apply: Some(|ctx| {
                        use crate::{init_registry_gpo, save_registry_gpo, set_dword_gpo, set_string_gpo};

                        ctx.post_status("Disabling SmartScreen via GPO...");
                        let (hkey, gpo) = init_registry_gpo()?;

                        set_string_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows\System", "**del.ShellSmartScreenLevel", "")?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableSmartScreen", 0)?;

                        // Also set standard registry keys for better coverage
                        let _ = crate::set_string(&winsafe::HKEY::LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer", "SmartScreenEnabled", "Off");
                        let _ = crate::set_dword(&winsafe::HKEY::CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\AppHost", "EnableWebContentEvaluation", 0);

                        save_registry_gpo(hkey, gpo)?;
                        ctx.report_progress();
                        Ok(())
                }),
                gpo_ops: Some(&[
                        crate::gpo_str!(r"SOFTWARE\Policies\Microsoft\Windows\System", "**del.ShellSmartScreenLevel", "", RegistryValue::Delete),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableSmartScreen", 0, 1),
                ]),
        },
        crate::tweak! {
                id: "disable_password_reveal",
                category: "security",
                name: "Disable Password Reveal",
                description: "Hides the password reveal button (eye icon) in password fields.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\CredUI", "DisablePasswordReveal", 1),
                ],
                },
        crate::tweak! {
                id: "disable_defender",
                category: "security",
                name: "Disable Windows Defender",
                description: "Note: Requires Tamper Protection to be disabled in Settings first. Disables Windows Defender Antivirus and its services via GPO.",
                effect: TweakEffect::Restart,
                enabled_ops: &[],
                                custom_apply: Some(|ctx| {
                        use crate::{init_registry_gpo, save_registry_gpo, set_dword_gpo};

                        ctx.post_status("Checking Defender status...");

                        // Check Tamper Protection and Real-time protection
                        let hklm_safe = winsafe::HKEY::LOCAL_MACHINE;
                        let tamper_disabled = crate::check_dword(&hklm_safe, r"SOFTWARE\Microsoft\Windows Defender\Features", "TamperProtection", 4).unwrap_or(false);
                        let realtime_disabled = crate::check_dword(&hklm_safe, r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection", "DisableRealtimeMonitoring", 1).unwrap_or(false);

                        if !tamper_disabled || !realtime_disabled {
                            ctx.post_status("WARNING: Tamper Protection or Real-time protection is enabled. Please disable them in Windows Security settings first.");
                        }

                        ctx.post_status("Disabling Windows Defender via GPO...");
                        let (hkey, gpo) = init_registry_gpo()?;

                        // Disable anti-tamper via GPO
                        let _ = set_dword_gpo(hkey, r"SOFTWARE\Microsoft\Windows Defender\Features", "TamperProtection", 4);

                        // Disable Windows Defender via Policies
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender", "DisableAntiSpyware", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender", "DisableAntiVirus", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender", "ServiceKeepAlive", 0)?;

                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableRealtimeMonitoring", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableBehaviorMonitoring", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableIOAVProtection", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableOnAccessProtection", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableScriptScanning", 1)?;

                        // Direct HKLM writes
                        set_dword_gpo(hkey, r"SOFTWARE\Microsoft\Windows Defender", "DisableAntiSpyware", 1)?;
                        set_dword_gpo(hkey, r"SOFTWARE\Microsoft\Windows Defender", "DisableAntiVirus", 1)?;

                        // Services
                        set_dword_gpo(hkey, r"System\CurrentControlSet\Services\WdFilter", "Start", 4)?;
                        set_dword_gpo(hkey, r"System\CurrentControlSet\Services\WdNisDrv", "Start", 4)?;
                        set_dword_gpo(hkey, r"System\CurrentControlSet\Services\WdNisSvc", "Start", 4)?;
                        set_dword_gpo(hkey, r"System\CurrentControlSet\Services\WinDefend", "Start", 4)?;
                        set_dword_gpo(hkey, r"System\CurrentControlSet\Services\MDCoreSvc", "Start", 4)?;

                        save_registry_gpo(hkey, gpo)?;
                        ctx.report_progress();
                        Ok(())
                }),
                gpo_ops: Some(&[
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender", "DisableAntiSpyware", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender", "DisableAntiVirus", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender", "ServiceKeepAlive", 0, 1),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableRealtimeMonitoring", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableBehaviorMonitoring", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableIOAVProtection", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableOnAccessProtection", 1, 0),
                        crate::gpo_dword!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection", "DisableScriptScanning", 1, 0),
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "usb_write_protect",
                category: "security",
                name: "Enable USB Write Protection",
                description: "Prevents writing to USB storage devices.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\StorageDevicePolicies", "WriteProtect", 1),
                ],
                },
        crate::tweak! {
                id: "disable_uac",
                category: "security",
                name: "Disable UAC",
                description: "Disables User Account Control (UAC). Warning: Significantly reduces security.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "EnableLUA", 0, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_defender_spynet",
                category: "security",
                name: "Disable Defender Cloud Reporting",
                description: "Disables Microsoft Defender Antivirus cloud-based protection (SpyNet) and sample submission.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows Defender\Spynet", "SpyNetReporting", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows Defender\Spynet", "SubmitSamplesConsent", 2),
                ],
                },
        crate::tweak! {
                id: "disable_mrt_reporting",
                category: "security",
                name: "Disable MRT Infection Reporting",
                description: "Prevents Malicious Software Removal Tool (MRT) from reporting infection information back to Microsoft.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\MRT", "DontReportInfectionInformation", 1),
                ],
                },
        crate::tweak! {
                id: "disable_broad_file_access",
                category: "security",
                name: "Disable Broad File System Access",
                description: "Blocks apps from accessing your entire file system without prompt.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
                        crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
                ],
                },
        crate::tweak! {
                id: "disable_memory_integrity",
                category: "security",
                name: "Disable Memory Integrity (Core Isolation)",
                description: "Disables Hypervisor-protected Code Integrity (HVCI). Can improve gaming performance but reduces security.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity", "Enabled", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "kernel_stack_protection",
                category: "security",
                name: "Enable Kernel-mode Hardware-enforced Stack Protection",
                description: "Enables a hardware-based security feature that protects against return-oriented programming (ROP) attacks.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "KernelStackProtection", 1),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\KernelStackProtection", "Enabled", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "enable_pua_protection",
                category: "security",
                name: "Enable PUA Protection",
                description: "Enables Potentially Unwanted Application (PUA) protection in Microsoft Defender.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows Defender\MpEngine", "PUAProtection", 1),
                ],
                },
        crate::tweak! {
                id: "restrict_bitlocker_removable",
                category: "security",
                name: "Deny Write Access to Non-BitLocker Removable Drives",
                description: "Prevents writing to removable drives unless they are protected by BitLocker.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\FVE", "RDVDenyWriteAccess", 1),
                ],
                },
        crate::tweak! {
                id: "restrict_bitlocker_fixed",
                category: "security",
                name: "Deny Write Access to Non-BitLocker Fixed Drives",
                description: "Prevents writing to fixed data drives unless they are protected by BitLocker.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\FVE", "FDVDenyWriteAccess", 1),
                ],
                },
        crate::tweak! {
                id: "disable_smartscreen_phishing_protection",
                category: "security",
                name: "Disable SmartScreen Phishing Protection",
                description: "Disables SmartScreen Phishing Protection service and notifications.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "ServiceEnabled", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "NotifyMalicious", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "NotifyPasswordReuse", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "NotifyUnsafeApp", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_smart_app_control",
                category: "security",
                name: "Disable Smart App Control",
                description: "Attempts to disable Smart App Control. Note: This feature is highly protected and may require a fresh Windows install or offline editing to fully disable if it was enabled during setup.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\CI\Policy", "VerifiedAndReputablePolicyState", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
            id: "disable_task_mgr",
            category: "security",
            name: "Enable/Disable Task Manager",
            description: "Enables or disables the Task Manager for all users.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "DisableTaskMgr", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "DisableTaskMgr", 1),
            ],
                    },
        crate::tweak! {
            id: "restrict_anonymous",
            category: "security",
            name: "Restrict Anonymous Access",
            description: "Restricts anonymous enumeration of SAM accounts and shares.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Lsa", "restrictanonymous", 1, 0),
            ],
                        requires_restart: true
        },
];
