#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

#region Initialize
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc).
$REGS = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
$REGS.ForEach({
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$_', 'Start', '3', [RegistryValueKind]::DWord)
    Start-Service $_
})

# Installs Winget if not present. Mainly specific to LTSC 2019 and LTSC 2021.
if (-Not (Get-Command -CommandType Application -Name winget.exe)) {
    # Installs Winget's dependencies on LTSC 2019 and newer; does not work for LTSC 2016.
    wsreset.exe -i | Wait-Process

    Download_File 'https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle' -Destination ./

    Add-AppxPackage -Path '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'

    Remove-Item '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'
}
#endregion


#region Use optimal online NTP servers for more accurate system time.
Stop-Service w32time

# Make a clean slate for the time sync settings.
w32tm.exe /unregister
w32tm.exe /register

w32tm.exe /config /syncfromflags:manual /manualpeerlist:"time.cloudflare.com time.nist.gov time.windows.com"
Start-Service w32time

w32tm.exe /resync
#endregion


# Stops various annoyances, one being Windows Update restarting your PC without your consent.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Annoyances.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Annoyances.log'"

# Minimize data sent to Microsoft through normal means, also improves performance.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Privacy.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Privacy.log'"

# Correcting mistakes from other optimizers and user-error.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Repairs.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Repairs.log'"

# Improves how consistent the performance is for networking, FPS, etc.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Stability.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Stability.log'"


# Lower input delay and a little lower GPU usage (potentially higher FPS, depending on the game).
# Borderless windowed and fullscreen would otherwise be too similar. 
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment', '__COMPAT_LAYER', '~ DISABLEDXMAXIMIZEDWINDOWEDMODE', [RegistryValueKind]::String)

# Prevent network throttling to make online games have less percieved stuttering.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile', 'SystemResponsiveness', '0', [RegistryValueKind]::DWord)

# Increase process priority of games.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games', 'Priority', '6', [RegistryValueKind]::DWord)

# Allow global adjustment of timer resolution precision to enforce 0.5ms, so poorly written apps can't fuck up the precision for other apps.
# -> In detail: https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\kernel', 'GlobalTimerResolutionRequests', '1', [RegistryValueKind]::DWord)

#region Install SetTimerResolution
function STR_Requirement {
    $localArgs = "--NoLogo powershell.exe -Command winget.exe install Microsoft.VCRedist.2015+.x64 -s winget -eh --accept-package-agreements --accept-source-agreements --force"
    Start-Process -Wait ".\..\Third-party\NanaRun\MinSudo.exe" -ArgumentList $localArgs
}
STR_Requirement

Unblock-File -Path "..\Third-party\STR\SetTimerResolution.exe"
New-Item "$env:LOCALAPPDATA\Programs\STR" -ItemType Directory
Copy-Item "..\Third-party\STR\SetTimerResolution.exe" -Destination "$env:LOCALAPPDATA\Programs\STR" -Recurse

function STR_Service {
    $action = New-ScheduledTaskAction -Execute "SetTimerResolution.exe" -WorkingDirectory "$env:LOCALAPPDATA\Programs\STR" -Argument "--resolution 5000 --no-console"
    $trigger = New-ScheduledTaskTrigger -AtLogon
    $principal = New-ScheduledTaskPrincipal -GroupId "BUILTIN\Administrators" -RunLevel Highest
    $settings = New-ScheduledTaskSettingsSet
    $task = New-ScheduledTask -Action $action -Principal $principal -Trigger $trigger -Settings $settings
    Register-ScheduledTask STR -InputObject $task
    Start-ScheduledTask STR
}
STR_Service
#endregion

# Do not page drivers and other system code to a disk, keep it in memory.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management', 'DisablePagingExecutive', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'ShowSyncProviderNotifications', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer', 'NoLowDiskSpaceChecks', '1', [RegistryValueKind]::DWord)

# Disable tracking of application startups.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'Start_TrackProgs', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Policies\Microsoft\Windows\EdgeUI', 'DisableMFUTracking', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\EdgeUI', 'DisableMFUTracking', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'DisableAcrylicBackgroundOnLogon', '1', [RegistryValueKind]::DWord)

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

# Do not analyze apps' execution time data.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib', 'Disable Performance Counters', '1', [RegistryValueKind]::DWord)


#region NTFS tweaks
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem', 'LongPathsEnabled', '1', [RegistryValueKind]::DWord)

