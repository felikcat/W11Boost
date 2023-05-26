#Requires -Version 5 -RunAsAdministrator

# Enables Windows Audio services.
$audio = 1

$bluetooth_and_wifi = 0

# Support for microphone recording from a Bluetooth headset/earbuds.
# Causes a major sound quality reduction on the Samsung Galaxy Buds Pro 1.
$bluetooth_hands_free = 0

# Overrides $bluetooth_and_wifi to support WiFi, but with no Bluetooth support.
$wifi_only = 0

if ($audio)
{
    $_regs = @("Audiosrv", "AudioEndpointBuilder")
    $_regs.ForEach({
        Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 2 -Force
    })
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\BthAvctpSvc" -Name "Start" -Type DWord -Value 3 -Force
}

if ($wifi_only)
{
    $bluetooth_and_wifi = 0
}

if ($bluetooth_and_wifi)
{
    $_regs = @("bthserv", "RmSvc", "NcbService")
    $_regs.ForEach({
        Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 3 -Force
    })

    $_regs = @("DcomLaunch", "RpcEptMapper", "RpcSs", "BrokerInfrastructure")
    $_regs.ForEach({
        Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\$_" -Name "Start" -Type DWord -Value 2 -Force
    })
}

if ($bluetooth_hands_free)
{
    Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Services\BTAGService" -Name "Start" -Type DWord -Value 3 -Force
}