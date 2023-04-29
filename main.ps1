if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Run TuneUp11 as Administrator!"
    Break
}

$host.ui.rawui.windowtitle = "TuneUp11 by github.com/felikcat"

# If these are disabled, Windows Update will break and so will this script.
# Disabled StorSvc breaks the Windows Store.
# Disabled DoSvc (delivery optimization) breaks winget.
$services = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")

for ($i = 0; $i -lt $services.length; $i++) {
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$services[$i]" /v "Start" /t REG_DWORD /d 3 /f
    sc.exe start $services[$i]
}


##+=+= Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_LastRun.log")
. ".\imports.ps1"
Import-Module .\PolicyFileEditor\PolicyFileEditor.psm1
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
##+=+=


##+=+= "Fast startup" causes stability issues, and increases disk wear from excessive I/O usage.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -ValueName 'HiberbootEnabled' -Data '0' -Type 'Dword'

attrib +R "$env:windir\System32\SleepStudy\UserNotPresentSession.etl"
##+=+=


##+=+=
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableActivityFeed' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'PublishUserActivities' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'UploadUserActivities' -Data '0' -Type 'Dword'
##+=+=


##+=+= Use optimal online NTP servers for more accurate system time.
net.exe stop w32time
# Make a clean slate for the time sync settings.
w32tm.exe /unregister
w32tm.exe /register

w32tm.exe /config /syncfromflags:manual /manualpeerlist:"time.cloudflare.com ntppool1.time.nl ntppool2.time.nl"
net.exe start w32time
w32tm.exe /resync
##+=+=


Set-Recommended-Ethernet-Tweaks


##+=+= Replacing the Windows Search Index. Indexing file contents is pointless if you're organized.
sc.exe stop WSearch
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v "Start" /t REG_DWORD /d 4 /f

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

# --source winget prevents error 0x8a150044 if the Windows Store isn't reachable.
.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install voidtools.Everything -eh --accept-package-agreements --accept-source-agreements --source winget"
##+=+=


# Replaces Windows built-in thumbnailing for many file formats.
.\NSudoLC.exe -U:E -P:E -M:S powershell.exe -Command "winget.exe install Xanashi.Icaros -eh --accept-package-agreements --accept-source-agreements --source winget"

# Skip to the sign-on screen.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Personalization' -ValueName 'NoLockScreen' -Data '1' -Type 'Dword'

