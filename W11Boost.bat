@echo off

reg.exe query HKU\S-1-5-19 || (
	echo ==== Error ====
	echo Right click on this file and select 'Run as administrator'
	echo Press any key to exit...
	Pause>nul
	exit /b
)

powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "& {Start-Process powershell.exe -ArgumentList '-NoProfile -ExecutionPolicy Bypass -File ""main.ps1""' -Verb RunAs}"
