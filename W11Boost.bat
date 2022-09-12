@echo off
title W11Boost by https://github.com/nermur

REM Disables Sticky, Filter, and Toggle Keys.
set /A avoid_key_annoyances=1

REM 0 = disables File History.
set /A file_history=1

REM Ensures Windows' audio ducking/attenuation is disabled.
set /A no_audio_reduction=0

REM Undermines software that clear the clipboard automatically.
set /A no_clipboard_history=1

REM Prevents random packet loss/drop-outs in exchange for a higher battery drain.
set /A no_ethernet_power_saving=1

REM Use NVIDIA ShadowPlay, AMD ReLive, or OBS Studio instead.
set /A no_game_dvr=1

REM Disables GPS services, which always run even if there's no GPS hardware installed.
set /A no_geolocation=0

REM Disables all non-essential security mitigations; drastically improves performance for older CPUs (such as an Intel i7-4790K).
set /A no_mitigations=1

REM NOTICE: Some settings set might intersect with WiFi adapters, as tweaks are applied to all network interfaces.
set /A recommended_ethernet_tweaks=1

REM "Everything" can circumvent the performance impact of Windows Search Indexing while providing faster and more accurate results.
set /A replace_windows_search=0

REM See https://www.startallback.com/ for more information.
set /A replace_windows11_interface=0

REM Resets all network interfaces back to their manufacturer's default settings.
REM Recommended before applying our network tweaks, as it's a "clean slate".
set /A reset_network_interface_settings=1

REM 0 = disable Explorer's thumbnail (images/video previews) border shadows.
set /A thumbnail_shadows=1


reg.exe query HKU\S-1-5-19 || (
	echo ==== Error ====
	echo Right click on this file and select 'Run as administrator'
	echo Press any key to exit...
	Pause>nul
	exit /b
)

REM If these are disabled, Windows Update will break and so will this script.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\AppXSvc" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ClipSVC" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\TokenBroker" /v "Start" /t REG_DWORD /d 3 /f
REM StorSvc being disabled breaks the Windows Store.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\StorSvc" /v "Start" /t REG_DWORD /d 3 /f
sc.exe start AppXSvc
sc.exe start ClipSVC
sc.exe start StorSvc

cls
echo.
echo ==== Current settings ====
echo.
echo avoid_key_annoyances = %avoid_key_annoyances%
echo file_history = %file_history%
echo no_audio_reduction = %no_audio_reduction%
echo no_clipboard_history = %no_clipboard_history%
echo no_ethernet_power_saving = %no_ethernet_power_saving%
echo no_game_dvr = %no_game_dvr%
echo no_geolocation = %no_geolocation%
echo no_mitigations = %no_mitigations%
echo recommended_ethernet_tweaks = %recommended_ethernet_tweaks%
echo replace_windows_search = %replace_windows_search%
echo replace_windows11_interface = %replace_windows11_interface%
echo reset_network_interface_settings = %reset_network_interface_settings%
echo thumbnail_shadows = %thumbnail_shadows%
echo.
Pause

cd %~dp0

REM Won't make a restore point if there's already one within the past 24 hours.
WMIC.exe /Namespace:\\root\default Path SystemRestore Call CreateRestorePoint "W11Boost by nermur", 100, 7

REM "Fast startup" causes stability issues, and increases disk wear from excessive I/O usage.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power" /V "HiberbootEnabled" /T REG_DWORD /D 0 /F
attrib +R %WinDir%\System32\SleepStudy\UserNotPresentSession.etl

if %avoid_key_annoyances%==1 (
	reg.exe import Registry\Optional\avoid_key_annoyances.reg
)

if %file_history%==1 (
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\FileHistory" /v "Disabled" /t REG_DWORD /d 0 /f
	schtasks.exe /Change /ENABLE /TN "\Microsoft\Windows\FileHistory\File History (maintenance mode)" 
) else (
	if %file_history%==0 (
		reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\FileHistory" /v "Disabled" /t REG_DWORD /d 1 /f
		schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
	)
)

