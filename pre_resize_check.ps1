Write-Host "=== PRE-RESIZE STATE ==="
Write-Host "Config:"
Get-Content "$env:USERPROFILE\.wslconfig"
Write-Host ""
Write-Host "Disk space:"
Get-PSDrive C | Format-Table -AutoSize
Write-Host ""
Write-Host "vhdx size:"
$vhdx = "C:\Users\casey\AppData\Local\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Get-Item $vhdx | Select-Object @{N='SizeGB';E={[math]::Round($_.Length/1GB,2)}}, LastWriteTime | Format-Table -AutoSize
Write-Host ""
Write-Host "Shutting down WSL..."
wsl --shutdown
Start-Sleep -Seconds 5
Write-Host "Running distros:"
wsl --list --running
