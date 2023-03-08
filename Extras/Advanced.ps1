# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 1: Ensures Windows' audio ducking/attenuation is disabled.
$no_audio_reduction = 0

# If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$no_game_dvr = 1

# NOTICE: Some settings set might intersect with WiFi adapters, as tweaks are applied to all network interfaces.
$recommended_ethernet_tweaks = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# Harden Windows with visual changes and security tweaks.
$harden = 1

# 1 = disable Explorer's thumbnail (images/video previews) border shadows.
# -> 1 is recommended if dark mode is used.
$no_thumbnail_shadows = 1

# 1 is recommended.
# System Restore problems:
# - Cannot restore backups from previous versions of Windows; can't revert Windows updates with System Restore.
# - Will revert other personal files (program settings and installations).
$no_system_restore = 1

# File History:
# - Is unreliable with creating snapshots of files.
# - Use Restic or TrueNAS with Syncthing for backups instead.
$file_history = 0

# 0: Disables GPS services, which always run even if there's no GPS hardware installed.
$geolocation = 1

# Lowers CPU load and prevents stuttering at high RAM usage, but increases RAM usage drastically.
# 32GB of RAM or more is recommended.
$less_game_stuttering = 1

# 1: Prevents random packet loss/drop-outs in exchange for a higher battery drain.
$no_ethernet_power_saving = 1

# In games like Hogwarts Legacy this really helps reduce stuttering, but lowers Windows' overall security.
# You must replace Windows Defender with Kaspersky Free or a different anti-virus that has its own separate virtualization security.
$reduce_mitigations = 0

# == Initialize ==
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Advanced.ps1 -> Requires Administrator!"
    Break
}
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_Advanced_LastRun.log")
. ".\..\imports.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
# ====

Clear
Write-Output "
==== Current settings ====

no_audio_reduction = $no_audio_reduction
avoid_key_annoyances = $avoid_key_annoyances
ethernet_power_saving = $ethernet_power_saving
file_history = $file_history
geolocation = $geolocation
less_game_stuttering = $less_game_stuttering
no_game_dvr = $no_game_dvr
no_thumbnail_shadows = $no_thumbnail_shadows
no_system_restore = $no_system_restore
no_ethernet_power_saving = $no_ethernet_power_saving
reduce_mitigations = $reduce_mitigations

"
Pause

if ($disable_ethernet_power_saving)
{
    Disable-Ethernet-Power-Saving
}

if ($no_thumbnail_shadows)
{
    Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 2 -Force
}
elseif (!$no_thumbnail_shadows)
{
    Set-ItemProperty -Path "HKCR:\SystemFileAssociations\image" -Name "Treatment" -Type DWord -Value 0 -Force
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

if ($no_geolocation)
{
    reg.exe import ".\Non-GPO Registry\Geolocation\Disable.reg"
    sc.exe stop lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}
elseif (!$no_geolocation)
{
    reg.exe import ".\Non-GPO Registry\Geolocation\Enable.reg"
    Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if ($no_audio_reduction)
{
    reg.exe add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio" /v "UserDuckingPreference" /t REG_DWORD /d "3" /f
    reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Internet Explorer\LowRegistry\Audio\PolicyConfig\PropertyStore" /f
}
if (!$no_audio_reduction)
{
    reg.exe delete "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Multimedia\Audio\UserDuckingPreference" /f
}

if ($no_game_dvr)
{
    reg.exe import ".\Non-GPO Registry\No Game DVR.reg"
}

if ($reset_network_interface_settings)
{
    Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'
}

if ($no_system_restore)
{
    reg.exe import ".\Registry\Computer Configuration\Administrative Templates\System\System Restore.reg"
    # Delete all restore points.
    vssadmin.exe delete shadows /all /quiet

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"
}

if ($harden)
{
    reg.exe import ".\Non-GPO Registry\Harden.reg"
}

if ($less_game_stuttering)
{
    # MemoryCompression: While enabled; increases CPU load to reduce I/O load and handle Out Of Memory situations more smoothly; akin to Linux's zRAM.
    # -> Its downside is worsened stuttering in video games.
    # PageCombining: While enabled; reduces memory usage but increases CPU load.
    Disable-MMAgent -MemoryCompression
    Disable-MMAgent -PageCombining
}

if ($reduce_mitigations)
{
    reg.exe import ".\Non-GPO Registry\Reduce Mitigations.reg"
    # Unnecessary considering the previous registry entries imported, but do it anyway!
    bcdedit.exe /set hypervisorlaunchtype off

    Remove-All-ProcessMitigations
    # DEP is required for effectively all updated game anti-cheats.
    Set-ProcessMitigation -System -Enable DEP
    # CFG is required for Valorant, but also likely ESEA and FACEIT anti-cheats.
    Set-ProcessMitigation -System -Enable CFG

    # Data Execution Prevention (DEP).
    # -> EmulateAtlThunks

    # Address Space Layout Randomization (ASLR).
    # -> RequireInfo, ForceRelocateImages, BottomUp, HighEntropy

    # ControlFlowGuard (CFG).
    # -> SuppressExports, StrictCFG

    # Validate exception chains (SEHOP).
    # -> SEHOP, SEHOPTelemetry, AuditSEHOP

    # Heap integrity.
    # -> TerminateOnError
    Set-ProcessMitigation -System -Disable EmulateAtlThunks, RequireInfo, ForceRelocateImages, BottomUp, HighEntropy, SuppressExports, StrictCFG, SEHOP, SEHOPTelemetry, AuditSEHOP, TerminateOnError

    # Ensure "Data Execution Prevention" (DEP) only applies to operating system components, along with kernel-mode drivers.
    # Applying DEP to user-mode programs will slow down and break some, such as the original Deus Ex.
    bcdedit.exe /set nx Optin
}