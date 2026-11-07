Write-Host "=== C free now ==="
Write-Host ("  {0} GB free  /  {1} GB used  /  {2} GB total" -f [math]::Round((Get-PSDrive C).Free/1GB,2), [math]::Round((Get-PSDrive C).Used/1GB,2), [math]::Round(((Get-PSDrive C).Free+(Get-PSDrive C).Used)/1GB,2))

Write-Host "`n=== wsl dir contents ==="
Get-ChildItem "$env:LOCALAPPDATA\wsl" -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
    if ($_.PSIsContainer) { Write-Host ("  [D] {0}" -f $_.FullName) }
    else { Write-Host ("      {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,3)) }
}

Write-Host "`n=== docker wsl dir contents ==="
Get-ChildItem "$env:LOCALAPPDATA\Docker\wsl" -Recurse -ErrorAction SilentlyContinue | ForEach-Object {
    if ($_.PSIsContainer) { Write-Host ("  [D] {0}" -f $_.FullName) }
    else { Write-Host ("      {0}  {1} GB" -f $_.FullName, [math]::Round($_.Length/1GB,3)) }
}

Write-Host "`n=== Top 20 directories under C ==="
Get-ChildItem C:\ -Directory -ErrorAction SilentlyContinue | ForEach-Object {
    $size = (Get-ChildItem $_.FullName -Recurse -File -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    if ($size -gt 0) { Write-Host ("  {0,-40}  {1,8:N2} GB" -f $_.Name, ($size/1GB)) }
} | Sort-Object { $_.ToString().Substring(40) } -Descending | Select-Object -First 20
