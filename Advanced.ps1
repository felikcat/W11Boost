#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

# 0: Assumption that thumbnail corruption is rare; run the 'Disk Cleanup' app if it happens.
$automatic_thumbnail_clearing = 0

# 0: Stops apps installed from the Windows Store from updating themselves.
# -> Alternative: open PowerShell as administrator to then run `winget upgrade --all`.
$automatic_windows_store_app_updates = 1


# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 0: Prevents random packet loss/drop-outs in exchange for higher battery drain.
$ethernet_power_saving = 0

# 1: No mouse acceleration, which also helps old video games not using RawInput.
$flat_mouse_sensitivity = 1

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

Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"

. "..\Regions\IMPORTS.ps1"
##+=+=

Clear-Host
Write-Output "
==== Current settings ====

automatic_thumbnail_clearing = $automatic_thumbnail_clearing
automatic_windows_store_app_updates = $automatic_windows_store_app_updates

avoid_key_annoyances = $avoid_key_annoyances
ethernet_power_saving = $ethernet_power_saving
flat_mouse_sensitivity = $flat_mouse_sensitivity
geolocation = $geolocation
nuke_microsoft_edge = $nuke_microsoft_edge

show_hidden_files = $show_hidden_files

windows_search_indexing = $windows_search_indexing
"
Pause

if (!$automatic_thumbnail_clearing)
{
    # Depend on the user clearing out thumbnail caches manually if they get corrupted.
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache', 'Autorun', '0', [RegistryValueKind]::DWord)
}

if (!$automatic_windows_store_app_updates)
{
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\WindowsStore', 'AutoDownload', '2', [RegistryValueKind]::DWord)
}

if (!$windows_search_indexing)
{
    Stop-Service WSearch
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch', 'Start', '4', [RegistryValueKind]::DWord)

    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings', 'IsDeviceSearchHistoryEnabled', '0', [RegistryValueKind]::DWord)

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"
}

if ($show_hidden_files)
{
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'DontPrettyPath', '1', [RegistryValueKind]::DWord)
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'Hidden', '1', [RegistryValueKind]::DWord)
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'HideFileExt', '0', [RegistryValueKind]::DWord)
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'ShowSuperHidden', '1', [RegistryValueKind]::DWord)
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState', 'FullPath', '1', [RegistryValueKind]::DWord)
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

if ($avoid_key_annoyances)
{
    # Filter keys.
    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Accessibility\Keyboard Response', 'Flags', '98', [RegistryValueKind]::String)

    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Accessibility\StickyKeys', 'Flags', '482', [RegistryValueKind]::String)

    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Accessibility\ToggleKeys', 'Flags', '38', [RegistryValueKind]::String)
}

if (!$geolocation)
{
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors', 'DisableLocation', '1', [RegistryValueKind]::DWord)

    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors', 'DisableLocationScripting', '1', [RegistryValueKind]::DWord)

    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors', 'DisableWindowsLocationProvider', '1', [RegistryValueKind]::DWord)

    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\lfsvc', 'Start', '4', [RegistryValueKind]::DWord)

    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Settings\FindMyDevice', 'LocationSyncEnabled', '0', [RegistryValueKind]::DWord)

    Stop-Service lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($flat_mouse_sensitivity)
{
    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Mouse', "MouseSpeed" , '0', [RegistryValueKind]::String)
    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Mouse', "MouseThreshold1", '0', [RegistryValueKind]::String)
    [Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Mouse', "MouseThreshold2", '0', [RegistryValueKind]::String)
}
