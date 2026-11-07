$p = "\\wsl$\Ubuntu\home\eileen\.cache\huggingface\hub\models--stabilityai--sdxl-turbo"
if (Test-Path $p) {
    Write-Host "Path exists"
    Get-ChildItem $p -Force -ErrorAction SilentlyContinue | Select-Object Name, LastWriteTime | Format-Table -AutoSize
    Write-Host "--- Files ---"
    Get-ChildItem $p -Recurse -File -Force -ErrorAction SilentlyContinue | Select-Object FullName, @{N="SizeGB";E={[math]::Round($_.Length/1GB,3)}}, LastWriteTime | Format-Table -AutoSize
}
