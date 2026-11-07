# Cloudflare Sync Bridge Reference

## Purpose
Upload vector embeddings to Cloudflare Vectorize (or Workers KV) for edge‑side semantic search.

## Expected Environment
- `CLOUDFLARE_API_TOKEN` set in the process environment (never hard‑coded).
- `CLOUDFLARE_ACCOUNT_ID` optionally set for multi‑account setups.
- Python `requests` library available (standard in the Hermès environment).

## Minimal Implementation (`cloudflare_sync.py`)
```python
import os, json, requests

API_TOKEN = os.getenv("CLOUDFLARE_API_TOKEN")
ACCOUNT_ID = os.getenv("CLOUDFLARE_ACCOUNT_ID", "<default-id>")
BASE_URL = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/vectorize"


def upload_vectors(collection: str, vectors: list[dict]):
    url = f"{BASE_URL}/collections/{collection}/vectors"
    headers = {
        "Authorization": f"Bearer {API_TOKEN}",
        "Content-Type": "application/json",
    }
    resp = requests.post(url, headers=headers, json={"vectors": vectors})
    resp.raise_for_status()
    return resp.json()

if __name__ == "__main__":
    import sys, pathlib
    collection = sys.argv[1]
    data_path = pathlib.Path(sys.argv[2])
    vectors = json.loads(data_path.read_text())
    result = upload_vectors(collection, vectors)
    print("Upload result:", result)
```

## Key Points
- **Authentication**: Use Bearer token from env; never commit token to repo.
- **Batch size**: Cloudflare API accepts up to 10 k vectors per request – batch accordingly.
- **Error handling**: `raise_for_status()` surfaces HTTP errors; catch and log for retries.
- **Idempotency**: Include a stable `id` field in each vector payload to avoid duplicates.

## Pitfalls & Workarounds
- **Missing token** – script exits with clear message; ensure env var is exported before running.
- **Rate limits** – respect `Retry-After` header; implement exponential backoff if needed.
- **Network connectivity** – verify outbound HTTPS is allowed from the host.

## Integration
- Called after `watchdog_vectorization.py` finishes embedding a file.
- Can be wrapped in a higher‑level orchestrator that batches multiple files before upload.

---
*Generated from session 2026‑08‑17*