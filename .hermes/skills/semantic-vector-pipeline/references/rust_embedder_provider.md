# Rust Embedder Provider — Local Transformer Embeddings

## Overview
The `superinstance-embedder` crate provides a **local-first, GPU-accelerated** embedding provider for the semantic vector pipeline. It replaces API-dependent providers (DeepInfra, Cloudflare) with on-device transformer inference via Candle (Rust ML framework).

## Integration Point
In `embedder.py`, add a new provider branch:

```python
elif provider == "rust":
    from superinstance_embedder import PyEmbedder
    embedder = PyEmbedder(config_path)  # or PyEmbedder.default()
    # embed_hybrid(text, domain) -> list[float] (512-dim)
    # embed_semantic(text) -> list[float] (384/768-dim)
    # search(vector, limit) -> list[dict]
    # upsert(id, text, domain, payload) -> None
```

## Provider Comparison

| Aspect | deepinfra | cloudflare | local (hash) | **rust (NEW)** |
|--------|-----------|------------|--------------|----------------|
| **Latency** | ~200-500ms | ~100-300ms | ~1ms | ~10-50ms (GPU) |
| **Privacy** | Cloud | Cloud | Local | **Local** |
| **Dimensions** | 384/768 | 384/768 | 32 | **512 (hybrid)** |
| **Semantic quality** | High | High | None | **High** |
| **Domain awareness** | No | No | Keywords | **32-dim taxonomy** |
| **Cross-domain synergy** | No | No | No | **Yes** |
| **GPU acceleration** | N/A | N/A | No | **CUDA (RTX 4050)** |
| **Cost** | $/token | $/token | Free | **Free (hardware)** |
| **Offline** | No | No | Yes | **Yes** |

## Hybrid Vector Advantage
The Rust embedder produces **512-dim hybrid vectors**:
- **32-dim domain**: SuperInstance taxonomy (ternary-math → synergy)
- **384/768-dim semantic**: MiniLM/MPNet/BGE/E5 transformer output
- **96-dim metadata hash**: Deterministic content fingerprint

This enables:
1. **Fast domain filtering** — first 32 dims for coarse routing
2. **Rich semantic similarity** — transformer embeddings for meaning
3. **Exact deduplication** — metadata hash for identity
4. **Cross-domain synergy detection** — domain mismatch + semantic match = innovation signal

## Build Requirements
```bash
# Prerequisites
rustup update stable
# Windows: Visual Studio Build Tools 2022 + Windows 10 SDK
# CUDA: NVIDIA CUDA Toolkit 12.x (for candle-cuda feature)

# Build
cd /c/Users/casey/polln/superinstance-embedder
cargo build --release --features python,qdrant

# Install Python module
pip install -e .  # or: maturin develop --release
```

## Configuration
`embedder_config.toml` (auto-generated on first run):
```toml
[model]
model_type = "MiniLmL6V2"  # or MpnetBaseV2, BgeSmallEnV15, E5SmallV2
max_length = 512
normalize = true
pooling = "Mean"

[device]
use_cuda = true
cuda_device_id = 0
fallback_to_cpu = true

[hybrid]
domain_weight = 0.3
semantic_weight = 0.6
metadata_weight = 0.1
semantic_dim = 384  # 768 for MPNet

[qdrant]
url = "http://localhost:6334"
collection_name = "superinstance"
vector_size = 512  # 32 + semantic_dim + 96
distance = "Cosine"
```

## Usage in Pipeline
```bash
# Set provider
export EMBEDDER_PROVIDER=rust

# Start watchdog (uses PyEmbedder internally)
./launch.sh

# Or direct CLI
si-embedder embed "agent-jam: collaborative music generation" --domain agent-music
si-embedder search "music timing" --limit 10
si-embedder synergies --domain agent-music --threshold 0.7
```

## Pitfalls & Workarounds (Session 2026-08-18)

### 1. Rust Edition Must Be 2021+
**Problem**: `async fn` not permitted in Rust 2015
**Fix**: Ensure `Cargo.toml` has `edition = "2021"` under `[package]`. The workspace root must also not force edition 2015.

