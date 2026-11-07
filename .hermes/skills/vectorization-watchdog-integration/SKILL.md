---
name: vectorization-watchdog-integration
title: Vectorization Watchdog Integration
description: End-to-end pattern for auto‑vectorizing repo files, storing them in Qdrant, and launching the runtime.
version: 1.0
---

# Overview
This skill captures the end‑to‑end pattern for automatically vectorizing repository files and storing them in a Qdrant collection. It includes:
- A **watchdog** (`watchdog_vectorization.py`) that monitors the repo root, triggers `vectorization_setup.py` on changes, and ensures the Qdrant collection exists.
- A **vectorization script** (`vectorization_setup.py`) that reads the changed file, creates a deterministic UUID from a SHA‑256 hash, generates a 32‑dim placeholder embedding, and upserts the point into Qdrant.
- A **launch helper** (`launch.sh`) that starts Qdrant (if needed), the watchdog, and placeholder services with cleanly‑handled shutdown.
- An **integration design** document (`integration_design.md`) that describes how this runtime fits into the SuperInstance ecosystem (Pincher, LLM‑as‑Compiler, Lever‑Runner, etc.).

## Core Steps
1. **Ensure Qdrant collection** – on start, `watchdog_vectorization.py` checks for `superinstance_vectors`; if missing it creates a 32‑dim cosine collection.
2. **Watch for file changes** – using `watchdog` library, debounce events, and invoke `vectorization_setup.py` with `WATCHED_PATH` env var.
3. **Compute deterministic ID** – `vectorization_setup.py` hashes file content, takes the first 32 hex chars, converts to a UUID (valid Qdrant ID).
4. **Generate embedding** – placeholder deterministic embedding derived from the SHA‑256 digest (32 floats in `[0,1)`). Replaceable with DeepInfra/Cloudflare model later.
5. **Upsert point** – HTTP `PUT` to `/collections/<name>/points?wait=true` with payload containing `id`, `vector`, and metadata (`path`, `type`, `timestamp`).
6. **Launch script** – `launch.sh` starts Qdrant container if not running, runs the watchdog in background, and traps exit to clean up.

## Pitfalls & Gotchas
- **UUID format** – Qdrant only accepts unsigned integers or UUID strings. Using the raw SHA‑256 hex caused `400 Bad Request`. Fixed by converting the first 32 hex chars to a UUID.
- **Collection existence** – early runs failed with `404 Not Found`. Added auto‑creation logic in the watchdog.
- **Debounce timing** – too aggressive debounce can miss rapid successive edits; adjust `self.debounce` as needed.
- **Embedding placeholder** – deterministic hash‑based vectors are fine for testing but not semantically meaningful. Swap `generate_embedding` with a real model when ready.
- **Environment variables** – `QDRANT_URL` defaults to `http://localhost:6333`. Override for remote deployments.

## References
- `references/watchdog_flow.md` – detailed session transcript and reasoning.
- `templates/launch.sh` – ready‑to‑use launch script.
- `integration_design.md` – high‑level architecture overview.

## Usage
1. Place `watchdog_vectorization.py` and `vectorization_setup.py` at the repo root.
2. Run `./launch.sh` (or `bash launch.sh`).
3. Edit any file; the vector will appear in Qdrant (`curl -X POST …/points/scroll`).
4. Replace `generate_embedding` with a real model when available.

---
*This skill is versioned for the SuperInstance project and will be updated as the embedder API evolves.*