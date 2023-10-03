#Requires -Version 5 -RunAsAdministrator

function Download_File {
    Start-BitsTransfer -MaxDownloadTime 120 -RetryInterval 60 -RetryTimeout 300 -TransferPolicy Unrestricted -Source @args
}

function PEAdd_HKCU {
    Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\User\registry.pol" -Key @args
}

function PEAdd_HKLM {
    Set-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\Machine\registry.pol" -Key @args
}

function PERemove_HKCU {
    Remove-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\User\registry.pol" -Key @args
}

function PERemove_HKLM {
    Remove-PolicyFileEntry -Path "$env:windir\System32\GroupPolicy\Machine\registry.pol" -Key @args
}