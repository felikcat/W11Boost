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

# 0: Apps installed from the Windows Store don't automatically update.
# -> It's recommended to occasionally open PowerShell as administrator to then run `winget upgrade --all`.
$automatic_windows_store_app_updates = 0


# Disables Sticky, Filter, and Toggle Keys.
$avoid_key_annoyances = 1

# 1: Only log events with warnings or errors will be recorded.
$change_event_viewer_behavior = 1

# 0: Relevant to disable smartscreen if you use an alternative antivirus.
$defender_smartscreen = 1

# 0: Prevents random packet loss/drop-outs in exchange for higher battery drain.
$ethernet_power_saving = 0

# File History will fail you when it's needed.
# - Alternative: Syncthing on this PC and a PC running either TrueNAS or Unraid.
$file_history = 0

# 1: No mouse acceleration, which also helps old video games that don't use RawInput.
$flat_mouse_sensitivity = 1

# 0: If NVIDIA ShadowPlay, AMD ReLive, or OBS Studio is used instead.
$game_dvr = 0

# 0: Disables GPS services, which run even if a GPS isn't installed.
$geolocation = 1

# 1: If having to manually unblock files you download is intolerable.
$no_blocked_files = 0

# 1: Gives Microsoft Edge a cleveland steamer.
$nuke_microsoft_edge = 0

# 1: Reduces stuttering in some games (Hogwarts Legacy), but lowers Windows' overall security;
#   -> Replace Windows Defender with an alternative antivirus using its own virtualization security, such as Kaspersky Free.
#   The Vanguard, ESEA, and Faceit anti-cheats will not run if "CFG" is off; enable "CFG" yourself if needed:
#   -> https://support.microsoft.com/en-us/windows/turn-on-control-flow-guard-on-your-computer-34838428-2471-4d77-8fd6-b8a8fba507a3
$reduce_mitigations = 1

# Helps with not getting tricked into opening malware and makes Windows less annoying for a power user.
$show_hidden_files = 1


# 0: 'Everything' will be installed to be used as the alternative search indexer.
# -> The Windows Search Index is prone to causing sudden declines in performance, especially on slow hard drives (HDDs).
$windows_search_indexing = 0

# 0: Recommended if Windows Defender isn't used.
$windows_security_systray = 1

##+=+= END OF OPTIONS ||-> Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\W11Boost_Advanced_LastRun.log")

Unblock-File -Path ".\..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path ".\..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. ".\..\Third-party\PolicyFileEditor\Commands.ps1"

. ".\..\imports.ps1"
##+=+=

Clear-Host
Write-Output "
==== Current settings ====
allow_system_restore = $allow_system_restore

automatic_compatibility = $automatic_compatibility
automatic_thumbnail_clearing = $automatic_thumbnail_clearing
automatic_windows_store_app_updates = $automatic_windows_store_app_updates

avoid_key_annoyances = $avoid_key_annoyances
change_event_viewer_behavior = $change_event_viewer_behavior
defender_smartscreen = $defender_smartscreen
ethernet_power_saving = $ethernet_power_saving
file_history = $file_history
flat_mouse_sensitivity = $flat_mouse_sensitivity
game_dvr = $game_dvr
geolocation = $geolocation
no_blocked_files = $no_blocked_files
nuke_microsoft_edge = $nuke_microsoft_edge

reduce_mitigations = $reduce_mitigations
show_hidden_files = $show_hidden_files

windows_search_indexing = $windows_search_indexing
windows_security_systray = $windows_security_systray
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
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\WSearch" -Name "Start" -Type DWord -Value 4 -Force

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\SearchSettings' -ValueName 'IsDeviceSearchHistoryEnabled' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

    .\..\Third-party\MinSudo.exe --NoLogo powershell.exe -Command "winget.exe install voidtools.Everything -s winget -eh --accept-package-agreements --accept-source-agreements"
}

if ($show_hidden_files)
{
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'DontPrettyPath' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Hidden' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'HideFileExt' -Data '0' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSuperHidden' -Data '1' -Type 'Dword'

    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\CabinetState' -ValueName 'FullPath' -Data '1' -Type 'Dword'
}

if ($no_blocked_files)
{
    # SaveZoneInformation 1 = disables blocking downloaded files.
    PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'
    # Don't block downloaded files in Explorer, also fixes File History not working for downloaded files.
    PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '1' -Type 'Dword'
}

if ($nuke_microsoft_edge)
{
    Download_File "https://raw.githubusercontent.com/felikcat/nuke-microsoft-edge/master/Nuke_Microsoft_Edge.ps1" -Destination ./
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
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 4 -Force
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

if ($reduce_mitigations)
{
    Disable-WindowsOptionalFeature -NoRestart -Online -Remove -FeatureName VirtualMachinePlatform

    bcdedit.exe /set "{default}" loadoptions DISABLE-LSA-ISO,DISABLE-VBS
    bcdedit.exe /set "{default}" vsmlaunchtype off

    $REGS =@("ConfigureKernelShadowStacksLaunch", "ConfigureSystemGuardLaunch",
    "HypervisorEnforcedCodeIntegrity", "LsaCfgFlags", "RequirePlatformSecurityFeatures")
    $REGS.ForEach({
        PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeviceGuard' -ValueName "**del.$_" -Data '' -Type 'String'
    })

    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeviceGuard' -ValueName 'EnableVirtualizationBasedSecurity' -Data 0 -Type DWord
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeviceGuard' -ValueName 'HVCIMATRequired' -Data 0 -Type 'Dword'

    PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Lsa' -ValueName 'LsaCfgFlags' -Data 0 -Type 'Dword'

    # Disabling DEP works for some VAC-secured games, but denies playing CS:GO or TF2 for 30 minutes or less straight; causes VAC errors.
    # -> Why: VAC requires both the client and server DLLs to be signed with a certificate to detect cheaters:
    # -> https://github.com/ValveSoftware/source-sdk-2013/issues/76#issuecomment-21562961
    Set-ProcessMitigation -System -Enable DEP

    # Ensure "Data Execution Prevention" (DEP) only applies to operating system components and all kernel-mode drivers.
    # OptIn is Microsoft's default value for Windows 10 LTSC 2021.
    # Applying DEP to user-mode programs will slow or break them down, such as the Deus Ex (2000) video game.
    bcdedit.exe /set "{default}" nx OptIn

    # Data Execution Prevention (DEP):
    #       -> EmulateAtlThunks
    # Address Space Layout Randomization (ASLR):
    #       -> RequireInfo, ForceRelocateImages, BottomUp, HighEntropy
    # Control Flow Guard (CFG):
    #       -> SuppressExports, StrictCFG
    # Validate exception chains (SEHOP):
    #       -> SEHOP, SEHOPTelemetry, AuditSEHOP
    # Heap integrity:
    #       -> TerminateOnError
    Set-ProcessMitigation -PolicyFilePath .\mitigation-policies.xml
}

if (!$defender_smartscreen)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableSmartScreen' -Data '0' -Type 'Dword'

    PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -ValueName 'SmartScreenEnabled' -Data 'Off' -Type 'String'

    PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -ValueName 'EnableWebContentEvaluation' -Data '0' -Type 'Dword'

    Disable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
}

if ($flat_mouse_sensitivity)
{
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -Type String -Value 0 -Force
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -Type String -Value 0 -Force
    Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -Type String -Value 0 -Force
}

if (!$windows_security_systray)
{
    PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -ValueName 'HideSystray' -Data '1' -Type 'Dword'
}

if ($change_event_viewer_behavior)
{
    # Don't log events without warnings or errors.
    auditpol.exe /set /category:* /Success:disable
}
