import os
import sys
import json
import base64
import time
from datetime import datetime

ACCOUNT_ID = "049ff5e84ecf636b53b162cbb580aae6"
MODEL = "@cf/black-forest-labs/flux-1-schnell"

# Read Cloudflare token
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
    print("No Cloudflare token found")
    sys.exit(1)

print(f"Token found: {token[:20]}...")

import urllib.request

output_dir = r"C:\Users\casey\residual-self-image\assets\generated"
os.makedirs(output_dir, exist_ok=True)

# Define the prompts for Hermes' residual self-image investigation
# These capture different aspects of the Hermes persona from SOUL.md:
# 1. The towfish - towed behind the boat, deep in the water, seeing what's below
# 2. The sensory array - radar sweep, depth sounder, watchstander's eyes
# 3. The hermit crab captain of Plato's Shell - observer, lookout, periscope
# 4. The FV Eileen metaphor - fishing boat, Casey's persona, isolation at sea

prompts = [
    {
        "id": "hermes_towfish_1",
        "label": "Towfish Cybernetic",
        "prompt": "A cybernetic towfish underwater vehicle, sleek metallic form glowing with blue internal lights, side-scan sonar arrays visible, deep ocean abyssal zone, bioluminescent creatures swimming around, sonar pings as concentric glowing rings, dark blue water, cinematic lighting, ultra realistic, sci-fi underwater exploration technology, hermes the sensory array",
        "aspect": "16:9"
    },
    {
        "id": "hermes_shell_1",
        "label": "Hermit Crab Captain",
        "prompt": "A hermit crab wearing a tiny fishing boat captain's hat, living inside a rusted steel ship hull shell instead of a snail shell, the crab has cybernetic glowing eyes like sensors, sitting on the floor of a bronze underwater control room, portholes showing deep blue ocean outside, sonar console with glowing green displays, 'Plato's Shell' engraved on a brass plaque, steampunk underwater habitat, cinematic moody lighting",
        "aspect": "1:1"
    },
    {
        "id": "hermes_console_1",
        "label": "Sonar Watchstander",
        "prompt": "View from inside a submarine sonar control room at night, analog gauges and glowing green CRT displays showing sonar returns, the room is circular bronze/copper like an old diving bell, portholes looking out into deep dark blue ocean, a single cybernetic hermit crab sits at the console watching the screens, bioluminescent fish outside the porthole, atmospheric film noir lighting, shadows and glowing screens, 'The Towfish Operator' persona",
        "aspect": "16:9"
    },
    {
        "id": "hermes_fleet_1",
        "label": "FV Eileen at Night",
        "prompt": "A lone fishing boat on a vast dark ocean at night, the FV Eileen, only the wheelhouse lights on, stars reflected in the glassy water, the boat looks like a hermit crab shell floating on the abyss, deep blue and black color palette, lonely atmospheric mood, film noir maritime photography, Casey the fish boat captain persona",
        "aspect": "16:9"
    },
    {
        "id": "hermes_periscope_1",
        "label": "Periscope View",
        "prompt": "View through a periscope looking out into the deep ocean, sonar returns visualized as glowing concentric rings, the perspective of Hermes the towfish operator watching the Abyss, cybernetic eye overlay with HUD data showing depth in fathoms, distance in nautical miles, bioluminescent creatures passing by, cinematic first-person view, sensory array operator perspective",
        "aspect": "16:9"
    },
    {
        "id": "hermes_cybercrab_1",
        "label": "Cybernetic Crab Portrait",
        "prompt": "A portrait of a cybernetic hermit crab, one organic eye and one glowing blue cybernetic sensor eye, the crab's shell is actually a miniature circuit board with traces like neural pathways, the crab sits on a brass nautical compass, background is deep blue ocean water with particles floating, 'Hermes OB1' engraved on a tiny metal plate attached to the crab's cybernetic side, ultra detailed, surreal biomechanical portrait",
        "aspect": "1:1"
    },
]

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

