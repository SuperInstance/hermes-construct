# Need to find the actual block size. We can read it by mounting/inspecting.
# Easiest way: run a quick command inside WSL to check df/lsblk

Write-Host "=== Quick WSL exec to get filesystem info ==="
wsl -d Ubuntu -- bash -c "df -B1 / ; echo --- ; tune2fs -l /dev/sda 2>/dev/null | grep -E 'Block (size|count)|Filesystem' ; echo --- ; lsblk -b" 2>&1
