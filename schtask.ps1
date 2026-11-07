# Test if we can create a scheduled task that runs elevated without UAC
Write-Host "=== Trying schtasks to create elevated task ==="

$taskName = "WSLCompact_$(Get-Random)"
$dpScript = "select vdisk file=`"C:\Users\casey\AppData\Local\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx`"`r`ncompact vdisk`r`nexit"

# Try /rl HIGHEST
try {
    schtasks /Create /TN $taskName /TR "cmd /c echo $dpScript > %TEMP%\dp.txt & diskpart /s %TEMP%\dp.txt" /SC ONCE /ST 00:00 /RL HIGHEST /F 2>&1 | Out-Host
    Write-Host "task created: $taskName"
    Write-Host "Trying to run it..."
    schtasks /Run /TN $taskName 2>&1 | Out-Host
    Start-Sleep -Seconds 10
    Write-Host "Removing task..."
    schtasks /Delete /TN $taskName /F 2>&1 | Out-Host
} catch {
    Write-Host "failed: $($_.Exception.Message)"
}

Write-Host "`n=== Disk now ==="
Write-Host ("  {0} GB free" -f [math]::Round((Get-PSDrive C).Free/1GB,2))
$v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
Write-Host ("  vhdx: {0} GB" -f [math]::Round($v.Length/1GB,2))

Write-Host "`n=== diskpart still running? ==="
tasklist | findstr /I "diskpart"
