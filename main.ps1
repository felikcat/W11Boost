# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# File History:
# - Is unreliable with creating snapshots of files.
# - Use Restic or TrueNAS with Syncthing for backups instead.
$file_history = 0

# 0: Disables GPS services, which always run even if there's no GPS hardware installed.
$geolocation = 1

# 1: Ensures Windows' audio ducking/attenuation is disabled.
$audio_reduction = 0

# 0: Prevents random packet loss/drop-outs in exchange for a higher battery drain.
$ethernet_power_saving = 0

# Use NVIDIA ShadowPlay, AMD ReLive, or OBS Studio instead.
$game_dvr = 0

# Prevents time desync issues that were caused by using time.windows.com
# NOTICE: If you are connected to your own local NTP server, don't use this.
$optimal_online_ntp = 1

# NOTICE: Some settings set might intersect with WiFi adapters, as tweaks are applied to all network interfaces.
$recommended_ethernet_tweaks = 1

# "Everything" can circumvent the performance impact of Windows Search Indexing while providing faster and more accurate results.
$replace_windows_search = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# 0 is recommended.
# System Restore problems:
# - Cannot restore backups from previous versions of Windows; can't revert Windows updates with System Restore.
# - Will revert other personal files (program settings and installations).
$system_restore = 0

# 0 = disable Explorer's thumbnail (images/video previews) border shadows.
# -> 0 is recommended if dark mode is used.
$thumbnail_shadows = 0

# Harden Windows with visual changes and security tweaks.
$harden = 1

if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
	Write-Warning "ERROR: W11Boost -> Requires Administrator!"
	Break
}

$host.ui.rawui.windowtitle = "W11Boost by Felik @ https://github.com/nermur"

# If these are disabled, Windows Update will break and so will this script.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\AppXSvc" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ClipSVC" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\TokenBroker" /v "Start" /t REG_DWORD /d 3 /f
# StorSvc being disabled breaks the Windows Store.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\StorSvc" /v "Start" /t REG_DWORD /d 3 /f
sc.exe start AppXSvc
sc.exe start ClipSVC
sc.exe start StorSvc

Clear-Host
Write-Output "
==== Current settings ====

avoid_key_annoyances = $avoid_key_annoyances
file_history = $file_history
geolocation = $geolocation
audio_reduction = $audio_reduction
ethernet_power_saving = $ethernet_power_saving
game_dvr = $game_dvr
optimal_online_ntp = $optimal_online_ntp
recommended_ethernet_tweaks = $recommended_ethernet_tweaks
replace_windows_search = $replace_windows_search
reset_network_interface_settings = $reset_network_interface_settings
system_restore = $system_restore
thumbnail_shadows = $thumbnail_shadows
harden = $harden
"
Pause

# == Initialize ==
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\W11Boost_LastRun.log")
. ".\imports.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
if ($reset_network_interface_settings) {
	Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'
}
# ====

# "Fast startup" causes stability issues, and increases disk wear from excessive I/O usage.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Shutdown.reg"
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power" /V "HiberbootEnabled" /T REG_DWORD /D 0 /F
attrib +R %WinDir%\System32\SleepStudy\UserNotPresentSession.etl

if ($avoid_key_annoyances) {
	reg.exe import ".\Non-GPO Registry\avoid_key_annoyances.reg"
}

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\OS Policies.reg"

if ($file_history) {
	Set-ItemProperty -Path "HKCR:\SOFTWARE\Policies\Microsoft\Windows\FileHistory" -Name "Disabled" -Type DWord -Value 0 -Force
	Set-ItemProperty -Path "HKCR:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 3 -Force
	Enable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
}
elseif (!$file_history) { 
	Set-ItemProperty -Path "HKCR:\SOFTWARE\Policies\Microsoft\Windows\FileHistory" -Name "Disabled" -Type DWord -Value 1 -Force
	Set-ItemProperty -Path "HKCR:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 4 -Force
	Disable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
}

if ($geolocation) {
	reg.exe import ".\Non-GPO Registry\Geolocation\Enable.reg"
	Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
	Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}
