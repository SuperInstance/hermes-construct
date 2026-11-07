Write-Host ("Free: {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))
$v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Write-Host ("vhdx: {0} GB" -f [math]::Round($v.Length/1GB,2))