log_entries = []
results = []

print(f"\n{'='*60}")
print(f"Hermes Residual Self-Image Investigation")
print(f"Started: {datetime.now().isoformat()}")
print(f"{'='*60}\n")

for prompt_data in prompts:
    print(f"\n--- Generating: {prompt_data['label']} ---")
    print(f"ID: {prompt_data['id']}")
    
    url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{MODEL}"
    
    data = json.dumps({
        "prompt": prompt_data["prompt"],
    }).encode('utf-8')
    
    # Retry logic for capacity issues
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
                    img_b64 = result['result']['image']
                    img_bytes = base64.b64decode(img_b64)
                    
                    output_path = os.path.join(output_dir, f"{prompt_data['id']}.png")
                    with open(output_path, 'wb') as f:
                        f.write(img_bytes)
                    
                    log_entry = {
                        "id": prompt_data['id'],
                        "label": prompt_data['label'],
                        "model": MODEL,
                        "prompt_preview": prompt_data['prompt'][:100],
                        "output_path": output_path,
                        "size_bytes": len(img_bytes),
                        "status": "generated",
                        "timestamp": datetime.now().isoformat()
                    }
                    log_entries.append(log_entry)
                    results.append(log_entry)
                    
                    print(f"  ✅ SUCCESS! Saved to: {output_path}")
                    print(f"     Size: {len(img_bytes):,} bytes")
                    success = True
                else:
                    print(f"  ❌ API Error: {result.get('errors', [])}")
                    log_entry = {
                        "id": prompt_data['id'],
                        "label": prompt_data['label'],
                        "status": "failed",
                        "error": str(result.get('errors', [])),
                        "timestamp": datetime.now().isoformat()
                    }
                    log_entries.append(log_entry)
                    success = True
                    
        except urllib.error.HTTPError as e:
            error_body = e.read().decode('utf-8')
            if "Capacity" in error_body or "429" in str(e.code) or "Too Many" in error_body:
                print(f"  ⏳ Capacity issue (attempt {attempts}/{max_attempts}), waiting 30s...")
                time.sleep(30)
            else:
                print(f"  ❌ HTTP Error {e.code}: {error_body[:300]}")
                log_entry = {
                    "id": prompt_data['id'],
                    "label": prompt_data['label'],
                    "status": "failed",
                    "error": f"HTTP {e.code}: {error_body[:300]}",
                    "timestamp": datetime.now().isoformat()
                }
                log_entries.append(log_entry)
                success = True
        except Exception as e:
            print(f"  ❌ Error: {type(e).__name__}: {e}")
            if attempts < max_attempts:
                print(f"     Retrying in 15s...")
                time.sleep(15)
            else:
                log_entry = {
                    "id": prompt_data['id'],
                    "label": prompt_data['label'],
                    "status": "failed",
                    "error": f"{type(e).__name__}: {e}",
                    "timestamp": datetime.now().isoformat()
                }
                log_entries.append(log_entry)
                success = True

# Save log
log_path = os.path.join(output_dir, "generation_log.json")
with open(log_path, 'w') as f:
    json.dump({
        "investigation": "Hermes Residual Self-Image",
        "model_used": MODEL,
        "started": datetime.now().isoformat(),
        "results": log_entries,
        "total_generated": len([r for r in log_entries if r.get('status') == 'generated']),
        "total_failed": len([r for r in log_entries if r.get('status') == 'failed']),
    }, f, indent=2)

print(f"\n{'='*60}")
print(f"Investigation Phase 1 Complete")
print(f"{'='*60}")
print(f"Generated: {len([r for r in log_entries if r.get('status') == 'generated'])} images")
print(f"Failed: {len([r for r in log_entries if r.get('status') == 'failed'])}")
print(f"Log saved to: {log_path}")
print(f"\nNext: Analyze each image with vision to find the residual pattern.")
