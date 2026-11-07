$vhdx = "C:\Users\casey\AppData\Local\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
try {
    $s = [System.IO.File]::Open($vhdx, 'Open', 'Read')
    $s.Close()
    Write-Host "UNLOCKED"
} catch {
    Write-Host "LOCKED: $($_.Exception.Message)"
}
