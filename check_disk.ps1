$dirs = Get-ChildItem C:\Users\casey\AppData\Local -Directory
$results = @()
foreach ($dir in $dirs) {
    $size = (Get-ChildItem $dir.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum
    $results += [PSCustomObject]@{Name=$dir.Name; SizeGB=[math]::Round($size/1GB,2)}
}
$results | Sort-Object SizeGB -Descending | Select-Object -First 15 | Format-Table