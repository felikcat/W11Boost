Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Personalization\Settings' -ValueName 'AcceptedPrivacyPolicy' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'LimitDiagnosticLogCollection' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -ValueName 'Enabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'AllowDeviceNameInTelemetry' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'AllowTelemetry' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -ValueName 'AllowTelemetry' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection' -ValueName 'MaxTelemetryAllowed' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DisableTelemetryOptInChangeNotification' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DoNotShowFeedbackNotifications' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DisableOneSettingsDownloads' -Data '1' -Type 'Dword'

# Disallow using your voice for dictation and to talk to Cortana and other apps using Windows' cloud-based speech recognition.
# If online speech recognition is enabled, Microsoft will use your voice data to help improve the speech service.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy' -ValueName 'HasAccepted' -Data '0' -Type 'Dword'

# Disable telemetry for Tablet PC's handwriting recognition.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\TabletPC' -ValueName 'PreventHandwritingDataSharing' -Data '1' -Type 'Dword'

# Disable advertising ID for apps.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo' -ValueName 'DisabledByGroupPolicy' -Data '1' -Type 'Dword'

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\OneDrive' -ValueName 'PreventNetworkTrafficPreUserSignIn' -Data '1' -Type 'Dword'

# Ask to not allow execution of experiments by Microsoft.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\PolicyManager\current\device\System' -ValueName 'AllowExperimentation' -Data '0' -Type 'Dword'


##+=+= [ctfmon.exe] Don't send Microsoft inking and typing data.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput' -ValueName 'AllowLinguisticDataCollection' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\InputPersonalization' -ValueName 'RestrictImplicitInkCollection' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\InputPersonalization' -ValueName 'RestrictImplicitTextCollection' -Data '1' -Type 'Dword'

##+=+=


Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Privacy' -ValueName 'TailoredExperiencesWithDiagnosticDataEnabled' -Data '0' -Type 'Dword'

# Stops Windows Widgets from running, unless you use a Widget you added.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Dsh' -ValueName 'AllowNewsAndInterests' -Data '0' -Type 'Dword'

# Remove the Widgets icon from the taskbar.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'TaskbarDa' -Data '0' -Type 'Dword'

# Disallow syncing cellular text messages to Microsoft's servers.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Messaging' -ValueName 'AllowMessageSync' -Data '0' -Type 'Dword'


##+=+= Fully disable the activity feed.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableActivityFeed' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'PublishUserActivities' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'UploadUserActivities' -Data '0' -Type 'Dword'
##+=+=


##+=+= Disable cloud/web usage in the start menu.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsAADCloudSearchEnabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsMSACloudSearchEnabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Windows Search' -ValueName 'AllowCloudSearch' -Data '0' -Type 'Dword'

# Search highlights.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDynamicSearchBoxEnabled' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Windows Search' -ValueName 'EnableDynamicContentInWSB' -Data '0' -Type 'Dword'

# Web suggestions that occur while typing.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Policies\Microsoft\Windows\Explorer' -ValueName 'DisableSearchBoxSuggestions' -Data '1' -Type 'Dword'
##+=+=


##+=+= Disables Cloud Content features; stops automatic installation of advertised ("suggested") apps among others; called "Content Delivery Manager" in Windows 10.

# Disable Consumer Experience:
# - In the start menu, programs that aren't installed ("suggestions") are gone.
# - Microsoft account notifications are gone.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableConsumerAccountStateContent' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableCloudOptimizedContent' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableWindowsConsumerFeatures' -Data '1' -Type 'Dword'

# Specifically for Windows 10; no effect on Windows 11.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager' -ValueName 'SubscribedContent-338389Enabled' -Data '0' -Type 'Dword'
##+=+=


##+=+= Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\AppV\CEIP' -ValueName 'CEIPEnable' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\SQMClient\Windows' -ValueName 'CEIPEnable' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'Disabled' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Messenger\Client' -ValueName 'CEIP' -Data '2' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
##+=+=


##+=+= Various Windows Error Reporting tweaks.

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'AutoApproveOSDumps' -Data '0' -Type 'Dword'

# 1 = Minimum level; "Always ask before sending data: Windows prompts users for consent to send reports."
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'DefaultConsent' -Data '1' -Type 'Dword'
# Don't ignore our custom consent settings.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'DefaultOverrideBehavior' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'DontSendAdditionalData' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\Windows Error Reporting' -ValueName 'LoggingDisabled' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -ValueName 'AllOrNone' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -ValueName 'IncludeKernelFaults' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -ValueName 'IncludeMicrosoftApps' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -ValueName 'IncludeShutdownErrs' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting' -ValueName 'IncludeWindowsApps' -Data '0' -Type 'Dword'

reg.exe delete "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList" /f

reg.exe delete "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\Consent" /f

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList' -ValueName '*' -Data '*' -Type 'String'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications' -ValueName '*' -Data '*' -Type 'String'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"
##+=+=

# Windows Error Reporting
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WerSvc" /v "Start" /t REG_DWORD /d 4 /f
# Connected User Experiences and Telemetry
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DiagTrack" /v "Start" /t REG_DWORD /d 4 /f
# Diagnostic Policy Service
# Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DPS" /v "Start" /t REG_DWORD /d 4 /f

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"