# Cloudflare Workers AI — Visual Generation
2026-08-18

## Overview

Cloudflare Workers AI provides reliable, fast image generation via API.

## Working Models

| Model ID | Status | Notes |
|----------|--------|-------|
| @cf/black-forest-labs/flux-1-schnell | ✅ RELIABLE | Fast, good quality, used for 2026-08-18 investigation |
| @cf/black-forest-labs/flux-2-dev | ❌ Issues | May require multipart/form-data format, different API |
| @cf/black-forest-labs/flux-2-klein | Untested | 4B parameter model |
| @cf/stabilityai/stable-diffusion-xl-lightning | Untested | |
| @cf/lykon/dreamshaper-8-lcm | Untested | |
| @cf/bytedance/stable-diffusion-xl-lightning | Untested | |

## API Endpoint

```
POST https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{MODEL_ID}
```

## Authentication

Read from:
`C:\Users\casey\AppData\Roaming\xdg.config\.wrangler\config\default.toml`

Key: `oauth_token`

## Working Payload (flux-1-schnell)

```json
{
  "prompt": "your prompt here"
}
```

**DO NOT** add extra fields like `steps` or `width/height` — this causes HTTP 400 errors for flux-1-schnell.

## Response Format

```json
{
  "success": true,
  "result": {
    "image": "base64_encoded_jpg_string"
  }
}
```

## Critical: Retry Logic

The API frequently returns:
```
HTTP 429: "Capacity temporarily exceeded"
```

**Required retry pattern:**
- Maximum attempts: 8-10
- Delay between retries: 20-30 seconds
- Exponential backoff optional but recommended

## Proven Working Script (2026-08-18)

```python
import urllib.request
import json
import base64
import time

ACCOUNT_ID = "049ff5e84ecf636b53b162cbb580aae6"
MODEL = "@cf/black-forest-labs/flux-1-schnell"

# Read token from wrangler config
config_path = os.path.expanduser(
    "~\\AppData\\Roaming\\xdg.config\\.wrangler\\config\\default.toml"
)

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

# Retry loop
for attempt in range(10):
    try:
        req = urllib.request.Request(
            url, 
            data=json.dumps({"prompt": prompt}).encode('utf-8'),
            headers=headers,
            method='POST'
        )
        with urllib.request.urlopen(req, timeout=120) as response:
            result = json.loads(response.read().decode('utf-8'))
            if result.get('success'):
                img_b64 = result['result']['image']
                img_bytes = base64.b64decode(img_b64)
                # Save to file
                break
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8')
        if "Capacity" in error_body or "429" in str(e.code):
            time.sleep(30)  # Wait and retry
        else:
            raise
```

## ComfyUI Status (Local)

User has **21 checkpoints** installed at:
`C:\Users\casey\Documents\ComfyUI\models\checkpoints\`

**Issue encountered 2026-08-18:**
- Lucineer made broken edits to `main.py`
- Dangling `else:` clauses without matching `if`
- `comfy_aimdo` module imported but not installed

**Fix applied:**
1. Fixed syntax error in conditional chain (try/except/else structure)
2. Created stub modules for `comfy_aimdo` to allow imports

All `comfy_aimdo` usage is guarded by `if comfy.memory_management.aimdo_enabled:` which will be `False`.

## MMX (MiniMax) Status

- Has audio/image/speech generation
- **Subject to credit limits**
- 2026-08-18: music, speech, and image all hit limits

Use as fallback only.
