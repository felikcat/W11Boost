use super::{Tweak, TweakEffect};
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::sync::Arc;

pub static REMOVE_AI_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_copilot",
            category: "remove_ai",
            name: "Disable Windows Copilot",
            description: "Disables Windows Copilot AI assistant completely, including data analysis and background agents.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                // Policies
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAIDataAnalysis", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "TurnOffSavingSnapshots", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableSettingsAgent", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAgentConnectors", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAgentWorkspaces", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableRemoteAgentConnectors", 1, RegistryValue::Delete),

                // Shell
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot\BingChat", "IsUserEligible", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot", "IsCopilotAvailable", 0, RegistryValue::Delete),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot", "CopilotDisabledReason", "FeatureIsDisabled", RegistryValue::Delete),

                // Capability Access
                 crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone\Microsoft.Copilot_8wekyb3d8bbwe", "Value", "Deny", "Prompt"),
                 crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\systemAIModels", "Value", "Deny", "Prompt"),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\Capabilities\systemAIModels", "RecordUsageData", 0, 1),

                 // Voice Activation
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationEnabled", 0, 1),

                 // Search Box
                 crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_copilot_taskbar",
            category: "remove_ai",
            name: "Disable Copilot Taskbar Button",
            description: "Hides the Copilot button and 'Ask Copilot' from the taskbar.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowCopilotButton", 0, RegistryValue::Delete),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarCompanion", 0, 1),
                // Disable hardware key if present
                 crate::reg_str!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\CopilotKey", "SetCopilotHardwareKey", " ", RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_recall",
            category: "remove_ai",
            name: "Disable Windows Recall",
            description: "Disables Windows Recall AI feature which captures screenshots and analyzes activity.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "TurnOffSavingSnapshots", 1),
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SettingSync\WindowsSettingHandlers", "A9HomeContentEnabled", 0, 1),
            ],
            requires_restart: true
        },
        crate::tweak! {
            id: "disable_edge_copilot",
            category: "remove_ai",
            name: "Disable Copilot in Edge",
            description: "Disables Copilot features within the Microsoft Edge browser.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "CopilotPageContext", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "HubsSidebarEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeEntraCopilotPageContext", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeHistoryAISearchEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ComposeInlineEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "GenAILocalFoundationalModelSettings", 1, RegistryValue::Delete),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "BuiltInAIAPIsEnabled", 0, RegistryValue::Delete),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AIGenThemesEnabled", 0, RegistryValue::Delete),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "DevToolsGenAiSettings", 2, RegistryValue::Delete),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ShareBrowsingHistoryWithCopilotSearchAllowed", 0, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_office_ai",
            category: "remove_ai",
            name: "Disable Office AI",
            description: "Disables AI training and features in Microsoft Office.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\office\16.0\common\ai\training\general", "disabletraining", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\office\16.0\common\ai\training\specific\adaptivefloatie", "disabletrainingofadaptivefloatie", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_paint_ai",
            category: "remove_ai",
            name: "Disable Paint AI Features",
            description: "Disables Cocreator, Generative Fill, Image Creator, and other AI features in Paint.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableCocreator", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableGenerativeFill", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableImageCreator", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableGenerativeErase", 1, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableRemoveBackground", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "disable_click_to_do",
            category: "remove_ai",
            name: "Disable Click To Do",
            description: "Disables the Click to Do AI feature.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1, RegistryValue::Delete),
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\Shell\ClickToDo", "DisableClickToDo", 1, RegistryValue::Delete),
            ],
        },
        crate::tweak! {
            id: "debloat_copilot",
            category: "remove_ai",
            name: "Remove Copilot App",
            description: "Uninstalls the Microsoft Copilot app.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],
            custom_apply: Some(|ctx| {
                super::debloat::remove_package("Microsoft.Copilot", ctx)?;
                super::debloat::remove_package("Microsoft.Windows.Ai.Copilot.Provider", ctx)
            })
        },
        crate::tweak! {
            id: "disable_recall_tasks",
            category: "remove_ai",
            name: "Disable Recall/AI Tasks",
            description: "Disables scheduled tasks related to Windows AI and Recall.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],
            custom_apply: Some(disable_recall_tasks_impl)
        },
        crate::tweak! {
            id: "disable_notepad_ai",
            category: "remove_ai",
            name: "Disable Notepad AI",
            description: "Disables AI features in Notepad (Rewrite, etc.).",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\WindowsNotepad", "DisableAIFeatures", 1, RegistryValue::Delete),
            ],
        },
];

fn disable_recall_tasks_impl(_ctx: &Arc<WorkerContext>) -> Result<()>
{
        let script = r#"
    Get-ScheduledTask -TaskPath '*' -ErrorAction SilentlyContinue | Where-Object { $_.TaskName -like '*WindowsAI*' -or $_.TaskPath -like '*WindowsAI*' } | Disable-ScheduledTask -ErrorAction SilentlyContinue
    Get-ScheduledTask -TaskName "*Office Actions Server*" -ErrorAction SilentlyContinue | Disable-ScheduledTask -ErrorAction SilentlyContinue
    "#;

        crate::common::run_system_command("powershell", &["-NoProfile", "-Command", script])
}
