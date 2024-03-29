#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

#region Preparation
Enable-WindowsOptionalFeature -NoRestart -Online -FeatureName VirtualMachinePlatform
bcdedit.exe /set "{default}" vsmlaunchtype auto

# Intentionally not checking for hashes here.
$STIG_NAME = "U_STIG_GPO_Package_October_2023"
Expand-Archive "..\Third-party\DoD-STIGS\$STIG_NAME.zip" -DestinationPath "$env:TEMP\$STIG_NAME" -Force
#endregion Preparation

$STIGS = @("DoD Adobe Acrobat Pro DC Continuous V2R1",
"DoD Adobe Acrobat Reader DC Continuous V2R1",
"DoD Google Chrome v2r8",
"DoD Internet Explorer 11 v2r4",
"DoD Microsoft Defender Antivirus STIG v2r4",
"DoD Microsoft Edge v1r7",
"DoD Mozilla Firefox v6r5",
"DoD Office 2019-M365 Apps v2r11",
"DoD Office System 2013 and Components",
"DoD Office System 2016 and Components",
"DoD Windows Firewall v1r7")

if ($WIN_BUILD -ge 21664) {
    $STIGS += ("DoD Windows 11 v1r5")
} elseif ($WIN_BUILD -le 19045) {
    $STIGS += ("DoD Windows 10 v2r8")
}
$STIGS.ForEach({
    Start-Process ".\..\Third-party\LGPO.exe" -WindowStyle hidden -ArgumentList `"/g`", `"$env:TEMP\$STIG_NAME\$_\GPOs`"
})

# Remove DoD notices.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System', 'legalnoticecaption', '', [RegistryValueKind]::String)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System', 'legalnoticetext', '', [RegistryValueKind]::String)
