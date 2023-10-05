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
    Location = New-Object Drawing.Point  (($Form.Width * 0.00725), ($Form.Height * 0.01 ))
    Size = New-Object Drawing.Size      (($Form.Width * .95), ($Form.Height * .425))
    Text = "Extras"
}

$DefaultInstallButton = New-Object Button -Property @{
    Location = New-Object Drawing.Point  (($Form.Width * 0.00725), ($Form.Height * .45 ))
    Size = New-Object System.Drawing.Size (($Form.Width * .95), ($Form.Height * .425))
    Text = "Default installation"
}

function CustomInstallWindow{
    $Form.Controls.Remove($DefaultInstallButton)
    $Form.Controls.Remove($ExtrasButton)

    $AdvancedButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  5, 0
        Size = New-Object Drawing.Size       288, 120
        Text = "Advanced"
    }
    $ApplyStigsButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  5, 125
        Size = New-Object Drawing.Size       288, 120
        Text = "Install STIG policies"
    }
    $DebloatWindowsButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  5, 250
        Size = New-Object Drawing.Size       288, 120
        Text = "Debloat Windows"
    }
    $InstallXboxMinimalButton = New-Object Button -Property @{
        Location = New-Object Drawing.Point  320, 250
        Size = New-Object Drawing.Size       288, 120
        Text = "Install Xbox services"
    }

    $Form.Controls.Add($AdvancedButton)
    $Form.Controls.Add($ApplyStigsButton)
    $Form.Controls.Add($DebloatWindowsButton)
    $Form.Controls.Add($InstallXboxMinimalButton)
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

$Form.Controls.Add($DefaultInstallButton)
$Form.Controls.Add($ExtrasButton)
$ExtrasButton.Add_Click({CustomInstallWindow})
$Form.ShowDialog()
