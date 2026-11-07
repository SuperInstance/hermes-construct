Write-Host "=== Try fstrim inside Ubuntu to zero-free the ext4 ==="
wsl -d Ubuntu -- bash -c "sudo fstrim -v / 2>&1; echo exit=\$?"
Write-Host "`n=== After fstrim, check vhdx size ==="
Start-Sleep -Seconds 3
$vhdx = Get-ChildItem "$env:LOCALAPPDATA\wsl" -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
if ($vhdx) {
    Write-Host ("  {0}  {1} GB" -f $vhdx.FullName, [math]::Round($vhdx.Length/1GB,2))
}
