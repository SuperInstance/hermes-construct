---
name: semantic-vector-pipeline
title: Semantic Vector Pipeline
description: End-to-end workflow for embedding repository assets into Qdrant.
version: 1.2
---

## Overview
This skill encapsulates the end-to-end workflow for turning repository assets (code, audio files, RDF triples, etc.) into high-dimensional vectors stored in a Qdrant collection. It covers:
- **Watchdog** (`watchdog_vectorization.py`) that monitors file changes, computes embeddings, and upserts points.
- **Universal embedder** (`embedder.py`) supporting DeepInfra, Cloudflare, or a deterministic local fallback via the `EMBEDDER_PROVIDER` environment variable.
- **Rust-native embedder** (`superinstance-embedder` crate) — local transformer embeddings via Candle (MiniLM, MPNet, BGE, E5) with CUDA acceleration for RTX 4050, hybrid vectors (32-dim domain + 384/768-dim semantic + 96-dim metadata = 512-dim), Qdrant gRPC client, PyO3 Python bindings, and full CLI.
- **Domain extractors** (`musician-soul/extractor.py`, `knowledge-graph/extractor.py`) loaded with `importlib` to handle hyphenated package names.
- **Batch extraction script** (`run_extractions.py`) that seeds the vector store with domain-specific prompts.
- **Launch helper** (`launch.sh`) that starts Qdrant, sets `EMBEDDER_PROVIDER`, and runs the watchdog.
- **Query utility** (`query_qdrant.py`) for similarity search.

## Step-by-Step Procedure
1. **Ensure Qdrant is running** – `docker start qdrant` (or `docker run \`\`\`` if not present).
2. **Choose embedder provider** – export `EMBEDDER_PROVIDER`:
   - `deepinfra` — DeepInfra API (requires `DEEPINFRA_API_KEY`)
   - `cloudflare` — Cloudflare Workers AI (requires `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`)
   - `local` — Deterministic hash fallback (no API keys, 32-dim)
   - `rust` — **NEW**: Rust `superinstance-embedder` binary via Python bindings (requires `superinstance-embedder` built with `python` feature, provides real transformer embeddings with GPU acceleration)
3. **Start the pipeline** – run `./launch.sh`. This launches the watchdog in the background.
4. **Seed the store** – optionally run `python run_extractions.py` to ingest audio prompts and RDF triples.
5. **Search** – `EMBEDDER_PROVIDER=local python query_qdrant.py "your query"` (or `rust` for semantic search).

