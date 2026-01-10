// Privacy & Telemetry tweaks

use super::{Tweak, TweakEffect};

pub static PRIVACY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_telemetry",
                category: "privacy",
                name: "Disable Telemetry",
                description: "Disables Windows telemetry, diagnostic data collection, and OneSettings downloads.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "LimitDiagnosticLogCollection", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableOneSettingsDownloads", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry", 0, RegistryValue::Dword(3)),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice", "EnableDeviceHealthAttestationService", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager", "MiscPolicyInfo", 2, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_sensor_permission_override",
        category: "privacy",
        name: "Disable Sensor Permission Override",
        description: "Disables the sensor permission state override.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}", "SensorPermissionState", 0, RegistryValue::Dword(1)),
        ],
        },
        crate::tweak! {
                id: "disable_activity_history",
                category: "privacy",
                name: "Disable Activity History/Timeline",
                description: "Disables Windows Timeline and activity history collection.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_suggested_content",
        category: "privacy",
        name: "Disable Suggested Content",
        description: "Disables suggested content, promotions, and suggestions in Windows.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353698Enabled", 0, RegistryValue::Dword(0)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338388Enabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338389Enabled", 0, RegistryValue::Dword(0)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338393Enabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353694Enabled", 0, RegistryValue::Dword(0)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353696Enabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338387Enabled", 0, RegistryValue::Dword(0)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SilentInstalledAppsEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenOverlayEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSyncProviderNotifications", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Privacy", "TailoredExperiencesWithDiagnosticDataEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "OemPreInstalledAppsEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEverEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContentEnabled", 0, RegistryValue::Dword(1)),
        ],
        },
        crate::tweak! {
        id: "disable_feedback",
        category: "privacy",
        name: "Disable Feedback Requests",
        description: "Prevents Windows from asking for feedback.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Siuf\Rules", "PeriodInNanoSeconds", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DoNotShowFeedbackNotifications", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_live_tiles",
        category: "privacy",
        name: "Disable Live Tiles",
        description: "Disables live tile updates on Start menu, preventing background data fetching.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications", "NoTileApplicationNotification", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_telemetry_services",
        category: "privacy",
        name: "Disable Telemetry Services",
        description: "Disables DiagTrack, dmwappushservice, and diagnostics hub services.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[],
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
        },
        crate::tweak! {
        id: "disable_handwriting_sharing",
        category: "privacy",
        name: "Disable Handwriting Data Sharing",
        description: "Prevents handwriting data from being shared with Microsoft.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\TabletPC", "PreventHandwritingDataSharing", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_handwriting_error_reports",
        category: "privacy",
        name: "Disable Handwriting Error Reports",
        description: "Prevents handwriting error reports from being sent to Microsoft.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports", "PreventHandwritingErrorReports", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                id: "disable_app_inventory",
                category: "privacy",
                name: "Disable Application Inventory",
                description: "Disables the collection of installed application inventory and telemetry.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInventory", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_search_web_results",
                category: "privacy",
                name: "Disable Search Web Results",
                description: "Disables Bing search results and web integration in Windows Search.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCloudSearch", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "EnableDynamicContentInWSB", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Windows Search", "CortanaConsent", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_search_location",
        category: "privacy",
        name: "Disable Search Location Access",
        description: "Prevents Windows Search from using your location.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowSearchToUseLocation", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_advertising_id",
        category: "privacy",
        name: "Disable Advertising ID",
        description: "Disables the advertising ID for relevant ads.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0, 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", "DisabledByGroupPolicy", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                id: "disable_ceip",
                category: "privacy",
                name: "Disable Customer Experience Improvement Program",
                description: "Disables the Windows Customer Experience Improvement Program (CEIP).",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0, RegistryValue::Dword(0)),
                ],
                                requires_restart: true
        },
        // disable_message_sync moved to sync.rs
        crate::tweak! {
                id: "disable_biometrics",
                category: "privacy",
                name: "Disable Biometrics",
                description: "Disables biometric features like Windows Hello.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Biometrics", "Enabled", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_clipboard_history",
        category: "privacy",
        name: "Disable Clipboard History",
        description: "Disables the collection of clipboard history.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowClipboardHistory", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Clipboard", "EnableClipboardHistory", 0, RegistryValue::Delete),
        ],
        },
        // disable_clipboard_sync removed (duplicate of sync.rs/disable_cross_device_clipboard)
        crate::tweak! {
        id: "disable_location_access",
        category: "privacy",
        name: "Disable Location Access",
        description: "Disables location services, sensors, and Windows Location Provider.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableLocation", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableWindowsLocationProvider", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableLocationScripting", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors", "DisableSensors", 1, RegistryValue::Delete),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_geolocation_service",
        category: "privacy",
        name: "Disable Geolocation Service",
        description: "Disables the Geolocation service (lfsvc) to prevent location tracking.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\lfsvc\Service\Configuration", "Status", 0, RegistryValue::Dword(1)),
        ],
        },
        crate::tweak! {
        id: "disable_webcam_access",
        category: "privacy",
        name: "Disable Webcam Access",
        description: "Blocks apps from accessing the webcam.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_microphone_access",
        category: "privacy",
        name: "Disable Microphone Access",
        description: "Blocks apps from accessing the microphone.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_tailored_experiences",
        category: "privacy",
        name: "Disable Tailored Experiences",
        description: "Disables personalized experiences based on diagnostic data.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy", "TailoredExperiencesWithDiagnosticDataEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Privacy", "TailoredExperiencesWithDiagnosticDataEnabled", 0, RegistryValue::Dword(1)),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_app_diagnostics_access",
        category: "privacy",
        name: "Disable App Diagnostics Access",
        description: "Blocks apps from accessing diagnostic information about other apps.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_account_info_access",
        category: "privacy",
        name: "Disable Account Info Access",
        description: "Blocks apps from accessing your name, picture, and other account info.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_contacts_access",
        category: "privacy",
        name: "Disable Contacts Access",
        description: "Blocks apps from accessing your contacts.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_calendar_access",
        category: "privacy",
        name: "Disable Calendar Access",
        description: "Blocks apps from accessing your calendar.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appointments", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_call_history_access",
        category: "privacy",
        name: "Disable Call History Access",
        description: "Blocks apps from accessing your call history.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_email_access",
        category: "privacy",
        name: "Disable Email Access",
        description: "Blocks apps from accessing and sending email.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_tasks_access",
        category: "privacy",
        name: "Disable Tasks Access",
        description: "Blocks apps from accessing your tasks.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_messaging_access",
        category: "privacy",
        name: "Disable Messaging Access",
        description: "Blocks apps from reading or sending messages (text or MMS).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_radios_access",
        category: "privacy",
        name: "Disable Radios Access",
        description: "Blocks apps from controlling device radios (like Bluetooth).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_documents_library_access",
        category: "privacy",
        name: "Disable Documents Library Access",
        description: "Blocks apps from accessing your documents library.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
                        id: "disable_pictures_library_access",
                        category: "privacy",
                        name: "Disable Pictures Library Access",
                        description: "Blocks apps from accessing your pictures library.",
                        effect: TweakEffect::Immediate,
                        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_videos_library_access",
        category: "privacy",
        name: "Disable Videos Library Access",
        description: "Blocks apps from accessing your videos library.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary", "Value", "Deny", RegistryValue::String("Allow")),
        ],
        },
        crate::tweak! {
        id: "disable_lock_screen_camera",
        category: "privacy",
        name: "Disable Lock Screen Camera",
        description: "Disables the camera button on the lock screen.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Personalization", "NoLockScreenCamera", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_text_input_personalization",
        category: "privacy",
        name: "Disable Text Input Personalization",
        description: "Disables collection of typing and handwriting data to improve recognition.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\input\TIPC", "Enabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization\TrainedDataStore", "HarvestContacts", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\InputPersonalization", "AllowInputPersonalization", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_bluetooth_advertising",
        category: "privacy",
        name: "Disable Bluetooth Advertising",
        description: "Prevents the device from broadcasting Bluetooth advertising data.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Bluetooth", "AllowAdvertising", 0, RegistryValue::Delete)
        ],
        },
        crate::tweak! {
        id: "disable_toast_notifications",
        category: "privacy",
        name: "Disable Toast Notifications",
        description: "Disables all toast (pop-up) notifications.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", "ToastEnabled", 0, RegistryValue::Delete)
        ],
        },
        crate::tweak! {
        id: "disable_language_list_access",
        category: "privacy",
        name: "Disable Website Language Access",
        description: "Prevents websites from accessing your language list.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut", 1, RegistryValue::Delete)
        ],
        },
        crate::tweak! {
        id: "disable_text_prediction",
        category: "privacy",
        name: "Disable Text Prediction",
        description: "Disables text prediction and auto-correction features.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\TabletTip\1.7", "EnableTextPrediction", 0, RegistryValue::Delete)
        ],
        },
        crate::tweak! {
        id: "disable_speech_model_updates",
        category: "privacy",
        name: "Disable Speech Model Updates",
        description: "Prevents automatic downloads and updates of speech recognition models.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Speech_OneCore\Preferences", "ModelDownloadAllowed", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_voice_activation",
        category: "privacy",
        name: "Disable Voice Activation",
        description: "Disables apps from listening for voice keywords (like 'Hey Cortana').",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationOnLockScreenEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationLastUsed", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_media_player_tracking",
        category: "privacy",
        name: "Disable Media Player Tracking",
        description: "Disables usage tracking in Windows Media Player.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\MediaPlayer\Preferences", "UsageTracking", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_app_archiving",
        category: "privacy",
        name: "Disable App Archiving",
        description: "Prevents Windows from automatically archiving apps you don't use often.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving", 0, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                                        id: "disable_appv_ceip",
                                        category: "privacy",
                                        name: "Disable App-V CEIP",
                                        description: "Disables Customer Experience Improvement Program for App-V.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\AppV\CEIP", "CEIPEnable", 0, RegistryValue::Delete),
                                        ],
                                                                                requires_restart: true
        },
        crate::tweak! {
                                        id: "disable_pchealth",
                                        category: "privacy",
                                        name: "Disable PC Health Reporting",
                                        description: "Disables PC Health error reporting and data collection.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[

                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "AllOrNone", 1, RegistryValue::Delete),
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeKernelFaults", 0, RegistryValue::Delete),
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeMicrosoftApps", 0, RegistryValue::Delete),
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeShutdownErrs", 0, RegistryValue::Delete),
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeWindowsApps", 0, RegistryValue::Delete),
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "DoReport", 0, RegistryValue::Delete),
                                        ],
                                                                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_device_driver_reporting",
        category: "privacy",
        name: "Disable Device Driver Reporting",
        description: "Prevents sending device driver error reports to Windows Error Reporting.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[

                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendGenericDriverNotFoundToWER", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendRequestAdditionalSoftwareToWER", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                                        id: "disable_oobe_privacy",
                                        category: "privacy",
                                        name: "Disable OOBE Privacy Experience",
                                        description: "Disables the privacy settings experience during user account setup.",
                                        effect: TweakEffect::Restart,
                                        enabled_ops: &[
                                                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1, RegistryValue::Delete),
                                        ],
                                                                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_windows_spotlight",
        category: "privacy",
        name: "Disable Windows Spotlight",
        description: "Disables Windows Spotlight features on the lock screen.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_tailored_experiences_policy",
        category: "privacy",
        name: "Disable Tailored Experiences (Policy)",
        description: "Disables tailored experiences via Group Policy.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
        id: "disable_third_party_suggestions",
        category: "privacy",
        name: "Disable Third-Party Suggestions",
        description: "Disables suggestions from third-party apps in Windows.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableThirdPartySuggestions", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                id: "disable_firefox_telemetry",
                category: "privacy",
                name: "Disable Firefox Telemetry",
                description: "Disables telemetry collection in Mozilla Firefox.",
                effect: TweakEffect::Restart,
                enabled_ops: &[

                        crate::reg_dword!("HKLM", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", 1, RegistryValue::Delete),
                ],
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
                        // Privacy Policy
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\privacy", "SendTelemetry", 0, RegistryValue::Delete),
                        // Common Policies
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common", "QMEnable", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common", "UpdateReliabilityData", 0, RegistryValue::Delete),
                        // Office Telemetry Agent
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\osm", "EnableLogging", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\osm", "EnableUpload", 0, RegistryValue::Delete),
                        // Common Settings
                        crate::reg_dword!("HKCU", r"Software\Microsoft\office\16.0\common", "QMEnable", 0, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\office\common\clienttelemetry", "DisableTelemetry", 1, RegistryValue::Delete),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\office\common\clienttelemetry", "DisableTelemetry", 1, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_device_name_telemetry",
                category: "privacy",
                name: "Disable Device Name in Telemetry",
                description: "Prevents sending the device name in Windows telemetry data.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_telemetry_optin_notification",
        category: "privacy",
        name: "Disable Telemetry Opt-in Notification",
        description: "Disables the notification asking to opt-in to telemetry.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableTelemetryOptInChangeNotification", 1, RegistryValue::Delete),
        ],
        },
        crate::tweak! {
                                id: "disable_appx_background_updates",
                                category: "privacy",
                                name: "Disable Appx Background Updates",
                                description: "Prevents Appx packages from updating in the background.",
                                effect: TweakEffect::Restart,
                                enabled_ops: &[
                                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "DisableBackgroundAutoUpdates", 1, RegistryValue::Delete),
                                ],
                                                                requires_restart: true
        },
        crate::tweak! {
                                id: "disable_page_combining",
                                category: "privacy",
                                name: "Disable Memory Page Combining",
                                description: "Disables memory page combining which can be a security/privacy risk.",
                                effect: TweakEffect::Restart,
                                enabled_ops: &[
                                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePageCombining", 1, RegistryValue::Delete),
                                ],
                                                                requires_restart: true
        },
        crate::tweak! {
                        id: "disable_instrumentation",
                        category: "privacy",
                        name: "Disable Instrumentation",
                        description: "Disables system instrumentation feedback.",
                        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoInstrumentation", 1),
        ],
        },
        crate::tweak! {
                        id: "disable_chat_auto_install",
                        category: "privacy",
                        name: "Disable Chat Auto Install",
                        description: "Prevents automatic installation of the Chat app.",
                        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Communications", "ConfigureChatAutoInstall", 0),
        ],
        },
        crate::tweak! {
                        id: "disable_diagtrack_toast",
                        category: "privacy",
                        name: "Disable DiagTrack Toasts",
                        description: "Disables diagnostic tracking toast notifications.",
                        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack", "ShowedToastAtLevel", 1),
        ],
        },
        crate::tweak! {
                id: "disable_oobe_privacy_experience",
                category: "privacy",
                name: "Disable OOBE Privacy Experience",
                description: "Disables the Privacy Settings Experience at sign-in.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_telemetry_tasks",
        category: "privacy",
        name: "Disable Telemetry Scheduled Tasks",
        description: "Disables various scheduled tasks related to telemetry and data collection (CEIP, Application Experience, etc).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[],
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
        },
        crate::tweak! {
                id: "disable_onedrive",
                category: "privacy",
                name: "Prevent OneDrive Usage",
                description: "Prevents OneDrive file sync and usage via Group Policy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OneDrive", "DisableFileSyncNGSC", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_game_bar",
                category: "privacy",
                name: "Disable Game Bar",
                description: "Disables the Xbox Game Bar recording and overlay features.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"System\GameConfigStore", "GameDVR_Enabled", 0, 1),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 0, 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GameDVR", "AllowGameDVR", 0),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "HistoricalCaptureEnabled", 0, 1),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AudioCaptureEnabled", 0, 1),
                        crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "CursorCaptureEnabled", 0, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_graphics_capture",
                category: "privacy",
                name: "Disable Graphics Capture",
                description: "Disables the Windows Graphics Capture API for added privacy.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GraphicsCapture", "AllowGraphicsCapture", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_app_launch_tracking",
        category: "privacy",
        name: "Disable App Launch Tracking",
        description: "Prevents Windows from tracking app launches to improve Start menu and search results.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
        ],
        },
        crate::tweak! {
        id: "disable_lock_screen_widgets",
        category: "privacy",
        name: "Disable Lock Screen Widgets",
        description: "Disables widgets like Weather on the Lock Screen.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Lock Screen", "LockScreenWidgetsEnabled", 0),
        ],
        },
        crate::tweak! {
        id: "disable_http_accept_language",
        category: "privacy",
        name: "Opt Out of HTTP Accept-Language",
        description: "Disables sending the Accept-Language header in HTTP requests for privacy.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut", 1),
        ],
                },
        crate::tweak! {
        id: "disable_input_personalization",
        category: "privacy",
        name: "Disable Input Personalization",
        description: "Disables collection of ink and typing data for personalization.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\input", "IsInputAppPreloadEnabled", 0, 1),
            crate::reg_dword!("HKCU", r"Software\Microsoft\input\Settings", "InsightsEnabled", 0, 1),
            crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 1, 0),
            crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 1, 0),
        ],
                },
        crate::tweak! {
        id: "disable_activity_access",
        category: "privacy",
        name: "Disable Activity Access",
        description: "Blocks apps from accessing your activity history.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_phone_call_access",
        category: "privacy",
        name: "Disable Phone Call Access",
        description: "Blocks apps from making phone calls.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_notification_access",
        category: "privacy",
        name: "Disable Notification Access",
        description: "Blocks apps from accessing your notifications.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
            id: "disable_content_delivery_manager",
            category: "privacy",
            name: "Disable Content Delivery Manager (Ads/Suggestions)",
            description: "Disables Windows 'Content Delivery Manager' used for ads, suggestions, and tips in Start, Lock Screen, and Settings.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SilentInstalledAppsEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338387Enabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "DisableWindowsSpotlightFeatures", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "OemPreInstalledAppsEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "DisableTailoredExperiencesWithDiagnosticData", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContentEnabled", 0, 1),
                // Policy
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures", 1),
                // Note: Some redundant keys from original retained but simplified if possible,
                // but list was long so I'm condensing the exact list provided in the viewed file.
                // The original had duplicate SilentInstalledAppsEnabled and RotatingLockScreenEnabled which I will deduplicate.
            ],

        },
        crate::tweak! {
        id: "disable_explorer_ads_policy",
        category: "privacy",
        name: "Disable Explorer Ads (Policy)",
        description: "Disables notifications and search box suggestions in File Explorer via Policy.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableNotificationCenter", 1, RegistryValue::Delete),
            crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1, RegistryValue::Delete),
        ],
                },
        crate::tweak! {
        id: "disable_music_library_access",
        category: "privacy",
        name: "Disable Music Library Access",
        description: "Blocks apps from accessing your music library.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_downloads_folder_access",
        category: "privacy",
        name: "Disable Downloads Folder Access",
        description: "Blocks apps from accessing your downloads folder.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_broad_filesystem_access",
        category: "privacy",
        name: "Disable Broad File System Access",
        description: "Blocks apps from accessing your file system broadly.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_gaze_input",
        category: "privacy",
        name: "Disable Eye Tracker (Gaze Input)",
        description: "Blocks apps from using eye tracking devices.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_cellular_data_access",
        category: "privacy",
        name: "Disable Cellular Data Access",
        description: "Blocks apps from using cellular data.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_bluetooth_sync_access",
        category: "privacy",
        name: "Disable Bluetooth Sync Access",
        description: "Blocks apps from syncing with Bluetooth devices.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync", "Value", "Deny", "Allow"),
            crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync", "Value", "Deny", "Allow"),
        ],
                },
        crate::tweak! {
        id: "disable_input_insights",
        category: "privacy",
        name: "Disable Input Insights",
        description: "Disables 'Insights' features for typing and input.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\input\Settings", "InsightsEnabled", 0, 1),
        ],
                },
        crate::tweak! {
        id: "restrict_implicit_ink_text",
        category: "privacy",
        name: "Restrict Implicit Ink & Text Collection",
        description: "Prevents Windows from implicitly collecting inking and typing data.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 1, 0),
            crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 1, 0),
        ],
                },
        crate::tweak! {
        id: "disable_media_player_telemetry",
        category: "privacy",
        name: "Disable Media Player Telemetry",
        description: "Disables usage tracking in the legacy Windows Media Player.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\MediaPlayer\Preferences", "UsageTracking", 0, 1),
        ],
                },
        crate::tweak! {
        id: "disable_vs_telemetry",
        category: "privacy",
        name: "Disable Visual Studio Telemetry",
        description: "Disables telemetry in Visual Studio.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch", 1),
        ],
                },
        crate::tweak! {
        id: "disable_office_feedback",
        category: "privacy",
        name: "Disable Office Feedback & Surveys",
        description: "Disables feedback surveys and other feedback mechanisms in Microsoft Office.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "Enabled", 0),
            crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "SurveyEnabled", 0),
            crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "IncludeEmail", 0),
        ],
                },
        crate::tweak! {
        id: "disable_siuf",
        category: "privacy",
        name: "Disable System Initiated User Feedback",
        description: "Prevents Windows from asking for feedback via popups.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKCU", r"Software\Microsoft\Siuf\rules", "NumberOfSIUFInPeriod", 0),
            crate::reg_dword!("HKLM", r"Software\Microsoft\Siuf\rules", "NumberOfSIUFInPeriod", 0),
        ],
                },
        crate::tweak! {
        id: "disable_push_notifications",
        category: "privacy",
        name: "Disable Cloud Push Notifications",
        description: "Disables cloud-based push notifications from installed applications.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", "NoCloudApplicationNotification", 1, 0),
        ],
                },
        crate::tweak! {
        id: "disable_location_services",
        category: "privacy",
        name: "Disable Location Services",
        description: "Disables Windows Location Services and prevents apps from accessing location.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", "DisableLocation", 1),
            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", "DisableSensors", 1),
            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", "DisableWindowsLocationProvider", 1),
            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\locationandsensors", "DisableLocationScripting", 1),
        ],
                },
        crate::tweak! {
        id: "disable_device_health_attestation",
        category: "privacy",
        name: "Disable Device Health Attestation",
        description: "Disables the Device Health Attestation service.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
            crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice", "EnableDeviceHealthAttestationService", 0),
        ],
                },
        crate::tweak! {
            id: "disable_npsm_svc",
            category: "privacy",
            name: "Disable Now Playing Session Manager",
            description: "Disables the Now Playing Session Manager service (NPSMsvc), which tracks media sessions.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\NPSMSvc", "Start", 4, 3),
            ],
                        requires_restart: true,
        },
];
