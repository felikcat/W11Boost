# AVOID: Dangerous & Harmful Windows Registry Edits

**WARNING:** Do not apply these registry edits. They are known to cause critical system instability, broken user interfaces, boot loops, or security risks on Windows 10 and Windows 11.

## 1. System Killers (BSODs, Boot Loops & Update Blocks)

### Forcing Native NVMe Drivers (Server 2025 Hack)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides`
*   **Values:** `735209102`, `1853569164`, `156965516` (set to `1`)
*   **The Risk:** Often promoted to "boost SSD speed," this forces an enterprise driver path not fully supported on consumer hardware.
*   **Result:** Frequently causes **Inaccessible Boot Device** BSODs, breaks Safe Mode, and can corrupt disk identifiers, causing backups to fail.

### WD SSD "HMB" Boost (Host Memory Buffer)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\StorPort`
*   **Value:** `HmbAllocationPolicy` (Set to `0` or modified)
*   **The Risk:** A trending tweak to "boost" DRAM-less NVMe SSD performance.
*   **Result:** On **Windows 11 24H2**, this triggers a firmware bug in **Western Digital (SN770/SN580)** drives, causing immediate and repeated **Blue Screens (CRITICAL_PROCESS_DIED)** or file corruption.

### MSConfig "Number of Processors"
*   **Location:** `msconfig.exe` > Boot > Advanced Options
*   **The Myth:** Checking this box and selecting the max number "activates" all cores to speed up boot.
*   **Reality:** Windows uses all cores by default. This setting is a **debug limit** for developers to test software on *fewer* cores.
*   **Result:** Causes **Blue Screen (BAD_SYSTEM_CONFIG_INFO)** or permanently caps your high-end CPU to 1 core, crippling performance.

### Disabling Desktop Window Manager (DWM)
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows\DWM` (Various hacks to force "Composition" off)
*   **The Risk:** Attempting to remove the Windows compositor to "reduce input lag."
*   **Result:** Windows 10/11 require DWM. Disabling it results in a **Black Screen** with only a cursor, rendering the system unusable.

### Disable GPU Timeout Detection (TDR)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers`
*   **Value:** `TdrLevel` = `0`
*   **The Myth:** Fixes game crashes by stopping Windows from resetting the driver.
*   **The Risk:** TDR is a safety mechanism. If your GPU hangs for >2 seconds, Windows resets the driver to recover the desktop.
*   **Result:** With TDR disabled, a simple GPU hiccup or game freeze causes your **entire PC to lock up permanently**, requiring a physical hard reboot and risking data corruption.

### ExplorerPatcher / StartAllBack "Leftovers" (24H2)
*   **Risk:** Updating to Windows 11 24H2 with active registry hooks from third-party shell tools.
*   **The Mechanism:** These tools inject old DLLs (like `dxgi.dll` or `ExplorerFrame.dll`) to restore the Windows 10 taskbar.
*   **Result:** Windows 11 24H2 blocks these DLLs. If forced via registry or left behind after a dirty uninstall, `explorer.exe` will **crash loop instantly** on login (flashing black screen), preventing access to the desktop.

### Disabling Critical Services (AppXSvc)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\AppXSvc`
*   **Value:** `Start` set to `4` (Disabled)
*   **The Risk:** Disabling the "AppX Deployment Service" to save RAM.
*   **Result:** Breaks the **Start Menu, Taskbar, Notification Center, and all Microsoft Store apps** (Calculator, Photos, Settings). Can also cause login loops.

### Disabling DNS Client (Dnscache)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\Dnscache`
*   **Value:** `Start` = `4` (Disabled)
*   **The Myth:** "Stops telemetry" or saves RAM.
*   **Result:** Unlike Windows 7, the DNS Client in Windows 11 is a hard dependency for the Network Store Interface (NSI). Disabling it results in **Total Internet Loss** (WiFi and Ethernet) for all Store apps, Windows Update, and modern browsers.

