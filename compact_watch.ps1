# Launch diskpart elevated and watch for it to appear
Write-Host "=== launching diskpart via RunAs (UAC will prompt) ==="
$proc = Start-Process -FilePath "diskpart.exe" -ArgumentList "/s C:\Users\casey\dp_compact.txt" -Verb RunAs -PassThru -WindowStyle Hidden
Write-Host "PID: $($proc.Id)  HasExited: $($proc.HasExited)"

# Watch diskpart and vhdx
for ($i = 0; $i -lt 30; $i++) {
    Start-Sleep -Seconds 5
    $free = (Get-PSDrive C).Free
    $v = Get-Item "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
    $vhdxGB = [math]::Round($v.Length/1GB, 2)
    $freeGB = [math]::Round($free/1GB, 2)
    Write-Host ("[t={0,2}] free={1} GB  vhdx={2} GB" -f ($i*5), $freeGB, $vhdxGB)

    # Check if diskpart is still running
    $dp = Get-Process diskpart -ErrorAction SilentlyContinue
    if (-not $dp) {
        Write-Host "diskpart has exited"
        break
    }
}
