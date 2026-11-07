# Windows Rust Development Workflow for SuperInstance

## Context
SuperInstance development on Windows (MSYS/Git Bash) with PowerShell for system operations. Key differences from Linux workflows.

## Disk Space Emergency Cleanup

When cargo/rustup operations fail silently at 95%+ disk usage:

```powershell
# Clean cargo cache (run in PowerShell)
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\registry\cache\*"
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\registry\src\*"
Remove-Item -Recurse -Force "$env:USERPROFILE\.cargo\git\*"

# Clean all target directories recursively
Get-ChildItem -Recurse -Directory -Filter "target" | Remove-Item -Recurse -Force

# Verify free space
Get-PSDrive C | Select-Object Used,Free
```

Need ~5GB free for cargo builds. ~10GB free after cleanup.

## PowerShell vs Bash Command Chaining

**WRONG (bash syntax):**
```bash
cd /c/Users/casey/polln/superinstance-embedder && cargo build --release
```

**CORRECT (PowerShell):**
```powershell
cd C:\Users\casey\polln\superinstance-embedder; cargo build --release
```

Use `;` not `&&` for command chaining in PowerShell.

## Path Resolution in Hermes Tools

### write_file tool
- Relative paths like `/c/Users/casey/...` resolve to `C:\c\Users\...` (extra `c\`)
- **Use absolute Windows paths:** `C:\Users\casey\polln\superinstance-embedder\src\models.rs`
- Verify with `ls` after writing

### terminal tool
- POSIX paths work: `/c/Users/casey/...`
- Windows paths work: `C:\Users\casey\...`
- Both acceptable

## Rust Toolchain Management

```powershell
# Check installed toolchains
rustup toolchain list

# Install stable (may fail due to network/proxy/disk)
rustup install stable

# Update stable
rustup update stable

# Set default
rustup default stable

# Show active toolchain
rustup show
```

If `rustup install stable` fails (exit 1, minimal output):
- Check network/proxy
- Ensure disk space (>5GB free)
- Try `rustup update stable` if already installed

## Cargo Build Commands

```powershell
# Check compilation (fast)
cargo check --features candle,qdrant

# Check with specific edition
cargo check --edition 2021

# Release build
cargo build --release --features candle,qdrant

# Test
cargo test --workspace

# Clean
cargo clean
```

## CUDA Build Requirements

- Visual Studio Build Tools 2019/2022 with C++ workload
- CUDA Toolkit 11.8+ (12.x recommended)
- Current blocker: CUDA 13.3 unsupported by cudarc 0.13.9
- CPU fallback: `Device::Cpu` in candle-core

## PyO3/maturin Setup

```powershell
# Install maturin
cargo install maturin

# Build Python bindings
maturin develop --release --features python,qdrant

# Or build wheel
maturin build --release --features python,qdrant
```

## Qdrant Local Development

```powershell
# Run Qdrant in Docker (if available)
docker run -p 6334:6334 qdrant/qdrant

# Or download binary from https://github.com/qdrant/qdrant/releases
```

## Verification Checklist

- [ ] Disk space >5GB free
- [ ] Rust toolchain installed (`rustup toolchain list`)
- [ ] Cargo check passes (`cargo check --features candle,qdrant`)
- [ ] Edition 2021 respected (no `async fn` errors)
- [ ] Release build succeeds (`cargo build --release --features candle,qdrant`)
- [ ] CLI runs (`si-embedder --help`)
- [ ] PyO3 bindings build (`maturin develop --release --features python,qdrant`)
- [ ] Qdrant accessible (localhost:6334)