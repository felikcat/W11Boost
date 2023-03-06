if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: TuneUp11 -> requires being run as Administrator!"
    Break
}

$host.ui.rawui.windowtitle = "TuneUp11 by github.com/felikcat"

# If these are disabled, Windows Update will break and so will this script.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\AppXSvc" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\ClipSVC" /v "Start" /t REG_DWORD /d 3 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\TokenBroker" /v "Start" /t REG_DWORD /d 3 /f
# Disabled StorSvc breaks the Windows Store.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\StorSvc" /v "Start" /t REG_DWORD /d 3 /f
# Disabled DoSvc (delivery optimization) breaks winget.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\DoSvc" /v "Start" /t REG_DWORD /d 3 /f
sc.exe start AppXSvc
sc.exe start ClipSVC
sc.exe start StorSvc
sc.exe start DoSvc

Clear-Host
Write-Output "Disable your anti-virus before continuing.
If this is uncomfortable for you, close this program."
Pause

# == Initialize ==
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_LastRun.log")
. ".\imports.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
# ====

# "Fast startup" causes stability issues, and increases disk wear from excessive I/O usage.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power" /V "HiberbootEnabled" /T REG_DWORD /D 0 /F
attrib +R %WinDir%\System32\SleepStudy\UserNotPresentSession.etl

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\OS Policies.reg"


# == Use optimal online NTP servers for more accurate system time. ==
net.exe stop w32time
# Make a clean slate for the time sync settings.
w32tm.exe /unregister
w32tm.exe /register

w32tm.exe /config /syncfromflags:manual /manualpeerlist:"time.cloudflare.com ntppool1.time.nl ntppool2.time.nl"
net.exe start w32time
w32tm.exe /resync
# ====

Set-Recommended-Ethernet-Tweaks

# == Replacing the Windows Search Index. Indexing file contents is pointless if you're organized.  ==
sc.exe stop WSearch
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v "Start" /t REG_DWORD /d 4 /f
reg.exe add "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings" /v "IsDeviceSearchHistoryEnabled" /t REG_DWORD /d 0 /f
# --source winget prevents error 0x8a150044 if the Windows Store isn't reachable.
.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install voidtools.Everything -eh --accept-package-agreements --accept-source-agreements --source winget"
# ====

# Replaces Windows built-in thumbnailing for many file formats.
.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install Xanashi.Icaros -eh --accept-package-agreements --accept-source-agreements --source winget"

# Skip to the sign-on screen.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Personalization" /v "NoLockScreen" /t REG_DWORD /d 1 /f
# Disable the acrylic blur at sign-in screen to improve performance at that screen.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System" /v "DisableAcrylicBackgroundOnLogon" /t REG_DWORD /d 1 /f

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
[Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", "~0x200000200000000", "Machine")

# Show what's slowing down bootups and shutdowns.
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System" /v "verbosestatus" /t REG_DWORD /d 1 /f

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
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"

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

# Disables various compatibility assistants and engines; it's assumed a TuneUp11 user is going to manually set compatibility when needed.
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

# Restore the classic Windows 10 context menu; more performant, and easier to use.
reg.exe add "HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32" /ve /d "" /f

# Disables Cloud Content features; stops automatic installation of advertised ("suggested") apps among others.
# Apparently is called "Content Delivery Manager" in Windows 10.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Cloud Content.reg"

# == Disable SmartScreen; delays program launches and is better done by other anti-malware programs. ==
reg.exe import ".\Registry\Computer Configuration\Windows Components\Windows Defender SmartScreen.reg"
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer" /v "SmartScreenEnabled" /t REG_SZ /d "Off" /f
reg.exe add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost" /v "EnableWebContentEvaluation" /t REG_DWORD /d 0 /f


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

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

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

reg.exe import ".\Non-GPO Registry\No Blocked Files.reg"

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
reg.exe import ".\Non-GPO Registry\Mistake Corrections.reg"
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

# Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# MemoryCompression: While enabled; increases CPU load to reduce I/O load and handle Out Of Memory situations more smoothly; akin to Linux's zRAM.
# -> Its downside is worsened stuttering in video games.
# PageCombining: While enabled; reduces memory usage but increases CPU load.
Enable-MMAgent -ApplicationLaunchPrefetching
Enable-MMAgent -ApplicationPreLaunch


# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2
# ====


# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no

# Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

# == Other registry tweaks ==
reg.exe import ".\Non-GPO Registry\Shutdown.reg"
reg.exe import ".\Non-GPO Registry\disable_services.reg"
reg.exe import ".\Non-GPO Registry\disable_typing_insights.reg"
reg.exe import ".\Non-GPO Registry\performance_options.reg"
reg.exe import ".\Non-GPO Registry\UAC.reg"
reg.exe import ".\Non-GPO Registry\Unsorted.reg"
reg.exe import ".\Non-GPO Registry\No Edge Autorun.reg"
reg.exe import  ".\Non-GPO Registry\Disable Delivery Optimization.reg"

reg.exe import ".\Non-GPO Registry\HiDPI Blurry Font Fix.reg"

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
A reboot is required to finish applying changes, press any key to continue.
"
Pause
shutdown.exe /r /t 00
