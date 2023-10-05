#Requires -Version 5 -RunAsAdministrator

#region Initialize
Push-Location $PSScriptRoot

# 'Import-Module example.psm1' fails if PowerShell script execution is disabled; do it manually.
Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"

& ".\GUI.ps1"

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc).
$REGS = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
$REGS.ForEach({
        Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 3
        Start-Service $_
    })

$WIN_EDITION = Get-ItemPropertyValue 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion' -Name ProductName
if ($WIN_EDITION -notmatch '.*Enterprise|.*Education|.*Server') {
    # Education == Enterprise; in terms of what W11Boost expects.
    # Education allows Home edition to directly upgrade, instead of having to do Home -> Pro -> Enterprise.
    $localArgs = '"$env:SystemRoot\system32\slmgr.vbs" /ipk NW6C2-QMPVW-D7KKK-3GKT6-VCFB2'
    Start-Process -Wait cscript.exe -ArgumentList $localArgs
}

$License_Check = (Get-CimInstance -Query 'SELECT LicenseStatus FROM SoftwareLicensingProduct WHERE Name LIKE "%Windows%" AND PartialProductKey IS NOT NULL AND LicenseStatus !=1').LicenseStatus
if ($License_Check) {
    # Windows needs to be activated, do it!
    Start-Process -Wait ".\..\Third-party\MAS\Geranium8566.bat" -ArgumentList /KMS38
}

# Installs Winget if not present. Mainly specific to LTSC 2019 and LTSC 2021.
if (-Not (Get-Command -CommandType Application -Name winget -ErrorAction SilentlyContinue)) {
    # Installs Winget's dependencies on LTSC 2019 and newer; does not work for LTSC 2016.
    wsreset.exe -i | Wait-Process

    Download_File 'https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle' -Destination ./

    Add-AppxPackage -Path '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'

    Remove-Item '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'
}
#endregion


. ".\IMPORTS.ps1"


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
& ".\Annoyances.ps1"

# Minimize data sent to Microsoft through normal means, also improves performance.
& ".\Privacy.ps1"

# Correcting mistakes from other optimizers and user-error.
& ".\Repairs.ps1"

# Improves how consistent the performance is for networking, FPS, etc.
& ".\Stability.ps1"


PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'ShowSyncProviderNotifications' -Value '0' -Type 'Dword'

PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Name 'NoLowDiskSpaceChecks' -Value '1' -Type 'Dword'

# Disable tracking of application startups.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'Start_TrackProgs' -Value '0' -Type 'Dword'
PEAdd_HKCU 'Software\Policies\Microsoft\Windows\EdgeUI' -Name 'DisableMFUTracking' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\EdgeUI' -Name 'DisableMFUTracking' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'DisableAcrylicBackgroundOnLogon' -Value '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

# Do not analyze apps' execution time data.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib' -Name 'Disable Performance Counters' -Value '1' -Type 'Dword'


##+=+= NTFS tweaks
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\FileSystem' -Name 'LongPathsEnabled' -Value '1' -Type 'Dword'

# Ensure "Virtual Memory Pagefile Encryption" is at its default of 'off'.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Policies' -Name 'NtfsEncryptPagingFile' -Value '0' -Type 'Dword'

# Allocate more RAM to NTFS' paged pool.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Policies' -Name 'NtfsForceNonPagedPoolAllocation' -Value '0' -Type 'Dword'
fsutil.exe behavior set memoryusage 2

# Do not use "Last Access Time Stamp Updates" by default; apps can still explicitly update these timestamps for themself.
fsutil.exe behavior set disablelastaccess 3
##+=+=


# Thankfully this does not disable the Windows Recovery Environment.
bcdedit.exe /set "{default}" recoveryenabled no

# Do not keep track of recently opened files.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -Name 'NoRecentDocsHistory' -Value '1' -Type 'Dword'


##+=+= Enable UAC (User Account Control).
# UAC requires the 'LUA File Virtualization Filter Driver'.
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\luafv" -Name "Start" -Type DWord -Value 2

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'PromptOnSecureDesktop' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'ConsentPromptBehaviorAdmin' -Value '5' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'EnableLUA' -Value '1' -Type 'Dword'
##+=+=


