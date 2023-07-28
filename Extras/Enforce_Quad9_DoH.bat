@echo off

cd %~dp0

.\..\Third-party\MinSudo.exe --NoLogo powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\Enforce_Quad9_DoH.ps1""' -Verb RunAs}"
