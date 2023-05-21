if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Run TuneUp11 as Administrator!"
    Break
}

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

if ($WINDOWS_EDITION -notlike '*Enterprise*')
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
    Break
}

# Required for: Windows Updates, Windows Store (StorSvc), winget (DoSvc/delivery optimization).
$services = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")
for ($i = 0; $i -lt $services.length; $i++) {
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$services[$i]" /v "Start" /t REG_DWORD /d 3 /f
    sc.exe start $services[$i]
}

# Stops various annoyances, one being Windows Update restarting your PC without your consent.
. ".\Regions\Annoyances.ps1"

# Minimizing data sent to Microsoft through normal means, also improves performance.
. ".\Regions\Privacy.ps1"

# Correcting mistakes from other software, mainly ones intended to optimize Windows.
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


##+=+= Recommended Ethernet tweaks
# Can reduce time taken to establish a connection, and prevent drop-outs.
# Drop-outs were the case with Intel I225-V revision 1 and 2, but not 3.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

# TCP is to be reliable under bad network conditions, so avoid moving closer to UDP's behavior.
netsh.exe int tcp set global timestamps=enabled
##+=+=


# If logged into a Microsoft account: Don't sync anything.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\SettingSync' -ValueName 'SyncPolicy' -Data '5' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'ShowSyncProviderNotifications' -Data '0' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoLowDiskSpaceChecks' -Data '1' -Type 'Dword'

# Disable tracking of application startups.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Start_TrackProgs' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Policies\Microsoft\Windows\EdgeUI' -ValueName 'DisableMFUTracking' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\EdgeUI' -ValueName 'DisableMFUTracking' -Data '1' -Type 'Dword'

# Disable the acrylic blur at sign-in screen to improve performance at that screen.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'DisableAcrylicBackgroundOnLogon' -Data '1' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Autochk\Proxy"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector"

Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Disable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

# Don't analyze programs' execution time data.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib' -ValueName 'Disable Performance Counters' -Data '1' -Type 'Dword'


##+=+= NTFS tweaks
# Enabling long paths (260 character limit) prevents issues in Scoop and other programs.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\FileSystem' -ValueName 'LongPathsEnabled' -Data '1' -Type 'Dword'

# Ensure "Virtual Memory Pagefile Encryption" is disabled; by default it's not configured (off).
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsEncryptPagingFile' -Data '0' -Type 'Dword'

# Reducing page-faults and stack usage is beneficial to lowering DPC latency.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Policies' -ValueName 'NtfsForceNonPagedPoolAllocation' -Data '1' -Type 'Dword'

# Don't use NTFS' "Last Access Time Stamp Updates" by default; a program can still explicitly update them for itself.
fsutil.exe behavior set disablelastaccess 3
##+=+=


# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Thankfully, this does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no

# Do not keep track of recently opened files.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoRecentDocsHistory' -Data '1' -Type 'Dword'


##+=+= Enable UAC (User Account Control).
# UAC requires the 'LUA File Virtualization Filter Driver'.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\luafv" /v "Start" /t REG_DWORD /d 2 /f

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'PromptOnSecureDesktop' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'ConsentPromptBehaviorAdmin' -Data '5' -Type 'Dword'
# UAC being disabled can break programs, such as Eddie-UI.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'EnableLUA' -Data '1' -Type 'Dword'
##+=+=


##+=+= Disable typing insights.

# [ctfmon.exe] obsessive writes to "HKCU\Software\Microsoft\Input\TypingInsights\Insights" if enabled.
# Provides prediction for software (touch) keyboards.
# Settings -> Time & language -> Typing -> Typing insights
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Input\Settings' -ValueName 'InsightsEnabled' -Data '0' -Type 'Dword'

# Prediction for hardware keyboards.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Input\Settings' -ValueName 'EnableHwkbTextPrediction' -Data '0' -Type 'Dword'
##+=+=


##+=+= Shutdown options
# Disables "Fast startup"; stability issues, and increased disk wear from excessive I/O usage.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -ValueName 'HiberbootEnabled' -Data '0' -Type 'Dword'
attrib.exe +R "$env:windir\System32\SleepStudy\UserNotPresentSession.etl"

# Use default shutdown behavior.
reg.exe delete "HKEY_CURRENT_USER\Control Panel\Desktop\AutoEndTasks" /f

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'DisableShutdownNamedPipe' -Data '1' -Type 'Dword'

# A security feature that's disabled by default in Windows 11 Pro. Enabling this makes shutdown times slow.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -ValueName 'ClearPageFileAtShutdown' -Data '0' -Type 'Dword'
##+=+=


# Hidden file extensions are abused to hide the real file format, example:
# An executable (.exe, .scr) pretending to be a PDF.
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'HideFileExt' -Data '0' -Type 'Dword'


##+=+= Speed up Visual Studio by disabling telemetry.

Disable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
# https://learn.microsoft.com/en-us/visualstudio/ide/visual-studio-experience-improvement-program?view=vs-2022
# PerfWatson2 (VSCEIP) is intensive on resources, disable it.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'Software\Microsoft\VSCommon\17.0\SQM' -ValueName 'OptIn' -Data '0' -Type 'Dword'

# Remove feedback button and its features.
# Feedback can still be given through the Visual Studio Installer:
# https://learn.microsoft.com/en-us/visualstudio/ide/how-to-report-a-problem-with-visual-studio?view=vs-2022
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableFeedbackDialog' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableEmailInput' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\VisualStudio\Feedback' -ValueName 'DisableScreenshotCapture' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\VisualStudio\Telemetry' -ValueName 'TurnOffSwitch' -Data '1' -Type 'Dword'
##+=+=


# System Properties -> Advanced -> Performance
reg.exe import ".\Registry\Performance Options.reg"

# Disable Delivery Optimization's "Allow downloads from other PCs".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization' -ValueName 'DODownloadMode' -Data '0' -Type 'Dword'

# Windows 11 only changes.
if ($WIN32_BUILDNUMBER -ge 21327)
{
    # Less RAM usage, no advertised apps, and restores the classic context menu.
    $Better_Shell = Start-Job {
        winget.exe install StartIsBack.StartAllBack -eh --accept-package-agreements --accept-source-agreements --source winget --force
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

Write-Host "
== TuneUp11 has completed. ==
-> Restart to fully apply its changes!
"
