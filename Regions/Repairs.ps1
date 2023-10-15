#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

# Loads Group Policies asynchronous. By default this is already asynchoronous.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows NT\CurrentVersion\Winlogon', 'SyncForegroundPolicy', '0', [RegistryValueKind]::DWord)

# Page Combining is a feature meant to reduce memory usage, but introduces security risks and lowers performance.
# https://kaimi.io/en/2020/07/reading-another-process-memory-via-windows-10-page-combining-en/
# https://forums.guru3d.com/threads/a-bit-detailed-info-about-memory-combining-in-win10.419262/
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management', 'DisablePageCombining', '1', [RegistryValueKind]::DWord)

# Prefer IPv6 whenever possible.
# https://docs.microsoft.com/en-US/troubleshoot/windows-server/networking/configure-ipv6-in-windows#use-registry-key-to-configure-ipv6
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Tcpip6\Parameters', 'DisabledComponents', '0', [RegistryValueKind]::DWord)

# Splitting SvcHost less decreases Windows' stability; set it to defaults.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control', 'SvcHostSplitThresholdInKB', '3670016', [RegistryValueKind]::DWord)

# Ensure IPv6 and its related features are enabled.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\iphlpsvc', 'Start', '2', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\IpxlatCfgSvc', 'Start', '3', [RegistryValueKind]::DWord)
Set-NetAdapterBinding -Name '*' -DisplayName 'Internet Protocol Version 6 (TCP/IPv6)' -Enabled 1

[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters', 'EnablePrefetcher', '3', [RegistryValueKind]::DWord)
# The memory performance issues related to requesting data from disk has been solved years ago.
# Disabling SysMain (Superfetch) would make memory page fetching slower by:
# - Less pages being cached into memory/RAM, and in an un-intelligent manner.
# - Increase the amount of random I/O reads and writes; much slower than RAM.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\SysMain', 'Start', '2', [RegistryValueKind]::DWord)

# Allow Phone -> PC linking on this device.
# NOTE 1: Allows advertised apps in the start menu on Windows 11; StartAllBack is used to side-step this problem.
# NOTE 2: 'DisableWindowsConsumerFeatures' = 1 only applies to Enterprise and Education editions of Windows.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\System', 'EnableMmx', '1', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent', 'DisableWindowsConsumerFeatures', '0', [RegistryValueKind]::DWord)

# Old versions of the Intel PROSet software set this to 0, breaking Windows' internet connectivity check.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\NlaSvc\Parameters\Internet', 'EnableActiveProbing', '1', [RegistryValueKind]::DWord)


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
