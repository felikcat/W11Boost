#Requires -Version 5 -RunAsAdministrator

New-PSDrive -Name "HKCR" -PSProvider "Registry" -Root "HKEY_CLASSES_ROOT" 

#region Disable Game DVR and Game Bar
PEAdd_HKCU 'System\GameConfigStore' -Name 'GameDVR_Enabled' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\GameDVR' -Name 'AllowGameDVR' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -Name 'AppCaptureEnabled' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -Name 'CursorCaptureEnabled' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -Name 'HistoricalCaptureEnabled' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -Name 'AudioCaptureEnabled' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -Name 'MicrophoneCaptureEnabledByDefault' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -Name 'CursorCaptureEnabled' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -Name 'CameraCaptureEnabledByDefault' -Value '0' -Type 'Dword'

# Block Xbox controller's home button from opening the game bar.
PEAdd_HKCU 'Software\Microsoft\GameBar' -Name 'UseNexusForGameBarEnabled' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\GameBar' -Name 'ShowStartupPanel' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SYSTEM\CurrentControlSet\Services\BcastDVRUserService' -Name 'Start' -Value '4' -Type 'Dword'
#endregion

#region Disable System Restore and File History; unreliable crap.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore' -Name 'DisableSR' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore' -Name 'RPSessionInterval' -Value '0' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"

# Remove all system restore points.
Get-CimInstance Win32_ShadowCopy | Remove-CimInstance

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\FileHistory' -Name 'Disabled' -Value '1' -Type 'Dword'
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 4
Disable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
#endregion

# Ask to not allow execution of experiments by Microsoft.
PEAdd_HKLM 'SOFTWARE\Microsoft\PolicyManager\current\device\System' -Name 'AllowExperimentation' -Value '0' -Type 'Dword'

#region Disable automatic Application Compatibility helpers
# Disable "Program Compatibility Assistant"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'DisablePCA' -Value '1' -Type 'Dword'

# Disable "Application Compatibility Engine"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'DisableEngine' -Value '1' -Type 'Dword'

# Disable "SwitchBack Compatibility Engine"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'SbEnable' -Value '0' -Type 'Dword'

# Disable user Steps Recorder
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'DisableUAR' -Value '1' -Type 'Dword'

# Disable "Remove Program Compatibility Property Page"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'DisablePropPage' -Value '0' -Type 'Dword'

# Disable "Inventory Collector"
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Name 'DisableInventory' -Value '1' -Type 'Dword'

# Disable 'Program Compatibility Assistant' service
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Services' -Name 'PcaSvc' -Value '4' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
#endregion

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'HideFastUserSwitching' -Value '1' -Type 'Dword'

# Power Throttling causes severe performance reduction for VMWare Workstation 17.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\Power\PowerThrottling' -Name 'PowerThrottlingOff' -Value '1' -Type 'Dword'

# Do not automatically update speech recognition and speech synthesis modules.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Speech' -Name 'AllowSpeechModelUpdate' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Name 'LinkResolveIgnoreLinkInfo' -Value '1' -Type 'Dword'

# Do not search disks to attempt fixing a missing shortcut.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Name 'NoResolveSearch' -Value '1' -Type 'Dword'
# Do not search all paths related to the missing shortcut.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Name 'NoResolveTrack' -Value '1' -Type 'Dword'

# Disable Explorer's thumbnail shadows.
Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 2

# W11Boost does changes that Windows developer builds would likely not anticipate.
PEAdd_HKLM 'SOFTWARE\Microsoft\WindowsSelfHost\UI\Visibility' -Name 'HideInsiderPage' -Value '1' -Type 'Dword'

# Enable multiple processes for explorer.exe
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'SeparateProcess' -Value '1' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Name 'IsDeviceSearchHistoryEnabled' -Value '0' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

# By default Windows does not automatically back-up the registry, but just in case they change this..
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager' -Name 'EnablePeriodicBackup' -Value '0' -Type 'Dword'

# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
PEAdd_HKLM 'SOFTWARE\Microsoft\FTH' -Name 'Enabled' -Value '0' -Type 'Dword'

# Sets Windows' default process priority; this is not the default for Windows Server.
Set-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\PriorityControl' -Name 'Win32PrioritySeparation' -Value '2' -Type 'Dword'

