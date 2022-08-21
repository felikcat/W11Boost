@echo off
title W11Boost by https://github.com/nermur

REM Disables Sticky, Filter, and Toggle Keys.
set /A avoid_key_annoyances=1

REM Ensures Windows' audio ducking/attenuation is disabled.
set /A disable_audio_reduction=0

REM Undermines software that clear the clipboard automatically.
set /A disable_clipboard_history=1

REM Use NVIDIA ShadowPlay, AMD ReLive, or OBS Studio instead.
set /A disable_game_dvr=1

REM Disables GPS services, which always run even if there's no GPS hardware installed.
set /A disable_geolocation=0

REM Routing through IPv6 is worse than IPv4 in some areas (higher latency/ping).
set /A disable_ipv6=0

REM Set to '1' if using a CPU that supports: https://en.wikipedia.org/wiki/Intel_5-level_paging
set /A disable_la57_cleanup=0

REM Printers are heavily exploitable, avoid using one if possible.
set /A disable_printer_support=0

REM If you don't want to install apps using the Microsoft Store from other devices or a web browser.
set /A disable_remote_msstore_installs=1

REM Disable Explorer's thumbnail border shadows.
set /A disable_thumbnail_shadows=0

REM Disables power saving features for network switches to increase their reliability.
set /A network_adapter_tweaks=1

REM Disables all security mitigations; drastically improves performance for older CPUs (such as an Intel i7-4790K).
REM Set to 1 by default simply cause there isn't solid evidence to Spectre and other related vulnerabilities being exploited on non-businesses in the wild.
set /A no_mitigations=1

REM Makes disks using the default file system (NTFS) faster, but disables File History and File Access Dates.
set /A ntfs_tweaks=1

REM "Everything" can circumvent the performance impact of Windows Search Indexing while providing faster and more accurate results.
set /A replace_windows_search=0


reg.exe query HKU\S-1-5-19 || (
	echo ==== Error ====
	echo Right click on this file and select 'Run as administrator'
	echo Press any key to exit...
	Pause>nul
	exit /b
)

REM If these are disabled, Windows Update will break and so will this script.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\AppXSvc" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\ClipSVC" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\TokenBroker" /v "Start" /t REG_DWORD /d 3 /f
REM Specifically breaks Windows Store if disabled previously.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\StorSvc" /v "Start" /t REG_DWORD /d 3 /f
sc.exe start AppXSvc
sc.exe start ClipSVC
sc.exe start StorSvc

cls
echo.
echo ==== Current settings ====
echo.
echo avoid_key_annoyances = %avoid_key_annoyances%
echo disable_audio_reduction = %disable_audio_reduction%
echo disable_clipboard_history = %disable_clipboard_history%
echo disable_game_dvr = %disable_game_dvr%
echo disable_geolocation = %disable_geolocation%
echo disable_ipv6 = %disable_ipv6%
echo disable_la57_cleanup = %disable_la57_cleanup%
echo disable_printer_support = %disable_printer_support%
echo disable_remote_msstore_installs = %disable_remote_msstore_installs%
echo disable_thumbnail_shadows = %disable_thumbnail_shadows%
echo network_adapter_tweaks = %network_adapter_tweaks%
echo no_mitigations = %no_mitigations%
echo ntfs_tweaks = %ntfs_tweaks%
echo replace_windows_search = %replace_windows_search%
echo.
Pause

cd %~dp0

REM Won't make a restore point if there's already one within the past 24 hours.
WMIC.exe /Namespace:\\root\default Path SystemRestore Call CreateRestorePoint "W11Boost", 100, 7

REM Sleep mode achieves the same goal without hammering the primary disk; downside: breaks if power cuts off; regardless, leaving a PC unattended is bad.
powercfg.exe /hibernate on
REM "Fast startup" causes stability issues, increases disk wear (from excessive I/O usage), and can drastically increase shutdown times on slow disks. 
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Power" /V HiberbootEnabled /T REG_DWORD /D 0 /F
attrib +R %WinDir%\System32\SleepStudy\UserNotPresentSession.etl

if %avoid_key_annoyances%==1 (
	reg.exe add "HKCU\Control Panel\Accessibility\StickyKeys" /v "Flags" /t REG_SZ /d 50 /f
	reg.exe add "HKCU\Control Panel\Accessibility\ToggleKeys" /v "Flags" /t REG_SZ /d 58 /f
	reg.exe add "HKCU\Control Panel\Accessibility\Keyboard Response" /v "Flags" /t REG_SZ /d 122 /f
)

if %disable_audio_reduction%==1 (
	reg.exe add "HKCU\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
	reg.exe delete "HKCU\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
)