if %no_audio_reduction%==1 (
	reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
	reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
)

if %no_clipboard_history%==1 (
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "AllowClipboardHistory" /t REG_DWORD /d 0 /f
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "AllowCrossDeviceClipboard" /t REG_DWORD /d 0 /f
)

if %no_ethernet_power_saving%==1 (
	powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""Networking\Ethernet\no_power_saving.ps1""' -Verb RunAs}"
)

if %no_game_dvr%==1 (
	reg.exe import Registry\Optional\no_game_dvr.reg
)

if %no_geolocation%==1 (
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableLocation" /t REG_DWORD /d 1 /f
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableLocationScripting" /t REG_DWORD /d 1 /f
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" /v "DisableWindowsLocationProvider" /t REG_DWORD /d 1 /f
	reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\lfsvc" /v "Start" /t REG_DWORD /d 4 /f
	sc.exe stop lfsvc
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Location\Notifications"
	schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Location\WindowsActionDialog"
)

if %no_mitigations%==1 (
	reg.exe import Registry\Optional\no_mitigations.reg

	powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""set_exploit_mitigations.ps1""' -Verb RunAs}"

	REM Ensure "Virtual Memory Pagefile Encryption" is disabled; by default it's not configured.
	fsutil.exe behavior set encryptpagingfile 0

	REM Ensure "Data Execution Prevention" (DEP) only applies to operating system components, along with Ring 0 (kernel-mode) drivers.
	REM Applying DEP to Ring 3 (user-mode) programs will slow down and break some, such as the original Deus Ex.
	bcdedit.exe /set nx Optin	
)

if %recommended_ethernet_tweaks%==1 (
	powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""Networking\Ethernet\recommended_tweaks.ps1""' -Verb RunAs}"
)

if %replace_windows_search%==1 (
	sc.exe stop WSearch
	reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v Start /t REG_DWORD /d 4 /f
	winget.exe install voidtools.Everything -eh
	winget.exe install stnkl.EverythingToolbar -eh
)

if %replace_windows11_interface%==1 (
	winget.exe install StartIsBack.StartAllBack -eh
)

if %reset_network_interface_settings%==1 (
	powershell.exe -Command "Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'"
)

if %thumbnail_shadows%==0 (
	reg.exe add "HKCR\SystemFileAssociations\image" /v "Treatment" /t REG_DWORD /d 0 /f
	reg.exe add "HKCR\SystemFileAssociations\image" /v "TypeOverlay" /t REG_SZ /d "" /f
) else (
	if %thumbnail_shadows%==1 (
		reg.exe add "HKCR\SystemFileAssociations\image" /v "Treatment" /t REG_DWORD /d 2 /f
		reg.exe add "HKCR\SystemFileAssociations\image" /v "TypeOverlay" /t REG_SZ /d "" /f
	)
)

reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoLowDiskSpaceChecks" /t REG_DWORD /d 1 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "LinkResolveIgnoreLinkInfo" /t REG_DWORD /d 1 /f

REM Don't search disks to attempt fixing a missing shortcut.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveSearch" /t REG_DWORD /d 1 /f

REM Don't search all paths related to the missing shortcut.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveTrack" /t REG_DWORD /d 1 /f

REM Don't waste time removing thumbnail caches; if they corrupt themselves, the user will do this themself.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache" /v "Autorun" /t REG_DWORD /d 0 /f

REM Don't check for an active connection through Microsoft's servers.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet" /v "EnableActiveProbing" /t REG_DWORD /d 0 /f

REM Ask OneDrive to only generate network traffic if signed in to OneDrive.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\OneDrive" /v "PreventNetworkTrafficPreUserSignIn" /t REG_DWORD /d 1 /f

