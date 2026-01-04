//! Registry verification tool for W11Boost
//! Dumps all registry values W11Boost modifies and compares against Windows defaults.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use winsafe::co::{self, KNOWNFOLDERID};
use winsafe::{GetLocalTime, HKEY, RegistryValue, SHGetKnownFolderPath};

// Policy values that should NOT exist on fresh Windows
const POLICY_VALUES_TO_DELETE: &[(&str, &str, &str, &str)] = &[
    // disable_copilot.rs
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot", "disable_copilot"),
    // disable_recall.rs
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", "disable_recall"),
    // minimize_forensics.rs
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisablePCA", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableEngine", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "SbEnable", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInventory", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableAPISampling", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableApplicationFootprint", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInstallTracing", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableWin32AppBackup", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}", "ScenarioExecutionEnabled", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Messenger\Client", "CEIP", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", "minimize_forensics"),
    ("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\FileHistory", "Disabled", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoSaveSettings", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoRecentDocsMenu", "minimize_forensics"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\System", "AllowClipboardHistory", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchHistory", "minimize_forensics"),
    ("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "NoRemoteDestinations", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", "minimize_forensics"),
    ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", "minimize_forensics"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\HandwritingErrorReports", "PreventHandwritingErrorReports", "minimize_forensics"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDiagnosticLogCollection", "minimize_forensics"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDumpCollection", "minimize_forensics"),
    // minimize_online_data_collection.rs
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips", "AllowOnlineTips", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\InputPersonalization", "AllowInputPersonalization", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata", "PreventDeviceMetadataFromNetwork", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\SearchCompanion", "DisableContentFileUpdates", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowCrossDeviceClipboard", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth", "AllowDiskHealthModelUpdates", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "DisableBackgroundAutoUpdates", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableCloudOptimizedContent", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableConsumerAccountStateContent", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableSoftLanding", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableWindowsConsumerFeatures", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCloudSearch", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "EnableDynamicContentInWSB", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate", "minimize_online_data_collection"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", "minimize_online_data_collection"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\Personalization", "NoLockScreen", "minimize_online_data_collection"),
    // non_intrusive_tweaks.rs
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", "DisabledByGroupPolicy", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableOneSettingsDownloads", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableTelemetryOptInChangeNotification", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\AppV\CEIP", "CEIPEnable", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "AllOrNone", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeKernelFaults", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeMicrosoftApps", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeShutdownErrs", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeWindowsApps", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "DoReport", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendGenericDriverNotFoundToWER", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendRequestAdditionalSoftwareToWER", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "Disabled", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\TabletPC", "PreventHandwritingDataSharing", "non_intrusive_tweaks"),
    ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "DoNotShowFeedbackNotifications", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "ServiceEnabled", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "HiberbootEnabled", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowStorageSenseGlobal", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageSense", "AllowStorageSenseGlobal", "non_intrusive_tweaks"),
    ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures", "non_intrusive_tweaks"),
    ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData", "non_intrusive_tweaks"),
    ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableThirdPartySuggestions", "non_intrusive_tweaks"),
    // disable_telemetry.rs
    ("HKLM", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", "disable_telemetry"),
    ("HKCU", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", "disable_telemetry"),
    ("HKCU", r"Software\Policies\Microsoft\office\16.0\common\privacy", "SendTelemetry", "disable_telemetry"),
];

// Policy subkeys that should NOT exist
const POLICY_SUBKEYS_TO_DELETE: &[(&str, &str, &str)] = &[
    // disable_sleep.rs
    ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "disable_sleep"),
    ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "disable_sleep"),
    ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "disable_sleep"),
    ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "disable_sleep"),
    // non_intrusive_tweaks.rs
    ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent", "non_intrusive_tweaks"),
];

