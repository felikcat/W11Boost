#Requires -Version 5 -RunAsAdministrator

$WIN_BUILD = Get-ItemPropertyValue 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion' -Name CurrentBuildNumber

function Download_File {
    Start-BitsTransfer -MaxDownloadTime 120 -RetryInterval 60 -RetryTimeout 300 -TransferPolicy Unrestricted -Source @args
}

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
    ($RawXml.toast.visual.binding.text | Where-Object { $_.id -eq "1" }).AppendChild($RawXml.CreateTextNode($ToastTitle)) > $null
    ($RawXml.toast.visual.binding.text | Where-Object { $_.id -eq "2" }).AppendChild($RawXml.CreateTextNode($ToastText)) > $null

    $SerializedXml = New-Object Windows.Data.Xml.Dom.XmlDocument
    $SerializedXml.LoadXml($RawXml.OuterXml)

    $Toast = [Windows.UI.Notifications.ToastNotification]::new($SerializedXml)
    $Toast.Tag = "PowerShell"
    $Toast.Group = "PowerShell"
    $Toast.ExpirationTime = [DateTimeOffset]::Now.AddMinutes(1)

    $Notifier = [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier("PowerShell")
    $Notifier.Show($Toast);
}

function SetReg {
    param(
        [String]$Path,
        [String]$Key,
        [String]$Value,
        [Microsoft.Win32.RegistryValueKind]$Type
    )
    try {
        [Microsoft.Win32.Registry]::SetValue($Path, $Key, $Value, $Type)
        Add-Content -Path "${HOME}\Desktop\W11Boost logs\Registry.log" -Value "Registry key set`nPath: $Path, Key: $Key`nValue: $Value`n"
    }
    catch {
        Add-Content -Path "${HOME}\Desktop\W11Boost logs\Registry.log" -Value "Error setting registry key`nPath: $Path, Key: $Key`nValue: $Value`n"
    }
}