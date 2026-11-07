import os
import sys
import json
import base64

ACCOUNT_ID = "049ff5e84ecf636b53b162cbb580aae6"

# Try the actual path from wrangler whoami
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

if not token:
    token = os.environ.get('CLOUDFLARE_API_TOKEN', '')

if not token:
    print(f"No token found")
    sys.exit(1)

print(f"Token found: {token[:20]}...")

import urllib.request

# Try FLUX.2-klein-4b (ultra-fast) or FLUX.2-dev
# Let's try multiple models
models_to_try = [
    "@cf/black-forest-labs/flux-2-klein-4b",    # Ultra-fast distilled
    "@cf/black-forest-labs/flux-2-klein-9b",    # Full
    "@cf/black-forest-labs/flux-1-schnell",      # FLUX.1
    "@cf/bytedance/stable-diffusion-xl-lightning",  # SDXL Lightning
]

prompt = """Deep ocean abyssal zone at night, cinematic atmospheric lighting, 
hermit crab captain wearing a fishing boat captain's hat inside a large rusted ship hull shell,
bioluminescent jellyfish and anglerfish swimming in the darkness,
dark blue and black color palette with glowing cyan and orange accents,
underwater photography style, ultra realistic, 8k resolution,
moody atmospheric, film noir underwater, 'Sound of the Abyss' album cover art"""

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

for model in models_to_try:
    print(f"\n{'='*60}")
    print(f"Trying model: {model}")
    print(f"{'='*60}")
    
    url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{model}"
    
    # Different models have different input schemas
    # Let's try a minimal payload
    data = json.dumps({
        "prompt": prompt,
    }).encode('utf-8')
    
    req = urllib.request.Request(url, data=data, headers=headers, method='POST')
    
    try:
        with urllib.request.urlopen(req, timeout=180) as response:
            result = json.loads(response.read().decode('utf-8'))
            if result.get('success'):
                res_data = result.get('result', {})
                if res_data.get('image'):
                    img_b64 = res_data['image']
                    img_bytes = base64.b64decode(img_b64)
                    
                    output_dir = r"C:\Users\casey\sound-of-the-abyss\artwork"
                    os.makedirs(output_dir, exist_ok=True)
                    safe_model_name = model.replace('/', '-').replace('@', '')
                    output_path = os.path.join(output_dir, f"cover-{safe_model_name}.png")
                    
                    with open(output_path, 'wb') as f:
                        f.write(img_bytes)
                    
                    print(f"\n✅ SUCCESS with {model}!")
                    print(f"   Image saved to: {output_path}")
                    print(f"   Size: {len(img_bytes):,} bytes")
                    # Try all models to give options
                    continue
                else:
                    print(f"   No image. Result keys: {list(res_data.keys())}")
            else:
                print(f"   API Error: {result.get('errors', [])}")
    except urllib.error.HTTPError as e:
        error_body = e.read().decode('utf-8')
        print(f"   HTTP Error {e.code}: {error_body[:500]}")
    except Exception as e:
        print(f"   Error: {type(e).__name__}: {e}")

print(f"\n{'='*60}")
print("Done trying models")
print(f"{'='*60}")
