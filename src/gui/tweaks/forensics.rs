// Forensics & Local Data tweaks

use super::{Tweak, TweakEffect};
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::sync::Arc;

pub static FORENSICS_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_performance_counters",
                category: "forensics",
                name: "Disable Performance Counters",
                description: "Stops analyzing apps' execution time data. Prevents performance monitoring artifacts.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib", "Disable Performance Counters", 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_recent_docs_history",
        category: "forensics",
        name: "Disable Recent Documents History",
        description: "Stops tracking recently opened files. Clears on exit and removes from Start Menu.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsHistory", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoRecentDocsMenu", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentDocsOnExit", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsNetHood", 1),
        ],
        },
        crate::tweak! {
                id: "disable_superfetch",
                category: "forensics",
                name: "Disable Superfetch (SysMain)",
                description: "Disables Superfetch service which caches app usage patterns to disk.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\SysMain", "Start", 4, 2),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Management", "EnableSuperfetch", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_prefetch",
                category: "forensics",
                name: "Disable Prefetch",
                description: "Disables Prefetch which stores app launch data in C:\\Windows\\Prefetch.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 0, 3),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_app_compat_tracking",
                category: "forensics",
                name: "Disable App Compatibility Tracking",
                description: "Disables Application Impact Telemetry, Program Compatibility Assistant, and related engines.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisablePCA", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableEngine", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "SbEnable", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR", 1),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInventory", 1),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\PcaSvc", "Start", 4, 2),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\AeLookupSvc", "Start", 4, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_api_install_tracking",
        category: "forensics",
        name: "Disable API & Install Tracking",
        description: "Disables API sampling, application footprint monitoring, and install tracing.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableAPISampling", 1),
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableApplicationFootprint", 1),
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInstallTracing", 1),
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableWin32AppBackup", 1),
        ],
        },
        crate::tweak! {
        id: "disable_perftrack",
        category: "forensics",
        name: "Disable PerfTrack",
        description: "Disables performance tracking and Customer Experience Improvement Program.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}", "ScenarioExecutionEnabled", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Messenger\Client", "CEIP", 2),
        ],
        },
        crate::tweak! {
        id: "disable_program_tracking",
        category: "forensics",
        name: "Disable Program Startup Tracking",
        description: "Disables tracking of application startups and most frequently used programs.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs", 0),
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoStartMenuMFUprogramsList", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentProgForNewUserInStartMenu", 1),
        ],
        },
        crate::tweak! {
                id: "disable_activity_feed_forensics",
                category: "forensics",
                name: "Disable Activity Feed",
                description: "Fully disables the activity feed including publishing and uploading user activities.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities", 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_link_tracking",
        category: "forensics",
        name: "Disable Shortcut Link Tracking",
        description: "Prevents searching disks to fix missing shortcuts and disables link resolve tracking.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "LinkResolveIgnoreLinkInfo", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveSearch", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveTrack", 1),
        ],
        },
        crate::tweak! {
        id: "disable_search_history",
        category: "forensics",
        name: "Disable Search History",
        description: "Disables device search history and Explorer search history.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDeviceSearchHistoryEnabled", 0),
                crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchHistory", 1),
        ],
        },
        crate::tweak! {
        id: "disable_file_history",
        category: "forensics",
        name: "Disable File History",
        description: "Disables Windows File History backup feature.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\FileHistory", "Disabled", 1),
        ],
        },
        crate::tweak! {
        id: "disable_userassist",
        category: "forensics",
        name: "Disable UserAssist",
        description: "Disables UserAssist which tracks program execution count and last run time.",
        effect: TweakEffect::Logoff,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings", "NoLog", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings", "NoLog", 1),
        ],
        },
        crate::tweak! {
                id: "disable_bam_dam",
                category: "forensics",
                name: "Disable BAM/DAM Services",
                description: "Disables Background Activity Moderator and Desktop Activity Moderator which track app execution.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\bam", "Start", 4, 1),
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\dam", "Start", 4, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
        id: "disable_open_save_mru",
        category: "forensics",
        name: "Disable Open/Save Dialog History",
        description: "Disables storing history in common Open/Save dialogs (ComDlg32 MRU).",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoFileMru", 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoBackButton", 0),
        ],
        },
        crate::tweak! {
        id: "disable_clipboard_history_forensics",
        category: "forensics",
        name: "Disable Clipboard History",
        description: "Disables Windows clipboard history which can store sensitive copied data.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\System", "AllowClipboardHistory", 0),
        ],
        },
        crate::tweak! {
        id: "disable_jump_list_remote",
        category: "forensics",
        name: "Disable Remote Files in Jump Lists",
        description: "Prevents tracking of remote/network files in taskbar Jump Lists.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "NoRemoteDestinations", 1),
        ],
        },
        crate::tweak! {
        id: "disable_handwriting_reports",
        category: "forensics",
        name: "Disable Handwriting Error Reports",
        description: "Prevents handwriting recognition error reports from being sent.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\HandwritingErrorReports", "PreventHandwritingErrorReports", 1),
        ],
        },
        crate::tweak! {
        id: "limit_diagnostic_collection",
        category: "forensics",
        name: "Limit Diagnostic & Dump Collection",
        description: "Limits diagnostic log and crash dump collection to minimum levels.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDiagnosticLogCollection", 1),
                crate::reg_dword!("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDumpCollection", 1),
        ],
        },
        crate::tweak! {
        id: "disable_explorer_settings_save",
        category: "forensics",
        name: "Disable Explorer Typed Paths",
        description: "Prevents Explorer from storing typed paths in address bar and uses PowerShell on Win+X.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DontUsePowerShellOnWinX", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoSaveSettings", 0),
        ],
        },
        crate::tweak! {
                id: "disable_last_access_time",
                category: "forensics",
                name: "Disable Last Access Time Updates",
                description: "Disables NTFS last access time updates to reduce file system forensic artifacts.",
                effect: TweakEffect::Restart,
                enabled_ops: &[],
                                custom_apply: Some(disable_last_access_apply),
                                requires_restart: true
        },
        crate::tweak! {
        id: "bsod_details",
        category: "forensics",
        name: "Show BSOD Details",
        description: "Shows detailed crash parameters on Blue Screen instead of just the sad face.",
        effect: TweakEffect::Immediate,
        enabled_ops: &[
                crate::reg_dword!("HKLM", r"System\CurrentControlSet\Control\CrashControl", "DisplayParameters", 1),
        ],
        },
];

fn disable_last_access_apply(ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status("fsutil behavior set disablelastaccess 3");
        crate::run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "3"])?;
        ctx.report_progress();
        Ok(())
}
