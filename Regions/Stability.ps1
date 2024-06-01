#Requires -Version 5 -RunAsAdministrator
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

New-PSDrive -Name "HKCR" -PSProvider "Registry" -Root "HKEY_CLASSES_ROOT"

# Ask to not allow execution of experiments by Microsoft.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\PolicyManager\current\device\System' -Key 'AllowExperimentation' -Value '0' -Type 'DWord'

#region Disable automatic Application Compatibility helpers
# Disable "Program Compatibility Assistant"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'DisablePCA' -Value '1' -Type 'DWord'

# Disable "Application Compatibility Engine"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'DisableEngine' -Value '1' -Type 'DWord'

# Disable "SwitchBack Compatibility Engine"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'SbEnable' -Value '0' -Type 'DWord'

# Disable user Steps Recorder
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'DisableUAR' -Value '1' -Type 'DWord'

# Disable "Remove Program Compatibility Property Page"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'DisablePropPage' -Value '0' -Type 'DWord'

# Disable "Inventory Collector"
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\AppCompat' -Key 'DisableInventory' -Value '1' -Type 'DWord'

# Disable 'Program Compatibility Assistant' service
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services' -Key 'PcaSvc' -Value '4' -Type 'DWord'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
#endregion

# Power Throttling causes severe performance reduction for VMWare Workstation 17.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power\PowerThrottling' -Key 'PowerThrottlingOff' -Value '1' -Type 'DWord'

# Do not automatically update speech recognition and speech synthesis modules.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Speech' -Key 'AllowSpeechModelUpdate' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Key 'LinkResolveIgnoreLinkInfo' -Value '1' -Type 'DWord'

# Do not search disks to attempt fixing a missing shortcut.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Key 'NoResolveSearch' -Value '1' -Type 'DWord'
# Do not search all paths related to the missing shortcut.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Key 'NoResolveTrack' -Value '1' -Type 'DWord'

# Disable Explorer's thumbnail shadows.
SetReg -Path 'HKEY_CLASSES_ROOT\SystemFileAssociations\image' -Key 'Treatment' -Value '2' -Type 'DWord'

# Enable multiple processes for explorer.exe
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'SeparateProcess' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsDeviceSearchHistoryEnabled' -Value '0' -Type 'DWord'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

# By default Windows does not automatically back-up the registry, but just in case they change this..
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager' -Key 'EnablePeriodicBackup' -Value '0' -Type 'DWord'

# https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\FTH' -Key 'Enabled' -Value '0' -Type 'DWord'

# Sets Windows' default process priority; this is not the default for Windows Server.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\PriorityControl' -Key 'Win32PrioritySeparation' -Value '2' -Type 'DWord'

# Allow Game Mode to pivot GPU resources more towards video games.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\GameBar' -Key 'AutoGameModeEnabled' -Value '1' -Type 'DWord'

# Disable the auto-updating of offline maps.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\Maps' -Key 'AutoUpdateEnabled' -Value '0' -Type 'DWord'


# SwapEffectUpgradeEnable: https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/for-best-performance--use-dxgi-flip-model#directflip
# VRROptimizeEnable: https://devblogs.microsoft.com/directx/os-variable-refresh-rate/
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\DirectX\UserGpuPreferences' -Key 'DirectXUserGlobalSettings' -Value 'VRROptimizeEnable=1;SwapEffectUpgradeEnable=1;' -Type 'String'

# Disables hardware-accelerated GPU scheduling except for already blocked GPU VIDs listed in:
# HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\BlockList\Kernel
# Why: stuttering issues in some games, such as Half-Life: Alyx.
# NOTE: This will break DLSS Frame Generation until turned back on.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers' -Key 'HwSchMode' -Value '1' -Type 'DWord'

Enable-MMAgent -MemoryCompression

#region Disallow automatic: app updates, security scanning, and system diagnostics.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\Maintenance' -Key 'MaintenanceDisabled' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\ScheduledDiagnostic' -Key 'EnabledExecution' -Value '0' -Type 'DWord'

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
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Appx' -Key 'AllowStorageSenseGlobal' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\StorageSense' -Key 'AllowStorageSenseGlobal' -Value '0' -Type 'DWord'

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
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\MRT' -Key 'DontOfferThroughWUAU' -Value '1' -Type 'DWord'
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_HB"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\RemovalTools\MRT_ERROR_HB"
#endregion


#region Windows Update changes.
# Deny updates that Microsoft deems as causing compatibility issues.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Key 'DisableWUfBSafeguards' -Value '0' -Type 'DWord'

# Notify to download then install Windows updates; no automatic Windows updates.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Key 'AUOptions' -Value '2' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Key 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork' -Value '0' -Type 'DWord'

# Never force restarts.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Key 'NoAutoUpdate' -Value '0' -Type 'DWord'
#endregion