Set-ItemProperty -Path 'HKCU:\Software\Microsoft\GameBar' -Name 'AutoGameModeEnabled' -Value '1' -Type 'Dword'

Set-ItemProperty -Path 'HKLM:\SYSTEM\Maps' -Name 'AutoUpdateEnabled' -Value '0' -Type 'Dword'


# SwapEffectUpgradeEnable: https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/for-best-performance--use-dxgi-flip-model#directflip
# VRROptimizeEnable: https://devblogs.microsoft.com/directx/os-variable-refresh-rate/
Set-ItemProperty -Path 'HKCU:\Software\Microsoft\DirectX\UserGpuPreferences' -Name 'DirectXUserGlobalSettings' -Value 'VRROptimizeEnable=1;SwapEffectUpgradeEnable=1;' -Type 'String'

# Enables hardware-accelerated GPU scheduling except for blocked GPU VIDs listed in:
# HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\BlockList\Kernel
Set-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers' -Name 'HwSchMode' -Type 'DWord' -Value '2'

Enable-MMAgent -MemoryCompression
Disable-MMAgent -PageCombining


#region Disallow automatic: app updates, security scanning, and system diagnostics.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance' -Name 'MaintenanceDisabled' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\ScheduledDiagnostic' -Name 'EnabledExecution' -Value '0' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"
#endregion

# Resets adapter settings to driver defaults; it's assumed if there were prior tweaks done, they're incorrect.
Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'

# Random disconnection fix for specific network adapters, such as Intel's I225-V
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

# Never periodically scan for other Access Points (AP) / Wi-Fi networks
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Global BG Scan blocking' -RegistryValue 2

netsh.exe int tcp set global timestamps=enabled


#region Automated file cleanup without user interaction is a bad idea, even if its ran only on low-disk space events.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Appx' -Name 'AllowStorageSenseGlobal' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\StorageSense' -Name 'AllowStorageSenseGlobal' -Value '0' -Type 'Dword'

Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion" -Name "StorageSense"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\Diagnostics"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\StorageSense"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskCleanup\SilentCleanup"
#endregion


#region Disable these scheduler tasks to keep performance and bandwidth usage more consistent.
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentFallBack"
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentLogOn"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\StartupAppTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\ApplicationData\DsSvcCleanup"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppxDeploymentClient\Pre-staged app cleanup"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\CertificateServicesClient\UserTask-Roam"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Clip\License Validation"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\CloudExperienceHost\CreateObjectTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\File Classification Infrastructure\Property Definition Sync"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\SmartRetry"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\International\Synchronize Language Settings"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\LanguageComponentsInstaller\ReconcileLanguageResources"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Maintenance\WinSAT"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\MUI\LPRemove"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\NetTrace\GatherNetworkInfo"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\PI\Sqm-Tasks"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Power Efficiency Diagnostics\AnalyzeSystem"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Printing\EduPrintProv"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\BackgroundUploadTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\SettingSync\NetworkStateChangeTask"

# https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/clean-up-the-winsxs-folder?view=windows-11
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Servicing\StartComponentCleanup"

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

# Microsoft's Malicious Removal Tool task can pop up out of nowhere if Windows Update is still allowed to connect.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\MRT' -Name 'DontOfferThroughWUAU' -Value '1' -Type 'Dword'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_HB"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_ERROR_HB"
#endregion


#region Windows Update changes.
# Deny pre-release (Release Preview, Beta, or Dev channel) updates.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'ManagePreviewBuildsPolicyValue' -Value '1' -Type 'Dword'

# Deny updates that Microsoft deems as causing compatibility issues.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'DisableWUfBSafeguards' -Value '0' -Type 'Dword'

# Opt out out of "being the first to get the latest non-security updates".
PEAdd_HKLM 'SOFTWARE\Microsoft\WindowsUpdate\UX\Settings' -Name 'IsContinuousInnovationOptedIn' -Value '0' -Type 'Dword'

# Notify to download then install Windows updates; no automatic Windows updates.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Name 'AUOptions' -Value '2' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork' -Value '0' -Type 'Dword'

# Never force restarts.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Name 'NoAutoUpdate' -Value '0' -Type 'Dword'

# Disable Delivery Optimization's "Allow downloads from other PCs".
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization' -Name 'DODownloadMode' -Value '0' -Type 'Dword'
#endregion
