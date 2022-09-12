# Can reduce time taken to establish a connection, and prevent drop-outs.
# Drop-outs were the case with Intel I225-V revision 1 to 2, but not 3.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0

# TCP is to be reliable under bad network conditions, unlike UDP. Don't make it more-so like UDP.
Set-NetTCPSetting -SettingName InternetCustom -Timestamps Enabled
