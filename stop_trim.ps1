# Kill the fstrim inside WSL first
Write-Host "=== Killing any sudo fstrim processes ==="
wsl -d Ubuntu -- bash -c "sudo pkill -9 fstrim 2>&1; sudo pkill -9 fstrim.fstrim 2>&1; echo done" 2>&1
wsl --shutdown 2>&1

Start-Sleep -Seconds 3

Write-Host "`n=== Disk now ==="
Write-Host ("  {0} GB free" -f [math]::Round((Get-PSDrive C).Free/1GB,2))

Write-Host "`n=== vhdx ==="
Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -EA SilentlyContinue | ForEach-Object {
    Write-Host ("  {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,2))
}

Write-Host "`n=== wsl processes ==="
tasklist | findstr /I "wsl lxss vmmem diskpart"
