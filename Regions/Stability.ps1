#Requires -Version 5 -RunAsAdministrator

New-PSDrive -Name "HKCR" -PSProvider "Registry" -Root "HKEY_CLASSES_ROOT" 

# Don't automatically update speech recognition and speech synthesis modules.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Speech' -ValueName 'AllowSpeechModelUpdate' -Data '0' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'LinkResolveIgnoreLinkInfo' -Data '1' -Type 'Dword'

# Don't search disks to attempt fixing a missing shortcut.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveSearch' -Data '1' -Type 'Dword'
# Don't search all paths related to the missing shortcut.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoResolveTrack' -Data '1' -Type 'Dword'

# Disable Explorer's thumbnail shadows.
Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 2 -Force

# Enable multiple processes for explorer.exe
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'SeparateProcess' -Data '1' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

# By default Windows doesn't automatically back-up the registry, but just in case they change this..
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager' -ValueName 'EnablePeriodicBackup' -Data '0' -Type 'Dword'

# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
PolEdit_HKLM 'SOFTWARE\Microsoft\FTH' -ValueName 'Enabled' -Data '0' -Type 'Dword'

# Sets Windows' default process priority; this is not the default for Windows Server.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\PriorityControl' -ValueName 'Win32PrioritySeparation' -Data '2' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\GameBar' -ValueName 'AutoGameModeEnabled' -Data '1' -Type 'Dword'

# https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/for-best-performance--use-dxgi-flip-model#directflip
PolEdit_HKCU 'Software\Microsoft\DirectX\UserGpuPreferences' -ValueName 'DirectXUserGlobalSettings' -Data 'SwapEffectUpgradeEnable=1;' -Type 'String'

# Enables hardware-accelerated GPU scheduling except for blocked GPU VIDs listed in:
# HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\BlockList\Kernel
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\GraphicsDrivers' -ValueName 'HwSchMode' -Data '2' -Type 'Dword'

Disable-MMAgent -MemoryCompression
Disable-MMAgent -PageCombining


##+=+= Disallow automatic: program updates, security scanning, and system diagnostics.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance' -ValueName 'MaintenanceDisabled' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"
##+=+=


# Resets adapter settings to driver defaults; it's assumed if there were prior tweaks done, they're incorrect.
Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'

Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

netsh.exe int tcp set global timestamps=enabled


##+=+= Automated file cleanup without user interaction is a bad idea, even if its ran only on low-disk space events.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowStorageSenseGlobal' -Data '0' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\StorageSense' -ValueName 'AllowStorageSenseGlobal' -Data '0' -Type 'Dword'

Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion" -Name "StorageSense" -Force

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
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\MRT' -ValueName 'DontOfferThroughWUAU' -Data '1' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_HB"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_ERROR_HB"
##+=+=
