Write-Host "Before kill:"
tasklist /FI "IMAGENAME eq wsl.exe" /FO TABLE
tasklist /FI "IMAGENAME eq wslservice.exe" /FO TABLE

Write-Host "`nKilling wsl.exe (PID 26996)..."
taskkill /F /PID 26996 2>&1
Write-Host "Killing wslservice.exe (PID 18652)..."
taskkill /F /PID 18652 2>&1

Start-Sleep -Seconds 3

Write-Host "`nAfter kill:"
tasklist /FI "IMAGENAME eq wsl.exe" /FO TABLE
tasklist /FI "IMAGENAME eq wslservice.exe" /FO TABLE
