#Requires -Version 5
using assembly System.Windows.Forms
using namespace System.Windows.Forms
using namespace System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

Push-Location $PSScriptRoot
. ".\IMPORTS.ps1"
New-Item -ItemType Directory "${HOME}\Desktop\W11Boost logs"

[Application]::EnableVisualStyles()
$code = @"
    [System.Runtime.InteropServices.DllImport("user32.dll")]
    public static extern bool SetProcessDPIAware();
"@
$Win32Helpers = Add-Type -MemberDefinition $code -Name "Win32Helpers" -PassThru
$null = $Win32Helpers::SetProcessDPIAware()

$Form = New-Object Form -Property @{
    StartPosition   = [FormStartPosition]::CenterScreen
    MaximizeBox     = $false
    FormBorderStyle = "FixedDialog"
    Topmost         = $true
    MinimumSize     = New-Object Drawing.Size 640, 480
    MaximumSize     = New-Object Drawing.Size 640, 480
    Padding         = 500

    Text            = "W11Boost"
    Font            = New-Object Font("Segoe UI", 16)
}

$ExtrasButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .01 ))
    Size     = New-Object Drawing.Size       (($Form.Width * .5), ($Form.Height * .425))
    Text     = "Extras"
}

$UninstallButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point    (($Form.Width * .00725), ($Form.Height * .45 ))
    Size     = New-Object Drawing.Size         (($Form.Width * .5), ($Form.Height * .425))
    Text     = "Remove W11Boost"
}

$InstallOnlyButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point    (($Form.Width * .511), ($Form.Height * .07 ))
    Size     = New-Object Drawing.Size         (($Form.Width * .45), ($Form.Height * .75 ))
    Text     = "Install W11Boost"
}

function PleaseWaitText {
    $Form.Controls.Clear()

    $Label = New-Object Label -Property @{
        Location = New-Object Drawing.Point (($Form.Width * .35 ), ($Form.Height * .35))
        AutoSize = $true
        Text     = "Please wait..."
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
        Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .01 ))
        Size     = New-Object Drawing.Size      (($Form.Width * .45), ($Form.Height * .25))
        Text     = "Debloat Windows"
    }

    $InstallXboxMinimalButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .275 ))
        Size     = New-Object Drawing.Size      (($Form.Width * .45), ($Form.Height * .25))
        Text     = "Install Xbox services"
    }

    $ApplyStigsButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .51), ($Form.Height * .01 ))
        Size     = New-Object Drawing.Size  (($Form.Width * .45), ($Form.Height * .25))
        Text     = "Install STIG policies"
    }

    $Form.Controls.AddRange(@($DebloatWindowsButton, $InstallXboxMinimalButton,
     $ApplyStigsButton, $GoBackButton ))

    $DebloatWindowsButton.Add_Click({
            PleaseWaitText

            & ".\..\Extras\Microsoft_App_Debloat.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Microsoft App Debloat.log"

            NewToastNotification "Removal of Microsoft's bloatware has finished." -ToastTitle "W11Boost"
            ExtrasWindow
        })

    $InstallXboxMinimalButton.Add_Click({
            PleaseWaitText

            & ".\..\Extras\Install_Xbox_Minimal.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Install Xbox Minimal.log"

            NewToastNotification "Installation of Xbox apps and services are complete." -ToastTitle "W11Boost"
            ExtrasWindow
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
            } else {
                    & ".\..\Extras\Apply_STIGs.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Apply STIGs.log"
                    [MessageBox]::Show($Form, "STIGs have been applied. Please reboot manually for the changes to take effect.", "W11Boost", [MessageBoxButtons]::OK)
            }
        }
        ExtrasWindow
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
    Location = New-Object Drawing.Point  (($Form.Width * .51), ($Form.Height * .55 ))
    Size     = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
    Text     = "<- Go back to prior screen"
}
$GoBackButton.Add_Click({ FirstWindowControls })

$InstallOnlyButton.Add_Click({
        PleaseWaitText

        & ".\Main.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Main.log"
        [MessageBox]::Show($Form, "Installation nearly complete; manually reboot to finish.", "W11Boost", [MessageBoxButtons]::OK)

        FirstWindowControls
    })

$Form.ShowDialog()
#endregion
