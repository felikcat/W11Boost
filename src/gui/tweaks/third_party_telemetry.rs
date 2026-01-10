// Third-party Telemetry tweaks

use super::{RegistryValue, Tweak, TweakEffect};

// Macro to generate env var tweaks
// Macro to generate env var tweaks
macro_rules! env_tweak {
        ($id:expr, $name:expr, $desc:expr, $var:expr, $val:expr) => {
                crate::tweak! {
                                id: $id,
                                category: "thirdparty_telemetry",
                                name: $name,
                                description: $desc,
                                effect: TweakEffect::Logoff,
                                enabled_ops: &[
                                    crate::reg_str!("HKCU", "Environment", $var, $val, RegistryValue::Delete)
                                ],
                                disabled_ops: Some(&[
                                    crate::reg_del!("HKCU", "Environment", $var, RegistryValue::Delete)
                                ])
                }
        };
}

// Macro for registry DWORD tweaks
macro_rules! reg_tweak {
        ($id:expr, $name:expr, $desc:expr, $hkey:expr, $subkey:expr, $value_name:expr, $val:expr) => {
                crate::tweak! {
                                id: $id,
                                category: "thirdparty_telemetry",
                                name: $name,
                                description: $desc,
                                effect: TweakEffect::Immediate,
                                enabled_ops: &[
                                    crate::reg_dword!($hkey, $subkey, $value_name, $val, RegistryValue::Delete)
                                ],
                                disabled_ops: Some(&[
                                    crate::reg_del!($hkey, $subkey, $value_name, RegistryValue::Delete)
                                ])
                }
        };
}

