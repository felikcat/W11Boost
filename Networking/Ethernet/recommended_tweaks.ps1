# Can reduce time taken to establish a connection, and prevent drop-outs.
# Drop-outs were the case with Intel I225-V revision 1 to 2, but not 3.
Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0
