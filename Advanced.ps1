#Requires -Version 5 -RunAsAdministrator
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

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

prefer_anydesk = $prefer_anydesk
" # AnyDesk and RustDesk.
Pause

if (!$automatic_thumbnail_clearing) {
    # Depend on the user clearing out thumbnail caches manually if they get corrupted.
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache' -Key 'Autorun' -Value '0' -Type 'DWord'
}

if (!$automatic_windows_store_app_updates) {
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\WindowsStore' -Key 'AutoDownload' -Value '2' -Type 'DWord'
}

if (!$windows_search_indexing) {
    Stop-Service WSearch
    SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch' -Key 'Start' -Value '4' -Type 'DWord'

    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\SearchSettings' -Key 'IsDeviceSearchHistoryEnabled' -Value '0' -Type 'DWord'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"
}

if ($show_hidden_files) {
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'DontPrettyPath' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'Hidden' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'HideFileExt' -Value '0' -Type 'DWord'
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'ShowSuperHidden' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState' -Key 'FullPath' -Value '1' -Type 'DWord'
}

if ($nuke_microsoft_edge) {
    Download_File "https://raw.githubusercontent.com/felikcat/nuke-microsoft-edge/master/Nuke_Microsoft_Edge.ps1" -Destination .\
    . ".\Nuke_Microsoft_Edge.ps1"
}

if (!$ethernet_power_saving) {
    $PROPERTIES = @("Advanced EEE", "Auto Disable Gigabit", "Energy Efficient Ethernet",
        "Gigabit Lite", "Green Ethernet", "Power Saving Mode",
        "Selective Suspend", "ULP", "Ultra Low Power Mode")
    # Disable features that can cause random packet loss/drop-outs.
    $PROPERTIES.ForEach({ Set-NetAdapterAdvancedProperty -Name '*' -DisplayName $_ -RegistryValue 0 })
}

if ($avoid_key_annoyances) {
    # Filter keys.
    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Accessibility\Keyboard Response' -Key 'Flags' -Value '98' -Type 'String'

    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Accessibility\StickyKeys' -Key 'Flags' -Value '482' -Type 'String'
    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Accessibility\ToggleKeys' -Key 'Flags' -Value '38' -Type 'String'
}

if (!$geolocation) {
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -Key 'DisableLocation' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -Key 'DisableLocationScripting' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -Key 'DisableWindowsLocationProvider' -Value '1' -Type 'DWord'

    SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\lfsvc' -Key 'Start' -Value '4' -Type 'DWord'

    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Settings\FindMyDevice' -Key 'LocationSyncEnabled' -Value '0' -Type 'DWord'

    Stop-Service lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($flat_mouse_sensitivity) {
    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Mouse' -Key "MouseSpeed" -Value '0' -Type 'String'
    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Mouse' -Key "MouseThreshold1" -Value '0' -Type 'String'
    SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Mouse' -Key "MouseThreshold2" -Value '0' -Type 'String'
}

if ($prefer_anydesk) {
    SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Personalization\Settings' -Key 'AcceptedPrivacyPolicy' -Value '0' -Type 'DWord'
    SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\DataCollection' -Key 'LimitDiagnosticLogCollection' -Value '1' -Type 'DWord'
    SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Remote Assistance' -Key 'fAllowToGetHelp' -Value '0' -Type 'DWord'
}
