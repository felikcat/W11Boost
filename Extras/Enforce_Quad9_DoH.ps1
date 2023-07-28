$NET_ALIAS = (Get-NetAdapter -Physical)
$NET_DEVID = $NET_ALIAS.InterfaceGuid

$IP = @("9.9.9.9", "149.112.112.112", "2620:fe::fe", "2620:fe::9")
$IP.ForEach({
    # We're not removing the same DoH entries if it already exists; hence "-ErrorAction SilentlyContinue".
    Add-DnsClientDohServerAddress -ServerAddress $_ -DohTemplate 'https://dns.quad9.net/dns-query' -AllowFallbackToUdp 0 -AutoUpgrade 1 -ErrorAction SilentlyContinue
})

$NET_ALIAS.ForEach({
    $NET_DEVID.ForEach({
        # -Force: If that registry existed, wipe it out.
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh\9.9.9.9" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh\149.112.112.112" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD

        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh6\2620:fe::fe" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
        New-Item "HKLM:\SYSTEM\CurrentControlSet\Services\Dnscache\InterfaceSpecificParameters\$_\DohInterfaceSettings\Doh6\2620:fe::9" -Force |
                New-ItemProperty -Name "DohFlags" -Value 1 -PropertyType QWORD
    })
    Set-DnsClientServerAddress -InterfaceIndex ($_).InterfaceIndex -ServerAddresses ([String[]]$IP)
})
Clear-DnsClientCache
