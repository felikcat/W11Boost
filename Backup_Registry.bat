@echo off

fltmc.exe >nul
if not %errorLevel% == 0 (
	echo ERROR: Backup Registry: Right click on this file and select 'Run as administrator'
	Pause
	exit 1
)

set Command=reg.exe export

for /f "tokens=1,2*" %%A in (
    'reg.exe query "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders" /v Personal'
) do if /i "%%~A" == "Personal" set "Documents=%%~C"
REM exitcode 2 = File cannot be found.
if not defined Documents exit /b 2

cd /d "%Documents%"

%Command% "HKEY_LOCAL_MACHINE" HKEY_LOCAL_MACHINE.reg
%Command% "HKEY_CURRENT_USER" HKEY_CURRENT_USER.reg
%Command% "HKEY_CLASSES_ROOT" HKEY_CLASSES_ROOT.reg
