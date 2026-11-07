$top = "\\wsl$\Ubuntu\home\eileen"
$results = @()
Get-ChildItem $top -Directory -Force -ErrorAction SilentlyContinue | ForEach-Object {
    try {
        $size = (Get-ChildItem $_.FullName -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue).Sum
        $results += [PSCustomObject]@{
            Path = $_.Name
            SizeGB = if ($size) { [math]::Round($size/1GB, 2) } else { 0 }
        }
    } catch {}
}
$results | Sort-Object SizeGB -Descending | Select-Object -First 20 | Format-Table -AutoSize