## Pitfalls & Workarounds
- **Temporary files** (`.hermes-tmp*`) generate invalid UUIDs. The watchdog now skips any path containing `.hermes-tmp`.
- **Hyphenated repo names** cannot be imported with normal `import`. Use `importlib.util.spec_from_file_location` as shown in `run_extractions.py`.
- **Embedding provider missing** – if `EMBEDDER_PROVIDER` is set but the required API key is absent, the embedder falls back to the deterministic hash implementation and logs a warning.
- **Qdrant collection missing** – the watchdog creates the collection on first run; if you manually delete it, restart the watchdog to recreate.
- **Large vectors** – the embedder always returns a 32-dim vector; adjust `embedder.py` if you need a different size.
- **Rust embedder build** – requires Rust toolchain (edition 2021), `candle-core` with optional `cuda` feature for GPU, and Visual Studio Build Tools on Windows for CUDA compilation. Use `cargo build --release --features python` to build the PyO3 extension module.
- **Rust embedder GPU** – CUDA 13.x requires `candle-core` 0.11+ with `cuda` feature; if CUDA unavailable, the embedder falls back to CPU automatically when `fallback_to_cpu=true` in config.
- **Python bindings** – the PyO3 module must be built with matching Python version (abi3-py38 default); install via `pip install -e .` from the embedder crate root or copy the `.so`/`.pyd` to site-packages.
- **PyO3/maturin build** – **Requires `[[lib]] crate-type = ["cdylib"]` in Cargo.toml**. Without this, the wheel builds without the extension module (only Python stubs). Output on Windows is `.pyd` file. Install via `pip install target/wheels/...`.
- **Qdrant Value API** – `Value` type has no `as_i64()`/`as_f64()`. Use `as_str()` and parse: `v.as_str().and_then(|s| s.parse::<u32>().ok())`.
- **Model file existence** – Check for actual files (`config.json`, `model.safetensors`), not just directory existence, to catch failed downloads.
- **Disk space** – Release builds need ~5GB free. Clean cargo cache (`~/.cargo/registry/cache/*`, `src/*`, `git/*`) and all `target/` dirs before building.
- **Windows Path Confusion** – **CRITICAL**: `write_file` tool resolves MSYS-style paths (`/c/Users/...`) incorrectly as `C:\c\Users\...`. Always use **absolute Windows paths** (`C:\Users\...`) when calling `write_file`. The terminal tool handles MSYS paths correctly; `write_file` does not.
- **PowerShell from bash/MSYS** – Running complex PowerShell commands via `powershell -Command "..."` from bash/MSYS terminal fails due to path escaping issues. Better pattern: write `.ps1` script via `write_file` (using Windows paths), then execute via `powershell -ExecutionPolicy Bypass -File "C:\path\to\script.ps1"`.
- **Qdrant Point IDs** – Must be unsigned integers or UUIDs, NOT strings like `"ternary-math"`. Error: `value is not a valid point ID, valid values are either an unsigned integer or a UUID`. Use integer IDs and maintain a name→ID lookup map.
- **PyO3 #[pyclass] constructors** – `#[pyclass]` structs need explicit `#[new]` method in `#[pymethods]` impl block to be instantiable from Python. Without it: `TypeError: No constructor defined for MyClass`.
- **PyO3 return type conversions** – Return types in `#[pymethods]` must match exactly. If method declares `-> PyResult<Vec<f32>>`, don't return `HybridVector` — use `.to_vec()` conversion. Also: `DomainVector` is NOT a `Result` type; it returns directly.
- **Qdrant Client/Server Version Mismatch** – `Qdrant client version 1.17.0 is incompatible with server version 1.19.0`. The client and server minor versions must match. Either pin Qdrant container version to match client, or upgrade qdrant-client Python package. Error: `Failed to obtain server version. Unable to check client-server compatibility.`
- **Qdrant gRPC/HTTP2 Protocol Errors** – When using older client with newer server, can see: `Unknown error h2 protocol error: http2 error MetadataMap { headers: {} }`. Use `prefer_grpc=False` on QdrantClient to force HTTP/1.1, or align client/server versions.
- **Qdrant with Python bindings** – When creating `PyEmbedder` with `qdrant_enabled=True`, the embedder will attempt gRPC connection during init. If Qdrant is not running or version mismatched, it will fail. Recommended pattern: init embedder with `qdrant_enabled=False`, use separate `QdrantClient` for upsert/search operations.
- **Qdrant Point IDs (Critical)** – Must be unsigned integers or UUIDs, **NOT strings** like `"ternary-math"`. Error: `value is not a valid point ID, valid values are either an unsigned integer or a UUID`. Fix: Use integer IDs `(1, 2, 3...)` and maintain a name→ID lookup map in code.
- **Qdrant Client/Server Version Mismatch** – Error: `Qdrant client version 1.17.0 is incompatible with server version 1.19.0`. Client and server minor versions must be within 1. 
  
  **CRITICAL NEW FINDING (2026-08-18):** When versions mismatch, the API methods themselves change signature or disappear:
  - **`.search()` method is GONE** from client 1.17.0 when talking to server 1.19.0. Error: `AttributeError: 'QdrantClient' object has no attribute 'search'`
  - **`.query()` method exists but expects `query_text`** (for fastembed auto-embedding integration), NOT `query=vector`. Error: `TypeError: QdrantFastembedMixin.query() missing 1 required positional argument: 'query_text'`
  
  Mitigation options:
  1. Pin Qdrant Docker version to match client: `docker run qdrant/qdrant:v1.7.x`
  2. Upgrade qdrant-client Python package: `pip install --upgrade qdrant-client`
  3. Use raw REST API via `client.rest` or direct HTTP calls if versions can't be aligned
  4. Use `client.query_points()` instead of `client.search()` — check available methods with `[m for m in dir(client) if not m.startswith('_')]`
- **Qdrant gRPC/HTTP2 Protocol Errors** – With older client + newer server: `Unknown error h2 protocol error: http2 error MetadataMap { headers: {} }`. Fix: Use `prefer_grpc=False` on QdrantClient to force HTTP/1.1, or align client/server versions.

## References
- `references/embedder_provider.md` – detailed env-var handling and fallback logic.
- `references/qdrant_setup.md` – Docker commands, collection schema, and common troubleshooting.
- `references/importlib_extractor_loading.md` – pattern for loading extractors from hyphenated directories.
- `references/rust_embedder_provider.md` – **NEW**: Complete documentation for the Rust embedder crate including build, config, CLI, Python bindings, and all pitfalls from 2026-08-18 session.

## Scripts
- `scripts/run_vector_pipeline.sh` – convenience wrapper that starts Qdrant (if needed) and launches the watchdog.
- `scripts/verify_rust_embedder.sh` – **NEW**: sanity check for `si-embedder` CLI and Python bindings.

---

*This skill is versioned and will be updated as the pipeline evolves.*