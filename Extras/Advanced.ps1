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

# Breaks NVIDIA Control Panel and CPU usage in the Task Manager purposefully;
# Re-enable the "NVIDIA Display Container LS" service when you need to configure settings, then disable after you're done.
# Configure hidden NVIDIA options: https://github.com/Orbmu2k/nvidiaProfileInspector/releases
# See real CPU usage: https://systeminformer.sourceforge.io/
$less_game_stuttering = 1

# 1: Prevents random packet loss/drop-outs in exchange for a higher battery drain.
$no_ethernet_power_saving = 1

# Requires Intel 11th gen / AMD Zen 3 or newer CPUs; not compatible with 'reduce_mitigations = 1'.
$excessive_mitigations = 0

# 1: Relevant to disable smartscreen if you use an alternative antivirus.
$no_smartscreen = 1

# 1: Reduces stuttering in Hogwarts Legacy, but lowers Windows' overall security;
# -> You must replace Windows Defender with Kaspersky Free which has its own separate virtualization security.
# -> The Vanguard, ESEA, and Faceit anti-cheats will complain about "CFG" being off; enable that yourself if needed.
$reduce_mitigations = 0

# SecurityHealthSystray.exe is redundant if Windows Defender isn't used.
$no_windows_security_systray = 1

$no_family_safety = 1

# If all displays are the same resolution and scaling factor, set $improved_hidpi to 1.
$improved_hidpi = 0

# 1: Only log events with warnings or errors will be recorded.
$change_event_viewer_behavior = 1

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
# Potentially reduces OpenSSL's security to increase compatibility with older OpenSSL on 10th gen Intel CPUs and newer.
$fix_openssl_sha_crash = 1

# If having to manually unblock files you download is intolerable, use $no_blocked_files 1.
$no_blocked_files = 0

# Helps with not getting tricked into opening malware and makes Windows less annoying for a power user.
$no_hidden_files = 1

$no_windows_search_indexing = 1


##+=+= Initialize
if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Advanced.ps1 -> Requires Administrator!"
    Break
}
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_Advanced_LastRun.log")
. ".\..\imports.ps1"
Unblock-File -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. ".\Third-party\PolicyFileEditor\Commands.ps1"
New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
##+=+=

Clear-Host
Write-Output "
==== Current settings ====

avoid_key_annoyances = $avoid_key_annoyances
change_event_viewer_behavior = $change_event_viewer_behavior
excessive_mitigations = $excessive_mitigations
file_history = $file_history
fix_openssl_sha_crash = $fix_openssl_sha_crash
geolocation = $geolocation
improved_hidpi = $improved_hidpi
less_game_stuttering = $less_game_stuttering
no_audio_reduction = $no_audio_reduction
no_blocked_files = $no_blocked_files
no_ethernet_power_saving = $no_ethernet_power_saving
no_game_dvr = $no_game_dvr
no_hidden_files = $no_hidden_files
no_smartscreen = $no_smartscreen
no_system_restore = $no_system_restore
no_thumbnail_shadows = $no_thumbnail_shadows
no_windows_security_systray = $no_windows_security_systray
reduce_mitigations = $reduce_mitigations
no_windows_search_indexing = $no_windows_search_indexing
"
Pause

# Replacing the Windows Search Index, as it's prone to causing sudden declines in performance.
if ($no_windows_search_indexing)
{
    sc.exe stop WSearch
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v "Start" /t REG_DWORD /d 4 /f

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

    # --source winget prevents error 0x8a150044 if the Windows Store isn't reachable.
    winget.exe install voidtools.Everything -eh --accept-package-agreements --accept-source-agreements --source winget
}

if ($no_hidden_files)
{
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'DontPrettyPath' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Hidden' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'HideFileExt' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSuperHidden' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState' -ValueName 'FullPath' -Data '1' -Type 'Dword'
}

