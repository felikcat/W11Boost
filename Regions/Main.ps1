#Requires -Version 5 -RunAsAdministrator

#region Initialize
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc).
$REGS = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
$REGS.ForEach({
        SetReg -Path "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$_" -Key 'Start' -Value '3' -Type 'DWord'
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


# Stops various annoyances, one being Windows Update restarting your PC without your consent.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Annoyances.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Annoyances.log'"

# Minimize data sent to Microsoft through normal means, also improves performance.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Privacy.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Privacy.log'"

# Correcting mistakes from other optimizers and user-error.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Repairs.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Repairs.log'"

# Improves how consistent the performance is for networking, FPS, etc.
Start-Process -WindowStyle hidden -FilePath "powershell.exe" -Verb RunAs ".\Stability.ps1 | Out-File '${HOME}\Desktop\W11Boost logs\Stability.log'"

# Prevent network throttling to make online games have less percieved stuttering.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile' -Key 'SystemResponsiveness' -Value '0' -Type 'DWord'

# Increase process priority of games.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games' -Key 'Priority' -Value '6' -Type 'DWord'

# Allow global adjustment of timer resolution precision to enforce 0.5ms, so poorly written apps can't fuck up the precision for other apps.
# -> In detail: https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\kernel' -Key 'GlobalTimerResolutionRequests' -Value '1' -Type 'DWord'

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
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -Key 'DisablePagingExecutive' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'ShowSyncProviderNotifications' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Key 'NoLowDiskSpaceChecks' -Value '1' -Type 'DWord'

# Disable tracking of application startups.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'Start_TrackProgs' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_CURRENT_USER\Software\Policies\Microsoft\Windows\EdgeUI' -Key 'DisableMFUTracking' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\EdgeUI' -Key 'DisableMFUTracking' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System' -Key 'DisableAcrylicBackgroundOnLogon' -Value '1' -Type 'DWord'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

# Do not analyze apps' execution time data.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib' -Key 'Disable Performance Counters' -Value '1' -Type 'DWord'


#region NTFS tweaks
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\FileSystem' -Key 'LongPathsEnabled' -Value '1' -Type 'DWord'

# Ensure "Virtual Memory Pagefile Encryption" is at its default of 'off'.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Policies' -Key 'NtfsEncryptPagingFile' -Value '0' -Type 'DWord'

# Allocate more RAM to NTFS' paged pool.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Policies' -Key 'NtfsForceNonPagedPoolAllocation' -Value '0' -Type 'DWord'
fsutil.exe behavior set memoryusage 2

# Do not use "Last Access Time Stamp Updates" by default; apps can still explicitly update these timestamps for themself.
fsutil.exe behavior set disablelastaccess 3
#endregion

# Allocate less resources to low-priority tasks, 10% total.
# https://learn.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile' -Key 'SystemResponsiveness' -Value '10' -Type 'DWord'

# Thankfully this does not disable the Windows Recovery Environment.
bcdedit.exe /set "{default}" recoveryenabled no

# Do not keep track of recently opened files.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Key 'NoRecentDocsHistory' -Value '1' -Type 'DWord'


#region Shutdown options
# Disables "Fast startup".
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Power' -Key 'HiberbootEnabled' -Value '0' -Type 'DWord'
(Get-Item "$env:windir\System32\SleepStudy\UserNotPresentSession.etl").Attributes = 'Archive', 'ReadOnly'

# Use default shutdown behavior.
Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "AutoEndTasks"

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Key 'DisableShutdownNamedPipe' -Value '1' -Type 'DWord'

# A security feature that's disabled by default in Windows 11. Enabling this makes shutdown times slow.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -Key 'ClearPageFileAtShutdown' -Value '0' -Type 'DWord'
#endregion


# Hidden file extensions are abused to hide the real file format, example:
# An executable (.exe, .scr) pretending to be a PDF.
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'HideFileExt' -Value '0' -Type 'DWord'


#region Speed up Visual Studio by disabling its telemetry.
Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
# https://learn.microsoft.com/en-us/visualstudio/ide/visual-studio-experience-improvement-program?view=vs-2022
# PerfWatson2 (VSCEIP) is intensive on resources, ask to disable it.
SetReg -Path 'HKEY_LOCAL_MACHINE\Software\Microsoft\VSCommon\17.0\SQM' -Key 'OptIn' -Value '0' -Type 'DWord'

# Remove feedback button and its features.
# Feedback can still be given through the Visual Studio Installer:
# https://learn.microsoft.com/en-us/visualstudio/ide/how-to-report-a-problem-with-visual-studio?view=vs-2022
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Key 'DisableFeedbackDialog' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Key 'DisableEmailInput' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Key 'DisableScreenshotCapture' -Value '1' -Type 'DWord'

SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\VisualStudio\Telemetry' -Key 'TurnOffSwitch' -Value '1' -Type 'DWord'
#endregion

# Restore the classic context menu on Windows 11.
if ($WIN_BUILD -ge 21664) {
    SetReg -Path 'HKEY_CURRENT_USER\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32' -Key '' -Value '' -Type 'String'
}

$NAME = @("InternetCustom", "DatacenterCustom", "Compat", "Datacenter", "Internet")
$NAME.ForEach({
        # BBRv2 is currently the best well-rounded TCP congestion algorithm.
        # Improvements from BBRv2 can be noticed if you're hosting game or web servers on this PC.
        # https://ieeexplore.ieee.org/abstract/document/9361674
        netsh.exe int tcp set supplemental Template=$_ CongestionProvider=bbr2
    })


#region Microsoft Edge tweaks
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'StartupBoostEnabled' -Value '0' -Type 'DWord'
# Disallow Microsoft News.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'NewTabPageContentEnabled' -Value '0' -Type 'DWord'

# Disable sponsored links on homepage.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'NewTabPageHideDefaultTopSites' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'DefaultBrowserSettingEnabled' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'DefaultBrowserSettingsCampaignEnabled' -Value '0' -Type 'DWord'

# Block recommendations and promotional notifications from Microsoft Edge
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'ShowRecommendationsEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'SpotlightExperiencesAndRecommendationsEnabled' -Value '0' -Type 'DWord'

SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'PromotionalTabsEnabled' -Value '0' -Type 'DWord'

# Disable various forms of telemetry.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'DiagnosticData' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Edge' -Key 'PersonalizationReportingEnabled' -Value '0' -Type 'DWord'
#endregion
