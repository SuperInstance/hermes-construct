# Try to kill the WSL VM components
Write-Host "=== Trying to kill wsl VM worker processes ==="
foreach ($name in @("wslrelay.exe", "wsl.exe", "wslservice.exe")) {
    Get-Process -Name $name -ErrorAction SilentlyContinue | ForEach-Object {
        Write-Host "  found PID $($_.Id) for $name"
        try {
            Stop-Process -Id $_.Id -Force -ErrorAction Stop
            Write-Host "    killed"
        } catch {
            Write-Host "    FAILED: $($_.Exception.Message)"
        }
    }
}

# vmwp.exe is WSL VM worker - in services session 0
$vmwp = Get-Process vmwp -ErrorAction SilentlyContinue
Write-Host "`nvmwp.exe PIDs: $($vmwp.Id -join ',')"
foreach ($p in $vmwp) {
    try {
        Stop-Process -Id $p.Id -Force -ErrorAction Stop
        Write-Host "  killed PID $($p.Id)"
    } catch {
        Write-Host "  FAILED PID $($p.Id): $($_.Exception.Message)"
    }
}

Start-Sleep -Seconds 2

Write-Host "`n=== after kill ==="
tasklist | findstr /I "wsl vmwp vmmem diskpart"
