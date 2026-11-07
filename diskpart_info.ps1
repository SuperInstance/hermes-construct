Get-CimInstance Win32_Process -Filter "Name = 'diskpart.exe'" | Select-Object ProcessId, ParentProcessId, CommandLine, SessionId | Format-List

Write-Host "=== CIM for vds.exe ==="
Get-CimInstance Win32_Process -Filter "Name = 'vds.exe'" | Select-Object ProcessId, ParentProcessId, CommandLine, SessionId | Format-List
