@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: Advanced.bat: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

.\..\Third-party\MinSudo.exe --NoLogo --Privileged powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\Advanced.ps1""' -Verb RunAs}"
