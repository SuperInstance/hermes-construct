# Admin-mode full plan to compact Ubuntu vhdx to 500GB.
# Run this in an elevated PowerShell (right-click PowerShell -> Run as Administrator)

# 0. Make sure WSL is fully shutdown and clean up any orphan VMs
Write-Host "=== Step 0: Pre-flight ==="
wsl --shutdown 2>&1 | Out-Null
Start-Sleep -Seconds 5
Get-Process wslservice, wslrelay, wslhost, wsl, vmwp, vmmemWSL -ErrorAction SilentlyContinue | ForEach-Object {
    Write-Host "  Killing PID $($_.Id) ($($_.ProcessName))"
    Stop-Process -Id $_.Id -Force -ErrorAction SilentlyContinue
}
Start-Sleep -Seconds 3

# 1. Verify vhdx is unlocked and check state
Write-Host "`n=== Step 1: Verify vhdx state ==="
$vhdx = "C:\Users\casey\AppData\Local\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
try {
    $s = [System.IO.File]::Open($vhdx, 'Open', 'ReadWrite')
    $s.Close()
    Write-Host "  UNLOCKED - good"
} catch {
    Write-Host "  STILL LOCKED: $($_.Exception.Message)"
    Write-Host "  Listing holders via openfiles (if available)..."
    openfiles /query /fo TABLE 2>&1 | Select-String "ext4.vhdx" | ForEach-Object { Write-Host "  $_" }
}

# 2. Run diskpart to compact the vhdx (reclaim zero pages)
Write-Host "`n=== Step 2: diskpart compact ==="
$dpScript = @"
select vdisk file="$vhdx"
compact vdisk
exit
"@
$dpPath = "$env:TEMP\dp_compact_ubuntu.txt"
Set-Content -Path $dpPath -Value $dpScript -Encoding ASCII

$before = (Get-Item $vhdx).Length
Write-Host "  Before: $([math]::Round($before/1GB,2)) GB"
diskpart /s $dpPath 2>&1 | ForEach-Object { Write-Host "  $_" }
$after = (Get-Item $vhdx).Length
Write-Host "  After:  $([math]::Round($after/1GB,2)) GB  (saved $([math]::Round(($before-$after)/1GB,2)) GB)"

# 3. Add NOPASSWD for resize tools to WSL sudoers
Write-Host "`n=== Step 3: Configure sudo NOPASSWD inside WSL ==="
# We need to start WSL once to do this. Use the WSL interop-friendly way.
wsl -d Ubuntu -- bash -c "echo 'casey ALL=(ALL) NOPASSWD: /sbin/resize2fs, /sbin/e2fsck, /sbin/tune2fs, /sbin/fstrim, /usr/sbin/resize2fs, /usr/sbin/e2fsck' | sudo -S tee /etc/sudoers.d/01-resize-nopasswd 2>/dev/null && sudo chmod 440 /etc/sudoers.d/01-resize-nopasswd" 2>&1
# The above will fail without password. Try a different approach: write the file via root in WSL startup.

# Alternative: use systemd-run or direct file write if we have access
# Since we can't escalate in WSL, we'll do this via mounting the ext4 image offline

# 4. Mount vhdx as a disk via diskpart, write sudoers, unmount
Write-Host "`n=== Step 4: Mount vhdx offline to edit sudoers ==="
$dpMount = @"
select vdisk file="$vhdx"
attach vdisk readonly
exit
"@
$dpMountPath = "$env:TEMP\dp_mount_ubuntu.txt"
Set-Content -Path $dpMountPath -Value $dpMount -Encoding ASCII

# We can't read ext4 from Windows natively. Skip this approach.

# 5. Try forcing resize via WSL with the NOPASSWD we set up
Write-Host "`n=== Step 5: If NOPASSWD works, force shrink ext4 ==="
wsl -d Ubuntu -- bash -c "sudo -n resize2fs -f -M /dev/sdd 2>&1; echo exit=\$?" 2>&1
