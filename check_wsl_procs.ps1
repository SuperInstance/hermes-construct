$procs = Get-Process wsl*, wslservice -ErrorAction SilentlyContinue
foreach ($p in $procs) {
    Write-Host "PID $($p.Id) - $($p.ProcessName)"
    $wmi = Get-WmiObject Win32_Process -Filter "ProcessId = $($p.Id)"
    $owner = $wmi.GetOwner()
    Write-Host "  User: $($owner.User) Domain: $($owner.Domain)"
    Write-Host "  Path: $($wmi.ExecutablePath)"
    Write-Host ""
}
