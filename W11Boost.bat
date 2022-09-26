@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: W11Boost: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

cd %~dp0

REM Using above -U:E "Current User (Elevated)" as of 18 Sept 2022 breaks winget.exe
REM Should be fixed when this issue is resolved: https://github.com/microsoft/winget-cli/issues/215
.\NSudoLC.exe -U:T -P:E -M:S -Priority:High -ShowWindowMode:Hide -Wait powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\main.ps1""' -Verb RunAs}"
