// Performance tweaks

use super::{Tweak, TweakEffect};
use winsafe;

pub static QOS_GAMES_LIST: &str = "100orange.exe;100orange_x86.exe;7DaysToDie.exe;AbioticFactor.exe;AC3MP.exe;AC4BFMP.exe;ACBMP.exe;ACC.exe;\
    Ace7Game.exe;ACRMP.exe;ACU.exe;AgeOfEmpires4.exe;Among Us.exe;AMS2.exe;AMS2AVX.exe;Anno1800.exe;Anno1800_plus.exe;\
    AOCClient-Win64-Shipping.exe;ApexLegends.exe;ArkAscended.exe;arma3.exe;arma3_x64.exe;arma3battleye.exe;\
    armoredcore6.exe;AssettoCorsa.exe;AvP.exe;AvP_Classic.exe;AvP_DX11.exe;Back4Blood.exe;BaldursGate3.exe;\
    BattleBit.exe;BattleBitEAC.exe;BattleChess.exe;BattleChess-Win64-Shipping.exe;BattlefieldV.exe;Battlerite.exe;\
    BeamNG.drive.x64.exe;beastieball.exe;Beat Saber.exe;bf1.exe;bf1Trial.exe;bf2042.exe;bf3.exe;bf4.exe;bfh.exe;bfv.exe;\
    bg3.exe;bg3_dx11.exe;Bioshock2.exe;blackdesert64.exe;BlackOps.exe;BlackOps3.exe;BloonsTD6.exe;Borderlands.exe;\
    Borderlands2.exe;Borderlands3.exe;BorderlandsGOTY.exe;BorderlandsPreSequel.exe;Brawlhalla.exe;btdb2_game.exe;\
    burnoutparadise.exe;Call of Duty.exe;call_to_arms.exe;call_to_arms_server.exe;Chaser.exe;\
    Chivalry2-Win64-Shipping.exe;CoD2MP_s.exe;CoDMP.exe;CoDUOMP.exe;CoDWaWmp.exe;Company of Heroes 2.exe;\
    ConanSandbox.exe;Content Warning.exe;Counter-Strike 2.exe;CrabChampions.exe;cs2.exe;csgo.exe;DarkSoulsII.exe;\
    DarkSoulsIII.exe;DarkSoulsRemastered.exe;DayZ_BE.exe;DayZ_x64.exe;DayZLauncher.exe;DBXV2.exe;DeadByDaylight.exe;\
    DeadByDaylight-Win64-Shipping.exe;DeadIsland.exe;DeadIsland2.exe;deadlock.exe;deadspace2.exe;deadspace3.exe;\
    Deceit.exe;DeepRockGalactic.exe;Demonologist.exe;Destiny2.exe;DeusEx.exe;Diablo III64.exe;Diablo IV.exe;\
    Discovery.exe;DontStarveTogether.exe;DOOMEternalx64vk.exe;DOOMx64vk.exe;dota2.exe;DragonAgeInquisition.exe;\
    duke3d.exe;DungeonSiege.exe;DungeonSiege2.exe;Dungeon Siege III.exe;DyingLightGame.exe;DyingLight2.exe;EDF5.exe;\
    eldenring.exe;EliteDangerous64.exe;enshrouded.exe;EoCApp.exe;EscapeFromTarkov.exe;eso64.exe;eu4.exe;eurotrucks2.exe;\
    EvilDead.exe;EvilDead-Win64-Shipping.exe;F1_24.exe;FactoryGame.exe;FactoryGame-Win64-Shipping.exe;Fallout76.exe;\
    FallGuys_client_game.exe;FC24.exe;FEAR2.exe;F.E.A.R. 3.exe;FEARMP.exe;fellowship-Win64-Shipping.exe;ffxiv_dx11.exe;\
    FIFA23.exe;ForHonor.exe;FortniteClient-Win64-Shipping.exe;ForzaHorizon4.exe;ForzaHorizon5.exe;FSD.exe;\
    FSD-Win64-Shipping.exe;Gang Beasts.exe;GBVS.exe;GBVS-Win64-Shipping.exe;GenshinImpact.exe;GhostOfTsushima.exe;\
    gmod.exe;GRAW.exe;graw2.exe;graw2_dedicated.exe;Grim Dawn.exe;GTA5.exe;GTAIV.exe;GuiltyGearStrive.exe;\
    Gunfire Reborn.exe;Gw2-64.exe;HaloInfinite.exe;Hearthstone.exe;helldivers2.exe;HellLetLoose.exe;\
    HLL-Win64-Shipping.exe;hl2.exe;HoboRPG.exe;hoi4.exe;HouseOfAshes.exe;HumanFallFlat.exe;HuntGame.exe;HW2.exe;\
    HW2-Win64-Shipping.exe;il2fb.exe;Insurgency.exe;InsurgencyClient-Win64-Shipping.exe;InsurgencyEAC.exe;\
    IntrepidStudiosLauncher.exe;iRacing.exe;ItTakesTwo.exe;iw3mp.exe;iw4mp.exe;Jantama_MahjongSoul.exe;KFGame.exe;\
    kingpin.exe;KingOfFighters2002UM.exe;KingOfFighters2002UM_x64.exe;KingOfFighters98UM.exe;kofxiv.exe;kofxiii.exe;\
    KOFXV_Steam.exe;LastBlade.exe;LastBlade2App.exe;League of Legends.exe;left4dead.exe;left4dead2.exe;LEGOBatman.exe;\
    LEGOBatman2.exe;LEGOBatman3.exe;LEGOBatman3_DX11.exe;LEGOCloneWars.exe;LEGO DC Super-villains_DX11.exe;\
    LEGOEMMET.exe;LEGOHarryPotter.exe;LEGOHobbit.exe;LEGOHobbit_DX11.exe;LEGOIndy.exe;LEGOIndy2.exe;\
    LEGOJurassicWorld.exe;LEGOJurassicWorld_DX11.exe;LEGOLCUR_DX11.exe;LEGOLOTR.exe;LEGOMARVEL.exe;\
    LEGOMARVELAvengers.exe;LEGOMARVELAvengers_DX11.exe;LEGOMARVEL2.exe;LEGOMARVEL2_DX11.exe;LEGONINJAGO.exe;\
    LEGONINJAGO_DX11.exe;LEGOPirates.exe;LEGOStarWarsSaga.exe;LEGO The Incredibles.exe;LEGO The Incredibles_DX11.exe;\
    LEGO The LEGO Movie 2_DX11.exe;LEGOSWTFA.exe;LEGOSWTFA_DX11.exe;LEGO_Worlds.exe;LEGO_Worlds_DX11.exe;\
    Lethal Company.exe;LittleHope.exe;LOTF2.exe;LOTF2-Win64-Shipping.exe;Madden24.exe;Magicka.exe;Magicka2.exe;\
    Maine-Win64-Shipping.exe;ManOfMedan.exe;Marvel.exe;Marvel-Win64-Shipping.exe;MarvelRivals_Launcher.exe;\
    MassEffect3.exe;MaxPayne3.exe;MCC-Win64-Shipping.exe;mcclauncher.exe;Minecraft.exe;MK11.exe;MonsterHunterRise.exe;\
    MonsterHunterWilds.exe;MonsterHunterWorld.exe;MortalKombat1.exe;MultiVersus.exe;MWOClient.exe;NarakaBladepoint.exe;\
    NBA2K24.exe;NewWorld.exe;NMS.exe;OmegaStrikers.exe;Outward.exe;Overcooked2.exe;Overwatch.exe;Painkiller.exe;\
    Palworld.exe;Palworld-Win64-Shipping.exe;PartyAnimals.exe;PathOfExile.exe;PathOfExile_x64.exe;\
    PAYDAY2_win32_release.exe;PAYDAY3Client.exe;PAYDAY3Client-Win64-Shipping.exe;pes2013.exe;PES6.exe;Phasmophobia.exe;\
    portal2.exe;PortalWars-Win64-Shipping.exe;ProjectZomboid64.exe;pso2.exe;PUBG.exe;QuakeChampions.exe;RabbitSteel.exe;\
    r5apex.exe;r5apex_dx12.exe;RaccoonCity.exe;Raft.exe;RainWorld.exe;RainbowSix.exe;RDR2.exe;re5dx9.exe;re6.exe;\
    ReadyOrNot.exe;RedFactionArmageddon.exe;RedFactionArmageddon_DX11.exe;RelicCoH2.exe;Remnant.exe;\
    Remnant-Win64-Shipping.exe;Remnant2.exe;Remnant2-Win64-Shipping.exe;rerev.exe;rerev2.exe;RF.exe;RF_120na.exe;\
    rf2.exe;RFG.exe;Risk of Rain 2.exe;RobloxPlayerBeta.exe;RocketLeague.exe;RogueTrooper.exe;R-Type_Dimensions.exe;\
    Rune.exe;RustClient.exe;SaintsRowTheThird.exe;SaintsRowTheThird_DX11.exe;Samurai Shodown Collection.exe;SC2_x64.exe;\
    SeaOfThieves.exe;Shogo.exe;sin.exe;SKEstivalVersus.exe;SKShinoviVersus.exe;Smite.exe;sniper5_dx12.exe;\
    sniper5_vulkan.exe;SnowRunner.exe;soldat.exe;soldat2.exe;SolarlandClient.exe;SolarlandClient-Win64-Shipping.exe;\
    SonsOfTheForest.exe;SOPFFO.exe;SoTGame.exe;SoulWorker.exe;squad_launcher.exe;SquadGame.exe;SSVS.exe;Stalker2.exe;\
    Stalker2-Win64-Shipping.exe;StarCitizen.exe;StarCraft.exe;Stardew Valley.exe;starwarsbattlefrontii.exe;\
    starwarsbattlefrontii_trial.exe;StreetFighter6.exe;t6mp.exe;t6zm.exe;TAE.EXE;Tekken8.exe;TemtemSwarm.exe;\
    Terraria.exe;tf_win64.exe;TheDevilInMe.exe;thedivision.exe;TheDivision2.exe;thedivision2launcher.exe;TheFinals.exe;\
    TheForest.exe;TheMatriarch.exe;TheMatriarch-Win64-Shipping.exe;ThroneAndLiberty.exe;Titanfall2.exe;TL.exe;\
    Torchlight2.exe;TotalA.exe;TOTClient.exe;TOTClient-Win64-Shipping.exe;Trackmania.exe;Troy.exe;TslGame.exe;\
    UT2004.exe;Valheim.exe;Valorant-Win64-Shipping.exe;vampire.exe;VampireSurvivors.exe;vermintide2.exe;\
    vermintide2_dx12.exe;victoria3.exe;VRising.exe;VRChat.exe;VTOLVR.exe;Warframe.x64.exe;Warhammer3.exe;WarThunder.exe;\
    Watch_Dogs.exe;WatchDogs2.exe;wgc.exe;Windblown.exe;WL3.exe;WorldOfTanks.exe;WorldOfWarships.exe;Wow.exe;\
    WoWSLauncher.exe;XDefiant.exe;XR_3DA.exe;xrEngine.exe;Youngblood_x64vk.exe;YuanShen.exe";

