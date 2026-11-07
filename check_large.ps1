$dirs = @('Downloads', 'Documents')
foreach ($d in $dirs) {
    $path = "C:\Users\casey\$d"
    if (Test-Path $path) {
        Write-Host "=== $d ==="
        Get-ChildItem $path -Directory -ErrorAction SilentlyContinue | ForEach-Object {
            $size = (Get-ChildItem $_.FullName -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1GB
            if ($size -gt 0.1) {
                Write-Host "  $($_.Name) : $([math]::Round($size,2)) GB"
            }
        }
        # Also check large files directly in the folder
        Get-ChildItem $path -File -ErrorAction SilentlyContinue | Sort-Object Length -Descending | Select-Object -First 10 | ForEach-Object {
            Write-Host "  FILE: $($_.Name) : $([math]::Round($_.Length/1GB,2)) GB"
        }
    }
}