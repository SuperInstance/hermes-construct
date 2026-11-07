# Check Lxss registry to find distro install location
$reg = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Lxss"
Get-ChildItem $reg | ForEach-Object {
    Write-Host "=== $($_.Name) ==="
    Get-ItemProperty $_.PSPath | Format-List Name,State,Version,BasePath,DistributionName,*
    Write-Host ""
}

Write-Host "=== ext4.vhdx files anywhere ==="
Get-PSDrive C, D, E, F -ErrorAction SilentlyContinue | ForEach-Object {
    Get-ChildItem ($_.Root) -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
        Write-Host ("  {0}  ({1} GB)" -f $_.FullName, [math]::Round($_.Length/1GB,2))
    }
}
