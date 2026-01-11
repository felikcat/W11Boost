use super::Tweak;

pub static PRIVACY_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_account_info_access",
            category: "privacy",
            name: "Disable Account Info Access",
            description: "Blocks apps from accessing your name, picture, and other account info.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userAccountInformation", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_activity_access",
            category: "privacy",
            name: "Disable Activity Access",
            description: "Blocks apps from accessing your activity history.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\activity", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_activity_history",
            category: "privacy",
            name: "Disable Activity History/Timeline",
            description: "Disables Windows Timeline and activity history collection.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_app_archiving",
            category: "privacy",
            name: "Disable App Archiving",
            description: "Prevents Windows from automatically archiving apps you don't use often.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_app_diagnostics_access",
            category: "privacy",
            name: "Disable App Diagnostics Access",
            description: "Blocks apps from accessing diagnostic information about other apps.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\appDiagnostics", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_app_launch_tracking",
            category: "privacy",
            name: "Disable App Launch Tracking",
            description: "Prevents Windows from tracking app launches to improve Start menu and search results.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
            ],
        },
        crate::tweak! {
            id: "disable_appx_background_updates",
            category: "privacy",
            name: "Disable Appx Background Updates",
            description: "Prevents Appx packages from updating in the background.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "DisableBackgroundAutoUpdates", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_biometrics",
            category: "privacy",
            name: "Disable Biometrics",
            description: "Disables biometric features like Windows Hello.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Biometrics", "Enabled", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_bluetooth_advertising",
            category: "privacy",
            name: "Disable Bluetooth Advertising",
            description: "Prevents the device from broadcasting Bluetooth advertising data.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Bluetooth", "AllowAdvertising", 0, RegistryValue::Delete)
            ],
        },
        crate::tweak! {
            id: "disable_bluetooth_sync_access",
            category: "privacy",
            name: "Disable Bluetooth Sync Access",
            description: "Blocks apps from syncing with Bluetooth devices.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\bluetoothSync", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_broad_filesystem_access",
            category: "privacy",
            name: "Disable Broad File System Access",
            description: "Blocks apps from accessing your file system broadly.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\broadFileSystemAccess", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_calendar_access",
            category: "privacy",
            name: "Disable Calendar Access",
            description: "Blocks apps from accessing your calendar.",
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
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCallHistory", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_cellular_data_access",
            category: "privacy",
            name: "Disable Cellular Data Access",
            description: "Blocks apps from using cellular data.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\cellularData", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_chat_auto_install",
            category: "privacy",
            name: "Disable Chat Auto Install",
            description: "Prevents automatic installation of the Chat app.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Communications", "ConfigureChatAutoInstall", 0),
            ],
        },
        crate::tweak! {
            id: "disable_clipboard_history",
            category: "privacy",
            name: "Disable Clipboard History",
            description: "Disables the collection of clipboard history.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowClipboardHistory", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Clipboard", "EnableClipboardHistory", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_push_notifications",
            category: "privacy",
            name: "Disable Cloud Push Notifications",
            description: "Disables cloud-based push notifications from installed applications.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", "NoCloudApplicationNotification", 1, 0),
            ],
        },
        crate::tweak! {
            id: "disable_contacts_access",
            category: "privacy",
            name: "Disable Contacts Access",
            description: "Blocks apps from accessing your contacts.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\contacts", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_device_driver_reporting",
            category: "privacy",
            name: "Disable Device Driver Reporting",
            description: "Prevents sending device driver error reports to Windows Error Reporting.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendGenericDriverNotFoundToWER", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendRequestAdditionalSoftwareToWER", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_device_name_telemetry",
            category: "privacy",
            name: "Disable Device Name in Telemetry",
            description: "Prevents sending the device name in Windows telemetry data.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_diagtrack_toast",
            category: "privacy",
            name: "Disable DiagTrack Toasts",
            description: "Disables diagnostic tracking toast notifications.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack", "ShowedToastAtLevel", 1),
            ],
        },
        crate::tweak! {
            id: "disable_documents_library_access",
            category: "privacy",
            name: "Disable Documents Library Access",
            description: "Blocks apps from accessing your documents library.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\documentsLibrary", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_downloads_folder_access",
            category: "privacy",
            name: "Disable Downloads Folder Access",
            description: "Blocks apps from accessing your downloads folder.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\downloadsfolder", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_email_access",
            category: "privacy",
            name: "Disable Email Access",
            description: "Blocks apps from accessing and sending email.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\email", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_explorer_ads_policy",
            category: "privacy",
            name: "Disable Explorer Ads (Policy)",
            description: "Disables notifications and search box suggestions in File Explorer via Policy.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableNotificationCenter", 1, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_gaze_input",
            category: "privacy",
            name: "Disable Eye Tracker (Gaze Input)",
            description: "Blocks apps from using eye tracking devices.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\gazeInput", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_game_bar",
            category: "privacy",
            name: "Disable Game Bar",
            description: "Disables the Xbox Game Bar recording and overlay features.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"System\GameConfigStore", "GameDVR_Enabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GameDVR", "AllowGameDVR", 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "HistoricalCaptureEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AudioCaptureEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "CursorCaptureEnabled", 0, 1),
            ],
        },
        crate::tweak! {
            id: "disable_geolocation_service",
            category: "privacy",
            name: "Disable Geolocation Service",
            description: "Disables the Geolocation service (lfsvc) to prevent location tracking.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\ControlSet001\Services\lfsvc\Service\Configuration", "Status", 0, RegistryValue::Dword(1)),
            ],
        },
        crate::tweak! {
            id: "disable_graphics_capture",
            category: "privacy",
            name: "Disable Graphics Capture",
            description: "Disables the Windows Graphics Capture API for added privacy.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GraphicsCapture", "AllowGraphicsCapture", 0),
            ],
        },
        crate::tweak! {
            id: "disable_input_insights",
            category: "privacy",
            name: "Disable Input Insights",
            description: "Disables 'Insights' features for typing and input.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\input\Settings", "InsightsEnabled", 0, 1),
            ],
        },
        crate::tweak! {
            id: "disable_instrumentation",
            category: "privacy",
            name: "Disable Instrumentation",
            description: "Disables system instrumentation feedback.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoInstrumentation", 1),
            ],
        },
        crate::tweak! {
            id: "disable_live_tiles",
            category: "privacy",
            name: "Disable Live Tiles",
            description: "Disables live tile updates on Start menu, preventing background data fetching.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\CurrentVersion\PushNotifications", "NoTileApplicationNotification", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_location_access",
            category: "privacy",
            name: "Disable Location Access",
            description: "Disables location services, sensors, and Windows Location Provider.",
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
            id: "disable_lock_screen_camera",
            category: "privacy",
            name: "Disable Lock Screen Camera",
            description: "Disables the camera button on the lock screen.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Personalization", "NoLockScreenCamera", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_lock_screen_widgets",
            category: "privacy",
            name: "Disable Lock Screen Widgets",
            description: "Disables widgets like Weather on the Lock Screen.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Lock Screen", "LockScreenWidgetsEnabled", 0),
            ],
        },
        crate::tweak! {
            id: "disable_page_combining",
            category: "privacy",
            name: "Disable Memory Page Combining",
            description: "Disables memory page combining which can be a security/privacy risk.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePageCombining", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_messaging_access",
            category: "privacy",
            name: "Disable Messaging Access",
            description: "Blocks apps from reading or sending messages (text or MMS).",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\chat", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_microphone_access",
            category: "privacy",
            name: "Disable Microphone Access",
            description: "Blocks apps from accessing the microphone.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_music_library_access",
            category: "privacy",
            name: "Disable Music Library Access",
            description: "Blocks apps from accessing your music library.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\musiclibrary", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_notification_access",
            category: "privacy",
            name: "Disable Notification Access",
            description: "Blocks apps from accessing your notifications.",
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userNotificationListener", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_npsm_svc",
            category: "privacy",
            name: "Disable Now Playing Session Manager",
            description: "Disables the Now Playing Session Manager service (NPSMsvc), which tracks media sessions.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\NPSMSvc", "Start", 4, 3),
            ],
        },
        crate::tweak! {
            id: "disable_office_feedback",
            category: "privacy",
            name: "Disable Office Feedback & Surveys",
            description: "Disables feedback surveys and other feedback mechanisms in Microsoft Office.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "Enabled", 0),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "SurveyEnabled", 0),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\office\16.0\common\feedback", "IncludeEmail", 0),
            ],
        },
        crate::tweak! {
            id: "disable_oobe_privacy",
            category: "privacy",
            name: "Disable OOBE Privacy Experience",
            description: "Disables the Privacy Settings Experience at sign-in.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_phone_call_access",
            category: "privacy",
            name: "Disable Phone Call Access",
            description: "Blocks apps from making phone calls.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall", "Value", "Deny", "Allow"),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\phoneCall", "Value", "Deny", "Allow"),
            ],
        },
        crate::tweak! {
            id: "disable_pictures_library_access",
            category: "privacy",
            name: "Disable Pictures Library Access",
            description: "Blocks apps from accessing your pictures library.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\picturesLibrary", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_radios_access",
            category: "privacy",
            name: "Disable Radios Access",
            description: "Blocks apps from controlling device radios (like Bluetooth).",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\radios", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_search_location",
            category: "privacy",
            name: "Disable Search Location Access",
            description: "Prevents Windows Search from using your location.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowSearchToUseLocation", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_search_web_results",
            category: "privacy",
            name: "Disable Search Web Results",
            description: "Disables Bing search results and web integration in Windows Search.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCloudSearch", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "EnableDynamicContentInWSB", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Windows Search", "CortanaConsent", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_sensor_permission_override",
            category: "privacy",
            name: "Disable Sensor Permission Override",
            description: "Disables the sensor permission state override.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Sensor\Overrides\{BFA794E4-F964-4FDB-90F6-51056BFE4B44}", "SensorPermissionState", 0, RegistryValue::Dword(1)),
            ],
        },
        crate::tweak! {
            id: "disable_speech_model_updates",
            category: "privacy",
            name: "Disable Speech Model Updates",
            description: "Prevents automatic downloads and updates of speech recognition models.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Speech_OneCore\Preferences", "ModelDownloadAllowed", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_suggested_content",
            category: "privacy",
            name: "Disable Suggested Content",
            description: "Disables suggested content, promotions, and suggestions in Windows.",
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
            id: "disable_tasks_access",
            category: "privacy",
            name: "Disable Tasks Access",
            description: "Blocks apps from accessing your tasks.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\userDataTasks", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_telemetry_optin_notification",
            category: "privacy",
            name: "Disable Telemetry Opt-in Notification",
            description: "Disables the notification asking to opt-in to telemetry.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableTelemetryOptInChangeNotification", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_text_input_personalization",
            category: "privacy",
            name: "Disable Text Input Personalization",
            description: "Disables collection of typing and handwriting data to improve recognition.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\input\TIPC", "Enabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 1, 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\InputPersonalization\TrainedDataStore", "HarvestContacts", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\InputPersonalization", "AllowInputPersonalization", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_text_prediction",
            category: "privacy",
            name: "Disable Text Prediction",
            description: "Disables text prediction and auto-correction features.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\TabletTip\1.7", "EnableTextPrediction", 0, RegistryValue::Delete)
            ],
        },
        crate::tweak! {
            id: "disable_third_party_suggestions",
            category: "privacy",
            name: "Disable Third-Party Suggestions",
            description: "Disables suggestions from third-party apps in Windows.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableThirdPartySuggestions", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_toast_notifications",
            category: "privacy",
            name: "Disable Toast Notifications",
            description: "Disables all toast (pop-up) notifications.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\PushNotifications", "ToastEnabled", 0, RegistryValue::Delete)
            ],
        },
        crate::tweak! {
            id: "disable_videos_library_access",
            category: "privacy",
            name: "Disable Videos Library Access",
            description: "Blocks apps from accessing your videos library.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\videosLibrary", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_voice_activation",
            category: "privacy",
            name: "Disable Voice Activation",
            description: "Disables apps from listening for voice keywords (like 'Hey Cortana').",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationOnLockScreenEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationLastUsed", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_webcam_access",
            category: "privacy",
            name: "Disable Webcam Access",
            description: "Blocks apps from accessing the webcam.",
            enabled_ops: &[
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam", "Value", "Deny", RegistryValue::String("Allow")),
                crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\webcam", "Value", "Deny", RegistryValue::String("Allow")),
            ],
        },
        crate::tweak! {
            id: "disable_language_list_access",
            category: "privacy",
            name: "Disable Website Language Access",
            description: "Prevents websites from accessing your language list.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut", 1, RegistryValue::Delete)
            ],
        },
        crate::tweak! {
            id: "disable_windows_spotlight",
            category: "privacy",
            name: "Disable Windows Spotlight",
            description: "Disables Windows Spotlight features on the lock screen.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_onedrive",
            category: "privacy",
            name: "Prevent OneDrive Usage",
            description: "Prevents OneDrive file sync and usage via Group Policy.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OneDrive", "DisableFileSyncNGSC", 1),
            ],
        },
];
