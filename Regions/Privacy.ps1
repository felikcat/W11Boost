#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

#region RustDesk and AnyDesk are better alternatives.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Personalization\Settings', 'AcceptedPrivacyPolicy', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'LimitDiagnosticLogCollection', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Remote Assistance', 'fAllowToGetHelp', '0', [RegistryValueKind]::DWord)
#endregion

#region Disable advertising ID for apps.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo', 'Enabled', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection', 'AllowTelemetry', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection', 'MaxTelemetryAllowed', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo', 'DisabledByGroupPolicy', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'AllowDeviceNameInTelemetry', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'AllowTelemetry', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'DisableOneSettingsDownloads', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'DisableTelemetryOptInChangeNotification', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection', 'DoNotShowFeedbackNotifications', '1', [RegistryValueKind]::DWord)
#endregion

# Disallow using your voice for dictation and to talk to Cortana and other apps using Windows' cloud-based speech recognition.
# If online speech recognition is enabled, Microsoft will use your voice data to help improve the speech service.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy', 'HasAccepted', '0', [RegistryValueKind]::DWord)

# Disable telemetry for Tablet PC's handwriting recognition.
New-Item 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\TabletPC'
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\TabletPC', 'PreventHandwritingDataSharing', '1', [RegistryValueKind]::DWord)

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\OneDrive', 'PreventNetworkTrafficPreUserSignIn', '1', [RegistryValueKind]::DWord)


#region [ctfmon.exe] Do not send Microsoft inking and typing data.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput', 'AllowLinguisticDataCollection', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Input\TIPC', 'Enabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\InputPersonalization', 'RestrictImplicitInkCollection', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\InputPersonalization', 'RestrictImplicitTextCollection', '1', [RegistryValueKind]::DWord)
#endregion

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy', 'TailoredExperiencesWithDiagnosticDataEnabled', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent', 'DisableTailoredExperiencesWithDiagnosticData', '1', [RegistryValueKind]::DWord)

# Stops Windows Widgets from running, unless you use a Widget you added.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Dsh', 'AllowNewsAndInterests', '0', [RegistryValueKind]::DWord)

# Remove the Widgets icon from the taskbar.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'TaskbarDa', '0', [RegistryValueKind]::DWord)

# Disable the language list fingerprinting method; useful for bypassing geolocation restrictions.
[Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\International\User Profile', 'HttpAcceptLanguageOptOut', '1', [RegistryValueKind]::DWord)

#region Fully disable the activity feed.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'EnableActivityFeed', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'PublishUserActivities', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'UploadUserActivities', '0', [RegistryValueKind]::DWord)
#endregion


#region Disable cloud/web usage in the start menu.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search', 'BingSearchEnabled', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search', 'CortanaConsent', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search', 'CortanaEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsAADCloudSearchEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsMSACloudSearchEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsDeviceSearchHistoryEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Search', 'AllowCloudSearch', '0', [RegistryValueKind]::DWord)

# Search highlights.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsDynamicSearchBoxEnabled', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Search', 'EnableDynamicContentInWSB', '0', [RegistryValueKind]::DWord)

# Web suggestions that occur while typing.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Policies\Microsoft\Windows\Explorer', 'DisableSearchBoxSuggestions', '1', [RegistryValueKind]::DWord)
#endregion


#region Disables "Cloud Content" features; stops automatic installation of "suggested" apps, and Microsoft account notifications.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent', 'DisableCloudOptimizedContent', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent', 'DisableConsumerAccountStateContent', '1', [RegistryValueKind]::DWord)

# 310093 = Windows Welcome Experience
# 353696 = Suggested Content in Settings app
# 338387 = Spotlight "Fun Facts"
# 338388 = App Suggestions in the Start Menu
# 338389 = Get tips, tricks, and suggestions as you use Windows

$REGS = @("ContentDeliveryAllowed", "OemPreInstalledAppsEnabled", "PreInstalledAppsEnabled", "PreInstalledAppsEverEnabled", "RotatingLockScreenEnabled", "RotatingLockScreenOverlayEnabled", "SilentInstalledAppsEnabled", "SoftLandingEnabled", "SystemPaneSuggestionsEnabled", "SubscribedContent-310093Enabled", "SubscribedContent-338387Enabled", "SubscribedContent-338388Enabled", "SubscribedContent-338389Enabled", "SubscribedContent-338393Enabled", "SubscribedContent-338394Enabled", "SubscribedContent-338396Enabled", "SubscribedContent-353694Enabled", "SubscribedContent-353696Enabled", "SubscribedContent-88000326Enabled")
$REGS.ForEach({
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager', $_, '0', [RegistryValueKind]::DWord)
})
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\Subscriptions' -Recurse -Force
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\SuggestedApps' -Recurse -Force
#endregion


#region Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\AppV\CEIP', 'CEIPEnable', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\SQMClient\Windows', 'CEIPEnable', '0', [RegistryValueKind]::DWord)

# Disable PerfTrack.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}', 'ScenarioExecutionEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Messenger\Client', 'CEIP', '2', [RegistryValueKind]::DWord)

# Disable "Application Impact Telemetry"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'AITEnable', '0', [RegistryValueKind]::DWord)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
#endregion


#region Various Windows Error Reporting tweaks.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'Disabled', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'AutoApproveOSDumps', '0', [RegistryValueKind]::DWord)

# 1 = Minimum consent level; "Always ask before sending data: Windows prompts users for consent to send reports."
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'DefaultConsent', '1', [RegistryValueKind]::DWord)

# Do not allow fully ignoring our custom consent settings.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'DefaultOverrideBehavior', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'DontSendAdditionalData', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting', 'LoggingDisabled', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'AllOrNone', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'IncludeKernelFaults', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'IncludeMicrosoftApps', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'IncludeShutdownErrs', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'IncludeWindowsApps', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting', 'DoReport', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings', 'DisableSendGenericDriverNotFoundToWER', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings', 'DisableSendRequestAdditionalSoftwareToWER', '1', [RegistryValueKind]::DWord)

Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList" -Recurse
Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent" -Recurse

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList', '*', '*', [RegistryValueKind]::String)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting', 'Disabled', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications', '*', '*', [RegistryValueKind]::String)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

# Disable 'Windows Error Reporting' service
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WerSvc', 'Start', '4', [RegistryValueKind]::DWord)
#endregion

# Don't ask to change the current privacy settings after applying a major Windows update.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\OOBE', 'DisablePrivacyExperience', '1', [RegistryValueKind]::DWord)


# Disable 'Connected User Experiences and Telemetry' service
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack', 'ShowedToastAtLevel', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DiagTrack', 'Start', '4', [RegistryValueKind]::DWord)

# Disable 'Diagnostic Policy Service'
# -> Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DPS', 'Start', '4', [RegistryValueKind]::DWord)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

# Sets environment variables to ask various apps to disable or minimize their telemetry.
. "..\Third-party\toptout\toptout_pwsh.ps1" -Env -Exec -ShowLog
