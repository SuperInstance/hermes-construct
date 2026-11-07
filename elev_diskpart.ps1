# Try to elevate and run diskpart compact

# Method 1: Try start-process verb runas
$diskpartScript = @"
select vdisk file="C:\Users\casey\AppData\Local\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}\ext4.vhdx"
compact vdisk
exit
"@
$scriptPath = "$env:TEMP\dp_compact.txt"
Set-Content -Path $scriptPath -Value $diskpartScript -Encoding ASCII

Write-Host "Script at: $scriptPath"
Get-Content $scriptPath

Write-Host "`n=== Trying Start-Process -Verb RunAs ==="
try {
    $proc = Start-Process -FilePath "diskpart.exe" -ArgumentList "/s `"$scriptPath`"" -Verb RunAs -Wait -PassThru -WindowStyle Hidden -ErrorAction Stop
    Write-Host "diskpart exit code: $($proc.ExitCode)"
} catch {
    Write-Host "Elevation failed: $($_.Exception.Message)"
}
