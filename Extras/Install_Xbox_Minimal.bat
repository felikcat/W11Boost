@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: Install_Xbox_Minimal.bat: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\Install_Xbox_Minimal.ps1""' -Verb RunAs}"
