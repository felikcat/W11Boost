@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: W11_LTSC_Replica.bat: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\W11_LTSC_Replica.ps1""' -Verb RunAs}"