### Disabling Windows Event Log
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\EventLog`
*   **Value:** `Start` = `4` (Disabled)
*   **The Myth:** Saves disk I/O and improves privacy.
*   **Result:** Crucial services (Network List Service, Task Scheduler) depend on this. Disabling it causes **massively long boot times**, breaks WiFi/Ethernet detection, and leaves you with zero logs to troubleshoot issues.

### "Instant" Shutdown (Zero Timeout)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control`
*   **Value:** `WaitToKillServiceTimeout` set to `0`
*   **The Risk:** Forces Windows to kill all processes instantly without waiting for them to save data.
*   **Result:** High risk of **User Profile Corruption** (cannot login), registry hive corruption, and data loss in open applications.


## 2. Feature Breakers (UI & Apps)

### Disabling User Account Control (UAC) Completely
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System`
*   **Value:** `EnableLUA` = `0`
*   **The Risk:** Disabling the UAC sandbox completely.
*   **Result:** **Permanently breaks all UWP apps.** Calculator, Photos, and the Microsoft Store will refuse to launch. Microsoft Edge may also fail to open.

### Disable "Background Apps" Globally (Windows 11)
*   **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications`
*   **Value:** `GlobalUserDisabled` = `1`
*   **The Risk:** Disables the background execution permission for all modern apps.
*   **Result:** Unlike Windows 10, Windows 11 relies on this for core features. **Alarms will not ring**, Mail/Calendar will not sync, and Phone Link will break.

### Force-Disabling IPv6
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters`
*   **Value:** `DisabledComponents` = `0xFFFFFFFF`
*   **The Myth:** "Speeds up internet" or "lowers ping."
*   **Result:** Windows 11 core components rely on IPv6 loopback. Disabling it breaks **Phone Link**, **Nearby Sharing**, **Miracast**, and causes the **Microsoft Store** to fail downloading apps.

### Removing WebView2 (The "De-Google" Script)
*   **Target:** Removing `Microsoft.WebView2` via script or registry brute-force.
*   **The Risk:** Treating WebView2 like "bloatware."
*   **Result:** WebView2 is a core OS dependency, distinct from the Edge Browser. Removing it breaks **Outlook (New), Teams, Copilot, Widgets, and the System Login/OOBE screens**. Reinstalling it often fails, requiring a full Windows reset.

### Force Move/Resize Taskbar (Windows 11)
*   **Keys:** `HKCU\...\Explorer\StuckRects3` or `TaskbarSi`
*   **The Risk:** Modifying binary data to force the Taskbar to the Top/Left or change its size.
*   **Result:** In Windows 11 22H2 and later, this code was removed. Using these tweaks now causes `explorer.exe` to **crash loop**, leaving you with no desktop interface.

### Setting `TabletInputService` to disabled
*   **The Risk:** Breaks keyboard entry into the start menu, settings, all UWP apps, box IME for Eastern Asian languages, emoji picker, and handwriting panel.

### Setting the `CI` environment variable to `1`
*   **The Risk:** Breaks gemini-cli.

### Removing "Microsoft.SecHealthUI"
*   **Method:** PowerShell debloat scripts removing AppX packages blindly.
*   **The Risk:** Removes the UI for Windows Defender.
*   **Result:** You cannot open Windows Security to manage antivirus or firewall exceptions. The antivirus still runs in the background (blocking files), but you have **no interface** to allow them or see what is happening.

## 3. "Snake Oil" & Performance Degraders

### Large System Cache
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `LargeSystemCache` = `1`
*   **The Myth:** Optimizes file caching speed.
*   **Reality:** Designed for Servers. On desktops, it aggressively steals RAM from games and apps to cache files, leading to **micro-stuttering** and "Out of Memory" errors.

### Second Level Data Cache (L2 Cache)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `SecondLevelDataCache`
*   **The Myth:** "Tell Windows your CPU's L2 cache size to boost speed."
*   **Reality:** Windows automatically retrieves this from the hardware (`CPUID`) at boot. Entering the wrong value can force the OS to optimize for the *wrong* cache size, **degrading performance**.

### Disable Paging Executive
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `DisablePagingExecutive` = `1`
*   **The Myth:** Keeps core drivers in RAM for speed.
*   **Reality:** Useless on systems with ample RAM. If memory *does* run low, the kernel refuses to page out idle data, forcing active applications to crash instead.

### Force-Disabling the Page File
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `PagingFiles` (Deleted/Empty)
*   **The Myth:** "I have 32GB RAM, I don't need a Page File. It slows me down."
*   **The Risk:** Windows is designed to use the Page File for memory commit guarantees.
*   **Result:** Causes random **crashes to desktop** in heavy apps (Cyberpunk 2077, Chrome, Video Editors) even with free RAM available. It also prevents **BSOD Mini Dumps** from being created, making diagnosis impossible.

### Realtime CPU Priority
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\{Game.exe}\PerfOptions`
*   **Value:** `CpuPriorityClass` = `3`
*   **The Myth:** Gives a game "Realtime" priority for max FPS.
*   **Reality:** Realtime priority is higher than mouse/keyboard input drivers. This causes **severe input lag**, audio stuttering, and can lock up the entire PC if the game hits 100% CPU.

