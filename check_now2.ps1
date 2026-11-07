Start-Sleep -Seconds 5
Write-Host "=== Disk now ==="
$free = (Get-PSDrive C).Free
Write-Host ("C: free: {0} GB" -f [math]::Round($free/1GB,2))

Write-Host "`n=== vhdx sizes ==="
Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host ("  {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,2))
}

Write-Host "`n=== wsl processes ==="
tasklist | findstr /I "wsl lxss"
