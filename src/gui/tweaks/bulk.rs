use crate::gui::shared_state::WorkerContext;
use crate::gui::tweaks::{RegistryOp, Tweak};
use anyhow::Result;
use std::sync::Arc;

pub static BULK_OPERATIONS_TWEAKS: &[Tweak] = &[
        crate::tweak! {
            id: "bulk_telemetry",
            category: "bulk",
            name: "Block Telemetry & Tracking",
            description: "Disables Windows & third-party telemetry, diagnostic data collection, and tracking services.",
            enabled_ops: TELEMETRY_OPS,
            custom_apply: Some(apply_telemetry_tweak),
        },
        crate::tweak! {
            id: "bulk_repair",
            category: "bulk",
            name: "Repair system from bad tweaks",
            description: "Restores default Windows behavior for services, memory management, and networking. Fixes common issues caused by bad 'optimizations'.",
            enabled_ops: REPAIR_OPS,
            custom_apply: Some(apply_repair_tweak),
        },
];

fn apply_telemetry_tweak(ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status("Blocking Telemetry & Tracking...");
        for op in TELEMETRY_OPS {
                super::execute_registry_op(op, ctx, "Telemetry")?;
        }
        disable_telemetry_services(ctx)?;
        disable_telemetry_scheduled_tasks(ctx)?;
        Ok(())
}

fn apply_repair_tweak(ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status("Applying System Repairs...");
        for op in REPAIR_OPS {
                super::execute_registry_op(op, ctx, "Repair")?;
        }
        apply_repair_custom_logic(ctx)?;
        Ok(())
}

/// Stops and disables known telemetry tracking services
fn disable_telemetry_services(ctx: &Arc<WorkerContext>) -> Result<()>
{
        let services = [
                "DiagTrack",
                "dmwappushservice",
                "diagnosticshub.standardcollector.service",
        ];
        for service in services {
                ctx.post_status(&format!("Disabling service: {}", service));
                let _ = crate::common::run_system_command("sc.exe", &["config", service, "start=", "disabled"]);
                let _ = crate::common::run_system_command("sc.exe", &["stop", service]);
                // We don't report progress here to avoid skewing the main progress bar too much,
                // or we could if we want to be granular.
        }
        Ok(())
}

fn disable_telemetry_scheduled_tasks(ctx: &Arc<WorkerContext>) -> Result<()>
{
        let tasks = [
                r"\Microsoft\Windows\Customer Experience Improvement Program\Consolidator",
                r"\Microsoft\Windows\Customer Experience Improvement Program\UsbCeip",
                r"\Microsoft\Windows\Application Experience\Microsoft Compatibility Appraiser",
                r"\Microsoft\Windows\Application Experience\ProgramDataUpdater",
                r"\Microsoft\Windows\Autochk\Proxy",
                r"\Microsoft\Windows\DiskDiagnostic\Microsoft-Windows-DiskDiagnosticDataCollector",
                r"\Microsoft\Windows\Maintenance\WinSAT",
        ];
        for task in tasks {
                ctx.post_status(&format!("Disabling task: {}", task));
                let _ = crate::common::run_system_command("schtasks", &["/Change", "/TN", task, "/Disable"]);
                // We don't report progress here to avoid skewing the main progress bar too much
        }
        Ok(())
}

