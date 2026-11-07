Write-Host "=== Trying wsl --manage Ubuntu --resize 100GB ==="
wsl --manage Ubuntu --resize 100GB 2>&1
Write-Host "`n=== exit: $LASTEXITCODE ==="

Write-Host "`n=== Try diskpart approach as fallback ==="
# Find the vhdx
$vhdx = Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
if ($vhdx) {
    Write-Host "vhdx: $($vhdx.FullName)  size: $([math]::Round($vhdx.Length/1GB,2)) GB"
    Write-Host "is locked?"
    try {
        $stream = [System.IO.File]::Open($vhdx.FullName, 'Open', 'Read')
        $stream.Close()
        Write-Host "  NO - file is not locked"
    } catch {
        Write-Host "  YES - locked: $($_.Exception.Message)"
    }
}
