use super::Tweak;
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::sync::Arc;

pub static REMOVE_AI_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "disable_click_to_do",
            category: "remove_ai",
            name: "Disable Click To Do",
            description: "Disables the Click to Do AI feature.",
            enabled_ops: &[
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1),
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\Shell\ClickToDo", "DisableClickToDo", 1),
            ],
        },
        crate::tweak! {
            id: "disable_edge_copilot",
            category: "remove_ai",
            name: "Disable Copilot in Edge",
            description: "Disables Copilot features within the Microsoft Edge browser.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "CopilotPageContext", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "HubsSidebarEnabled", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeEntraCopilotPageContext", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeHistoryAISearchEnabled", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ComposeInlineEnabled", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "GenAILocalFoundationalModelSettings", 1),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "BuiltInAIAPIsEnabled", 0),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AIGenThemesEnabled", 0),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "DevToolsGenAiSettings", 2),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ShareBrowsingHistoryWithCopilotSearchAllowed", 0),
            ],
        },
        crate::tweak! {
            id: "disable_copilot_taskbar",
            category: "remove_ai",
            name: "Disable Copilot Taskbar Button",
            description: "Hides the Copilot button and 'Ask Copilot' from the taskbar.",
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowCopilotButton", 0),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarCompanion", 0),
                // Disable hardware key if present
                 crate::reg_str!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\CopilotKey", "SetCopilotHardwareKey", " "),
            ],
        },
        crate::tweak! {
            id: "disable_notepad_ai",
            category: "remove_ai",
            name: "Disable Notepad AI",
            description: "Disables AI features in Notepad (Rewrite, etc.).",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\WindowsNotepad", "DisableAIFeatures", 1),
            ],
        },
        crate::tweak! {
            id: "disable_office_ai",
            category: "remove_ai",
            name: "Disable Office AI",
            description: "Disables AI training and features in Microsoft Office.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\office\16.0\common\ai\training\general", "disabletraining", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\office\16.0\common\ai\training\specific\adaptivefloatie", "disabletrainingofadaptivefloatie", 1),
            ],
        },
        crate::tweak! {
            id: "disable_paint_ai",
            category: "remove_ai",
            name: "Disable Paint AI Features",
            description: "Disables Cocreator, Generative Fill, Image Creator, and other AI features in Paint.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableCocreator", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableGenerativeFill", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableImageCreator", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableGenerativeErase", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Paint", "DisableRemoveBackground", 1),
            ],
        },
        crate::tweak! {
            id: "disable_recall_tasks",
            category: "remove_ai",
            name: "Disable Recall/AI Tasks",
            description: "Disables scheduled tasks related to Windows AI and Recall.",
            enabled_ops: &[],
            custom_apply: Some(disable_recall_tasks_impl)
        },
        crate::tweak! {
            id: "disable_copilot",
            category: "remove_ai",
            name: "Disable Windows Copilot",
            description: "Disables Windows Copilot AI assistant completely, including data analysis and background agents.",
            enabled_ops: &[
                // Policies
                crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAIDataAnalysis", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableClickToDo", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "TurnOffSavingSnapshots", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableSettingsAgent", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAgentConnectors", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableAgentWorkspaces", 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "DisableRemoteAgentConnectors", 1),

                // Shell
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot\BingChat", "IsUserEligible", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot", "IsCopilotAvailable", 0),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows\Shell\Copilot", "CopilotDisabledReason", "FeatureIsDisabled"),

                // Capability Access
                 crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone\Microsoft.Copilot_8wekyb3d8bbwe", "Value", "Deny"),
                 crate::reg_str!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\systemAIModels", "Value", "Deny"),
                 crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\Capabilities\systemAIModels", "RecordUsageData", 0),

                 // Voice Activation
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\VoiceActivation\UserPreferenceForAllApps", "AgentActivationEnabled", 0),

                 // Search Box
                 crate::reg_dword!("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchBoxSuggestions", 1),
            ],
        },
        crate::tweak! {
            id: "disable_recall",
            category: "remove_ai",
            name: "Disable Windows Recall",
            description: "Disables Windows Recall AI feature which captures screenshots and analyzes activity.",
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement", 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "TurnOffSavingSnapshots", 1),
                 crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SettingSync\WindowsSettingHandlers", "A9HomeContentEnabled", 0),
            ],
        },
        crate::tweak! {
            id: "debloat_copilot",
            category: "remove_ai",
            name: "Remove Copilot App",
            description: "Uninstalls the Microsoft Copilot app.",
            enabled_ops: &[],
            custom_apply: Some(|ctx| {
                super::debloat::remove_package("Microsoft.Copilot", ctx)?;
                super::debloat::remove_package("Microsoft.Windows.Ai.Copilot.Provider", ctx)
            })
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