/// Applies custom repair logic
fn apply_repair_custom_logic(ctx: &Arc<WorkerContext>) -> Result<()>
{
        // 1. Remove Timer Resolution Tools
        ctx.post_status("Cleaning up timer resolution tools...");
        let timer_script = r#"
            # Stop and remove TimerResolution
            Get-Process -Name TimerResolution -ErrorAction SilentlyContinue | Stop-Process -Force
            # Clean up known paths if possible, but mainly stop processes
            
            # Stop and remove ISLC
            Get-Process -Name "Intelligent standby list cleaner ISLC" -ErrorAction SilentlyContinue | Stop-Process -Force

            # Stop and remove Set Timer Resolution Service
            $service = Get-Service -Name "Set Timer Resolution Service", "STR" -ErrorAction SilentlyContinue
            if ($service) {
                Stop-Service -Name $service.Name -Force -ErrorAction SilentlyContinue
                sc.exe delete $service.Name *>$null
            }
            Get-Process -Name SetTimerResolutionService -ErrorAction SilentlyContinue | Stop-Process -Force
        "#;
        let _ = crate::common::run_powershell_command(timer_script);

        // 2. Reset TCP Auto-Tuning
        ctx.post_status("Resetting TCP Auto-Tuning...");
        let _ = crate::common::run_system_command(
                "netsh",
                &["interface", "tcp", "set", "global", "autotuninglevel=normal"],
        );

        // 3. Enable SysMain (Superfetch)
        ctx.post_status("Enabling SysMain service...");
        let _ = crate::common::run_system_command("sc", &["config", "SysMain", "start=", "auto"]);
        let _ = crate::common::run_system_command("sc", &["start", "SysMain"]);

        // 4. Enable HPET
        ctx.post_status("Enabling HPET device...");
        let hpet_cmd = r#"Get-PnpDevice -FriendlyName 'High precision event timer' -ErrorAction SilentlyContinue | Enable-PnpDevice -Confirm:$false"#;
        let _ = crate::common::run_powershell_command(hpet_cmd);

        // 5. Remove CI Environment Variable
        ctx.post_status("Removing CI environment variable...");
        let ci_script = r#"
            [Environment]::SetEnvironmentVariable("CI", $null, "User")
            [Environment]::SetEnvironmentVariable("CI", $null, "Machine")
        "#;
        let _ = crate::common::run_powershell_command(ci_script);

        // 6. Enable Memory Compression
        ctx.post_status("Enabling Memory Compression...");
        let _ = crate::common::run_powershell_command("Enable-MMAgent -mc -ErrorAction SilentlyContinue");

        // 7. Reset Network Adapter Parameters (TcpAckFrequency, TCPNoDelay)
        ctx.post_status("Resetting Network Adapter Parameters...");
        let net_script = r#"
            $adapters = Get-NetAdapter -Physical | Get-NetAdapterID
            foreach ($id in $adapters.GUID) {
                Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\$id" -Name "TcpAckFrequency" -ErrorAction SilentlyContinue
                Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\$id" -Name "TCPNoDelay" -ErrorAction SilentlyContinue
            }
        "#;
        let _ = crate::common::run_powershell_command(net_script);

        Ok(())
}

