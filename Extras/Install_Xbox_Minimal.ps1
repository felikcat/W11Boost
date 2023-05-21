if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
	Write-Warning "ERROR: Install_Xbox_Minimal.ps1 -> Requires Administrator!"
	Break
}

# Installations are within order of dependence. Gaming Services requires Xbox Identity Provider, and so on.
# https://apps.microsoft.com/store/detail/xbox-identity-provider/9WZDNCRD1HKW
# https://apps.microsoft.com/store/detail/gaming-services/9MWPM2CQNLHN
# https://apps.microsoft.com/store/detail/xbox-live-ingame-experience/9NKNC0LD5NN6
# Xbox Gaming App: https://www.microsoft.com/store/productId/9MV0B5HZVK9Z
$apps = @("9WZDNCRD1HKW", "9MWPM2CQNLHN", "9NKNC0LD5NN6", "9MV0B5HZVK9Z")

for ($i = 0; $i -lt $apps.length; $i++) {
    winget.exe install $apps[$i] --exact --silent --accept-package-agreements --accept-source-agreements --source msstore
}
