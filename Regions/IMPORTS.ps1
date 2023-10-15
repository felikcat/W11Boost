#Requires -Version 5 -RunAsAdministrator

# 'Import-Module example.psm1' fails if PowerShell script execution is disabled; do it manually.
#Unblock-File -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll"
#Add-Type -Path "..\Third-party\PolicyFileEditor\PolFileEditor.dll" -ErrorAction Stop
#. "..\Third-party\PolicyFileEditor\Commands.ps1"

function Download_File {
    Start-BitsTransfer -MaxDownloadTime 120 -RetryInterval 60 -RetryTimeout 300 -TransferPolicy Unrestricted -Source @args
}

# function PEAdd_HKCU {
#     Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\User\registry.pol" -Key @args
# }

# function PEAdd_HKLM {
#     Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\Machine\registry.pol" -Key @args
# }

# function PERemove_HKCU {
#     Remove-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\User\registry.pol" -Key @args
# }

# function PERemove_HKLM {
#     Remove-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\Machine\registry.pol" -Key @args
# }

function NewToastNotification {
    [cmdletbinding()]
    Param (
        [string]
        $ToastTitle,
        [string]
        [parameter(ValueFromPipeline)]
        $ToastText
    )

    [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null
    $Template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)

    $RawXml = [xml] $Template.GetXml()
    ($RawXml.toast.visual.binding.text | Where-Object {$_.id -eq "1"}).AppendChild($RawXml.CreateTextNode($ToastTitle)) > $null
    ($RawXml.toast.visual.binding.text | Where-Object {$_.id -eq "2"}).AppendChild($RawXml.CreateTextNode($ToastText)) > $null

    $SerializedXml = New-Object Windows.Data.Xml.Dom.XmlDocument
    $SerializedXml.LoadXml($RawXml.OuterXml)

    $Toast = [Windows.UI.Notifications.ToastNotification]::new($SerializedXml)
    $Toast.Tag = "PowerShell"
    $Toast.Group = "PowerShell"
    $Toast.ExpirationTime = [DateTimeOffset]::Now.AddMinutes(1)

    $Notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("PowerShell")
    $Notifier.Show($Toast);
}
