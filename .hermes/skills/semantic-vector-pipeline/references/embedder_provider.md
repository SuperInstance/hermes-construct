# Embedder Provider Details

The `embedder.py` module looks for the environment variable `EMBEDDER_PROVIDER` to decide which backend to use:

- **local** – deterministic hash‑based embedding (no external API, always available).
- **deepinfra** – calls DeepInfra's `/v1/embeddings` endpoint. Requires `DEEPINFRA_API_KEY` in the environment.
- **cloudflare** – calls Cloudflare's embeddings API. Requires `CLOUDFLARE_API_TOKEN`.

If the variable is set but the required API key is missing, the module logs a warning and falls back to the local hash implementation, ensuring the pipeline never crashes due to missing credentials.

**Typical usage**:
```bash
export EMBEDDER_PROVIDER=deepinfra
export DEEPINFRA_API_KEY=YOUR_KEY
python3 watchdog_vectorization.py
```

When running locally for quick tests, simply omit the variable or set it to `local`.
