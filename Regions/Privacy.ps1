#Requires -Version 5 -RunAsAdministrator

PEAdd_HKCU 'Software\Microsoft\Personalization\Settings' -Name 'AcceptedPrivacyPolicy' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'LimitDiagnosticLogCollection' -Value '1' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Name 'Enabled' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'AllowDeviceNameInTelemetry' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'AllowTelemetry' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -Name 'AllowTelemetry' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -Name 'MaxTelemetryAllowed' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'DisableTelemetryOptInChangeNotification' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'DoNotShowFeedbackNotifications' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Name 'DisableOneSettingsDownloads' -Value '1' -Type 'Dword'

# Disallow using your voice for dictation and to talk to Cortana and other apps using Windows' cloud-based speech recognition.
# If online speech recognition is enabled, Microsoft will use your voice data to help improve the speech service.
PEAdd_HKCU 'Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy' -Name 'HasAccepted' -Value '0' -Type 'Dword'

# Disable telemetry for Tablet PC's handwriting recognition.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\TabletPC' -Name 'PreventHandwritingDataSharing' -Value '1' -Type 'Dword'

# Disable advertising ID for apps.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo' -Name 'DisabledByGroupPolicy' -Value '1' -Type 'Dword'

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
PEAdd_HKLM 'SOFTWARE\Microsoft\OneDrive' -Name 'PreventNetworkTrafficPreUserSignIn' -Value '1' -Type 'Dword'

# Ask to not allow execution of experiments by Microsoft.
PEAdd_HKLM 'SOFTWARE\Microsoft\PolicyManager\current\device\System' -Name 'AllowExperimentation' -Value '0' -Type 'Dword'


#region [ctfmon.exe] Do not send Microsoft inking and typing data.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput' -Name 'AllowLinguisticDataCollection' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\InputPersonalization' -Name 'RestrictImplicitInkCollection' -Value '1' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\InputPersonalization' -Name 'RestrictImplicitTextCollection' -Value '1' -Type 'Dword'
#endregion


PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Privacy' -Name 'TailoredExperiencesWithDiagnosticDataEnabled' -Value '0' -Type 'Dword'

# Stops Windows Widgets from running, unless you use a Widget you added.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Dsh' -Name 'AllowNewsAndInterests' -Value '0' -Type 'Dword'

# Remove the Widgets icon from the taskbar.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'TaskbarDa' -Value '0' -Type 'Dword'


#region Fully disable the activity feed.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'EnableActivityFeed' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'PublishUserActivities' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'UploadUserActivities' -Value '0' -Type 'Dword'
#endregion


#region Disable cloud/web usage in the start menu.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Search' -Name 'BingSearchEnabled' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Search' -Name 'CortanaConsent' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Search' -Name 'CortanaEnabled' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Name 'IsAADCloudSearchEnabled' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Name 'IsMSACloudSearchEnabled' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Name 'IsDeviceSearchHistoryEnabled' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Windows Search' -Name 'AllowCloudSearch' -Value '0' -Type 'Dword'

# Search highlights.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Name 'IsDynamicSearchBoxEnabled' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Windows Search' -Name 'EnableDynamicContentInWSB' -Value '0' -Type 'Dword'

# Web suggestions that occur while typing.
PEAdd_HKCU 'Software\Policies\Microsoft\Windows\Explorer' -Name 'DisableSearchBoxSuggestions' -Value '1' -Type 'Dword'
#endregion


#region Disables "Cloud Content" features; stops automatic installation of "suggested" apps, and Microsoft account notifications.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Name 'DisableCloudOptimizedContent' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Name 'DisableConsumerAccountStateContent' -Value '1' -Type 'Dword'

$REGS = @("ContentDeliveryAllowed", "OemPreInstalledAppsEnabled", "PreInstalledAppsEnabled", "PreInstalledAppsEverEnabled", "RotatingLockScreenEnabled", "RotatingLockScreenOverlayEnabled", "SilentInstalledAppsEnabled", "SoftLandingEnabled", "SystemPaneSuggestionsEnabled", "SubscribedContent-338389Enabled", "SubscribedContent-338393Enabled", "SubscribedContent-338394Enabled", "SubscribedContent-338396Enabled", "SubscribedContent-353694Enabled", "SubscribedContent-353696Enabled", "SubscribedContent-88000326Enabled")
$REGS.ForEach({
    PEAdd_HKLM 'Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager' -Name $_ -Value '0' -Type 'Dword'
})
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\Subscriptions' -Recurse -Force
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\SuggestedApps' -Recurse -Force
#endregion


#region Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\AppV\CEIP' -Name 'CEIPEnable' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\SQMClient\Windows' -Name 'CEIPEnable' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'Disabled' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Messenger\Client' -Name 'CEIP' -Value '2' -Type 'Dword'

# Disable "Application Impact Telemetry"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'AITEnable' -Value '0' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
#endregion


#region Various Windows Error Reporting tweaks.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'AutoApproveOSDumps' -Value '0' -Type 'Dword'

# 1 = Minimum consent level; "Always ask before sending data: Windows prompts users for consent to send reports."
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'DefaultConsent' -Value '1' -Type 'Dword'

# Do not allow fully ignoring our custom consent settings.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'DefaultOverrideBehavior' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'DontSendAdditionalData' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Name 'LoggingDisabled' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Name 'AllOrNone' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Name 'IncludeKernelFaults' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Name 'IncludeMicrosoftApps' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Name 'IncludeShutdownErrs' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Name 'IncludeWindowsApps' -Value '0' -Type 'Dword'

Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList" -Recurse
Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent" -Recurse

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList' -Name '*' -Value '*' -Type 'String'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications' -Name '*' -Value '*' -Type 'String'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

# Disable 'Windows Error Reporting' service
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Services\WerSvc' -Name 'Start' -Value '4' -Type 'Dword'
#endregion


# Disable 'Connected User Experiences and Telemetry' service
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Services\DiagTrack' -Name 'Start' -Value '4' -Type 'Dword'

# Disable 'Diagnostic Policy Service'
# -> Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Services\DPS' -Name 'Start' -Value '4' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

# Third-party script that tells various apps to disable or minimize their telemetry.
Download_File 'https://raw.githubusercontent.com/beatcracker/toptout/master/examples/toptout_pwsh.ps1' -Destination .\

. ".\toptout_pwsh.ps1" -Env -Exec -ShowLog
