# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# Undermines software that clear the clipboard automatically.
$clipboard_history = 0

# File History:
# - Is unreliable with creating snapshots of files.
# - Use https://restic.net/ for automated backups instead, and Git for your own projects.
$file_history = 0

# Disables GPS services, which always run even if there's no GPS hardware installed.
$geolocation = 0

# Ensures Windows' audio ducking/attenuation is disabled.
$no_audio_reduction = 0

# Prevents random packet loss/drop-outs in exchange for a higher battery drain.
$no_ethernet_power_saving = 1

# Use NVIDIA ShadowPlay, AMD ReLive, or OBS Studio instead.
$no_game_dvr = 1

# Disables all non-essential security mitigations; drastically improves performance for older CPUs (such as an Intel i7-4790K).
$no_mitigations = 1

# Prevents time desync issues that were caused by using time.windows.com
# If you run and are connected to your own local NTP server, don't use this.
$optimal_online_ntp = 1

# NOTICE: Some settings set might intersect with WiFi adapters, as tweaks are applied to all network interfaces.
$recommended_ethernet_tweaks = 1

# "Everything" can circumvent the performance impact of Windows Search Indexing while providing faster and more accurate results.
$replace_windows_search = 1

# See https://www.startallback.com/ for more information.
$replace_windows11_interface = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# 0 is recommended.
# System Restore:
# - Cannot restore backups from previous versions of Windows; can't revert Windows updates with System Restore.
# - Will revert other personal files (program settings and installations).
$system_restore = 0

# 0 = disable Explorer's thumbnail (images/video previews) border shadows.
# 0 is recommended if dark mode is used.
$thumbnail_shadows = 1

if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
	Write-Warning "ERROR: W11Boost -> Right click on this file and select 'Run as administrator'"
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
clipboard_history = $clipboard_history
file_history = $file_history
geolocation = $geolocation
no_audio_reduction = $no_audio_reduction
no_ethernet_power_saving = $no_ethernet_power_saving
no_game_dvr = $no_game_dvr
no_mitigations = $no_mitigations
optimal_online_ntp = $optimal_online_ntp
recommended_ethernet_tweaks = $recommended_ethernet_tweaks
replace_windows_search = $replace_windows_search
replace_windows11_interface = $replace_windows11_interface
reset_network_interface_settings = $reset_network_interface_settings
system_restore = $system_restore
thumbnail_shadows = $thumbnail_shadows
"
Pause

# - Initialize -
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments')+"\W11Boost_LastRun.log")
. ".\imports.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR

# "Fast startup" causes stability issues, and increases disk wear from excessive I/O usage.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Shutdown.reg"
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power" /V "HiberbootEnabled" /T REG_DWORD /D 0 /F
attrib +R %WinDir%\System32\SleepStudy\UserNotPresentSession.etl

if ($avoid_key_annoyances) {
	reg.exe import ".\Non-GPO Registry\avoid_key_annoyances.reg"
}

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\OS Policies.reg"
if ($clipboard_history) {
	reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "AllowClipboardHistory" /t REG_DWORD /d 1 /f
}

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
elseif(!$geolocation) {
	reg.exe import ".\Non-GPO Registry\Geolocation\Disable.reg"
	sc.exe stop lfsvc
	Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
	Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($no_audio_reduction) {
	reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
	reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
}

if ($no_ethernet_power_saving) {
	Disable-Ethernet-Power-Saving
}

if ($no_game_dvr) {
	reg.exe import ".\Non-GPO Registry\no_game_dvr.reg"
}

if ($no_mitigations) {
	reg.exe import ".\Non-GPO Registry\no_mitigations.reg"
	Remove-All-ProcessMitigations
	Set-ProcessMitigation -PolicyFilePath mitigations.xml

	# Ensure "Data Execution Prevention" (DEP) only applies to operating system components, along with kernel-mode drivers.
	# Applying DEP to user-mode programs will slow down and break some, such as the original Deus Ex.
	bcdedit.exe /set nx Optin
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
	.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install voidtools.Everything -eh"
	.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install stnkl.EverythingToolbar -eh"
}

if ($replace_windows11_interface) {
	.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install StartIsBack.StartAllBack -eh"
}

if ($reset_network_interface_settings) {
	Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'
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

# Disallow automatic: software updates, security scanning, and system diagnostics.
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

# Disable Autoplay on all disk types.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer" /v "NoDriveTypeAutoRun" /t REG_DWORD /d 255 /f

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

# == Disable SmartScreen; delays program launches and is better done by other anti-malware software. ==
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
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

# Using the "classic" Hyper-V scheduler can break Hyper-V for QEMU with KVM.
bcdedit.exe /set hypervisorschedulertype core

# Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# MemoryCompression: Slightly increases CPU load, but reduces I/O load and makes Windows handle Out Of Memory situations smoothly; akin to Linux's zRAM.
Enable-MMAgent -ApplicationLaunchPrefetching -ApplicationPreLaunch -MemoryCompression

# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2
# ====

# Don't draw graphical elements for boot (spinner, Windows or BIOS logo, etc).
bcdedit.exe /set bootuxdisabled on

# Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

# Decrease shutdown time.
reg.exe import ".\Non-GPO Registry\quicker_shutdown.reg"

# == Other registry tweaks ==
reg.exe import ".\Non-GPO Registry\disable_services.reg"
reg.exe import ".\Non-GPO Registry\disable_typing_insights.reg"
reg.exe import ".\Non-GPO Registry\performance_options.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Device Installation.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\Group Policy.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\App Package Deployment.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Microsoft Edge.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Security.reg"

reg.exe import ".\Registry\User Configuration\Administrative Templates\Desktop.reg"
# ====

Clear-Host
Write-Warning "
Your PC will restart after a key is pressed!
It's required to fully apply changes.
"
Pause
shutdown.exe /r /t 00