if ($no_blocked_files)
{
    # SaveZoneInformation 1 = disables blocking downloaded files.
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'
    # Don't block downloaded files in Explorer, also fixes File History not working for downloaded files.
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'
}

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
    # Filter keys.
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Control Panel\Accessibility\Keyboard Response' -ValueName 'Flags' -Data '98' -Type 'String'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Control Panel\Accessibility\StickyKeys' -ValueName 'Flags' -Data '482' -Type 'String'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Control Panel\Accessibility\ToggleKeys' -ValueName 'Flags' -Data '38' -Type 'String'
}

if ($no_geolocation)
{
    reg.exe import ".\Registry\Geolocation\Disable.reg"
    sc.exe stop lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}
elseif (!$no_geolocation)
{
    reg.exe import ".\Registry\Geolocation\Enable.reg"
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
    Set-PolicyFileEntry -Path $PREG_USER -Key 'System\GameConfigStore' -ValueName 'GameDVR_Enabled' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\GameDVR' -ValueName 'AllowGameDVR' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -ValueName 'AppCaptureEnabled' -Data '0' -Type 'Dword'
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\GameDVR' -ValueName 'HistoricalCaptureEnabled' -Data '0' -Type 'Dword'

    # Block Xbox controller's home button from opening the game bar.
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\GameBar' -ValueName 'UseNexusForGameBarEnabled' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\GameBar' -ValueName 'ShowStartupPanel' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\BcastDVRUserService' -ValueName 'Start' -Data '4' -Type 'Dword'

}

if ($reset_network_interface_settings)
{
    Reset-NetAdapterAdvancedProperty -Name '*' -DisplayName '*'
}

if ($no_system_restore)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore' -ValueName 'DisableSR' -Data '1' -Type 'Dword'
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"

    # Delete all restore points.
    vssadmin.exe delete shadows /all /quiet
}

if ($excessive_mitigations)
{
    reg.exe import ".\Registry\Excessive Mitigations.reg"
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
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\NVDisplay.ContainerLocalSystem' -ValueName 'Start' -Data '4' -Type 'Dword'

    powercfg.exe -import "..\Third-party\Bitsum-Highest-Performance.pow"
    $_p = Get-CimInstance -Name root\cimv2\power -Class win32_PowerPlan -Filter "ElementName = 'Bitsum Highest Performance'"
    powercfg.exe /setactive ([string]$_p.InstanceID).Replace("Microsoft:PowerPlan\{","").Replace("}","")

    # Will make CPU usage appear at constantly 100% in Windows' Task Manager.
    # Makes CPU idle states never occur to reduce DPC latency and increase the speed of context switching.
    powercfg.exe /setacvalueindex SCHEME_CURRENT SUB_PROCESSOR IdleDisable 1
    powercfg.exe /setactive SCHEME_CURRENT
}

if ($reduce_mitigations)
{
    Disable-WindowsOptionalFeature -NoRestart -Online -Remove -FeatureName VirtualMachinePlatform

    reg.exe import ".\Registry\Reduce Mitigations.reg"
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
    Set-PolicyFileEntry -Path $PREG_USER -Key 'Control Panel\Desktop' -ValueName 'EnablePerProcessSystemDPI' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Control Panel\Desktop' -ValueName 'EnablePerProcessSystemDPI' -Data '1' -Type 'Dword'
}

if ($no_smartscreen)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableSmartScreen' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -ValueName 'SmartScreenEnabled' -Data 'Off' -Type 'String'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -ValueName 'EnableWebContentEvaluation' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
}

if ($no_family_safety)
{
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyMonitor"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyRefreshTask"
}

if ($no_windows_security_systray)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -ValueName 'HideSystray' -Data '1' -Type 'Dword'
}

if ($change_event_viewer_behavior)
{
    # Don't log events without warnings or errors.
    auditpol.exe /set /category:* /Success:disable
}

if ($fix_openssl_sha_crash)
{
    [Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", "~0x200000200000000", "Machine")
}