Write-Host "=== C drive ==="
$free = (Get-PSDrive C).Free
$used = (Get-PSDrive C).Used
Write-Host ("  free={0} GB  used={1} GB  total={2} GB" -f [math]::Round($free/1GB,2), [math]::Round($used/1GB,2), [math]::Round(($free+$used)/1GB,2))

Write-Host "`n=== AppData\Local\wsl ==="
if (Test-Path "$env:LOCALAPPDATA\wsl") {
    Get-ChildItem "$env:LOCALAPPDATA\wsl" -Force | ForEach-Object {
        if ($_.PSIsContainer) { Write-Host ("  [D] {0}" -f $_.FullName) }
        else { Write-Host ("      {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,4)) }
    }
} else {
    Write-Host "  path does not exist"
}

Write-Host "`n=== AppData\Local\wsl\{ce718fe0-...} ==="
$ubuntuDir = "$env:LOCALAPPDATA\wsl\{ce718fe0-2cfd-4ad9-9995-20b13b2f2ff9}"
if (Test-Path $ubuntuDir) {
    Get-ChildItem $ubuntuDir -Force | ForEach-Object {
        if ($_.PSIsContainer) { Write-Host ("  [D] {0}" -f $_.FullName) }
        else { Write-Host ("      {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,4)) }
    }
} else {
    Write-Host "  path does not exist"
}

Write-Host "`n=== Docker\\wsl\\main ==="
if (Test-Path "$env:LOCALAPPDATA\Docker\wsl\main") {
    Get-ChildItem "$env:LOCALAPPDATA\Docker\wsl\main" -Force | ForEach-Object {
        if ($_.PSIsContainer) { Write-Host ("  [D] {0}" -f $_.FullName) }
        else { Write-Host ("      {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,4)) }
    }
} else {
    Write-Host "  path does not exist"
}

Write-Host "`n=== wsl processes ==="
tasklist | findstr /I "wsl lxss diskpart vmmem"
