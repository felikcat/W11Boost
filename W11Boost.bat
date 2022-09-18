@echo off

reg.exe query HKU\S-1-5-19 || (
	echo ==== Error ====
	echo Right click on this file and select 'Run as administrator'
	echo Press any key to exit...
	Pause>nul
	exit /b
)

cd %~dp0

REM Using above -U:E "Current User (Elevated)" as of 18 Sept 2022 breaks winget.exe
REM Should be fixed when this issue is resolved: https://github.com/microsoft/winget-cli/issues/215
NSudoLC.exe -U:E -P:E -M:S -Priority:High -Wait powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File "".\main.ps1""' -Verb RunAs}"
