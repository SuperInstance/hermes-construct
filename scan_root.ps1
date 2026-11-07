$paths = @(
    "\\wsl$\Ubuntu\var",
    "\\wsl$\Ubuntu\opt",
    "\\wsl$\Ubuntu\root",
    "\\wsl$\Ubuntu\tmp",
    "\\wsl$\Ubuntu\srv",
    "\\wsl$\Ubuntu\usr",
    "\\wsl$\Ubuntu\etc",
    "\\wsl$\Ubuntu\lib"
)
$results = @()
foreach ($p in $paths) {
    if (Test-Path $p) {
        try {
            $size = (Get-ChildItem $p -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue).Sum
            $results += [PSCustomObject]@{
                Path = $p
                SizeGB = if ($size) { [math]::Round($size/1GB, 2) } else { 0 }
            }
        } catch {}
    }
}
$results | Sort-Object SizeGB -Descending | Format-Table -AutoSize
