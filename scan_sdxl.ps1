$paths = @(
    "\\wsl$\Ubuntu\home\eileen\.cache\huggingface\hub\models--stabilityai--sdxl-turbo"
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
            Get-ChildItem $p -File -Force | Select-Object Name, @{N="SizeGB";E={[math]::Round($_.Length/1GB,3)}}, LastWriteTime | Format-Table -AutoSize
        } catch {}
    }
}
