$targets = @(
    "\\wsl$\Ubuntu\home\eileen\.cache",
    "\\wsl$\Ubuntu\home\eileen\projects",
    "\\wsl$\Ubuntu\home\eileen\results",
    "\\wsl$\Ubuntu\home\eileen\backups",
    "\\wsl$\Ubuntu\home\eileen\tools",
    "\\wsl$\Ubuntu\home\eileen\models",
    "\\wsl$\Ubuntu\home\eileen\archive-rkZq8p",
    "\\wsl$\Ubuntu\home\eileen\.trash",
    "\\wsl$\Ubuntu\home\eileen\ship",
    "\\wsl$\Ubuntu\home\eileen\mist-art",
    "\\wsl$\Ubuntu\home\eileen\ai-writings",
    "\\wsl$\Ubuntu\home\eileen\micromamba",
    "\\wsl$\Ubuntu\home\eileen\micromamba-envs",
    "\\wsl$\Ubuntu\home\eileen\llama.cpp-rel",
    "\\wsl$\Ubuntu\home\eileen\llama.cpp-prebuilt",
    "\\wsl$\Ubuntu\home\eileen\.ollama",
    "\\wsl$\Ubuntu\home\eileen\.cargo",
    "\\wsl$\Ubuntu\home\eileen\.julia",
    "\\wsl$\Ubuntu\home\eileen\.bun",
    "\\wsl$\Ubuntu\home\eileen\.conda",
    "\\wsl$\Ubuntu\home\eileen\.mamba",
    "\\wsl$\Ubuntu\home\eileen\.rustup",
    "\\wsl$\Ubuntu\home\eileen\.opencode",
    "\\wsl$\Ubuntu\home\eileen\.superinstance",
    "\\wsl$\Ubuntu\home\eileen\.platformio",
    "\\wsl$\Ubuntu\home\eileen\.npm",
    "\\wsl$\Ubuntu\home\eileen\.npm-global",
    "\\wsl$\Ubuntu\home\eileen\.venvs",
    "\\wsl$\Ubuntu\home\eileen\tmp-builds",
    "\\wsl$\Ubuntu\home\eileen\.local",
    "\\wsl$\Ubuntu\home\eileen\install-logs",
    "\\wsl$\Ubuntu\home\eileen\Desktop"
)
$results = @()
foreach ($t in $targets) {
    if (Test-Path $t) {
        try {
            $size = (Get-ChildItem $t -Recurse -Force -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum -ErrorAction SilentlyContinue).Sum
            $results += [PSCustomObject]@{
                Path = $t
                SizeGB = if ($size) { [math]::Round($size/1GB, 2) } else { 0 }
            }
        } catch {}
    }
}
$results | Sort-Object SizeGB -Descending | Format-Table -AutoSize
