// Network tweaks

use super::Tweak;

pub static NETWORK_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_llmnr",
            category: "network",
            name: "Disable LLMNR",
            description: "Disables Link-Local Multicast Name Resolution to improve security.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient", "EnableMulticast", 0),
            ],
        },
        crate::tweak! {
        id: "disable_remote_assistance",
        category: "network",
        name: "Disable Remote Assistance",
        description: "Disables Windows Remote Assistance connections.",
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Remote Assistance", "fAllowToGetHelp", 0),
        ],
                },
        crate::tweak! {
        id: "disable_remote_desktop",
        category: "network",
        name: "Disable Remote Desktop",
        description: "Disables Remote Desktop connections to this computer.",
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Terminal Server", "fDenyTSConnections", 1),
        ],
                },
        crate::tweak! {
        id: "enable_ip_routing",
        category: "network",
        name: "Enable IP Routing",
        description: "Enables IP forwarding/routing between network interfaces.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters", "IPEnableRouter", 1),
        ],
        },
        crate::tweak! {
        id: "metered_ethernet",
        category: "network",
        name: "Set Ethernet as Metered",
        description: "Sets Ethernet connections as metered to reduce background data usage.",
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\DefaultMediaCost", "Ethernet", 2),
        ],
        },
];
