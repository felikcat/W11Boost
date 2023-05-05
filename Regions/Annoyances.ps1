wmic.exe UserAccount set PasswordExpires=False

# Disable "Title bar window shake", previously called "Aero shake".
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Policies\Microsoft\Windows\Explorer' -ValueName 'NoWindowMinimizingShortcuts' -Data '1' -Type 'Dword'

# If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Appx' -ValueName 'AllowAutomaticAppArchiving' -Data '0' -Type 'Dword'

# Skip to the sign-on screen.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Personalization' -ValueName 'NoLockScreen' -Data '1' -Type 'Dword'

# Disable "Look for an app in the Microsoft Store" or "Browse apps in the Microsoft Store".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\Explorer' -ValueName 'NoUseStoreOpenWith' -Data '1' -Type 'Dword'

# Do not show Windows tips.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\CloudContent' -ValueName 'DisableSoftLanding' -Data '1' -Type 'Dword'

# Disable optional driver updates; tends to be very outdated.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'ExcludeWUDriversInQualityUpdate' -Data '1' -Type 'Dword'

# Show what's slowing down bootups and shutdowns.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ValueName 'verbosestatus' -Data '1' -Type 'Dword'

# Don't suggest ways to "finish setting up my device to get the most out of Windows".
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\UserProfileEngagement' -ValueName 'ScoobeSystemSettingEnabled' -Data '0' -Type 'Dword'

# Disables the startup sound; why: https://youtu.be/UWUBjM2LNJU?t=772
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\Windows\CurrentVersion\Authentication\LogonUI\BootAnimation' -ValueName 'DisableStartupSound' -Data '1' -Type 'Dword'

# Disable "Show recommendations for tips, shortcuts, new apps, and more".
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced' -ValueName 'Start_IrisRecommendations' -Data '0' -Type 'Dword'


##+=+= Make automatic Windows updates tolerable.

# Hold back updates if Microsoft is aware they cause compatibility issues.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'DisableWUfBSafeguards' -Data '0' -Type 'Dword'

# Opt out out of "being the first to get the latest non-security updates".
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Microsoft\WindowsUpdate\UX\Settings' -ValueName 'IsContinuousInnovationOptedIn' -Data '0' -Type 'Dword'

# Notify for downloading and installing Windows updates.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'AUOptions' -Data '2' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate' -ValueName 'AllowAutoWindowsUpdateDownloadOverMeteredNetwork' -Data '0' -Type 'Dword'

# Make restarts never forced.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'NoAutoUpdate' -Data '0' -Type 'Dword'

# Disable feedback reminders
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Siuf\Rules' -ValueName 'NumberOfSIUFInPeriod' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_USER -Key 'Software\Microsoft\Siuf\Rules' -ValueName 'PeriodInNanoSeconds' -Data '0' -Type 'Dword'

# Prompt to install updates every Sunday at 03:00/3:00AM.
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'ScheduledInstallDay' -Data '1' -Type 'Dword'
Set-PolicyFileEntry -Path $PREG_MACHINE -Key 'SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU' -ValueName 'ScheduledInstallTime' -Data '3' -Type 'Dword'

Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdates"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\InstallService\ScanForUpdatesAsUser"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan Static Task"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\Schedule Scan"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\UpdateModelTask"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\UpdateOrchestrator\USO_UxBroker"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\Scheduled Start"
Disable-ScheduledTask -TaskName "\Microsoft\Windows\WindowsUpdate\sih"
##+=+=

