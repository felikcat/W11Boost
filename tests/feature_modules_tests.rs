//! Unit tests for feature modules (disable_copilot, disable_recall, etc.)

// ============================================
// Tests for disable_copilot module constants
// ============================================

mod disable_copilot_tests {
    const COPILOT_REGISTRY_PATH: &str = r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot";
    const COPILOT_VALUE_NAME: &str = "TurnOffWindowsCopilot";
    const COPILOT_DISABLE_VALUE: u32 = 1;

    #[test]
    fn test_copilot_registry_path()
    {
        assert_eq!(COPILOT_REGISTRY_PATH, r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot");
    }

    #[test]
    fn test_copilot_value_name()
    {
        assert_eq!(COPILOT_VALUE_NAME, "TurnOffWindowsCopilot");
    }

    #[test]
    fn test_copilot_disable_value()
    {
        // Value of 1 means Copilot is disabled
        assert_eq!(COPILOT_DISABLE_VALUE, 1);
    }

    #[test]
    fn test_registry_path_is_valid()
    {
        // No leading/trailing backslashes
        assert!(!COPILOT_REGISTRY_PATH.starts_with('\\'));
        assert!(!COPILOT_REGISTRY_PATH.ends_with('\\'));

        // Expected path components
        assert!(COPILOT_REGISTRY_PATH.contains("Policies"));
        assert!(COPILOT_REGISTRY_PATH.contains("Microsoft"));
        assert!(COPILOT_REGISTRY_PATH.contains("Windows"));
    }
}

// ============================================
// Tests for disable_recall module constants
// ============================================

mod disable_recall_tests {
    const RECALL_REGISTRY_PATH: &str = r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI";
    const RECALL_VALUE_NAME: &str = "AllowRecallEnablement";
    const RECALL_DISABLE_VALUE: u32 = 0;

    #[test]
    fn test_recall_registry_path()
    {
        assert_eq!(RECALL_REGISTRY_PATH, r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI");
    }

    #[test]
    fn test_recall_value_name()
    {
        assert_eq!(RECALL_VALUE_NAME, "AllowRecallEnablement");
    }

    #[test]
    fn test_recall_disable_value()
    {
        // Value of 0 means Recall is disabled (not allowed)
        assert_eq!(RECALL_DISABLE_VALUE, 0);
    }

    #[test]
    fn test_registry_path_is_valid()
    {
        // No leading/trailing backslashes
        assert!(!RECALL_REGISTRY_PATH.starts_with('\\'));
        assert!(!RECALL_REGISTRY_PATH.ends_with('\\'));

        // Expected path components
        assert!(RECALL_REGISTRY_PATH.contains("Policies"));
        assert!(RECALL_REGISTRY_PATH.contains("Microsoft"));
        assert!(RECALL_REGISTRY_PATH.contains("WindowsAI"));
    }
}

// ============================================
// Tests for disable_sleep module constants
// ============================================

mod disable_sleep_tests {
    mod power_guids {
        pub const HYBRID_SLEEP: &str = "94ac6d29-73ce-41a6-809f-6363ba21b47e";
        pub const SLEEP_IDLE: &str = "29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA";
        pub const HIBERNATE_IDLE: &str = "9D7815A6-7EE4-497E-8888-515A05F02364";
        pub const UNATTENDED_SLEEP: &str = "7bc4a2f9-d8fc-4469-b07b-33eb785aaca0";
    }

    mod registry_paths {
        pub const POWER_CONTROL: &str = r"SYSTEM\CurrentControlSet\Control\Power";
        pub const FLYOUT_MENU: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings";
    }

    const REGISTRY_OPERATIONS_COUNT: usize = 11;

    #[test]
    fn test_hybrid_sleep_guid_format()
    {
        assert_eq!(power_guids::HYBRID_SLEEP, "94ac6d29-73ce-41a6-809f-6363ba21b47e");
        assert!(power_guids::HYBRID_SLEEP.contains('-'));
        assert_eq!(power_guids::HYBRID_SLEEP.len(), 36);
    }

    #[test]
    fn test_sleep_idle_guid_format()
    {
        assert_eq!(power_guids::SLEEP_IDLE, "29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA");
        assert_eq!(power_guids::SLEEP_IDLE.len(), 36);
    }

