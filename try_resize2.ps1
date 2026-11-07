Write-Host "Step 1: wsl --shutdown (with --force)"
wsl --shutdown --force 2>&1
Write-Host "exit: $LASTEXITCODE"

Start-Sleep -Seconds 2

Write-Host "`nStep 2: check lock state"
$vhdx = Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
if ($vhdx) {
    try {
        $s = [System.IO.File]::Open($vhdx.FullName, 'Open', 'Read')
        $s.Close()
        Write-Host "  UNLOCKED - good"
    } catch {
        Write-Host "  LOCKED: $($_.Exception.Message)"
    }
}

Write-Host "`nStep 3: process check"
tasklist | findstr /I "wsl lxss vmmem"

Write-Host "`nStep 4: try the resize"
wsl --manage Ubuntu --resize 100GB 2>&1 | Select-Object -First 5
Write-Host "exit: $LASTEXITCODE"
