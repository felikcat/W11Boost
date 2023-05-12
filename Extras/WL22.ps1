if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
	Write-Warning "ERROR: WL22.ps1 -> Requires Administrator!"
	Break
}

# Microsoft's Publisher ID.
$ID="8wekyb3d8bbwe"

# Don't remove the following, as there's no getting them back through normal means:
# Microsoft.XboxSpeechToTextOverlay_$ID

# Clipchamp - Video Editor | Paint
# Cortana | Microsoft News
# MSN Weather | Xbox
# Get Help | Microsoft Tips
# Office | Microsoft Solitaire Collection
# Microsoft Sticky Notes | OneDrive
# OneDrive Sync | Power Automate
# Microsoft To Do: Lists, Tasks & Reminders
# Windows Camera
# Mail and Calendar | Feedback Hub
# Windows Maps | Windows Voice Recorder
# Xbox Live in-game experience | Xbox Game Bar Plugin
# Xbox Game Bar | Xbox Identity Provider
# Phone Link | Windows Media Player (not the same as Windows 7's WMP)
# Movies & TV | Quick Assist
# Microsoft Teams | Windows Web Experience Pack
# Notepad
$apps = @("Clipchamp.Clipchamp_yxz26nhyzhsrt", "Microsoft.Paint_$ID",
"Microsoft.549981C3F5F10_$ID", "Microsoft.BingNews_$ID",
"Microsoft.BingWeather_$ID", "Microsoft.GamingApp_$ID",
"Microsoft.GetHelp_$ID", "Microsoft.Getstarted_$ID",
"Microsoft.MicrosoftOfficeHub_$ID", "Microsoft.MicrosoftSolitaireCollection_$ID",
"Microsoft.MicrosoftStickyNotes_$ID", "Microsoft.OneDrive",
"Microsoft.OneDriveSync_$ID", "Microsoft.PowerAutomateDesktop_$ID",
"Microsoft.Todos_$ID",
"Microsoft.WindowsCamera_$ID",
"microsoft.windowscommunicationsapps_$ID", "Microsoft.WindowsFeedbackHub_$ID",
"Microsoft.WindowsMaps_$ID", "Microsoft.WindowsSoundRecorder_$ID",
"Microsoft.Xbox.TCUI_$ID", "Microsoft.XboxGameOverlay_$ID",
"Microsoft.XboxGamingOverlay_$ID", "Microsoft.XboxIdentityProvider_$ID",
"Microsoft.YourPhone_$ID", "Microsoft.ZuneMusic_$ID",
"Microsoft.ZuneVideo_$ID", "MicrosoftCorporationII.QuickAssist_$ID",
"MicrosoftTeams_$ID", "MicrosoftWindows.Client.WebExperience_cw5n1h2txyewy",
"Microsoft.WindowsNotepad_$ID")

for ($i = 0; $i -lt $apps.length; $i++) {
    winget.exe uninstall $apps[$i] --exact --silent --accept-source-agreements
}