pub static PERFORMANCE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "disable_animations",
                category: "performance",
                name: "Disable Animations",
                description: "Disables window animations for faster, snappier UI.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_str!("HKCU", r"Control Panel\Desktop\WindowMetrics", "MinAnimate", "0", "0"),
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "TaskbarAnimations", 0, 1),
                ],
                },
        crate::tweak! {
                id: "disable_aero_peek",
                category: "performance",
                name: "Disable Aero Peek",
                description: "Disables the Aero Peek feature (preview desktop when hovering taskbar).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\DWM", "EnableAeroPeek", 0, 1),
                ],
                },
        crate::tweak! {
                id: "disable_power_throttling",
                category: "performance",
                name: "Disable Power Throttling",
                description: "Disables CPU power throttling for background apps. May increase power usage.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff", 1, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_fth",
                category: "performance",
                name: "Disable Fault Tolerant Heap",
                description: "Disables FTH which mitigates application crashes but uses resources.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\FTH", "Enabled", 0, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_scheduled_diagnostics",
                category: "performance",
                name: "Disable Scheduled Diagnostics",
                description: "Prevents automatic system diagnostics from running in the background.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"Software\Microsoft\Windows\ScheduledDiagnostics", "EnabledExecution", 0, 1),
                ],
                },
        crate::tweak! {
                id: "ntfs_nonpaged_pool",
                category: "performance",
                name: "Increase NTFS Memory Usage",
                description: "Increases NTFS memory cache for better disk performance.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsMemoryUsage", 2, 1),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_paging_executive",
                category: "performance",
                name: "Disable Paging Executive",
                description: "Keeps kernel drivers in memory instead of paging them to disk.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive", 1, 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "system_responsiveness",
                category: "performance",
                name: "Optimize System Responsiveness",
                description: "Prioritizes system tasks over multimedia tasks.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "SystemResponsiveness", 0, 20),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "priority_separation",
                category: "performance",
                name: "Optimize Processor Scheduling",
                description: "Optimizes CPU scheduling for best performance of foreground apps.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\PriorityControl", "Win32PrioritySeparation", 38, 2),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_bandwidth_throttling",
                category: "performance",
                name: "Disable Bandwidth Throttling",
                description: "Disables network bandwidth throttling for non-multimedia traffic.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Psched", "NonBestEffortLimit", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_last_access",
                category: "performance",
                name: "Disable NTFS Last Access Update",
                description: "Disables updating the 'Last Accessed' timestamp on files to improve disk performance.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsDisableLastAccessUpdate", 1, 0),
                ],
                                requires_restart: true
        },
        crate::tweak! {
                id: "disable_experimentation",
                category: "performance",
                name: "Disable Windows Experimentation",
                description: "Disables Microsoft experimentation features.",
                effect: TweakEffect::Restart,
                enabled_ops: &[
                        crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\PolicyManager\default\System\AllowExperimentation", "value", 0, RegistryValue::Delete),
                ],
                                requires_restart: true
        },
        crate::tweak! {
            id: "game_bar_tweaks",
            category: "performance",
            name: "Optimize Game Bar",
            description: "Enables Auto Game Mode and disables Game Bar features.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\GameBar", "AutoGameModeEnabled", 1, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\GameBar", "AllowAutoGameMode", 1, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\GameBar", "UseNexusForGameBarEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\GameBar", "ShowStartupPanel", 0, 1),
            ],
                    },
        crate::tweak! {
            id: "directx_swap_effect_upgrade",
            category: "performance",
            name: "Enable DirectX Swap Effect Upgrade",
            description: "Enables DirectX Swap Effect Upgrade for better performance.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\DirectX\GraphicsSettings", "SwapEffectUpgradeCache", 1, RegistryValue::Delete),
            ],
                    },
        crate::tweak! {
            id: "disable_game_dvr_recording",
            category: "performance",
            name: "Disable Game DVR Background Recording",
            description: "Disables background recording of gameplay (GameDVR) to save resources.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "HistoricalCaptureEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 0, 1),
                crate::reg_dword!("HKCU", r"System\GameConfigStore", "GameDVR_Enabled", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\GameDVR", "AllowGameDVR", 0, RegistryValue::Delete),
            ],
                    },
        crate::tweak! {
            id: "enable_qos_dscp_marking",
            category: "performance",
            name: "Enable QoS DSCP Marking & Priority",
            description: "Enables QoS packet tagging and sets High Priority (DSCP 46) for specified processes.",
            effect: TweakEffect::Restart,
            enabled_ops: &[],
                        custom_apply: Some(|ctx| {
                ctx.post_status("Enabling QoS DSCP Marking...");

                // 1. Enable global policy
                crate::common::set_string(
                    &winsafe::HKEY::LOCAL_MACHINE,
                    r"SOFTWARE\Policies\Microsoft\Windows\QoS",
                    "Application DSCP Marking Request",
                    "Allowed"
                )?;

                // 2. Process custom apps
                if let Some(apps_list) = ctx.input_values.get("enable_qos_dscp_marking") {
                    if !apps_list.trim().is_empty() {
                        for app in apps_list.split(';') {
                            let app = app.trim();
                            if app.is_empty() { continue; }

                            let key_name = format!(r"SOFTWARE\Policies\Microsoft\Windows\QoS\{}", app);
                            ctx.post_status(&format!("Creating QoS policy for {}", app));

                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Version", "1.0")?;
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Application Name", app)?;
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "DSCP Value", "46")?; // 46 = EF (Expedited Forwarding)
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Protocol", "*")?;
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Local Port", "*")?;
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Remote Port", "*")?;
                            crate::common::set_string(&winsafe::HKEY::LOCAL_MACHINE, &key_name, "Throttle Rate", "-1")?;
                        }
                    }
                }
                Ok(())
            }),
                        requires_restart: true,
            has_custom_input: true,
            default_text: Some(QOS_GAMES_LIST),
        },
        crate::tweak! {
            id: "optimize_timer_resolution",
            category: "performance",
            name: "Optimize Timer Resolution",
            description: "Configures the system timer resolution for lower latency.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Psched", "TimerResolution", 1, RegistryValue::Delete),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "network_throttling_index",
            category: "performance",
            name: "Disable Network Throttling Index",
            description: "Disables network throttling mechanism for non-multimedia traffic (default limits ~10 packets/ms).",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile", "NetworkThrottlingIndex", 0xffffffff, 10),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "gaming_task_priority",
            category: "performance",
            name: "Set High Priority for Games",
            description: "Sets the system scheduler priority for gaming tasks to High.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games", "Priority", 6, 2),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games", "Scheduling Category", "High", "Medium"),
                crate::reg_str!("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games", "SFIO Priority", "High", "Normal"),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "disable_device_power_saving",
            category: "performance",
            name: "Disable Device Power Saving",
            description: "Prevents Windows from turning off devices (USB, Network, etc.) to save power.",
            effect: TweakEffect::Immediate,
            enabled_ops: &[],
                        custom_apply: Some(|ctx| {
                ctx.post_status("Disabling device power saving...");
                let cmd = r#"Get-WmiObject MSPower_DeviceEnable -Namespace root\wmi | ForEach-Object { $_.enable = $false; $_.psbase.put() }"#;
                let _ = crate::run_powershell_command(cmd);
                Ok(())
            }),
                        command: Some("Get-WmiObject MSPower_DeviceEnable -Namespace root\\wmi | ForEach-Object { $_.enable = $false; $_.psbase.put() }"),
        },
        crate::tweak! {
            id: "disable_driver_updates",
            category: "performance",
            name: "Disable Automatic Driver Updates",
            description: "Prevents Windows Update from automatically installing drivers.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching", "SearchOrderConfig", 0, 1),
                crate::reg_dword!("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\DeviceSetup\Settings", "PreventDeviceDriverUpdate", 1, 0),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate", "ExcludeWUDriversInQualityUpdate", 1, RegistryValue::Delete),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "optimize_browser_policies",
            category: "performance",
            name: "Optimize Browser Policies (Edge, Chrome, Brave)",
            description: "Disables background running and startup boost for major browsers.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                // Edge
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "BackgroundModeEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "StartupBoostEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Microsoft\Edge", "UserFeedbackAllowed", 0, RegistryValue::Delete),
                // Chrome
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Google\Chrome", "BackgroundModeEnabled", 0, RegistryValue::Delete),
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\Google\Chrome", "MetricsReportingEnabled", 0, RegistryValue::Delete),
                // Brave
                crate::reg_dword!("HKLM", r"SOFTWARE\Policies\BraveSoftware\Brave", "BackgroundModeEnabled", 0, RegistryValue::Delete),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
            id: "optimize_app_timeouts",
            category: "performance",
            name: "Optimize App Termination Timeouts",
            description: "Reduces the time Windows waits for hung apps to close during shutdown or when force-closing.",
            effect: TweakEffect::Restart,
            enabled_ops: &[
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "WaitToKillAppTimeout", "2000", "20000"),
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "HungAppTimeout", "1000", "5000"),
                crate::reg_str!("HKCU", r"Control Panel\Desktop", "AutoEndTasks", "1", "0"),
            ],
                        requires_restart: true,
        },
        crate::tweak! {
                id: "visual_fx_performance",
                category: "performance",
                name: "Visual Effects for Performance",
                description: "Sets visual effects to performance mode while keeping useful effects enabled.",
                effect: TweakEffect::Immediate,
                enabled_ops: &[
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\VisualEffects", "VisualFXSetting", 3, RegistryValue::Delete),
                        // Keep listview alpha selection
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ListviewAlphaSelect", 1, 1),
                        // Keep listview shadows
                        crate::reg_dword!("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ListviewShadow", 1, 1),
                        // Keep font smoothing
                        crate::reg_str!("HKCU", r"Control Panel\Desktop", "FontSmoothing", "2", "2"),
                ],
                },
        crate::tweak! {
            id: "optimize_platform_tick",
            category: "performance",
            name: "Optimize Platform Tick & Timers",
            description: "Optimizes system timers by disabling dynamic tick and enhancing TSC synchronization (BCD).",
            effect: TweakEffect::Restart,
            enabled_ops: &[],
                        custom_apply: Some(|ctx| {
                ctx.post_status("Optimizing platform tick...");
                let _ = crate::run_system_command("bcdedit", &["/deletevalue", "useplatformclock"]);
                let _ = crate::run_system_command("bcdedit", &["/deletevalue", "useplatformtick"]);
                let _ = crate::run_system_command("bcdedit", &["/set", "disabledynamictick", "yes"]);
                let _ = crate::run_system_command("bcdedit", &["/set", "tscsyncpolicy", "enhanced"]);
                Ok(())
            }),
                        requires_restart: true,
            command: Some("bcdedit /deletevalue useplatformclock\nbcdedit /deletevalue useplatformtick\nbcdedit /set disabledynamictick yes\nbcdedit /set tscsyncpolicy enhanced"),
        },
];
