Write-Host "=== Current disk state ==="
$free = (Get-PSDrive C).Free
$used = (Get-PSDrive C).Used
Write-Host ("C: free: {0} GB   used: {1} GB   total: {2} GB" -f [math]::Round($free/1GB,2), [math]::Round($used/1GB,2), [math]::Round(($free+$used)/1GB,2))

Write-Host "`n=== vhdx sizes ==="
$vhdx = Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue
foreach ($v in $vhdx) {
    Write-Host ("  {0}  {1} GB" -f $v.FullName, [math]::Round($v.Length/1GB,2))
}
$docker = Get-ChildItem "$env:LOCALAPPDATA\Docker\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue
foreach ($v in $docker) {
    Write-Host ("  {0}  {1} GB" -f $v.FullName, [math]::Round($v.Length/1GB,2))
}

Write-Host "`n=== WSL process state ==="
tasklist | findstr /I "wsl lxss"

Write-Host "`n=== Is vhdx locked? ==="
$target = Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
if ($target) {
    try {
        $s = [System.IO.File]::Open($target.FullName, 'Open', 'Read')
        $s.Close()
        Write-Host "  NOT LOCKED"
    } catch {
        Write-Host "  STILL LOCKED: $($_.Exception.Message)"
    }
}
