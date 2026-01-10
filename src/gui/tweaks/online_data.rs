// Online Data Collection tweaks

use super::{Tweak, TweakEffect};

pub static ONLINE_DATA_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_online_tips",
                category: "online_data",
                name: "Disable Online Tips",
                description: "Prevents Windows from fetching online tips and help content.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips", "AllowOnlineTips", 1),
                ],
                },
        crate::tweak! {
                id: "disable_device_metadata",
                category: "online_data",
                name: "Disable Device Metadata Retrieval",
                description: "Prevents Windows from downloading device metadata from the Internet.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata", "PreventDeviceMetadataFromNetwork", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_search_companion",
                category: "online_data",
                name: "Disable Search Companion Updates",
                description: "Prevents Search Companion from downloading content file updates.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\SearchCompanion", "DisableContentFileUpdates", 1),
                ],
                },
        crate::tweak! {
                id: "disable_disk_health_updates",
                category: "online_data",
                name: "Disable Disk Health Model Updates",
                description: "Prevents downloading updates to the Disk Failure Prediction Model.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth", "AllowDiskHealthModelUpdates", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_cloud_content",
                category: "online_data",
                name: "Disable Cloud Content",
                description: "Disables cloud-optimized content and Windows consumer features (ads).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableCloudOptimizedContent", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableConsumerAccountStateContent", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableSoftLanding", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableWindowsConsumerFeatures", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                    id: "disable_speech_updates",
                    category: "online_data",
                    name: "Disable Speech Model Updates",
                    description: "Prevents automatic download of new speech recognition models.",
                    effect: TweakEffect::Restart,
                    enabled_ops: &[
                            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate", 0),
                    ],
                                        requires_restart: true

        },
        crate::tweak! {
            id: "disable_map_updates",
            category: "online_data",
            name: "Disable Offline Map Updates",
            description: "Disables automatic updates for offline maps to save bandwidth.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\Maps", "AutoUpdateEnabled", 0, 1),
            ],
                    },
];