### Clear Page File at Shutdown
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `ClearPageFileAtShutdown` = `1`
*   **The Myth:** Frees up space or "cleans" RAM.
*   **Reality:** Forces Windows to overwrite the virtual memory file with zeros at shutdown. Adds **minutes** to shutdown time with zero performance benefit.

### Always Unload DLL
*   **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer`
*   **Value:** `AlwaysUnloadDll` = `1`
*   **The Myth:** Forces Windows to free RAM immediately.
*   **Reality:** This feature was removed in **Windows 2000**. The key is placebo and does absolutely nothing in modern Windows.

### Disable SvcHost Splitting
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control`
*   **Value:** `SvcHostSplitThresholdInKB` (Set to absurdly high values)
*   **The Risk:** Forces all Windows services to share a single host process.
*   **Result:** While it saves a tiny amount of RAM (negligible on modern PCs), it destroys system stability. If **one** service crashes, **all** services crash, leading to an immediate BSOD or total system lockup.

### Changing "Timer Resolution" (ISLC/STR)
*   **Tools:** Intelligent Standby List Cleaner (ISLC), TimerResolution.exe, SetTimerResolutionService
*   **The Myth:** Forcing 0.5ms or 1ms timer resolution reduces input lag.
*   **Reality:** Windows 10/11 schedulers are tickless and handle this dynamically. Forcing high global resolution prevents the CPU from idling, increasing heat and power consumption while degrading multi-threaded performance.

### Disabling TCP Auto-Tuning
*   **Command:** `netsh interface tcp set global autotuninglevel=disabled`
*   **The Myth:** "Stabilizes" ping or connection speed.
*   **Reality:** Prevents Windows from dynamically adjusting the TCP receive window size. This effectively caps your download speed at ancient 1990s levels (often <10Mbps) on high-speed fiber connections.

### QoS Bandwidth Limit (The "20% Speed" Myth)
*   **Key:** `HKLM\SOFTWARE\Policies\Microsoft\Windows\Psched`
*   **Value:** `NonBestEffortLimit` = `0`
*   **The Myth:** "Windows reserves 20% of your internet bandwidth. Set to 0 to unlock it."
*   **Reality:** Windows *only* reserves bandwidth if a high-priority system task (like Windows Update) is active *and* the network is congested. If you are gaming/browsing, you already have 100%. Setting this to 0 has no impact on daily speed.

### Disabling SysMain (Superfetch)
*   **Service:** SysMain
*   **The Myth:** Saves RAM and CPU usage.
*   **Reality:** SysMain preloads your frequently used apps into *unused* RAM. Disabling it makes apps launch slower. "Free" RAM is wasted RAM; Windows automatically frees this memory instantly if an app needs it.

### Threaded DPC Mode (Ordinary DPCs)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\kernel`
*   **Value:** `ThreadDpcEnable` = `1`
*   **The Myth:** Reduces latency by running DPCs (Deferred Procedure Calls) as threads.
*   **Reality:** This is a debugging feature for kernel developers. On consumer versions of Windows, it often breaks audio drivers, causes massive DPC latency spikes, and leads to system-wide micro-stutters.

