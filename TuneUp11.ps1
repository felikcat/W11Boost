#Requires -Version 5 -RunAsAdministrator

##+=+= Initialize
$host.ui.rawui.windowtitle = "TuneUp11 by github.com/felikcat"
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_LastRun.log")

# 'Import-Module example.psm1' fails if PowerShell script execution is disabled; do it manually.
Unblock-File -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. ".\Third-party\PolicyFileEditor\Commands.ps1"

. ".\imports.ps1"
##+=+=

if ($_WINDOWS_EDITION -notlike '*Enterprise*')
{
    Clear-Host
    Write-Warning "
Only Enterprise editions of Windows are supported.

Upgrade to an Enterprise edition for free (without reinstalling Windows) using: https://github.com/massgravel/Microsoft-Activation-Scripts
"
    Write-Host "
Tutorial:
1. Use 'Method 2 - Traditional' to run MAS.

2. After MAS is open, press 6 on the keyboard for 'Extras', then 1 for 'Change Windows Edition'.

3. Select an Enterprise edition and wait.
-> If you're on Home: Professional -> Enterprise.
-> If you're on Pro, you can already upgrade to Enterprise.

4. Attempt to use TuneUp11 again after the switch to Enterprise is completed.

"
    Pause
    exit 1
}

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc).
$_regs = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
$_regs.ForEach({
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 3 -Force
    Start-Service $_
})

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
net.exe start w32time
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

# Disable the acrylic blur at sign-in screen to improve performance at that screen.
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
# Enabling long paths (260 character limit) prevents issues in Scoop and other programs.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\FileSystem' -ValueName 'LongPathsEnabled' -Data '1' -Type 'Dword'

# Ensure "Virtual Memory Pagefile Encryption" is disabled; by default it's not configured (off).
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsEncryptPagingFile' -Data '0' -Type 'Dword'

# Reducing page-faults and stack usage is beneficial to lowering DPC latency.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsForceNonPagedPoolAllocation' -Data '1' -Type 'Dword'

# Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3
##+=+=


# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Thankfully, this does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no

# Do not keep track of recently opened files.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoRecentDocsHistory' -Data '1' -Type 'Dword'


##+=+= Enable UAC (User Account Control).
# UAC requires the 'LUA File Virtualization Filter Driver'.
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\luafv" -Name "Start" -Type DWord -Value 2 -Force

PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'PromptOnSecureDesktop' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'ConsentPromptBehaviorAdmin' -Data '5' -Type 'Dword'
# UAC being disabled can break programs, such as Eddie-UI.
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
# Disables "Fast startup"; stability issues, and increased disk wear from excessive I/O usage.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -ValueName 'HiberbootEnabled' -Data '0' -Type 'Dword'
attrib.exe +R "$env:windir\System32\SleepStudy\UserNotPresentSession.etl"

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
# PerfWatson2 (VSCEIP) is intensive on resources, disable it.
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
if ($_WIN32_BUILDNUMBER -ge 21327)
{
    # Less RAM usage, no advertised apps, and restores the classic context menu.
    $Better_Shell = Start-Job {

    .\Third-party\MinSudo.exe --NoLogo powershell.exe -Command "winget.exe install StartIsBack.StartAllBack -eh --accept-package-agreements --accept-source-agreements --source winget --force"
    }
    # Use the BBRv2 TCP congestion control algorithm; the differences:
    # -> https://web.archive.org/web/20220313173158/http://web.archive.org/screenshot/https://docs.google.com/spreadsheets/d/1I1NcVVbuC7aq4nGalYxMNz9pgS9OLKcFHssIBlj9xXI
    # "Template=" applies globally (to all network interface templates).
    netsh.exe int tcp set supplemental CongestionProvider=bbr2 Template=
}

# If this directory was non-existent before running TuneUp11, then add the "Hidden" attribute to line up with default behavior.
attrib.exe +H "$env:windir\System32\GroupPolicy"
gpupdate.exe /force

# A race condition while grabbing the output of this job is possible, but it doesn't matter.
Receive-Job $Better_Shell

Stop-Transcript
Clear-Host
Write-Host "
== TuneUp11 has completed. ==
-> Restart to fully apply its changes!
"
Pause
