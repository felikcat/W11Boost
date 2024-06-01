#Requires -Version 5 -RunAsAdministrator
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

#region Disable advertising ID for apps.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -Key 'Enabled' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -Key 'AllowTelemetry' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -Key 'MaxTelemetryAllowed' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo' -Key 'DisabledByGroupPolicy' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'AllowDeviceNameInTelemetry' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'AllowTelemetry' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'DisableOneSettingsDownloads' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'DisableTelemetryOptInChangeNotification' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'DoNotShowFeedbackNotifications' -Value '1' -Type 'DWord'
#endregion

# Disallow using your voice for dictation and to talk to Cortana and other apps using Windows' cloud-based speech recognition.
# If online speech recognition is enabled, Microsoft will use your voice data to help improve the speech service.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy' -Key 'HasAccepted' -Value '0' -Type 'DWord'

# Disable telemetry for Tablet PC's handwriting recognition.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\TabletPC' -Key 'PreventHandwritingDataSharing' -Value '1' -Type 'DWord'

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\OneDrive' -Key 'PreventNetworkTrafficPreUserSignIn' -Value '1' -Type 'DWord'


#region [ctfmon.exe] Do not send Microsoft inking and typing data.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput' -Key 'AllowLinguisticDataCollection' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Input\TIPC' -Key 'Enabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\InputPersonalization' -Key 'RestrictImplicitInkCollection' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\InputPersonalization' -Key 'RestrictImplicitTextCollection' -Value '1' -Type 'DWord'
#endregion

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy' -Key 'TailoredExperiencesWithDiagnosticDataEnabled' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Key 'DisableTailoredExperiencesWithDiagnosticData' -Value '1' -Type 'DWord'

# Stops Windows Widgets from running, unless you use a Widget you added.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Dsh' -Key 'AllowNewsAndInterests' -Value '0' -Type 'DWord'

# Remove the Widgets icon from the taskbar.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'TaskbarDa' -Value '0' -Type 'DWord'

# Disable the language list fingerprinting method; useful for bypassing geolocation restrictions.
SetReg -Path 'HKEY_CURRENT_USER\Control Panel\International\User Profile' -Key 'HttpAcceptLanguageOptOut' -Value '1' -Type 'DWord'

#region Fully disable the activity feed.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System' -Key 'EnableActivityFeed' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System' -Key 'PublishUserActivities' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System' -Key 'UploadUserActivities' -Value '0' -Type 'DWord'
#endregion


#region Disable cloud/web usage in the start menu.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search' -Key 'BingSearchEnabled' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search' -Key 'CortanaConsent' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Search' -Key 'CortanaEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsAADCloudSearchEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsMSACloudSearchEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsDeviceSearchHistoryEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Search' -Key 'AllowCloudSearch' -Value '0' -Type 'DWord'

# Search highlights.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsDynamicSearchBoxEnabled' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Search' -Key 'EnableDynamicContentInWSB' -Value '0' -Type 'DWord'

# Web suggestions that occur while typing.
SetReg -Path 'HKEY_CURRENT_USER\Software\Policies\Microsoft\Windows\Explorer' -Key 'DisableSearchBoxSuggestions' -Value '1' -Type 'DWord'
#endregion


#region Disables "Cloud Content" features; stops automatic installation of "suggested" apps, and Microsoft account notifications.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Key 'DisableCloudOptimizedContent' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Key 'DisableConsumerAccountStateContent' -Value '1' -Type 'DWord'

# 310093 = Windows Welcome Experience
# 353696 = Suggested Content in Settings app
# 338387 = Spotlight "Fun Facts"
# 338388 = App Suggestions in the Start Menu
# 338389 = Get tips, tricks, and suggestions as you use Windows

$REGS = @("ContentDeliveryAllowed", "OemPreInstalledAppsEnabled", "PreInstalledAppsEnabled", "PreInstalledAppsEverEnabled", "RotatingLockScreenEnabled", "RotatingLockScreenOverlayEnabled", "SilentInstalledAppsEnabled", "SoftLandingEnabled", "SystemPaneSuggestionsEnabled", "SubscribedContent-310093Enabled", "SubscribedContent-338387Enabled", "SubscribedContent-338388Enabled", "SubscribedContent-338389Enabled", "SubscribedContent-338393Enabled", "SubscribedContent-338394Enabled", "SubscribedContent-338396Enabled", "SubscribedContent-353694Enabled", "SubscribedContent-353696Enabled", "SubscribedContent-88000326Enabled")
$REGS.ForEach({
        SetReg -Path 'HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager' -Key $_ -Value '0' -Type 'DWord'
    })
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\Subscriptions' -Recurse -Force
Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager\SuggestedApps' -Recurse -Force
#endregion


#region Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\AppV\CEIP' -Key 'CEIPEnable' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\SQMClient\Windows' -Key 'CEIPEnable' -Value '0' -Type 'DWord'

# Disable PerfTrack.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}' -Key 'ScenarioExecutionEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Messenger\Client' -Key 'CEIP' -Value '2' -Type 'DWord'

# Disable "Application Impact Telemetry"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'AITEnable' -Value '0' -Type 'DWord'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
#endregion


#region Various Windows Error Reporting tweaks.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'Disabled' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'AutoApproveOSDumps' -Value '0' -Type 'DWord'

# 1 = Minimum consent level; "Always ask before sending data: Windows prompts users for consent to send reports."
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'DefaultConsent' -Value '1' -Type 'DWord'

# Do not allow fully ignoring our custom consent settings.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'DefaultOverrideBehavior' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'DontSendAdditionalData' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Windows Error Reporting' -Key 'LoggingDisabled' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'AllOrNone' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'IncludeKernelFaults' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'IncludeMicrosoftApps' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'IncludeShutdownErrs' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'IncludeWindowsApps' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -Key 'DoReport' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings' -Key 'DisableSendGenericDriverNotFoundToWER' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings' -Key 'DisableSendRequestAdditionalSoftwareToWER' -Value '1' -Type 'DWord'

Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList" -Recurse
Remove-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent" -Recurse

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList' -Key '*' -Value '*' -Type 'String'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting' -Key 'Disabled' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications' -Key '*' -Value '*' -Type 'String'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

# Disable 'Windows Error Reporting' service
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WerSvc' -Key 'Start' -Value '4' -Type 'DWord'
#endregion

# Don't ask to change the current privacy settings after applying a major Windows update.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\OOBE' -Key 'DisablePrivacyExperience' -Value '1' -Type 'DWord'


# Disable 'Connected User Experiences and Telemetry' service
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack' -Key 'ShowedToastAtLevel' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DiagTrack' -Key 'Start' -Value '4' -Type 'DWord'

# Disable 'Diagnostic Policy Service'
# -> Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DPS' -Key 'Start' -Value '4' -Type 'DWord'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

# Sets environment variables to ask various apps to disable or minimize their telemetry.
. "..\Third-party\toptout\toptout_pwsh.ps1" -Env -Exec -ShowLog
