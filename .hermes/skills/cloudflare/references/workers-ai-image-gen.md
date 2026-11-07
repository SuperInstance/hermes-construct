# Workers AI: Image Generation Reference

## Key Discoveries (2026-08-18)

### Model API Differences

**FLUX.1 models** (like `@cf/black-forest-labs/flux-1-schnell`) accept simple JSON:
```python
import urllib.request
import json

url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/@cf/black-forest-labs/flux-1-schnell"

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

data = json.dumps({
    "prompt": "Your image prompt here",
}).encode('utf-8')

# Response contains base64 image in result['image']
```

**FLUX.2 models** (like `@cf/black-forest-labs/flux-2-klein-4b`, `@cf/black-forest-labs/flux-2-dev`) require **multipart format**, not simple JSON. These will fail with:
```
required properties at '/' are 'multipart'
```

### Token Extraction from Wrangler Config

On Windows, OAuth token is stored at:
```
C:\Users\<username>\AppData\Roaming\xdg.config\.wrangler\config\default.toml
```

Parse manually (no `toml` dependency needed):
```python
config_path = os.path.expanduser("~\\AppData\\Roaming\\xdg.config\\.wrangler\\config\\default.toml")

token = ""
if os.path.exists(config_path):
    with open(config_path, 'r') as f:
        for line in f:
            line = line.strip()
            if line.startswith('oauth_token'):
                if '=' in line:
                    parts = line.split('=', 1)
                    if len(parts) == 2:
                        token = parts[1].strip().strip('"').strip("'")
                        break
```

### Available Image Models (2026-08-18)

From `wrangler ai models list`:

| Model ID | Description | Working with JSON API |
|----------|-------------|----------------------|
| `@cf/black-forest-labs/flux-1-schnell` | FLUX.1 [schnell] - 12B params | ✅ YES |
| `@cf/black-forest-labs/flux-2-dev` | FLUX.2 [dev] - highly realistic | ⚠️ Needs multipart |
| `@cf/black-forest-labs/flux-2-klein-9b` | FLUX.2 [klein] 9B | ⚠️ Needs multipart |
| `@cf/black-forest-labs/flux-2-klein-4b` | FLUX.2 [klein] ultra-fast distilled | ⚠️ Needs multipart |
| `@cf/bytedance/stable-diffusion-xl-lightning` | SDXL-Lightning - very fast | Untested |
| `@cf/lykon/dreamshaper-8-lcm` | Dreamshaper - photorealism | Untested |
| `@cf/leonardo/phoenix-1.0` | Phoenix 1.0 - great prompt adherence | Untested |
| `@cf/leonardo/lucid-origin` | Lucid Origin - adaptable/prompt-responsive | Untested |
| `@cf/stabilityai/stable-diffusion-xl-base-1.0` | SDXL Base | Untested |

### Example Working Code

```python
import os
import json
import base64
import urllib.request

ACCOUNT_ID = "your-account-id"

# Get token (see extraction above)
token = "..."

prompt = "Deep ocean abyssal zone, hermit crab captain, bioluminescent creatures"

model = "@cf/black-forest-labs/flux-1-schnell"
url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{model}"

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

data = json.dumps({"prompt": prompt}).encode('utf-8')

req = urllib.request.Request(url, data=data, headers=headers, method='POST')

with urllib.request.urlopen(req, timeout=180) as response:
    result = json.loads(response.read().decode('utf-8'))
    if result.get('success') and result.get('result', {}).get('image'):
        img_bytes = base64.b64decode(result['result']['image'])
        with open("output.png", 'wb') as f:
            f.write(img_bytes)
```

### Rate Limits

FLUX.1-schnell returned HTTP 429 (Too Many Requests) after ~3-4 rapid consecutive calls. Add delays between generations.

### Proven Retry Pattern (2026-08-18)

Use this exponential backoff retry loop when calling the API:

```python
success = False
attempts = 0
max_attempts = 10

while not success and attempts < max_attempts:
    attempts += 1
    try:
        req = urllib.request.Request(url, data=data, headers=headers, method='POST')
        with urllib.request.urlopen(req, timeout=120) as response:
            result = json.loads(response.read().decode('utf-8'))
            
            if result.get('success') and result.get('result', {}).get('image'):
                # Success - process image
                success = True
                
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8')
        if "Capacity" in error_body or "429" in str(e.code) or "Too Many" in error_body:
            # Capacity issue - wait and retry
            print(f"  ⏳ Capacity issue (attempt {attempts}/{max_attempts}), waiting 30s...")
            time.sleep(30)
        elif "401" in str(e.code):
            # Auth error - token may have expired
            print(f"  ❌ Auth error (401) - token may have expired")
            # Don't retry auth errors - need new token
            success = True  # Mark as handled so we stop
        else:
            print(f"  ❌ HTTP Error {e.code}")
            # Don't retry other errors
            success = True
```

**Key Decisions from 2026-08-18:**
- 10 maximum attempts (generous, gives capacity time to recover)
- 30-second delay between retries (Cloudflare capacity is transient but slow to recover)
- Auth errors (401) do NOT retry — the token may have rotated or expired
- Capacity errors (429, "Capacity temporarily exceeded") ARE retried

---

## Cross-Reference: User's Image Generation Providers

This user has multiple image generation options:

1. **MMX (MiniMax) CLI** - `mmx image generate`
   - Integrated CLI tool
   - Can hit credit limits quickly for heavy usage

2. **Cloudflare Workers AI** - As documented above
   - FLUX.1 works with simple JSON API
   - FLUX.2 needs multipart

3. **DeepInfra** - User mentioned availability (API key location unknown)
   - FLUX.1-dev, FLUX.1-schnell, SD3, etc.

4. **Local ComfyUI** - 21 checkpoints at `C:\Users\casey\Documents\ComfyUI\`
   - Best for highest quality and unlimited generation
   - Local GPU (RTX 4050)
