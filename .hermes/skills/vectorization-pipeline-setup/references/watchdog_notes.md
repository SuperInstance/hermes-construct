# Watchdog Notes

`watchdog_vectorization.py` uses the `watchdog` Python library to monitor the entire `polln` repository.

- **Debounce**: 1 second to avoid duplicate triggers on rapid file saves.
- **Collection auto‑creation**: On start it checks `QDRANT_URL/collections/superinstance_vectors`. If a 404 is returned, it creates a 32‑dim cosine collection.
- **Environment**: Passes the changed file path via `WATCHED_PATH` and respects `EMBEDDER_PROVIDER` for embedding.
- **Graceful shutdown**: Ctrl+C stops the observer; the script also traps `SIGTERM` when run in a container.

**Common pitfalls**
- Ensure Docker Qdrant is running before starting the watchdog.
- If you see `DeprecationWarning` about `datetime.utcnow`, the latest `vectorization_setup.py` already fixes it.
- For large repos, consider narrowing the watch path to `src/` or `data/` to reduce noise.