REM Disallow automatic: software updates, security scanning, and system diagnostics.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance" /v "MaintenanceDisabled" /t REG_DWORD /d 1 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Diagnosis\Scheduled"


REM == GROUP 1: Ask to stop sending diagnostic data to Microsoft ==

reg.exe import Registry/prevent_microsoft_diagnostics.reg
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Feedback\Siuf\DmClient"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

REM Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
reg.exe import Registry\disable_CEIP.reg
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Autochk\Proxy"
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

REM == GROUP 1: END ==


REM Disable "Program Compatibility Assistant".
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisablePCA" /t REG_DWORD /d 1 /f

REM Disable "Application Compatibility Engine".
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisableEngine" /t REG_DWORD /d 1 /f
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Application Experience\PcaPatchDbTask"

REM Disable "SwitchBack Compatibility Engine".
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "SbEnable" /t REG_DWORD /d 0 /f
REM Disable user steps recorder.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat" /v "DisableUAR" /t REG_DWORD /d 1 /f

REM Disable Autoplay on all disk types.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoDriveTypeAutoRun" /t REG_DWORD /d 255 /f

REM Disable WER (Windows Error Reporting).
reg.exe import Registry\disable_WER.reg
schtasks.exe /Change /DISABLE /TN "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

REM Ask to not allow execution of experiments by Microsoft.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PolicyManager\current\device\System" /v "AllowExperimentation" /t REG_DWORD /d 0 /f

REM Disable tracking of application startups.
reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v "Start_TrackProgs" /t REG_DWORD /d 0 /f

REM Disable all Content Delivery Manager features, which stops automatic installation of advertised apps among others.
reg.exe import Registry\disable_CDM.reg

REM Disable SmartScreen, it delays the launch of software and is better done by other anti-malware software.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "EnableSmartScreen" /t REG_DWORD /d 0 /f
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer" /v "SmartScreenEnabled" /t REG_SZ /d "Off" /f
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost" /v "EnableWebContentEvaluation" /t REG_DWORD /d 0 /f

REM Don't increase overall system/DPC latency just for minimal power saving.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power\PowerThrottling" /v "PowerThrottlingOff" /t REG_DWORD /d 1 /f

REM Automated file cleanup without user interaction is a bad idea; Storage Sense only runs on low-disk space events.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\StorageSense" /v "AllowStorageSenseGlobal" /t REG_DWORD /d 0 /f
reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense" /f
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
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DoSvc" /v Start /t REG_DWORD /d 4 /f

REM Disables "Diagnostic Policy Service"; logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DPS" /v Start /t REG_DWORD /d 4 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\PcaSvc" /v Start /t REG_DWORD /d 4 /f

REM Don't analyze programs' execution time data.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib" /v "Disable Performance Counters" /t REG_DWORD /d 1 /f

REM Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3


REM == GROUP 2: Correct mistakes by others ==

reg.exe import Registry\mistake_corrections.reg

REM Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

REM Using the "classic" Hyper-V scheduler can break Hyper-V for QEMU with KVM.
bcdedit.exe /set hypervisorschedulertype core

REM Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
powershell.exe -Command "Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1"

REM MemoryCompression: Slightly increases CPU load, but reduces I/O load and makes Windows handle Out Of Memory situations smoothly; akin to Linux's zRAM.
powershell.exe -Command "Enable-MMAgent -ApplicationLaunchPrefetching -ApplicationPreLaunch -MemoryCompression"

REM Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2

REM == GROUP 2: END ==


REM Don't draw graphical elements for boot (spinner, Windows or BIOS logo, etc).
bcdedit.exe /set bootuxdisabled on

REM Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

REM Decrease shutdown time.
reg.exe import Registry\quicker_shutdown.reg

taskkill.exe /IM explorer.exe /F
start explorer.exe
echo.
echo Your PC will restart after a key is pressed; required to fully apply changes
echo.
Pause
shutdown.exe /r /t 00