// Non-policy values that should NOT exist
const SETTINGS_TO_DELETE: &[(&str, &str, &str, &str)] = &[
    // disable_sleep.rs
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowHibernateOption", "disable_sleep"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowSleepOption", "disable_sleep"),
    // minimize_forensics.rs
    ("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib", "Disable Performance Counters", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsHistory", "minimize_forensics"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoStartMenuMFUprogramsList", "minimize_forensics"),
    ("HKCU", r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager", "EnablePeriodicBackup", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "LinkResolveIgnoreLinkInfo", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveSearch", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveTrack", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoFileMru", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoBackButton", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentDocsOnExit", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentProgForNewUserInStartMenu", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsNetHood", "minimize_forensics"),
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Management", "EnableSuperfetch", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings", "NoLog", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings", "NoLog", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbnailCache", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbsDBOnNetworkFolders", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DontUsePowerShellOnWinX", "minimize_forensics"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDeviceSearchHistoryEnabled", "minimize_forensics"),
    // non_intrusive_tweaks.rs
    ("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "PeriodInNanoSeconds", "non_intrusive_tweaks"),
    ("HKLM", r"SYSTEM\CurrentControlSet\Policies", "NtfsForceNonPagedPoolAllocation", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "verbosestatus", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\System", "AllowExperimentation", "non_intrusive_tweaks"),
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM", "OptIn", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM", "OptIn", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM", "OptIn", "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack", "ShowedToastAtLevel", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "Disabled", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "AutoApproveOSDumps", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DefaultConsent", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DontSendAdditionalData", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\OneDrive", "PreventNetworkTrafficPreUserSignIn", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput", "AllowLinguisticDataCollection", "non_intrusive_tweaks"),
    ("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut", "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "MSAOptional", "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSyncProviderNotifications", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEverEnabled", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338388Enabled", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338393Enabled", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353696Enabled", "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-88000326Enabled", "non_intrusive_tweaks"),
];

// Settings to restore to Windows defaults (hkey, subkey, value, default, module)
// Only values that actually exist on stock Windows 11
const SETTINGS_TO_RESTORE: &[(&str, &str, &str, u32, &str)] = &[
    // disable_sleep.rs
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Power", "HibernateEnabledDefault", 1, "disable_sleep"),
    // minimize_forensics.rs
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 3, "minimize_forensics"),
    // non_intrusive_tweaks.rs
    ("HKLM", r"SOFTWARE\Microsoft\FTH", "Enabled", 1, "non_intrusive_tweaks"),
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive", 0, "non_intrusive_tweaks"),
    ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 1, "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry", 3, "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "MaxTelemetryAllowed", 3, "non_intrusive_tweaks"),
    ("HKLM", r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0, "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0, "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 0, "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 0, "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "SeparateProcess", 0, "non_intrusive_tweaks"),
    ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "HideFileExt", 1, "non_intrusive_tweaks"),
    // ContentDeliveryManager - values that exist on stock Windows
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SilentInstalledAppsEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenOverlayEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "OemPreInstalledAppsEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEnabled", 1, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "FeatureManagementEnabled", 0, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338389Enabled", 0, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-310093Enabled", 0, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353694Enabled", 0, "non_intrusive_tweaks"),
    ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353698Enabled", 0, "non_intrusive_tweaks"),
];

// Services to restore (service_name, default_start, module)
// 1=System, 2=Auto, 3=Manual, 4=Disabled
const SERVICES_TO_RESTORE: &[(&str, u32, &str)] = &[
    ("SysMain", 2, "minimize_forensics"),
    ("PcaSvc", 2, "minimize_forensics"),
    ("bam", 1, "minimize_forensics"),
    ("dam", 1, "minimize_forensics"),
    ("dmwappushservice", 3, "minimize_forensics"),
    ("diagsvc", 3, "minimize_forensics"),
    ("DiagTrack", 2, "non_intrusive_tweaks"),
    ("WerSvc", 3, "non_intrusive_tweaks"),
];

