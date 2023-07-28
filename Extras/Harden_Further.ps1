#Requires -Version 5 -RunAsAdministrator

#region Initialize
Push-Location $PSScriptRoot

Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"
#endregion

bcdedit.exe /set "{default}" vsmlaunchtype auto

# Ensure "Data Execution Prevention" (DEP) only applies to operating system components and all kernel-mode drivers.
# OptIn is Microsoft's default value for Windows 10 LTSC 2021.
# Applying DEP to user-mode apps will slow or break them down, such as the Deus Ex (2000) video game.
bcdedit.exe /set "{default}" nx OptIn


##+=+= Enable -> Windows Defender Smartscreen
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'EnableSmartScreen' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -Name 'SmartScreenEnabled' -Value 'On' -Type 'String'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -Name 'EnableWebContentEvaluation' -Value '1' -Type 'Dword'

Enable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
##+=+=


##+=+= Enable -> Windows Security System tray
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -Name 'HideSystray' -Value '0' -Type 'Dword'
##+=+=


##+=+= Enable -> Blocking downloaded files.
# SaveZoneInformation 0 = enables blocking downloaded files.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -Name 'SaveZoneInformation' -Value '0' -Type 'Dword'
# Do not block downloaded files in Explorer, also fixes File History not working for downloaded files.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -Name 'SaveZoneInformation' -Value '0' -Type 'Dword'
##+=+=

# Disable additional risky services that the DoD STIGs left alone.
$REGS = @("ShellHWDetection", "Spooler", "LanmanServer", "SSDPSRV", "lfsvc")
$REGS.ForEach({
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 4
})

gpupdate.exe /force
