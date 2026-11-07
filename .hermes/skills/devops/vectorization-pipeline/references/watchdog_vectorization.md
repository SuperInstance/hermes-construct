# Watchdog Vectorization Overview

- **File:** `watchdog_vectorization.py`
- **Purpose:** Monitor the repository root for any file changes.
- **Key behaviours:**
  - Reads `QDRANT_URL` and `COLLECTION` from env (defaults to `http://localhost:6333` and `superinstance_vectors`).
  - On start, attempts to create the collection if it does not exist (POST to `/collections`).
  - Uses `watchdog.observers.Observer` and a custom `FileSystemEventHandler`.
  - Skips any path containing `.hermes-tmp` to avoid invalid UUID errors.
  - When a change is detected, sets `WATCHED_PATH` env var and runs `python3 vectorization_setup.py` via `subprocess.run`.
- **Design notes:**
  - The collection creation payload uses `{"vectors": {"size": 32, "distance": "Cosine"}}`.
  - The script runs in the background; `launch.sh` starts it with `nohup`‑style handling via the Hermes process manager.
- **Pitfalls addressed:**
  - Previously attempted to upsert temporary files, causing Qdrant `400 Bad Request` due to non‑UUID IDs. Now filtered early.
  - Auto‑creation ensures a fresh environment works without manual Qdrant setup.