// Environment variables that should NOT exist (from disable_telemetry.rs)
const ENV_VARS: &[&str] = &[
    "DOTNET_CLI_TELEMETRY_OPTOUT",
    "DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT",
    "DOTNET_SVCUTIL_TELEMETRY_OPTOUT",
    "MLDOTNET_CLI_TELEMETRY_OPTOUT",
    "MSSQL_CLI_TELEMETRY_OPTOUT",
    "VSTEST_TELEMETRY_OPTEDIN",
    "POWERSHELL_TELEMETRY_OPTOUT",
    "AZURE_CORE_COLLECT_TELEMETRY",
    "AZURE_DEV_COLLECT_TELEMETRY",
    "AZUREML_SDKV2_TELEMETRY_OPTOUT",
    "FUNCTIONS_CORE_TOOLS_TELEMETRY_OPTOUT",
    "NEXT_TELEMETRY_DISABLED",
    "NUXT_TELEMETRY_DISABLED",
    "GATSBY_TELEMETRY_DISABLED",
    "NG_CLI_ANALYTICS",
    "NG_CLI_ANALYTICS_SHARE",
    "ASTRO_TELEMETRY_DISABLED",
    "STORYBOOK_DISABLE_TELEMETRY",
    "REDWOOD_DISABLE_TELEMETRY",
    "YARN_ENABLE_TELEMETRY",
    "HINT_TELEMETRY",
    "VUEDX_TELEMETRY",
    "STRAPI_TELEMETRY_DISABLED",
    "SLS_TELEMETRY_DISABLED",
    "SLS_TRACKING_DISABLED",
    "CALCOM_TELEMETRY_DISABLED",
    "SKU_TELEMETRY",
    "EMBER_CLI_ANALYTICS",
    "CAPACITOR_TELEMETRY",
    "CARBON_TELEMETRY_DISABLED",
    "DAGSTER_DISABLE_TELEMETRY",
    "FEAST_TELEMETRY",
    "MELTANO_DISABLE_TRACKING",
    "RASA_TELEMETRY_ENABLED",
    "HAMILTON_TELEMETRY_ENABLED",
    "HF_HUB_DISABLE_TELEMETRY",
    "GRADIO_ANALYTICS_ENABLED",
    "RAGAS_DO_NOT_TRACK",
    "OPENLLM_DO_NOT_TRACK",
    "FLWR_TELEMETRY_ENABLED",
    "STREAMLIT_TELEMETRY_OPT_OUT",
    "WHYLOGS_NO_ANALYTICS",
    "JINA_OPTOUT_TELEMETRY",
    "SCHEMATHESIS_TELEMETRY",
    "DBT_SEND_ANONYMOUS_USAGE_STATS",
    "TERRAFORM_TELEMETRY",
    "CHECKPOINT_DISABLE",
    "VAGRANT_CHECKPOINT_DISABLE",
    "PACKER_CHECKPOINT_DISABLE",
    "CONSUL_CHECKPOINT_DISABLE",
    "ARM_DISABLE_TERRAFORM_PARTNER_ID",
    "CHEF_TELEMETRY_OPT_OUT",
    "AUTOMATEDLAB_TELEMETRY_OPTOUT",
    "NUKE_TELEMETRY_OPTOUT",
    "PNPPOWERSHELL_DISABLETELEMETRY",
    "EARTHLY_DISABLE_ANALYTICS",
    "WERF_TELEMETRY",
    "SCOUT_DISABLE",
    "INFRACOST_SELF_HOSTED_TELEMETRY",
    "BATECT_ENABLE_TELEMETRY",
    "DECK_ANALYTICS",
    "DO_NOT_TRACK",
    "KICS_COLLECT_TELEMETRY",
    "DISABLE_CRASH_REPORT",
    "CIRCLECI_CLI_TELEMETRY_OPTOUT",
    "CODER_TELEMETRY_ENABLE",
    "SAM_CLI_TELEMETRY",
    "CLOUDSDK_CORE_DISABLE_USAGE_REPORTING",
    "HOOKDECK_CLI_TELEMETRY_OPTOUT",
    "STRIPE_CLI_TELEMETRY_OPTOUT",
    "F5_ALLOW_TELEMETRY",
    "TEEM_DISABLE",
    "MSLAB_TELEMETRY_LEVEL",
    "INFLUXD_REPORTING_DISABLED",
    "QUILT_DISABLE_USAGE_METRICS",
    "QDRANT__TELEMETRY_DISABLED",
    "MONGODB_ATLAS_TELEMETRY_ENABLE",
    "FERRETDB_TELEMETRY",
    "CUBESTORE_TELEMETRY",
    "CUBEJS_TELEMETRY",
    "EVENTSTORE_TELEMETRY_OPTOUT",
    "HOMEBREW_NO_ANALYTICS",
    "CHOOSENIM_NO_ANALYTICS",
    "COCOAPODS_DISABLE_STATS",
    "ARDUINO_METRICS_ENABLED",
    "ROCKSET_CLI_TELEMETRY_OPTOUT",
    "APOLLO_TELEMETRY_DISABLED",
    "SFDX_DISABLE_TELEMETRY",
    "SF_DISABLE_TELEMETRY",
    "SALTO_TELEMETRY_DISABLE",
    "BF_CLI_TELEMETRY",
    "MOBILE_CENTER_TELEMETRY",
    "APPCD_TELEMETRY",
    "TUIST_STATS_OPT_OUT",
    "GOTELEMETRY",
    "RAY_USAGE_STATS_ENABLED",
    "APTOS_DISABLE_TELEMETRY",
    "SPEEDSTER_DISABLE_TELEMETRY",
    "DISABLE_DEEPCHECKS_ANONYMOUS_TELEMETRY",
    "DISABLE_DCS_ANONYMOUS_TELEMETRY",
    "DACFX_TELEMETRY_OPTOUT",
    "REACT_APP_WEBINY_TELEMETRY",
    "PRISMA_TELEMETRY",
    "ORYX_DISABLE_TELEMETRY",
    "SQA_OPT_OUT",
    "HASURA_GRAPHQL_ENABLE_TELEMETRY",
    "MEILI_NO_ANALYTICS",
    "NOCODB_TELEMETRY",
    "NC_DISABLE_TELE",
    "PROSE_TELEMETRY_OPTOUT",
    "RESTLER_TELEMETRY_OPTOUT",
    "PROJECTOR_TELEMETRY_ENABLED",
    "MEDUSA_DISABLE_TELEMETRY",
    "TELEMETRY_ENABLED",
    "ALIBUILD_NO_ANALYTICS",
    "FASTLANE_OPT_OUT_USAGE",
    "COVERITY_CLI_TELEMETRY_OPTOUT",
    "GRADLE_ENTERPRISE_ANALYTICS_DISABLE",
    "LOST_PIXEL_DISABLE_TELEMETRY",
    "DOCKER_SCAN_SUGGEST",
    "KUBEAPT_DISABLE_TELEMETRY",
    "DASH_DISABLE_TELEMETRY",
    "DAGGER_TELEMETRY_DISABLE",
    "NEONKUBE_DISABLE_TELEMETRY",
    "OTTERIZE_TELEMETRY_ENABLED",
    "PORTER_TELEMETRY_ENABLED",
    "PREEVY_DISABLE_TELEMETRY",
    "REPORTPORTAL_CLIENT_JS_NO_ANALYTICS",
    "AGENT_NO_ANALYTICS",
    "BUGGER_OFF",
    "SUGGESTIONS_OPT_OUT",
    "DA_TEST_DISABLE_TELEMETRY",
    "ET_NO_TELEMETRY",
    "LYNX_ANALYTICS",
    "DISABLE_QUICKWIT_TELEMETRY",
    "AUTOMAGICA_NO_TELEMETRY",
    "NETDATA_ANONYMOUS_STATISTICS",
    "TILT_TELEMETRY",
    "MM_LOGSETTINGS_ENABLEDIAGNOSTICS",
    "LS_METRICS_HOST_ENABLED",
    "PANTS_ANONYMOUS_TELEMETRY_ENABLED",
    "FLAGSMITH_TELEMETRY_DISABLED",
    "ONE_CODEX_NO_TELEMETRY",
    "AITOOLSVSCODE_DISABLETELEMETRY",
    "IG_PRO_OPT_OUT",
    "REDOCLY_TELEMETRY",
    "HARDHAT_DISABLE_TELEMETRY_PROMPT",
    "TAOS_TELEMETRY_REPORTING",
    "MF_SEND_TELEMETRY",
    "TELEPORT_ANONYMOUS_TELEMETRY",
    "TUNNELMOLE_TELEMETRY",
    "WG_TELEMETRY_DISABLED",
    "ANYCABLE_DISABLE_TELEMETRY",
    "RIG_TELEMETRY_ENABLED",
    "ELECTRIC_TELEMETRY",
    "SNOWFLAKE_DISABLE_TELEMETRY",
    "OASDIFF_NO_TELEMETRY",
    "EMQX_TELEMETRY__ENABLE",
    "KARATE_TELEMETRY",
    "CONJUR_TELEMETRY_ENABLED",
    "CONJUR_FEATURE_TELEMETRY_ENDPOINT_ENABLED",
    "TAIGA_TELEMETRY_ENABLED",
    "FA_NOTRACK",
    "INFISICAL_TELEMETRY_ENABLED",
    "BOXYHQ_NO_ANALYTICS",
    "ADSERVER_DO_NOT_TRACK",
    "UPSTASH_DISABLE_TELEMETRY",
    "BITRISE_ANALYTICS_DISABLED",
    "AKITA_DISABLE_TELEMETRY",
    "REDGATE_DISABLE_TELEMETRY",
    "FLAKE_CHECKER_NO_TELEMETRY",
    "PP_TOOLS_TELEMETRY_OPTOUT",
    "PIPELINES_TELEMETRY_OPT_OUT",
    "PATCHER_TELEMETRY_OPT_OUT",
    "RETRACED_NO_TELEMETRY",
    "KEYSTONE_TELEMETRY_DISABLED",
    "STP_DISABLE_TELEMETRY",
    "USAGE_DISABLE",
    "ANALYTICS_DISABLED",
    "ORBIT_TELEMETRY_DISABLED",
    "FAL_STATS_ENABLED",
    "LEON_TELEMETRY",
    "AP_TELEMETRY_ENABLED",
    "ITERATIVE_DO_NOT_TRACK",
    "DVC_NO_ANALYTICS",
    "BALENARC_NO_ANALYTICS",
    "DOZZLE_NO_ANALYTICS",
    "RIFF_DISABLE_TELEMETRY",
    "VISTRAILS_USAGE_STATS",
    "MOMENTUM_TELEMETRY_LEVEL",
    "DISABLE_NON_ESSENTIAL_MODEL_CALLS",
    "CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY",
    "CI",
    "DISABLE_TELEMETRY",
    "DISABLETELEMETRY",
    "NO_TELEMETRY",
    "TELEMETRY_DISABLED",
    "COLLECT_TELEMETRY",
    "ALLOW_UI_ANALYTICS",
    "DISABLE_ANALYTICS",
    "ANALYTICS",
    "HOMEBREW_NO_ANALYTICS_THIS_RUN",
];

