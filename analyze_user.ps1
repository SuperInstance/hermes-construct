$dirs = Get-ChildItem "C:\Users\casey" -Directory -ErrorAction SilentlyContinue
$results = @()
foreach ($dir in $dirs) {
    try {
        $size = (Get-ChildItem $dir.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1GB
        $results += [PSCustomObject]@{Name=$dir.Name; SizeGB=[math]::Round($size,2)}
    } catch {
        $results += [PSCustomObject]@{Name=$dir.Name; SizeGB=0}
    }
}
$results | Sort-Object SizeGB -Descending | Select-Object -First 20 | Format-Table -AutoSize