### 2. qdrant_client Value API Changes
**Problem**: `Value` type has no `as_i64()` or `as_f64()` methods
**Fix**: Use `as_str()` and parse:
```rust
tests: payload.get("tests").and_then(|v| v.as_str().and_then(|s| s.parse::<u32>().ok())).unwrap_or(0)
```

### 3. Windows Path Escaping in Python Subprocess
**Problem**: `dest.display()` produces `C:\Users\...` which breaks Python string escaping (`\U` interpreted as unicode escape)
**Fix**: Use raw strings in Python script:
```rust
let dest_str = dest.to_string_lossy().replace('\\', "\\\\");
let python_script = format!(r#"local_dir = r"{}""#, dest_str);
```

### 4. Model File Existence Check
**Problem**: Directory exists but is empty (failed download) → tokenizer load fails
**Fix**: Check for actual model files, not just directory:
```rust
let model_files_exist = model_path.join("config.json").exists() 
    && model_path.join("model.safetensors").exists() 
    && tokenizer_path.exists();
if !model_files_exist { self.download_model(&model_type, &model_path).await?; }
```

### 5. Disk Space for Release Builds
**Problem**: `cargo build --release` needs ~5GB free space (incremental compilation artifacts)
**Fix**: Clean cargo cache and target dirs before build:
```bash
rm -rf ~/.cargo/registry/cache/* ~/.cargo/registry/src/* ~/.cargo/git/*
rm -rf target/
```

### 6. Qdrant gRPC vs REST Ports
**Note**: gRPC port is **6334**, REST is **6333**. The Rust embedder uses gRPC client (`Qdrant::from_url`). Ensure Qdrant started with gRPC enabled (default in v1.7+).

### 7. Default Qdrant Config Should Be Disabled
**Fix**: Set `enabled: false` in `QdrantConfig::default()` so CLI works without running Qdrant:
```rust
impl Default for QdrantConfig {
    fn default() -> Self {
        Self { enabled: false, ... }
    }
}
```

### 8. CUDA 13.x / cudarc 0.13 Incompatibility
**Problem**: CUDA 13.3 not supported by cudarc 0.13.9
**Workaround**: CPU fallback implemented via `Device::Cpu` when `fallback_to_cpu=true`. GPU requires newer candle/cudarc or CUDA toolkit downgrade.

### 9. PyO3/maturin Build Configuration
**Problem**: Wheel missing `.pyd` extension module
**Fix**: Add `[lib]` section to `Cargo.toml` and create `pyproject.toml`:
```toml
# Cargo.toml
[lib]
name = "superinstance_embedder"
crate-type = ["cdylib", "rlib"]

# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "superinstance-embedder"
...

[tool.maturin]
features = ["python"]
compatibility = "off"
bindings = "pyo3"
```

### 10. Domain Encoder Keyword Matching
The 32 SuperInstance domains are matched by splitting domain names on `-` and checking if any keyword appears in the input text:
```rust
for (idx, domain) in DOMAINS.iter().enumerate() {
    for kw in domain.split('-') {
        if text_lower.contains(kw) { vec[idx] = (vec[idx] + 0.5).min(1.0); }
    }
}
```
This enables domain detection like "tensor algebra" → `ternary-math` + `ternary-ml`.

## Testing
```bash
# Unit tests (no Qdrant needed)
cargo test --lib

# Integration test (requires Qdrant)
docker run -d -p 6334:6334 qdrant/qdrant
cargo test --features qdrant --test integration_test

# Python smoke test
python -c "from superinstance_embedder import PyEmbedder; e=PyEmbedder(); print(e.embed_semantic('test'))"
```

## Architecture Summary (2026-08-18)
- **10 modules**: config, domain, error, models, embeddings, qdrant, cli, python, lib, main
- **Feature gates**: candle (default), onnx, qdrant, python, server
- **Binary**: `si-embedder` with embed/index/search/synergies/serve/domain/config commands
- **PyO3 bindings**: embed(), search(), domain_vector(), health_check(), index_directory()
- **32 SuperInstance domains** with keyword-based detection
- **Hybrid vector fusion**: weighted concatenation of domain + semantic + metadata