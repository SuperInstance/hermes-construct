# Check WSL version and what we'd need to reinstall
wsl --version 2>&1 | Out-String | Write-Host
Write-Host "==="
# What's actually used inside WSL?
wsl -d Ubuntu -- bash -c "df -h / /home 2>&1; echo ===; ls /home 2>&1" 2>&1