# Disable the acrylic blur at sign-in screen to improve performance at that screen.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'DisableAcrylicBackgroundOnLogon' -Data '1' -Type 'Dword'

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
[Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", "~0x200000200000000", "Machine")

# Show what's slowing down bootups and shutdowns.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'verbosestatus' -Data '1' -Type 'Dword'

# Disable optional driver updates; tends to be very outdated.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'ExcludeWUDriversInQualityUpdate' -Data '1' -Type 'Dword'


##+=+= Disable feedback reminders
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Siuf\Rules' -ValueName 'NumberOfSIUFInPeriod' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Siuf\Rules' -ValueName 'PeriodInNanoSeconds' -Data '0' -Type 'Dword'
##+=+=

# Disable advertising ID for apps.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo' -ValueName 'DisabledByGroupPolicy' -Data '1' -Type 'Dword'

# Don't automatically update speech recognition and speech synthesis modules.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Speech' -ValueName 'AllowSpeechModelUpdate' -Data '0' -Type 'Dword'

# Don't block downloaded files in Explorer, also fixes File History not working for downloaded files.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'

# If logged into a Microsoft account: Don't sync anything.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SettingSync' -ValueName 'SyncPolicy' -Data '5' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Privacy' -ValueName 'TailoredExperiencesWithDiagnosticDataEnabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo' -ValueName 'Enabled' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSyncProviderNotifications' -Data '0' -Type 'Dword'

# Disable tracking of application startups.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Start_TrackProgs' -Data '0' -Type 'Dword'

# Don't automatically search the web; annoying when trying to search to access a program quickly from the keyboard.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Policies\Microsoft\Windows\Explorer' -ValueName 'DisableSearchBoxSuggestions' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoLowDiskSpaceChecks' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'LinkResolveIgnoreLinkInfo' -Data '1' -Type 'Dword'

# Don't search disks to attempt fixing a missing shortcut.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveSearch' -Data '1' -Type 'Dword'

# Don't search all paths related to the missing shortcut.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveTrack' -Data '1' -Type 'Dword'

# Depend on the user clearing out thumbnail caches manually if they get corrupted.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache' -ValueName 'Autorun' -Data '0' -Type 'Dword'

# Don't check for an active connection through Microsoft's servers.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache' -ValueName 'Autorun' -Data '0' -Type 'Dword'

# Old versions of the Intel PROSet/Wireless driver set this to 0, breaking Windows' internet connectivity check.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet' -ValueName 'EnableActiveProbing' -Data '1' -Type 'Dword'


##+=+= Disallow automatic: program updates, security scanning, and system diagnostics.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance' -ValueName 'MaintenanceDisabled' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"
##+=+=


# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Explorer' -ValueName 'NoUseStoreOpenWith' -Data '1' -Type 'Dword'

# Ask OneDrive to only generate network traffic if signed in to OneDrive.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\OneDrive' -ValueName 'PreventNetworkTrafficPreUserSignIn' -Data '1' -Type 'Dword'


##+=+= Ask to stop sending diagnostic data to Microsoft.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'AllowDeviceNameInTelemetry' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'AllowTelemetry' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DisableTelemetryOptInChangeNotification' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DoNotShowFeedbackNotifications' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DataCollection' -ValueName 'DisableOneSettingsDownloads' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClient"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Feedback\Siuf\DmClientOnScenarioDownload"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\ReconcileFeatures"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataFlushing"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\FeatureConfig\UsageDataReporting"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Flighting\OneSettings\RefreshCache"
##+=+=


##+=+= Disables various compatibility assistants and engines.
# Disable "Program Compatibility Assistant".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePCA' -Data '1' -Type 'Dword'

# Disable "Application Compatibility Engine".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableEngine' -Data '1' -Type 'Dword'

# Disable "SwitchBack Compatibility Engine".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'SbEnable' -Data '0' -Type 'Dword'

# Disable user Steps Recorder.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableUAR' -Data '1' -Type 'Dword'

# Disable "Remove Program Compatibility Property Page".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePropPage' -Data '0' -Type 'Dword'

# Disable "Inventory Collector".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableInventory' -Data '1' -Type 'Dword'

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Application Compatibility.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
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


# Disable telemetry for Tablet PC's handwriting recognition.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\TabletPC' -ValueName 'PreventHandwritingDataSharing' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

# Disable Windows Error Reporting (WER).
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Error Reporting.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Error Reporting\QueueReporting"

# Ask to not allow execution of experiments by Microsoft.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\PolicyManager\current\device\System' -ValueName 'AllowExperimentation' -Data '0' -Type 'Dword'

# Restore the classic Windows 10 context menu; more performant and easier to use.
reg.exe import ".\Non-GPO Registry\Old Context Menus.reg"


##+=+= Disables Cloud Content features; stops automatic installation of advertised ("suggested") apps among others; called "Content Delivery Manager" in Windows 10.

# Disable Consumer Experience:
# - In the start menu, programs that aren't installed ("suggestions") are gone.
# - Microsoft account notifications are gone.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableConsumerAccountStateContent' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableCloudOptimizedContent' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableWindowsConsumerFeatures' -Data '1' -Type 'Dword'
##+=+=


# Do not show Windows tips.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableSoftLanding' -Data '1' -Type 'Dword'


##+=+= Automated file cleanup without user interaction is a bad idea, even if its ran only on low-disk space events.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowStorageSenseGlobal' -Data '0' -Type 'Dword'
reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense" /f

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\Diagnostics"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\StorageSense"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskCleanup\SilentCleanup"
##+=+=


# Disable "Title bar window shake", previously called "Aero shake".
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Policies\Microsoft\Windows\Explorer' -ValueName 'NoWindowMinimizingShortcuts' -Data '1' -Type 'Dword'

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowAutomaticAppArchiving' -Data '0' -Type 'Dword'


##+=+= Disable these scheduler tasks to keep performance and bandwidth usage more consistent.
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentFallBack"
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentLogOn"

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
Disable-ScheduledTask -TaskName "\Microsoft\Windows\NetTrace\GatherNetworkInfo"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\PI\Sqm-Tasks"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Printing\EduPrintProv"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RecoveryEnvironment\VerifyWinRE"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"

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

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WS\WSTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WwanSvc\OobeDiscovery"

# Microsoft's Malicious Removal Tool task can pop up out of nowhere if Windows Update is still allowed to connect.
# MRT will remove "malicious" (false positives) files that other anti-virus software like Kaspersky purposefully exclude.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\MRT' -ValueName 'DontOfferThroughWUAU' -Data '1' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_HB"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_ERROR_HB"
##+=+=


Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"


##+=+= Prevent Windows Update obstructions and other annoyances.
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Update.reg"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdates"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdatesAsUser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan Static Task"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\UpdateModelTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\USO_UxBroker"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\sih"
##+=+=


# SaveZoneInformation 1 = disables blocking downloaded files.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'

# Don't analyze programs' execution time data.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib' -ValueName 'Disable Performance Counters' -Data '1' -Type 'Dword'


##+=+= NTFS tweaks
# Enabling long paths (260 character limit) prevents issues in Scoop and other programs.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\FileSystem' -ValueName 'LongPathsEnabled' -Data '1' -Type 'Dword'

# Ensure "Virtual Memory Pagefile Encryption" is disabled; by default it's not configured (off).
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsEncryptPagingFile' -Data '0' -Type 'Dword'

# Reducing page-faults and stack usage is beneficial to lowering DPC latency.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsForceNonPagedPoolAllocation' -Data '1' -Type 'Dword'

# Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3
##+=+=


# Can severely degrade a program's performance if it got marked for "crashing" too often, such is the case for Assetto Corsa.
# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\FTH' -ValueName 'Enabled' -Data '0' -Type 'Dword'


##+=+= START: Correct mistakes by others
reg.exe import ".\Non-GPO Registry\Mistake Corrections.reg"

# Process Lasso or manually setting a non-battery saving power profile is preferred instead.
# Don't make the power saving profiles less helpful.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Power\PowerThrottling' -ValueName 'PowerThrottlingOff' -Data '0' -Type 'Dword'

# Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

Enable-MMAgent -ApplicationLaunchPrefetching
Enable-MMAgent -ApplicationPreLaunch

# Draw graphical elements for boot (progress spinner, Windows or BIOS logo, etc).
# This is useful to tell if something went wrong if a BSOD can't show up.
bcdedit.exe /deletevalue bootuxdisabled

# Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# Use the BBRv2 TCP congestion control algorithm; the differences:
# https://web.archive.org/web/20220313173158/http://web.archive.org/screenshot/https://docs.google.com/spreadsheets/d/1I1NcVVbuC7aq4nGalYxMNz9pgS9OLKcFHssIBlj9xXI
netsh.exe int tcp set supplemental Template=Internet CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=Datacenter CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=Compat CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=DatacenterCustom CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=InternetCustom CongestionProvider=bbr2

# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2
##+=+=


# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no

# Don't log events without warnings or errors.
auditpol.exe /set /category:* /Success:disable

reg.exe import ".\Non-GPO Registry\No Edge Autorun.reg"
reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Microsoft Edge.reg"
Disable-ScheduledTask -TaskName "\MicrosoftEdgeUpdateTaskMachineCore"
Disable-ScheduledTask -TaskName "\MicrosoftEdgeUpdateTaskMachineUA"

# Stops Windows Widgets from running, unless you use a Widget you added.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Dsh' -ValueName 'AllowNewsAndInterests' -Data '0' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
reg.exe import ".\Non-GPO Registry\Visual Studio 2022.reg"


##+=+= Other registry tweaks
reg.exe import ".\Non-GPO Registry\Shutdown.reg"
reg.exe import ".\Non-GPO Registry\Disable Services.reg"
reg.exe import ".\Non-GPO Registry\disable_typing_insights.reg"
reg.exe import ".\Non-GPO Registry\performance_options.reg"
reg.exe import ".\Non-GPO Registry\Enable UAC.reg"
reg.exe import ".\Non-GPO Registry\Unsorted.reg"
reg.exe import ".\Non-GPO Registry\Disable Delivery Optimization.reg"
reg.exe import ".\Non-GPO Registry\Disable Cloud Search.reg"

# Allow Phone -> PC linking on this device.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableMmx' -Data '1' -Type 'Dword'

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\App Package Deployment.reg"

reg.exe import ".\Registry\Computer Configuration\Administrative Templates\Windows Components\Windows Security.reg"
##+=+=


gpupdate.exe /force
#shutdown.exe /r /t 00