#[derive(Default)]
struct Results {
    ok: u32,
    fail: u32,
    entries: Vec<(String, bool, String)>, // (module, is_ok, message)
}

impl Results {
    fn add(&mut self, module: &str, is_ok: bool, message: String) {
        if is_ok {
            self.ok += 1;
        } else {
            self.fail += 1;
        }
        self.entries.push((module.to_string(), is_ok, message));
    }
}

fn get_hkey(hkey_str: &str) -> HKEY {
    if hkey_str == "HKLM" { HKEY::LOCAL_MACHINE } else { HKEY::CURRENT_USER }
}

fn read_dword(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<u32> {
    match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
        Ok(RegistryValue::Dword(v)) => Some(v),
        _ => None,
    }
}

fn value_exists(hkey: &HKEY, subkey: &str, value_name: &str) -> bool {
    // Try to read as DWORD first
    if hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD).is_ok() {
        return true;
    }
    // Try as string
    if hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_SZ).is_ok() {
        return true;
    }
    false
}

fn subkey_exists(hkey: &HKEY, subkey: &str) -> bool {
    hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::READ).is_ok()
}

fn read_string(hkey: &HKEY, subkey: &str, value_name: &str) -> Option<String> {
    match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_REG_SZ) {
        Ok(RegistryValue::Sz(s)) => Some(s),
        _ => None,
    }
}

