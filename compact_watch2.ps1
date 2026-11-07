# Try without Verb RunAs - just launch diskpart normally
Write-Host "=== Launching diskpart without elevation ==="
$proc = Start-Process -FilePath "diskpart.exe" -ArgumentList "/s C:\Users\casey\dp_compact.txt" -PassThru -WindowStyle Hidden -RedirectStandardOutput "C:\Users\casey\dp_out.txt" -RedirectStandardError "C:\Users\casey\dp_err.txt"
Write-Host "PID: $($proc.Id)"

for ($i = 0; $i -lt 30; $i++) {
    Start-Sleep -Seconds 5
    $free = (Get-PSDrive C).Free
    try { $v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx" -ErrorAction Stop } catch { $v = $null }
    $vhdxGB = if ($v) { [math]::Round($v.Length/1GB, 2) } else { "n/a" }
    $freeGB = [math]::Round($free/1GB, 2)
    Write-Host ("[t={0,2}] free={1} GB  vhdx={2} GB" -f ($i*5), $freeGB, $vhdxGB)

    $dp = Get-Process diskpart -ErrorAction SilentlyContinue
    if (-not $dp) {
        Write-Host "diskpart exited"
        break
    }
}

Write-Host "`n=== diskpart output ==="
if (Test-Path "C:\Users\casey\dp_out.txt") { Get-Content "C:\Users\casey\dp_out.txt" }
if (Test-Path "C:\Users\casey\dp_err.txt") { Write-Host "--- stderr ---"; Get-Content "C:\Users\casey\dp_err.txt" }
