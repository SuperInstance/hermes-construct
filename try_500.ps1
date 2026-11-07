Write-Host "=== before resize ==="
$v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Write-Host ("  {0}  ({1} bytes  {2} GB)" -f $v.FullName, $v.Length, [math]::Round($v.Length/1GB,2))
Write-Host ("  Free C: {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))

# Make sure WSL is shutdown so vhdx isn't locked
wsl --shutdown 2>&1 | Out-Null
Start-Sleep -Seconds 3

Write-Host "`n=== running wsl --manage Ubuntu --resize 500GB ==="
$out = wsl --manage Ubuntu --resize 500GB 2>&1 | Out-String
Write-Host $out
Write-Host "exit: $LASTEXITCODE"

Write-Host "`n=== after resize ==="
$v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Write-Host ("  {0}  ({1} bytes  {2} GB)" -f $v.FullName, $v.Length, [math]::Round($v.Length/1GB,2))
Write-Host ("  Free C: {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))
