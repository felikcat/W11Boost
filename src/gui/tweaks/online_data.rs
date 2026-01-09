// Online Data Collection tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static ONLINE_DATA_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_online_tips",
                category: "online_data",
                name: "Disable Online Tips",
                description: "Prevents Windows from fetching online tips and help content.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips",
                        value_name: "AllowOnlineTips",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips",
                        value_name: "AllowOnlineTips",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_device_metadata",
                category: "online_data",
                name: "Disable Device Metadata Retrieval",
                description: "Prevents Windows from downloading device metadata from the Internet.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata",
                        value_name: "PreventDeviceMetadataFromNetwork",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata",
                        value_name: "PreventDeviceMetadataFromNetwork",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_search_companion",
                category: "online_data",
                name: "Disable Search Companion Updates",
                description: "Prevents Search Companion from downloading content file updates.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\SearchCompanion",
                        value_name: "DisableContentFileUpdates",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\SearchCompanion",
                        value_name: "DisableContentFileUpdates",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_disk_health_updates",
                category: "online_data",
                name: "Disable Disk Health Model Updates",
                description: "Prevents downloading updates to the Disk Failure Prediction Model.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth",
                        value_name: "AllowDiskHealthModelUpdates",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth",
                        value_name: "AllowDiskHealthModelUpdates",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_cloud_content",
                category: "online_data",
                name: "Disable Cloud Content",
                description: "Disables cloud-optimized content and Windows consumer features (ads).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableCloudOptimizedContent",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableConsumerAccountStateContent",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableSoftLanding",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableWindowsConsumerFeatures",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableCloudOptimizedContent",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableConsumerAccountStateContent",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableSoftLanding",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableWindowsConsumerFeatures",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                    id: "disable_speech_updates",
                    category: "online_data",
                    name: "Disable Speech Model Updates",
                    description: "Prevents automatic download of new speech recognition models.",
                    effect: TweakEffect::Restart,
                    enabled_ops: &[RegistryOp {
                            hkey: "HKLM",
                            subkey: r"SOFTWARE\Policies\Microsoft\Speech",
                            value_name: "AllowSpeechModelUpdate",
                            value: RegistryValue::Dword(0),
                            stock_value: RegistryValue::Delete
            }],
                    disabled_ops: Some(&[RegistryOp {
                            hkey: "HKLM",
                            subkey: r"SOFTWARE\Policies\Microsoft\Speech",
                            value_name: "AllowSpeechModelUpdate",
                            value: RegistryValue::Delete,
                            stock_value: RegistryValue::Delete
            }]),
                    requires_restart: true

        },
        crate::tweak! {
            id: "disable_map_updates",
            category: "online_data",
            name: "Disable Offline Map Updates",
            description: "Disables automatic updates for offline maps to save bandwidth.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SYSTEM\Maps",
                    value_name: "AutoUpdateEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1)
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SYSTEM\Maps",
                    value_name: "AutoUpdateEnabled",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
            ]),
        },
];
