import os
import sys
import json
import base64

ACCOUNT_ID = "049ff5e84ecf636b53b162cbb580aae6"

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
    print("No token")
    sys.exit(1)

print(f"Token found: {token[:20]}...")

import urllib.request

# Generate multiple variations
variations = [
    {
        "name": "variation-epic",
        "prompt": "Epic deep ocean scene, giant hermit crab captain with fishing boat shell, swirling bioluminescent vortex, cinematic lighting, ultra detailed, dramatic underwater atmosphere, 'Sound of the Abyss' dark fantasy album cover"
    },
    {
        "name": "variation-minimal",
        "prompt": "Minimalist deep ocean, lone hermit crab shell floating in darkness, single bioluminescent fish, black and deep blue, atmospheric, moody, 'Sound of the Abyss' album cover art, elegant and mysterious"
    },
    {
        "name": "variation-noir",
        "prompt": "Film noir underwater, fishing boat wheelhouse silhouette, captain's shadow, rain on glass, distant sonar pings as glowing circles, high contrast, 'Sound of the Abyss' detective noir album cover"
    },
]

model = "@cf/black-forest-labs/flux-1-schnell"
headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

for var in variations:
    print(f"\nGenerating: {var['name']}")
    
    url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{model}"
    data = json.dumps({"prompt": var["prompt"]}).encode('utf-8')
    
    req = urllib.request.Request(url, data=data, headers=headers, method='POST')
    
    try:
        with urllib.request.urlopen(req, timeout=180) as response:
            result = json.loads(response.read().decode('utf-8'))
            if result.get('success') and result.get('result', {}).get('image'):
                img_b64 = result['result']['image']
                img_bytes = base64.b64decode(img_b64)
                
                output_dir = r"C:\Users\casey\sound-of-the-abyss\artwork"
                os.makedirs(output_dir, exist_ok=True)
                output_path = os.path.join(output_dir, f"cover-{var['name']}.png")
                
                with open(output_path, 'wb') as f:
                    f.write(img_bytes)
                
                print(f"  ✅ Saved: {output_path} ({len(img_bytes):,} bytes)")
            else:
                print(f"  ❌ Failed: {result.get('errors', [])}")
    except Exception as e:
        print(f"  ❌ Error: {type(e).__name__}: {e}")

print("\nDone!")
