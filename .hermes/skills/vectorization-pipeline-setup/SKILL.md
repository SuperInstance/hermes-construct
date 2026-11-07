---
name: vectorization-pipeline-setup
description: End-to-end workflow for automatic file embedding into Qdrant using watchdog, universal embedder, and domain extractors.
version: 1.1
tags: [vectorization, qdrant, watchdog, embedder, extraction]
---

# Overview

**Recent updates (2026-08-18)**
- Fixed deprecation warning: switched `datetime.datetime.utcnow()` to `datetime.datetime.now(datetime.UTC)` in `vectorization_setup.py`.
- Added `embedder.py` universal wrapper supporting `local`, `deepinfra`, `cloudflare`, and **`rust`** providers.
- Updated `run_extractions.py` to load hyphenated packages via `importlib.util`.
- `watchdog_vectorization.py` now auto-creates the Qdrant collection if missing and respects `EMBEDDER_PROVIDER` env var.
- `launch.sh` now exports `EMBEDDER_PROVIDER` before starting the watchdog.
- Added reference docs (`embedder_notes.md`, `extractor_notes.md`, `watchdog_notes.md`) and verification scripts.
- **NEW**: Rust `superinstance-embedder` crate with Candle transformers, hybrid vectors (512-dim), Qdrant gRPC client, PyO3 bindings, and `si-embedder` CLI.

# Overview
This skill captures the end-to-end workflow for automatically embedding any file change in the `polln` repository into a Qdrant vector store. It includes:
- A **watchdog** (`watchdog_vectorization.py`) that watches the repo and triggers `vectorization_setup.py`.
- A **universal embedder** (`embedder.py`) that can use DeepInfra, Cloudflare, a deterministic local fallback, or **Rust PyO3 bindings**.
- **Domain extractors** (`musician-soul/extractor.py` and `knowledge-graph/extractor.py`) that produce textual prompts for audio assets and RDF triples.
- A **batch ingestion script** (`run_extractions.py`) that loads the extractors, embeds their output, and upserts into the same collection.
- A **search helper** (`query_qdrant.py`) for semantic lookup.

## Prerequisites
- Docker with the `qdrant/qdrant:latest` image.
- Python 3.13+ with `watchdog` (installed via `pip install watchdog`).
- **For Rust provider**: Rust toolchain (edition 2021), `candle-core` with optional `cuda` feature, Visual Studio Build Tools on Windows.
- Environment variables:
  - `QDRANT_URL` (default `http://localhost:6333`).
  - `EMBEDDER_PROVIDER` – `local` (fallback), `deepinfra`, `cloudflare`, or `rust`.
  - Provider API keys (`DEEPINFRA_API_KEY`, `CLOUDFLARE_API_TOKEN`) when using remote services.

## Step-by-Step Procedure
1. **Start Qdrant** – `docker start qdrant` (or run the `docker run ...` command if the container does not exist).
2. **Launch the watchdog** – `export EMBEDDER_PROVIDER=local && python3 watchdog_vectorization.py &` (or use `./launch.sh`).
3. **Make a file change** – any edit under the repo triggers vectorization automatically.
4. **Run batch extraction** – `python3 run_extractions.py` to ingest domain-specific prompts.
5. **Search** – `python3 query_qdrant.py "your query"`.
6. **Stop the watchdog** – `kill <pid>` or `pkill -f watchdog_vectorization.py`.

## Pitfalls & Gotchas
- **Deprecation warning** – replace `datetime.datetime.utcnow()` with `datetime.datetime.now(datetime.UTC)` (patched in `vectorization_setup.py`).
- **Hyphenated package imports** – Python cannot import `musician-soul` directly; use `importlib.util` to load the module from its file path (implemented in `run_extractions.py`).
- **Background process management** – always start the watchdog with `notify_on_complete=true` or keep the PID to kill later; otherwise the process may linger.
- **Empty Qdrant responses** – ensure the collection exists; the watchdog creates it on start if missing.
- **Embedding provider selection** – if `EMBEDDER_PROVIDER` is unset or invalid, the embedder falls back to the deterministic hash-based vector.
- **Rust embedder build** – requires `cargo build --release --features python,qdrant`; PyO3 bindings need `crate-type = ["cdylib", "rlib"]` in Cargo.toml and a `pyproject.toml` for maturin.
- **Rust embedder GPU** – CUDA 13.x requires `candle-core` 0.11+ with `cuda` feature; if CUDA unavailable, falls back to CPU when `fallback_to_cpu=true`.
- **Disk space** – Rust release builds need ~5GB free; clean `~/.cargo/registry/cache/*` and `target/` before building.

## References
- `references/embedder_notes.md` – details on provider selection and fallback (updated with `rust` provider).
- `references/extractor_notes.md` – description of audio and RDF extractors.
- `references/watchdog_notes.md` – watchdog behavior, debounce, collection auto-creation.
- `references/rust_embedder_provider.md` (in `semantic-vector-pipeline` skill) – complete Rust embedder documentation.

## Templates
- `templates/launch.sh` – starter script for Qdrant + watchdog (updated with provider options).

## Scripts
- `scripts/verify_vector.sh` – quick sanity-check script for upserted vectors.
- `scripts/verify_rust_embedder.sh` (in `semantic-vector-pipeline` skill) – **NEW**: sanity check for `si-embedder` CLI and Python bindings.

---

*Skill designed for the user's "direct, no-fluff" style – all steps are concise, ready-to-copy commands.*