### Disabling Spectre/Meltdown Mitigations
*   **Keys:** `FeatureSettingsOverride`, `FeatureSettingsOverrideMask` in Memory Management.
*   **The Myth:** huge CPU performance gains (30%+).
*   **Reality:** On modern CPUs (Intel 9th Gen+, Ryzen 2000+), hardware fixes make the performance cost negligible (<2%). Disabling them leaves your browser memory vulnerable to malicious JavaScript stealing passwords.

### Enabling HPET (High Precision Event Timer) via BIOS/OS
*   **Command:** `bcdedit /set useplatformclock true`
*   **The Myth:** "Smoother" mouse movement or better hit registration.
*   **Reality:** Forces Windows to use a slower, legacy hardware timer on the motherboard instead of the modern, low-latency TSC (Time Stamp Counter) on the CPU. Adds latency and reduces FPS.

### Increasing Mouse/Keyboard Queue Size
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\kbdclass\Parameters`
*   **Value:** `KeyboardDataQueueSize` (>100)
*   **The Myth:** "Fit more inputs" for better responsiveness.
*   **Reality:** The queue is a buffer for *delayed* inputs. If your PC assumes it needs a larger buffer, it implies it can't process inputs fast enough. In reality, this does nothing for latency and just wastes non-paged pool memory along with causing more stutters.

### IO Page Lock Limit
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `IoPageLockLimit`
*   **The Myth:** Increases disk transfer speeds.
*   **Reality:** A relic from Windows 2000. Modern Windows versions ignore this value or manage I/O buffers dynamically far better than any static registry value can.

### Disabling Multi-Plane Overlay (MPO)
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows\Dwm\OverlayTestMode`
*   **The Myth:** Fixes flickering or stuttering.
*   **Reality:** MPO acts as a hardware accelerator for windowed games and video. Disabling it forces the DWM to compose every frame on the GPU via copy, increasing latency and GPU usage for windowed borderless games.

