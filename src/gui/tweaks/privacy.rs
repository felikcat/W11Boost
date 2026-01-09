// Privacy & Telemetry tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static PRIVACY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_telemetry",
                category: "privacy",
                name: "Disable Telemetry",
                description: "Disables Windows telemetry, diagnostic data collection, and OneSettings downloads.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "AllowTelemetry",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDiagnosticLogCollection",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "DisableOneSettingsDownloads",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                                value_name: "AllowTelemetry",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(3)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice",
                                value_name: "EnableDeviceHealthAttestationService",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                                value_name: "MiscPolicyInfo",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "AllowTelemetry",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDiagnosticLogCollection",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "DisableOneSettingsDownloads",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                                value_name: "AllowTelemetry",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice",
                                value_name: "EnableDeviceHealthAttestationService",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                                value_name: "MiscPolicyInfo",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_sensor_permission_override",
                category: "privacy",
                name: "Disable Sensor Permission Override",
                description: "Disables the sensor permission state override.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}",
                        value_name: "SensorPermissionState",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}",
                        value_name: "SensorPermissionState",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_cortana",
                category: "privacy",
                name: "Disable Cortana",
                description: "Disables Cortana assistant completely via Group Policy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortana",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortana",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_background_apps",
                category: "privacy",
                name: "Disable Background Apps",
                description: "Prevents UWP apps from running in the background, improving privacy and performance.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
                                value_name: "GlobalUserDisabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
                                value_name: "GlobalUserDisabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy",
                                value_name: "LetAppsRunInBackground",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
                                value_name: "GlobalUserDisabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
                                value_name: "GlobalUserDisabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy",
                                value_name: "LetAppsRunInBackground",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        // restrict_app_capabilities removed in favor of individual tweaks
        crate::tweak! {
                id: "disable_copilot",
                category: "privacy",
                name: "Disable Windows Copilot",
                description: "Disables Windows Copilot AI assistant completely.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\WindowsCopilot",
                                value_name: "TurnOffWindowsCopilot",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
                                value_name: "TurnOffWindowsCopilot",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "HubsSidebarEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\WindowsCopilot",
                                value_name: "TurnOffWindowsCopilot",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
                                value_name: "TurnOffWindowsCopilot",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "HubsSidebarEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_activity_history",
                category: "privacy",
                name: "Disable Activity History/Timeline",
                description: "Disables Windows Timeline and activity history collection.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "EnableActivityFeed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "PublishUserActivities",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "UploadUserActivities",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "EnableActivityFeed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "PublishUserActivities",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "UploadUserActivities",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_suggested_content",
                category: "privacy",
                name: "Disable Suggested Content",
                description: "Disables suggested content, promotions, and suggestions in Windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353698Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338388Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338389Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338393Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353694Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353696Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SystemPaneSuggestionsEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338387Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(0)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SilentInstalledAppsEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SoftLandingEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "RotatingLockScreenEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "RotatingLockScreenOverlayEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowSyncProviderNotifications",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
        },
                        // Merged from v2
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "ContentDeliveryAllowed", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "OemPreInstalledAppsEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "PreInstalledAppsEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "PreInstalledAppsEverEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "SubscribedContentEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353698Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338388Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338389Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338393Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353694Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-353696Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SystemPaneSuggestionsEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SubscribedContent-338387Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SilentInstalledAppsEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "SoftLandingEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "RotatingLockScreenEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                                value_name: "RotatingLockScreenOverlayEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "ShowSyncProviderNotifications",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        // Merged from v2
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "ContentDeliveryAllowed", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "OemPreInstalledAppsEnabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "PreInstalledAppsEnabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "PreInstalledAppsEverEnabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", value_name: "SubscribedContentEnabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        // disable_scoobe moved to boot.rs
        crate::tweak! {
                id: "disable_feedback",
                category: "privacy",
                name: "Disable Feedback Requests",
                description: "Prevents Windows from asking for feedback.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Siuf\Rules",
                                value_name: "NumberOfSIUFInPeriod",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Siuf\Rules",
                                value_name: "PeriodInNanoSeconds",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "DoNotShowFeedbackNotifications",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Siuf\Rules",
                                value_name: "NumberOfSIUFInPeriod",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Siuf\Rules",
                                value_name: "PeriodInNanoSeconds",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                value_name: "DoNotShowFeedbackNotifications",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_error_reporting",
                category: "privacy",
                name: "Disable Error Reporting",
                description: "Disables Windows Error Reporting which sends crash data to Microsoft.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "Disabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "DontSendAdditionalData", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "DontShowUI", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "LoggingDisabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\pchealth\errorreporting", value_name: "DoReport", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "Disabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "DontSendAdditionalData", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "DontShowUI", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", value_name: "LoggingDisabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\pchealth\errorreporting", value_name: "DoReport", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                ])
        },
        crate::tweak! {
                id: "disable_live_tiles",
                category: "privacy",
                name: "Disable Live Tiles",
                description: "Disables live tile updates on Start menu, preventing background data fetching.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications",
                        value_name: "NoTileApplicationNotification",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications",
                        value_name: "NoTileApplicationNotification",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_telemetry_services",
                category: "privacy",
                name: "Disable Telemetry Services",
                description: "Disables DiagTrack, dmwappushservice, and diagnostics hub services.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| {
                        let services = ["DiagTrack", "dmwappushservice", "diagnosticshub.standardcollector.service"];
                        for service in services {
                                ctx.post_status(&format!("Disabling service: {}", service));
                                let _ = crate::run_system_command("sc.exe", &["config", service, "start=", "disabled"]);
                                let _ = crate::run_system_command("sc.exe", &["stop", service]);
                                ctx.report_progress();
                        }
                        Ok(())
                }),
                custom_revert: Some(|ctx| {
                        let services = ["DiagTrack", "dmwappushservice", "diagnosticshub.standardcollector.service"];
                        for service in services {
                                ctx.post_status(&format!("Enabling service: {}", service));
                                let _ = crate::run_system_command("sc.exe", &["config", service, "start=", "auto"]);
                                let _ = crate::run_system_command("sc.exe", &["start", service]);
                                ctx.report_progress();
                        }
                        Ok(())
                })
        },
        crate::tweak! {
                id: "disable_handwriting_sharing",
                category: "privacy",
                name: "Disable Handwriting Data Sharing",
                description: "Prevents handwriting data from being shared with Microsoft.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
                        value_name: "PreventHandwritingDataSharing",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
                        value_name: "PreventHandwritingDataSharing",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_handwriting_error_reports",
                category: "privacy",
                name: "Disable Handwriting Error Reports",
                description: "Prevents handwriting error reports from being sent to Microsoft.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports",
                        value_name: "PreventHandwritingErrorReports",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports",
                        value_name: "PreventHandwritingErrorReports",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_app_inventory",
                category: "privacy",
                name: "Disable Application Inventory",
                description: "Disables the collection of installed application inventory and telemetry.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInventory",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableUAR",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "AITEnable",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInventory",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableUAR",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "AITEnable",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_search_web_results",
                category: "privacy",
                name: "Disable Search Web Results",
                description: "Disables Bing search results and web integration in Windows Search.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "DisableWebSearch",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "ConnectedSearchUseWeb",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "AllowCloudSearch",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "EnableDynamicContentInWSB",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Windows Search",
                                value_name: "CortanaConsent",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "DisableWebSearch",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "ConnectedSearchUseWeb",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "AllowCloudSearch",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                                value_name: "EnableDynamicContentInWSB",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Windows Search",
                                value_name: "CortanaConsent",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_search_location",
                category: "privacy",
                name: "Disable Search Location Access",
                description: "Prevents Windows Search from using your location.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowSearchToUseLocation",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowSearchToUseLocation",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_cortana_lock_screen",
                category: "privacy",
                name: "Disable Cortana on Lock Screen",
                description: "Prevents Cortana from being used when the device is locked.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortanaAboveLock",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
                        value_name: "AllowCortanaAboveLock",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_advertising_id",
                category: "privacy",
                name: "Disable Advertising ID",
                description: "Disables the advertising ID for relevant ads.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
                                value_name: "DisabledByGroupPolicy",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
                                value_name: "DisabledByGroupPolicy",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_ceip",
                category: "privacy",
                name: "Disable Customer Experience Improvement Program",
                description: "Disables the Windows Customer Experience Improvement Program (CEIP).",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\SQMClient\Windows",
                        value_name: "CEIPEnable",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\SQMClient\Windows",
                        value_name: "CEIPEnable",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0) }]),
                requires_restart: true
        },
        // disable_message_sync moved to sync.rs
        crate::tweak! {
                id: "disable_biometrics",
                category: "privacy",
                name: "Disable Biometrics",
                description: "Disables biometric features like Windows Hello.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Biometrics",
                        value_name: "Enabled",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Biometrics",
                        value_name: "Enabled",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_clipboard_history",
                category: "privacy",
                name: "Disable Clipboard History",
                description: "Disables the collection of clipboard history.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "AllowClipboardHistory",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Clipboard",
                                value_name: "EnableClipboardHistory",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "AllowClipboardHistory",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Clipboard",
                                value_name: "EnableClipboardHistory",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ])
        },
        // disable_clipboard_sync removed (duplicate of sync.rs/disable_cross_device_clipboard)
        crate::tweak! {
                id: "disable_location_access",
                category: "privacy",
                name: "Disable Location Access",
                description: "Disables location services, sensors, and Windows Location Provider.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableLocation",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableWindowsLocationProvider",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableLocationScripting",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableSensors",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableLocation",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableWindowsLocationProvider",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableLocationScripting",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors",
                                value_name: "DisableSensors",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_geolocation_service",
                category: "privacy",
                name: "Disable Geolocation Service",
                description: "Disables the Geolocation service (lfsvc) to prevent location tracking.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\ControlSet001\Services\lfsvc\Service\Configuration",
                        value_name: "Status",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(1)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\ControlSet001\Services\lfsvc\Service\Configuration",
                        value_name: "Status",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Dword(1)
        }])
        },
        crate::tweak! {
                id: "disable_webcam_access",
                category: "privacy",
                name: "Disable Webcam Access",
                description: "Blocks apps from accessing the webcam.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_microphone_access",
                category: "privacy",
                name: "Disable Microphone Access",
                description: "Blocks apps from accessing the microphone.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_tailored_experiences",
                category: "privacy",
                name: "Disable Tailored Experiences",
                description: "Disables personalized experiences based on diagnostic data.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableTailoredExperiencesWithDiagnosticData",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                                value_name: "TailoredExperiencesWithDiagnosticDataEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                                value_name: "DisableTailoredExperiencesWithDiagnosticData",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_app_diagnostics_access",
                category: "privacy",
                name: "Disable App Diagnostics Access",
                description: "Blocks apps from accessing diagnostic information about other apps.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_account_info_access",
                category: "privacy",
                name: "Disable Account Info Access",
                description: "Blocks apps from accessing your name, picture, and other account info.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_contacts_access",
                category: "privacy",
                name: "Disable Contacts Access",
                description: "Blocks apps from accessing your contacts.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_calendar_access",
                category: "privacy",
                name: "Disable Calendar Access",
                description: "Blocks apps from accessing your calendar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_call_history_access",
                category: "privacy",
                name: "Disable Call History Access",
                description: "Blocks apps from accessing your call history.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_email_access",
                category: "privacy",
                name: "Disable Email Access",
                description: "Blocks apps from accessing and sending email.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_tasks_access",
                category: "privacy",
                name: "Disable Tasks Access",
                description: "Blocks apps from accessing your tasks.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_messaging_access",
                category: "privacy",
                name: "Disable Messaging Access",
                description: "Blocks apps from reading or sending messages (text or MMS).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_radios_access",
                category: "privacy",
                name: "Disable Radios Access",
                description: "Blocks apps from controlling device radios (like Bluetooth).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                id: "disable_documents_library_access",
                category: "privacy",
                name: "Disable Documents Library Access",
                description: "Blocks apps from accessing your documents library.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::String("Allow")
        },
                ])
        },
        crate::tweak! {
                                id: "disable_pictures_library_access",
                                category: "privacy",
                                name: "Disable Pictures Library Access",
                                description: "Blocks apps from accessing your pictures library.",
                                effect: TweakEffect::Immediate,
                                enabled_ops: &[
                                        RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                                                value_name: "Value",
                                                value: RegistryValue::String("Deny"),
                                                stock_value: RegistryValue::String("Allow")
        },
                                        RegistryOp {
                                                hkey: "HKCU",
                                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                                                value_name: "Value",
                                                value: RegistryValue::String("Deny"),
                                                stock_value: RegistryValue::String("Allow")
        },
                                ],
                                disabled_ops: Some(&[
                                        RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                                                value_name: "Value",
                                                value: RegistryValue::String("Allow"),
                                                stock_value: RegistryValue::String("Allow")
        },
                                        RegistryOp {
                                                hkey: "HKCU",
                                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary",
                                                value_name: "Value",
                                                value: RegistryValue::String("Allow"),
                                                stock_value: RegistryValue::String("Allow")
        },
                                ])
        },
        crate::tweak! {
                id: "disable_videos_library_access",
                category: "privacy",
                name: "Disable Videos Library Access",
                description: "Blocks apps from accessing your videos library.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Deny"),
                                stock_value: RegistryValue::String("Allow")
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary",
                                value_name: "Value",
                                value: RegistryValue::String("Allow"),
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_lock_screen_camera",
                category: "privacy",
                name: "Disable Lock Screen Camera",
                description: "Disables the camera button on the lock screen.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Personalization",
                        value_name: "NoLockScreenCamera",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Personalization",
                        value_name: "NoLockScreenCamera",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_text_input_personalization",
                category: "privacy",
                name: "Disable Text Input Personalization",
                description: "Disables collection of typing and handwriting data to improve recognition.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\input\TIPC",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization",
                                value_name: "RestrictImplicitInkCollection",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization",
                                value_name: "RestrictImplicitTextCollection",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization\TrainedDataStore",
                                value_name: "HarvestContacts",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                                value_name: "AllowInputPersonalization",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\input\TIPC",
                                value_name: "Enabled",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization",
                                value_name: "RestrictImplicitInkCollection",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization",
                                value_name: "RestrictImplicitTextCollection",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Dword(0) },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\InputPersonalization\TrainedDataStore",
                                value_name: "HarvestContacts",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Dword(1) },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\InputPersonalization",
                                value_name: "AllowInputPersonalization",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "disable_bluetooth_advertising",
                category: "privacy",
                name: "Disable Bluetooth Advertising",
                description: "Prevents the device from broadcasting Bluetooth advertising data.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Bluetooth",
                        value_name: "AllowAdvertising",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Bluetooth",
                        value_name: "AllowAdvertising",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_toast_notifications",
                category: "privacy",
                name: "Disable Toast Notifications",
                description: "Disables all toast (pop-up) notifications.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\PushNotifications",
                        value_name: "ToastEnabled",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\Windows\CurrentVersion\PushNotifications",
                        value_name: "ToastEnabled",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_language_list_access",
                category: "privacy",
                name: "Disable Website Language Access",
                description: "Prevents websites from accessing your language list.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\International\User Profile",
                        value_name: "HttpAcceptLanguageOptOut",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Control Panel\International\User Profile",
                        value_name: "HttpAcceptLanguageOptOut",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_text_prediction",
                category: "privacy",
                name: "Disable Text Prediction",
                description: "Disables text prediction and auto-correction features.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\TabletTip\1.7",
                        value_name: "EnableTextPrediction",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\TabletTip\1.7",
                        value_name: "EnableTextPrediction",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_speech_model_updates",
                category: "privacy",
                name: "Disable Speech Model Updates",
                description: "Prevents automatic downloads and updates of speech recognition models.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Speech_OneCore\Preferences",
                                value_name: "ModelDownloadAllowed",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Speech",
                                value_name: "AllowSpeechModelUpdate",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Speech_OneCore\Preferences",
                                value_name: "ModelDownloadAllowed",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Speech",
                                value_name: "AllowSpeechModelUpdate",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "disable_voice_activation",
                category: "privacy",
                name: "Disable Voice Activation",
                description: "Disables apps from listening for voice keywords (like 'Hey Cortana').",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationEnabled",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationOnLockScreenEnabled",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationLastUsed",
                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationEnabled",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationOnLockScreenEnabled",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps",
                                value_name: "AgentActivationLastUsed",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ])
        },
        crate::tweak! {
                id: "disable_media_player_tracking",
                category: "privacy",
                name: "Disable Media Player Tracking",
                description: "Disables usage tracking in Windows Media Player.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\MediaPlayer\Preferences",
                        value_name: "UsageTracking",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Microsoft\MediaPlayer\Preferences",
                        value_name: "UsageTracking",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                        id: "disable_app_archiving",
                                        category: "privacy",
                                        name: "Disable App Archiving",
                                        description: "Prevents Windows from automatically archiving apps you don't use often.",
                                        effect: TweakEffect::Immediate,
                                        enabled_ops: &[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                                                value_name: "AllowAutomaticAppArchiving",
                                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                                        disabled_ops: Some(&[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                                                value_name: "AllowAutomaticAppArchiving",
                                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                        id: "disable_appv_ceip",
                                        category: "privacy",
                                        name: "Disable App-V CEIP",
                                        description: "Disables Customer Experience Improvement Program for App-V.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\AppV\CEIP",
                                                value_name: "CEIPEnable",
                                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                                        disabled_ops: Some(&[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\AppV\CEIP",
                                                value_name: "CEIPEnable",
                                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                                        requires_restart: true
        },
        crate::tweak! {
                                        id: "disable_pchealth",
                                        category: "privacy",
                                        name: "Disable PC Health Reporting",
                                        description: "Disables PC Health error reporting and data collection.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "AllOrNone", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeKernelFaults", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeMicrosoftApps", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeShutdownErrs", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeWindowsApps", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "DoReport", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                                        ],
                                        disabled_ops: Some(&[
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "AllOrNone", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeKernelFaults", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeMicrosoftApps", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeShutdownErrs", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "IncludeWindowsApps", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", value_name: "DoReport", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                        ]),
                                        requires_restart: true
        },
        crate::tweak! {
                                        id: "disable_device_driver_reporting",
                                        category: "privacy",
                                        name: "Disable Device Driver Reporting",
                                        description: "Prevents sending device driver error reports to Windows Error Reporting.",
                                        effect: TweakEffect::Immediate,
                                        enabled_ops: &[
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", value_name: "DisableSendGenericDriverNotFoundToWER", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", value_name: "DisableSendRequestAdditionalSoftwareToWER", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                                        ],
                                        disabled_ops: Some(&[
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", value_name: "DisableSendGenericDriverNotFoundToWER", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", value_name: "DisableSendRequestAdditionalSoftwareToWER", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                                        ])
        },
        crate::tweak! {
                                        id: "disable_oobe_privacy",
                                        category: "privacy",
                                        name: "Disable OOBE Privacy Experience",
                                        description: "Disables the privacy settings experience during user account setup.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\OOBE",
                                                value_name: "DisablePrivacyExperience",
                                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                        disabled_ops: Some(&[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\OOBE",
                                                value_name: "DisablePrivacyExperience",
                                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                                        requires_restart: true
        },
        crate::tweak! {
                                        id: "disable_wtds",
                                        category: "privacy",
                                        name: "Disable Windows Tds Components",
                                        description: "Disables Windows Tds (Telemetry) components.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components",
                                                value_name: "ServiceEnabled",
                                                value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                                        disabled_ops: Some(&[RegistryOp {
                                                hkey: "HKLM",
                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components",
                                                value_name: "ServiceEnabled",
                                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                                        requires_restart: true
        },
        crate::tweak! {
                id: "disable_windows_spotlight",
                category: "privacy",
                name: "Disable Windows Spotlight",
                description: "Disables Windows Spotlight features on the lock screen.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableWindowsSpotlightFeatures",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableWindowsSpotlightFeatures",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_tailored_experiences_policy",
                category: "privacy",
                name: "Disable Tailored Experiences (Policy)",
                description: "Disables tailored experiences via Group Policy.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableTailoredExperiencesWithDiagnosticData",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableTailoredExperiencesWithDiagnosticData",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_third_party_suggestions",
                category: "privacy",
                name: "Disable Third-Party Suggestions",
                description: "Disables suggestions from third-party apps in Windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableThirdPartySuggestions",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                        value_name: "DisableThirdPartySuggestions",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_firefox_telemetry",
                category: "privacy",
                name: "Disable Firefox Telemetry",
                description: "Disables telemetry collection in Mozilla Firefox.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Mozilla\Firefox",
                                value_name: "DisableTelemetry",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Mozilla\Firefox",
                                value_name: "DisableTelemetry",
                                value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Mozilla\Firefox",
                                value_name: "DisableTelemetry",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Mozilla\Firefox",
                                value_name: "DisableTelemetry",
                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_office_telemetry",
                category: "privacy",
                name: "Disable Office Telemetry",
                description: "Disables telemetry, data collection, and feedback in Microsoft Office 2016+.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        // Privacy Policy
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\privacy", value_name: "SendTelemetry", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        // Common Policies
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common", value_name: "QMEnable", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common", value_name: "UpdateReliabilityData", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        // Office Telemetry Agent
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\osm", value_name: "EnableLogging", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\osm", value_name: "EnableUpload", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        // Common Settings
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\16.0\common", value_name: "QMEnable", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\16.0\common\clienttelemetry", value_name: "DisableTelemetry", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\common\clienttelemetry", value_name: "DisableTelemetry", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                ],
                disabled_ops: Some(&[
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\privacy", value_name: "SendTelemetry", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common", value_name: "QMEnable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common", value_name: "UpdateReliabilityData", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\osm", value_name: "EnableLogging", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\osm", value_name: "EnableUpload", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\16.0\common", value_name: "QMEnable", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\16.0\common\clienttelemetry", value_name: "DisableTelemetry", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                        RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\office\common\clienttelemetry", value_name: "DisableTelemetry", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_device_name_telemetry",
                category: "privacy",
                name: "Disable Device Name in Telemetry",
                description: "Prevents sending the device name in Windows telemetry data.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "AllowDeviceNameInTelemetry",
                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "AllowDeviceNameInTelemetry",
                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_telemetry_optin_notification",
                category: "privacy",
                name: "Disable Telemetry Opt-in Notification",
                description: "Disables the notification asking to opt-in to telemetry.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                        value_name: "DisableTelemetryOptInChangeNotification",
                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                                        disabled_ops: Some(&[RegistryOp {
                                                                hkey: "HKLM",
                                                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                                                                value_name: "DisableTelemetryOptInChangeNotification",
                                                                value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                id: "disable_appx_background_updates",
                                category: "privacy",
                                name: "Disable Appx Background Updates",
                                description: "Prevents Appx packages from updating in the background.",
                                effect: TweakEffect::Restart,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                                        value_name: "DisableBackgroundAutoUpdates",
                                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                                        value_name: "DisableBackgroundAutoUpdates",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                                requires_restart: true
        },
        crate::tweak! {
                                id: "disable_page_combining",
                                category: "privacy",
                                name: "Disable Memory Page Combining",
                                description: "Disables memory page combining which can be a security/privacy risk.",
                                effect: TweakEffect::Restart,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                                        value_name: "DisablePageCombining",
                                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                                        value_name: "DisablePageCombining",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }]),
                                requires_restart: true
        },
        crate::tweak! {
                                id: "disable_instrumentation",
                                category: "privacy",
                                name: "Disable Instrumentation",
                                description: "Disables system instrumentation feedback.",
                                effect: TweakEffect::Immediate,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKCU",
                                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                        value_name: "NoInstrumentation",
                                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKCU",
                                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                        value_name: "NoInstrumentation",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                id: "disable_chat_auto_install",
                                category: "privacy",
                                name: "Disable Chat Auto Install",
                                description: "Prevents automatic installation of the Chat app.",
                                effect: TweakEffect::Immediate,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Communications",
                                        value_name: "ConfigureChatAutoInstall",
                                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Communications",
                                        value_name: "ConfigureChatAutoInstall",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                id: "disable_diagtrack_toast",
                                category: "privacy",
                                name: "Disable DiagTrack Toasts",
                                description: "Disables diagnostic tracking toast notifications.",
                                effect: TweakEffect::Immediate,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack",
                                        value_name: "ShowedToastAtLevel",
                                        value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKLM",
                                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack",
                                        value_name: "ShowedToastAtLevel",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                                id: "disable_background_app_toggle",
                                category: "privacy",
                                name: "Disable Background App Toggle",
                                description: "Disables the global toggle for background applications.",
                                effect: TweakEffect::Logoff,
                                enabled_ops: &[RegistryOp {
                                        hkey: "HKCU",
                                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Search",
                                        value_name: "BackgroundAppGlobalToggle",
                                        value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete }],
                                disabled_ops: Some(&[RegistryOp {
                                        hkey: "HKCU",
                                        subkey: r"Software\Microsoft\Windows\CurrentVersion\Search",
                                        value_name: "BackgroundAppGlobalToggle",
                                        value: RegistryValue::Delete, stock_value: RegistryValue::Delete }])
        },
        crate::tweak! {
                id: "disable_oobe_privacy_experience",
                category: "privacy",
                name: "Disable OOBE Privacy Experience",
                description: "Disables the Privacy Settings Experience at sign-in.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\OOBE",
                                value_name: "DisablePrivacyExperience",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\OOBE",
                                value_name: "DisablePrivacyExperience",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\OOBE",
                                value_name: "DisablePrivacyExperience",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\OOBE",
                                value_name: "DisablePrivacyExperience",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_telemetry_tasks",
                category: "privacy",
                name: "Disable Telemetry Scheduled Tasks",
                description: "Disables various scheduled tasks related to telemetry and data collection (CEIP, Application Experience, etc).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| {
                        let tasks = [
                                r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
                                r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
                                r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
                                r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
                                r"\Microsoft\Windows\Autochk\Proxy",
                                r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
                                r"\Microsoft\Windows\Maintenance\WinSAT",
                        ];
                        for task in tasks {
                                ctx.post_status(&format!("Disabling task: {}", task));
                                let _ = crate::run_system_command("schtasks", &["/Change", "/TN", task, "/Disable"]);
                                ctx.report_progress();
                        }
                        Ok(())
                }),
                custom_revert: Some(|ctx| {
                        let tasks = [
                                r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
                                r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
                                r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
                                r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
                                r"\Microsoft\Windows\Autochk\Proxy",
                                r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
                                r"\Microsoft\Windows\Maintenance\WinSAT",
                        ];
                        for task in tasks {
                                ctx.post_status(&format!("Enabling task: {}", task));
                                let _ = crate::run_system_command("schtasks", &["/Change", "/TN", task, "/Enable"]);
                                ctx.report_progress();
                        }
                        Ok(())
                })
        },
        crate::tweak! {
                id: "disable_onedrive",
                category: "privacy",
                name: "Prevent OneDrive Usage",
                description: "Prevents OneDrive file sync and usage via Group Policy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
                        value_name: "DisableFileSyncNGSC",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\OneDrive",
                        value_name: "DisableFileSyncNGSC",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_game_bar",
                category: "privacy",
                name: "Disable Game Bar",
                description: "Disables the Xbox Game Bar recording and overlay features.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"System\GameConfigStore",
                                value_name: "GameDVR_Enabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "AppCaptureEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
                                value_name: "AllowGameDVR",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "HistoricalCaptureEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "AudioCaptureEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "CursorCaptureEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"System\GameConfigStore",
                                value_name: "GameDVR_Enabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "AppCaptureEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\GameDVR",
                                value_name: "AllowGameDVR",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "HistoricalCaptureEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "AudioCaptureEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\GameDVR",
                                value_name: "CursorCaptureEnabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_graphics_capture",
                category: "privacy",
                name: "Disable Graphics Capture",
                description: "Disables the Windows Graphics Capture API for added privacy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\GraphicsCapture",
                        value_name: "AllowGraphicsCapture",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete // Default is allowed
                }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\GraphicsCapture",
                        value_name: "AllowGraphicsCapture",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
                }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_app_launch_tracking",
                category: "privacy",
                name: "Disable App Launch Tracking",
                description: "Prevents Windows from tracking app launches to improve Start menu and search results.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_paint_cocreator",
                category: "privacy",
                name: "Disable Cocreator in Paint",
                description: "Disables the AI-powered Cocreator feature in Microsoft Paint.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableCocreator",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableCocreator",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_paint_generative_fill",
                category: "privacy",
                name: "Disable Generative Fill in Paint",
                description: "Disables the AI-powered Generative Fill feature in Microsoft Paint.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableGenerativeFill",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableGenerativeFill",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_paint_image_creator",
                category: "privacy",
                name: "Disable Image Creator in Paint",
                description: "Disables the AI-powered Image Creator (DALL-E) feature in Microsoft Paint.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableImageCreator",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint",
                                value_name: "DisableImageCreator",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_lock_screen_widgets",
                category: "privacy",
                name: "Disable Lock Screen Widgets",
                description: "Disables widgets like Weather on the Lock Screen.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Lock Screen",
                                value_name: "LockScreenWidgetsEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Lock Screen",
                                value_name: "LockScreenWidgetsEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
            id: "disable_http_accept_language",
            category: "privacy",
            name: "Opt Out of HTTP Accept-Language",
            description: "Disables sending the Accept-Language header in HTTP requests for privacy.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\International\User Profile",
                    value_name: "HttpAcceptLanguageOptOut",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Control Panel\International\User Profile",
                    value_name: "HttpAcceptLanguageOptOut",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "disable_input_personalization",
            category: "privacy",
            name: "Disable Input Personalization",
            description: "Disables collection of ink and typing data for personalization.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\input",
                    value_name: "IsInputAppPreloadEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\input\Settings",
                    value_name: "InsightsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\InputPersonalization",
                    value_name: "RestrictImplicitTextCollection",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\InputPersonalization",
                    value_name: "RestrictImplicitInkCollection",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\input",
                    value_name: "IsInputAppPreloadEnabled",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\input\Settings",
                    value_name: "InsightsEnabled",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\InputPersonalization",
                    value_name: "RestrictImplicitTextCollection",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\InputPersonalization",
                    value_name: "RestrictImplicitInkCollection",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(0),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_activity_access",
            category: "privacy",
            name: "Disable Activity Access",
            description: "Blocks apps from accessing your activity history.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow")
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow")
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow")
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow")
                },
            ]),
        },
        crate::tweak! {
            id: "disable_phone_call_access",
            category: "privacy",
            name: "Disable Phone Call Access",
            description: "Blocks apps from making phone calls.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow")
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall", // Assuming HKCU mirror exists for consistency, though original only had HKLM
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow")
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow")
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow")
                },
            ]),
        },
        crate::tweak! {
            id: "disable_notification_access",
            category: "privacy",
            name: "Disable Notification Access",
            description: "Blocks apps from accessing your notifications.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow")
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow")
                },
            ]),
        },
        crate::tweak! {
            id: "disable_content_delivery_manager",
            category: "privacy",
            name: "Disable Content Delivery Manager (Ads/Suggestions)",
            description: "Disables Windows 'Content Delivery Manager' used for ads, suggestions, and tips in Start, Lock Screen, and Settings.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SystemPaneSuggestionsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SoftLandingEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SilentInstalledAppsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "RotatingLockScreenEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SubscribedContent-338387Enabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SilentInstalledAppsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "DisableWindowsSpotlightFeatures",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "RotatingLockScreenEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "PreInstalledAppsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "OemPreInstalledAppsEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "ContentDeliveryAllowed",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "DisableTailoredExperiencesWithDiagnosticData",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(0),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SubscribedContentEnabled",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
                // Policy
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                    value_name: "DisableWindowsSpotlightFeatures",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\CloudContent",
                    value_name: "DisableWindowsSpotlightFeatures",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "SystemPaneSuggestionsEnabled",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager",
                    value_name: "DisableWindowsSpotlightFeatures",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(0),
                },
                // Re-enable others as needed, simplified for brevity here (user can reset defaults)
            ]),
        },
        crate::tweak! {
            id: "disable_explorer_ads_policy",
            category: "privacy",
            name: "Disable Explorer Ads (Policy)",
            description: "Disables notifications and search box suggestions in File Explorer via Policy.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableNotificationCenter",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableSearchBoxSuggestions",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableNotificationCenter",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                    value_name: "DisableSearchBoxSuggestions",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "disable_windows_copilot_policy",
            category: "privacy",
            name: "Disable Windows Copilot (Policy)",
            description: "Disables Windows Copilot via Group Policy.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\windowscopilot",
                    value_name: "TurnOffWindowsCopilot",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete,
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\windowscopilot",
                    value_name: "TurnOffWindowsCopilot",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete,
                },
            ]),
        },
        crate::tweak! {
            id: "disable_music_library_access",
            category: "privacy",
            name: "Disable Music Library Access",
            description: "Blocks apps from accessing your music library.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_downloads_folder_access",
            category: "privacy",
            name: "Disable Downloads Folder Access",
            description: "Blocks apps from accessing your downloads folder.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_broad_filesystem_access",
            category: "privacy",
            name: "Disable Broad File System Access",
            description: "Blocks apps from accessing your file system broadly.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_gaze_input",
            category: "privacy",
            name: "Disable Eye Tracker (Gaze Input)",
            description: "Blocks apps from using eye tracking devices.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_cellular_data_access",
            category: "privacy",
            name: "Disable Cellular Data Access",
            description: "Blocks apps from using cellular data.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_bluetooth_sync_access",
            category: "privacy",
            name: "Disable Bluetooth Sync Access",
            description: "Blocks apps from syncing with Bluetooth devices.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                    value_name: "Value",
                    value: RegistryValue::String("Deny"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync",
                    value_name: "Value",
                    value: RegistryValue::String("Allow"),
                    stock_value: RegistryValue::String("Allow"),
                },
            ]),
        },
        crate::tweak! {
            id: "disable_input_insights",
            category: "privacy",
            name: "Disable Input Insights",
            description: "Disables 'Insights' features for typing and input.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\input\Settings", value_name: "InsightsEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\input\Settings", value_name: "InsightsEnabled", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
            ]),
        },
        crate::tweak! {
            id: "restrict_implicit_ink_text",
            category: "privacy",
            name: "Restrict Implicit Ink & Text Collection",
            description: "Prevents Windows from implicitly collecting inking and typing data.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\InputPersonalization", value_name: "RestrictImplicitTextCollection", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\InputPersonalization", value_name: "RestrictImplicitInkCollection", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\InputPersonalization", value_name: "RestrictImplicitTextCollection", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\InputPersonalization", value_name: "RestrictImplicitInkCollection", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
            ]),
        },
        crate::tweak! {
            id: "disable_media_player_telemetry",
            category: "privacy",
            name: "Disable Media Player Telemetry",
            description: "Disables usage tracking in the legacy Windows Media Player.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\MediaPlayer\Preferences", value_name: "UsageTracking", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(1) },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\MediaPlayer\Preferences", value_name: "UsageTracking", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(1) },
            ]),
        },
        crate::tweak! {
            id: "disable_vs_telemetry",
            category: "privacy",
            name: "Disable Visual Studio Telemetry",
            description: "Disables telemetry in Visual Studio.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\VisualStudio\Telemetry", value_name: "TurnOffSwitch", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\VisualStudio\Telemetry", value_name: "TurnOffSwitch", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_office_feedback",
            category: "privacy",
            name: "Disable Office Feedback & Surveys",
            description: "Disables feedback surveys and other feedback mechanisms in Microsoft Office.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "Enabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "SurveyEnabled", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "IncludeEmail", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "Enabled", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "SurveyEnabled", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\office\16.0\common\feedback", value_name: "IncludeEmail", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_copilot_policy",
            category: "privacy",
            name: "Disable Windows Copilot (Policy)",
            description: "Disables Windows Copilot features via Group Policy registry key.",
            effect: TweakEffect::Logoff,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\Windows\windowscopilot", value_name: "TurnOffWindowsCopilot", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Policies\Microsoft\Windows\windowscopilot", value_name: "TurnOffWindowsCopilot", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_siuf",
            category: "privacy",
            name: "Disable System Initiated User Feedback",
            description: "Prevents Windows from asking for feedback via popups.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Siuf\rules", value_name: "NumberOfSIUFInPeriod", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\Siuf\rules", value_name: "NumberOfSIUFInPeriod", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKCU", subkey: r"Software\Microsoft\Siuf\rules", value_name: "NumberOfSIUFInPeriod", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\Siuf\rules", value_name: "NumberOfSIUFInPeriod", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_push_notifications",
            category: "privacy",
            name: "Disable Cloud Push Notifications",
            description: "Disables cloud-based push notifications from installed applications.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", value_name: "NoCloudApplicationNotification", value: RegistryValue::Dword(1), stock_value: RegistryValue::Dword(0) },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", value_name: "NoCloudApplicationNotification", value: RegistryValue::Dword(0), stock_value: RegistryValue::Dword(0) },
            ]),
        },
        crate::tweak! {
            id: "disable_background_apps_policy",
            category: "privacy",
            name: "Disable Background Apps",
            description: "Prevents apps from running in the background to save resources and improve privacy.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsRunInBackground", value: RegistryValue::Dword(2), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsActivateWithVoice", value: RegistryValue::Dword(2), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsSyncWithDevices", value: RegistryValue::Dword(2), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsRunInBackground", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsActivateWithVoice", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\appprivacy", value_name: "LetAppsSyncWithDevices", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_location_services",
            category: "privacy",
            name: "Disable Location Services",
            description: "Disables Windows Location Services and prevents apps from accessing location.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableLocation", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableSensors", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableWindowsLocationProvider", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableLocationScripting", value: RegistryValue::Dword(1), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableLocation", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableSensors", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableWindowsLocationProvider", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", value_name: "DisableLocationScripting", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_device_health_attestation",
            category: "privacy",
            name: "Disable Device Health Attestation",
            description: "Disables the Device Health Attestation service.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice", value_name: "EnableDeviceHealthAttestationService", value: RegistryValue::Dword(0), stock_value: RegistryValue::Delete },
            ],
            disabled_ops: Some(&[
                RegistryOp { hkey: "HKLM", subkey: r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice", value_name: "EnableDeviceHealthAttestationService", value: RegistryValue::Delete, stock_value: RegistryValue::Delete },
            ]),
        },
        crate::tweak! {
            id: "disable_recall",
            category: "privacy",
            name: "Disable Windows Recall",
            description: "Disables Windows Recall AI feature which captures screenshots and analyzes activity.",
            effect: TweakEffect::Restart,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                value_name: "AllowRecallEnablement",
                value: RegistryValue::Dword(0),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                value_name: "AllowRecallEnablement",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_ai_data_analysis",
            category: "privacy",
            name: "Disable AI Data Analysis",
            description: "Disables AI-powered data analysis features.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAIDataAnalysis",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAIDataAnalysis",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Delete
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAIDataAnalysis",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Policies\Microsoft\Windows\WindowsAI",
                    value_name: "DisableAIDataAnalysis",
                    value: RegistryValue::Delete,
                    stock_value: RegistryValue::Delete
                },
            ]),
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_npsm_svc",
            category: "privacy",
            name: "Disable Now Playing Session Manager",
            description: "Disables the Now Playing Session Manager service (NPSMsvc), which tracks media sessions.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SYSTEM\CurrentControlSet\Services\NPSMSvc",
                    value_name: "Start",
                    value: RegistryValue::Dword(4),
                    stock_value: RegistryValue::Dword(3),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKLM",
                    subkey: r"SYSTEM\CurrentControlSet\Services\NPSMSvc",
                    value_name: "Start",
                    value: RegistryValue::Dword(3),
                    stock_value: RegistryValue::Dword(3),
                },
            ]),
            requires_restart: true,
        },
];
