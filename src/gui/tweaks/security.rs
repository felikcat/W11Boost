// Security tweaks

use super::{Tweak, TweakEffect};

pub static SECURITY_TWEAKS: &[Tweak] = &[
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
