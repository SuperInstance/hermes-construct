---
name: vectorization-pipeline
description: End-to-end file change detection, deterministic embedding, and Qdrant upsert workflow.
summary: Turn any repository file change into a vector stored in Qdrant, with auto‑creation and temporary‑file handling.
---

## Overview
This skill encodes the standard workflow we use in the SuperInstance workspace to turn any file change into a dense vector stored in a Qdrant collection.

1. **Watchdog** (`watchdog_vectorization.py`) monitors the repository root. It creates the collection `superinstance_vectors` on‑first run (32‑dim, cosine) and skips any temporary files matching `*.hermes‑tmp*`.
2. When a change is detected, the watchdog sets `WATCHED_PATH` and invokes `vectorization_setup.py`.
3. **Vectorization Setup** reads the file, computes a SHA‑256 hash, converts the first 32 hex chars into a UUID (valid Qdrant point ID), generates a deterministic 32‑dim embedding (hash‑derived), infers a payload type, and upserts the point via the Qdrant REST API.
4. The pipeline is idempotent – re‑processing the same content yields the same UUID and vector, preventing duplicate points.

**Advanced Evolution (NEW)**: The `semantic-vector-pipeline` skill extends this with multi-provider embedders (DeepInfra, Cloudflare, local hash) and the `superinstance-embedder` Rust crate provides local transformer embeddings (MiniLM, MPNet, BGE, E5) with CUDA acceleration, hybrid 512-dim vectors (32 domain + 384/768 semantic + 96 metadata), Qdrant gRPC client, and PyO3 Python bindings. See `semantic-vector-pipeline` skill for the evolved architecture.

## Key Steps
- Ensure Docker is running and the Qdrant container is available on `localhost:6333`.
- Run `./launch.sh` to start Qdrant (if not already) and the watchdog in the background.
- Use `curl` or the provided Python scripts to query the collection.

## Pitfalls & Work‑arounds
- **Temporary files** (`.hermes‑tmp*`) produce invalid UUIDs. The pipeline now explicitly skips them (see line 67 in `vectorization_setup.py`).
- **Collection missing** – the watchdog auto‑creates the collection; if you start the watchdog before Qdrant is up, it will retry until the service is reachable.
- **Embedding size mismatch** – the placeholder embedding is 32‑dim; if you switch to a real model, update `COLLECTION` config to match the new dimension.
- **UUID generation** – using `uuid.UUID(hash[:32])` guarantees a valid unsigned‑integer or UUID string for Qdrant.
- **Production upgrade** – for semantic search quality, migrate to `semantic-vector-pipeline` with `EMBEDDER_PROVIDER=rust` for local transformer embeddings (512-dim hybrid vectors, GPU accelerated, no API keys).

## References
- `references/watchdog_vectorization.md` – detailed code walk‑through and rationale.
- `references/vectorization_setup.md` – embedding algorithm and Qdrant upsert payload.