pub static THIRD_PARTY_TELEMETRY_TWEAKS: &[Tweak] = &[
        reg_tweak!(
                "office_client_telemetry",
                "Office Client Telemetry",
                "Disables Office client telemetry via common registry key",
                "HKCU",
                r"Software\Microsoft\office\common\clienttelemetry",
                "DisableTelemetry",
                1
        ),
        reg_tweak!(
                "office_client_telemetry_16",
                "Office 16.0 Client Telemetry",
                "Disables Office 16.0 client telemetry",
                "HKCU",
                r"Software\Microsoft\office\16.0\common\clienttelemetry",
                "DisableTelemetry",
                1
        ),
        reg_tweak!(
                "office_qm_enable",
                "Office QM Enable",
                "Disables Office Quality Metrics (QM)",
                "HKCU",
                r"Software\Microsoft\office\16.0\common",
                "QMEnable",
                0
        ),
        reg_tweak!(
                "office_feedback_enabled",
                "Office Feedback",
                "Disables Office feedback features",
                "HKCU",
                r"Software\Microsoft\office\16.0\common\feedback",
                "Enabled",
                0
        ),
        reg_tweak!(
                "visual_studio_telemetry_switch",
                "Visual Studio Telemetry Switch",
                "Turns off generic Visual Studio telemetry switch",
                "HKCU",
                r"Software\Microsoft\VisualStudio\Telemetry",
                "TurnOffSwitch",
                1
        ),
        reg_tweak!(
                "firefox_telemetry_hklm",
                "Firefox Telemetry (System)",
                "Disables Firefox telemetry via HKLM policy",
                "HKLM",
                r"Software\Policies\Mozilla\Firefox",
                "DisableTelemetry",
                1
        ),
        reg_tweak!(
                "firefox_telemetry_hkcu",
                "Firefox Telemetry (User)",
                "Disables Firefox telemetry via HKCU policy",
                "HKCU",
                r"Software\Policies\Mozilla\Firefox",
                "DisableTelemetry",
                1
        ),
        reg_tweak!(
                "office365_telemetry",
                "Office 365 Telemetry",
                "Sets Office 365 telemetry to minimum level (3)",
                "HKCU",
                r"Software\Policies\Microsoft\office\16.0\common\privacy",
                "SendTelemetry",
                3
        ),
        reg_tweak!(
                "nvidia_telemetry",
                "NVIDIA Telemetry",
                "Disables NVIDIA Control Panel telemetry",
                "HKCU",
                r"Software\NVIDIA Corporation\NVControlPanel2\Client",
                "OptInOrOutPreference",
                0
        ),
        // === .NET / Microsoft ===
        env_tweak!(
                "dotnet_cli_telemetry",
                ".NET CLI Telemetry",
                "Disables .NET CLI telemetry",
                "DOTNET_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "dotnet_interactive_telemetry",
                ".NET Interactive Telemetry",
                "Disables .NET Interactive CLI telemetry",
                "DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "dotnet_svcutil_telemetry",
                ".NET SvcUtil Telemetry",
                "Disables .NET SvcUtil telemetry",
                "DOTNET_SVCUTIL_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "mldotnet_telemetry",
                "ML.NET CLI Telemetry",
                "Disables ML.NET CLI telemetry",
                "MLDOTNET_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "mssql_cli_telemetry",
                "MSSQL CLI Telemetry",
                "Disables MSSQL CLI telemetry",
                "MSSQL_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "vstest_telemetry",
                "VSTest Telemetry",
                "Disables VSTest telemetry",
                "VSTEST_TELEMETRY_OPTEDIN",
                "0"
        ),
        env_tweak!(
                "powershell_telemetry",
                "PowerShell Telemetry",
                "Disables PowerShell telemetry",
                "POWERSHELL_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "azure_core_telemetry",
                "Azure Core Telemetry",
                "Disables Azure Core telemetry",
                "AZURE_CORE_COLLECT_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "azure_dev_telemetry",
                "Azure Dev Telemetry",
                "Disables Azure Dev telemetry",
                "AZURE_DEV_COLLECT_TELEMETRY",
                "no"
        ),
        env_tweak!(
                "azureml_sdk_telemetry",
                "Azure ML SDK Telemetry",
                "Disables Azure ML SDK v2 telemetry",
                "AZUREML_SDKV2_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "azure_functions_telemetry",
                "Azure Functions Telemetry",
                "Disables Azure Functions Core Tools telemetry",
                "FUNCTIONS_CORE_TOOLS_TELEMETRY_OPTOUT",
                "1"
        ),
        // === Node.js / JavaScript Ecosystem ===
        env_tweak!(
                "nextjs_telemetry",
                "Next.js Telemetry",
                "Disables Next.js telemetry",
                "NEXT_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "nuxt_telemetry",
                "Nuxt Telemetry",
                "Disables Nuxt telemetry",
                "NUXT_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "gatsby_telemetry",
                "Gatsby Telemetry",
                "Disables Gatsby telemetry",
                "GATSBY_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "angular_cli_analytics",
                "Angular CLI Analytics",
                "Disables Angular CLI analytics",
                "NG_CLI_ANALYTICS",
                "false"
        ),
        env_tweak!(
                "angular_cli_analytics_share",
                "Angular CLI Analytics Share",
                "Disables Angular CLI analytics sharing",
                "NG_CLI_ANALYTICS_SHARE",
                "false"
        ),
        env_tweak!(
                "astro_telemetry",
                "Astro Telemetry",
                "Disables Astro telemetry",
                "ASTRO_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "storybook_telemetry",
                "Storybook Telemetry",
                "Disables Storybook telemetry",
                "STORYBOOK_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "redwood_telemetry",
                "Redwood Telemetry",
                "Disables Redwood telemetry",
                "REDWOOD_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "yarn_telemetry",
                "Yarn Telemetry",
                "Disables Yarn telemetry",
                "YARN_ENABLE_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "hint_telemetry",
                "Hint Telemetry",
                "Disables webhint telemetry",
                "HINT_TELEMETRY",
                "off"
        ),
        env_tweak!(
                "vuedx_telemetry",
                "VueDX Telemetry",
                "Disables VueDX telemetry",
                "VUEDX_TELEMETRY",
                "off"
        ),
        env_tweak!(
                "strapi_telemetry",
                "Strapi Telemetry",
                "Disables Strapi telemetry",
                "STRAPI_TELEMETRY_DISABLED",
                "true"
        ),
        env_tweak!(
                "serverless_telemetry",
                "Serverless Telemetry",
                "Disables Serverless Framework telemetry",
                "SLS_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "serverless_tracking",
                "Serverless Tracking",
                "Disables Serverless Framework tracking",
                "SLS_TRACKING_DISABLED",
                "1"
        ),
        env_tweak!(
                "calcom_telemetry",
                "Cal.com Telemetry",
                "Disables Cal.com telemetry",
                "CALCOM_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "sku_telemetry",
                "SKU Telemetry",
                "Disables SKU telemetry",
                "SKU_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "ember_cli_analytics",
                "Ember CLI Analytics",
                "Disables Ember CLI analytics",
                "EMBER_CLI_ANALYTICS",
                "false"
        ),
        env_tweak!(
                "capacitor_telemetry",
                "Capacitor Telemetry",
                "Disables Capacitor telemetry",
                "CAPACITOR_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "carbon_telemetry",
                "Carbon Telemetry",
                "Disables Carbon telemetry",
                "CARBON_TELEMETRY_DISABLED",
                "1"
        ),
        // === Python Ecosystem ===
        env_tweak!(
                "dagster_telemetry",
                "Dagster Telemetry",
                "Disables Dagster telemetry",
                "DAGSTER_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "feast_telemetry",
                "Feast Telemetry",
                "Disables Feast telemetry",
                "FEAST_TELEMETRY",
                "False"
        ),
        env_tweak!(
                "meltano_tracking",
                "Meltano Tracking",
                "Disables Meltano tracking",
                "MELTANO_DISABLE_TRACKING",
                "True"
        ),
        env_tweak!(
                "rasa_telemetry",
                "Rasa Telemetry",
                "Disables Rasa telemetry",
                "RASA_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "hamilton_telemetry",
                "Hamilton Telemetry",
                "Disables Hamilton telemetry",
                "HAMILTON_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "huggingface_telemetry",
                "Hugging Face Telemetry",
                "Disables Hugging Face Hub telemetry",
                "HF_HUB_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "gradio_analytics",
                "Gradio Analytics",
                "Disables Gradio analytics",
                "GRADIO_ANALYTICS_ENABLED",
                "False"
        ),
        env_tweak!(
                "ragas_telemetry",
                "Ragas Telemetry",
                "Disables Ragas telemetry",
                "RAGAS_DO_NOT_TRACK",
                "1"
        ),
        env_tweak!(
                "openllm_telemetry",
                "OpenLLM Telemetry",
                "Disables OpenLLM telemetry",
                "OPENLLM_DO_NOT_TRACK",
                "1"
        ),
        env_tweak!(
                "flower_telemetry",
                "Flower Telemetry",
                "Disables Flower telemetry",
                "FLWR_TELEMETRY_ENABLED",
                "0"
        ),
        env_tweak!(
                "streamlit_telemetry",
                "Streamlit Telemetry",
                "Disables Streamlit telemetry",
                "STREAMLIT_TELEMETRY_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "whylogs_analytics",
                "WhyLogs Analytics",
                "Disables WhyLogs analytics",
                "WHYLOGS_NO_ANALYTICS",
                "true"
        ),
        env_tweak!(
                "jina_telemetry",
                "Jina Telemetry",
                "Disables Jina telemetry",
                "JINA_OPTOUT_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "schemathesis_telemetry",
                "Schemathesis Telemetry",
                "Disables Schemathesis telemetry",
                "SCHEMATHESIS_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "dbt_telemetry",
                "dbt Telemetry",
                "Disables dbt anonymous usage stats",
                "DBT_SEND_ANONYMOUS_USAGE_STATS",
                "false"
        ),
        // === DevOps / Infrastructure ===
        env_tweak!(
                "terraform_telemetry",
                "Terraform Telemetry",
                "Disables Terraform telemetry",
                "TERRAFORM_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "hashicorp_checkpoint",
                "HashiCorp Checkpoint",
                "Disables HashiCorp checkpoint",
                "CHECKPOINT_DISABLE",
                "1"
        ),
        env_tweak!(
                "vagrant_checkpoint",
                "Vagrant Checkpoint",
                "Disables Vagrant checkpoint",
                "VAGRANT_CHECKPOINT_DISABLE",
                "1"
        ),
        env_tweak!(
                "packer_checkpoint",
                "Packer Checkpoint",
                "Disables Packer checkpoint",
                "PACKER_CHECKPOINT_DISABLE",
                "1"
        ),
        env_tweak!(
                "consul_checkpoint",
                "Consul Checkpoint",
                "Disables Consul checkpoint",
                "CONSUL_CHECKPOINT_DISABLE",
                "1"
        ),
        env_tweak!(
                "arm_terraform_partner",
                "ARM Terraform Partner ID",
                "Disables ARM Terraform partner ID",
                "ARM_DISABLE_TERRAFORM_PARTNER_ID",
                "true"
        ),
        env_tweak!(
                "chef_telemetry",
                "Chef Telemetry",
                "Disables Chef telemetry",
                "CHEF_TELEMETRY_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "automatedlab_telemetry",
                "AutomatedLab Telemetry",
                "Disables AutomatedLab telemetry",
                "AUTOMATEDLAB_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "nuke_telemetry",
                "NUKE Telemetry",
                "Disables NUKE build telemetry",
                "NUKE_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "pnp_powershell_telemetry",
                "PnP PowerShell Telemetry",
                "Disables PnP PowerShell telemetry",
                "PNPPOWERSHELL_DISABLETELEMETRY",
                "true"
        ),
        env_tweak!(
                "earthly_analytics",
                "Earthly Analytics",
                "Disables Earthly analytics",
                "EARTHLY_DISABLE_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "werf_telemetry",
                "Werf Telemetry",
                "Disables Werf telemetry",
                "WERF_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "scout_telemetry",
                "Scout Telemetry",
                "Disables Scout telemetry",
                "SCOUT_DISABLE",
                "1"
        ),
        env_tweak!(
                "infracost_telemetry",
                "Infracost Telemetry",
                "Disables Infracost telemetry",
                "INFRACOST_SELF_HOSTED_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "batect_telemetry",
                "Batect Telemetry",
                "Disables Batect telemetry",
                "BATECT_ENABLE_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "deck_analytics",
                "Deck Analytics",
                "Disables Deck analytics",
                "DECK_ANALYTICS",
                "off"
        ),
        env_tweak!(
                "do_not_track",
                "Do Not Track (Generic)",
                "Sets generic DO_NOT_TRACK flag",
                "DO_NOT_TRACK",
                "1"
        ),
        env_tweak!(
                "kics_telemetry",
                "KICS Telemetry",
                "Disables KICS telemetry",
                "KICS_COLLECT_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "crash_report",
                "Crash Report (Generic)",
                "Disables generic crash reporting",
                "DISABLE_CRASH_REPORT",
                "1"
        ),
        env_tweak!(
                "circleci_telemetry",
                "CircleCI CLI Telemetry",
                "Disables CircleCI CLI telemetry",
                "CIRCLECI_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "coder_telemetry",
                "Coder Telemetry",
                "Disables Coder telemetry",
                "CODER_TELEMETRY_ENABLE",
                "false"
        ),
        // === Cloud Providers ===
        env_tweak!(
                "sam_cli_telemetry",
                "SAM CLI Telemetry",
                "Disables AWS SAM CLI telemetry",
                "SAM_CLI_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "gcloud_usage_reporting",
                "Google Cloud Usage Reporting",
                "Disables Google Cloud SDK usage reporting",
                "CLOUDSDK_CORE_DISABLE_USAGE_REPORTING",
                "true"
        ),
        env_tweak!(
                "hookdeck_telemetry",
                "Hookdeck CLI Telemetry",
                "Disables Hookdeck CLI telemetry",
                "HOOKDECK_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "stripe_cli_telemetry",
                "Stripe CLI Telemetry",
                "Disables Stripe CLI telemetry",
                "STRIPE_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "f5_telemetry",
                "F5 Telemetry",
                "Disables F5 telemetry",
                "F5_ALLOW_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "teem_telemetry",
                "Teem Telemetry",
                "Disables Teem telemetry",
                "TEEM_DISABLE",
                "true"
        ),
        env_tweak!(
                "mslab_telemetry",
                "MSLab Telemetry",
                "Disables MSLab telemetry",
                "MSLAB_TELEMETRY_LEVEL",
                "None"
        ),
        // === Database Tools ===
        env_tweak!(
                "influxdb_reporting",
                "InfluxDB Reporting",
                "Disables InfluxDB reporting",
                "INFLUXD_REPORTING_DISABLED",
                "true"
        ),
        env_tweak!(
                "quilt_metrics",
                "Quilt Metrics",
                "Disables Quilt usage metrics",
                "QUILT_DISABLE_USAGE_METRICS",
                "True"
        ),
        env_tweak!(
                "qdrant_telemetry",
                "Qdrant Telemetry",
                "Disables Qdrant telemetry",
                "QDRANT__TELEMETRY_DISABLED",
                "true"
        ),
        env_tweak!(
                "mongodb_atlas_telemetry",
                "MongoDB Atlas Telemetry",
                "Disables MongoDB Atlas CLI telemetry",
                "MONGODB_ATLAS_TELEMETRY_ENABLE",
                "false"
        ),
        env_tweak!(
                "ferretdb_telemetry",
                "FerretDB Telemetry",
                "Disables FerretDB telemetry",
                "FERRETDB_TELEMETRY",
                "disable"
        ),
        env_tweak!(
                "cubestore_telemetry",
                "Cube Store Telemetry",
                "Disables Cube Store telemetry",
                "CUBESTORE_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "cubejs_telemetry",
                "Cube.js Telemetry",
                "Disables Cube.js telemetry",
                "CUBEJS_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "eventstore_telemetry",
                "EventStore Telemetry",
                "Disables EventStore telemetry",
                "EVENTSTORE_TELEMETRY_OPTOUT",
                "1"
        ),
        // === Shells & CLI Tools ===
        env_tweak!(
                "homebrew_analytics",
                "Homebrew Analytics",
                "Disables Homebrew analytics",
                "HOMEBREW_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "choosenim_analytics",
                "Choosenim Analytics",
                "Disables Choosenim analytics",
                "CHOOSENIM_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "cocoapods_stats",
                "CocoaPods Stats",
                "Disables CocoaPods stats",
                "COCOAPODS_DISABLE_STATS",
                "true"
        ),
        env_tweak!(
                "arduino_metrics",
                "Arduino Metrics",
                "Disables Arduino metrics",
                "ARDUINO_METRICS_ENABLED",
                "false"
        ),
        env_tweak!(
                "rockset_cli_telemetry",
                "Rockset CLI Telemetry",
                "Disables Rockset CLI telemetry",
                "ROCKSET_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "apollo_telemetry",
                "Apollo Telemetry",
                "Disables Apollo telemetry",
                "APOLLO_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "sfdx_telemetry",
                "SFDX Telemetry",
                "Disables Salesforce DX telemetry",
                "SFDX_DISABLE_TELEMETRY",
                "true"
        ),
        env_tweak!(
                "sf_telemetry",
                "Salesforce CLI Telemetry",
                "Disables Salesforce CLI telemetry",
                "SF_DISABLE_TELEMETRY",
                "true"
        ),
        env_tweak!(
                "salto_telemetry",
                "Salto Telemetry",
                "Disables Salto telemetry",
                "SALTO_TELEMETRY_DISABLE",
                "1"
        ),
        env_tweak!(
                "bf_cli_telemetry",
                "BF CLI Telemetry",
                "Disables Bot Framework CLI telemetry",
                "BF_CLI_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "mobile_center_telemetry",
                "Mobile Center Telemetry",
                "Disables Mobile Center telemetry",
                "MOBILE_CENTER_TELEMETRY",
                "off"
        ),
        env_tweak!(
                "appcd_telemetry",
                "Appcd Telemetry",
                "Disables Appcd telemetry",
                "APPCD_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "tuist_stats",
                "Tuist Stats",
                "Disables Tuist stats",
                "TUIST_STATS_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "go_telemetry",
                "Go Telemetry",
                "Disables Go telemetry",
                "GOTELEMETRY",
                "off"
        ),
        // === AI/ML Tools ===
        env_tweak!(
                "ray_usage_stats",
                "Ray Usage Stats",
                "Disables Ray usage stats",
                "RAY_USAGE_STATS_ENABLED",
                "0"
        ),
        env_tweak!(
                "aptos_telemetry",
                "Aptos Telemetry",
                "Disables Aptos telemetry",
                "APTOS_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "speedster_telemetry",
                "Speedster Telemetry",
                "Disables Speedster telemetry",
                "SPEEDSTER_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "deepchecks_telemetry",
                "Deepchecks Telemetry",
                "Disables Deepchecks telemetry",
                "DISABLE_DEEPCHECKS_ANONYMOUS_TELEMETRY",
                "True"
        ),
        env_tweak!(
                "dcs_telemetry",
                "DCS Telemetry",
                "Disables DCS telemetry",
                "DISABLE_DCS_ANONYMOUS_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "dacfx_telemetry",
                "DacFx Telemetry",
                "Disables DacFx telemetry",
                "DACFX_TELEMETRY_OPTOUT",
                "1"
        ),
        // === Web Frameworks ===
        env_tweak!(
                "webiny_telemetry",
                "Webiny Telemetry",
                "Disables Webiny telemetry",
                "REACT_APP_WEBINY_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "prisma_telemetry",
                "Prisma Telemetry",
                "Disables Prisma telemetry",
                "PRISMA_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "oryx_telemetry",
                "Oryx Telemetry",
                "Disables Oryx telemetry",
                "ORYX_DISABLE_TELEMETRY",
                "true"
        ),
        env_tweak!(
                "sqa_telemetry",
                "SQA Telemetry",
                "Disables SQA telemetry",
                "SQA_OPT_OUT",
                "true"
        ),
        env_tweak!(
                "hasura_telemetry",
                "Hasura Telemetry",
                "Disables Hasura GraphQL telemetry",
                "HASURA_GRAPHQL_ENABLE_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "meilisearch_analytics",
                "Meilisearch Analytics",
                "Disables Meilisearch analytics",
                "MEILI_NO_ANALYTICS",
                "true"
        ),
        env_tweak!(
                "nocodb_telemetry",
                "NocoDB Telemetry",
                "Disables NocoDB telemetry",
                "NOCODB_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "nc_telemetry",
                "NC Telemetry",
                "Disables NC telemetry",
                "NC_DISABLE_TELE",
                "1"
        ),
        env_tweak!(
                "prose_telemetry",
                "Prose Telemetry",
                "Disables Prose telemetry",
                "PROSE_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "restler_telemetry",
                "RESTler Telemetry",
                "Disables RESTler telemetry",
                "RESTLER_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "projector_telemetry",
                "Projector Telemetry",
                "Disables Projector telemetry",
                "PROJECTOR_TELEMETRY_ENABLED",
                "0"
        ),
        env_tweak!(
                "medusa_telemetry",
                "Medusa Telemetry",
                "Disables Medusa telemetry",
                "MEDUSA_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "generic_telemetry_enabled",
                "Generic Telemetry Enabled",
                "Disables generic TELEMETRY_ENABLED flag",
                "TELEMETRY_ENABLED",
                "0"
        ),
        // === Build Tools ===
        env_tweak!(
                "alibuild_analytics",
                "AliBuild Analytics",
                "Disables AliBuild analytics",
                "ALIBUILD_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "fastlane_usage",
                "Fastlane Usage",
                "Disables Fastlane usage tracking",
                "FASTLANE_OPT_OUT_USAGE",
                "YES"
        ),
        env_tweak!(
                "coverity_telemetry",
                "Coverity CLI Telemetry",
                "Disables Coverity CLI telemetry",
                "COVERITY_CLI_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "gradle_enterprise_analytics",
                "Gradle Enterprise Analytics",
                "Disables Gradle Enterprise analytics",
                "GRADLE_ENTERPRISE_ANALYTICS_DISABLE",
                "1"
        ),
        env_tweak!(
                "lost_pixel_telemetry",
                "Lost Pixel Telemetry",
                "Disables Lost Pixel telemetry",
                "LOST_PIXEL_DISABLE_TELEMETRY",
                "1"
        ),
        // === Container & Kubernetes ===
        env_tweak!(
                "docker_scan_suggest",
                "Docker Scan Suggest",
                "Disables Docker scan suggestions",
                "DOCKER_SCAN_SUGGEST",
                "false"
        ),
        env_tweak!(
                "kubeapt_telemetry",
                "KubeAPT Telemetry",
                "Disables KubeAPT telemetry",
                "KUBEAPT_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "dash_telemetry",
                "Dash Telemetry",
                "Disables Dash telemetry",
                "DASH_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "dagger_telemetry",
                "Dagger Telemetry",
                "Disables Dagger telemetry",
                "DAGGER_TELEMETRY_DISABLE",
                "1"
        ),
        env_tweak!(
                "neonkube_telemetry",
                "NeonKUBE Telemetry",
                "Disables NeonKUBE telemetry",
                "NEONKUBE_DISABLE_TELEMETRY",
                "true"
        ),
        env_tweak!(
                "otterize_telemetry",
                "Otterize Telemetry",
                "Disables Otterize telemetry",
                "OTTERIZE_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "porter_telemetry",
                "Porter Telemetry",
                "Disables Porter telemetry",
                "PORTER_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "preevy_telemetry",
                "Preevy Telemetry",
                "Disables Preevy telemetry",
                "PREEVY_DISABLE_TELEMETRY",
                "1"
        ),
        // === Testing Tools ===
        env_tweak!(
                "reportportal_analytics",
                "ReportPortal Analytics",
                "Disables ReportPortal analytics",
                "REPORTPORTAL_CLIENT_JS_NO_ANALYTICS",
                "true"
        ),
        env_tweak!(
                "agent_analytics",
                "Agent Analytics",
                "Disables Agent analytics",
                "AGENT_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "bugger_off",
                "Bugger Off",
                "Disables Bugger telemetry",
                "BUGGER_OFF",
                "1"
        ),
        env_tweak!(
                "suggestions_optout",
                "Suggestions Opt-out",
                "Disables suggestions tracking",
                "SUGGESTIONS_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "da_test_telemetry",
                "DA Test Telemetry",
                "Disables DA Test telemetry",
                "DA_TEST_DISABLE_TELEMETRY",
                "1"
        ),
        // === Misc Developer Tools ===
        env_tweak!(
                "et_telemetry",
                "ET Telemetry",
                "Disables ET telemetry",
                "ET_NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "lynx_analytics",
                "Lynx Analytics",
                "Disables Lynx analytics",
                "LYNX_ANALYTICS",
                "0"
        ),
        env_tweak!(
                "quickwit_telemetry",
                "Quickwit Telemetry",
                "Disables Quickwit telemetry",
                "DISABLE_QUICKWIT_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "automagica_telemetry",
                "Automagica Telemetry",
                "Disables Automagica telemetry",
                "AUTOMAGICA_NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "netdata_stats",
                "Netdata Stats",
                "Disables Netdata anonymous statistics",
                "NETDATA_ANONYMOUS_STATISTICS",
                "0"
        ),
        env_tweak!(
                "tilt_telemetry",
                "Tilt Telemetry",
                "Disables Tilt telemetry",
                "TILT_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "mattermost_diagnostics",
                "Mattermost Diagnostics",
                "Disables Mattermost diagnostics",
                "MM_LOGSETTINGS_ENABLEDIAGNOSTICS",
                "false"
        ),
        env_tweak!(
                "ls_metrics_host",
                "LS Metrics Host",
                "Disables LS Metrics Host",
                "LS_METRICS_HOST_ENABLED",
                "0"
        ),
        env_tweak!(
                "pants_telemetry",
                "Pants Telemetry",
                "Disables Pants anonymous telemetry",
                "PANTS_ANONYMOUS_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "flagsmith_telemetry",
                "Flagsmith Telemetry",
                "Disables Flagsmith telemetry",
                "FLAGSMITH_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "one_codex_telemetry",
                "One Codex Telemetry",
                "Disables One Codex telemetry",
                "ONE_CODEX_NO_TELEMETRY",
                "True"
        ),
        env_tweak!(
                "aitools_vscode_telemetry",
                "AI Tools VSCode Telemetry",
                "Disables AI Tools VSCode telemetry",
                "AITOOLSVSCODE_DISABLETELEMETRY",
                "1"
        ),
        env_tweak!(
                "ig_pro_optout",
                "IG Pro Opt-out",
                "Disables IG Pro tracking",
                "IG_PRO_OPT_OUT",
                "YES"
        ),
        env_tweak!(
                "redocly_telemetry",
                "Redocly Telemetry",
                "Disables Redocly telemetry",
                "REDOCLY_TELEMETRY",
                "off"
        ),
        env_tweak!(
                "hardhat_telemetry",
                "Hardhat Telemetry",
                "Disables Hardhat telemetry prompt",
                "HARDHAT_DISABLE_TELEMETRY_PROMPT",
                "1"
        ),
        env_tweak!(
                "tdengine_telemetry",
                "TDengine Telemetry",
                "Disables TDengine telemetry reporting",
                "TAOS_TELEMETRY_REPORTING",
                "0"
        ),
        env_tweak!(
                "metaflow_telemetry",
                "Metaflow Telemetry",
                "Disables Metaflow telemetry",
                "MF_SEND_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "teleport_telemetry",
                "Teleport Telemetry",
                "Disables Teleport anonymous telemetry",
                "TELEPORT_ANONYMOUS_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "tunnelmole_telemetry",
                "Tunnelmole Telemetry",
                "Disables Tunnelmole telemetry",
                "TUNNELMOLE_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "wundergraph_telemetry",
                "WunderGraph Telemetry",
                "Disables WunderGraph telemetry",
                "WG_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "anycable_telemetry",
                "AnyCable Telemetry",
                "Disables AnyCable telemetry",
                "ANYCABLE_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "rig_telemetry",
                "Rig Telemetry",
                "Disables Rig telemetry",
                "RIG_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "electric_telemetry",
                "Electric Telemetry",
                "Disables Electric telemetry",
                "ELECTRIC_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "snowflake_telemetry",
                "Snowflake Telemetry",
                "Disables Snowflake telemetry",
                "SNOWFLAKE_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "oasdiff_telemetry",
                "OASDiff Telemetry",
                "Disables OASDiff telemetry",
                "OASDIFF_NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "emqx_telemetry",
                "EMQX Telemetry",
                "Disables EMQX telemetry",
                "EMQX_TELEMETRY__ENABLE",
                "false"
        ),
        env_tweak!(
                "karate_telemetry",
                "Karate Telemetry",
                "Disables Karate telemetry",
                "KARATE_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "conjur_telemetry",
                "Conjur Telemetry",
                "Disables Conjur telemetry",
                "CONJUR_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "conjur_feature_telemetry",
                "Conjur Feature Telemetry",
                "Disables Conjur feature telemetry endpoint",
                "CONJUR_FEATURE_TELEMETRY_ENDPOINT_ENABLED",
                "false"
        ),
        env_tweak!(
                "taiga_telemetry",
                "Taiga Telemetry",
                "Disables Taiga telemetry",
                "TAIGA_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!("fa_notrack", "FA NoTrack", "Disables FA tracking", "FA_NOTRACK", "true"),
        env_tweak!(
                "infisical_telemetry",
                "Infisical Telemetry",
                "Disables Infisical telemetry",
                "INFISICAL_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "boxyhq_analytics",
                "BoxyHQ Analytics",
                "Disables BoxyHQ analytics",
                "BOXYHQ_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "adserver_tracking",
                "Adserver Tracking",
                "Disables Adserver tracking",
                "ADSERVER_DO_NOT_TRACK",
                "1"
        ),
        env_tweak!(
                "upstash_telemetry",
                "Upstash Telemetry",
                "Disables Upstash telemetry",
                "UPSTASH_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "bitrise_analytics",
                "Bitrise Analytics",
                "Disables Bitrise analytics",
                "BITRISE_ANALYTICS_DISABLED",
                "1"
        ),
        env_tweak!(
                "akita_telemetry",
                "Akita Telemetry",
                "Disables Akita telemetry",
                "AKITA_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "redgate_telemetry",
                "Redgate Telemetry",
                "Disables Redgate telemetry",
                "REDGATE_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "flake_checker_telemetry",
                "Flake Checker Telemetry",
                "Disables Flake Checker telemetry",
                "FLAKE_CHECKER_NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "pp_tools_telemetry",
                "PP Tools Telemetry",
                "Disables PP Tools telemetry",
                "PP_TOOLS_TELEMETRY_OPTOUT",
                "1"
        ),
        env_tweak!(
                "pipelines_telemetry",
                "Pipelines Telemetry",
                "Disables Pipelines telemetry",
                "PIPELINES_TELEMETRY_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "patcher_telemetry",
                "Patcher Telemetry",
                "Disables Patcher telemetry",
                "PATCHER_TELEMETRY_OPT_OUT",
                "1"
        ),
        env_tweak!(
                "retraced_telemetry",
                "Retraced Telemetry",
                "Disables Retraced telemetry",
                "RETRACED_NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "keystone_telemetry",
                "Keystone Telemetry",
                "Disables Keystone telemetry",
                "KEYSTONE_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "stp_telemetry",
                "STP Telemetry",
                "Disables STP telemetry",
                "STP_DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "usage_disable",
                "Usage Disable (Generic)",
                "Disables generic USAGE_DISABLE flag",
                "USAGE_DISABLE",
                "1"
        ),
        env_tweak!(
                "analytics_disabled",
                "Analytics Disabled (Generic)",
                "Disables generic ANALYTICS_DISABLED flag",
                "ANALYTICS_DISABLED",
                "1"
        ),
        env_tweak!(
                "orbit_telemetry",
                "Orbit Telemetry",
                "Disables Orbit telemetry",
                "ORBIT_TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "fal_stats",
                "FAL Stats",
                "Disables FAL stats",
                "FAL_STATS_ENABLED",
                "false"
        ),
        env_tweak!(
                "leon_telemetry",
                "Leon Telemetry",
                "Disables Leon AI telemetry",
                "LEON_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "ap_telemetry",
                "AP Telemetry",
                "Disables AP telemetry",
                "AP_TELEMETRY_ENABLED",
                "false"
        ),
        env_tweak!(
                "iterative_telemetry",
                "Iterative Telemetry",
                "Disables Iterative telemetry",
                "ITERATIVE_DO_NOT_TRACK",
                "1"
        ),
        env_tweak!(
                "dvc_analytics",
                "DVC Analytics",
                "Disables DVC analytics",
                "DVC_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "balena_analytics",
                "Balena Analytics",
                "Disables Balena analytics",
                "BALENARC_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "dozzle_analytics",
                "Dozzle Analytics",
                "Disables Dozzle analytics",
                "DOZZLE_NO_ANALYTICS",
                "1"
        ),
        env_tweak!(
                "riff_telemetry",
                "Riff Telemetry",
                "Disables Riff telemetry",
                "RIFF_DISABLE_TELEMETRY",
                "true"
        ),
        env_tweak!(
                "vistrails_usage_stats",
                "VisTrails Stats",
                "Disables VisTrails usage stats",
                "VISTRAILS_USAGE_STATS",
                "off"
        ),
        env_tweak!(
                "momentum_telemetry",
                "Momentum Telemetry",
                "Disables Momentum telemetry",
                "MOMENTUM_TELEMETRY_LEVEL",
                "silent"
        ),
        // === Claude Code ===
        env_tweak!(
                "claude_nonessential_calls",
                "Claude Non-essential Calls",
                "Disables Claude non-essential model calls",
                "DISABLE_NON_ESSENTIAL_MODEL_CALLS",
                "1"
        ),
        env_tweak!(
                "claude_feedback_survey",
                "Claude Feedback Survey",
                "Disables Claude Code feedback survey",
                "CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY",
                "1"
        ),
        env_tweak!(
                "claude_nonessential_traffic",
                "Claude Non-essential Traffic",
                "Disables Claude Code non-essential traffic",
                "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC",
                "1"
        ),
        env_tweak!(
                "claude_error_reporting",
                "Claude Error Reporting",
                "Disables Claude error reporting",
                "DISABLE_ERROR_REPORTING",
                "1"
        ),
        env_tweak!(
                "claude_tool_search",
                "Claude Tool Search",
                "Enables Claude tool search",
                "ENABLE_TOOL_SEARCH",
                "1"
        ),
        // === Generic / Cross-Platform ===
        // NOTE: Do not set "CI", it will break numerous programs including gemini-cli.
        env_tweak!(
                "disable_telemetry_generic",
                "Disable Telemetry (Generic)",
                "Sets generic DISABLE_TELEMETRY flag",
                "DISABLE_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "disabletelemetry_generic",
                "DisableTelemetry (Generic)",
                "Sets generic DISABLETELEMETRY flag",
                "DISABLETELEMETRY",
                "true"
        ),
        env_tweak!(
                "no_telemetry_generic",
                "No Telemetry (Generic)",
                "Sets generic NO_TELEMETRY flag",
                "NO_TELEMETRY",
                "1"
        ),
        env_tweak!(
                "telemetry_disabled_generic",
                "Telemetry Disabled (Generic)",
                "Sets generic TELEMETRY_DISABLED flag",
                "TELEMETRY_DISABLED",
                "1"
        ),
        env_tweak!(
                "collect_telemetry_generic",
                "Collect Telemetry (Generic)",
                "Disables generic COLLECT_TELEMETRY flag",
                "COLLECT_TELEMETRY",
                "0"
        ),
        env_tweak!(
                "allow_ui_analytics_generic",
                "Allow UI Analytics (Generic)",
                "Disables generic ALLOW_UI_ANALYTICS flag",
                "ALLOW_UI_ANALYTICS",
                "false"
        ),
        env_tweak!(
                "disable_analytics_generic",
                "Disable Analytics (Generic)",
                "Sets generic DISABLE_ANALYTICS flag",
                "DISABLE_ANALYTICS",
                "true"
        ),
        env_tweak!(
                "analytics_no_generic",
                "Analytics No (Generic)",
                "Disables generic ANALYTICS flag",
                "ANALYTICS",
                "no"
        ),
        // === Homebrew Extra ===
        env_tweak!(
                "homebrew_analytics_run",
                "Homebrew Analytics This Run",
                "Disables Homebrew analytics for this run",
                "HOMEBREW_NO_ANALYTICS_THIS_RUN",
                "1"
        ),
        env_tweak!(
                "appcd_telemetry_false",
                "Appcd Telemetry (False)",
                "Disables Appcd telemetry (false variant)",
                "APPCD_TELEMETRY",
                "false"
        ),
        env_tweak!(
                "canvas_lms_stats_collection",
                "Canvas LMS Stats",
                "Opt-out of Canvas LMS stats collection",
                "CANVAS_LMS_STATS_COLLECTION",
                "opt_out"
        ),
        env_tweak!(
                "mm_servicesettings_securityfixalert",
                "MM Service Settings Security Fix Alert",
                "Disables MM Service Settings Security Fix Alert",
                "MM_SERVICESETTINGS_ENABLESECURITYFIXALERT",
                "false"
        ),
        env_tweak!(
                "automatedlab_telemetry_optin",
                "AutomatedLab Telemetry Opt-In",
                "Disables AutomatedLab telemetry opt-in",
                "AUTOMATEDLAB_TELEMETRY_OPTIN",
                "0"
        ),
        env_tweak!(
                "disable_checkpoint_signature",
                "Disable Checkpoint Signature",
                "Disables Checkpoint Signature",
                "disable_checkpoint_signature",
                "true"
        ),
];
