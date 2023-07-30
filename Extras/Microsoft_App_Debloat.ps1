#Requires -Version 5 -RunAsAdministrator

# Microsoft's Publisher ID.
$ID="8wekyb3d8bbwe"

# Do not remove the following, as there's no getting them back through normal means:
# Microsoft.XboxSpeechToTextOverlay_$ID

# Clipchamp - Video Editor | Paint
# Cortana | Microsoft News
# MSN Weather | Xbox
# Get Help | Microsoft Tips
# Office | Microsoft Solitaire Collection
# Microsoft Sticky Notes | OneDrive
# OneDrive Sync | Power Automate
# Microsoft To Do: Lists, Tasks & Reminders | Microsoft Photos
# Mail and Calendar | Feedback Hub
# Windows Voice Recorder
# Xbox Live in-game experience | Xbox Game Bar Plugin
# Xbox Game Bar | Xbox Identity Provider
# Windows Media Player (not the same as Windows 7's WMP / Legacy WMP)
# Movies & TV | Quick Assist
# Microsoft Teams | Windows Web Experience Pack
# Snipping Tool
$APPS = @("Clipchamp.Clipchamp_yxz26nhyzhsrt", "Microsoft.Paint_$ID",
"Microsoft.549981C3F5F10_$ID", "Microsoft.BingNews_$ID",
"Microsoft.BingWeather_$ID", "Microsoft.GamingApp_$ID",
"Microsoft.GetHelp_$ID", "Microsoft.Getstarted_$ID",
"Microsoft.MicrosoftOfficeHub_$ID", "Microsoft.MicrosoftSolitaireCollection_$ID",
"Microsoft.MicrosoftStickyNotes_$ID", "Microsoft.OneDrive",
"Microsoft.OneDriveSync_$ID", "Microsoft.PowerAutomateDesktop_$ID",
"Microsoft.Todos_$ID", "Microsoft.Windows.Photos_$ID",
"microsoft.windowscommunicationsapps_$ID", "Microsoft.WindowsFeedbackHub_$ID",
"Microsoft.WindowsSoundRecorder_$ID",
"Microsoft.Xbox.TCUI_$ID", "Microsoft.XboxGameOverlay_$ID",
"Microsoft.XboxGamingOverlay_$ID", "Microsoft.XboxIdentityProvider_$ID",
"Microsoft.ZuneMusic_$ID",
"Microsoft.ZuneVideo_$ID", "MicrosoftCorporationII.QuickAssist_$ID",
"MicrosoftTeams_$ID", "MicrosoftWindows.Client.WebExperience_cw5n1h2txyewy",
"Microsoft.ScreenSketch_$ID")

$APPS.ForEach({
    winget.exe uninstall $_ --exact --silent --accept-source-agreements
})
