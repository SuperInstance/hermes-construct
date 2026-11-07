$paths = @($env:LOCALAPPDATA, "$env:USERPROFILE\AppData\Local", "C:\AppData\Local")
foreach ($p in $paths) {
    Write-Host "Scanning: $p"
    Get-ChildItem $p -Filter ext4.vhdx -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
        Write-Host ("  FOUND: {0}  ({1} GB)" -f $_.FullName, [math]::Round($_.Length/1GB,2))
    }
}
Write-Host "---"
Write-Host ("Free on C: {0} GB" -f [math]::Round((Get-PSDrive C).Free/1GB,2))
