use anyhow::Result;
use winsafe::HKEY;

use w11boost::{run_system_command, set_dword};

/* Ignored for security reasons:
    - PowerShell module logging
    - Event Viewer
*/

// (hkey: "HKLM"|"HKCU", subkey, value_name, value)
const REGISTRY_DWORDS: &[(&str, &str, &str, u32)] = &[
	// Do not analyze apps' execution time data.
	("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib", "Disable Performance Counters", 1),
	// Do not keep track of recently opened files.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsHistory", 1),
	// Disable Superfetch.
	("HKLM", r"SYSTEM\CurrentControlSet\Services\SysMain", "Start", 4),
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Management", "EnableSuperfetch", 0),
	// Disable Prefetch.
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 0),
	// Disable "Application Impact Telemetry".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", 0),
	// Disable "Program Compatibility Assistant".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisablePCA", 1),
	// Disable "Application Compatibility Engine".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableEngine", 1),
	// Disable "SwitchBack Compatibility Engine".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "SbEnable", 0),
	// Disable "User Steps Recorder".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR", 1),
	// Disable "Inventory Collector".
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInventory", 1),
	// API Sampling monitors the sampled collection of APIs used during system runtime.
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableAPISampling", 1),
	// Application Footprint monitors registry and file usage to help diagnose compatibility problems.
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableApplicationFootprint", 1),
	// Install Tracing tracks application installs.
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInstallTracing", 1),
	// Compatibility scan for backed up applications.
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableWin32AppBackup", 1),
	// Disable "Program Compatibility Assistant" service.
	("HKLM", r"SYSTEM\CurrentControlSet\Services", "PcaSvc", 4),
	// Disable PerfTrack.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}", "ScenarioExecutionEnabled", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Messenger\Client", "CEIP", 2),
	// Disable tracking of application startups.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 0),
	("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
	// Fully disable the activity feed.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", 0),
	// Do not search disks to attempt fixing a missing shortcut.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "LinkResolveIgnoreLinkInfo", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveSearch", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveTrack", 1),
	// Disable Device Search History.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDeviceSearchHistoryEnabled", 0),
	// By default Windows does not automatically back-up the registry, but just in case..
	("HKCU", r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager", "EnablePeriodicBackup", 0),
	// Remove frequent programs list from the Start Menu.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoStartMenuMFUprogramsList", 1),
	// Disable File History.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\FileHistory", "Disabled", 1),
	// Disable UserAssist - tracks program execution statistics (run count, last run time).
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings", "NoLog", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings", "NoLog", 1),
	// Disable thumbnail cache.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbnailCache", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbsDBOnNetworkFolders", 1),
	// Disable Background Activity Moderator (BAM).
	("HKLM", r"SYSTEM\CurrentControlSet\Services\bam", "Start", 4),
	// Disable Desktop Activity Moderator (DAM).
	("HKLM", r"SYSTEM\CurrentControlSet\Services\dam", "Start", 4),
	// Disable storing typed paths in Explorer address bar.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DontUsePowerShellOnWinX", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoSaveSettings", 0),
	// Disable storing Open/Save dialog history (ComDlg32 MRU).
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoFileMru", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoBackButton", 0),
	// Disable Application Experience service.
	("HKLM", r"SYSTEM\CurrentControlSet\Services\AeLookupSvc", "Start", 4),
	// Disable dmwappushservice - WAP push message routing for diagnostics.
	("HKLM", r"SYSTEM\CurrentControlSet\Services\dmwappushservice", "Start", 4),
	// Disable diagnostics hub.
	("HKLM", r"SYSTEM\CurrentControlSet\Services\diagsvc", "Start", 4),
	// Disable MRU lists in common dialogs via policy.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoRecentDocsMenu", 1),
	// Disable Clipboard History.
	("HKLM", r"Software\Policies\Microsoft\Windows\System", "AllowClipboardHistory", 0),
	// Disable search history.
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchHistory", 1),
	// Do not track remote files in Jump Lists.
	("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "NoRemoteDestinations", 1),
	// Clear history of recently opened documents on exit.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentDocsOnExit", 1),
	// Clear recent programs list for new users.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentProgForNewUserInStartMenu", 1),
	// Disable Windows Error Reporting logging to system event log.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", 1),
	("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", 1),
	// Do not add network shares to Network Locations when opening documents.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsNetHood", 1),
	// Prevent handwriting error reports.
	("HKLM", r"Software\Policies\Microsoft\Windows\HandwritingErrorReports", "PreventHandwritingErrorReports", 1),
	// Limit diagnostic log collection.
	("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDiagnosticLogCollection", 1),
	// Limit dump collection to kernel mini dumps only.
	("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDumpCollection", 1),
];

fn get_hkey(hkey_str: &str) -> HKEY
{
	if hkey_str == "HKLM" { HKEY::LOCAL_MACHINE } else { HKEY::CURRENT_USER }
}

pub fn run() -> Result<()>
{
	for (hkey_str, subkey, value_name, value) in REGISTRY_DWORDS {
		set_dword(&get_hkey(hkey_str), subkey, value_name, *value)?;
	}

	// Do not use "Last Access Time Stamp Updates" by default; apps can still
	// explicitly update these timestamps for themselves.
	run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "3"])?;

	Ok(())
}
