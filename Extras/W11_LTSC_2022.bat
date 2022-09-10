@echo on
set PARMS=--exact --silent --accept-source-agreements

REM Clipchamp - Video Editor
winget.exe uninstall Clipchamp.Clipchamp_yxz26nhyzhsrt %PARMS%
REM Disney+
winget.exe uninstall Disney.37853FC22B2CE_6rarf9sa4v8jt %PARMS%
REM Cortana
winget.exe uninstall Microsoft.549981C3F5F10_8wekyb3d8bbwe %PARMS%
REM Microsoft News
winget.exe uninstall Microsoft.BingNews_8wekyb3d8bbwe %PARMS%
REM MSN Weather
winget.exe uninstall Microsoft.BingWeather_8wekyb3d8bbwe %PARMS%
REM Xbox
winget.exe uninstall Microsoft.GamingApp_8wekyb3d8bbwe %PARMS%
REM Get Help
winget.exe uninstall Microsoft.GetHelp_8wekyb3d8bbwe %PARMS%
REM Microsoft Tips
winget.exe uninstall Microsoft.Getstarted_8wekyb3d8bbwe %PARMS%
REM Office
winget.exe uninstall Microsoft.MicrosoftOfficeHub_8wekyb3d8bbwe %PARMS%
REM Microsoft Solitaire Collection
winget.exe uninstall Microsoft.MicrosoftSolitaireCollection_8wekyb3d8bbwe %PARMS%
REM Microsoft Sticky Notes
winget.exe uninstall Microsoft.MicrosoftStickyNotes_8wekyb3d8bbwe %PARMS%

REM OneDrive
winget.exe uninstall Microsoft.OneDrive %PARMS%
REM OneDrive Sync
winget.exe uninstall Microsoft.OneDriveSync_8wekyb3d8bbwe %PARMS%

REM Power Automate
winget.exe uninstall Microsoft.PowerAutomateDesktop_8wekyb3d8bbwe %PARMS%
REM Microsoft To Do: Lists, Tasks & Reminders
winget.exe uninstall Microsoft.Todos_8wekyb3d8bbwe %PARMS%
REM Microsoft Photos
winget.exe uninstall Microsoft.Windows.Photos_8wekyb3d8bbwe %PARMS%
REM Windows Clock
winget.exe uninstall Microsoft.WindowsAlarms_8wekyb3d8bbwe %PARMS%
REM Windows Camera
winget.exe uninstall Microsoft.WindowsCamera_8wekyb3d8bbwe %PARMS%
REM Mail and Calendar
winget.exe uninstall microsoft.windowscommunicationsapps_8wekyb3d8bbwe %PARMS%
REM Feedback Hub
winget.exe uninstall Microsoft.WindowsFeedbackHub_8wekyb3d8bbwe %PARMS%
REM Windows Maps
winget.exe uninstall Microsoft.WindowsMaps_8wekyb3d8bbwe %PARMS%
REM Windows Voice Recorder
winget.exe uninstall Microsoft.WindowsSoundRecorder_8wekyb3d8bbwe %PARMS%

REM Xbox Live in-game experience
winget.exe uninstall Microsoft.Xbox.TCUI_8wekyb3d8bbwe %PARMS%
REM Xbox Game Bar Plugin
winget.exe uninstall Microsoft.XboxGameOverlay_8wekyb3d8bbwe %PARMS%
REM Xbox Game Bar
winget.exe uninstall Microsoft.XboxGamingOverlay_8wekyb3d8bbwe %PARMS%
REM Xbox Identity Provider
winget.exe uninstall Microsoft.XboxIdentityProvider_8wekyb3d8bbwe %PARMS%

REM Phone Link
winget.exe uninstall Microsoft.YourPhone_8wekyb3d8bbwe %PARMS%
REM Windows Media Player (note: not the same Windows Media Player from Windows 7)
winget.exe uninstall Microsoft.ZuneMusic_8wekyb3d8bbwe %PARMS%
REM Movies & TV
winget.exe uninstall Microsoft.ZuneVideo_8wekyb3d8bbwe %PARMS%
REM Quick Assist
winget.exe uninstall MicrosoftCorporationII.QuickAssist_8wekyb3d8bbwe %PARMS%
REM Microsoft Teams
winget.exe uninstall MicrosoftTeams_8wekyb3d8bbwe %PARMS%
REM Windows Web Experience Pack
winget.exe uninstall MicrosoftWindows.Client.WebExperience_cw5n1h2txyewy %PARMS%
REM Spotify - Music and Podcasts
winget.exe uninstall SpotifyAB.SpotifyMusic_zpdnekdrzrea0 %PARMS%

REM Don't remove the following, as there's no getting them back through normal means:
REM Microsoft.XboxSpeechToTextOverlay_8wekyb3d8bbwe