# Ensure "Virtual Memory Pagefile Encryption" is at its default of 'off'.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Policies', 'NtfsEncryptPagingFile', '0', [RegistryValueKind]::DWord)

# Allocate more RAM to NTFS' paged pool.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Policies', 'NtfsForceNonPagedPoolAllocation', '0', [RegistryValueKind]::DWord)
fsutil.exe behavior set memoryusage 2

# Do not use "Last Access Time Stamp Updates" by default; apps can still explicitly update these timestamps for themself.
fsutil.exe behavior set disablelastaccess 3
#endregion

# Allocate less resources to low-priority tasks, 10% total.
# https://learn.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile', 'SystemResponsiveness', '10', [RegistryValueKind]::DWord)

# Thankfully this does not disable the Windows Recovery Environment.
bcdedit.exe /set "{default}" recoveryenabled no

# Do not keep track of recently opened files.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer', 'NoRecentDocsHistory', '1', [RegistryValueKind]::DWord)


#region Shutdown options
# Disables "Fast startup".
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power', 'HiberbootEnabled', '0', [RegistryValueKind]::DWord)
(Get-Item "$env:windir\System32\SleepStudy\UserNotPresentSession.etl").Attributes = 'Archive', 'ReadOnly'

# Use default shutdown behavior.
Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "AutoEndTasks"

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System', 'DisableShutdownNamedPipe', '1', [RegistryValueKind]::DWord)

# A security feature that's disabled by default in Windows 11. Enabling this makes shutdown times slow.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management', 'ClearPageFileAtShutdown', '0', [RegistryValueKind]::DWord)
#endregion


# Hidden file extensions are abused to hide the real file format, example:
# An executable (.exe, .scr) pretending to be a PDF.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'HideFileExt', '0', [RegistryValueKind]::DWord)


#region Speed up Visual Studio by disabling its telemetry.
Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
# https://learn.microsoft.com/en-us/visualstudio/ide/visual-studio-experience-improvement-program?view=vs-2022
# PerfWatson2 (VSCEIP) is intensive on resources, ask to disable it.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\Software\Microsoft\VSCommon\17.0\SQM', 'OptIn', '0', [RegistryValueKind]::DWord)

# Remove feedback button and its features.
# Feedback can still be given through the Visual Studio Installer:
# https://learn.microsoft.com/en-us/visualstudio/ide/how-to-report-a-problem-with-visual-studio?view=vs-2022
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback', 'DisableFeedbackDialog', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback', 'DisableEmailInput', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback', 'DisableScreenshotCapture', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\VisualStudio\Telemetry', 'TurnOffSwitch', '1', [RegistryValueKind]::DWord)
#endregion

$APPS = @("Microsoft.BingNews", "Microsoft.WindowsFeedbackHub")
$APPS.ForEach({
    Get-AppxPackage -all $_ | Remove-AppxPackage -AllUsers
})

# Restore the classic context menu on Windows 11.
if ($WIN_BUILD -ge 21664) {
    [Registry]::SetValue('HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32', '', '', [RegistryValueKind]::String)
}

$NAME = @("InternetCustom", "DatacenterCustom", "Compat", "Datacenter", "Internet")
$NAME.ForEach({
        # BBRv2 is currently the best well-rounded TCP congestion algorithm.
        # Improvements from BBRv2 can be noticed if you're hosting game or web servers on this PC.
        # https://ieeexplore.ieee.org/abstract/document/9361674
        netsh.exe int tcp set supplemental Template=$_ CongestionProvider=bbr2
    })


#region Microsoft Edge tweaks
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'StartupBoostEnabled', '0', [RegistryValueKind]::DWord)
# Disallow Microsoft News.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'NewTabPageContentEnabled', '0', [RegistryValueKind]::DWord)

# Disable sponsored links on homepage.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'NewTabPageHideDefaultTopSites', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'DefaultBrowserSettingEnabled', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'DefaultBrowserSettingsCampaignEnabled', '0', [RegistryValueKind]::DWord)

# Block recommendations and promotional notifications from Microsoft Edge
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'ShowRecommendationsEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'SpotlightExperiencesAndRecommendationsEnabled', '0', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'PromotionalTabsEnabled', '0', [RegistryValueKind]::DWord)

# Disable various forms of telemetry.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'DiagnosticData', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge', 'PersonalizationReportingEnabled', '0', [RegistryValueKind]::DWord)
#endregion
