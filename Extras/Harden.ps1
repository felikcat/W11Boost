#Requires -Version 5 -RunAsAdministrator

#region Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ("$env:TEMP\W11Boost - Extras - Harden.log")

Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"
#endregion


#region GPO prep-work -> Prevents their failure
Enable-WindowsOptionalFeature -NoRestart -Online -FeatureName VirtualMachinePlatform

$DBGFLAGS = @("loadoptions", "testsigning", "flightsigning", "nointegritychecks", "hypervisordebug")
$DBGFLAGS.ForEach({
    bcdedit.exe /deletevalue "{default}" $_
})
bcdedit.exe /set "{default}" vsmlaunchtype auto

# Source: https://public.cyber.mil/stigs/gpo/
Remove-Item -LiteralPath "$env:TEMP\U_STIG_GPO_Package_July_2023" -Recurse
Expand-Archive "..\GPO Templates\U_STIG_GPO_Package_July_2023.zip" -DestinationPath ("$env:TEMP\U_STIG_GPO_Package_July_2023")
#endregion GPO prep-work -> Prevents their failure


$GPOS = @("DoD Adobe Acrobat Pro DC Continuous V2R1",
"DoD Adobe Acrobat Reader DC Continuous V2R1",
"DoD Google Chrome v2r8",
"DoD Internet Explorer 11 v2r4",
"DoD Microsoft Defender Antivirus STIG v2r4",
"DoD Microsoft Edge v1r7",
"DoD Mozilla Firefox v6r5",
"DoD Office 2019-M365 Apps v2r8",
"DoD Office System 2013 and Components",
"DoD Office System 2016 and Components",
"")
Start-Process -Wait "..\Third-party\LGPO.exe" -ArgumentList '/g "..\GPO Templates\$_\GPOs"'




# Ensure "Data Execution Prevention" (DEP) only applies to operating system components and all kernel-mode drivers.
# OptIn is Microsoft's default value for Windows 10 LTSC 2021.
# Applying DEP to user-mode programs will slow or break them down, such as the Deus Ex (2000) video game.
bcdedit.exe /set "{default}" nx OptIn

# Data Execution Prevention (DEP):
#       -> EmulateAtlThunks
# Address Space Layout Randomization (ASLR):
#       -> RequireInfo, ForceRelocateImages, BottomUp, HighEntropy
# Control Flow Guard (CFG):
#       -> SuppressExports, StrictCFG
# Validate exception chains (SEHOP):
#       -> SEHOP, SEHOPTelemetry, AuditSEHOP
# Heap integrity:
#       -> TerminateOnError
Set-ProcessMitigation -PolicyFilePath .\mitigation-policies.xml
##+=+=


##+=+= Enable -> Windows Defender Smartscreen
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableSmartScreen' -Data '1' -Type 'Dword'

PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -ValueName 'SmartScreenEnabled' -Data 'On' -Type 'String'

PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\AppHost' -ValueName 'EnableWebContentEvaluation' -Data '1' -Type 'Dword'

Enable-ScheduledTask -TaskName "\Microsoft\Windows\AppID\SmartScreenSpecific"
##+=+=


##+=+= Enable -> Windows Security System tray
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows Defender Security Center\Systray' -ValueName 'HideSystray' -Data '0' -Type 'Dword'
##+=+=


##+=+= Enable -> Blocking downloaded files.
# SaveZoneInformation 0 = enables blocking downloaded files.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '0' -Type 'Dword'
# Don't block downloaded files in Explorer, also fixes File History not working for downloaded files.
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Policies\Attachments' -ValueName 'SaveZoneInformation' -Data '0' -Type 'Dword'
##+=+=

# Disable additional risky services that the DoD STIGs left alone.
$REGS = @("ShellHWDetection", "Spooler", "LanmanServer", "SSDPSRV", "lfsvc")
$REGS.ForEach({
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 4
})

#region Enforce DNS-over-HTTPS using Quad9's service.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows NT\DNSClient' -ValueName 'DoHPolicy' -Data '3' -Type 'Dword'

$NET_ALIAS = (Get-NetAdapter -Physical)
$NET_DEVID = $NET_ALIAS.InterfaceGuid

$IP = @("9.9.9.9", "149.112.112.112", "2620:fe::fe", "2620:fe::9")
$IP.ForEach({
    # We're not removing the same DoH entries if it already exists; hence "-ErrorAction SilentlyContinue".
    Add-DnsClientDohServerAddress -ServerAddress $_ -DohTemplate 'https://dns.quad9.net/dns-query' -AllowFallbackToUdp 0 -AutoUpgrade 1 -ErrorAction SilentlyContinue
})

$NET_ALIAS.ForEach({
    $NET_DEVID.ForEach({
        # -Force: If that registry existed, wipe it out.
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh\9.9.9.9" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh\149.112.112.112" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD

        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh6\2620:fe::fe" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh6\2620:fe::9" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
    })
    Set-DnsClientServerAddress -InterfaceIndex ($_).InterfaceIndex -ServerAddresses ([String[]]$IP)
})
Clear-DnsClientCache
#endregion


gpupdate.exe /force
