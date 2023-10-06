#Requires -Version 5
using assembly System.Windows.Forms
using namespace System.Windows.Forms
using namespace System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
Push-Location $PSScriptRoot

New-Item -ItemType Directory "${HOME}\Desktop\W11Boost logs"

[Application]::EnableVisualStyles()
$code = @"
    [System.Runtime.InteropServices.DllImport("user32.dll")]
    public static extern bool SetProcessDPIAware();
"@
$Win32Helpers = Add-Type -MemberDefinition $code -Name "Win32Helpers" -PassThru
$null = $Win32Helpers::SetProcessDPIAware()

$Form = New-Object Form -Property @{
    StartPosition = [FormStartPosition]::CenterScreen
    MaximizeBox = $false
    FormBorderStyle = "FixedDialog"
    Topmost = $true
    MinimumSize = New-Object Drawing.Size 640, 480
    MaximumSize = New-Object Drawing.Size 640, 480
    Padding = 500

    Text = "W11Boost"
    Font = New-Object Font("Segoe UI", 16)
}

$ExtrasButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .01 ))
    Size = New-Object Drawing.Size       (($Form.Width * .5),     ($Form.Height * .425))
    Text = "Extras"
}

$UninstallButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point    (($Form.Width * .00725), ($Form.Height * .45 ))
    Size = New-Object Drawing.Size         (($Form.Width * .5),     ($Form.Height * .425))
    Text = "Remove W11Boost"
}

$InstallOnlyButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point    (($Form.Width * .511), ($Form.Height * .07 ))
    Size = New-Object Drawing.Size         (($Form.Width * .45),     ($Form.Height * .75 ))
    Text = "Install W11Boost"
}

function FirstWindowControls {
    $Form.Controls.Clear()

    $Form.Controls.Add($InstallOnlyButton)
    $Form.Controls.Add($ExtrasButton)
    $Form.Controls.Add($UninstallButton)
}

function CustomInstallWindow{
    $Form.Controls.Clear()

    $AdvancedButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .01 ))
        Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
        Text = "Advanced"
    }

    $DebloatWindowsButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .275 ))
        Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
        Text = "Debloat Windows"
    }

    $InstallXboxMinimalButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .00725), ($Form.Height * .55 ))
        Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
        Text = "Install Xbox services"
    }

    $ApplyStigsButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .5), ($Form.Height * .01 ))
        Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
        Text = "Install STIG policies"
    }

    $Form.Controls.Add($AdvancedButton)
    $Form.Controls.Add($DebloatWindowsButton)
    $Form.Controls.Add($InstallXboxMinimalButton)
    $Form.Controls.Add($ApplyStigsButton)
    $Form.Controls.Add($GoBack)  
}

function AdvancedWindow {
    $ListBox = New-Object Listbox -Property @{
        Size = New-Object Drawing.Size 640,480
        SelectionMode = "MultiExtended"
    }
    $ListBox.Items.Add("Item 1")
    $ListBox.Items.Add("Test 2")
    $Form.Controls.Add($ListBox)
}

#region Draw initial Form/Window and setup button actions.
FirstWindowControls

$ExtrasButton.Add_Click({CustomInstallWindow})
$UninstallButton.Add_Click({
    $Prompt = [MessageBox]::Show("This will uninstall W11Boost, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)
    if ($Prompt -eq "Y") {
        $Form.Controls.Clear()
        $Label = New-Object Label -Property @{
            Location = New-Object Drawing.Point (($Form.Width * 0.06), ($Form.Height * .35))
            AutoSize = $true
            Text = "Uninstalling W11Boost, please wait..."
        }
        $Form.Controls.Add($Label)

        & ".\..\Extras\Uninstall.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Uninstall.log"

        [MessageBox]::Show($Form, "Removal nearly complete; manually reboot to finish.", "W11Boost", [MessageBoxButtons]::OK)
        $Form.Controls.Clear()
        FirstWindowControls
    }
})

$GoBack = New-Object Button -Property @{
    Location = New-Object Drawing.Point  (($Form.Width * .5), ($Form.Height * .55 ))
    Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
    Text = "<- Go back to prior screen"
}
$GoBack.Add_Click({FirstWindowControls})

$InstallOnlyButton.Add_Click({
    $Form.Controls.Clear()
    $Label = New-Object Label -Property @{
        Location = New-Object Drawing.Point (($Form.Width * 0.1), ($Form.Height * .35))
        AutoSize = $true
        Text = "Installing W11Boost, please wait..."
    }
    $Form.Controls.Add($Label)

    & ".\Main.ps1" | Out-File "${HOME}\Desktop\W11Boost logs\Main.log"

    [MessageBox]::Show($Form, "Installation nearly complete; manually reboot to finish.", "W11Boost", [MessageBoxButtons]::OK)
    $Form.Controls.Clear()
    FirstWindowControls
})

$Form.ShowDialog()
#endregion
