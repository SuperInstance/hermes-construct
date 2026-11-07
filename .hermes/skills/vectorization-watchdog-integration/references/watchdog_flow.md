## Session Walkthrough (2026‑08‑18)

- Cloned SuperInstance repos (`git-agent-codespace`, `musician-soul`, `knowledge-graph`, `superinstance-embedder`).
- Implemented `watchdog_vectorization.py` to auto‑create Qdrant collection `superinstance_vectors` if missing.
- Fixed UUID generation in `vectorization_setup.py` (SHA‑256 → first 32 hex chars → UUID) to satisfy Qdrant ID format.
- Added deterministic placeholder embedding (32‑dim floats from hash digest).
- Verified upsert works: `curl -X POST …/points/scroll` returned three points, including `README.md`.
- Created `launch.sh` script to start Qdrant container (if not running), launch watchdog, and cleanly shut down.
- Updated `integration_design.md` with checklist and design notes.
- Added skill `vectorization-watchdog-integration` to capture this pattern for future sessions.

### Key Commands Used
```bash
# Start Qdrant (if not already running)
docker run -d --name qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant:latest

# Launch the runtime
./launch.sh
```

### Lessons Learned
- Always ensure Qdrant collection exists before upserting points.
- Use UUIDs for point IDs; raw hex strings cause `400 Bad Request`.
- Debounce interval can be tuned (`self.debounce = 1.0`).
- Placeholder embeddings are fine for testing; swap with real model later.
