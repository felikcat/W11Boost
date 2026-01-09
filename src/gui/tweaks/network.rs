// Network tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static NETWORK_TWEAKS: &[Tweak] = &[
        crate::tweak! {
        id: "metered_ethernet",
        category: "network",
        name: "Set Ethernet as Metered",
        description: "Sets Ethernet connections as metered to reduce background data usage.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\DefaultMediaCost",
                value_name: "Ethernet",
                value: RegistryValue::Dword(2),
                stock_value: RegistryValue::Dword(1)
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\NetworkList\DefaultMediaCost",
                value_name: "Ethernet",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
        id: "enable_ip_routing",
        category: "network",
        name: "Enable IP Routing",
        description: "Enables IP forwarding/routing between network interfaces.",
        effect: TweakEffect::Restart,
        enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                value_name: "IPEnableRouter",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(0)
        }],
        disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters",
                value_name: "IPEnableRouter",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(0)
        }]),
        requires_restart: true
        },
        crate::tweak! {
                id: "msmq_tcp_nodelay",
                category: "network",
                name: "Enable MSMQ TCP NoDelay",
                description: "Disables Nagle's algorithm for MSMQ, potentially reducing latency.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\MSMQ\parameters",
                        value_name: "TCPNoDelay",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\MSMQ\parameters",
                        value_name: "TCPNoDelay",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }]),
                requires_restart: true
        },
        crate::tweak! {
            id: "disable_llmnr",
            category: "network",
            name: "Disable LLMNR",
            description: "Disables Link-Local Multicast Name Resolution to improve security.",
            effect: TweakEffect::Restart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
                value_name: "EnableMulticast",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows NT\DNSClient",
                value_name: "EnableMulticast",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_remote_assistance",
            category: "network",
            name: "Disable Remote Assistance",
            description: "Disables Windows Remote Assistance connections.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
                value_name: "fAllowToGetHelp",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Dword(1)
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\Remote Assistance",
                value_name: "fAllowToGetHelp",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1)
            }]),
        },
        crate::tweak! {
            id: "disable_remote_desktop",
            category: "network",
            name: "Disable Remote Desktop",
            description: "Disables Remote Desktop connections to this computer.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\Terminal Server",
                value_name: "fDenyTSConnections",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Dword(1) // Default can vary
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SYSTEM\CurrentControlSet\Control\Terminal Server",
                value_name: "fDenyTSConnections",
                value: RegistryValue::Dword(0), // 0 means ALLOW connections
                stock_value: RegistryValue::Dword(0)
            }]),
        },
];
