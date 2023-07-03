#Requires -Version 5 -RunAsAdministrator

##+=+= Initialize
$host.ui.rawui.windowtitle = "W11Boost by github.com/felikcat"
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\W11Boost_LastRun.log")

# 'Import-Module example.psm1' fails if PowerShell script execution is disabled; do it manually.
Unblock-File -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. ".\Third-party\PolicyFileEditor\Commands.ps1"

. ".\imports.ps1"

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc).
$REGS = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
$REGS.ForEach({
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 3 -Force
    Start-Service $_
})
##+=+=


if ($WIN_EDITION -notmatch '.*Enterprise|.*Education|.*Server')
{
    # Education == Enterprise; in terms of what W11Boost expects.
    # Education allows Home edition to directly upgrade, instead of having to do Home -> Pro -> Enterprise.
    cscript.exe "$env:SystemRoot\system32\slmgr.vbs" /ipk NW6C2-QMPVW-D7KKK-3GKT6-VCFB2 | Out-Null
}

$License_Check = Get-WMIObject -Query 'SELECT LicenseStatus FROM SoftwareLicensingProduct WHERE Name LIKE "%Windows%" AND PartialProductKey IS NOT NULL AND LicenseStatus !=1'
if ([bool]::TryParse($a, [ref]$License_Check))
{
    & ([ScriptBlock]::Create((Invoke-RestMethod https://massgrave.dev/get))) /KMS38
}


# Install Winget if it's not present. Mainly an LTSC 2019 and LTSC 2021 specific issue.
if (-Not (Get-Command -CommandType Application -Name winget -ErrorAction SilentlyContinue))
{
    # Installs Winget's dependencies on LTSC 2019 and newer; doesn't work for LTSC 2016.
    wsreset.exe -i | Out-Null
    
    Download_File 'https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle' -Destination ./

    Add-AppxPackage -Path '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'

    Remove-Item '.\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle'  
}


# Stops various annoyances, one being Windows Update restarting your PC without your consent.
. ".\Regions\Annoyances.ps1"

# Minimize data sent to Microsoft through normal means, also improves performance.
. ".\Regions\Privacy.ps1"

# Correcting mistakes from other optimizers and user-error.
. ".\Regions\Repairs.ps1"

# Improves how consistent the performance is for networking, FPS, etc.
. ".\Regions\Stability.ps1"


##+=+= Use optimal online NTP servers for more accurate system time.
net.exe stop w32time

# Make a clean slate for the time sync settings.
w32tm.exe /unregister
w32tm.exe /register

w32tm.exe /config /syncfromflags:manual /manualpeerlist:"time.cloudflare.com ntppool1.time.nl ntppool2.time.nl"
Start-Service w32time
w32tm.exe /resync
##+=+=


# If logged into a Microsoft account: Don't sync anything.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\SettingSync' -ValueName 'SyncPolicy' -Data '5' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSyncProviderNotifications' -Data '0' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoLowDiskSpaceChecks' -Data '1' -Type 'Dword'

# Disable tracking of application startups.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Start_TrackProgs' -Data '0' -Type 'Dword'
PolEdit_HKCU 'Software\Policies\Microsoft\Windows\EdgeUI' -ValueName 'DisableMFUTracking' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\EdgeUI' -ValueName 'DisableMFUTracking' -Data '1' -Type 'Dword'

PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'DisableAcrylicBackgroundOnLogon' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

# Don't analyze programs' execution time data.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib' -ValueName 'Disable Performance Counters' -Data '1' -Type 'Dword'


##+=+= NTFS tweaks
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\FileSystem' -ValueName 'LongPathsEnabled' -Data '1' -Type 'Dword'

# Ensure "Virtual Memory Pagefile Encryption" is at its default of 'off'.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsEncryptPagingFile' -Data '0' -Type 'Dword'

PolEdit_HKLM 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsForceNonPagedPoolAllocation' -Data '1' -Type 'Dword'

# Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3
##+=+=


# Thankfully this does not disable the Windows Recovery Environment.
bcdedit.exe /set "{default}" recoveryenabled no

# Do not keep track of recently opened files.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoRecentDocsHistory' -Data '1' -Type 'Dword'


##+=+= Enable UAC (User Account Control).
# UAC requires the 'LUA File Virtualization Filter Driver'.
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\luafv" -Name "Start" -Type DWord -Value 2 -Force

PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'PromptOnSecureDesktop' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'ConsentPromptBehaviorAdmin' -Data '5' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'EnableLUA' -Data '1' -Type 'Dword'
##+=+=


##+=+= Disable typing insights.

# [ctfmon.exe] obsessive writes to "HKCU\Software\Microsoft\Input\TypingInsights\Insights" if enabled.
# Provides prediction for software (touch) keyboards.
# Settings -> Time & language -> Typing -> Typing insights
PolEdit_HKCU 'Software\Microsoft\Input\Settings' -ValueName 'InsightsEnabled' -Data '0' -Type 'Dword'

# Prediction for hardware keyboards.
PolEdit_HKCU 'Software\Microsoft\Input\Settings' -ValueName 'EnableHwkbTextPrediction' -Data '0' -Type 'Dword'
##+=+=


##+=+= Shutdown options
# Disables "Fast startup".
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -ValueName 'HiberbootEnabled' -Data '0' -Type 'Dword'
(Get-Item "$env:windir\System32\SleepStudy\UserNotPresentSession.etl").Attributes = 'Archive', 'ReadOnly'



# Use default shutdown behavior.
Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name "AutoEndTasks" -Force

PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'DisableShutdownNamedPipe' -Data '1' -Type 'Dword'

# A security feature that's disabled by default in Windows 11 Pro. Enabling this makes shutdown times slow.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -ValueName 'ClearPageFileAtShutdown' -Data '0' -Type 'Dword'
##+=+=


# Hidden file extensions are abused to hide the real file format, example:
# An executable (.exe, .scr) pretending to be a PDF.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'HideFileExt' -Data '0' -Type 'Dword'


##+=+= Speed up Visual Studio by disabling telemetry.

Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
# https://learn.microsoft.com/en-us/visualstudio/ide/visual-studio-experience-improvement-program?view=vs-2022
# PerfWatson2 (VSCEIP) is intensive on resources, ask to disable it.
PolEdit_HKLM 'Software\Microsoft\VSCommon\17.0\SQM' -ValueName 'OptIn' -Data '0' -Type 'Dword'

# Remove feedback button and its features.
# Feedback can still be given through the Visual Studio Installer:
# https://learn.microsoft.com/en-us/visualstudio/ide/how-to-report-a-problem-with-visual-studio?view=vs-2022
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableFeedbackDialog' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableEmailInput' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableScreenshotCapture' -Data '1' -Type 'Dword'

PolEdit_HKCU 'Software\Microsoft\VisualStudio\Telemetry' -ValueName 'TurnOffSwitch' -Data '1' -Type 'Dword'
##+=+=


# System Properties -> Advanced -> Performance
reg.exe import ".\Registry\Performance Options.reg"

# Disable Delivery Optimization's "Allow downloads from other PCs".
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization' -ValueName 'DODownloadMode' -Data '0' -Type 'Dword'

# Windows 11 only changes.
if ($WIN_BUILDNUMBER -ge 21327)
{
    # Less RAM usage, no advertised apps, and restores the classic context menu.
    .\Third-party\MinSudo.exe --NoLogo powershell.exe -Command "winget.exe install StartIsBack.StartAllBack -eh --accept-package-agreements --accept-source-agreements --source winget --force" | Out-Null

    # Note: "Template=" applies to all network interface templates.
    netsh.exe int tcp set supplemental CongestionProvider=bbr2 Template=
}

# If this directory was non-existent before running W11Boost, then add the "Hidden" attribute to line up with default behavior.
(Get-Item "$env:windir\System32\GroupPolicy").Attributes = "Directory", "Hidden"
gpupdate.exe /force

Restart-Computer -Force
