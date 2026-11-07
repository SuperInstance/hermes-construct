Get-Process -Name diskpart -ErrorAction SilentlyContinue | Format-List Id, ProcessName, Path, Company, Description, StartTime, Responding, Handle, HandleCount
Write-Host "---"
Get-Process -Name vds -ErrorAction SilentlyContinue | Format-List Id, ProcessName, Path, Company, StartTime, Responding
