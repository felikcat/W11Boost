#Requires -Version 5 -RunAsAdministrator

Remove-Item -Path "$env:windir\System32\GroupPolicy" -Recurse -Force
gpupdate.exe /force