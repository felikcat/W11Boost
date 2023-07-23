#Requires -Version 5 -RunAsAdministrator

Remove-Item -Path "$env:windir\System32\GroupPolicy" -Recurse -Force
gpupdate.exe /force

if ($env:PROCESSOR_IDENTIFIER -match 'GenuineIntel') {
    [Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", $null, "Machine")
}

if ($WIN_BUILDNUMBER -ge 21327) # Windows 11 only
{
    Remove-Item -Path "HKCU:\SOFTWARE\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32" -Recurse -Force
}

Enable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\Consolidator"
Enable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\KernelCeipTask"
Enable-ScheduledTask -TaskName "\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip"

Enable-ScheduledTask -TaskName "\NvTmRep_CrashReport1_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Enable-ScheduledTask -TaskName "\NvTmRep_CrashReport2_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Enable-ScheduledTask -TaskName "\NvTmRep_CrashReport3_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"
Enable-ScheduledTask -TaskName "\NvTmRep_CrashReport4_{B2FE1952-0186-46C3-BAEC-A80AA35AC5B8}"

Enable-ScheduledTask -TaskName "\Microsoft\VisualStudio\Updates\BackgroundDownload"
Enable-ScheduledTask -TaskName "\Microsoft\Windows\Shell\IndexerAutomaticMaintenance"

Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\fhsvc" -Name "Start" -Type DWord -Value 3 -Force
Enable-ScheduledTask -TaskName "\Microsoft\Windows\FileHistory\File History (maintenance mode)"

Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\Notifications"
Enable-ScheduledTask -TaskName "\Microsoft\Windows\Location\WindowsActionDialog"