fn get_documents_path() -> PathBuf {
    SHGetKnownFolderPath(&KNOWNFOLDERID::Documents, co::KF::DEFAULT, None)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn main() {
    let mut results = Results::default();

    println!("W11Boost Registry Verification\nChecking registry entries...\n");

    // Check policy values (should NOT exist)
    for (hkey_str, subkey, value_name, module) in POLICY_VALUES_TO_DELETE {
        let hkey = get_hkey(hkey_str);
        let path = format!("{}\\{}\\{}", hkey_str, subkey, value_name);

        if value_exists(&hkey, subkey, value_name) {
            let current = read_dword(&hkey, subkey, value_name)
                .map(|v| v.to_string())
                .or_else(|| read_string(&hkey, subkey, value_name).map(|s| format!("\"{}\"", s)))
                .unwrap_or_else(|| "exists".to_string());
            results.add(module, false, format!("FAIL {}\n       Current: {}  |  Expected: Not present", path, current));
        } else {
            results.add(module, true, format!("OK   {}\n       Not present (expected)", path));
        }
    }

    // Check policy subkeys (should NOT exist)
    for (hkey_str, subkey, module) in POLICY_SUBKEYS_TO_DELETE {
        let hkey = get_hkey(hkey_str);
        let path = format!("{}\\{}", hkey_str, subkey);

        if subkey_exists(&hkey, subkey) {
            results.add(module, false, format!("FAIL {}\n       Subkey exists  |  Expected: Not present", path));
        } else {
            results.add(module, true, format!("OK   {}\n       Not present (expected)", path));
        }
    }

    // Check settings to delete (should NOT exist)
    for (hkey_str, subkey, value_name, module) in SETTINGS_TO_DELETE {
        let hkey = get_hkey(hkey_str);
        let path = format!("{}\\{}\\{}", hkey_str, subkey, value_name);

        if value_exists(&hkey, subkey, value_name) {
            let current = read_dword(&hkey, subkey, value_name)
                .map(|v| v.to_string())
                .or_else(|| read_string(&hkey, subkey, value_name).map(|s| format!("\"{}\"", s)))
                .unwrap_or_else(|| "exists".to_string());
            results.add(module, false, format!("FAIL {}\n       Current: {}  |  Expected: Not present", path, current));
        } else {
            results.add(module, true, format!("OK   {}\n       Not present (expected)", path));
        }
    }

    // Check settings to restore (should match default values)
    for (hkey_str, subkey, value_name, expected, module) in SETTINGS_TO_RESTORE {
        let hkey = get_hkey(hkey_str);
        let path = format!("{}\\{}\\{}", hkey_str, subkey, value_name);

        match read_dword(&hkey, subkey, value_name) {
            Some(current) if current == *expected => {
                results.add(module, true, format!("OK   {}\n       Value: {} (matches default)", path, current));
            }
            Some(current) => {
                results.add(module, false, format!("FAIL {}\n       Current: {}  |  Expected: {} (default)", path, current, expected));
            }
            None => {
                // Value doesn't exist - might be OK for some settings
                results.add(module, false, format!("FAIL {}\n       Not found  |  Expected: {} (default)", path, expected));
            }
        }
    }

    // Check services (should match default Start values)
    for (service_name, expected_start, module) in SERVICES_TO_RESTORE {
        let subkey = format!(r"SYSTEM\CurrentControlSet\Services\{}", service_name);
        let path = format!("HKLM\\{}\\Start", subkey);

        match read_dword(&HKEY::LOCAL_MACHINE, &subkey, "Start") {
            Some(current) if current == *expected_start => {
                let desc = match current {
                    1 => "System",
                    2 => "Auto",
                    3 => "Manual",
                    4 => "Disabled",
                    _ => "Unknown",
                };
                results.add(module, true, format!("OK   {}\n       Value: {} ({}) - matches default", path, current, desc));
            }
            Some(current) => {
                let current_desc = match current {
                    1 => "System",
                    2 => "Auto",
                    3 => "Manual",
                    4 => "Disabled",
                    _ => "Unknown",
                };
                let expected_desc = match *expected_start {
                    1 => "System",
                    2 => "Auto",
                    3 => "Manual",
                    4 => "Disabled",
                    _ => "Unknown",
                };
                results.add(module, false, format!("FAIL {}\n       Current: {} ({})  |  Expected: {} ({})", path, current, current_desc, expected_start, expected_desc));
            }
            None => {
                results.add(module, false, format!("FAIL {}\n       Service not found", path));
            }
        }
    }

    // Check environment variables (should NOT exist)
    let env_path = r"Environment";
    for var_name in ENV_VARS {
        let path = format!("HKCU\\Environment\\{}", var_name);

        if let Some(value) = read_string(&HKEY::CURRENT_USER, env_path, var_name) {
            results.add("disable_telemetry", false, format!("FAIL {}\n       Current: \"{}\"  |  Expected: Not present", path, value));
        } else {
            results.add("disable_telemetry", true, format!("OK   {}\n       Not present (expected)", path));
        }
    }

    // Generate report
    let documents = get_documents_path();
    let timestamp = get_timestamp();
    let log_path = documents.join(format!("registry-dump-{}.log", timestamp));

    let total = results.ok + results.fail;

    // Build report content
    let mut report = String::new();
    report.push_str("==============================================\n");
    report.push_str("W11Boost Registry Verification Report\n");
    report.push_str(&format!("Generated: {}\n", timestamp.replace("_", " ")));
    report.push_str("==============================================\n\n");
    report.push_str(&format!("SUMMARY:\n  Total: {}  |  OK: {}  |  FAIL: {}\n\n", total, results.ok, results.fail));

    if results.fail == 0 {
        report.push_str("STATUS: All registry values match Windows defaults.\n");
        report.push_str("        W11Boost has been successfully removed.\n");
    } else {
        report.push_str("STATUS: Some registry values differ from Windows defaults.\n");
        report.push_str("        W11Boost modifications may still be present.\n");
    }

    report.push_str("\n==============================================\n\n");

    // Group by module
    let mut current_module = String::new();
    for (module, _, message) in &results.entries {
        if module != &current_module {
            if !current_module.is_empty() {
                report.push('\n');
            }
            report.push_str(&format!("[{}]\n", module));
            current_module = module.clone();
        }
        report.push_str(&format!("  {}\n", message));
    }

    report.push_str("\n==============================================\n");
    report.push_str("END OF REPORT\n");
    report.push_str("==============================================\n");

    // Write to file
    match File::create(&log_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(report.as_bytes()) {
                eprintln!("Failed to write report: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create log file: {}", e);
        }
    }

    // Print summary
    println!("SUMMARY:");
    println!("  Total: {}  |  OK: {}  |  FAIL: {}", total, results.ok, results.fail);
    println!();

    if results.fail == 0 {
        println!("All registry values match Windows defaults.");
    } else {
        println!("{} values differ from Windows defaults.", results.fail);
    }

    println!();
    println!("Report saved to: {}", log_path.display());
    println!("\nPress Enter to exit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn get_timestamp() -> String {
    let t = GetLocalTime();
    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        t.wYear, t.wMonth, t.wDay, t.wHour, t.wMinute, t.wSecond
    )
}
