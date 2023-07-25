#Requires -Version 5 -RunAsAdministrator

# System Restore problems:
# - Cannot restore backups from previous versions of Windows.
# - Cannot revert Windows updates.
# - Will revert other personal files (program settings and installations).
$allow_system_restore = 0


# 0: Manually set compatibility modes for programs requiring say Windows XP mode.
# Potential performance and reliability benefits from forcing this to be manual (compatibility modes enabled by you only).
$automatic_compatibility = 0

# 0: Assumption that thumbnail corruption is rare; run the 'Disk Cleanup' program if it happens.
$automatic_thumbnail_clearing = 0

# 0: Stops apps installed from the Windows Store from updating themselves.
# -> Alternative: open PowerShell as administrator to then run `winget upgrade --all`.
$automatic_windows_store_app_updates = 1


# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 0: Prevents random packet loss/drop-outs in exchange for higher battery drain.
$ethernet_power_saving = 0

# File History will fail you when it's needed.
# - Alternative: Syncthing on this PC and a PC running either TrueNAS or Unraid.
$file_history = 0

# 1: No mouse acceleration, which also helps old video games not using RawInput.
$flat_mouse_sensitivity = 1

# 0: If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$game_dvr = 0

# 0: Disables GPS services, which run even if a GPS is not installed.
$geolocation = 1

# 1: Removes Microsoft Edge.
$nuke_microsoft_edge = 0

# Helps with not getting tricked into opening malware and makes Windows less annoying for a power user.
$show_hidden_files = 1


# 0: Disables the Windows Search Index.
# It is prone to causing sudden declines in performance, more so on slow hard drives (HDDs).
# Use "Installer 64-bit" from https://www.voidtools.com/ if the search becomes intolerable for you.
$windows_search_indexing = 0


##+=+= END OF OPTIONS ||-> Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\W11Boost_Advanced_LastRun.log")

Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"

. "..\Regions\IMPORTS.ps1"
##+=+=

Clear-Host
Write-Output "
==== Current settings ====
allow_system_restore = $allow_system_restore

automatic_compatibility = $automatic_compatibility
automatic_thumbnail_clearing = $automatic_thumbnail_clearing
automatic_windows_store_app_updates = $automatic_windows_store_app_updates

avoid_key_annoyances = $avoid_key_annoyances
ethernet_power_saving = $ethernet_power_saving
file_history = $file_history
flat_mouse_sensitivity = $flat_mouse_sensitivity
game_dvr = $game_dvr
geolocation = $geolocation
nuke_microsoft_edge = $nuke_microsoft_edge

show_hidden_files = $show_hidden_files

windows_search_indexing = $windows_search_indexing
"
Pause

if (!$automatic_windows_store_app_updates)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\WindowsStore' -ValueName 'AutoDownload' -Data '2' -Type 'Dword'
}

if (!$automatic_thumbnail_clearing)
{
    # Depend on the user clearing out thumbnail caches manually if they get corrupted.
    PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache' -ValueName 'Autorun' -Data '0' -Type 'Dword'
}

if (!$automatic_compatibility)
{
    # Disable "Program Compatibility Assistant".
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePCA' -Data '1' -Type 'Dword'

    # Disable "Application Compatibility Engine".
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableEngine' -Data '1' -Type 'Dword'

    # Disable "SwitchBack Compatibility Engine".
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'SbEnable' -Data '0' -Type 'Dword'

    # Disable user Steps Recorder.
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableUAR' -Data '1' -Type 'Dword'

    # Disable "Remove Program Compatibility Property Page".
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePropPage' -Data '0' -Type 'Dword'

    # Disable "Inventory Collector".
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableInventory' -Data '1' -Type 'Dword'

    # Disable 'Program Compatibility Assistant' service
    PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services' -ValueName 'PcaSvc' -Data '4' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
}

if (!$windows_search_indexing)
{
    Stop-Service WSearch
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\WSearch" -Name "Start" -Type DWord -Value 4

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"
}

