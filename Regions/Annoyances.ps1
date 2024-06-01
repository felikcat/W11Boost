#Requires -Version 5 -RunAsAdministrator
Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"

wmic.exe UserAccount set PasswordExpires=False

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Appx' -Key 'AllowAutomaticAppArchiving' -Value '0' -Type 'DWord'

# Skip to the sign-on screen.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Personalization' -Key 'NoLockScreen' -Value '1' -Type 'DWord'

# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\Explorer' -Key 'NoUseStoreOpenWith' -Value '1' -Type 'DWord'

# Do not show Windows tips.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Key 'DisableSoftLanding' -Value '1' -Type 'DWord'

# Show what's slowing down bootups and shutdowns.
SetReg -Path 'HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Key 'verbosestatus' -Value '1' -Type 'DWord'

# Do not suggest ways to "finish setting up my device to get the most out of Windows".
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement' -Key 'ScoobeSystemSettingEnabled' -Value '0' -Type 'DWord'

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
SetReg -Path 'HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Key 'Start_IrisRecommendations' -Value '0' -Type 'DWord'

# All background images are converted to JPEG, so avoid its compression as much as possible.
SetReg -Path 'HKEY_CURRENT_USER\Control Panel\Desktop' -Key 'JPEGImportQuality' -Value '100' -Type 'DWord'

# Disable feedback reminders.
SetReg -Path 'HKEY_CURRENT_USER\SOFTWARE\Microsoft\Siuf\Rules' -Key 'NumberOfSIUFInPeriod' -Value '0' -Type 'DWord'
SetReg -Path 'HKEY_CURRENT_USER\SOFTWARE\Microsoft\Siuf\Rules' -Key 'PeriodInNanoSeconds' -Value '0' -Type 'DWord'
