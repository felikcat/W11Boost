#Requires -Version 5 -RunAsAdministrator

# Prefer IPv6 whenever possible; avoids NAT and handles fragmentation locally instead of on the router.
# https://docs.microsoft.com/en-US/troubleshoot/windows-server/networking/configure-ipv6-in-windows#use-registry-key-to-configure-ipv6
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters' -ValueName 'DisabledComponents' -Data '0' -Type 'Dword'

# Splitting SvcHost less decreases Windows' stability; set it to defaults.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control' -ValueName 'SvcHostSplitThresholdInKB' -Data '3670016' -Type 'Dword'

# Disabling threaded DPCs is for debugging purposes and will cause spinlocks; it does not lower DPC latency.
PolEdit_HKLM 'System\CurrentControlSet\Control\Session Manager\kernel' -ValueName 'ThreadDpcEnable' -Data '1' -Type 'Dword'

# Delaying the startup of third-party programs gives Windows more room to breathe for its own jobs, speeding up the overall startup time.
Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize" -Name "Startupdelayinmsec" -Force

# Disabling "smart multi-homed name resolution" can make DNS requests extremely slow.
# Some VPN clients disable this as a hack to prevent DNS leaks.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows NT\DNSClient' -ValueName 'DisableSmartNameResolution' -Data '0' -Type 'Dword'
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services\Dnscache\Parameters' -ValueName 'DisableParallelAandAAAA' -Data '0' -Type 'Dword'

# Modern GPUs can dispatch multiple high-priority queues without slowing each other down, so enable preemption.
# If a high-priority job is running and preemption is off, it could lead to other software waiting too long to get a share of the GPU's time, and become noticeably slow.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler" -Name "EnablePreemption" -Force

# Explains MPO well and still pertains to Windows in principle:
# https://kernel.org/doc/html/next/gpu/amdgpu/display/mpo-overview.html
Remove-ItemProperty -Path "HKLM:\SOFTWARE\Microsoft\Windows\Dwm" -Name "OverlayTestMode" -Force

# These keys will regenerate if Windows is installed to an HDD.
# These keys don't exist for an SSD or NVMe Windows installation by default.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "EnablePrefetcher" -Force
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "EnableSuperfetch" -Force

PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters' -ValueName 'EnablePrefetcher' -Data '3' -Type 'Dword'
# The memory performance issues related to requesting data from disk has been solved years ago.
# Disabling SysMain (Superfetch) would make memory page fetching slower by:
# - Less pages being cached into memory/RAM, and in an un-intelligent manner.
# - Increase the amount of random I/O reads and writes; much slower than RAM.
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\SysMain" -Name "Start" -Type DWord -Value 2 -Force

# https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem
# LargeSystemCache is obsolete and not supported.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -ValueName 'LargeSystemCache' -Data '0' -Type 'Dword'

# Allow Phone -> PC linking on this device.
# NOTE 1: Allows advertised apps in the start menu on Windows 11; StartAllBack is used to side-step this problem.
# NOTE 2: 'DisableWindowsConsumerFeatures' = 1 only applies to Enterprise and Education editions of Windows.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableMmx' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableWindowsConsumerFeatures' -Data '0' -Type 'Dword'

# Process Lasso or manually setting a non-battery saving power profile is preferred instead.
# Don't make the power saving profiles less helpful.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Power\PowerThrottling' -ValueName 'PowerThrottlingOff' -Data '0' -Type 'Dword'

# Old versions of the Intel PROSet software set this to 0, breaking Windows' internet connectivity check.
PolEdit_HKLM 'SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet' -ValueName 'EnableActiveProbing' -Data '1' -Type 'Dword'

# Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /set uselegacyapicmode no
bcdedit.exe /deletevalue x2apicpolicy

# Enable idle tickless for lower power draw and benefits to real-time programs like DAWs and virtual machines, including foreground programs like video games.
# Personal anecdote: I used to run Windows 10/11 in a QEMU virtual machine ("VFIO"-type configuration), full (not idle) tickless was necessary to not have games stutter.
bcdedit.exe /set disabledynamictick no

# Deny global adjustment of timer resolution precision so poorly written programs can't fuck up the precision for other programs.
# -> In detail: https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/
# -> A poorly written program anecdote: https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/#comment-103111
if ($WIN_BUILDNUMBER -ge 18836) # Windows 2004/20H1 first public preview build.
{
    PolEdit_HKLM 'SYSTEM\CurrentControlSet\Control\Session Manager\kernel' -ValueName 'GlobalTimerResolutionRequests' -Data '0' -Type 'Dword'
}

Enable-MMAgent -ApplicationLaunchPrefetching
Enable-MMAgent -ApplicationPreLaunch

# Draw graphical elements for boot (progress spinner, Windows or BIOS logo, etc).
# This is useful to tell if something went wrong if a BSOD can't show up.
bcdedit.exe /deletevalue bootuxdisabled

# Ensure IPv6 and its related features are enabled.
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\iphlpsvc" -Name "Start" -Type DWord -Value 2 -Force
Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" -Name "Start" -Type DWord -Value 3 -Force
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2

# Revert to Windows' default shutdown behavior regarding handling of apps and programs.
$REGS = @("WaitToKillAppTimeOut", "HungAppTimeout", "WaitToKillServiceTimeout")
$REGS.ForEach({
    Remove-ItemProperty -Path "HKCU:\Control Panel\Desktop" -Name $_ -Force
    Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control" -Name $_ -Force
})

# https://www.intel.com/content/www/us/en/developer/articles/troubleshooting/openssl-sha-crash-bug-requires-application-update.html
if ($env:PROCESSOR_IDENTIFIER -match 'GenuineIntel') {
    [Environment]::SetEnvironmentVariable("OPENSSL_ia32cap", "~0x200000200000000", "Machine")
}

# Ensure default 2GB memory boundary for x86 programs.
# Prevent bugs or crashes with x86 programs that aren't specifically tested for LargeAddressAware (3GB limit).
# Manually patch programs with LAA instead if it's known to be beneficial, such as in GTA:SA.
Remove-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" -Name "AllocationPreference" -Force
