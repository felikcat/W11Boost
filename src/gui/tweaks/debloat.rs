// Windows App Debloating

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::sync::Arc;

pub static DEBLOAT_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "debloat_teams_legacy",
                category: "debloat",
                name: "Remove Teams (legacy)",
                description: "Uninstalls the legacy Microsoft Teams app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("MicrosoftTeams", ctx))
        },
        crate::tweak! {
                id: "debloat_teams",
                category: "debloat",
                name: "Remove Teams",
                description: "Uninstalls the new Microsoft Teams app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("MSTeams", ctx))
        },
        crate::tweak! {
                id: "debloat_copilot",
                category: "debloat",
                name: "Remove Copilot App",
                description: "Uninstalls the Microsoft Copilot app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Copilot", ctx))
        },
        crate::tweak! {
                id: "debloat_cortana",
                category: "debloat",
                name: "Remove Cortana",
                description: "Uninstalls the Cortana app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.549981C3F5F10", ctx))
        },
        crate::tweak! {
                id: "debloat_clipchamp",
                category: "debloat",
                name: "Remove Clipchamp",
                description: "Uninstalls the Clipchamp video editor.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Clipchamp.Clipchamp", ctx))
        },
        crate::tweak! {
                id: "debloat_disney",
                category: "debloat",
                name: "Remove Disney+",
                description: "Uninstalls the Disney+ app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Disney.37853FC22B2CE", ctx))
        },
        crate::tweak! {
                id: "debloat_spotify",
                category: "debloat",
                name: "Remove Spotify",
                description: "Uninstalls the Spotify app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("SpotifyAB.SpotifyMusic", ctx))
        },
        crate::tweak! {
                id: "debloat_films_tv",
                category: "debloat",
                name: "Remove Films and TV",
                description: "Uninstalls the Films and TV (Zune Video) app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.ZuneVideo", ctx))
        },
        crate::tweak! {
                id: "debloat_solitaire",
                category: "debloat",
                name: "Remove Solitaire",
                description: "Uninstalls Microsoft Solitaire Collection.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.MicrosoftSolitaireCollection", ctx))
        },
        crate::tweak! {
                id: "debloat_mail_calendar",
                category: "debloat",
                name: "Remove Mail and Calendar",
                description: "Uninstalls Windows Mail and Calendar apps.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("microsoft.windowscommunicationsapps", ctx))
        },
        crate::tweak! {
                id: "debloat_skype",
                category: "debloat",
                name: "Remove Skype",
                description: "Uninstalls the Skype app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.SkypeApp", ctx))
        },
        crate::tweak! {
                id: "debloat_people",
                category: "debloat",
                name: "Remove People",
                description: "Uninstalls the People app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.People", ctx))
        },
        crate::tweak! {
                id: "debloat_office_hub",
                category: "debloat",
                name: "Remove Office Hub",
                description: "Uninstalls the Microsoft Office Hub (Get Office) app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.MicrosoftOfficeHub", ctx))
        },
        crate::tweak! {
                id: "debloat_onenote",
                category: "debloat",
                name: "Remove OneNote",
                description: "Uninstalls the OneNote app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Office.OneNote", ctx))
        },
        crate::tweak! {
                id: "debloat_outlook",
                category: "debloat",
                name: "Remove Outlook",
                description: "Uninstalls the new Outlook for Windows.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.OutlookForWindows", ctx))
        },
        crate::tweak! {
                id: "debloat_todo",
                category: "debloat",
                name: "Remove To Do",
                description: "Uninstalls Microsoft To Do.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Todos", ctx))
        },
        crate::tweak! {
                id: "debloat_weather",
                category: "debloat",
                name: "Remove Weather",
                description: "Uninstalls the Bing Weather app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.BingWeather", ctx))
        },
        crate::tweak! {
                id: "debloat_news",
                category: "debloat",
                name: "Remove News",
                description: "Uninstalls the Bing News app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.BingNews", ctx))
        },
        crate::tweak! {
                id: "debloat_bing_search",
                category: "debloat",
                name: "Remove Bing Search",
                description: "Uninstalls the Bing Search app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.BingSearch", ctx))
        },
        crate::tweak! {
                id: "debloat_get_help",
                category: "debloat",
                name: "Remove Get Help",
                description: "Uninstalls the Get Help app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.GetHelp", ctx))
        },
        crate::tweak! {
                id: "debloat_tips",
                category: "debloat",
                name: "Remove Tips",
                description: "Uninstalls the Windows Tips app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Getstarted", ctx))
        },
        crate::tweak! {
                id: "debloat_feedback_hub",
                category: "debloat",
                name: "Remove Feedback Hub",
                description: "Uninstalls the Feedback Hub app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.WindowsFeedbackHub", ctx))
        },
        crate::tweak! {
                id: "debloat_power_automate",
                category: "debloat",
                name: "Remove Power Automate",
                description: "Uninstalls Power Automate Desktop.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.PowerAutomateDesktop", ctx))
        },
        crate::tweak! {
                id: "debloat_dev_home",
                category: "debloat",
                name: "Remove Dev Home",
                description: "Uninstalls Microsoft Dev Home.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Windows.DevHome", ctx))
        },
        crate::tweak! {
                id: "debloat_3d_viewer",
                category: "debloat",
                name: "Remove 3D Viewer",
                description: "Uninstalls the 3D Viewer app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.Microsoft3DViewer", ctx))
        },
        crate::tweak! {
                id: "debloat_paint_3d",
                category: "debloat",
                name: "Remove Paint 3D",
                description: "Uninstalls Paint 3D.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.MSPaint", ctx))
        },
        crate::tweak! {
                id: "debloat_mixed_reality",
                category: "debloat",
                name: "Remove Mixed Reality Portal",
                description: "Uninstalls the Mixed Reality Portal.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.MixedReality.Portal", ctx))
        },
        crate::tweak! {
                id: "debloat_family",
                category: "debloat",
                name: "Remove Family",
                description: "Uninstalls the Microsoft Family app.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("MicrosoftCorporationII.MicrosoftFamily", ctx))
        },
        crate::tweak! {
                id: "debloat_sticky_notes",
                category: "debloat",
                name: "Remove Sticky Notes",
                description: "Uninstalls Microsoft Sticky Notes.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.MicrosoftStickyNotes", ctx))
        },
        crate::tweak! {
                id: "debloat_maps",
                category: "debloat",
                name: "Remove Maps",
                description: "Uninstalls Windows Maps.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| remove_package("Microsoft.WindowsMaps", ctx))
        },
        crate::tweak! {
            id: "remove_mixed_reality_settings",
            category: "debloat",
            name: "Remove Mixed Reality Settings",
            description: "Removes the Mixed Reality page from the Settings app.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Holographic",
                    value_name: "FirstRunSucceeded",
                    value: RegistryValue::Dword(0),
                    stock_value: RegistryValue::Dword(1),
                },
            ],
            disabled_ops: Some(&[
                RegistryOp {
                    hkey: "HKCU",
                    subkey: r"Software\Microsoft\Windows\CurrentVersion\Holographic",
                    value_name: "FirstRunSucceeded",
                    value: RegistryValue::Dword(1),
                    stock_value: RegistryValue::Dword(1),
                },
            ]),
        },
];

fn remove_package(package_name: &str, ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status(&format!("Removing app package: {package_name}"));

        let ps_script = format!(r#"
        $pkg = '{package_name}'
        Get-AppxPackage -AllUsers -Name "*$pkg*" 2>$null | Remove-AppxPackage -AllUsers -ErrorAction SilentlyContinue
        Get-AppxProvisionedPackage -Online 2>$null | Where-Object {{ $_.PackageName -like "*$pkg*" }} | Remove-AppxProvisionedPackage -Online -ErrorAction SilentlyContinue
        "#);

        crate::run_system_command(
                "powershell.exe",
                &["-ExecutionPolicy", "Bypass", "-Command", &ps_script],
        )?;
        ctx.report_progress();
        Ok(())
}
