function Set-Recommended-Ethernet-Tweaks {
    # Can reduce time taken to establish a connection, and prevent drop-outs.
    # Drop-outs were the case with Intel I225-V revision 1 to 2, but not 3.
    Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

    # TCP is to be reliable under bad network conditions, unlike UDP. Don't make it more-so like UDP.
    Set-NetTCPSetting -SettingName InternetCustom -Timestamps Enabled
}

function Disable-Ethernet-Power-Saving {
    $properties = @("Advanced EEE", "Auto Disable Gigabit", "Energy Efficient Ethernet",
        "Gigabit Lite", "Green Ethernet", "Power Saving Mode",
        "Selective Suspend", "ULP", "Ultra Low Power Mode")
    # Disable features that can cause random packet loss/drop-outs.
    for ($i = 0; $i -lt $properties.length; $i++) {
        Set-NetAdapterAdvancedProperty -Name '*' -DisplayName $properties[$i] -RegistryValue 0
    }
}

# Delete ExploitGuard ProcessMitigations for a given key in the registry;
# if no other settings exist under the specified key, the key is deleted as well.
function Remove-ProcessMitigations([Object] $Key, [string] $Name) {
    Try {
        if ($Key.GetValue("MitigationOptions")) {
            Write-Host "Removing MitigationOptions for:      " $Name
            Remove-ItemProperty -Path $Key.PSPath -Name "MitigationOptions" -ErrorAction Stop;
        }
        if ($Key.GetValue("MitigationAuditOptions")) {
            Write-Host "Removing MitigationAuditOptions for: " $Name
            Remove-ItemProperty -Path $Key.PSPath -Name "MitigationAuditOptions" -ErrorAction Stop;
        }

        # Remove the FilterFullPath value if there is nothing else
        if (($Key.SubKeyCount -eq 0) -and ($Key.ValueCount -eq 1) -and ($Key.GetValue("FilterFullPath"))) {
            Remove-ItemProperty -Path $Key.PSPath -Name "FilterFullPath" -ErrorAction Stop;
        }

        # If the key is empty now, delete it
        if (($Key.SubKeyCount -eq 0) -and ($Key.ValueCount -eq 0)) {
            Write-Host "Removing empty Entry:                " $Name
            Remove-Item -Path $Key.PSPath -ErrorAction Stop
        }
    }
    Catch {
        Write-Host "ERROR:" $_.Exception.Message "- at ($MitigationItemName)"
    }
}

# Delete all ExploitGuard ProcessMitigations
function Remove-All-ProcessMitigations {
    Get-ChildItem -Path "HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options" | ForEach-Object {
        $MitigationItem = $_;
        $MitigationItemName = $MitigationItem.PSChildName

        Try {
            Remove-ProcessMitigations $MitigationItem $MitigationItemName

            # "UseFilter" indicate full path filters may be present
            if ($MitigationItem.GetValue("UseFilter")) {
                Get-ChildItem -Path $MitigationItem.PSPath | ForEach-Object {
                    $FullPathItem = $_
                    if ($FullPathItem.GetValue("FilterFullPath")) {
                        $Name = $MitigationItemName + "-" + $FullPathItem.GetValue("FilterFullPath")
                        Write-Host "Removing FullPathEntry:              " $Name
                        Remove-ProcessMitigations $FullPathItem $Name
                    }

                    # If there are no subkeys now, we can delete the "UseFilter" value
                    if ($MitigationItem.SubKeyCount -eq 0) {
                        Remove-ItemProperty -Path $MitigationItem.PSPath -Name "UseFilter" -ErrorAction Stop
                    }
                }
            }
            if (($MitigationItem.SubKeyCount -eq 0) -and ($MitigationItem.ValueCount -eq 0)) {
                Write-Host "Removing empty Entry:                " $MitigationItemName
                Remove-Item -Path $MitigationItem.PSPath -ErrorAction Stop
            }
        }
        Catch {
            Write-Host "ERROR:" $_.Exception.Message "- at ($MitigationItemName)"
        }
    }
}

# Delete all ExploitGuard System-wide Mitigations
function Remove-All-SystemMitigations {
    $Kernel = Get-Item -Path "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\kernel"

    Try {
        if ($Kernel.GetValue("MitigationOptions")) {
            Write-Host "Removing System MitigationOptions"
            Remove-ItemProperty -Path $Kernel.PSPath -Name "MitigationOptions" -ErrorAction Stop;
        }
        if ($Kernel.GetValue("MitigationAuditOptions")) {
            Write-Host "Removing System MitigationAuditOptions"
            Remove-ItemProperty -Path $Kernel.PSPath -Name "MitigationAuditOptions" -ErrorAction Stop;
        }
    }
    Catch {
        Write-Host "ERROR:" $_.Exception.Message "- System"
    }
}