### Disabling Windows Error Reporting (WER)
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\Disabled` = `1`
*   **The Myth:** Stops "spying" and saves disk space.
*   **Reality:** WER is the mechanism that generates **BSOD Mini Dumps**. If you disable this service, when your PC crashes (Blue Screen of Death), **no dump file will be created**. This makes it impossible to analyze the crash using tools like BlueScreenView or WhoCrashed to identify the faulty driver or hardware. You are flying blind when diagnosing instability.


## 4. Dangerous Combinations

### The "Zombie OS" Combo
*   **Edits:** Disabling `wuauserv` (Update Service) **+** Deleting `AppXSvc` key.
*   **Result:** Prevents all updates and Store access. Since the Store depends on the Update service, and the Start Menu depends on AppX, this combination leaves the OS in an unrepairable state where `sfc /scannow` fails and no system components can be re-installed.

### The "Silent Admin" Security Hole
*   **Edits:** `EnableLUA = 0` (Disable UAC) **+** `ConsentPromptBehaviorAdmin = 0`
*   **Result:** Malware and scripts can execute with System/Root privileges instantly without you ever seeing a prompt, notification, or warning.

## 5. Windows 10 Registry Edits Incompatible with Windows 11

The following registry keys and values control features that exist in Windows 10 but have been removed, deprecated, or fundamentally re-architected in Windows 11. Applying these tweaks in Windows 11 will typically result in no effect, or in some cases (like Taskbar positioning), may cause user interface instability.

### Taskbar Customization
The Windows 11 Taskbar was rebuilt from scratch using XAML, breaking compatibility with legacy Explorer tweaks that manipulated the old code.

| Feature | Registry Key & Path | Windows 10 Function | Windows 11 Status |
| :--- | :--- | :--- | :--- |
| **Taskbar Position** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\StuckRects3`<br>**Value:** `Settings` (Binary) | Allowed moving the taskbar to the **Top**, **Left**, or **Right** by modifying the 13th byte. | **Broken / Unsupported**<br>Windows 11 hardcodes the taskbar to the bottom. Forcing other values often causes the taskbar to disappear or crash `explorer.exe`. |
| **Taskbar Size** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced`<br>**Value:** `TaskbarSi` | *Unofficial:* Changed taskbar size (0=Small, 1=Med, 2=Large). | **Removed**<br>Functioned in early Windows 11 builds but was patched out in versions 22H2/23H2. It is now completely ignored. |
| **Small Icons** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced`<br>**Value:** `TaskbarSmallIcons` | Toggled "Use small taskbar buttons" to reduce bar height. | **Ignored**<br>The new taskbar has a fixed height and grid size. This native Windows 10 value is not read by the new shell. |
| **Toolbars** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Streams\Desktop`<br>**Value:** `TaskbarWinXP` | Stored configurations for user-added toolbars (e.g., Address, Links, NetSpeedMonitor). | **Feature Removed**<br>Windows 11 does not support adding custom toolbars or deskbands to the taskbar. |
| **"My People" Bar** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced\People`<br>**Value:** `PeopleBand` | Showed the "My People" contact hub on the taskbar. | **Feature Removed**<br>The "My People" feature was completely deprecated and removed from the OS. |
| **News & Interests** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Feeds`<br>**Value:** `ShellFeedsTaskbarViewMode` | Controlled the weather/news popup on the taskbar. | **Replaced**<br>Replaced by the "Widgets" feature, which uses different registry keys (e.g., `TaskbarDa`). |

### Start Menu & Live Tiles
The Windows 11 Start Menu uses a static icon grid and does not support the "Metro" Live Tile infrastructure or XML layouts.

| Feature | Registry Key & Path | Windows 10 Function | Windows 11 Status |
| :--- | :--- | :--- | :--- |
| **Live Tiles** | **Key:** `HKCU\Software\Policies\Microsoft\Windows\CurrentVersion\PushNotifications`<br>**Value:** `NoTileApplicationNotification` | Disabled animations and updates for Live Tiles. | **Obsolete**<br>Live Tiles do not exist in Windows 11. |
| **Start Layout XML** | **Key:** `HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer`<br>**Value:** `StartLayoutFile` | Applied a locked tile layout using a specific XML schema (groups/sizes). | **Incompatible Schema**<br>Windows 11 requires a JSON-based configuration (`.json`) for the "Pinned" section. Legacy XML layouts are largely ignored. |
| **Full Screen Start** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\StartPage`<br>**Value:** `MakeAllAppsDefault` | Forced the Start Menu to open in full-screen (Tablet) mode. | **Ignored**<br>Windows 11 does not support a full-screen Start Menu interface on the desktop. |
| **More Tiles** | **Key:** `HKLM\SOFTWARE\Policies\Microsoft\Windows\Explorer`<br>**Value:** `ShowMoreTiles` | Enabled a 4th column of tiles in the Start Menu groups. | **Obsolete**<br>The grid system for tiles has been removed. |

### Deprecated System Features
Tweaks for features that have been removed from the operating system entirely or significantly altered.

| Feature | Registry Key & Path | Windows 10 Function | Windows 11 Status |
| :--- | :--- | :--- | :--- |
| **Windows Timeline** | **Key:** `HKLM\SOFTWARE\Policies\Microsoft\Windows\System`<br>**Value:** `EnableActivityFeed` | Enabled the "Timeline" history view in Task View. | **Feature Removed**<br>Timeline is gone; Task View only shows virtual desktops and current windows. |
| **Tablet Mode** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\ImmersiveShell`<br>**Value:** `TabletMode` | Manually forced the UI into "Tablet Mode." | **Changed**<br>Windows 11 switches modes automatically based on hardware posture. The manual toggle and specific registry force switch are ignored. |
| **Action Center** | **Key:** `HKCU\Control Panel\Quick Actions`<br>**Value:** `PinnedQuickActionSlotCount` | Customized the 4xN grid of Quick Actions. | **Replaced**<br>Action Center is split into "Notifications" and "Quick Settings." The legacy layout blobs do not apply to the new Quick Settings UI. |
| **Volume Mixer** | **Key:** `HKLM\Software\Microsoft\Windows NT\CurrentVersion\MTCUVC`<br>**Value:** `EnableMtcUvc` | `0` = Restored the legacy (Win7 style) volume mixer flyout. | **Ignored**<br>Windows 11 enforces the modern volume flyout and Settings-based mixer; this key no longer reverts it. |
| **Ribbon UI** | **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Ribbon`<br>**Value:** `MinimizedStateTabletModeOff` | Controlled if the Explorer Ribbon was minimized or expanded. | **Obsolete**<br>The Ribbon UI has been replaced by a modern XAML Command Bar. |

