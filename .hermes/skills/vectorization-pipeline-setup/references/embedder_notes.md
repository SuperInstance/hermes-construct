# Embedder Notes

The universal `embedder.py` selects an embedding backend based on the `EMBEDDER_PROVIDER` environment variable.

- **local** – deterministic SHA‑256 based 32‑dim vector (fallback, no network).
- **deepinfra** – calls DeepInfra `/v1/embeddings` endpoint. Requires `DEEPINFRA_API_KEY`.
- **cloudflare** – uses Cloudflare Images/AI embeddings API. Requires `CLOUDFLARE_API_TOKEN`.
- **rust** – uses local `superinstance-embedder` PyO3 bindings (requires `superinstance-embedder` built with `python` feature). Returns 512-dim hybrid vectors (32-dim domain + 384/768-dim semantic + 96-dim metadata). GPU-accelerated via Candle/CUDA on RTX 4050, CPU fallback. No API keys needed. Offline-capable.

The embedder returns a list of float values. Dimension depends on provider: 32 (local), 384/768 (deepinfra/cloudflare), 512 (rust). If the provider request fails, it falls back to the local hash implementation.