if %disable_clipboard_history%==1 (
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v "AllowClipboardHistory" /t REG_DWORD /d 0 /f
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v "AllowCrossDeviceClipboard" /t REG_DWORD /d 0 /f
)

if %disable_game_dvr%==1 (
	reg.exe add "HKCU\System\GameConfigStore" /v "GameDVR_Enabled" /t REG_DWORD /d 0 /f
	reg.exe add "HKLM\Software\Policies\Microsoft\Windows\GameDVR" /v "AllowGameDVR" /t REG_DWORD /d 0 /f
	reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR" /v "AppCaptureEnabled" /t REG_DWORD /d 0 /f
	reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\GameDVR" /v "HistoricalCaptureEnabled" /t REG_DWORD /d 0 /f
	reg.exe add "HKCU\Software\Microsoft\GameBar" /v "UseNexusForGameBarEnabled" /t REG_DWORD /d 0 /f
	reg.exe add "HKCU\Software\Microsoft\GameBar" /v "ShowStartupPanel" /t REG_DWORD /d 0 /f
)

if %disable_geolocation%==1 (
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableLocation" /t REG_DWORD /d 1 /f
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableLocationScripting" /t REG_DWORD /d 1 /f
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableWindowsLocationProvider" /t REG_DWORD /d 1 /f
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\lfsvc" /v "Start" /t REG_DWORD /d 4 /f
	sc.exe stop lfsvc
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Location\Notifications"
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Location\WindowsActionDialog"
)

if %disable_ipv6%==1 (
	sc.exe stop iphlpsvc
	sc.exe stop IpxlatCfgSvc
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v Start /t REG_DWORD /d 4 /f
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v Start /t REG_DWORD /d 4 /f
	reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce" /v "disable_ipv6" /t REG_SZ /f /d "powershell -Command Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 0"
)

if %disable_la57_cleanup%==1 (
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Kernel\La57Cleanup"
)

if %disable_printer_support%==1 (
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\Spooler" /v "Start" /t REG_DWORD /d 4 /f
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\PrintNotify" /v "Start" /t REG_DWORD /d 4 /f
)

if %disable_remote_msstore_installs%==1 (
	reg.exe add "HKLM\Software\Policies\Microsoft\PushToInstall" /v "DisablePushToInstall" /t REG_DWORD /d "1" /f
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\PushToInstall\Registration"
)

if %disable_thumbnail_shadows%==1 (
	reg.exe add "HKCR\SystemFileAssociations\image" /v "Treatment" /t REG_DWORD /d 0 /f
	reg.exe add "HKCR\SystemFileAssociations\image" /v "TypeOverlay" /t REG_SZ /d "" /f
)

if %network_adapter_tweaks%==1 (
	powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""network_adapter_tweaks.ps1""' -Verb RunAs}"
)

if %no_mitigations%==1 (
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" /v FeatureSettingsOverride /t REG_DWORD /d 3 /f 
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" /v FeatureSettingsOverrideMask /t REG_DWORD /d 3 /f
	REM Use the faster but less secure Hyper-V scheduler.
	bcdedit.exe /set hypervisorschedulertype classic
	REM Allow Intel TSX.
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Kernel" /v DisableTsx /t REG_DWORD /d 0 /f
	powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""set_exploit_mitigations.ps1""' -Verb RunAs}"
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\CredentialGuard" /v "Enabled" /t REG_DWORD /d 0 /f
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity" /v "Enabled" /t REG_DWORD /d 0 /f
)

if %ntfs_tweaks%==1 (
	fsutil.exe behavior set disablelastaccess 1
	fsutil.exe behavior set encryptpagingfile 0
	reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\FileHistory" /v "Disabled" /t REG_DWORD /d 1 /f
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\I/O System" /v "IoBlockLegacyFsFilters" /t REG_DWORD /d 1 /f
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
)

if %replace_windows_search%==1 (
	sc.exe stop WSearch
	reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\WSearch" /v Start /t REG_DWORD /d 4 /f
	winget.exe install voidtools.Everything -eh
	winget.exe install stnkl.EverythingToolbar -eh
)

reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoLowDiskSpaceChecks" /t REG_DWORD /d 1 /f
reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "LinkResolveIgnoreLinkInfo" /t REG_DWORD /d 1 /f

REM Don't search disks to attempt fixing a missing shortcut.
reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveSearch" /t REG_DWORD /d 1 /f

REM Don't search all paths related to the missing shortcut.
reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveTrack" /t REG_DWORD /d 1 /f

