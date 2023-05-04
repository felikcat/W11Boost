# Don't automatically update speech recognition and speech synthesis modules.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Speech' -ValueName 'AllowSpeechModelUpdate' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'LinkResolveIgnoreLinkInfo' -Data '1' -Type 'Dword'

# Don't search disks to attempt fixing a missing shortcut.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveSearch' -Data '1' -Type 'Dword'

# Don't search all paths related to the missing shortcut.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveTrack' -Data '1' -Type 'Dword'

# Can severely degrade a program's performance if it got marked for "crashing" too often, such is the case for Assetto Corsa.
# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\FTH' -ValueName 'Enabled' -Data '0' -Type 'Dword'

# Video games that fail to request a higher timer frequency will benefit from Windows' old timer resolution behavior.
# Requires 0.5ms timer resolution to be fully effective.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\kernel' -ValueName 'GlobalTimerResolutionRequests' -Data '1' -Type 'Dword'

# Enforce 0.5ms (the minimum) timer resolution.
Disable-ScheduledTask "\Intelligent StandbyList Cleaner"
Unblock-File -Path "..\Third-party\STR\SetTimerResolution.exe"
Copy-Item "..\Third-party\STR" -Destination "$env:LOCALAPPDATA\Programs" -Recurse
Start-Process -WorkingDirectory "$env:LOCALAPPDATA\Programs\STR" -FilePath "SetTimerResolution.exe" -Args "-install"

# MemoryCompression: While enabled; increases CPU load to reduce I/O load and handle Out Of Memory situations more smoothly; akin to Linux's zRAM.
# -> Its downside is worsened stuttering in video games.
# PageCombining: While enabled; reduces memory usage but increases CPU load.
Disable-MMAgent -MemoryCompression
Disable-MMAgent -PageCombining


##+=+= Disallow automatic: program updates, security scanning, and system diagnostics.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance' -ValueName 'MaintenanceDisabled' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"
##+=+=


##+=+= Automated file cleanup without user interaction is a bad idea, even if its ran only on low-disk space events.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowStorageSenseGlobal' -Data '0' -Type 'Dword'
reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense" /f

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\Diagnostics"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\StorageSense"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskCleanup\SilentCleanup"
##+=+=


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

