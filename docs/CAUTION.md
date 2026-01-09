# CAUTION: Use with Care

The following registry edits are functional but may cause annoyances, minor glitches, or unexpected behavior in specific scenarios.

## 1. Interface & Responsiveness

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

### Classic Context Menu (Windows 11)
*   **Key:** `HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32`
*   **Value:** (Default) set to Blank.
*   **Tweak:** Restore the Windows 10 Right-Click menu.
*   **Side Effect:** This overrides a specific COM object. Microsoft often resets this key during major Feature Updates (e.g., 23H2), requiring re-application.

---

## 2. Gaming & Network

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