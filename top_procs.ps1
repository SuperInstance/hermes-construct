Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First 10 ProcessName,@{N='WS_MB';E={[math]::Round($_.WorkingSet64/1MB,0)}},@{N='Private_MB';E={[math]::Round($_.PrivateMemorySize/1MB,0)}} | Format-Table -AutoSize

Write-Host "`n=== disk free ==="
Write-Host ("  {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))

Write-Host "`n=== vhdx ==="
Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -EA SilentlyContinue | ForEach-Object {
    Write-Host ("  {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,2))
}