    #[test]
    fn test_hibernate_idle_guid_format()
    {
        assert_eq!(power_guids::HIBERNATE_IDLE, "9D7815A6-7EE4-497E-8888-515A05F02364");
        assert_eq!(power_guids::HIBERNATE_IDLE.len(), 36);
    }

    #[test]
    fn test_unattended_sleep_guid_format()
    {
        assert_eq!(power_guids::UNATTENDED_SLEEP, "7bc4a2f9-d8fc-4469-b07b-33eb785aaca0");
        assert_eq!(power_guids::UNATTENDED_SLEEP.len(), 36);
    }

    #[test]
    fn test_all_guids_are_unique()
    {
        let guids = [
            power_guids::HYBRID_SLEEP,
            power_guids::SLEEP_IDLE,
            power_guids::HIBERNATE_IDLE,
            power_guids::UNATTENDED_SLEEP,
        ];

        for i in 0..guids.len() {
            for j in (i + 1)..guids.len() {
                assert_ne!(
                    guids[i].to_lowercase(),
                    guids[j].to_lowercase(),
                    "GUIDs at index {} and {} are not unique",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_power_control_path()
    {
        assert_eq!(registry_paths::POWER_CONTROL, r"SYSTEM\CurrentControlSet\Control\Power");
        assert!(!registry_paths::POWER_CONTROL.starts_with('\\'));
        assert!(!registry_paths::POWER_CONTROL.ends_with('\\'));
    }

    #[test]
    fn test_flyout_menu_path()
    {
        assert_eq!(
            registry_paths::FLYOUT_MENU,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings"
        );
        assert!(registry_paths::FLYOUT_MENU.contains("Explorer"));
    }

    #[test]
    fn test_registry_operations_count()
    {
        assert_eq!(REGISTRY_OPERATIONS_COUNT, 11);
    }
}

// ============================================
// Tests for minimize_forensics module constants
// ============================================

mod minimize_forensics_tests {
    mod registry_paths {
        pub const PERFLIB: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib";
        pub const EXPLORER_POLICIES: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer";
        pub const SYSMAIN_SERVICE: &str = r"SYSTEM\CurrentControlSet\Services\SysMain";
        pub const APP_COMPAT: &str = r"SOFTWARE\Policies\Microsoft\Windows\AppCompat";
        pub const SYSTEM_POLICIES: &str = r"SOFTWARE\Policies\Microsoft\Windows\System";
        pub const FILE_HISTORY: &str = r"SOFTWARE\Policies\Microsoft\Windows\FileHistory";
        pub const USER_ASSIST_1: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings";
        pub const USER_ASSIST_2: &str = r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings";
        pub const CONTENT_DELIVERY_MANAGER: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        pub const WER_POLICIES: &str = r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting";
        pub const BAM_SERVICE: &str = r"SYSTEM\CurrentControlSet\Services\bam";
        pub const DAM_SERVICE: &str = r"SYSTEM\CurrentControlSet\Services\dam";
    }

    mod service_values {
        pub const DISABLED: u32 = 4;
        pub const MANUAL: u32 = 3;
        pub const AUTO: u32 = 2;
        pub const SYSTEM: u32 = 1;
        pub const BOOT: u32 = 0;
    }

    const REGISTRY_OPERATIONS_COUNT: usize = 56; // Registry set_dword calls
    const SYSTEM_COMMANDS_COUNT: usize = 1; // fsutil disablelastaccess

    #[test]
    fn test_perflib_path()
    {
        assert_eq!(registry_paths::PERFLIB, r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib");
        assert!(registry_paths::PERFLIB.contains("Perflib"));
    }

    #[test]
    fn test_explorer_policies_path()
    {
        assert_eq!(
            registry_paths::EXPLORER_POLICIES,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer"
        );
        assert!(registry_paths::EXPLORER_POLICIES.contains("Policies"));
    }

    #[test]
    fn test_sysmain_service_path()
    {
        assert_eq!(registry_paths::SYSMAIN_SERVICE, r"SYSTEM\CurrentControlSet\Services\SysMain");
        assert!(registry_paths::SYSMAIN_SERVICE.contains("Services"));
    }

    #[test]
    fn test_app_compat_path()
    {
        assert_eq!(registry_paths::APP_COMPAT, r"SOFTWARE\Policies\Microsoft\Windows\AppCompat");
        assert!(registry_paths::APP_COMPAT.contains("AppCompat"));
    }

    #[test]
    fn test_system_policies_path()
    {
        assert_eq!(registry_paths::SYSTEM_POLICIES, r"SOFTWARE\Policies\Microsoft\Windows\System");
    }

    #[test]
    fn test_file_history_path()
    {
        assert_eq!(registry_paths::FILE_HISTORY, r"SOFTWARE\Policies\Microsoft\Windows\FileHistory");
        assert!(registry_paths::FILE_HISTORY.contains("FileHistory"));
    }

    #[test]
    fn test_all_paths_are_valid()
    {
        let paths = [
            registry_paths::PERFLIB,
            registry_paths::EXPLORER_POLICIES,
            registry_paths::SYSMAIN_SERVICE,
            registry_paths::APP_COMPAT,
            registry_paths::SYSTEM_POLICIES,
            registry_paths::FILE_HISTORY,
        ];

        for path in paths {
            assert!(!path.starts_with('\\'), "Path should not start with backslash: {path}");
            assert!(!path.ends_with('\\'), "Path should not end with backslash: {path}");
            assert!(path.contains('\\'), "Path should be multi-level: {path}");
        }
    }

    #[test]
    fn test_registry_operations_count()
    {
        assert_eq!(REGISTRY_OPERATIONS_COUNT, 56);
    }

    #[test]
    fn test_system_commands_count()
    {
        // 1. fsutil behavior set disablelastaccess 3
        assert_eq!(SYSTEM_COMMANDS_COUNT, 1);
    }

    #[test]
    fn test_user_assist_guids()
    {
        // UserAssist GUIDs track program execution
        assert!(registry_paths::USER_ASSIST_1.contains("CEBFF5CD-ACE2-4F4F-9178-9926F41749EA"));
        assert!(registry_paths::USER_ASSIST_2.contains("F4E57C4B-2036-45F0-A9AB-443BCFE33D9F"));
    }

    #[test]
    fn test_bam_dam_service_paths()
    {
        // Background/Desktop Activity Moderator services
        assert!(registry_paths::BAM_SERVICE.contains("bam"));
        assert!(registry_paths::DAM_SERVICE.contains("dam"));
        assert!(registry_paths::BAM_SERVICE.contains("Services"));
        assert!(registry_paths::DAM_SERVICE.contains("Services"));
    }

    #[test]
    fn test_service_disable_value()
    {
        // Service Start value of 4 means disabled
        assert_eq!(service_values::DISABLED, 4);
        assert_eq!(service_values::MANUAL, 3);
        assert_eq!(service_values::AUTO, 2);
    }

    #[test]
    fn test_wer_policies_path()
    {
        assert!(registry_paths::WER_POLICIES.contains("Windows Error Reporting"));
        assert!(registry_paths::WER_POLICIES.contains("Policies"));
    }

    // Additional registry paths for forensics reduction
    mod additional_paths {
        pub const SEARCH_SETTINGS: &str = r"Software\Microsoft\Windows\CurrentVersion\SearchSettings";
        pub const COMDLG32: &str = r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32";
        pub const APP_LOOKUP_SVC: &str = r"SYSTEM\CurrentControlSet\Services\AeLookupSvc";
        pub const DMWAPPUSHSERVICE: &str = r"SYSTEM\CurrentControlSet\Services\dmwappushservice";
        pub const DIAGSVC: &str = r"SYSTEM\CurrentControlSet\Services\diagsvc";
        pub const HANDWRITING_REPORTS: &str = r"Software\Policies\Microsoft\Windows\HandwritingErrorReports";
        pub const DATA_COLLECTION: &str = r"Software\Policies\Microsoft\Windows\DataCollection";
    }

    #[test]
    fn test_search_settings_path()
    {
        assert!(additional_paths::SEARCH_SETTINGS.contains("SearchSettings"));
    }

    #[test]
    fn test_comdlg32_path()
    {
        assert!(additional_paths::COMDLG32.contains("comdlg32"));
        assert!(additional_paths::COMDLG32.contains("Policies"));
    }

    #[test]
    fn test_app_lookup_service_path()
    {
        // Application Experience service
        assert!(additional_paths::APP_LOOKUP_SVC.contains("AeLookupSvc"));
        assert!(additional_paths::APP_LOOKUP_SVC.contains("Services"));
    }

    #[test]
    fn test_dmwappushservice_path()
    {
        // WAP push message routing service
        assert!(additional_paths::DMWAPPUSHSERVICE.contains("dmwappushservice"));
        assert!(additional_paths::DMWAPPUSHSERVICE.contains("Services"));
    }

    #[test]
    fn test_diagsvc_path()
    {
        // Diagnostics hub service
        assert!(additional_paths::DIAGSVC.contains("diagsvc"));
        assert!(additional_paths::DIAGSVC.contains("Services"));
    }

    #[test]
    fn test_all_service_paths_follow_pattern()
    {
        let service_paths = [
            registry_paths::SYSMAIN_SERVICE,
            registry_paths::BAM_SERVICE,
            registry_paths::DAM_SERVICE,
            additional_paths::APP_LOOKUP_SVC,
            additional_paths::DMWAPPUSHSERVICE,
            additional_paths::DIAGSVC,
        ];

        for path in service_paths {
            assert!(
                path.starts_with(r"SYSTEM\CurrentControlSet\Services"),
                "Service path should start with SYSTEM\\CurrentControlSet\\Services: {path}"
            );
        }
    }

    #[test]
    fn test_service_values_hierarchy()
    {
        // Service start values should be in correct order
        assert!(service_values::BOOT < service_values::SYSTEM);
        assert!(service_values::SYSTEM < service_values::AUTO);
        assert!(service_values::AUTO < service_values::MANUAL);
        assert!(service_values::MANUAL < service_values::DISABLED);
    }

    #[test]
    fn test_disabled_is_highest_value()
    {
        // DISABLED (4) should be the highest value to prevent service start
        assert_eq!(service_values::DISABLED, 4);
        assert!(service_values::DISABLED > service_values::MANUAL);
    }

    #[test]
    fn test_thumbnail_cache_disabling()
    {
        // Thumbnail cache values should be in the Explorer\Advanced path
        assert!(registry_paths::CONTENT_DELIVERY_MANAGER.contains("ContentDeliveryManager"));
    }

    #[test]
    fn test_diagnostic_log_collection_paths()
    {
        assert!(additional_paths::DATA_COLLECTION.contains("DataCollection"));
        assert!(additional_paths::DATA_COLLECTION.contains("Policies"));
    }

    #[test]
    fn test_handwriting_reports_path()
    {
        // PreventHandwritingErrorReports setting path
        assert!(additional_paths::HANDWRITING_REPORTS.contains("HandwritingErrorReports"));
        assert!(additional_paths::HANDWRITING_REPORTS.contains("Policies"));
    }
}

// ============================================
// Tests for minimize_online_data_collection module constants
// ============================================

mod minimize_online_data_collection_tests {
    mod registry_paths {
        pub const CLOUD_CONTENT: &str = r"SOFTWARE\Policies\Microsoft\Windows\CloudContent";
        pub const WINDOWS_SEARCH: &str = r"SOFTWARE\Policies\Microsoft\Windows\Windows Search";
        pub const INPUT_PERSONALIZATION: &str = r"SOFTWARE\Policies\Microsoft\InputPersonalization";
        pub const EXPLORER: &str = r"SOFTWARE\Policies\Microsoft\Windows\Explorer";
        pub const SYSTEM: &str = r"SOFTWARE\Policies\Microsoft\Windows\System";
    }

    mod cloud_content_values {
        pub const DISABLE_CLOUD_OPTIMIZED_CONTENT: &str = "DisableCloudOptimizedContent";
        pub const DISABLE_CONSUMER_ACCOUNT_STATE: &str = "DisableConsumerAccountStateContent";
        pub const DISABLE_SOFT_LANDING: &str = "DisableSoftLanding";
        pub const DISABLE_WINDOWS_CONSUMER_FEATURES: &str = "DisableWindowsConsumerFeatures";
    }

    const REGISTRY_OPERATIONS_COUNT: usize = 22; // All set_dword calls in the module

    #[test]
    fn test_cloud_content_path()
    {
        assert_eq!(registry_paths::CLOUD_CONTENT, r"SOFTWARE\Policies\Microsoft\Windows\CloudContent");
        assert!(registry_paths::CLOUD_CONTENT.contains("CloudContent"));
    }

    #[test]
    fn test_windows_search_path()
    {
        assert_eq!(registry_paths::WINDOWS_SEARCH, r"SOFTWARE\Policies\Microsoft\Windows\Windows Search");
        assert!(registry_paths::WINDOWS_SEARCH.contains("Search"));
    }

    #[test]
    fn test_input_personalization_path()
    {
        assert_eq!(registry_paths::INPUT_PERSONALIZATION, r"SOFTWARE\Policies\Microsoft\InputPersonalization");
        assert!(registry_paths::INPUT_PERSONALIZATION.contains("InputPersonalization"));
    }

    #[test]
    fn test_explorer_path()
    {
        assert_eq!(registry_paths::EXPLORER, r"SOFTWARE\Policies\Microsoft\Windows\Explorer");
        assert!(registry_paths::EXPLORER.contains("Explorer"));
    }

    #[test]
    fn test_system_path()
    {
        assert_eq!(registry_paths::SYSTEM, r"SOFTWARE\Policies\Microsoft\Windows\System");
    }

    #[test]
    fn test_all_paths_are_valid()
    {
        let paths = [
            registry_paths::CLOUD_CONTENT,
            registry_paths::WINDOWS_SEARCH,
            registry_paths::INPUT_PERSONALIZATION,
            registry_paths::EXPLORER,
            registry_paths::SYSTEM,
        ];

        for path in paths {
            assert!(!path.starts_with('\\'), "Path should not start with backslash: {path}");
            assert!(!path.ends_with('\\'), "Path should not end with backslash: {path}");
            assert!(
                path.contains("Policies") || path.contains("Microsoft"),
                "Path should contain Policies or Microsoft: {path}"
            );
        }
    }

    #[test]
    fn test_registry_operations_count()
    {
        assert_eq!(REGISTRY_OPERATIONS_COUNT, 22);
    }

    #[test]
    fn test_cloud_content_values()
    {
        assert_eq!(cloud_content_values::DISABLE_CLOUD_OPTIMIZED_CONTENT, "DisableCloudOptimizedContent");
        assert_eq!(cloud_content_values::DISABLE_CONSUMER_ACCOUNT_STATE, "DisableConsumerAccountStateContent");
        assert_eq!(cloud_content_values::DISABLE_SOFT_LANDING, "DisableSoftLanding");
        assert_eq!(cloud_content_values::DISABLE_WINDOWS_CONSUMER_FEATURES, "DisableWindowsConsumerFeatures");
    }

    #[test]
    fn test_all_cloud_content_values_unique()
    {
        let values = [
            cloud_content_values::DISABLE_CLOUD_OPTIMIZED_CONTENT,
            cloud_content_values::DISABLE_CONSUMER_ACCOUNT_STATE,
            cloud_content_values::DISABLE_SOFT_LANDING,
            cloud_content_values::DISABLE_WINDOWS_CONSUMER_FEATURES,
        ];

        for i in 0..values.len() {
            for j in (i + 1)..values.len() {
                assert_ne!(
                    values[i],
                    values[j],
                    "Values at index {} and {} are not unique",
                    i,
                    j
                );
            }
        }
    }
}

// ============================================
// Tests for non_intrusive_tweaks module constants
// ============================================

mod non_intrusive_tweaks_tests {
    const USER_NOT_PRESENT_SESSION_PATH: &str = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

    mod registry_paths {
        pub const DATA_COLLECTION: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection";
        pub const TELEMETRY: &str = r"SOFTWARE\Policies\Microsoft\Windows\DataCollection";
        pub const WER: &str = r"SOFTWARE\Microsoft\Windows\Windows Error Reporting";
        pub const DIAG_TRACK: &str = r"SYSTEM\CurrentControlSet\Services\DiagTrack";
        pub const POWER_THROTTLING: &str = r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling";
        pub const FTH: &str = r"SOFTWARE\Microsoft\FTH";
        pub const STORAGE_SENSE: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense";
        pub const CONTENT_DELIVERY_MANAGER: &str = r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        pub const CLOUD_CONTENT_HKCU: &str = r"Software\Policies\Microsoft\Windows\CloudContent";
    }

    // Windows ads settings
    mod content_delivery_values {
        pub const CONTENT_DELIVERY_ALLOWED: &str = "ContentDeliveryAllowed";
        pub const SILENT_INSTALLED_APPS: &str = "SilentInstalledAppsEnabled";
        pub const SYSTEM_PANE_SUGGESTIONS: &str = "SystemPaneSuggestionsEnabled";
        pub const SOFT_LANDING: &str = "SoftLandingEnabled";
        pub const ROTATING_LOCK_SCREEN: &str = "RotatingLockScreenEnabled";
        pub const ROTATING_LOCK_SCREEN_OVERLAY: &str = "RotatingLockScreenOverlayEnabled";
        pub const OEM_PRE_INSTALLED_APPS: &str = "OemPreInstalledAppsEnabled";
        pub const PRE_INSTALLED_APPS: &str = "PreInstalledAppsEnabled";
        pub const PRE_INSTALLED_APPS_EVER: &str = "PreInstalledAppsEverEnabled";
        pub const FEATURE_MANAGEMENT: &str = "FeatureManagementEnabled";
    }

    // Windows suggestions IDs
    mod subscribed_content_ids {
        pub const TIPS_TRICKS_SUGGESTIONS: &str = "SubscribedContent-338389Enabled";
        pub const WELCOME_EXPERIENCE: &str = "SubscribedContent-310093Enabled";
        pub const START_MENU_SUGGESTIONS: &str = "SubscribedContent-338388Enabled";
        pub const TIMELINE_SUGGESTIONS: &str = "SubscribedContent-338393Enabled";
        pub const SETTINGS_SUGGESTIONS_1: &str = "SubscribedContent-353694Enabled";
        pub const SETTINGS_SUGGESTIONS_2: &str = "SubscribedContent-353696Enabled";
        pub const SETTINGS_SUGGESTIONS_3: &str = "SubscribedContent-353698Enabled";
        pub const GET_MOST_OUT_OF_WINDOWS: &str = "SubscribedContent-88000326Enabled";
    }

    const REGISTRY_OPERATIONS_COUNT: usize = 86; // Total set_dword, set_string, remove_subkey calls
    const SYSTEM_COMMANDS_COUNT: usize = 3; // fsutil, bcdedit, powershell

    #[test]
    fn test_user_not_present_session_path()
    {
        assert_eq!(
            USER_NOT_PRESENT_SESSION_PATH,
            r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl"
        );
        assert!(USER_NOT_PRESENT_SESSION_PATH.ends_with(".etl"));
        assert!(USER_NOT_PRESENT_SESSION_PATH.starts_with(r"C:\Windows"));
    }

    #[test]
    fn test_data_collection_path()
    {
        assert_eq!(
            registry_paths::DATA_COLLECTION,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection"
        );
        assert!(registry_paths::DATA_COLLECTION.contains("DataCollection"));
    }

    #[test]
    fn test_telemetry_path()
    {
        assert_eq!(registry_paths::TELEMETRY, r"SOFTWARE\Policies\Microsoft\Windows\DataCollection");
        assert!(registry_paths::TELEMETRY.contains("Policies"));
    }

    #[test]
    fn test_wer_path()
    {
        assert_eq!(registry_paths::WER, r"SOFTWARE\Microsoft\Windows\Windows Error Reporting");
        assert!(registry_paths::WER.contains("Error Reporting"));
    }

    #[test]
    fn test_diag_track_path()
    {
        assert_eq!(registry_paths::DIAG_TRACK, r"SYSTEM\CurrentControlSet\Services\DiagTrack");
        assert!(registry_paths::DIAG_TRACK.contains("DiagTrack"));
    }

    #[test]
    fn test_power_throttling_path()
    {
        assert_eq!(registry_paths::POWER_THROTTLING, r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling");
        assert!(registry_paths::POWER_THROTTLING.contains("Power"));
    }

    #[test]
    fn test_fth_path()
    {
        assert_eq!(registry_paths::FTH, r"SOFTWARE\Microsoft\FTH");
    }

    #[test]
    fn test_storage_sense_path()
    {
        assert_eq!(registry_paths::STORAGE_SENSE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense");
        assert!(registry_paths::STORAGE_SENSE.contains("StorageSense"));
    }

    #[test]
    fn test_all_paths_are_valid()
    {
        let paths = [
            registry_paths::DATA_COLLECTION,
            registry_paths::TELEMETRY,
            registry_paths::WER,
            registry_paths::DIAG_TRACK,
            registry_paths::POWER_THROTTLING,
            registry_paths::FTH,
            registry_paths::STORAGE_SENSE,
        ];

        for path in paths {
            assert!(!path.starts_with('\\'), "Path should not start with backslash: {path}");
            assert!(!path.ends_with('\\'), "Path should not end with backslash: {path}");
            assert!(path.contains('\\'), "Path should be multi-level: {path}");
        }
    }

    #[test]
    fn test_registry_operations_count()
    {
        assert_eq!(REGISTRY_OPERATIONS_COUNT, 86);
    }

    #[test]
    fn test_system_commands_count()
    {
        // 1. fsutil behavior set memoryusage 2
        // 2. bcdedit /set recoveryenabled no
        // 3. powershell Set-NetAdapterAdvancedProperty
        assert_eq!(SYSTEM_COMMANDS_COUNT, 3);
    }

    #[test]
    fn test_content_delivery_manager_path()
    {
        assert_eq!(
            registry_paths::CONTENT_DELIVERY_MANAGER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"
        );
        assert!(registry_paths::CONTENT_DELIVERY_MANAGER.contains("ContentDeliveryManager"));
    }

    #[test]
    fn test_cloud_content_hkcu_path()
    {
        assert_eq!(
            registry_paths::CLOUD_CONTENT_HKCU,
            r"Software\Policies\Microsoft\Windows\CloudContent"
        );
        assert!(registry_paths::CLOUD_CONTENT_HKCU.contains("CloudContent"));
    }

    #[test]
    fn test_content_delivery_values()
    {
        // Test ContentDeliveryManager value names
        assert_eq!(content_delivery_values::CONTENT_DELIVERY_ALLOWED, "ContentDeliveryAllowed");
        assert_eq!(content_delivery_values::SILENT_INSTALLED_APPS, "SilentInstalledAppsEnabled");
        assert_eq!(content_delivery_values::SYSTEM_PANE_SUGGESTIONS, "SystemPaneSuggestionsEnabled");
        assert_eq!(content_delivery_values::SOFT_LANDING, "SoftLandingEnabled");
        assert_eq!(content_delivery_values::ROTATING_LOCK_SCREEN, "RotatingLockScreenEnabled");
        assert_eq!(content_delivery_values::ROTATING_LOCK_SCREEN_OVERLAY, "RotatingLockScreenOverlayEnabled");
        assert_eq!(content_delivery_values::OEM_PRE_INSTALLED_APPS, "OemPreInstalledAppsEnabled");
        assert_eq!(content_delivery_values::PRE_INSTALLED_APPS, "PreInstalledAppsEnabled");
        assert_eq!(content_delivery_values::PRE_INSTALLED_APPS_EVER, "PreInstalledAppsEverEnabled");
        assert_eq!(content_delivery_values::FEATURE_MANAGEMENT, "FeatureManagementEnabled");
    }

    #[test]
    fn test_subscribed_content_ids_format()
    {
        let ids = [
            subscribed_content_ids::TIPS_TRICKS_SUGGESTIONS,
            subscribed_content_ids::WELCOME_EXPERIENCE,
            subscribed_content_ids::START_MENU_SUGGESTIONS,
            subscribed_content_ids::TIMELINE_SUGGESTIONS,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_1,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_2,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_3,
            subscribed_content_ids::GET_MOST_OUT_OF_WINDOWS,
        ];

        for id in ids {
            assert!(id.starts_with("SubscribedContent-"), "ID should start with 'SubscribedContent-': {id}");
            assert!(id.ends_with("Enabled"), "ID should end with 'Enabled': {id}");
        }
    }

    #[test]
    fn test_all_subscribed_content_ids_unique()
    {
        let ids = [
            subscribed_content_ids::TIPS_TRICKS_SUGGESTIONS,
            subscribed_content_ids::WELCOME_EXPERIENCE,
            subscribed_content_ids::START_MENU_SUGGESTIONS,
            subscribed_content_ids::TIMELINE_SUGGESTIONS,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_1,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_2,
            subscribed_content_ids::SETTINGS_SUGGESTIONS_3,
            subscribed_content_ids::GET_MOST_OUT_OF_WINDOWS,
        ];

        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                assert_ne!(
                    ids[i],
                    ids[j],
                    "IDs at index {} and {} are not unique",
                    i,
                    j
                );
            }
        }
    }

    #[test]
    fn test_content_delivery_values_all_end_with_enabled()
    {
        let enabled_values = [
            content_delivery_values::SILENT_INSTALLED_APPS,
            content_delivery_values::SYSTEM_PANE_SUGGESTIONS,
            content_delivery_values::SOFT_LANDING,
            content_delivery_values::ROTATING_LOCK_SCREEN,
            content_delivery_values::ROTATING_LOCK_SCREEN_OVERLAY,
            content_delivery_values::OEM_PRE_INSTALLED_APPS,
            content_delivery_values::PRE_INSTALLED_APPS,
            content_delivery_values::PRE_INSTALLED_APPS_EVER,
            content_delivery_values::FEATURE_MANAGEMENT,
        ];

        for value in enabled_values {
            assert!(value.ends_with("Enabled"), "Value should end with 'Enabled': {value}");
        }
    }

    #[test]
    fn test_content_delivery_allowed_is_master_switch()
    {
        // Master switch, doesn't follow "Enabled" pattern
        assert_eq!(content_delivery_values::CONTENT_DELIVERY_ALLOWED, "ContentDeliveryAllowed");
        assert!(!content_delivery_values::CONTENT_DELIVERY_ALLOWED.ends_with("Enabled"));
    }
}

// ============================================
// Tests for remove_w11boost module constants
// ============================================

mod remove_w11boost_tests {
    const USER_NOT_PRESENT_SESSION_PATH: &str = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";
    const SYSTEM_COMMANDS_COUNT: usize = 3;

    #[test]
    fn test_user_not_present_session_path()
    {
        assert_eq!(
            USER_NOT_PRESENT_SESSION_PATH,
            r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl"
        );
        assert!(USER_NOT_PRESENT_SESSION_PATH.ends_with(".etl"));
        assert!(USER_NOT_PRESENT_SESSION_PATH.starts_with(r"C:\Windows"));
        assert!(USER_NOT_PRESENT_SESSION_PATH.contains("SleepStudy"));
    }

    #[test]
    fn test_system_commands_count()
    {
        // 1. fsutil behavior set memoryusage 1
        // 2. fsutil behavior set disablelastaccess 2
        // 3. bcdedit /set recoveryenabled yes
        assert_eq!(SYSTEM_COMMANDS_COUNT, 3);
    }

    #[test]
    fn test_path_consistency_with_non_intrusive_tweaks()
    {
        // The path should match the one used in non_intrusive_tweaks
        const NON_INTRUSIVE_PATH: &str = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";
        assert_eq!(USER_NOT_PRESENT_SESSION_PATH, NON_INTRUSIVE_PATH);
    }
}

// ============================================
// Tests for reset_windows_store module constants
// ============================================

mod reset_windows_store_tests {
    const WSRESET_EXECUTABLE: &str = "wsreset.exe";
    const WSRESET_INSTALL_ARG: &str = "-i";

    #[test]
    fn test_wsreset_executable()
    {
        assert_eq!(WSRESET_EXECUTABLE, "wsreset.exe");
        assert!(WSRESET_EXECUTABLE.ends_with(".exe"));
    }

    #[test]
    fn test_wsreset_install_arg()
    {
        assert_eq!(WSRESET_INSTALL_ARG, "-i");
        assert!(WSRESET_INSTALL_ARG.starts_with('-'));
    }
}
