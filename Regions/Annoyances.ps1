#Requires -Version 5 -RunAsAdministrator

wmic.exe UserAccount set PasswordExpires=False

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Appx' -Name 'AllowAutomaticAppArchiving' -Value '0' -Type 'Dword'

# Skip to the sign-on screen.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Personalization' -Name 'NoLockScreen' -Value '1' -Type 'Dword'

# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Explorer' -Name 'NoUseStoreOpenWith' -Value '1' -Type 'Dword'

# Do not show Windows tips.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Name 'DisableSoftLanding' -Value '1' -Type 'Dword'

PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -Name 'DisableTailoredExperiencesWithDiagnosticData' -Value '1' -Type 'Dword'

# Show what's slowing down bootups and shutdowns.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'verbosestatus' -Value '1' -Type 'Dword'

# Do not suggest ways to "finish setting up my device to get the most out of Windows".
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement' -Name 'ScoobeSystemSettingEnabled' -Value '0' -Type 'Dword'

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'Start_IrisRecommendations' -Value '0' -Type 'Dword'

# All background images are converted to JPEG, so avoid its compression as much as possible.
PEAdd_HKCU 'Control Panel\Desktop' -Name 'JPEGImportQuality' -Value '100' -Type 'Dword'

# Disable feedback reminders.
PEAdd_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -Name 'NumberOfSIUFInPeriod' -Value '0' -Type 'Dword'
PEAdd_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -Name 'PeriodInNanoSeconds' -Value '0' -Type 'Dword'
