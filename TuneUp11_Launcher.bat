@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: TuneUp11_Launcher.bat: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

REM A bypass for Windows' default of disabling PowerShell script execution.
powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\TuneUp11.ps1""' -Verb RunAs}"
