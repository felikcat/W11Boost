@echo on
set PARMS=--exact --silent --accept-source-agreements

REM Installations are within order of dependence. Gaming Services requires Xbox Identity Provider, and so on.

REM https://apps.microsoft.com/store/detail/xbox-identity-provider/9WZDNCRD1HKW
winget.exe install 9WZDNCRD1HKW %PARMS%

REM https://apps.microsoft.com/store/detail/gaming-services/9MWPM2CQNLHN
winget.exe install 9MWPM2CQNLHN %PARMS%

REM https://apps.microsoft.com/store/detail/xbox-live-ingame-experience/9NKNC0LD5NN6
winget.exe install 9NKNC0LD5NN6 %PARMS%

REM Xbox Gaming App: https://www.microsoft.com/store/productId/9MV0B5HZVK9Z
winget.exe install 9MV0B5HZVK9Z %PARMS%

