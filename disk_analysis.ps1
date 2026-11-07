$dirs = Get-ChildItem C:\ -Directory -ErrorAction SilentlyContinue
$results = @()
foreach ($dir in $dirs) {
    try {
        $size = (Get-ChildItem $dir.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1GB
        $results += [PSCustomObject]@{Name=$dir.Name; Path=$dir.FullName; SizeGB=[math]::Round($size,2)}
    } catch {
        $results += [PSCustomObject]@{Name=$dir.Name; Path=$dir.FullName; SizeGB=0}
    }
}
$results | Sort-Object SizeGB -Descending | Select-Object -First 25 | Format-Table -AutoSize