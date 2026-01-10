// Network tweaks

use super::{Tweak, TweakEffect};

pub static NETWORK_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "metered_ethernet",
        category: "network",
        name: "Set Ethernet as Metered",
        description: "Sets Ethernet connections as metered to reduce background data usage.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\DefaultMediaCost", "Ethernet", 2, 1),
        ],
        },
        crate::tweak! {
        id: "enable_ip_routing",
        category: "network",
        name: "Enable IP Routing",
        description: "Enables IP forwarding/routing between network interfaces.",
        effect: TweakEffect::Restart,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters", "IPEnableRouter", 1, 0),
        ],
                requires_restart: true
        },
        crate::tweak! {
                id: "msmq_tcp_nodelay",
                category: "network",
                name: "Enable MSMQ TCP NoDelay",
                description: "Disables Nagle's algorithm for MSMQ, potentially reducing latency.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\MSMQ\parameters", "TCPNoDelay", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
            id: "disable_llmnr",
            category: "network",
            name: "Disable LLMNR",
            description: "Disables Link-Local Multicast Name Resolution to improve security.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient", "EnableMulticast", 0),
            ],
                        requires_restart: true
        },
        crate::tweak! {
            id: "disable_remote_assistance",
            category: "network",
            name: "Disable Remote Assistance",
            description: "Disables Windows Remote Assistance connections.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Remote Assistance", "fAllowToGetHelp", 0, 1),
            ],
                    },
        crate::tweak! {
            id: "disable_remote_desktop",
            category: "network",
            name: "Disable Remote Desktop",
            description: "Disables Remote Desktop connections to this computer.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Terminal Server", "fDenyTSConnections", 1, 1),
            ],
                    },
];
