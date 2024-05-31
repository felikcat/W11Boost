#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

New-PSDrive -Name "HKCR" -PSProvider "Registry" -Root "HKEY_CLASSES_ROOT"

# Ask to not allow execution of experiments by Microsoft.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PolicyManager\current\device\System', 'AllowExperimentation', '0', [RegistryValueKind]::DWord)

#region Disable automatic Application Compatibility helpers
# Disable "Program Compatibility Assistant"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'DisablePCA', '1', [RegistryValueKind]::DWord)

# Disable "Application Compatibility Engine"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'DisableEngine', '1', [RegistryValueKind]::DWord)

# Disable "SwitchBack Compatibility Engine"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'SbEnable', '0', [RegistryValueKind]::DWord)

# Disable user Steps Recorder
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'DisableUAR', '1', [RegistryValueKind]::DWord)

# Disable "Remove Program Compatibility Property Page"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'DisablePropPage', '0', [RegistryValueKind]::DWord)

# Disable "Inventory Collector"
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat', 'DisableInventory', '1', [RegistryValueKind]::DWord)

# Disable 'Program Compatibility Assistant' service
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services', 'PcaSvc', '4', [RegistryValueKind]::DWord)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
#endregion

# Power Throttling causes severe performance reduction for VMWare Workstation 17.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power\PowerThrottling', 'PowerThrottlingOff', '1', [RegistryValueKind]::DWord)

# Do not automatically update speech recognition and speech synthesis modules.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Speech', 'AllowSpeechModelUpdate', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer', 'LinkResolveIgnoreLinkInfo', '1', [RegistryValueKind]::DWord)

# Do not search disks to attempt fixing a missing shortcut.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer', 'NoResolveSearch', '1', [RegistryValueKind]::DWord)
# Do not search all paths related to the missing shortcut.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer', 'NoResolveTrack', '1', [RegistryValueKind]::DWord)

# Disable Explorer's thumbnail shadows.
[Registry]::SetValue('HKEY_CLASSES_ROOT\SystemFileAssociations\image', 'Treatment', '2', [RegistryValueKind]::DWord)

# Enable multiple processes for explorer.exe
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'SeparateProcess', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsDeviceSearchHistoryEnabled', '0', [RegistryValueKind]::DWord)
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

# By default Windows does not automatically back-up the registry, but just in case they change this..
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager', 'EnablePeriodicBackup', '0', [RegistryValueKind]::DWord)

# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\FTH', 'Enabled', '0', [RegistryValueKind]::DWord)

# Sets Windows' default process priority; this is not the default for Windows Server.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\PriorityControl', 'Win32PrioritySeparation', '2', [RegistryValueKind]::DWord)

# Allow Game Mode to pivot GPU resources more towards video games.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\GameBar', 'AutoGameModeEnabled', '1', [RegistryValueKind]::DWord)

# Disable the auto-updating of offline maps.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\Maps', 'AutoUpdateEnabled', '0', [RegistryValueKind]::DWord)


# SwapEffectUpgradeEnable: https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/for-best-performance--use-dxgi-flip-model#directflip
# VRROptimizeEnable: https://devblogs.microsoft.com/directx/os-variable-refresh-rate/
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\DirectX\UserGpuPreferences', 'DirectXUserGlobalSettings', 'VRROptimizeEnable=1;SwapEffectUpgradeEnable=1;', [RegistryValueKind]::String)

# Disables hardware-accelerated GPU scheduling except for already blocked GPU VIDs listed in:
# HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\BlockList\Kernel
# Why: stuttering issues in some games, such as Half-Life: Alyx.
# NOTE: This will break DLSS Frame Generation until turned back on.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers', 'HwSchMode', '1', [RegistryValueKind]::DWord)

Enable-MMAgent -MemoryCompression

#region Disallow automatic: app updates, security scanning, and system diagnostics.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance', 'MaintenanceDisabled', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\ScheduledDiagnostic', 'EnabledExecution', '0', [RegistryValueKind]::DWord)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\Scheduled"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Diagnosis\RecommendedTroubleshootingScanner"
#endregion

# Resets adapter settings to driver defaults; it's assumed if there were prior tweaks done, they're incorrect.
Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'

# Random disconnection fix for specific network adapters, such as Intel's I225-V.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

# Never periodically scan for other Access Points (AP) / Wi-Fi networks.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Global BG Scan blocking' -RegistryValue 2

netsh.exe int tcp set global timestamps=enabled


#region Automated file cleanup without user interaction is a bad idea, even if ran only on low-disk space events.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Appx', 'AllowStorageSenseGlobal', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\StorageSense', 'AllowStorageSenseGlobal', '0', [RegistryValueKind]::DWord)

Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion" -Name "StorageSense"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\Diagnostics"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskFootprint\StorageSense"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskCleanup\SilentCleanup"
#endregion


#region Disable these scheduler tasks to keep performance and bandwidth usage more consistent.
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentFallBack"
Disable-ScheduledTask -TaskName "\Microsoft\Office\OfficeTelemetryAgentLogOn"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\StartupAppTask"
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
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemoteAssistance\RemoteAssistanceTask"

# https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/clean-up-the-winsxs-folder?view=windows-11
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Servicing\StartComponentCleanup"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Sysmain\ResPriStaticDbSync"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Sysmain\WsSwapAssessmentTask"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WDI\ResolutionHost"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Windows Filtering Platform\BfeOnServiceStartTypeChange"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WlanSvc\CDSSync"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WOF\WIM-Hash-Management"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WOF\WIM-Hash-Validation"

Disable-ScheduledTask -TaskName "\Microsoft\Windows\WS\WSTask"

# Microsoft's Malicious Removal Tool task can pop up out of nowhere if Windows Update is still allowed to connect.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\MRT', 'DontOfferThroughWUAU', '1', [RegistryValueKind]::DWord)
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_HB"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_ERROR_HB"
#endregion


#region Windows Update changes.
# Deny updates that Microsoft deems as causing compatibility issues.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate', 'DisableWUfBSafeguards', '0', [RegistryValueKind]::DWord)

# Notify to download then install Windows updates; no automatic Windows updates.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU', 'AUOptions', '2', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate', 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork', '0', [RegistryValueKind]::DWord)

# Never force restarts.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU', 'NoAutoUpdate', '0', [RegistryValueKind]::DWord)
#endregion
