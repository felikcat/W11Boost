# Disable features that can cause random packet loss/drop-outs.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Auto Disable Gigabit' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Energy Efficient Ethernet' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Gigabit Lite' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Green Ethernet' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Power Saving Mode' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Selective Suspend' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'ULP' -RegistryValue 0
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Ultra Low Power Mode' -RegistryValue 0
