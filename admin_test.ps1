# Try to query Administrator account
net user Administrator 2>&1

Write-Host "`n=== Try start elevated cmd as Administrator (may prompt for password) ==="
try {
    $proc = Start-Process -FilePath "cmd.exe" -ArgumentList "/c whoami" -Credential (Get-Credential -Message "Enter admin password" -UserName "Administrator") -Wait -PassThru -NoNewWindow -ErrorAction Stop
    Write-Host "Output:"
    $proc | Format-List
} catch {
    Write-Host "Failed: $($_.Exception.Message)"
}
