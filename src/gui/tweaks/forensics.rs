// Forensics & Local Data tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};
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
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib",
                        value_name: "Disable Performance Counters",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib",
                        value_name: "Disable Performance Counters",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_recent_docs_history",
                category: "forensics",
                name: "Disable Recent Documents History",
                description: "Stops tracking recently opened files. Clears on exit and removes from Start Menu.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoRecentDocsHistory",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "NoRecentDocsMenu",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "ClearRecentDocsOnExit",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoRecentDocsNetHood",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoRecentDocsHistory",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "NoRecentDocsMenu",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "ClearRecentDocsOnExit",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoRecentDocsNetHood",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_superfetch",
                category: "forensics",
                name: "Disable Superfetch (SysMain)",
                description: "Disables Superfetch service which caches app usage patterns to disk.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\SysMain",
                                value_name: "Start",
                                value: RegistryValue::Dword(4),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Management",
                                value_name: "EnableSuperfetch",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\SysMain",
                                value_name: "Start",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Management",
                                value_name: "EnableSuperfetch",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_prefetch",
                category: "forensics",
                name: "Disable Prefetch",
                description: "Disables Prefetch which stores app launch data in C:\\Windows\\Prefetch.",
                effect: TweakEffect::Restart,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
                        value_name: "EnablePrefetcher",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Dword(3)
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
                        value_name: "EnablePrefetcher",
                        value: RegistryValue::Dword(3),
                        stock_value: RegistryValue::Dword(3)
        }]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_app_compat_tracking",
                category: "forensics",
                name: "Disable App Compatibility Tracking",
                description: "Disables Application Impact Telemetry, Program Compatibility Assistant, and related engines.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "AITEnable",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisablePCA",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableEngine",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "SbEnable",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableUAR",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInventory",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\PcaSvc",
                                value_name: "Start",
                                value: RegistryValue::Dword(4),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\AeLookupSvc",
                                value_name: "Start",
                                value: RegistryValue::Dword(4),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "AITEnable",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisablePCA",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableEngine",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "SbEnable",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableUAR",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInventory",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\PcaSvc",
                                value_name: "Start",
                                value: RegistryValue::Dword(3),
                                stock_value: RegistryValue::Dword(2)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\AeLookupSvc",
                                value_name: "Start",
                                value: RegistryValue::Dword(3),
                                stock_value: RegistryValue::Dword(3)
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_api_install_tracking",
                category: "forensics",
                name: "Disable API & Install Tracking",
                description: "Disables API sampling, application footprint monitoring, and install tracing.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableAPISampling",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableApplicationFootprint",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInstallTracing",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableWin32AppBackup",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableAPISampling",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableApplicationFootprint",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableInstallTracing",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                                value_name: "DisableWin32AppBackup",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_perftrack",
                category: "forensics",
                name: "Disable PerfTrack",
                description: "Disables performance tracking and Customer Experience Improvement Program.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}",
                                value_name: "ScenarioExecutionEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Messenger\Client",
                                value_name: "CEIP",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}",
                                value_name: "ScenarioExecutionEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Messenger\Client",
                                value_name: "CEIP",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_program_tracking",
                category: "forensics",
                name: "Disable Program Startup Tracking",
                description: "Disables tracking of application startups and most frequently used programs.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoStartMenuMFUprogramsList",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "ClearRecentProgForNewUserInStartMenu",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "Start_TrackProgs",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Windows\EdgeUI",
                                value_name: "DisableMFUTracking",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoStartMenuMFUprogramsList",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "ClearRecentProgForNewUserInStartMenu",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_activity_feed_forensics",
                category: "forensics",
                name: "Disable Activity Feed",
                description: "Fully disables the activity feed including publishing and uploading user activities.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "EnableActivityFeed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "PublishUserActivities",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "UploadUserActivities",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "EnableActivityFeed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "PublishUserActivities",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\System",
                                value_name: "UploadUserActivities",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_link_tracking",
                category: "forensics",
                name: "Disable Shortcut Link Tracking",
                description: "Prevents searching disks to fix missing shortcuts and disables link resolve tracking.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "LinkResolveIgnoreLinkInfo",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoResolveSearch",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoResolveTrack",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "LinkResolveIgnoreLinkInfo",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoResolveSearch",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
                                value_name: "NoResolveTrack",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_search_history",
                category: "forensics",
                name: "Disable Search History",
                description: "Disables device search history and Explorer search history.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                                value_name: "IsDeviceSearchHistoryEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchHistory",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
                                value_name: "IsDeviceSearchHistoryEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "DisableSearchHistory",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_file_history",
                category: "forensics",
                name: "Disable File History",
                description: "Disables Windows File History backup feature.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\FileHistory",
                        value_name: "Disabled",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Windows\FileHistory",
                        value_name: "Disabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_userassist",
                category: "forensics",
                name: "Disable UserAssist",
                description: "Disables UserAssist which tracks program execution count and last run time.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings",
                                value_name: "NoLog",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings",
                                value_name: "NoLog",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings",
                                value_name: "NoLog",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings",
                                value_name: "NoLog",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_bam_dam",
                category: "forensics",
                name: "Disable BAM/DAM Services",
                description: "Disables Background Activity Moderator and Desktop Activity Moderator which track app execution.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\bam",
                                value_name: "Start",
                                value: RegistryValue::Dword(4),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\dam",
                                value_name: "Start",
                                value: RegistryValue::Dword(4),
                                stock_value: RegistryValue::Dword(1)
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\bam",
                                value_name: "Start",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SYSTEM\CurrentControlSet\Services\dam",
                                value_name: "Start",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Dword(1)
        },
                ]),
                requires_restart: true
        },
        crate::tweak! {
                id: "disable_open_save_mru",
                category: "forensics",
                name: "Disable Open/Save Dialog History",
                description: "Disables storing history in common Open/Save dialogs (ComDlg32 MRU).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                                value_name: "NoFileMru",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                                value_name: "NoBackButton",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                                value_name: "NoFileMru",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32",
                                value_name: "NoBackButton",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_clipboard_history_forensics",
                category: "forensics",
                name: "Disable Clipboard History",
                description: "Disables Windows clipboard history which can store sensitive copied data.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"Software\Policies\Microsoft\Windows\System",
                        value_name: "AllowClipboardHistory",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"Software\Policies\Microsoft\Windows\System",
                        value_name: "AllowClipboardHistory",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_jump_list_remote",
                category: "forensics",
                name: "Disable Remote Files in Jump Lists",
                description: "Prevents tracking of remote/network files in taskbar Jump Lists.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                        value_name: "NoRemoteDestinations",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Policies\Microsoft\Windows\Explorer",
                        value_name: "NoRemoteDestinations",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_error_reporting_log",
                category: "forensics",
                name: "Disable Error Reporting Logging",
                description: "Disables Windows Error Reporting logging to system event log.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                                value_name: "LoggingDisabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                                value_name: "LoggingDisabled",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                                value_name: "LoggingDisabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                                value_name: "LoggingDisabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_handwriting_reports",
                category: "forensics",
                name: "Disable Handwriting Error Reports",
                description: "Prevents handwriting recognition error reports from being sent.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"Software\Policies\Microsoft\Windows\HandwritingErrorReports",
                        value_name: "PreventHandwritingErrorReports",
                        value: RegistryValue::Dword(1),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"Software\Policies\Microsoft\Windows\HandwritingErrorReports",
                        value_name: "PreventHandwritingErrorReports",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "limit_diagnostic_collection",
                category: "forensics",
                name: "Limit Diagnostic & Dump Collection",
                description: "Limits diagnostic log and crash dump collection to minimum levels.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDiagnosticLogCollection",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDumpCollection",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDiagnosticLogCollection",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"Software\Policies\Microsoft\Windows\DataCollection",
                                value_name: "LimitDumpCollection",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_explorer_settings_save",
                category: "forensics",
                name: "Disable Explorer Typed Paths",
                description: "Prevents Explorer from storing typed paths in address bar and uses PowerShell on Win+X.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DontUsePowerShellOnWinX",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "NoSaveSettings",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                                value_name: "DontUsePowerShellOnWinX",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                                value_name: "NoSaveSettings",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_last_access_time",
                category: "forensics",
                name: "Disable Last Access Time Updates",
                description: "Disables NTFS last access time updates to reduce file system forensic artifacts.",
                effect: TweakEffect::Restart,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(disable_last_access_apply),
                custom_revert: Some(enable_last_access_apply),
                requires_restart: true
        },
        crate::tweak! {
            id: "bsod_details",
            category: "forensics",
            name: "Show BSOD Details",
            description: "Shows detailed crash parameters on Blue Screen instead of just the sad face.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[RegistryOp {
                hkey: "HKLM",
                subkey: r"System\CurrentControlSet\Control\CrashControl",
                value_name: "DisplayParameters",
                value: RegistryValue::Dword(1),
                stock_value: RegistryValue::Delete
            }],
            disabled_ops: Some(&[RegistryOp {
                hkey: "HKLM",
                subkey: r"System\CurrentControlSet\Control\CrashControl",
                value_name: "DisplayParameters",
                value: RegistryValue::Delete,
                stock_value: RegistryValue::Delete
            }])
        },
];

fn disable_last_access_apply(ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status("fsutil behavior set disablelastaccess 3");
        crate::run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "3"])?;
        ctx.report_progress();
        Ok(())
}

fn enable_last_access_apply(ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status("fsutil behavior set disablelastaccess 2");
        crate::run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "2"])?;
        ctx.report_progress();
        Ok(())
}
