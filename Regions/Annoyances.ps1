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

# Disable optional driver updates; tends to be very outdated.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'ExcludeWUDriversInQualityUpdate' -Value '1' -Type 'Dword'

# Show what's slowing down bootups and shutdowns.
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -Name 'verbosestatus' -Value '1' -Type 'Dword'

# Do not suggest ways to "finish setting up my device to get the most out of Windows".
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement' -Name 'ScoobeSystemSettingEnabled' -Value '0' -Type 'Dword'

# Disables the startup sound; why: https://youtu.be/UWUBjM2LNJU?t=772
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\EditionOverrides' -Name 'UserSetting_DisableStartupSound' -Value '1' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\BootAnimation' -Name 'DisableStartupSound' -Value '1' -Type 'Dword'

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
PEAdd_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -Name 'Start_IrisRecommendations' -Value '0' -Type 'Dword'


##+=+= Make Windows Update tolerable.

# Block updates that Microsoft deems as causing compatibility issues.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'DisableWUfBSafeguards' -Value '0' -Type 'Dword'

# Opt out out of "being the first to get the latest non-security updates".
PEAdd_HKLM 'SOFTWARE\Microsoft\WindowsUpdate\UX\Settings' -Name 'IsContinuousInnovationOptedIn' -Value '0' -Type 'Dword'

# Notify to download then install Windows updates; no automatic Windows updates.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Name 'AUOptions' -Value '2' -Type 'Dword'
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -Name 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork' -Value '0' -Type 'Dword'

# Never force restarts.
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -Name 'NoAutoUpdate' -Value '0' -Type 'Dword'

# Disable Delivery Optimization's "Allow downloads from other PCs".
PEAdd_HKLM 'SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization' -Name 'DODownloadMode' -Value '0' -Type 'Dword'
##+=+=


# Disable feedback reminders.
PEAdd_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -Name 'NumberOfSIUFInPeriod' -Value '0' -Type 'Dword'
PEAdd_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -Name 'PeriodInNanoSeconds' -Value '0' -Type 'Dword'