---

# CAUTION: Use with Care

The following registry edits are functional but may cause annoyances, minor glitches, or unexpected behavior in specific scenarios.

## 1. Interface & Responsiveness

### Disabling Widgets via Registry (UCPD)
*   **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced`
*   **Value:** `TaskbarDa` = `0`
*   **The Issue:** In Windows 11 23H2+, this key is protected by the **User Choice Protection Driver (UCPD)**.
*   **Side Effect:** Manually toggling this key in the registry often causes it to revert instantly or can destabilize `explorer.exe`. Use the official Taskbar Settings toggle or Group Policy instead.

### Disable Notification Center
*   **Key:** `HKCU\Software\Policies\Microsoft\Windows\Explorer`
*   **Value:** `DisableNotificationCenter` = `1`
*   **Side Effect:** In Windows 11, the Notifications and **Calendar** are combined. Enabling this tweak prevents you from opening the Calendar flyout to check dates.

### Auto End Tasks
*   **Key:** `HKCU\Control Panel\Desktop`
*   **Value:** `AutoEndTasks` = `1`
*   **The Tweak:** Automatically kills hung apps at shutdown.
*   **Side Effect:** If you have an unsaved document (Word/Notepad) and shut down, Windows may kill the app **before you can hit 'Save'**, causing data loss.

### Menu Show Delay
*   **Key:** `HKCU\Control Panel\Desktop`
*   **Value:** `MenuShowDelay` (Default: 400)
*   **Tweak:** Setting to `0`.
*   **Side Effect:** Menus open instantly but become **difficult to navigate**. If your mouse moves 1 pixel off the path, the menu vanishes immediately.
*   **Recommendation:** Set to `20` or `100`, not `0`.

### Startup Delay (Serialize)
*   **Key:** `HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize`
*   **Value:** `StartupDelayInMSec` = `0`
*   **Tweak:** Removes the delay before startup apps launch.
*   **Side Effect:** Causes a massive **CPU/Disk spike** immediately after login if you have multiple startup apps (Steam, Discord, etc.), freezing the system for several seconds.

---

## 2. Gaming, Network & Performance

### Disabling Memory Compression
*   **Command:** `Disable-MMAgent -mc`
*   **The Myth:** "Compression uses CPU, turning it off boosts FPS."
*   **Reality:** Windows compresses stale RAM (nanoseconds) to avoid writing to the slow SSD (milliseconds).
*   **Side Effect:** On systems with <32GB RAM, disabling this forces Windows to swap to disk much earlier. This causes **massive stuttering** when alt-tabbing or running background apps (Discord/Spotify) while gaming.

### MSI Mode "High Priority"
*   **Tool:** MSI Utility v3 / Registry
*   **Value:** Priority `High`
*   **The Myth:** Reduces latency by prioritizing GPU interrupts.
*   **Side Effect:** Unless the driver explicitly supports it, forcing High Priority can starve other devices (USB, Audio), leading to **crackling audio**, mouse micro-stutters, and random BSODs (DPC_WATCHDOG_VIOLATION).

### Disabling VBS (Core Isolation / Memory Integrity)
*   **Action:** Turning off "Memory Integrity" in Windows Security.
*   **The Trade-off:** This *does* increase gaming performance by ~5-10% on some systems.
*   **Why Caution?:** It disables **Kernel Data Protection**. If you download a malicious file or a game cheat that contains a rootkit, your entire OS can be compromised at the hardware level.

### Unparking CPU Cores
*   **Tools:** ParkControl / Registry Hacks
*   **The Myth:** Preventing cores from sleeping boosts FPS.
*   **Reality:** Modern schedulers (Intel Thread Director / AMD Ryzen) manage this efficiently.
*   **Side Effect:** Forcing all cores to stay awake generates excess heat. On modern CPUs, this can cause the chip to hit thermal limits faster, preventing P-Cores from boosting to their maximum turbo frequency, actually **lowering** gaming performance.

### Network Throttling Index
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile`
*   **Value:** `NetworkThrottlingIndex` = `ffffffff`
*   **Tweak:** Disables Windows network throttling for non-multimedia tasks.
*   **Side Effect:** Can cause **audio crackling**, driver instability, or glitches in video streaming (YouTube/Twitch) if you are downloading a game in the background.

