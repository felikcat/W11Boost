#Requires -Version 5 -RunAsAdministrator

#region Initialize
Push-Location $PSScriptRoot
Start-Transcript -Path ([Environment]::GetFolderPath('MyDocuments') + "\W11Boost Logs\Extras - Apply DOD Stigs.log")

Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
. "..\Third-party\PolicyFileEditor\Commands.ps1"
#endregion

#region Preparation
Enable-WindowsOptionalFeature -NoRestart -Online -FeatureName VirtualMachinePlatform
bcdedit.exe /set "{default}" vsmlaunchtype auto

$STIG_HASH = (Get-FileHash -Algorithm SHA256 $env:TEMP\U_STIG_GPO_Package_July_2023.zip).Hash
$EXPECTED_HASH = "EFB40A99A97FE34A376A4A195D933F198846D5358CE904B350508DA0A6E94F00"

if ($STIG_HASH -ne $EXPECTED_HASH)
{
    # Source: https://public.cyber.mil/stigs/gpo/
    Download_File 'https://dl.dod.cyber.mil/wp-content/uploads/stigs/zip/U_STIG_GPO_Package_July_2023.zip' -Destination $env:TEMP
    if ($STIG_HASH -ne $EXPECTED_HASH)
    {
        Clear-Host
        Write-Warning "Did not match the expected SHA256 file hash; stopping now to prevent a potential security breach."
        Exit 1
    }
    Expand-Archive "$env:TEMP\U_STIG_GPO_Package_July_2023.zip" -DestinationPath "$env:TEMP\U_STIG_GPO_Package_July_2023"
}
#endregion Preparation

$STIGS = @("DoD Adobe Acrobat Pro DC Continuous V2R1",
"DoD Adobe Acrobat Reader DC Continuous V2R1",
"DoD Google Chrome v2r8",
"DoD Internet Explorer 11 v2r4",
"DoD Microsoft Defender Antivirus STIG v2r4",
"DoD Microsoft Edge v1r7",
"DoD Mozilla Firefox v6r5",
"DoD Office 2019-M365 Apps v2r8",
"DoD Office System 2013 and Components",
"DoD Office System 2016 and Components",
"DoD Windows 11 v1r3",
"DoD Windows Firewall v1r7")
$STIGS.ForEach({
    # Starting this many processes is horrendously slow.
    Start-Process -Wait "..\Third-party\LGPO.exe" -ArgumentList '/g "$env:TEMP\U_STIG_GPO_Package_July_2023\$_\GPOs"'
})
