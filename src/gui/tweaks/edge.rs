// Microsoft Edge tweaks

use super::{RegistryOp, RegistryValue, Tweak, TweakEffect};

pub static EDGE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_edge_updates",
                category: "edge",
                name: "Disable Edge Updates",
                description: "Prevents Microsoft Edge from auto-updating.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                        value_name: "UpdateDefault",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                        value_name: "UpdateDefault",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_edge_sidebar",
                category: "edge",
                name: "Disable Edge Sidebar",
                description: "Disables the sidebar in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                        value_name: "HubsSidebarEnabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                        value_name: "HubsSidebarEnabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_edge_first_run",
                category: "edge",
                name: "Disable Edge First Run Import",
                description: "Prevents Edge from importing browser data on first run.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ImportOnEachLaunch",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutoImportAtFirstRun",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ImportOnEachLaunch",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutoImportAtFirstRun",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_shortcut",
                category: "edge",
                name: "Disable Edge Desktop Shortcut",
                description: "Prevents Microsoft Edge from creating a desktop shortcut after updates.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                                value_name: "CreateDesktopShortcut{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                                value_name: "CreateDesktopShortcut{2CD8A007-E189-409D-A2C8-9AF4EF3C72AA}",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\EdgeUpdate",
                                value_name: "CreateDesktopShortcut{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_personalization",
                category: "edge",
                name: "Disable Edge Personalization",
                description: "Disables Edge's personalization features and reporting.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                        value_name: "PersonalizationReportingEnabled",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                        value_name: "PersonalizationReportingEnabled",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "enable_edge_do_not_track",
                category: "edge",
                name: "Enable Edge Do Not Track",
                description: "Enables the 'Do Not Track' request in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ConfigureDoNotTrack",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "ConfigureDoNotTrack",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ConfigureDoNotTrack",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "ConfigureDoNotTrack",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_payment_query",
                category: "edge",
                name: "Disable Edge Payment Method Query",
                description: "Prevents websites from checking if you have a payment method saved.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "PaymentMethodQueryEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "PaymentMethodQueryEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "PaymentMethodQueryEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "PaymentMethodQueryEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_bing_search",
                category: "edge",
                name: "Disable Edge Search in Bing",
                description: "Disables Microsoft Search in Bing from the address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AddressBarMicrosoftSearchInBingProviderEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AddressBarMicrosoftSearchInBingProviderEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AddressBarMicrosoftSearchInBingProviderEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AddressBarMicrosoftSearchInBingProviderEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_feedback",
                category: "edge",
                name: "Disable Edge User Feedback",
                description: "Disables the user feedback feature in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "UserFeedbackAllowed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "UserFeedbackAllowed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "UserFeedbackAllowed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "UserFeedbackAllowed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_autofill",
                category: "edge",
                name: "Disable Edge Autofill",
                description: "Disables credit card and address autofill in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutofillCreditCardEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AutofillCreditCardEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutofillAddressEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AutofillAddressEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutofillCreditCardEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AutofillCreditCardEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AutofillAddressEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AutofillAddressEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_search_suggestions",
                category: "edge",
                name: "Disable Edge Search Suggestions",
                description: "Disables search suggestions in the Edge address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SearchSuggestEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SearchSuggestEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SearchSuggestEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SearchSuggestEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_shopping",
                category: "edge",
                name: "Disable Edge Shopping Assistant",
                description: "Disables the shopping assistant in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "EdgeShoppingAssistantEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "EdgeShoppingAssistantEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "EdgeShoppingAssistantEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "EdgeShoppingAssistantEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_signin",
                category: "edge",
                name: "Disable Edge Browser Sign-in",
                description: "Disables the ability to sign in to Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "BrowserSignin",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "BrowserSignin",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "BrowserSignin",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "BrowserSignin",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_password_manager",
                category: "edge",
                name: "Disable Edge Password Manager",
                description: "Disables the built-in password manager in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "PasswordManagerEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "PasswordManagerEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "PasswordManagerEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "PasswordManagerEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_smartscreen",
                category: "edge",
                name: "Disable Edge SmartScreen",
                description: "Disables Microsoft Defender SmartScreen in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SmartScreenEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SmartScreenEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SmartScreenEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SmartScreenEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_typosquatting",
                category: "edge",
                name: "Disable Edge Typosquatting Checker",
                description: "Disables the typosquatting checker in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "TyposquattingCheckerEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "TyposquattingCheckerEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "TyposquattingCheckerEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "TyposquattingCheckerEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_prelaunch",
                category: "edge",
                name: "Disable Edge Prelaunch & Startup Boost",
                description: "Prevents Microsoft Edge from prelaunching or running in the background for startup boost.",
                effect: TweakEffect::Logoff,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main",
                                value_name: "AllowPrelaunch",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "StartupBoostEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
                        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\Main",
                                value_name: "AllowPrelaunch",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "StartupBoostEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
                        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_address_bar_dropdown",
                category: "edge",
                name: "Disable Edge Address Bar Dropdown",
                description: "Disables the dropdown suggestions in the Edge address bar.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Browser",
                        value_name: "AllowAddressBarDropdown",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\PolicyManager\current\device\Browser",
                        value_name: "AllowAddressBarDropdown",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_legacy_edge_features",
                category: "edge",
                name: "Disable Legacy Edge Tracking",
                description: "Disables tracking and suggestions in Legacy Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "DoNotTrack",
                                value: RegistryValue::Dword(1),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "ShowSearchSuggestionsGlobal",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "Use FormSuggest",
                                value: RegistryValue::String("no"),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "OptimizeWindowsSearchResultsForScreenReaders",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\FlipAhead",
                                value_name: "FPEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "DoNotTrack",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "ShowSearchSuggestionsGlobal",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "Use FormSuggest",
                                value: RegistryValue::String("yes"),
                                stock_value: RegistryValue::String("yes")
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Main",
                                value_name: "OptimizeWindowsSearchResultsForScreenReaders",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\FlipAhead",
                                value_name: "FPEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_cortana_search",
                category: "edge",
                name: "Disable Legacy Edge Cortana & History",
                description: "Disables Cortana and search history in Legacy Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI",
                                value_name: "EnableCortana",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI\ShowSearchHistory",
                                value_name: "",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI",
                                value_name: "EnableCortana",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\ServiceUI\ShowSearchHistory",
                                value_name: "",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_eme",
                category: "edge",
                name: "Disable Edge Encrypted Media",
                description: "Disables Encrypted Media Extensions (EME) in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Privacy",
                        value_name: "EnableEncryptedMediaExtensions",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\Privacy",
                        value_name: "EnableEncryptedMediaExtensions",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_edge_tab_preloading",
                category: "edge",
                name: "Disable Edge Tab Preloading",
                description: "Prevents Edge from preloading tabs for faster browsing.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader",
                        value_name: "AllowTabPreloading",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Policies\Microsoft\MicrosoftEdge\TabPreloader",
                        value_name: "AllowTabPreloading",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_edge_phishing_filter",
                category: "edge",
                name: "Disable Edge Phishing Filter",
                description: "Disables the phishing filter in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\PhishingFilter",
                        value_name: "EnabledV9",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::Delete
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKCU",
                        subkey: r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\microsoft.microsoftedge_8wekyb3d8bbwe\MicrosoftEdge\PhishingFilter",
                        value_name: "EnabledV9",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::Delete
        }])
        },
        crate::tweak! {
                id: "disable_edge_clsid_policy",
                category: "edge",
                name: "Disable Edge CLSID Policy",
                description: "Disables a specific CLSID policy related to Microsoft Edge extensions.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Ext\CLSID",
                        value_name: "{1FD49718-1D00-4B19-AF5F-070AF6D5D54C}",
                        value: RegistryValue::Dword(0),
                        stock_value: RegistryValue::String("1")
        }],
                disabled_ops: Some(&[RegistryOp {
                        hkey: "HKLM",
                        subkey: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Ext\CLSID",
                        value_name: "{1FD49718-1D00-4B19-AF5F-070AF6D5D54C}",
                        value: RegistryValue::Delete,
                        stock_value: RegistryValue::String("1")
        }])
        },
        crate::tweak! {
                id: "disable_edge_local_providers",
                category: "edge",
                name: "Disable Edge Local Providers",
                description: "Disables local providers for suggestions in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "LocalProvidersEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "LocalProvidersEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "LocalProvidersEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "LocalProvidersEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_web_widget",
                category: "edge",
                name: "Disable Edge Web Widget",
                description: "Disables the web widget in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "WebWidgetAllowed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "WebWidgetAllowed",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "WebWidgetAllowed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "WebWidgetAllowed",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_proofing",
                category: "edge",
                name: "Disable Edge Microsoft Editor",
                description: "Disables Microsoft Editor proofing services in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "MicrosoftEditorProofingEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "MicrosoftEditorProofingEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "MicrosoftEditorProofingEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "MicrosoftEditorProofingEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_nav_error_web_service",
                category: "edge",
                name: "Disable Edge Nav Error Web Service",
                description: "Disables using a web service to resolve navigation errors in Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ResolveNavigationErrorsUseWebService",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "ResolveNavigationErrorsUseWebService",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "ResolveNavigationErrorsUseWebService",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "ResolveNavigationErrorsUseWebService",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_alternate_error_pages",
                category: "edge",
                name: "Disable Edge Alternate Error Pages",
                description: "Disables alternate error pages in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AlternateErrorPagesEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AlternateErrorPagesEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "AlternateErrorPagesEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "AlternateErrorPagesEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "optimize_edge_network_prediction",
                category: "edge",
                name: "Optimize Edge Network Prediction",
                description: "Sets Edge network prediction options to 'off' (2).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "NetworkPredictionOptions",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "NetworkPredictionOptions",
                                value: RegistryValue::Dword(2),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "NetworkPredictionOptions",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "NetworkPredictionOptions",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
        crate::tweak! {
                id: "disable_edge_site_safety",
                category: "edge",
                name: "Disable Edge Site Safety Services",
                description: "Disables site safety services in Microsoft Edge.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SiteSafetyServicesEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SiteSafetyServicesEnabled",
                                value: RegistryValue::Dword(0),
                                stock_value: RegistryValue::Delete
        },
                ],
                disabled_ops: Some(&[
                        RegistryOp {
                                hkey: "HKLM",
                                subkey: r"SOFTWARE\Policies\Microsoft\Edge",
                                value_name: "SiteSafetyServicesEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                        RegistryOp {
                                hkey: "HKCU",
                                subkey: r"Software\Policies\Microsoft\Edge",
                                value_name: "SiteSafetyServicesEnabled",
                                value: RegistryValue::Delete,
                                stock_value: RegistryValue::Delete
        },
                ])
        },
];
