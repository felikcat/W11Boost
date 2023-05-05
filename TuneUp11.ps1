if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]"Administrator"))
{
    Write-Warning "ERROR: Run TuneUp11 as Administrator!"
    Break
}

$host.ui.rawui.windowtitle = "TuneUp11 by github.com/felikcat"

# If these are disabled, Windows Update will break and so will this script.
# Disabled StorSvc breaks the Windows Store.
# Disabled DoSvc (delivery optimization) breaks winget.
$services = @("AppXSvc", "ClipSVC", "TokenBroker", "StorSvc", "DoSvc")

for ($i = 0; $i -lt $services.length; $i++) {
    reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$services[$i]" /v "Start" /t REG_DWORD /d 3 /f
    sc.exe start $services[$i]
}


##+=+= Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\TuneUp11_LastRun.log")

# 'Import-Module example.psm1' fails if PowerShell script execution is disabled; do it manually.
Unblock-File -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path ".\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. ".\Third-party\PolicyFileEditor\Commands.ps1"

. ".\imports.ps1"
##+=+=

# Stops various annoyances, one being Windows Update restarting your PC without your consent.
. ".\Regions\Annoyances.ps1"

# Minimizing data sent to Microsoft through normal means, also improves performance.
. ".\Regions\Privacy.ps1"

# Correcting mistakes from other software, mainly ones intended to optimize Windows.
. ".\Regions\Repairs.ps1"

# Improves how consistent the performance is of networking, FPS, etc.
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


Set-Recommended-Ethernet-Tweaks


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

# Depend on the user clearing out thumbnail caches manually if they get corrupted.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VolumeCaches\Thumbnail Cache' -ValueName 'Autorun' -Data '0' -Type 'Dword'

# Old versions of the Intel PROSet software set this to 0, breaking Windows' internet connectivity check.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet' -ValueName 'EnableActiveProbing' -Data '1' -Type 'Dword'


##+=+= Disables various compatibility assistants and engines.

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

Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\ProgramDataUpdater"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\Application Experience\PcaPatchDbTask"
##+=+=


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


# Use the BBRv2 TCP congestion control algorithm; the differences:
# https://web.archive.org/web/20220313173158/http://web.archive.org/screenshot/https://docs.google.com/spreadsheets/d/1I1NcVVbuC7aq4nGalYxMNz9pgS9OLKcFHssIBlj9xXI
netsh.exe int tcp set supplemental Template=Internet CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=Datacenter CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=Compat CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=DatacenterCustom CongestionProvider=bbr2
netsh.exe int tcp set supplemental Template=InternetCustom CongestionProvider=bbr2

# Ask to enter recovery options after 3 failed boots instead of forcing it.
# NOTE: Thankfully, this does not disable the Windows Recovery Environment.
bcdedit.exe /set recoveryenabled no


##+=+= Stop Microsoft Edge from wasting CPU cycles and bandwidth.
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
##+=+=


# Do not keep track of recently opened files.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer' -ValueName 'NoRecentDocsHistory' -Data '1' -Type 'Dword'

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Biometrics' -ValueName 'Enabled' -Data '0' -Type 'Dword'


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


# Program Compatibility Assistant
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\PcaSvc" /v "Start" /t REG_DWORD /d 4 /f


##+=+= Shutdown options
# Disables "Fast startup"; stability issues, and increased disk wear from excessive I/O usage.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Power' -ValueName 'HiberbootEnabled' -Data '0' -Type 'Dword'
attrib.exe +R "$env:windir\System32\SleepStudy\UserNotPresentSession.etl"

# Use default shutdown behavior.
reg.exe delete "HKEY_CURRENT_USER\Control Panel\Desktop\AutoEndTasks" /f

# Use shutdown time-out defaults for programs.
reg.exe delete "HKEY_CURRENT_USER\Control Panel\Desktop\WaitToKillAppTimeOut" /f
reg.exe delete "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\HungAppTimeout" /f
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control' -ValueName 'WaitToKillServiceTimeout' -Data '5000' -Type 'String'

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

reg.exe import ".\Registry\Disable Delivery Optimization.reg"

# Restore the classic Windows 10 context menu; more performant and easier to use.
reg.exe import ".\Registry\Old Context Menus.reg"

# If this directory was non-existent before running TuneUp11, then add the "Hidden" attribute to line up with default behavior.
attrib.exe +H "$env:windir\System32\GroupPolicy"
gpupdate.exe /force

Add-Type -AssemblyName PresentationCore,PresentationFramework
[System.Windows.MessageBox]::Show(
        (new-object System.Windows.Window -Property @{TopMost = $True}),
        'Reboot Windows to fully apply changes.

Pressing OK will only close this dialog.',
        'TuneUp11 has finished.')