if ($show_hidden_files)
{
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'DontPrettyPath' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Hidden' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'HideFileExt' -Data '0' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSuperHidden' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState' -ValueName 'FullPath' -Data '1' -Type 'Dword'
}

if ($nuke_microsoft_edge)
{
    Download_File "https://raw.githubusercontent.com/felikcat/nuke-microsoft-edge/master/Nuke_Microsoft_Edge.ps1" -Destination .\
    . ".\Nuke_Microsoft_Edge.ps1"
}

if (!$ethernet_power_saving)
{
    $PROPERTIES = @("Advanced EEE", "Auto Disable Gigabit", "Energy Efficient Ethernet",
    "Gigabit Lite", "Green Ethernet", "Power Saving Mode",
    "Selective Suspend", "ULP", "Ultra Low Power Mode")
    # Disable features that can cause random packet loss/drop-outs.
    $PROPERTIES.ForEach({Set-NetAdapterAdvancedProperty -Name '*' -DisplayName $_ -RegistryValue 0})
}

if (!$file_history)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\FileHistory' -ValueName 'Disabled' -Data '1' -Type 'Dword'
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 4
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
}

if ($avoid_key_annoyances)
{
    # Filter keys.
    PolEdit_HKCU 'Control Panel\Accessibility\Keyboard Response' -ValueName 'Flags' -Data '98' -Type 'String'

    PolEdit_HKCU 'Control Panel\Accessibility\StickyKeys' -ValueName 'Flags' -Data '482' -Type 'String'

    PolEdit_HKCU 'Control Panel\Accessibility\ToggleKeys' -ValueName 'Flags' -Data '38' -Type 'String'
}

if (!$geolocation)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableLocation' -Data '1' -Type 'Dword'

    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableLocationScripting' -Data '1' -Type 'Dword'

    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableWindowsLocationProvider' -Data '1' -Type 'Dword'

    PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services\lfsvc' -ValueName 'Start' -Data '4' -Type 'Dword'

    PolEdit_HKLM 'SOFTWARE\Microsoft\Settings\FindMyDevice' -ValueName 'LocationSyncEnabled' -Data '0' -Type 'Dword'

    Stop-Service lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if (!$game_dvr)
{
    PolEdit_HKCU 'System\GameConfigStore' -ValueName 'GameDVR_Enabled' -Data '0' -Type 'Dword'

    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\GameDVR' -ValueName 'AllowGameDVR' -Data '0' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -ValueName 'AppCaptureEnabled' -Data '0' -Type 'Dword'
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -ValueName 'CursorCaptureEnabled' -Data '0' -Type 'Dword'
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -ValueName 'HistoricalCaptureEnabled' -Data '0' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -ValueName 'AudioCaptureEnabled' -Data '0' -Type 'Dword'
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -ValueName 'MicrophoneCaptureEnabledByDefault' -Data '0' -Type 'Dword'
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -ValueName 'CursorCaptureEnabled' -Data '0' -Type 'Dword'
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\AppBroadcast\GlobalSettings' -ValueName 'CameraCaptureEnabledByDefault' -Data '0' -Type 'Dword'

    # Block Xbox controller's home button from opening the game bar.
    PolEdit_HKCU 'Software\Microsoft\GameBar' -ValueName 'UseNexusForGameBarEnabled' -Data '0' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\GameBar' -ValueName 'ShowStartupPanel' -Data '0' -Type 'Dword'

    PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services\BcastDVRUserService' -ValueName 'Start' -Data '4' -Type 'Dword'
}

if (!$allow_system_restore)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore' -ValueName 'DisableSR' -Data '1' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"

    # Remove all system restore points.
    Get-CimInstance Win32_ShadowCopy | Remove-CimInstance
}

if ($flat_mouse_sensitivity)
{
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -Type String -Value 0
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -Type String -Value 0
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -Type String -Value 0
}
