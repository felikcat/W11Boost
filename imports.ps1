#Requires -Version 5 -RunAsAdministrator

$_WIN32_BUILDNUMBER = (Get-WmiObject Win32_OperatingSystem).BuildNumber
# You can bypass this check without any noticeable issues.
# Downside: Group Policy doesn't have full power on non-Enterprise editions, mainly specific to Windows' Telemetry.
$_WINDOWS_EDITION = Get-ItemPropertyValue 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion' -Name ProductName

function Download_File
{
    Start-BitsTransfer -MaxDownloadTime 120 -RetryInterval 60 -RetryTimeout 300 -TransferPolicy Unrestricted -Source @args
}

function PolEdit_HKCU
{
    Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\User\registry.pol" -Key @args
}

function PolEdit_HKLM
{
    Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\Machine\registry.pol" -Key @args
}