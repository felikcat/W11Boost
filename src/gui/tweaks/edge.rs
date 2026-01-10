// Microsoft Edge tweaks

use super::{RegistryValue, Tweak, TweakEffect};

pub static EDGE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_edge_updates",
                category: "edge",
                name: "Disable Edge Updates",
                description: "Prevents Microsoft Edge from auto-updating.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\EdgeUpdate", "UpdateDefault", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\EdgeUpdate", "UpdateDefault", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_sidebar",
                category: "edge",
                name: "Disable Edge Sidebar",
                description: "Disables the sidebar in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "HubsSidebarEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "HubsSidebarEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_first_run",
                category: "edge",
                name: "Disable Edge First Run Import",
                description: "Prevents Edge from importing browser data on first run.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ImportOnEachLaunch", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutoImportAtFirstRun", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ImportOnEachLaunch", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutoImportAtFirstRun", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_shortcut",
                category: "edge",
                name: "Disable Edge Desktop Shortcut",
                description: "Prevents Microsoft Edge from creating a desktop shortcut after updates.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\EdgeUpdate", "CreateDesktopShortcut{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\EdgeUpdate", "CreateDesktopShortcut{2CD8A007-E189-409D-A2C8-9AF4EF3C72AA}", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\EdgeUpdate", "CreateDesktopShortcut{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_personalization",
                category: "edge",
                name: "Disable Edge Personalization",
                description: "Disables Edge's personalization features and reporting.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PersonalizationReportingEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PersonalizationReportingEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "enable_edge_do_not_track",
                category: "edge",
                name: "Enable Edge Do Not Track",
                description: "Enables the 'Do Not Track' request in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ConfigureDoNotTrack", 1),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "ConfigureDoNotTrack", 1),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ConfigureDoNotTrack", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "ConfigureDoNotTrack", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_payment_query",
                category: "edge",
                name: "Disable Edge Payment Method Query",
                description: "Prevents websites from checking if you have a payment method saved.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PaymentMethodQueryEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "PaymentMethodQueryEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PaymentMethodQueryEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "PaymentMethodQueryEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_bing_search",
                category: "edge",
                name: "Disable Edge Search in Bing",
                description: "Disables Microsoft Search in Bing from the address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AddressBarMicrosoftSearchInBingProviderEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "AddressBarMicrosoftSearchInBingProviderEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AddressBarMicrosoftSearchInBingProviderEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "AddressBarMicrosoftSearchInBingProviderEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_feedback",
                category: "edge",
                name: "Disable Edge User Feedback",
                description: "Disables the user feedback feature in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "UserFeedbackAllowed", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "UserFeedbackAllowed", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "UserFeedbackAllowed", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "UserFeedbackAllowed", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_autofill",
                category: "edge",
                name: "Disable Edge Autofill",
                description: "Disables credit card and address autofill in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutofillCreditCardEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "AutofillCreditCardEnabled", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutofillAddressEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "AutofillAddressEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutofillCreditCardEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "AutofillCreditCardEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AutofillAddressEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "AutofillAddressEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_search_suggestions",
                category: "edge",
                name: "Disable Edge Search Suggestions",
                description: "Disables search suggestions in the Edge address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SearchSuggestEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "SearchSuggestEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SearchSuggestEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "SearchSuggestEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_shopping",
                category: "edge",
                name: "Disable Edge Shopping Assistant",
                description: "Disables the shopping assistant in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeShoppingAssistantEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "EdgeShoppingAssistantEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "EdgeShoppingAssistantEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "EdgeShoppingAssistantEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_signin",
                category: "edge",
                name: "Disable Edge Browser Sign-in",
                description: "Disables the ability to sign in to Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "BrowserSignin", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "BrowserSignin", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "BrowserSignin", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "BrowserSignin", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_password_manager",
                category: "edge",
                name: "Disable Edge Password Manager",
                description: "Disables the built-in password manager in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PasswordManagerEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "PasswordManagerEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "PasswordManagerEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "PasswordManagerEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_smartscreen",
                category: "edge",
                name: "Disable Edge SmartScreen",
                description: "Disables Microsoft Defender SmartScreen in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SmartScreenEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "SmartScreenEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SmartScreenEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "SmartScreenEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_typosquatting",
                category: "edge",
                name: "Disable Edge Typosquatting Checker",
                description: "Disables the typosquatting checker in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "TyposquattingCheckerEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "TyposquattingCheckerEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "TyposquattingCheckerEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "TyposquattingCheckerEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_prelaunch",
                category: "edge",
                name: "Disable Edge Prelaunch & Startup Boost",
                description: "Prevents Microsoft Edge from prelaunching or running in the background for startup boost.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main", "AllowPrelaunch", 0),
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "StartupBoostEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main", "AllowPrelaunch", RegistryValue::Delete),
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "StartupBoostEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_address_bar_dropdown",
                category: "edge",
                name: "Disable Edge Address Bar Dropdown",
                description: "Disables the dropdown suggestions in the Edge address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Browser", "AllowAddressBarDropdown", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\Browser", "AllowAddressBarDropdown", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_legacy_edge_features",
                category: "edge",
                name: "Disable Legacy Edge Tracking",
                description: "Disables tracking and suggestions in Legacy Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "DoNotTrack", 1),
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "ShowSearchSuggestionsGlobal", 0),
                        crate::reg_str!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "Use FormSuggest", "no"),
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "OptimizeWindowsSearchResultsForScreenReaders", 0),
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\FlipAhead", "FPEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "DoNotTrack", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "ShowSearchSuggestionsGlobal", RegistryValue::Delete),
                        crate::reg_str!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "Use FormSuggest", "yes"),
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main", "OptimizeWindowsSearchResultsForScreenReaders", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\FlipAhead", "FPEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_cortana_search",
                category: "edge",
                name: "Disable Legacy Edge Cortana & History",
                description: "Disables Cortana and search history in Legacy Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI", "EnableCortana", 0),
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI\ShowSearchHistory", "", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI", "EnableCortana", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI\ShowSearchHistory", "", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_eme",
                category: "edge",
                name: "Disable Edge Encrypted Media",
                description: "Disables Encrypted Media Extensions (EME) in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Privacy", "EnableEncryptedMediaExtensions", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Privacy", "EnableEncryptedMediaExtensions", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_tab_preloading",
                category: "edge",
                name: "Disable Edge Tab Preloading",
                description: "Prevents Edge from preloading tabs for faster browsing.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader", "AllowTabPreloading", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader", "AllowTabPreloading", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_phishing_filter",
                category: "edge",
                name: "Disable Edge Phishing Filter",
                description: "Disables the phishing filter in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\PhishingFilter", "EnabledV9", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKCU", r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\PhishingFilter", "EnabledV9", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_clsid_policy",
                category: "edge",
                name: "Disable Edge CLSID Policy",
                description: "Disables a specific CLSID policy related to Microsoft Edge extensions.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Ext\CLSID", "{1FD49718-1D00-4B19-AF5F-070AF6D5D54C}", 0, RegistryValue::String("1")),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Ext\CLSID", "{1FD49718-1D00-4B19-AF5F-070AF6D5D54C}", RegistryValue::String("1")),
                ])
        },
        crate::tweak! {
                id: "disable_edge_local_providers",
                category: "edge",
                name: "Disable Edge Local Providers",
                description: "Disables local providers for suggestions in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "LocalProvidersEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "LocalProvidersEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "LocalProvidersEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "LocalProvidersEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_web_widget",
                category: "edge",
                name: "Disable Edge Web Widget",
                description: "Disables the web widget in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "WebWidgetAllowed", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "WebWidgetAllowed", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "WebWidgetAllowed", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "WebWidgetAllowed", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_proofing",
                category: "edge",
                name: "Disable Edge Microsoft Editor",
                description: "Disables Microsoft Editor proofing services in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "MicrosoftEditorProofingEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "MicrosoftEditorProofingEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "MicrosoftEditorProofingEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "MicrosoftEditorProofingEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_nav_error_web_service",
                category: "edge",
                name: "Disable Edge Nav Error Web Service",
                description: "Disables using a web service to resolve navigation errors in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ResolveNavigationErrorsUseWebService", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "ResolveNavigationErrorsUseWebService", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "ResolveNavigationErrorsUseWebService", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "ResolveNavigationErrorsUseWebService", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_alternate_error_pages",
                category: "edge",
                name: "Disable Edge Alternate Error Pages",
                description: "Disables alternate error pages in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AlternateErrorPagesEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "AlternateErrorPagesEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "AlternateErrorPagesEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "AlternateErrorPagesEnabled", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "optimize_edge_network_prediction",
                category: "edge",
                name: "Optimize Edge Network Prediction",
                description: "Sets Edge network prediction options to 'off' (2).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "NetworkPredictionOptions", 2),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "NetworkPredictionOptions", 2),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "NetworkPredictionOptions", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "NetworkPredictionOptions", RegistryValue::Delete),
                ])
        },
        crate::tweak! {
                id: "disable_edge_site_safety",
                category: "edge",
                name: "Disable Edge Site Safety Services",
                description: "Disables site safety services in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SiteSafetyServicesEnabled", 0),
                        crate::reg_dword!("HKCU", r"Software\Policies\Microsoft\Edge", "SiteSafetyServicesEnabled", 0),
                ],
                disabled_ops: Some(&[
                        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "SiteSafetyServicesEnabled", RegistryValue::Delete),
                        crate::reg_del!("HKCU", r"Software\Policies\Microsoft\Edge", "SiteSafetyServicesEnabled", RegistryValue::Delete),
                ])
        },
];
