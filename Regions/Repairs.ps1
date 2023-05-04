# Prefer IPv6 whenever possible; avoids NAT and handles fragmentation locally instead of on the router.
# https://docs.microsoft.com/en-US/troubleshoot/windows-server/networking/configure-ipv6-in-windows#use-registry-key-to-configure-ipv6
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters' -ValueName 'DisabledComponents' -Data '0' -Type 'Dword'

# Splitting SvcHost less decreases Windows' stability; set it to defaults.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control' -ValueName 'SvcHostSplitThresholdInKB' -Data '3670016' -Type 'Dword'

# Disabling threaded DPCs is for debugging purposes and will cause spinlocks; it does not lower DPC latency.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'System\CurrentControlSet\Control\Session Manager\kernel' -ValueName 'ThreadDpcEnable' -Data '1' -Type 'Dword'

# Delaying the startup of third-party programs gives Windows more room to breathe for its own jobs, speeding up the overall startup time.
reg.exe delete "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize\Startupdelayinmsec" /f

# Disabling "smart multi-homed name resolution" can make DNS requests extremely slow.
# Some VPN clients disable this as a hack to prevent DNS leaks.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows NT\DNSClient' -ValueName 'DisableSmartNameResolution' -Data '0' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Services\Dnscache\Parameters' -ValueName 'DisableParallelAandAAAA' -Data '0' -Type 'Dword'

# Modern GPUs can dispatch multiple high-priority queues without slowing each other down, so enable preemption.
# If a high-priority job is running and preemption is off, it could lead to other software waiting too long to get a share of the GPU's time, and become noticeably slow.
reg.exe delete "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler\EnablePreemption" /f

# Explains MPO well and still pertains to Windows in principle:
# https://kernel.org/doc/html/next/gpu/amdgpu/display/mpo-overview.html
reg.exe delete "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\Dwm\OverlayTestMode" /f

# These keys will regenerate if Windows is installed to an HDD.
# These keys don't exist for an SSD or NVMe Windows installation by default.
reg.exe delete "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\EnablePrefetcher" /f
reg.exe delete "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\EnableSuperfetch" /f

Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters' -ValueName 'EnablePrefetcher' -Data '3' -Type 'Dword'
# The memory performance issues related to requesting data from disk has been solved years ago.
# Disabling SysMain (Superfetch) would make memory page fetching slower by:
# - Less pages being cached into memory/RAM, and in an un-intelligent manner.
# - Increase the amount of random I/O reads and writes; much slower than RAM.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\SysMain" /v "Start" /t REG_DWORD /d 2 /f

# https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-operatingsystem
# LargeSystemCache is obsolete and not supported.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management' -ValueName 'LargeSystemCache' -Data '0' -Type 'Dword'

# Allow Phone -> PC linking on this device.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\System' -ValueName 'EnableMmx' -Data '1' -Type 'Dword'

# Process Lasso or manually setting a non-battery saving power profile is preferred instead.
# Don't make the power saving profiles less helpful.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SYSTEM\CurrentControlSet\Control\Power\PowerThrottling' -ValueName 'PowerThrottlingOff' -Data '0' -Type 'Dword'

# Use sane defaults for these sensitive timer related settings.
bcdedit.exe /deletevalue tscsyncpolicy
bcdedit.exe /deletevalue uselegacyapicmode
bcdedit.exe /deletevalue useplatformclock
bcdedit.exe /deletevalue x2apicpolicy
bcdedit.exe /set disabledynamictick yes
bcdedit.exe /set uselegacyapicmode no

Enable-MMAgent -ApplicationLaunchPrefetching
Enable-MMAgent -ApplicationPreLaunch

# Draw graphical elements for boot (progress spinner, Windows or BIOS logo, etc).
# This is useful to tell if something went wrong if a BSOD can't show up.
bcdedit.exe /deletevalue bootuxdisabled

# Ensure IPv6 and its related features are enabled.
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc" /v "Start" /t REG_DWORD /d 2 /f
reg.exe add "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc" /v "Start" /t REG_DWORD /d 3 /f
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

# Programs that rely on 8.3 filenames from the DOS-era will break if this is disabled.
fsutil.exe behavior set disable8dot3 2
