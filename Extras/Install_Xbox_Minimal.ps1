#Requires -Version 5 -RunAsAdministrator

# Installations are within order of dependence. Gaming Services requires Xbox Identity Provider, and so on.
# https://apps.microsoft.com/store/detail/xbox-identity-provider/9WZDNCRD1HKW
# https://apps.microsoft.com/store/detail/gaming-services/9MWPM2CQNLHN
# https://apps.microsoft.com/store/detail/xbox-live-ingame-experience/9NKNC0LD5NN6
# Xbox Gaming App: https://www.microsoft.com/store/productId/9MV0B5HZVK9Z
$_apps = @("9WZDNCRD1HKW", "9MWPM2CQNLHN", "9NKNC0LD5NN6", "9MV0B5HZVK9Z")
$_apps.ForEach({
    winget.exe install $_ --exact --silent --accept-package-agreements --accept-source-agreements --source msstore
})
