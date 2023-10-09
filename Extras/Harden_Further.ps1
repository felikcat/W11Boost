#Requires -Version 5 -RunAsAdministrator

# Disable LLMNR -> https://www.blackhillsinfosec.com/how-to-disable-llmnr-why-you-want-to/
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows NT\DNSClient' -Name 'EnableMulticast' -Value '0' -Type 'Dword'

# Break old apps relying on DOS "short names".
# https://ttcshelbyville.wordpress.com/2018/12/02/should-you-disable-8dot3-for-performance-and-security/
fsutil.exe behavior set disable8dot3 1
fsutil 8dot3name strip /s /v c:

bcdedit.exe /set "{default}" vsmlaunchtype auto

# Ensure "Data Execution Prevention" (DEP) only applies to operating system components and all kernel-mode drivers.
# OptIn is Microsoft's default value for Windows 10 LTSC 2021.
# Applying DEP to user-mode apps will slow or break them down, such as the Deus Ex (2000) video game.
bcdedit.exe /set "{default}" nx OptIn


#region Enable -> Windows Defender Smartscreen
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -Name 'EnableSmartScreen' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -Name 'SmartScreenEnabled' -Value 'On' -Type 'String'

PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -Name 'EnableWebContentEvaluation' -Value '1' -Type 'Dword'

Enable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
#endregion


# Enable -> Windows Security System tray
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -Name 'HideSystray' -Value '0' -Type 'Dword'


# Enable -> Blocking downloaded files.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -Name 'SaveZoneInformation' -Value '1' -Type 'Dword'
# Blocking downloaded files in Explorer can sometimes break File History for downloaded files.
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -Name 'SaveZoneInformation' -Value '1' -Type 'Dword'

# Automatically enable or disable Smart App Control for Windows 11 build 22621 and newer.
# Force enabling does not work, therefore Evaluation mode has to be used.
PEAdd_HKLM 'SYSTEM\CurrentControlSet\Control\CI\Policy' -Name 'VerifiedAndReputablePolicyState' -Value '2' -Type 'Dword'


# Disable additional risky services that the DoD STIGs left alone.
$REGS = @("ShellHWDetection", "Spooler", "LanmanServer", "SSDPSRV", "lfsvc")
$REGS.ForEach({
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 4
})

gpupdate.exe /force
