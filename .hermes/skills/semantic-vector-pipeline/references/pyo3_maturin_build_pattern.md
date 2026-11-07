# PyO3/maturin Build Pattern for superinstance-embedder — 2026-08-18

## Cargo.toml Configuration

```toml
[package]
name = "superinstance-embedder"
version = "0.1.0"
edition = "2021"

[lib]
name = "superinstance_embedder"
crate-type = ["cdylib"]  # REQUIRED for PyO3 extension module

[features]
default = ["candle"]
candle = ["candle-core", "candle-nn", "candle-transformers", "tokenizers", "half"]
onnx = ["ort"]
qdrant = ["qdrant-client", "tokio"]
python = ["pyo3/abi3-py38", "pyo3/extension-module"]
server = ["axum", "tower-http", "serde_json"]

[dependencies]
# ... core deps ...

pyo3 = { version = "0.23", features = ["abi3-py38", "extension-module"], optional = true }
maturin = { version = "1.8", optional = true }
```

## Build Commands

```bash
# Option 1: maturin (recommended)
maturin build --release --features python
pip install target/wheels/superinstance_embedder-*.whl

# Option 2: cargo + maturin develop
cargo build --release --features python
maturin develop --release

# Verify
python -c "import superinstance_embedder as sie; e = sie.PyEmbedder(); print(e.embed_hybrid('test'))"
```

## Common Failures

| Symptom | Cause | Fix |
|---------|-------|-----|
| Wheel builds but no `.pyd`/`.so` inside | Missing `crate-type = ["cdylib"]` in `[[lib]]` | Add `[[lib]] crate-type = ["cdylib"]` |
| `ModuleNotFoundError: No module named 'superinstance_embedder'` | Not installed in current Python env | `pip install target/wheels/...` or `maturin develop` |
| `ImportError: DLL load failed` | Python version mismatch | Use `abi3-py38` feature for broad compatibility |
| `cargo build` works but `maturin` fails | maturin not installed | `cargo install maturin` |

## Windows-Specific Notes

- Output file: `superinstance_embedder.pyd` (not `.so`)
- Requires Visual Studio Build Tools for Rust compilation
- Python 3.13 compatible with `abi3-py38`
- Disk space: ~5GB free needed for release build + wheel

## PyEmbedder API (Exposed to Python)

```python
class PyEmbedder:
    def embed_hybrid(self, text: str) -> list[float]      # 512-dim hybrid vector
    def embed_semantic(self, text: str) -> list[float]     # 384/768-dim semantic only
    def embed_batch(self, texts: list[str]) -> list[list[float]]
    def search(self, query: str, limit: int = 10) -> list[dict]
    def upsert(self, texts: list[str], payloads: list[dict] = None) -> bool
    def domain_vector(self, text: str) -> list[float]      # 32-dim domain vector
    def health_check(self) -> dict
```

## Config Integration

The Python bindings read `EmbedderConfig` from:
1. `config.json` in current directory
2. `~/.config/superinstance-embedder/config.json`
3. Defaults (CPU, MiniLM-L6-v2, Qdrant disabled)

```python
# Override config programmatically
e = sie.PyEmbedder(config_path="custom_config.json")
```