# Check if we can elevate
$cur = [Security.Principal.WindowsIdentity]::GetCurrent()
$prin = New-Object Security.Principal.WindowsPrincipal($cur)
Write-Host "Current user: $($cur.Name)"
Write-Host "Is admin: $($prin.IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator))"

Write-Host "`n=== Try to stop LxssManager service (non-admin) ==="
Stop-Service -Name LxssManager -Force -ErrorAction SilentlyContinue
Write-Host "Stop result: $LASTEXITCODE  (0=success, otherwise error)"

Write-Host "`n=== Try via SC command ==="
sc.exe stop LxssManager 2>&1
sc.exe query LxssManager 2>&1 | Select-Object -First 5

Write-Host "`n=== Check UAC / elevation options ==="
# Test if we can launch an elevated process via Start-Process
try {
    $proc = Start-Process -FilePath "cmd.exe" -ArgumentList "/c whoami /priv | findstr SeDebug" -Verb RunAs -Wait -PassThru -ErrorAction Stop
    Write-Host "Elevation worked: PID $($proc.Id)"
} catch {
    Write-Host "Elevation not available: $($_.Exception.Message)"
}
