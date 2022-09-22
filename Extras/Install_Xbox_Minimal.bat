@echo off

fsutil dirty query %SYSTEMDRIVE% >nul
if not %errorLevel% == 0 (
	echo ERROR: Install Xbox Minimal: Right click on this file and select 'Run as administrator'
	Pause
)

cd %~dp0

powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\IXM.ps1""' -Verb RunAs}"
