use crate::common::{run_system_command, set_dword};
use winsafe::HKEY;
use anyhow::Result;

/* Ignored for security reasons:
    - PowerShell module logging
    - Event Viewer
*/

pub fn run() -> Result<()>
{
        let hklm = HKEY::LOCAL_MACHINE;
        let hkcu = HKEY::CURRENT_USER;

        // Do not analyze apps' execution time data.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib",
                "Disable Performance Counters",
                1,
        )?;

        // Do not keep track of recently opened files.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsHistory",
                1,
        )?;

        // Disable Superfetch.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\SysMain", "Start", 4)?;
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Management",
                "EnableSuperfetch",
                0,
        )?;

        // Disable Prefetch.
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
                "EnablePrefetcher",
                0,
        )?;

        // Do not use "Last Access Time Stamp Updates" by default; apps can still
        // explicitly update these timestamps for themself.
        run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "3"])?;

        // Disable "Application Impact Telemetry".
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", 0)?;

        // Disable "Program Compatibility Assistant".
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisablePCA", 1)?;

        // Disable "Application Compatibility Engine".
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableEngine",
                1,
        )?;

        // Disable "SwitchBack Compatibility Engine".
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "SbEnable", 0)?;

        // Disable "User Steps Recorder".
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR", 1)?;

        // Disable "Inventory Collector".
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableInventory",
                1,
        )?;

        // API Sampling monitors the sampled collection of application programming interfaces used during system runtime to help diagnose compatibility problems.
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableAPISampling",
                1,
        )?;

        // Application Footprint monitors the sampled collection of registry and file usage to help diagnose compatibility problems.
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableApplicationFootprint",
                1,
        )?;

        // Install Tracing is a mechanism that tracks application installs to help diagnose compatibility problems.
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableInstallTracing",
                1,
        )?;

        //The compatibility scan for backed up applications evaluates for compatibility problems in installed applications.
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableWin32AppBackup",
                1,
        )?;

        // Disable "Program Compatibility Assistant" service.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services", "PcaSvc", 4)?;

        // Disable PerfTrack.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}",
                "ScenarioExecutionEnabled",
                0,
        )?;
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Messenger\Client", "CEIP", 2)?;

        // Disable tracking of application startups.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
                "DisableMFUTracking",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "Start_TrackProgs",
                0,
        )?;
        set_dword(
                &hkcu,
                r"Software\Policies\Microsoft\Windows\EdgeUI",
                "DisableMFUTracking",
                1,
        )?;

        // Fully disable the activity feed.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "EnableActivityFeed",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "PublishUserActivities",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "UploadUserActivities",
                0,
        )?;

        // Do not search disks to attempt fixing a missing shortcut.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "LinkResolveIgnoreLinkInfo",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoResolveSearch",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoResolveTrack",
                1,
        )?;

        // Disable Device Search History.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                "IsDeviceSearchHistoryEnabled",
                0,
        )?;

        // By default Windows does not automatically back-up the registry, but just in case they change this..
        set_dword(
                &hkcu,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager",
                "EnablePeriodicBackup",
                0,
        )?;

        // Remove frequent programs list from the Start Menu.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoStartMenuMFUprogramsList",
                1,
        )?;

        // Disable File History.
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\FileHistory", "Disabled", 1)?;

        // Disable UserAssist - tracks program execution statistics (run count, last run time).
        // Two GUIDs track different application categories.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings",
                "NoLog",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings",
                "NoLog",
                1,
        )?;

        // Disable thumbnail cache - prevents image previews from persisting after file deletion.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbnailCache",
                1,
        )?;
        // Also disable thumbnails on network folders.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DisableThumbsDBOnNetworkFolders",
                1,
        )?;

        // Disable Background Activity Moderator (BAM) - tracks background app execution with full paths.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\bam", "Start", 4)?;

        // Disable Desktop Activity Moderator (DAM) - similar to BAM for desktop apps.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\dam", "Start", 4)?;

        // Disable storing typed paths in Explorer address bar.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "DontUsePowerShellOnWinX",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                "NoSaveSettings",
                0,
        )?;

        // Disable storing Open/Save dialog history (ComDlg32 MRU).
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                "NoFileMru",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                "NoBackButton",
                0,
        )?;

        // Disable Application Experience service - collects app launch telemetry.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\AeLookupSvc", "Start", 4)?;

        // Disable dmwappushservice - WAP push message routing for diagnostics.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\dmwappushservice", "Start", 4)?;

        // Disable diagnostics hub - collects diagnostic data.
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\diagsvc", "Start", 4)?;

        // Disable MRU lists in common dialogs via policy.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                "NoRecentDocsMenu",
                1,
        )?;

        // Disable Clipboard History - prevents clipboard contents from being stored in memory.
        set_dword(
                &hklm,
                r"Software\Policies\Microsoft\Windows\System",
                "AllowClipboardHistory",
                0,
        )?;

        // Disable search history - prevents search queries from being stored in the registry.
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                "DisableSearchHistory",
                1,
        )?;

        // Do not track remote files in Jump Lists.
        set_dword(
                &hkcu,
                r"Software\Policies\Microsoft\Windows\Explorer",
                "NoRemoteDestinations",
                1,
        )?;

        // Clear history of recently opened documents on exit.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "ClearRecentDocsOnExit",
                1,
        )?;

        // Clear recent programs list for new users.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "ClearRecentProgForNewUserInStartMenu",
                1,
        )?;

        // Disable Windows Error Reporting logging to system event log.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                "LoggingDisabled",
                1,
        )?;
        set_dword(
                &hkcu,
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                "LoggingDisabled",
                1,
        )?;

        // Do not add network shares to Network Locations when opening documents.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                "NoRecentDocsNetHood",
                1,
        )?;

        // Prevent handwriting error reports.
        set_dword(
                &hklm,
                r"Software\Policies\Microsoft\Windows\HandwritingErrorReports",
                "PreventHandwritingErrorReports",
                1,
        )?;

        // Limit diagnostic log collection.
        set_dword(
                &hklm,
                r"Software\Policies\Microsoft\Windows\DataCollection",
                "LimitDiagnosticLogCollection",
                1,
        )?;

        // Limit dump collection to kernel mini dumps only.
        set_dword(
                &hklm,
                r"Software\Policies\Microsoft\Windows\DataCollection",
                "LimitDumpCollection",
                1,
        )?;

        Ok(())
}
