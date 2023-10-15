#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

# Disable LLMNR -> https://www.blackhillsinfosec.com/how-to-disable-llmnr-why-you-want-to/
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows NT\DNSClient', 'EnableMulticast', '0', [RegistryValueKind]::DWord)

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
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'EnableSmartScreen', '1', [RegistryValueKind]::DWord)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer', 'SmartScreenEnabled', 'On', [RegistryValueKind]::String)

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost', 'EnableWebContentEvaluation', '1', [RegistryValueKind]::DWord)

Enable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
#endregion


# Enable -> Windows Security System tray
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray', 'HideSystray', '0', [RegistryValueKind]::DWord)


# Enable -> Blocking downloaded files.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments', 'SaveZoneInformation', '1', [RegistryValueKind]::DWord)
# Blocking downloaded files in Explorer can sometimes break File History for downloaded files.
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Policies\Attachments', 'SaveZoneInformation', '1', [RegistryValueKind]::DWord)

# Automatically enable or disable Smart App Control for Windows 11 build 22621 and newer.
# Force enabling does not work, therefore Evaluation mode has to be used.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\CI\Policy', 'VerifiedAndReputablePolicyState', '2', [RegistryValueKind]::DWord)


# Disable additional risky services that the DoD STIGs left alone.
$REGS = @("ShellHWDetection", "Spooler", "LanmanServer", "SSDPSRV", "lfsvc")
$REGS.ForEach({
    [Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\$_', 'Start', '4', [RegistryValueKind]::DWord)
})

gpupdate.exe /force
