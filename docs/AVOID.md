# AVOID: Dangerous & Harmful Windows Registry Edits

**WARNING:** Do not apply these registry edits. They are known to cause critical system instability, broken user interfaces, boot loops, or security risks on Windows 10 and Windows 11.

## 1. System Killers (BSODs & Boot Loops)

### Forcing Native NVMe Drivers (Server 2025 Hack)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Policies\Microsoft\FeatureManagement\Overrides`
*   **Values:** `735209102`, `1853569164`, `156965516` (set to `1`)
*   **The Risk:** Often promoted to "boost SSD speed," this forces an enterprise driver path not fully supported on consumer hardware.
*   **Result:** Frequently causes **Inaccessible Boot Device** BSODs, breaks Safe Mode, and can corrupt disk identifiers, causing backups to fail.

### Disabling Desktop Window Manager (DWM)
*   **Key:** `HKLM\SOFTWARE\Microsoft\Windows\DWM` (Various hacks to force "Composition" off)
*   **The Risk:** Attempting to remove the Windows compositor to "reduce input lag."
*   **Result:** Windows 10/11 require DWM. Disabling it results in a **Black Screen** with only a cursor, rendering the system unusable.

### Disabling Critical Services (AppXSvc)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Services\AppXSvc`
*   **Value:** `Start` set to `4` (Disabled)
*   **The Risk:** Disabling the "AppX Deployment Service" to save RAM.
*   **Result:** Breaks the **Start Menu, Taskbar, Notification Center, and all Microsoft Store apps** (Calculator, Photos, Settings). Can also cause login loops.

### "Instant" Shutdown (Zero Timeout)
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control`
*   **Value:** `WaitToKillServiceTimeout` set to `0`
*   **The Risk:** Forces Windows to kill all processes instantly without waiting for them to save data.
*   **Result:** High risk of **User Profile Corruption** (cannot login), registry hive corruption, and data loss in open applications.

---

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

### Force Move/Resize Taskbar (Windows 11)
*   **Keys:** `HKCU\...\Explorer\StuckRects3` or `TaskbarSi`
*   **The Risk:** Modifying binary data to force the Taskbar to the Top/Left or change its size.
*   **Result:** In Windows 11 22H2 and later, this code was removed. Using these tweaks now causes `explorer.exe` to **crash loop**, leaving you with no desktop interface.

### Setting `TabletInputService` to disabled
*   **The Risk:** Breaks keyboard entry into the start menu, settings, all UWP apps, box IME for Eastern Asian languages, emoji picker, and handwriting panel.

### Setting the `CI` environment variable to `1`
*   **The Risk:** Breaks gemini-cli.

---

## 3. "Snake Oil" & Performance Degraders
These tweaks are legacies from the Windows XP/7 era. They do **not** improve performance on modern systems and often cause stuttering or memory errors.

### Large System Cache
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `LargeSystemCache` = `1`
*   **The Myth:** Optimizes file caching speed.
*   **Reality:** Designed for Servers. On desktops, it aggressively steals RAM from games and apps to cache files, leading to **micro-stuttering** and "Out of Memory" errors.

### Disable Paging Executive
*   **Key:** `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management`
*   **Value:** `DisablePagingExecutive` = `1`
*   **The Myth:** Keeps core drivers in RAM for speed.
*   **Reality:** Useless on systems with ample RAM. If memory *does* run low, the kernel refuses to page out idle data, forcing active applications to crash instead.

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


---

## 4. Dangerous Combinations

### The "Zombie OS" Combo
*   **Edits:** Disabling `wuauserv` (Update Service) **+** Deleting `AppXSvc` key.
*   **Result:** Prevents all updates and Store access. Since the Store depends on the Update service, and the Start Menu depends on AppX, this combination leaves the OS in an unrepairable state where `sfc /scannow` fails and no system components can be re-installed.

### The "Silent Admin" Security Hole
*   **Edits:** `EnableLUA = 0` (Disable UAC) **+** `ConsentPromptBehaviorAdmin = 0`
*   **Result:** Malware and scripts can execute with System/Root privileges instantly without you ever seeing a prompt, notification, or warning.