#Requires -Version 5 -RunAsAdministrator
using namespace Microsoft.Win32

wmic.exe UserAccount set PasswordExpires=False

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Appx', 'AllowAutomaticAppArchiving', '0', [RegistryValueKind]::DWord)

# Skip to the sign-on screen.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Personalization', 'NoLockScreen', '1', [RegistryValueKind]::DWord)

# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Explorer', 'NoUseStoreOpenWith', '1', [RegistryValueKind]::DWord)

# Do not show Windows tips.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent', 'DisableSoftLanding', '1', [RegistryValueKind]::DWord)

# Show what's slowing down bootups and shutdowns.
[Registry]::SetValue('HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System', 'verbosestatus', '1', [RegistryValueKind]::DWord)

# Do not suggest ways to "finish setting up my device to get the most out of Windows".
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement', 'ScoobeSystemSettingEnabled', '0', [RegistryValueKind]::DWord)

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
[Registry]::SetValue('HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced', 'Start_IrisRecommendations', '0', [RegistryValueKind]::DWord)

# All background images are converted to JPEG, so avoid its compression as much as possible.
[Registry]::SetValue('HKEY_CURRENT_USER\Control Panel\Desktop', 'JPEGImportQuality', '100', [RegistryValueKind]::DWord)

# Disable feedback reminders.
[Registry]::SetValue('HKEY_CURRENT_USER\SOFTWARE\Microsoft\Siuf\Rules', 'NumberOfSIUFInPeriod', '0', [RegistryValueKind]::DWord)
[Registry]::SetValue('HKEY_CURRENT_USER\SOFTWARE\Microsoft\Siuf\Rules', 'PeriodInNanoSeconds', '0', [RegistryValueKind]::DWord)
