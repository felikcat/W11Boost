#region Increase pagefile to total RAM * 3 + 257MB.
$DESIRED_PAGEFILE = (Get-CimInstance Win32_PhysicalMemoryArray).MaxCapacity * 1KB/1MB * 3 + 257
$PageFile = Get-CimInstance -ClassName Win32_PageFileSetting -Filter "Name like '%pagefile.sys'"
$PageFile | Remove-CimInstance
$PageFile = New-CimInstance -ClassName Win32_PageFileSetting -Property @{ Name= "C:\pagefile.sys" }
$PageFile | Set-CimInstance -Property @{ InitialSize = $DESIRED_PAGEFILE; MaximumSize = $DESIRED_PAGEFILE }
#endregion