### TCP Acknowledgement Frequency
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\{NIC-ID}`
*   **Values:** `TcpAckFrequency` = `1`, `TCPNoDelay` = `1`
*   **Tweak:** Forces immediate packet acknowledgement (disables Nagle's Algorithm).
*   **Side Effect:** May lower ping slightly in old games, but significantly **reduces raw download speeds** and increases CPU usage during heavy network traffic (torrenting/downloads).

### Win32 Priority Separation
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl`
*   **Value:** `Win32PrioritySeparation` (e.g., `26`, `28`, `6`)
*   **Tweak:** Changes CPU scheduling priority for foreground vs. background.
*   **Side Effect:** Setting this incorrectly can cause **micro-stuttering** in games or cause background apps (Discord voice, OBS streaming) to lag/cut out because they are starved of CPU time.


### Disable Game Bar
*   **Key:** `HKCU\Software\Microsoft\GameBar`
*   **Tweak:** Disables the Xbox Game Bar overlay and background recording.
*   **Side Effect:** This breaks **Xbox Game Pass invites**, Achievement notifications, and **Xbox Party Chat**. It also disables the built-in screen recorder (`Win+Alt+R`).

---


## 3. Privacy & Updates

### Disabling Telemetry
*   **Key:** `HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection`
*   **Value:** `AllowTelemetry` = `0`
*   **Tweak:** Reduces diagnostic data sent to Microsoft.
*   **Side Effect:**
    *   **Breaks Windows Insider:** You cannot receive beta builds.
    *   **Settings Lock:** You will see "Some settings are managed by your organization" in the Settings app.

### Disable Web Search (Start Menu)
*   **Key:** `HKCU\Software\Policies\Microsoft\Windows\Explorer`
*   **Value:** `DisableSearchBoxSuggestions` = `1`
*   **Tweak:** Removes Bing results from Start Menu.
*   **Side Effect:** Can leave a large **blank space** in the Search UI in some builds of Windows 11. You also lose calculator/conversion features in the search bar.

---

## 4. Risky Combinations

### The "Gaming Priority" Clash
*   **Edit 1:** `Win32PrioritySeparation` (Processor Scheduling) set to emphasize foreground.
*   **Edit 2:** `SystemResponsiveness` set to `0` (Multimedia Profile).
*   **Result:** Combining these can totally starve background processes. If you stream (OBS) or use voice chat (Discord) while gaming, your **mic may cut out or the stream may stutter** because the background apps are not getting enough CPU cycles to function.

## 5. Hardware Specific

### Show Seconds in System Clock
*   **Key:** `HKCU\...\Explorer\Advanced` -> `ShowSecondsInSystemClock` = `1`
*   **Side Effect:** Microsoft confirms this prevents the CPU from entering low-power "C-States" because the screen must refresh every second. This **measurably increases battery drain** on laptops.

### Fast Startup (Hiberboot) for Dual Booters
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Power`
*   **Value:** `HiberbootEnabled` = `1` (Default)
*   **The Risk:** Hibernates the kernel to speed up boot.
*   **Side Effect:** **Data Corruption** for Dual Boot users. If you mount your Windows drive in Linux while Fast Startup is active, the NTFS partition is in a "dirty" state. Writing to it from Linux will corrupt the partition table.