if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Warning "ERROR: Remove_Harmful_Software.ps1 -> Requires Administrator!"
    Break
}

$apps = @("Razer Synapse", "Corsair.iCUE.4", "Corsair.iCUE.3", "")

for ($i = 0; $i -lt $apps.length; $i++) {
    winget.exe uninstall $apps[$i] --exact --silent --accept-source-agreements --force --disable-interactivity
}