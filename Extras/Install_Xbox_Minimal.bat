@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: Install Xbox Minimal: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\IXM.ps1""' -Verb RunAs}"