REM Don't waste time removing thumbnail caches; if they corrupt themselves, the user will do this themself.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache" /v "Autorun" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache" /v "Autorun" /t REG_DWORD /d 0 /f

REM Don't check for an active connection through Microsoft's servers.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet" /v EnableActiveProbing /t REG_DWORD /d 0 /f

REM Ask OneDrive to only generate network traffic if signed in to OneDrive.
reg.exe add "HKLM\SOFTWARE\Microsoft\OneDrive" /v "PreventNetworkTrafficPreUserSignIn" /t REG_DWORD /d 1 /f

REM Don't allow automatic: software updates, security scanning, and system diagnostics.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance" /v "MaintenanceDisabled" /t REG_DWORD /d 1 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Diagnosis\Scheduled"

REM Ask nicely to stop sending diagnostic data to Microsoft.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection" /v "AllowTelemetry" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy" /v "TailoredExperiencesWithDiagnosticDataEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "AllowTelemetry" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "DisableEnterpriseAuthProxy" /t REG_DWORD /d 1 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "DisableOneSettingsDownloads" /t REG_DWORD /d 1 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "DisableTelemetryOptInChangeNotification" /t REG_DWORD /d 1 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "DoNotShowFeedbackNotifications" /t REG_DWORD /d 1 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v "PublishUserActivities" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Policies\DataCollection" /v "AllowTelemetry" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SYSTEM\ControlSet001\Control\WMI\Autologger\AutoLogger-Diagtrack-Listener" /v "Start" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SYSTEM\ControlSet001\Services\DiagTrack" /v "Start" /t REG_DWORD /d 4 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Feedback\Siuf\DmClient"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

REM Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
reg.exe add "HKLM\SOFTWARE\Microsoft\SQMClient\Windows" /v "CEIPEnable" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\AppV\CEIP" /v "CEIPEnable" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Messenger\Client" /v "CEIP" /t REG_DWORD /d 2 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Autochk\Proxy"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

REM Disable "Application Compatibility Engine".
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisableEngine" /t REG_DWORD /d 1 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\ProgramDataUpdater"

REM Disable "Application Telemetry".
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "AITEnable" /t REG_DWORD /d 0 /f
REM Disable "Program Compatibility Assistant".
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisablePCA" /t REG_DWORD /d 1 /f
REM Disable "SwitchBack Compatibility Engine".
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "SbEnable" /t REG_DWORD /d 0 /f
REM Disable user steps recorder.
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisableUAR" /t REG_DWORD /d 1 /f

REM Disable Autoplay on all disk types.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoDriveTypeAutoRun" /t REG_DWORD /d 255 /f

REM Disable WER (Windows Error Reporting).
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting" /v "Disabled" /t REG_DWORD /d 1 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting" /v "AutoApproveOSDumps" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting" /v "DontSendAdditionalData" /t REG_DWORD /d 1 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

REM Ask nicely to not allow execution of experiments by Microsoft.
reg.exe add "HKLM\SOFTWARE\Microsoft\PolicyManager\current\device\System" /v "AllowExperimentation" /t REG_DWORD /d 0 /f

REM Disable tracking of application startups.
reg.exe add "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v "Start_TrackProgs" /t REG_DWORD /d 0 /f

REM Disable all Content Delivery Manager features, which stops automatic installation of advertised apps among others.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "ContentDeliveryAllowed" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "FeatureManagementEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "OemPreInstalledAppsEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "PreInstalledAppsEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "PreInstalledAppsEverEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "RemediationRequired" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "RotatingLockScreenEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "RotatingLockScreenOverlayEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "SilentInstalledAppsEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "SoftLandingEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager" /v "SystemPaneSuggestionsEnabled" /t REG_DWORD /d 0 /f

REM Disable SmartScreen, it delays the launch of software and is better done by other anti-malware software (like Kaspersky).
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\System" /v "EnableSmartScreen" /t REG_DWORD /d 0 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer" /v "SmartScreenEnabled" /t REG_SZ /d "Off" /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost" /v "EnableWebContentEvaluation" /t REG_DWORD /d 0 /f

REM Disable legacy PowerShell.
powershell.exe -Command "Disable-WindowsOptionalFeature -NoRestart -Online -FeatureName "MicrosoftWindowsPowerShellV2Root""
powershell.exe -Command "Disable-WindowsOptionalFeature -NoRestart -Online -FeatureName "MicrosoftWindowsPowerShellV2""

REM Increasing overall system/DPC latency for the sake of minimal power saving is bad.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Power\PowerThrottling" /v "PowerThrottlingOff" /t REG_DWORD /d 1 /f

