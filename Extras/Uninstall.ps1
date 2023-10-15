#Requires -Version 5 -RunAsAdministrator

# The following use registry keys: 
# - ScheduledTasks
# - Services
reg.exe import $env:USERPROFILE\W11Boost_Backups\HKEY_CLASSES_ROOT.reg
reg.exe import $env:USERPROFILE\W11Boost_Backups\HKEY_CURRENT_USER.reg
reg.exe import $env:USERPROFILE\W11Boost_Backups\HKEY_LOCAL_MACHINE.reg

if ($env:PROCESSOR_IDENTIFIER -match 'GenuineIntel') {
    [Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", $null, "Machine")
}
