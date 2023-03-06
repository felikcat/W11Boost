# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 1: Ensures Windows' audio ducking/attenuation is disabled.
$audio_reduction = 0

# If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$game_dvr = 0

# NOTICE: Some settings set might intersect with WiFi adapters, as tweaks are applied to all network interfaces.
$recommended_ethernet_tweaks = 1

# "Everything" can circumvent the performance impact of Windows Search Indexing while providing faster and more accurate results.
$replace_windows_search = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# Harden Windows with visual changes and security tweaks.
$harden = 1

# 0 = disable Explorer's thumbnail (images/video previews) border shadows.
# -> 0 is recommended if dark mode is used.
$thumbnail_shadows = 0

# 0 is recommended.
# System Restore problems:
# - Cannot restore backups from previous versions of Windows; can't revert Windows updates with System Restore.
# - Will revert other personal files (program settings and installations).
$system_restore = 0

# File History:
# - Is unreliable with creating snapshots of files.
# - Use Restic or TrueNAS with Syncthing for backups instead.
$file_history = 0

# 0: Disables GPS services, which always run even if there's no GPS hardware installed.
$geolocation = 1

# Lowers CPU load and prevents stuttering at high RAM usage, but increases RAM usage drastically.
# 32GB of RAM or more is recommended.
$less_game_stuttering = 1

# == Initialize ==
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Advanced.ps1 -> Requires Administrator!"
    Break
}
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_Advanced_LastRun.log")
. ".\imports.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
# ====

Write-Output "
==== Current settings ====

avoid_key_annoyances = $avoid_key_annoyances
file_history = $file_history
geolocation = $geolocation
audio_reduction = $audio_reduction
ethernet_power_saving = $ethernet_power_saving
game_dvr = $game_dvr
optimal_online_ntp = $optimal_online_ntp
recommended_ethernet_tweaks = $recommended_ethernet_tweaks
replace_windows_search = $replace_windows_search
system_restore = $system_restore

"

if (disable_ethernet_power_saving)
{
    Disable-Ethernet-Power-Saving
}

if ($thumbnail_shadows)
{
    Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 0 -Force
}
elseif (!$thumbnail_shadows)
{
    Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 2 -Force
}

if ($file_history)
{
    Set-ItemProperty -Path "HKCR:\SOFTWARE\Policies\Microsoft\Windows\FileHistory" -Name "Disabled" -Type DWord -Value 0 -Force
    Set-ItemProperty -Path "HKCR:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 3 -Force
    Enable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
}
elseif (!$file_history)
{
    Set-ItemProperty -Path "HKCR:\SOFTWARE\Policies\Microsoft\Windows\FileHistory" -Name "Disabled" -Type DWord -Value 1 -Force
    Set-ItemProperty -Path "HKCR:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 4 -Force
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"
}

if ($avoid_key_annoyances)
{
    reg.exe import ".\Non-GPO Registry\avoid_key_annoyances.reg"
}


if ($geolocation)
{
    reg.exe import ".\Non-GPO Registry\Geolocation\Enable.reg"
    Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}
elseif (!$geolocation)
{
    reg.exe import ".\Non-GPO Registry\Geolocation\Disable.reg"
    sc.exe stop lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($audio_reduction)
{
    reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio\UserDuckingPreference" /f
}
if (!$audio_reduction)
{
    reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
    reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
}

if (!$game_dvr)
{
    reg.exe import ".\Non-GPO Registry\No Game DVR.reg"
}

if ($reset_network_interface_settings)
{
    Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'
}

if (!$system_restore)
{
    reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\System Restore.reg"
    # Delete all restore points.
    vssadmin.exe delete shadows /all /quiet

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"
}

if ($harden)
{
    sc.exe stop Spooler
    reg.exe import ".\Non-GPO Registry\Harden.reg"
}

if ($less_game_stuttering)
{
    Disable-MMAgent -MemoryCompression
    Disable-MMAgent -PageCombining
}