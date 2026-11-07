# Disk Space Analysis & Recovery (2026-08-18)

## Context
SuperInstance embedder upgrade blocked by 952G/953G used (100%) on C: drive. Release build + maturin needs ~5GB free.

## PowerShell Analysis Script
```powershell
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
```

## Top Space Hogs Found

| Location | Size | Contents | Recovery Action |
|----------|------|----------|-----------------|
| **Downloads** | 62.6 GB | Jetson OS images (22 GB), ComfyUI (18 GB), NVIDIA drivers (6 GB), Ubuntu ARM64 (5 GB), large media | Delete old installers, OS images, duplicates |
| **Documents** | 56.9 GB | **ComfyUI: 55 GB** (models/checkpoints — duplicated from `C:\Users\casey\Documents\ComfyUI\`) | Remove if duplicated |
| **.ollama** | 14.6 GB | Local LLM models | `ollama list` → `ollama rm <model>` |
| **polln** | 6.3 GB | Project build artifacts (`target/`, `node_modules/`) | `cargo clean` in each crate |
| **OneDrive** | 5.6 GB | Cloud sync cache | Clear local cache if files online-only |
| **.docker** | 4.8 GB | Docker images/containers | `docker system prune -a --volumes` |
| **.cache** | 1.5 GB | Misc caches | Safe to delete |
| **.cargo** | 1.1 GB | Rust registry cache | `Remove-Item "$env:USERPROFILE\.cargo\registry\cache\*"` and `src\*` |
| **.rustup** | 1.3 GB | Rust toolchains | Remove unused toolchains |

## Safe Cleanup Commands

```powershell
# 1. Rust caches (re-downloads on build)
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\registry\cache\*"
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\registry\src\*"
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\git\*"

# 2. Cargo target directories
Get-ChildItem -Recurse -Directory -Filter "target" | Remove-Item -Recurse -Force

# 3. Docker
docker system prune -a --volumes

# 4. Ollama
ollama list
ollama rm <unused-models>

# 5. Downloads (review first!)
Remove-Item "C:\Users\casey\Downloads\jetson-orin-nano-devkit-super-SD-image_JP6.2.1" -Recurse -Force
Remove-Item "C:\Users\casey\Downloads\ComfyUI" -Recurse -Force
Remove-Item "C:\Users\casey\Downloads\nvidia" -Recurse -Force
Remove-Item "C:\Users\casey\Downloads\ubuntu-22.04-preinstalled-server-arm64+tegra-jetson.img" -Force
Remove-Item "C:\Users\casey\Downloads\the-relay-night-full-production.mp3" -Force

# 6. Documents\ComfyUI (if duplicated)
Remove-Item "C:\Users\casey\Documents\ComfyUI" -Recurse -Force
```

## Results
- **Before**: 952G/953G used (100%), 0 GB free
- **After cleanup**: ~24.9 GB free (927G used)
- **After release build**: ~344 MB free (build artifacts consumed space)
- **After maturin build**: ~2 GB free (wheel + artifacts)

## Key Insight
**Always clean cargo cache AND target dirs BEFORE `cargo build --release`**. The build itself consumes 5-10GB temporarily. Run cleanup, verify free space >5GB, then build.