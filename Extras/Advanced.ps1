# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 1: Ensures Windows' audio ducking/attenuation is disabled.
$no_audio_reduction = 0

# If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$no_game_dvr = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# 1 = disable Explorer's thumbnail (images/video previews) border shadows.
# -> 1 is recommended if dark mode is used.
$no_thumbnail_shadows = 1

# 1 is recommended.
# System Restore problems:
# - Cannot restore backups from previous versions of Windows; can't revert Windows updates with System Restore.
# - Will revert other personal files (program settings and installations).
$no_system_restore = 1

# Windows' File History:
# - Is unreliable with creating snapshots of files.
# - Use Restic or TrueNAS with Syncthing for backups instead.
$file_history = 0

# 0: Disables GPS services, which always run even if there's no GPS hardware installed.
$geolocation = 1

# Breaks NVIDIA Control Panel purposefully; re-enable the "NVIDIA Display Container LS" service when you need to configure settings, then disable after you're done.
# Also useful: https://github.com/Orbmu2k/nvidiaProfileInspector/releases
$less_game_stuttering = 1

# 1: Prevents random packet loss/drop-outs in exchange for a higher battery drain.
$no_ethernet_power_saving = 1

# Very high level of security, but likely to break lots of older (pre-2019) PCs.
$excessive_mitigations = 0

# 1: Relevant to disable smartscreen if you use an alternative antivirus.
$no_smartscreen = 0

# 1: Reduces stuttering in Hogwarts Legacy, but lowers Windows' overall security;
# -> You must replace Windows Defender with Kaspersky Free which has its own separate virtualization security.
# -> The Vanguard, ESEA, and Faceit anti-cheats will complain about "CFG" being off; enable that yourself if needed.
$reduce_mitigations = 0


$no_family_safety = 1

# If all displays are the same resolution and scaling factor, set $improved_hidpi to 1.
$improved_hidpi = 1

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

Clear-Host
Write-Output "
==== Current settings ====

no_audio_reduction = $no_audio_reduction
avoid_key_annoyances = $avoid_key_annoyances
no_ethernet_power_saving = $no_ethernet_power_saving
file_history = $file_history
geolocation = $geolocation
less_game_stuttering = $less_game_stuttering
no_game_dvr = $no_game_dvr
no_thumbnail_shadows = $no_thumbnail_shadows
no_system_restore = $no_system_restore
excessive_mitigations = $excessive_mitigations
no_ethernet_power_saving = $no_ethernet_power_saving
reduce_mitigations = $reduce_mitigations
improved_hidpi = $improved_hidpi
no_smartscreen = $no_smartscreen

"
Pause

if ($no_ethernet_power_saving)
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

if ($excessive_mitigations)
{
    reg.exe import ".\Non-GPO Registry\Excessive Mitigations.reg"
    # Solves STIGs: V-253275 (High), V-253285 (Medium)
    Disable-WindowsOptionalFeature -NoRestart -Online -Remove -FeatureName SMB1Protocol, IIS-WebServerRole, MicrosoftWindowsPowerShellV2Root, MicrosoftWindowsPowerShellV2
    Set-ProcessMitigation -System -Enable DEP, EmulateAtlThunks, RequireInfo, BottomUp, HighEntropy, CFG, SuppressExports, SEHOP, SEHOPTelemetry, AuditSEHOP, TerminateOnError
    # StrictCFG breaks far too many programs, and even NVIDIA's GPU drivers.
    # ForceRelocateImages (Mandatory ASLR) breaks a few programs, such as Hogwarts Legacy.
    Set-ProcessMitigation -System -Disable StrictCFG ForceRelocateImages
    bcdedit.exe /set hypervisorlaunchtype auto
    bcdedit.exe /set nx OptOut
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
    Disable-WindowsOptionalFeature -NoRestart -Online -Remove -FeatureName VirtualMachinePlatform

    reg.exe import ".\Non-GPO Registry\Reduce Mitigations.reg"
    # Unnecessary considering the previous registry entries imported, but do it anyway!
    bcdedit.exe /set hypervisorlaunchtype off

    # System != Process.
    Remove-All-ProcessMitigations

    # DEP is required for effectively all updated game anti-cheats.
    Set-ProcessMitigation -System -Enable DEP

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
    Set-ProcessMitigation -System -Disable EmulateAtlThunks, RequireInfo, ForceRelocateImages, BottomUp, HighEntropy, CFG, SuppressExports, StrictCFG, SEHOP, SEHOPTelemetry, AuditSEHOP, TerminateOnError

    # Ensure "Data Execution Prevention" (DEP) only applies to operating system components, along with kernel-mode drivers.
    # Applying DEP to user-mode programs will slow or break them down, such as the original Deus Ex.
    bcdedit.exe /set nx Optin
}

if ($improved_hidpi)
{
    reg.exe import ".\Non-GPO Registry\HiDPI Blurry Font Fix.reg"
}

if ($no_smartscreen)
{
    reg.exe import ".\Non-GPO Registry\Disable SmartScreen.reg"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
}

if ($no_family_safety)
{
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyMonitor"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyRefreshTask"
}