elseif (!$geolocation) {
	reg.exe import ".\Non-GPO Registry\Geolocation\Disable.reg"
	sc.exe stop lfsvc
	Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
	Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($audio_reduction) {
	reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio\UserDuckingPreference" /f
}
if (!$audio_reduction) {
	reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
	reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
}

if (!$ethernet_power_saving) {
	Disable-Ethernet-Power-Saving
}

if (!$game_dvr) {
	reg.exe import ".\Non-GPO Registry\No Game DVR.reg"
}

if ($optimal_online_ntp) {
	reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Windows Time Service.reg"
	# Tier 1 ASNs like NTT are preferred since they are critical to routing functioning correctly for ISPs around the world.
	w32tm.exe /config /syncfromflags:manual /manualpeerlist:"x.ns.gin.ntt.net y.ns.gin.ntt.net ntp.ripe.net time.nist.gov time.cloudflare.com time.google.com"
}

if ($recommended_ethernet_tweaks) {
	Set-Recommended-Ethernet-Tweaks
}

if ($replace_windows_search) {
	sc.exe stop WSearch
	reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v "Start" /t REG_DWORD /d 4 /f
	reg.exe add "HKCU\Software\Microsoft\Windows\CurrentVersion\SearchSettings" /v "IsDeviceSearchHistoryEnabled" /t REG_DWORD /d 0 /f 
	.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install voidtools.Everything -eh --accept-package-agreements --accept-source-agreements"
	.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install stnkl.EverythingToolbar -eh --accept-package-agreements --accept-source-agreements"
}

if (!$system_restore) {
	reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\System Restore.reg"
	# Delete all restore points.
	vssadmin.exe delete shadows /all /quiet
}

if ($thumbnail_shadows) {
	Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 0 -Force
}
elseif (!$thumbnail_shadows) {
	Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 2 -Force
}

if ($harden) {
	reg.exe import ".\Non-GPO Regsitry\Harden.reg"
}

# Disable the acrylic blur at sign-in screen to improve performance at that screen.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "DisableAcrylicBackgroundOnLogon" /t REG_DWORD /d 1 /f

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
[Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", "~0x200000200000000", "Machine")

# Exclude the optional driver updates by default.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate" /v "ExcludeWUDriversInQualityUpdate" /t REG_DWORD /d 1 /f

# Disable feedback reminders
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Siuf\Rules" /v "NumberOfSIUFInPeriod" /t REG_DWORD /d 0 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Siuf\Rules" /v "PeriodInNanoSeconds" /t REG_DWORD /d 0 /f

# Don't automatically update speech recognition and speech synthesis modules.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Speech" /v "AllowSpeechModelUpdate" /t REG_DWORD /d 0 /f

# Don't block downloaded files in Explorer, also fixes File History not working for downloaded files.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Attachments" /v "SaveZoneInformation" /t REG_DWORD /d 1 /f

# If logged into a Microsoft account: Don't sync anything.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SettingSync" /v "SyncPolicy" /t REG_DWORD /d 5 /f

# Unsorted privacy stuff.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Privacy" /v "TailoredExperiencesWithDiagnosticDataEnabled" /t REG_DWORD /d 0 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo" /v "Enabled" /t REG_DWORD /d 0 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v "ShowSyncProviderNotifications" /t REG_DWORD /d 0 /f
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection" /v "LimitDiagnosticLogCollection" /t REG_DWORD /d 1 /f

# Don't automatically search the web; annoying when trying to search to access a program quickly from the keyboard.
reg.exe add "HKEY_CURRENT_USER\Software\Policies\Microsoft\Windows\Explorer" /v "DisableSearchBoxSuggestions" /t REG_DWORD /d 1 /f

reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoLowDiskSpaceChecks" /t REG_DWORD /d 1 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "LinkResolveIgnoreLinkInfo" /t REG_DWORD /d 1 /f

# Don't search disks to attempt fixing a missing shortcut.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveSearch" /t REG_DWORD /d 1 /f

# Don't search all paths related to the missing shortcut.
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoResolveTrack" /t REG_DWORD /d 1 /f

# Depend on the user clearing out thumbnail caches manually if they get corrupted.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache" /v "Autorun" /t REG_DWORD /d 0 /f

# Don't check for an active connection through Microsoft's servers.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet" /v "EnableActiveProbing" /t REG_DWORD /d 0 /f

# Disallow automatic: program updates, security scanning, and system diagnostics.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance" /v "MaintenanceDisabled" /t REG_DWORD /d 1 /f
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
reg.exe import ".\Registry\Computer Configuration\Windows Components\OneDrive.reg"

# Ask to stop sending diagnostic data to Microsoft.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Data Collection and Preview Builds.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"

# Disables various compatibility assistants and engines; it's assumed a W11Boost user is going to manually set compatibility when needed.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Application Compatibility.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"

# Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\App-V.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Internet Communication Management.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

# Disable Windows Error Reporting (WER).
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Error Reporting.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

# Ask to not allow execution of experiments by Microsoft.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PolicyManager\current\device\System" /v "AllowExperimentation" /t REG_DWORD /d 0 /f

# Disable tracking of application startups.
reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced" /v "Start_TrackProgs" /t REG_DWORD /d 0 /f

# Disables Cloud Content features; stops automatic installation of advertised ("suggested") apps among others.
# Apparently is called "Content Delivery Manager" in Windows 10.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Cloud Content.reg"
reg.exe import ".\LTSC 2022 Registry\disable_CDM.reg"

# == Disable SmartScreen; delays program launches and is better done by other anti-malware programs. ==
reg.exe import ".\Registry\Computer Configuration\Windows Components\Windows Defender SmartScreen.reg"

reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer" /v "SmartScreenEnabled" /t REG_SZ /d "Off" /f

reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost" /v "EnableWebContentEvaluation" /t REG_DWORD /d 0 /f
# ====


# Automated file cleanup without user interaction is a bad idea, even if Storage Sense only runs on low-disk space events.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Storage Sense.reg"
reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense" /f
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\Diagnostics"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\StorageSense"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskCleanup\SilentCleanup"

# == Disable these scheduler tasks to keep performance and bandwidth usage more consistent. ==
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentFallBack"
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentLogOn"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\StartupAppTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\ApplicationData\DsSvcCleanup"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppxDeploymentClient\Pre-staged app cleanup"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\CertificateServicesClient\UserTask-Roam"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Clip\License Validation"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\CloudExperienceHost\CreateObjectTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\File Classification Infrastructure\Property Definition Sync"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\HelloFace\FODCleanupTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\SmartRetry"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\International\Synchronize Language Settings"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Maintenance\WinSAT"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Maps\MapsToastTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Maps\MapsUpdateTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Mobile Broadband Accounts\MNO Metadata Parser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\MUI\LPRemove"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Multimedia\SystemSoundsService"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\NetTrace\GatherNetworkInfo"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\PI\Sqm-Tasks"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Printing\EduPrintProv"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Ras\MobilityManager"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RecoveryEnvironment\VerifyWinRE"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyMonitor"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyRefreshTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTaskLogon"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SoftwareProtectionPlatform\SvcRestartTaskNetwork"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SoftwareProtectionPlatform\SvcTrigger"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Speech\SpeechModelDownloadTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Sysmain\ResPriStaticDbSync"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Sysmain\WsSwapAssessmentTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\USB\Usb-Notifications"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WDI\ResolutionHost"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Filtering Platform\BfeOnServiceStartTypeChange"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WlanSvc\CDSSync"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WOF\WIM-Hash-Management"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WOF\WIM-Hash-Validation"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Work Folders\Work Folders Logon Synchronization"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Work Folders\Work Folders Maintenance Work"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WS\WSTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WwanSvc\OobeDiscovery"
# ====


# == Prevent Windows Update obstructions and other annoyances. ==
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Update.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdates"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdatesAsUser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan Static Task"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\UpdateModelTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\USO_UxBroker"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\sih"
# ====


# Disable "Delivery Optimization".
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DoSvc" /v "Start" /t REG_DWORD /d 4 /f

# Don't analyze programs' execution time data.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib" /v "Disable Performance Counters" /t REG_DWORD /d 1 /f

# == NTFS tweaks ==
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Filesystem.reg"
# Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3
# ====


# Can severely degrade a program's performance if it got marked for "crashing" too often, such is the case for Assetto Corsa.
# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
reg.exe add "HKEY_LOCAL_MACHINE\Software\Microsoft\FTH" /v "Enabled" /t REG_DWORD /d 0 /f

# == Correct mistakes by others ==
reg.exe import ".\Non-GPO Registry\mistake_corrections.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Power Management.reg"

# Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

# Draw graphical elements for boot (progress spinner, Windows or BIOS logo, etc).
# This is useful to tell if something went wrong if a BSOD can't show up.
bcdedit.exe /deletevalue bootuxdisabled

# Using the "classic" Hyper-V scheduler can break Hyper-V for QEMU with KVM.
bcdedit.exe /set hypervisorschedulertype core

# Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# MemoryCompression: Slightly increase CPU load to reduce I/O load, and handle Out Of Memory situations smoothly; akin to Linux's zRAM.
$options = @("-ApplicationLaunchPrefetching", "-ApplicationPreLaunch", "-MemoryCompression")
for ($i = 0; $i -lt $options.length; $i++) {
    Enable-MMAgent $options[$i]
}

# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2
# ====


# Trade higher memory usage for less CPU load.
Disable-MMAgent -PageCombining

# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no

# Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

# Decrease shutdown time.
reg.exe import ".\Non-GPO Registry\quicker_shutdown.reg"

# == Other registry tweaks ==
reg.exe import ".\Non-GPO Registry\disable_services.reg"
reg.exe import ".\Non-GPO Registry\disable_typing_insights.reg"
reg.exe import ".\Non-GPO Registry\performance_options.reg"
reg.exe import ".\Non-GPO Registry\UAC.reg"
reg.exe import ".\Non-GPO Registry\Deny Screenshots By Apps.reg"
reg.exe import ".\Non-GPO Registry\Unsorted.reg"
reg.exe import ".\Non-GPO Registry\No Edge Autorun.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Device Installation.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Group Policy.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\App Package Deployment.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Microsoft Edge.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Security.reg"

reg.exe import ".\Registry\User Configuration\Administrative Templates\Desktop.reg"
# ====

Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
reg.exe import ".\Non-GPO Registry\Visual Studio 2022.reg"

Clear-Host
Write-Warning "
Your PC will restart after a key is pressed!
It's required to fully apply changes.
"
Pause
shutdown.exe /r /t 00
