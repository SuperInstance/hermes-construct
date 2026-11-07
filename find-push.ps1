Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'python.exe' -and $_.CommandLine -like '*proart-push*' } | ForEach-Object { Write-Output $_.ProcessId }