REM Automated file cleanup (without user interaction) is a bad idea; Storage Sense only runs on low-disk space events.
reg.exe add "HKLM\SOFTWARE\Policies\Microsoft\Windows\StorageSense" /v "AllowStorageSenseGlobal" /t REG_DWORD /d 0 /f
reg.exe delete "HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense" /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\DiskFootprint\Diagnostics"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\DiskFootprint\StorageSense"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\DiskCleanup\SilentCleanup"

REM Disable these scheduler tasks to keep performance and bandwidth usage more consistent.
schtasks.exe /Change /DISABLE /TN "\Microsoft\Office\OfficeTelemetryAgentFallBack"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Office\OfficeTelemetryAgentLogOn"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\AppID\SmartScreenSpecific"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\StartupAppTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\ApplicationData\DsSvcCleanup"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\AppxDeploymentClient\Pre-staged app cleanup"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\CertificateServicesClient\UserTask-Roam"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Clip\License Validation"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\CloudExperienceHost\CreateObjectTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\File Classification Infrastructure\Property Definition Sync"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\HelloFace\FODCleanupTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\InstallService\ScanForUpdates"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\InstallService\ScanForUpdatesAsUser"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\InstallService\SmartRetry"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\International\Synchronize Language Settings"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Maintenance\WinSAT"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Maps\MapsToastTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Maps\MapsUpdateTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Mobile Broadband Accounts\MNO Metadata Parser"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\MUI\LPRemove"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Multimedia\SystemSoundsService"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\NetTrace\GatherNetworkInfo"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\PI\Sqm-Tasks"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Printing\EduPrintProv"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Ras\MobilityManager"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\RecoveryEnvironment\VerifyWinRE"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Shell\FamilySafetyMonitor"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Shell\FamilySafetyRefreshTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTaskLogon"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTaskNetwork"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\SoftwareProtectionPlatform\SvcTrigger"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Speech\SpeechModelDownloadTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Sysmain\ResPriStaticDbSync"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Sysmain\WsSwapAssessmentTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan Static Task"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\UpdateOrchestrator\UpdateModelTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\UpdateOrchestrator\USO_UxBroker"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\USB\Usb-Notifications"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WDI\ResolutionHost"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Windows Filtering Platform\BfeOnServiceStartTypeChange"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WindowsUpdate\sih"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WlanSvc\CDSSync"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WOF\WIM-Hash-Management"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WOF\WIM-Hash-Validation"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Work Folders\Work Folders Logon Synchronization"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Work Folders\Work Folders Maintenance Work"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WS\WSTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\WwanSvc\OobeDiscovery"

REM Disable "Delivery Optimization".
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\DoSvc" /v Start /t REG_DWORD /d 4 /f

REM Disables "Diagnostic Policy Service"; logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\DPS" /v Start /t REG_DWORD /d 4 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Services\PcaSvc" /v Start /t REG_DWORD /d 4 /f

REM Don't analyze programs' execution time data.
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib" /v "Disable Performance Counters" /t REG_DWORD /d 1 /f

REM == Clean up mistakes modded Windows ISOs and other optimizer scripts make ==

REM Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

REM MemoryCompression: Reduces I/O load and prevents an Out Of Memory situation, akin to Linux's zRAM.
powershell.exe -Command "Enable-MMAgent -ApplicationLaunchPrefetching -ApplicationPreLaunch -MemoryCompression"

REM Delaying the startup of third-party programs gives Windows more room to breathe for its own jobs, speeding up the overall startup time.
reg.exe delete "HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize" /v "Startupdelayinmsec" /f

REM Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2

REM == GROUP END ==

REM Don't draw graphical elements for boot (spinner, Windows or BIOS logo, etc).
bcdedit.exe /set bootuxdisabled on

REM A worthless security measure, just use BitLocker.
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" /v ClearPageFileAtShutdown /t REG_DWORD /d 0 /f

REM Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

REM Decrease shutdown time.
reg.exe add "HKCU\Control Panel\Desktop" /v WaitToKillAppTimeOut /t REG_SZ /d 2000 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control" /v WaitToKillServiceTimeout /t REG_SZ /d 2000 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control" /v HungAppTimeout /t REG_SZ /d 2000 /f
reg.exe add "HKLM\SYSTEM\CurrentControlSet\Control" /v AutoEndTasks /t REG_SZ /d 1 /f
reg.exe add "HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" /v DisableShutdownNamedPipe /t REG_DWORD /d 1 /f

taskkill.exe /IM explorer.exe /F
start explorer.exe
echo.
echo Your PC will restart after a key is pressed; required to fully apply changes
echo.
Pause
shutdown.exe /r /t 00