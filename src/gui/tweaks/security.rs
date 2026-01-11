// Security tweaks

use super::Tweak;

pub static SECURITY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "restrict_bitlocker_fixed",
        category: "security",
        name: "Deny Write Access to Non-BitLocker Fixed Drives",
        description: "Prevents writing to fixed data drives unless they are protected by BitLocker.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\FVE", "FDVDenyWriteAccess", 1),
        ],
        },
        crate::tweak! {
        id: "restrict_bitlocker_removable",
        category: "security",
        name: "Deny Write Access to Non-BitLocker Removable Drives",
        description: "Prevents writing to removable drives unless they are protected by BitLocker.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\FVE", "RDVDenyWriteAccess", 1),
        ],
        },
        crate::tweak! {
        id: "disable_autoplay",
        category: "security",
        name: "Disable Autoplay",
        description: "Disables Autoplay for all drives.",
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\AutoplayHandlers", "DisableAutoplay", 1),
        ],
                },
        crate::tweak! {
        id: "disable_broad_file_access",
        category: "security",
        name: "Disable Broad File System Access",
        description: "Blocks apps from accessing your entire file system without prompt.",
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny"),
        ],
        },
        crate::tweak! {
        id: "disable_password_reveal",
        category: "security",
        name: "Disable Password Reveal",
        description: "Hides the password reveal button (eye icon) in password fields.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\CredUI", "DisablePasswordReveal", 1),
        ],
        },
        crate::tweak! {
                id: "kernel_stack_protection",
                category: "security",
                name: "Enable Kernel-mode Hardware-enforced Stack Protection",
                description: "Enables a hardware-based security feature that protects against return-oriented programming (ROP) attacks.",
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "KernelStackProtection", 1),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\KernelStackProtection", "Enabled", 1),
                ],
        },
        crate::tweak! {
        id: "enable_pua_protection",
        category: "security",
        name: "Enable PUA Protection",
        description: "Enables Potentially Unwanted Application (PUA) protection in Microsoft Defender.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows Defender\MpEngine", "PUAProtection", 1),
        ],
        },
        crate::tweak! {
        id: "usb_write_protect",
        category: "security",
        name: "Enable USB Write Protection",
        description: "Prevents writing to USB storage devices.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\StorageDevicePolicies", "WriteProtect", 1),
        ],
        },
        crate::tweak! {
            id: "restrict_anonymous",
            category: "security",
            name: "Restrict Anonymous Access",
            description: "Restricts anonymous enumeration of SAM accounts and shares.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Lsa", "restrictanonymous", 1),
            ],
        },
];