#region Shutdown options
# Disables "Fast startup".
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -Name 'HiberbootEnabled' -Value '0' -Type 'Dword'
(Get-Item "$env:windir\System32\SleepStudy\UserNotPresentSession.etl").Attributes = 'Archive', 'ReadOnly'

# Use default shutdown behavior.
Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "AutoEndTasks"

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'DisableShutdownNamedPipe' -Value '1' -Type 'Dword'

# A security feature that's disabled by default in Windows 11. Enabling this makes shutdown times slow.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -Name 'ClearPageFileAtShutdown' -Value '0' -Type 'Dword'
#endregion


# Hidden file extensions are abused to hide the real file format, example:
# An executable (.exe, .scr) pretending to be a PDF.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'HideFileExt' -Value '0' -Type 'Dword'


#region Speed up Visual Studio by disabling its telemetry.
if (Get-CimInstance MSFT_VSInstance) {
    Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
    # https://learn.microsoft.com/en-us/visualstudio/ide/visual-studio-experience-improvement-program?view=vs-2022
    # PerfWatson2 (VSCEIP) is intensive on resources, ask to disable it.
    PEAdd_HKLM 'Software\Microsoft\VSCommon\17.0\SQM' -Name 'OptIn' -Value '0' -Type 'Dword'

    # Remove feedback button and its features.
    # Feedback can still be given through the Visual Studio Installer:
    # https://learn.microsoft.com/en-us/visualstudio/ide/how-to-report-a-problem-with-visual-studio?view=vs-2022
    PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Name 'DisableFeedbackDialog' -Value '1' -Type 'Dword'
    PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Name 'DisableEmailInput' -Value '1' -Type 'Dword'
    PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -Name 'DisableScreenshotCapture' -Value '1' -Type 'Dword'

    PEAdd_HKCU 'Software\Microsoft\VisualStudio\Telemetry' -Name 'TurnOffSwitch' -Value '1' -Type 'Dword'
}
#endregion

$APPS = @("Microsoft.BingNews_8wekyb3d8bbwe", "Microsoft.WindowsFeedbackHub_8wekyb3d8bbwe")
$APPS.ForEach({
        $localArgs = "--NoLogo powershell.exe -Command winget.exe uninstall $_ --exact --silent --accept-source-agreements"
        Start-Process -Wait ".\..\Third-party\NanaRun\MinSudo.exe" -ArgumentList $localArgs
    })

# Restore the classic context menu.
New-Item -Path "HKCU:\SOFTWARE\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32" -Value "" -Type String -Force

$NAME = @("InternetCustom", "DatacenterCustom", "Compat", "Datacenter", "Internet")
$NAME.ForEach({
        # BBRv2 is currently the best well-rounded TCP congestion algorithm.
        # Improvements from BBRv2 can be noticed if you're hosting game or web servers on this PC.
        # https://ieeexplore.ieee.org/abstract/document/9361674
        netsh.exe int tcp set supplemental Template=$_ CongestionProvider=bbr2
    })


#region Microsoft Edge tweaks
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'StartupBoostEnabled' -Value '0' -Type 'Dword'
# Disallow Microsoft News.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'NewTabPageContentEnabled' -Value '0' -Type 'Dword'

# Disable sponsored links on homepage.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'NewTabPageHideDefaultTopSites' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'DefaultBrowserSettingEnabled' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'DefaultBrowserSettingsCampaignEnabled' -Value '0' -Type 'Dword'

# Block recommendations and promotional notifications from Microsoft Edge
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'ShowRecommendationsEnabled' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'SpotlightExperiencesAndRecommendationsEnabled' -Value '0' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'PromotionalTabsEnabled' -Value '0' -Type 'Dword'

# Disable various forms of telemetry.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'DiagnosticData' -Value '0' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Edge' -Name 'PersonalizationReportingEnabled' -Value '0' -Type 'Dword'
#endregion



# If this directory was non-existent before running W11Boost, then add the "Hidden" attribute to line up with default behavior.
(Get-Item "$env:windir\System32\GroupPolicy").Attributes = "Directory", "Hidden"
gpupdate.exe /force

Restart-Computer
