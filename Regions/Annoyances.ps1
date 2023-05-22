#Requires -Version 5

wmic.exe UserAccount set PasswordExpires=False

# Disable "Title bar window shake", previously called "Aero shake".
PolEdit_HKCU 'Software\Policies\Microsoft\Windows\Explorer' -ValueName 'NoWindowMinimizingShortcuts' -Data '1' -Type 'Dword'

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowAutomaticAppArchiving' -Data '0' -Type 'Dword'

# Skip to the sign-on screen.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Personalization' -ValueName 'NoLockScreen' -Data '1' -Type 'Dword'

# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\Explorer' -ValueName 'NoUseStoreOpenWith' -Data '1' -Type 'Dword'

# Do not show Windows tips.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableSoftLanding' -Data '1' -Type 'Dword'

# Disable optional driver updates; tends to be very outdated.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'ExcludeWUDriversInQualityUpdate' -Data '1' -Type 'Dword'

# Show what's slowing down bootups and shutdowns.
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'verbosestatus' -Data '1' -Type 'Dword'

# Don't suggest ways to "finish setting up my device to get the most out of Windows".
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement' -ValueName 'ScoobeSystemSettingEnabled' -Data '0' -Type 'Dword'

# Disables the startup sound; why: https://youtu.be/UWUBjM2LNJU?t=772
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\EditionOverrides' -ValueName 'UserSetting_DisableStartupSound' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\BootAnimation' -ValueName 'DisableStartupSound' -Data '1' -Type 'Dword'

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
PolEdit_HKCU 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Start_IrisRecommendations' -Data '0' -Type 'Dword'


##+=+= Make automatic Windows updates tolerable.

# Block updates that Microsoft deems as causing compatibility issues.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'DisableWUfBSafeguards' -Data '0' -Type 'Dword'

# Opt out out of "being the first to get the latest non-security updates".
PolEdit_HKLM 'SOFTWARE\Microsoft\WindowsUpdate\UX\Settings' -ValueName 'IsContinuousInnovationOptedIn' -Data '0' -Type 'Dword'

# Notify to download then install Windows updates; no automatic Windows updates.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'AUOptions' -Data '2' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork' -Data '0' -Type 'Dword'

# Never force restarts.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'NoAutoUpdate' -Data '0' -Type 'Dword'

# Disable feedback reminders.
PolEdit_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -ValueName 'NumberOfSIUFInPeriod' -Data '0' -Type 'Dword'
PolEdit_HKCU 'SOFTWARE\Microsoft\Siuf\Rules' -ValueName 'PeriodInNanoSeconds' -Data '0' -Type 'Dword'

# Prompt to install updates every Sunday at 03:00/3:00AM.
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'ScheduledInstallDay' -Data '1' -Type 'Dword'
PolEdit_HKLM 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'ScheduledInstallTime' -Data '3' -Type 'Dword'

##+=+=

