# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 0: If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$game_dvr = 0

# 0: Disables Biometric services meant for fingerprint readers, which runs even if there's no Biometric devices.
$biometrics = 1

# Resets all network interfaces back to their manufacturer's default settings.
# Recommended before applying our network tweaks, as it's a "clean slate".
$reset_network_interface_settings = 1

# System Restore problems:
# - Cannot restore backups from previous versions of Windows.
# - Cannot revert Windows updates.
# - Will revert other personal files (program settings and installations).
$allow_system_restore = 1

# File History will fail you when it's needed.
# - Alternative: Syncthing on this PC and a PC running either TrueNAS or Unraid.
$file_history = 0

# 0: Disables GPS services, which run even if a GPS isn't installed.
$geolocation = 1

# Breaks NVIDIA Control Panel and CPU usage in the Task Manager purposefully;
# Re-enable the "NVIDIA Display Container LS" service when you need to configure settings, then disable after you're done.
# Configure hidden NVIDIA options: https://github.com/Orbmu2k/nvidiaProfileInspector/releases
# See real CPU usage: https://systeminformer.sourceforge.io/
$less_game_stuttering = 1

# 0: Prevents random packet loss/drop-outs in exchange for higher battery drain.
$ethernet_power_saving = 0

# 0: Relevant to disable smartscreen if you use an alternative antivirus.
$defender_smartscreen = 1

# 1: Reduces stuttering in some games (Hogwarts Legacy), but lowers Windows' overall security;
# -> You must replace Windows Defender with Kaspersky Free; Kaspersky has its own separate virtualization security.
# -> The Vanguard, ESEA, and Faceit anti-cheats will complain about "CFG" being off; enable that yourself if needed.
$reduce_mitigations = 0

# 0: Recommended if Windows Defender isn't used.
$windows_security_systray = 1

# 0: Disables parental controls, which seemingly can't be used without a Microsoft account.
$family_safety = 0

# 1: Only log events with warnings or errors will be recorded.
$change_event_viewer_behavior = 1

# If having to manually unblock files you download is intolerable, use $no_blocked_files 1.
$no_blocked_files = 0

# Helps with not getting tricked into opening malware and makes Windows less annoying for a power user.
$show_hidden_files = 1

# 0: 'Everything' will be installed to be used as the alternative search indexer.
# -> The Windows Search Index is prone to causing sudden declines in performance, especially on slow hard drives (HDDs).
$windows_search_indexing = 0

# Stops Microsoft Edge from wasting CPU cycles and bandwidth while closed.
$automatic_microsoft_edge_updates = 1

# 0: Manually set compatibility modes for programs requiring say Windows XP mode.
# Potential performance and reliability benefits from forcing this to be manual (compatibility modes enabled by you only).
$automated_compatibility = 0



##+=+= END OF OPTIONS ||-> Initialize
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

allow_system_restore = $allow_system_restore
automated_compatibility = $automated_compatibility
automatic_microsoft_edge_updates = $automatic_microsoft_edge_updates
avoid_key_annoyances = $avoid_key_annoyances
biometrics = $biometrics
change_event_viewer_behavior = $change_event_viewer_behavior
defender_smartscreen = $defender_smartscreen
ethernet_power_saving = $ethernet_power_saving
file_history = $file_history
game_dvr = $game_dvr
geolocation = $geolocation
less_game_stuttering = $less_game_stuttering
no_blocked_files = $no_blocked_files
reduce_mitigations = $reduce_mitigations
show_hidden_files = $show_hidden_files
windows_search_indexing = $windows_search_indexing
windows_security_systray = $windows_security_systray
"
Pause

if (!$biometrics)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Biometrics' -ValueName 'Enabled' -Data '0' -Type 'Dword'
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\WbioSrvc' -ValueName 'Start' -Data '4' -Type 'Dword'
}

