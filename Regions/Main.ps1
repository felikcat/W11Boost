#Requires -Version 5
using assembly System.Windows.Forms
using namespace System.Windows.Forms
using namespace System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

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

$DefaultInstallButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point    (($Form.Width * .511), ($Form.Height * .07 ))
    Size = New-Object Drawing.Size         (($Form.Width * .45),     ($Form.Height * .75 ))
    Text = "Install W11Boost"
}

function FirstWindowControls {
    $Form.Controls.Clear()

    $Form.Controls.Add($DefaultInstallButton)
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

    $UseQuad9DNS = New-Object Button -Property @{
        Location = New-Object Drawing.Point  (($Form.Width * .5), ($Form.Height * .275 ))
        Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
        Text = "Install Quad9 DNS-over-HTTPS"
    }

    $Form.Controls.Add($AdvancedButton)
    $Form.Controls.Add($DebloatWindowsButton)
    $Form.Controls.Add($InstallXboxMinimalButton)
    $Form.Controls.Add($ApplyStigsButton)
    $Form.Controls.Add($UseQuad9DNS)
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

function UninstallPrompt {
    $Prompt = [MessageBox]::Show("This will uninstall W11Boost, are you sure?", "W11Boost", [MessageBoxButtons]::YesNo)
    if ($Prompt -eq "Y") {
        ####
        [MessageBox]::Show("Removal nearly complete, please reboot manually to complete.", "W11Boost", [MessageBoxButtons]::OK)
    }
}

FirstWindowControls
# Never run Add_Click more than once.
$ExtrasButton.Add_Click({CustomInstallWindow})
$UninstallButton.Add_Click({UninstallPrompt})

# Keep GoBack outside of functions.
$GoBack = New-Object Button -Property @{
    Location = New-Object Drawing.Point  (($Form.Width * .5), ($Form.Height * .55 ))
    Size = New-Object Drawing.Size       (($Form.Width * .45), ($Form.Height * .25))
    Text = "<- Go back to prior screen"
}
$GoBack.Add_Click({FirstWindowControls})

$Form.ShowDialog()