static TELEMETRY_OPS: &[RegistryOp] = &[
        // Disables usage tracking in Windows Media Player.
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\MediaPlayer\Preferences",
                "UsageTracking",
                0,
                RegistryValue::Delete
        ),
        // --- From privacy.rs ---
        // Disables Windows telemetry and diagnostic data collection
        // 1. AllowTelemetry (Policy) - Set to 0 (Security Level) to disable telemetry
        // 2. LimitDiagnosticLogCollection - Limit logs sent to MS
        // 3. DisableOneSettingsDownloads - Prevent downloading settings from OneSettings service
        // 4. DataCollection/AllowTelemetry - Set to Security (0) or Basic (1) depending on edition, we force 0
        // 5. EnableDeviceHealthAttestationService - Disable health reporting
        // 6. MiscPolicyInfo - Reserve manager policy
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "AllowTelemetry",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "LimitDiagnosticLogCollection",
                1
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "DisableOneSettingsDownloads",
                1
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                "AllowTelemetry",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\devicehealthattestationservice",
                "EnableDeviceHealthAttestationService",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\ReserveManager",
                "MiscPolicyInfo",
                2
        ),
        // Disables tailored experiences (ads/recommendations based on usage)
        // 1. TailoredExperiencesWithDiagnosticDataEnabled - Disable in Privacy key
        // 2. DisableTailoredExperiencesWithDiagnosticData - Disable via Policy
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
                0
        ),
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Privacy",
                "TailoredExperiencesWithDiagnosticDataEnabled",
                0
        ),
        crate::reg_dword!(
                "HKCU",
                r"Software\Policies\Microsoft\Windows\CloudContent",
                "DisableTailoredExperiencesWithDiagnosticData",
                1
        ),
        // Disables feedback requests (SIUF - System Initiated User Feedback)
        // 1. NumberOfSIUFInPeriod - Set to 0 to stop feedback prompts
        // 2. PeriodInNanoSeconds - Set to 0 (never)
        // 3. DoNotShowFeedbackNotifications - Disable notifications via Policy
        crate::reg_dword!("HKCU", r"Software\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0),
        crate::reg_dword!("HKCU", r"Software\Microsoft\Siuf\Rules", "PeriodInNanoSeconds", 0),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "DoNotShowFeedbackNotifications",
                1
        ),
        // Disables handwriting data sharing
        // 1. PreventHandwritingDataSharing - Stop sharing handwriting data with MS
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
                "PreventHandwritingDataSharing",
                1
        ),
        // Disables handwriting error reports
        // 1. PreventHandwritingErrorReports - Stop sending error reports for handwriting
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\HandwritingErrorReports",
                "PreventHandwritingErrorReports",
                1
        ),
        // Disables application inventory (tracking of installed apps)
        // 1. DisableInventory - Stop app inventory collection
        // 2. DisableUAR - Disable User Access Logging (UAL) / User App_V Reporting
        // 3. AITEnable - Disable Application Impact Telemetry
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableInventory",
                1
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
                "DisableUAR",
                1
        ),
        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable", 0),
        // Disables Advertising ID
        // 1. Enabled - Disable Advertising ID in HKCU and HKLM
        // 2. DisabledByGroupPolicy - Enforce disable via Policy
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                "Enabled",
                0
        ),
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                "Enabled",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
                "DisabledByGroupPolicy",
                1
        ),
        // Disables Customer Experience Improvement Program (CEIP)
        // 1. CEIPEnable - Set to 0 to disable CEIP
        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
        // --- From third_party_telemetry.rs ---
        // Disables Microsoft Office Telemetry
        // 1. DisableTelemetry - Disable in common client telemetry
        // 2. QMEnable - Disable standard Quality Metrics
        // 3. Feedback/Enabled - Disable feedback
        // 4. SendTelemetry - Set to 3 (Disabled)
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\office\common\clienttelemetry",
                "DisableTelemetry",
                1
        ),
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\office\16.0\common\clienttelemetry",
                "DisableTelemetry",
                1
        ),
        crate::reg_dword!("HKCU", r"Software\Microsoft\office\16.0\common", "QMEnable", 0),
        crate::reg_dword!("HKCU", r"Software\Microsoft\office\16.0\common\feedback", "Enabled", 0),
        // Disables Visual Studio Telemetry
        // 1. TurnOffSwitch - Master switch for VS telemetry
        // 2. OptIn - Opt out of VSCEIP / PerfWatson; VS2017, VS2019, VS2022, VS2025
        crate::reg_dword!("HKCU", r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch", 1),
        crate::reg_dword!("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM", "OptIn", 0),
        crate::reg_dword!("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM", "OptIn", 0),
        crate::reg_dword!("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM", "OptIn", 0),
        crate::reg_dword!("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\18.0\SQM", "OptIn", 0),
        // Disables Firefox Telemetry
        // 1. DisableTelemetry - Disable via Policy (HKLM & HKCU)
        crate::reg_dword!("HKLM", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", 1),
        crate::reg_dword!("HKCU", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry", 1),
        crate::reg_dword!(
                "HKCU",
                r"Software\Policies\Microsoft\office\16.0\common\privacy",
                "SendTelemetry",
                3
        ),
        // Disables NVidia Telemetry
        // 1. OptInOrOutPreference - Set to 0 to opt out
        crate::reg_dword!(
                "HKCU",
                r"Software\NVIDIA Corporation\NVControlPanel2\Client",
                "OptInOrOutPreference",
                0
        ),
        // Disables Developer Tools & CLI Telemetry (Environment Variables)
        // Checks various environment variables used by dev tools to disable analytics.

        // .NET / Microsoft CLI Tools
        crate::reg_str!("HKCU", "Environment", "DOTNET_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "DOTNET_SVCUTIL_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "MLDOTNET_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "MSSQL_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "VSTEST_TELEMETRY_OPTEDIN", "0"),
        crate::reg_str!("HKCU", "Environment", "POWERSHELL_TELEMETRY_OPTOUT", "1"),
        // Azure / Cloud CLI
        crate::reg_str!("HKCU", "Environment", "AZURE_CORE_COLLECT_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "AZURE_DEV_COLLECT_TELEMETRY", "no"),
        crate::reg_str!("HKCU", "Environment", "AZUREML_SDKV2_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "FUNCTIONS_CORE_TOOLS_TELEMETRY_OPTOUT", "1"),
        // JavaScript / Web Frameworks (Next.js, Nuxt, Gatsby, etc.)
        crate::reg_str!("HKCU", "Environment", "NEXT_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "NUXT_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "GATSBY_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "NG_CLI_ANALYTICS", "false"),
        crate::reg_str!("HKCU", "Environment", "NG_CLI_ANALYTICS_SHARE", "false"),
        crate::reg_str!("HKCU", "Environment", "ASTRO_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "STORYBOOK_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "REDWOOD_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "YARN_ENABLE_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "HINT_TELEMETRY", "off"),
        crate::reg_str!("HKCU", "Environment", "VUEDX_TELEMETRY", "off"),
        crate::reg_str!("HKCU", "Environment", "STRAPI_TELEMETRY_DISABLED", "true"),
        crate::reg_str!("HKCU", "Environment", "SLS_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "SLS_TRACKING_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "CALCOM_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "SKU_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "EMBER_CLI_ANALYTICS", "false"),
        crate::reg_str!("HKCU", "Environment", "CAPACITOR_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "CARBON_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "DAGSTER_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "FEAST_TELEMETRY", "False"),
        crate::reg_str!("HKCU", "Environment", "MELTANO_DISABLE_TRACKING", "True"),
        crate::reg_str!("HKCU", "Environment", "RASA_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "HAMILTON_TELEMETRY_ENABLED", "false"),
        // AI / Machine Learning Tools (HuggingFace, Gradio, etc.)
        crate::reg_str!("HKCU", "Environment", "HF_HUB_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "GRADIO_ANALYTICS_ENABLED", "False"),
        crate::reg_str!("HKCU", "Environment", "RAGAS_DO_NOT_TRACK", "1"),
        crate::reg_str!("HKCU", "Environment", "OPENLLM_DO_NOT_TRACK", "1"),
        crate::reg_str!("HKCU", "Environment", "FLWR_TELEMETRY_ENABLED", "0"),
        crate::reg_str!("HKCU", "Environment", "STREAMLIT_TELEMETRY_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "WHYLOGS_NO_ANALYTICS", "true"),
        crate::reg_str!("HKCU", "Environment", "JINA_OPTOUT_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "SCHEMATHESIS_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "DBT_SEND_ANONYMOUS_USAGE_STATS", "false"),
        // Infrastructure / DevOps Tools (Terraform, Vagrant, Checkpoint, etc.)
        crate::reg_str!("HKCU", "Environment", "TERRAFORM_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "CHECKPOINT_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "VAGRANT_CHECKPOINT_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "PACKER_CHECKPOINT_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "CONSUL_CHECKPOINT_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "ARM_DISABLE_TERRAFORM_PARTNER_ID", "true"),
        crate::reg_str!("HKCU", "Environment", "CHEF_TELEMETRY_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "AUTOMATEDLAB_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "NUKE_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "PNPPOWERSHELL_DISABLETELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "EARTHLY_DISABLE_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "WERF_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "SCOUT_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "INFRACOST_SELF_HOSTED_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "BATECT_ENABLE_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "DECK_ANALYTICS", "off"),
        crate::reg_str!("HKCU", "Environment", "DO_NOT_TRACK", "1"),
        crate::reg_str!("HKCU", "Environment", "KICS_COLLECT_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_CRASH_REPORT", "1"),
        crate::reg_str!("HKCU", "Environment", "CIRCLECI_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "CODER_TELEMETRY_ENABLE", "false"),
        crate::reg_str!("HKCU", "Environment", "SAM_CLI_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "CLOUDSDK_CORE_DISABLE_USAGE_REPORTING", "true"),
        crate::reg_str!("HKCU", "Environment", "HOOKDECK_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "STRIPE_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "F5_ALLOW_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "TEEM_DISABLE", "true"),
        crate::reg_str!("HKCU", "Environment", "MSLAB_TELEMETRY_LEVEL", "None"),
        crate::reg_str!("HKCU", "Environment", "INFLUXD_REPORTING_DISABLED", "true"),
        crate::reg_str!("HKCU", "Environment", "QUILT_DISABLE_USAGE_METRICS", "True"),
        crate::reg_str!("HKCU", "Environment", "QDRANT__TELEMETRY_DISABLED", "true"),
        crate::reg_str!("HKCU", "Environment", "MONGODB_ATLAS_TELEMETRY_ENABLE", "false"),
        crate::reg_str!("HKCU", "Environment", "FERRETDB_TELEMETRY", "disable"),
        crate::reg_str!("HKCU", "Environment", "CUBESTORE_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "CUBEJS_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "EVENTSTORE_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "HOMEBREW_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "CHOOSENIM_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "COCOAPODS_DISABLE_STATS", "true"),
        crate::reg_str!("HKCU", "Environment", "ARDUINO_METRICS_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "ROCKSET_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "APOLLO_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "SFDX_DISABLE_TELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "SF_DISABLE_TELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "SALTO_TELEMETRY_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "BF_CLI_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "MOBILE_CENTER_TELEMETRY", "off"),
        crate::reg_str!("HKCU", "Environment", "APPCD_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "TUIST_STATS_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "GOTELEMETRY", "off"),
        crate::reg_str!("HKCU", "Environment", "RAY_USAGE_STATS_ENABLED", "0"),
        crate::reg_str!("HKCU", "Environment", "APTOS_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "SPEEDSTER_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_DEEPCHECKS_ANONYMOUS_TELEMETRY", "True"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_DCS_ANONYMOUS_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DACFX_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "REACT_APP_WEBINY_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "PRISMA_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "ORYX_DISABLE_TELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "SQA_OPT_OUT", "true"),
        crate::reg_str!("HKCU", "Environment", "HASURA_GRAPHQL_ENABLE_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "MEILI_NO_ANALYTICS", "true"),
        crate::reg_str!("HKCU", "Environment", "NOCODB_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "NC_DISABLE_TELE", "1"),
        crate::reg_str!("HKCU", "Environment", "PROSE_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "RESTLER_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "PROJECTOR_TELEMETRY_ENABLED", "0"),
        crate::reg_str!("HKCU", "Environment", "MEDUSA_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "TELEMETRY_ENABLED", "0"),
        crate::reg_str!("HKCU", "Environment", "ALIBUILD_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "FASTLANE_OPT_OUT_USAGE", "YES"),
        crate::reg_str!("HKCU", "Environment", "COVERITY_CLI_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "GRADLE_ENTERPRISE_ANALYTICS_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "LOST_PIXEL_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DOCKER_SCAN_SUGGEST", "false"),
        crate::reg_str!("HKCU", "Environment", "KUBEAPT_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DASH_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DAGGER_TELEMETRY_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "NEONKUBE_DISABLE_TELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "OTTERIZE_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "PORTER_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "PREEVY_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "REPORTPORTAL_CLIENT_JS_NO_ANALYTICS", "true"),
        crate::reg_str!("HKCU", "Environment", "AGENT_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "BUGGER_OFF", "1"),
        crate::reg_str!("HKCU", "Environment", "SUGGESTIONS_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "DA_TEST_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "ET_NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "LYNX_ANALYTICS", "0"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_QUICKWIT_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "AUTOMAGICA_NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "NETDATA_ANONYMOUS_STATISTICS", "0"),
        crate::reg_str!("HKCU", "Environment", "TILT_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "MM_LOGSETTINGS_ENABLEDIAGNOSTICS", "false"),
        crate::reg_str!("HKCU", "Environment", "LS_METRICS_HOST_ENABLED", "0"),
        crate::reg_str!("HKCU", "Environment", "PANTS_ANONYMOUS_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "FLAGSMITH_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "ONE_CODEX_NO_TELEMETRY", "True"),
        crate::reg_str!("HKCU", "Environment", "AITOOLSVSCODE_DISABLETELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "IG_PRO_OPT_OUT", "YES"),
        crate::reg_str!("HKCU", "Environment", "REDOCLY_TELEMETRY", "off"),
        crate::reg_str!("HKCU", "Environment", "HARDHAT_DISABLE_TELEMETRY_PROMPT", "1"),
        crate::reg_str!("HKCU", "Environment", "TAOS_TELEMETRY_REPORTING", "0"),
        crate::reg_str!("HKCU", "Environment", "MF_SEND_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "TELEPORT_ANONYMOUS_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "TUNNELMOLE_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "WG_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "ANYCABLE_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "RIG_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "ELECTRIC_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "SNOWFLAKE_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "OASDIFF_NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "EMQX_TELEMETRY__ENABLE", "false"),
        crate::reg_str!("HKCU", "Environment", "KARATE_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "CONJUR_TELEMETRY_ENABLED", "false"),
        crate::reg_str!(
                "HKCU",
                "Environment",
                "CONJUR_FEATURE_TELEMETRY_ENDPOINT_ENABLED",
                "false"
        ),
        crate::reg_str!("HKCU", "Environment", "TAIGA_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "FA_NOTRACK", "true"),
        crate::reg_str!("HKCU", "Environment", "INFISICAL_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "BOXYHQ_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "ADSERVER_DO_NOT_TRACK", "1"),
        crate::reg_str!("HKCU", "Environment", "UPSTASH_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "BITRISE_ANALYTICS_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "AKITA_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "REDGATE_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "FLAKE_CHECKER_NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "PP_TOOLS_TELEMETRY_OPTOUT", "1"),
        crate::reg_str!("HKCU", "Environment", "PIPELINES_TELEMETRY_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "PATCHER_TELEMETRY_OPT_OUT", "1"),
        crate::reg_str!("HKCU", "Environment", "RETRACED_NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "KEYSTONE_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "STP_DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "USAGE_DISABLE", "1"),
        crate::reg_str!("HKCU", "Environment", "ANALYTICS_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "ORBIT_TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "FAL_STATS_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "LEON_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "AP_TELEMETRY_ENABLED", "false"),
        crate::reg_str!("HKCU", "Environment", "ITERATIVE_DO_NOT_TRACK", "1"),
        crate::reg_str!("HKCU", "Environment", "DVC_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "BALENARC_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "DOZZLE_NO_ANALYTICS", "1"),
        crate::reg_str!("HKCU", "Environment", "RIFF_DISABLE_TELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "VISTRAILS_USAGE_STATS", "off"),
        crate::reg_str!("HKCU", "Environment", "MOMENTUM_TELEMETRY_LEVEL", "silent"),
        // Generic / Miscellaneous
        crate::reg_str!("HKCU", "Environment", "DISABLE_NON_ESSENTIAL_MODEL_CALLS", "1"),
        crate::reg_str!("HKCU", "Environment", "CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY", "1"),
        crate::reg_str!("HKCU", "Environment", "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC", "1"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_ERROR_REPORTING", "1"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "DISABLETELEMETRY", "true"),
        crate::reg_str!("HKCU", "Environment", "NO_TELEMETRY", "1"),
        crate::reg_str!("HKCU", "Environment", "TELEMETRY_DISABLED", "1"),
        crate::reg_str!("HKCU", "Environment", "COLLECT_TELEMETRY", "0"),
        crate::reg_str!("HKCU", "Environment", "ALLOW_UI_ANALYTICS", "false"),
        crate::reg_str!("HKCU", "Environment", "DISABLE_ANALYTICS", "true"),
        crate::reg_str!("HKCU", "Environment", "ANALYTICS", "no"),
        crate::reg_str!("HKCU", "Environment", "HOMEBREW_NO_ANALYTICS_THIS_RUN", "1"),
        crate::reg_str!("HKCU", "Environment", "APPCD_TELEMETRY", "false"),
        crate::reg_str!("HKCU", "Environment", "CANVAS_LMS_STATS_COLLECTION", "opt_out"),
        crate::reg_str!(
                "HKCU",
                "Environment",
                "MM_SERVICESETTINGS_ENABLESECURITYFIXALERT",
                "false"
        ),
        crate::reg_str!("HKCU", "Environment", "AUTOMATEDLAB_TELEMETRY_OPTIN", "0"),
        crate::reg_str!("HKCU", "Environment", "disable_checkpoint_signature", "true"),
        // Disable App-V CEIP
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\AppV\CEIP",
                "CEIPEnable",
                0,
                RegistryValue::Delete
        ),
        // Disable PC Health Reporting
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "AllOrNone",
                1,
                RegistryValue::Delete
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "IncludeKernelFaults",
                0,
                RegistryValue::Delete
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "IncludeMicrosoftApps",
                0,
                RegistryValue::Delete
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "IncludeShutdownErrs",
                0,
                RegistryValue::Delete
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "IncludeWindowsApps",
                0,
                RegistryValue::Delete
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting",
                "DoReport",
                0,
                RegistryValue::Delete
        ),
];

static REPAIR_OPS: &[RegistryOp] = &[
        // Repair Service Split Threshold
        // Restores default service hosting behavior. Fixes potential stability issues.
        crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control", "SvcHostSplitThresholdInKB"),
        crate::reg_del!("HKLM", r"SYSTEM\ControlSet001\Control", "SvcHostSplitThresholdInKB"),
        // Reset Processor Scheduling (Win32PrioritySeparation)
        // Deletes the key so Windows uses its default value (2 or 26 depending on edition).
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\PriorityControl",
                "Win32PrioritySeparation"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\ControlSet001\Control\PriorityControl",
                "Win32PrioritySeparation"
        ),
        // Repair DPC Settings
        // Removes regular/ordinary DPC overrides.
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
                "ThreadDpcEnable"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\ControlSet001\Control\Session Manager\kernel",
                "ThreadDpcEnable"
        ),
        // Restore CPU Security Mitigations (Spectre/Meltdown)
        // Re-enables mitigations.
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "FeatureSettingsOverride"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "FeatureSettingsOverrideMask"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management",
                "FeatureSettingsOverride"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management",
                "FeatureSettingsOverrideMask"
        ),
        // Reset Input Queue Sizes
        // Resets mouse and keyboard data queue sizes to default (100).
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Services\kbdclass\Parameters",
                "KeyboardDataQueueSize",
                100
        ),
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\ControlSet001\Services\kbdclass\Parameters",
                "KeyboardDataQueueSize",
                100
        ),
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Services\mouclass\Parameters",
                "MouseDataQueueSize",
                100
        ),
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\ControlSet001\Services\mouclass\Parameters",
                "MouseDataQueueSize",
                100
        ),
        // Reset CSRSS Priority
        // Removes custom performance options for csrss.exe.
        crate::reg_del_key!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\csrss.exe",
                "PerfOptions"
        ),
        // Reset Multi-Plane Overlay (MPO)
        // Restores default MPO behavior by deleting OverlayTestMode.
        crate::reg_del!("HKLM", r"SOFTWARE\Microsoft\Windows\Dwm", "OverlayTestMode"),
        // Reset Memory Management
        // Resets various memory management tweaks to defaults (Delete = Default).
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "ClearPageFileAtShutdown"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "DisablePagingExecutive"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "LargeSystemCache"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "NonPagedPoolQuota"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "NonPagedPoolSize"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "PagedPoolQuota"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "PagedPoolSize"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SecondLevelDataCache"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SystemPages"
        ),
        // Delete potentially harmful keys
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "IoPageLockLimit"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "CacheUnmapBehindLengthInMB"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SimulateCommitSavings"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "TrackLockedPages"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "TrackPtes"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "DynamicMemory"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "EnforceWriteProtection"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "MakeLowMemory"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SystemCacheLimit"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SessionSpaceLimit"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "WriteWatch"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "SnapUnloads"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "MapAllocationFragment"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "Mirroring"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "DontVerifyRandomDrivers"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "EnableLowVaAccess"
        ),
        // Remove Raw Mouse Throttle Tweaks
        crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleEnabled"),
        crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleForced"),
        crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleDuration"),
        crate::reg_del!("HKCU", r"Control Panel\Mouse", "RawMouseThrottleLeeway"),
        // Enable Prefetcher (Set to 3: App launch and Boot enabled)
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
                "EnablePrefetcher",
                3
        ),
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\ControlSet001\Control\Session Manager\Memory Management\PrefetchParameters",
                "EnablePrefetcher",
                3
        ),
        // Remove NVMe Driver Hacks
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides",
                "735209102"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides",
                "1853569164"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides",
                "156965516"
        ),
        // Enable AppX Deployment Service
        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\AppXSvc", "Start", 3), // 3 = Manual (Trigger Start)
        // Enable User Account Control (UAC)
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "EnableLUA",
                1
        ),
        // Enable Background Apps Globally
        crate::reg_dword!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications",
                "GlobalUserDisabled",
                0
        ),
        // Enable Tablet Input Service
        crate::reg_dword!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Services\TabletInputService",
                "Start",
                3
        ),
        // Enable Windows Error Reporting
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "Disabled",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "DontSendAdditionalData",
                1
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "DontShowUI",
                0
        ),
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\Windows Error Reporting",
                "LoggingDisabled",
                0
        ),
        crate::reg_dword!("HKLM", r"Software\Microsoft\pchealth\errorreporting", "DoReport", 1),
        // Fix Storage & Drivers
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control\StorPort",
                "HmbAllocationPolicy"
        ),
        crate::reg_del!("HKLM", r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "TdrLevel"),
        // Fix Services & Network
        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\Dnscache", "Start", 2), // Auto
        crate::reg_str!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Control",
                "WaitToKillServiceTimeout",
                "5000"
        ),
        crate::reg_del!(
                "HKLM",
                r"SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters",
                "DisabledComponents"
        ),
        crate::reg_del!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
                "NetworkThrottlingIndex"
        ),
        // Fix UI & Shell (Windows 11 Compatibility)
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "TaskbarSi"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "TaskbarSmallIcons"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Streams\Desktop",
                "TaskbarWinXP"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People",
                "PeopleBand"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\StartPage",
                "MakeAllAppsDefault"
        ),
        crate::reg_del!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "ShowMoreTiles"),
        crate::reg_del!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
                "StartLayoutFile"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Policies\Microsoft\Windows\CurrentVersion\PushNotifications",
                "NoTileApplicationNotification"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\ImmersiveShell",
                "TabletMode"
        ),
        crate::reg_del!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "EnableActivityFeed"
        ),
        crate::reg_del!(
                "HKLM",
                r"Software\Microsoft\Windows NT\CurrentVersion\MTCUVC",
                "EnableMtcUvc"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Ribbon",
                "MinimizedStateTabletModeOff"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Policies\Microsoft\Windows\Explorer",
                "DisableNotificationCenter"
        ),
        crate::reg_del!("HKCU", r"Control Panel\Desktop", "AutoEndTasks"),
        crate::reg_str!("HKCU", r"Control Panel\Desktop", "MenuShowDelay", "400"),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize",
                "StartupDelayInMSec"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowSecondsInSystemClock"
        ),
        crate::reg_del!(
                "HKCU",
                r"Software\Microsoft\Windows\CurrentVersion\Explorer",
                "AlwaysUnloadDll"
        ),
        crate::reg_del!(
                "HKLM",
                r"SOFTWARE\Policies\Microsoft\Windows\Psched",
                "NonBestEffortLimit"
        ),
        // Fix BAD_TWEAKS: Security & Misc
        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Services\EventLog", "Start", 2), // Auto
        crate::reg_dword!(
                "HKLM",
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "ConsentPromptBehaviorAdmin",
                5
        ), // Prompt for non-Windows binaries
];