if (!$automated_compatibility)
{
    # Disable "Program Compatibility Assistant".
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePCA' -Data '1' -Type 'Dword'

    # Disable "Application Compatibility Engine".
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableEngine' -Data '1' -Type 'Dword'

    # Disable "SwitchBack Compatibility Engine".
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'SbEnable' -Data '0' -Type 'Dword'

    # Disable user Steps Recorder.
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableUAR' -Data '1' -Type 'Dword'

    # Disable "Remove Program Compatibility Property Page".
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisablePropPage' -Data '0' -Type 'Dword'

    # Disable "Inventory Collector".
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\AppCompat' -ValueName 'DisableInventory' -Data '1' -Type 'Dword'

    # Disable 'Program Compatibility Assistant' service
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\PcaSvc" /v "Start" /t REG_DWORD /d 4 /f

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
}

if (!$automatic_microsoft_edge_updates)
{
    # Disable opening Edge on Windows startup: for the Chromium version.
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Edge' -ValueName 'StartupBoostEnabled' -Data '0' -Type 'Dword'

    # Disable opening Edge on Windows startup: for the legacy version.
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main' -ValueName 'AllowPrelaunch' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader' -ValueName 'AllowTabPreloading' -Data '0' -Type 'Dword'

    # Do not auto-update Microsoft Edge while it's closed.
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\MicrosoftEdgeElevationService' -ValueName 'Start' -Data '4' -Type 'Dword'
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\edgeupdate' -ValueName 'Start' -Data '4' -Type 'Dword'
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\edgeupdatem' -ValueName 'Start' -Data '4' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\MicrosoftEdgeUpdateTaskMachineCore"
    Disable-ScheduledTask -TaskName "\MicrosoftEdgeUpdateTaskMachineUA"

    reg.exe delete "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree\MicrosoftEdgeUpdateTaskMachineCore" /f
    reg.exe delete "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree\MicrosoftEdgeUpdateTaskMachineUA" /f
}

if (!$windows_search_indexing)
{
    sc.exe stop WSearch
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\WSearch" /v "Start" /t REG_DWORD /d 4 /f

    Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

    winget.exe install voidtools.Everything -s winget -eh --accept-package-agreements --accept-source-agreements
}

if ($show_hidden_files)
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

if (!$ethernet_power_saving)
{
    $properties = @("Advanced EEE", "Auto Disable Gigabit", "Energy Efficient Ethernet",
    "Gigabit Lite", "Green Ethernet", "Power Saving Mode",
    "Selective Suspend", "ULP", "Ultra Low Power Mode")
    for ($i = 0; $i -lt $properties.length; $i++) {
        # Disable features that can cause random packet loss/drop-outs.
        Set-NetAdapterAdvancedProperty -Name '*' -DisplayName $properties[$i] -RegistryValue 0
    }
}

if (!$file_history)
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

if (!$geolocation)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableLocation' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableLocationScripting' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors' -ValueName 'DisableWindowsLocationProvider' -Data '1' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\lfsvc' -ValueName 'Start' -Data '4' -Type 'Dword'

    sc.exe stop lfsvc
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
}

if (!$game_dvr)
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

if (!$allow_system_restore)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore' -ValueName 'DisableSR' -Data '1' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\SystemRestore\SR"

    # Delete all restore points.
    vssadmin.exe delete shadows /all /quiet
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

    # DEP is required for effectively all maintained game anti-cheats.
    # Notes:
    # - VAC requires both the client and server DLLs to be signed with a certificate to detect cheaters; see:
    # - https://github.com/ValveSoftware/source-sdk-2013/issues/76#issuecomment-21562961
    # - This means disabling DEP works for some VAC "enabled" games, but you can't play CS:GO or TF2 for an hour straight without VAC errors.
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

    # Ensure "Data Execution Prevention" (DEP) only applies to operating system components and all kernel-mode drivers.
    # Applying DEP to user-mode programs will slow or break them down, such as the Deus Ex (2000) video game.
    bcdedit.exe /set nx Optin
}

if (!$defender_smartscreen)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableSmartScreen' -Data '0' -Type 'Dword'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -ValueName 'SmartScreenEnabled' -Data 'Off' -Type 'String'

    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -ValueName 'EnableWebContentEvaluation' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
}

if (!$family_safety)
{
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyMonitor"
    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\FamilySafetyRefreshTask"
}

if (!$windows_security_systray)
{
    Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -ValueName 'HideSystray' -Data '1' -Type 'Dword'
}

if ($change_event_viewer_behavior)
{
    # Don't log events without warnings or errors.
    auditpol.exe /set /category:* /Success:disable
}
