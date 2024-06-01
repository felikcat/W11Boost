#Requires -Version 5 -RunAsAdministrator
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

# Loads Group Policies asynchronous. By default this is already asynchoronous.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows NT\CurrentVersion\Winlogon' -Key 'SyncForegroundPolicy' -Value '0' -Type 'DWord'

# Page Combining is a feature meant to reduce memory usage, but introduces security risks and lowers performance.
# https://kaimi.io/en/2020/07/reading-another-process-memory-via-windows-10-page-combining-en/
# https://forums.guru3d.com/threads/a-bit-detailed-info-about-memory-combining-in-win10.419262/
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -Key 'DisablePageCombining' -Value '1' -Type 'DWord'

# Prefer IPv6 whenever possible.
# https://docs.microsoft.com/en-US/troubleshoot/windows-server/networking/configure-ipv6-in-windows#use-registry-key-to-configure-ipv6
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters' -Key 'DisabledComponents' -Value '0' -Type 'DWord'

# Splitting SvcHost less decreases Windows' stability; set it to defaults.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control' -Key 'SvcHostSplitThresholdInKB' -Value '3670016' -Type 'DWord'

# Ensure IPv6 and its related features are enabled.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc' -Key 'Start' -Value '2' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc' -Key 'Start' -Value '3' -Type 'DWord'
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters' -Key 'EnablePrefetcher' -Value '3' -Type 'DWord'
# The memory performance issues related to requesting data from disk has been solved years ago.
# Disabling SysMain (Superfetch) would make memory page fetching slower by:
# - Less pages being cached into memory/RAM, and in an un-intelligent manner.
# - Increase the amount of random I/O reads and writes; much slower than RAM.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\SysMain' -Key 'Start' -Value '2' -Type 'DWord'

# Allow Phone -> PC linking on this device.
# NOTE 1: Allows advertised apps in the start menu on Windows 11; StartAllBack is used to side-step this problem.
# NOTE 2: 'DisableWindowsConsumerFeatures' = 1 only applies to Enterprise and Education editions of Windows.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System' -Key 'EnableMmx' -Value '1' -Type 'DWord'
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Key 'DisableWindowsConsumerFeatures' -Value '0' -Type 'DWord'

# Old versions of the Intel PROSet software set this to 0, breaking Windows' internet connectivity check.
SetReg -Path 'HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet' -Key 'EnableActiveProbing' -Value '1' -Type 'DWord'


# Disabling threaded DPCs is for debugging purposes and will cause spinlocks; it does not lower DPC latency.
Remove-ItemProperty -Path 'HKLM:\System\CurrentControlSet\Control\Session Manager\kernel' -Name 'ThreadDpcEnable'

# Delaying the startup of third-party apps gives Windows more room to breathe for its own jobs, speeding up the overall startup time.
Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize" -Name "Startupdelayinmsec"

# Modern GPUs can dispatch multiple high-priority queues without slowing each other down, so enable preemption.
# If a high-priority job is running and preemption is off, it could lead to other software waiting too long to get a share of the GPU's time, and become noticeably slow.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler" -Name "EnablePreemption"

# Explains MPO well and still pertains to Windows in principle:
# https://kernel.org/doc/html/next/gpu/amdgpu/display/mpo-overview.html
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\Dwm" -Name "OverlayTestMode"

# These keys will regenerate if Windows is installed to an HDD.
# These keys Do not exist for an SSD or NVMe Windows installation by default.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "EnablePrefetcher"
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "EnableSuperfetch"

# Ensure default 2GB memory boundary for x86 apps.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "AllocationPreference"

# Disk defragmentation does TRIMs on SSDs. Not running TRIMs at least once a week will reduce performance.
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Dfrg\BootOptimizeFunction" -Name "Enable"

# Revert to Windows' default shutdown behavior regarding handling of apps.
$REGS = @("WaitToKillAppTimeOut", "HungAppTimeout", "WaitToKillServiceTimeout")
$REGS.ForEach({
        Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name $_
        Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control" -Name $_
    })


# Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue "{default}" tscsyncpolicy
bcdedit.exe /deletevalue "{default}" useplatformclock
bcdedit.exe /deletevalue "{default}" x2apicpolicy
bcdedit.exe /set "{default}" uselegacyapicmode no

# Enable idle tickless.
bcdedit.exe /set "{default}" disabledynamictick no

# Draw graphical elements for boot (progress spinner, Windows or BIOS logo, etc).
# This is useful to tell if something went wrong if a BSOD can't show up.
bcdedit.exe /deletevalue "{default}" bootuxdisabled

Enable-MMAgent -ApplicationLaunchPrefetching
Enable-MMAgent -ApplicationPreLaunch

# Apps that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
if ($env:PROCESSOR_IDENTIFIER -match 'GenuineIntel') {
    setx.exe /M OPENSSL_ia32cap "~0x200000200000000"
}
