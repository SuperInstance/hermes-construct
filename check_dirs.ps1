$dirs = @('.cargo', '.rustup', 'node_modules', 'VirtualBox VMs', '.ollama', 'polln', '.conda', '.pyenv', '.cache', '.docker', 'OneDrive', 'Downloads', 'ai-writings', 'Documents', 'Music', 'Videos', 'Pictures', 'Desktop', 'SuperInstance-papers', 'residual-self-image', 'hermes-construct', 'hermes-nerve-center')
foreach ($d in $dirs) {
    $path = "C:\Users\casey\$d"
    if (Test-Path $path) {
        $size = (Get-ChildItem $path -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1GB
        Write-Host "$d : $([math]::Round($size,2)) GB"
    } else {
        Write-Host "$d : NOT FOUND"
    }
}