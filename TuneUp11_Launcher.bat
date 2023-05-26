@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: TuneUp11_Launcher.bat: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

REM 1. Using above -U:E "Current User (Elevated)" as of 18 Sept 2022 breaks winget.exe
REM -> Should be fixed when this issue is resolved: https://github.com/microsoft/winget-cli/issues/215
REM 2. A bypass for Windows' default of disabling PowerShell script execution.
.\Third-party\MinSudo.exe --NoLogo --Privileged powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\TuneUp11.ps1""' -Verb RunAs}"
