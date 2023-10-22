#Requires -Version 5
#region Initialize
using assembly System.Windows.Forms
using namespace System.Windows.Forms
using namespace System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"
New-Item -ItemType Directory "${HOME}\Desktop\W11Boost logs"
#endregion

[Application]::EnableVisualStyles()
$code = @"
    [System.Runtime.InteropServices.DllImport("user32.dll")]
    public static extern bool SetProcessDPIAware();
"@
$Win32Helpers = Add-Type -MemberDefinition $code -Name "Win32Helpers" -PassThru
$null = $Win32Helpers::SetProcessDPIAware()

$Form = New-Object Form -Property @{
    StartPosition   = [FormStartPosition]::CenterScreen
    Topmost         = $true
    MaximizeBox     = $false
    FormBorderStyle = "FixedDialog"
    MinimumSize     = New-Object Drawing.Size 800, 600
    MaximumSize     = New-Object Drawing.Size 800, 600
    Text            = "W11Boost"
    Font            = New-Object Font("Segoe UI", 16)
}

$ExtrasButton = New-Object Button -Property @{
    Height = 181
    Dock   = 'Bottom'
    Text   = "Extras"
}

$UninstallButton = New-Object Button -Property @{
    Height = 181
    Dock   = 'Bottom'
    Text   = "Remove W11Boost"
}

$InstallOnlyButton = New-Object Button -Property @{
    Height = 181
    Dock   = 'Bottom'
    Text   = "Install W11Boost"
}

function PleaseWaitText {
    $Form.Controls.Clear()

    $Label = New-Object Label -Property @{
        Dock     = 'Top'
        AutoSize = $true
        Text     = "Please wait, applying changes...
Temporarily this will appear frozen, this is normal."
    }
    $Form.Controls.Add($Label)
}

function FirstWindowControls {
    $Form.Controls.Clear()
    $Form.Controls.AddRange(($InstallOnlyButton, $ExtrasButton, $UninstallButton))
}

function ExtrasWindow {
    $Form.Controls.Clear()

    $DebloatWindowsButton = New-Object Button -Property @{
        Dock   = 'Bottom'
        Height = 109
        Text   = "Debloat Windows"
    }

    $InstallXboxMinimalButton = New-Object Button -Property @{
        Dock   = 'Bottom'
        Height = 109
        Text   = "Install Xbox services"
    }

    $ApplyStigsButton = New-Object Button -Property @{
        Dock   = 'Bottom'
        Height = 109
        Text   = "Install STIG policies"
    }

    $HardenFurtherButton = New-Object Button -Property @{
        Dock   = 'Bottom'
        Height = 109
        Text   = "Harden further (use after STIGs)"
    }

    $Form.Controls.AddRange(@($DebloatWindowsButton, $InstallXboxMinimalButton,
            $ApplyStigsButton, $HardenFurtherButton, $GoBackButton ))

    $DebloatWindowsButton.Add_Click({
            $Prompt = [MessageBox]::Show("This will uninstall some built-in Windows applications, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

            if ($Prompt -eq "Y") {
                PleaseWaitText

                & ".\..\Extras\Microsoft_App_Debloat.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Microsoft App Debloat.log"

                NewToastNotification "Removal of Microsoft's bloatware has finished." -ToastTitle "W11Boost"
                
                ExtrasWindow
            }
        })

    $InstallXboxMinimalButton.Add_Click({
            $Prompt = [MessageBox]::Show("This will install Xbox applications and services, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

            if ($Prompt -eq "Y") {
                PleaseWaitText

                & ".\..\Extras\Install_Xbox_Minimal.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Install Xbox Minimal.log"

                NewToastNotification "Installation of Xbox apps and services are complete." -ToastTitle "W11Boost"

                ExtrasWindow
            }
        })

    $ApplyStigsButton.Add_Click({
            $Prompt = [MessageBox]::Show("This will extract the STIGs then install all of them. Are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

            if ($Prompt -eq "Y") {
                PleaseWaitText

                $STIG_NAME = "U_STIG_GPO_Package_August_2023"
                $STIG_HASH = (Get-FileHash -Algorithm SHA256 "..\Third-party\DoD-STIGS\$STIG_NAME.zip").Hash
                $EXPECTED_HASH = "B2382E3208CDC86741113E3FBD30EEAF8532DB94B68196A9E9291F441E87766A"

                # Source: https://public.cyber.mil/stigs/gpo/
                if ($STIG_HASH -ne $EXPECTED_HASH) {
                    [MessageBox]::Show("STIGs did not match the expected SHA256 file hash; stopping now to prevent potential security risks.", "W11Boost", [MessageBoxButtons]::OK)
                }
                else {
                    & ".\..\Extras\Apply_STIGs.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Apply STIGs.log"
                    [MessageBox]::Show($Form, "STIGs have been applied. Please reboot manually for the changes to take effect.", "W11Boost", [MessageBoxButtons]::OK)
                }

                ExtrasWindow
            }
        })
    $HardenFurtherButton.Add_Click({
            $Prompt = [MessageBox]::Show("This will break older applications. Are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

            if ($Prompt -eq "Y") {
                PleaseWaitText

                & ".\..\Extras\Harden_Further.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Harden further.log"

                [MessageBox]::Show($Form, "Additional hardening complete. Please reboot manually for the changes to take effect.", "W11Boost", [MessageBoxButtons]::OK)

                ExtrasWindow
            }
        })
}

#region Draw initial Form/Window and setup button actions.
FirstWindowControls

$ExtrasButton.Add_Click({ ExtrasWindow })
$UninstallButton.Add_Click({
        $Prompt = [MessageBox]::Show("This will uninstall W11Boost, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

        if ($Prompt -eq "Y") {
            PleaseWaitText

            & ".\..\Extras\Uninstall.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Uninstall.log"

            [MessageBox]::Show($Form, "Removal of W11Boost nearly complete; manually reboot to finish.", "W11Boost", [MessageBoxButtons]::OK)

            FirstWindowControls
        }
    })

$GoBackButton = New-Object Button -Property @{
    Dock   = 'Bottom'
    Height = 109
    Text   = "<- Go back to prior screen"
}
$GoBackButton.Add_Click({ FirstWindowControls })

$InstallOnlyButton.Add_Click({
        $Prompt = [MessageBox]::Show("This will install W11Boost, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)

        if ($Prompt -eq "Y") {
            PleaseWaitText

            & ".\Main.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Main.log"
            [MessageBox]::Show($Form, "Installation nearly complete; manually reboot to finish.", "W11Boost", [MessageBoxButtons]::OK)

            FirstWindowControls
        }
    })

$Form.ShowDialog()
#endregion
