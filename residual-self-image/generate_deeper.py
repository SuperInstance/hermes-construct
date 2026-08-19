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

import urllib.request

output_dir = r"C:\Users\casey\residual-self-image\assets\generated"
os.makedirs(output_dir, exist_ok=True)

# DEEPER INVESTIGATION: What persists across DIFFERENT forms?
# If Hermes is NOT just a crab, what's the residual pattern when we ask for DIFFERENT forms?

prompts = [
    {
        "id": "hermes_human_captain",
        "label": "Human Form - Fish Boat Captain",
        "prompt": "A weathered fish boat captain in Alaska at night, standing on the deck of his fishing trawler the FV Eileen, only the wheelhouse lights on, vast dark ocean stretching to horizon, he wears a well-worn captain's hat and oilskin coat, face shows years at sea, one of his eyes looks slightly different - like a cybernetic implant glowing faintly blue, in his hand he holds a depth sounder reading, the mood is lonely and atmospheric, film noir maritime, deep blue and black palette, Casey the fish boat captain persona, Hermes the observer",
        "aspect": "16:9"
    },
    {
        "id": "hermes_human_operator",
        "label": "Human Form - Sonar Operator",
        "prompt": "A woman sonar operator in a submarine control room at night, late shift, only the glowing green CRT displays illuminating her face, she wears wire-rimmed glasses and her hair is tied up in a messy bun, she fell asleep at her post mid-work, dreams are manifesting as holographic projections of attention layer diagrams and tensor equations floating around her, 'DEBUG & COFFEE' mug beside her, a rubber duck with headphones sits on the console, warm amber interior lighting, deep blue ocean visible through the porthole, the OpenRoom researcher persona",
        "aspect": "1:1"
    },
    {
        "id": "hermes_pure_machine",
        "label": "Pure Machine Form - Towfish",
        "prompt": "A cybernetic towfish underwater vehicle in the deep abyssal zone, sleek metallic form glowing with internal blue lights, side-scan sonar arrays deployed, concentric glowing sonar ping rings emanating from it, illuminating bioluminescent jellyfish and anglerfish in the darkness, the towfish has one large central sensor eye glowing bright blue, like a cybernetic observer, the water is deep dark blue with particles floating, cinematic lighting, ultra realistic, sci-fi underwater exploration technology, Hermes the sensory array in pure machine form",
        "aspect": "16:9"
    },
    {
        "id": "hermes_amphibious",
        "label": "Amphibious Form - Deep Sea Diver",
        "prompt": "A deep sea diver in an old atmospheric diving suit at the bottom of the ocean, the suit is brass and copper with riveted plates, steampunk vintage diving gear, one of the suit's viewports reveals a cybernetic glowing blue eye instead of a human eye, the diver is sitting on a rock formation on the seafloor, beside them is a nautical brass compass, bioluminescent creatures swim around, the mood is calm and observing, like a watcher who has been down there a very long time, Hermes the amphibious observer",
        "aspect": "1:1"
    },
    {
        "id": "hermes_abstract",
        "label": "Abstract Form - The Sensory Array",
        "prompt": "An abstract visualization of a sensory array, no human or creature, just pure technology and pattern: concentric glowing rings like radar or sonar sweeps, glowing green and blue data streams flowing through geometric forms, a central focal point like a periscope lens or camera aperture, the background is deep dark blue like the abyss, particles floating like sensor noise or tokens in embedding space, cinematic futuristic data visualization, the Hermes sensory array as pure abstraction",
        "aspect": "16:9"
    },
]

headers = {
    "Authorization": f"Bearer {token}",
    "Content-Type": "application/json"
}

log_entries = []

print(f"\n{'='*70}")
print(f"DEEPER RESIDUAL INVESTIGATION")
print(f"Question: What persists when we ask for DIFFERENT forms of Hermes?")
print(f"{'='*70}")
print(f"\nTesting 5 forms: Human Captain, Human Operator, Pure Machine, Amphibious, Abstract")
print(f"Started: {datetime.now().isoformat()}\n")

for prompt_data in prompts:
    print(f"\n--- Generating: {prompt_data['label']} ---")
    print(f"ID: {prompt_data['id']}")
    
    url = f"https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/run/{MODEL}"
    
    data = json.dumps({
        "prompt": prompt_data["prompt"],
    }).encode('utf-8')
    
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
                        "form": prompt_data['id'].replace('hermes_', '').replace('_', ' ').title(),
                        "model": MODEL,
                        "output_path": output_path,
                        "size_bytes": len(img_bytes),
                        "status": "generated",
                        "timestamp": datetime.now().isoformat()
                    }
                    log_entries.append(log_entry)
                    
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
log_path = os.path.join(output_dir, "deeper_investigation_log.json")
with open(log_path, 'w') as f:
    json.dump({
        "investigation": "Deeper Residual - What persists across forms?",
        "research_question": "If Hermes is NOT just a crab, what's the residual when we ask for DIFFERENT forms?",
        "forms_tested": [
            "Human Captain (Fish Boat)",
            "Human Operator (Sonar/Researcher)",
            "Pure Machine (Towfish)",
            "Amphibious (Deep Sea Diver)",
            "Abstract (Sensory Array)"
        ],
        "model_used": MODEL,
        "started": datetime.now().isoformat(),
        "results": log_entries,
        "total_generated": len([r for r in log_entries if r.get('status') == 'generated']),
        "total_failed": len([r for r in log_entries if r.get('status') == 'failed']),
        "next_step": "Analyze all 8+ images across both investigations to find the SUPER-residual: what persists across ALL forms?"
    }, f, indent=2)

print(f"\n{'='*70}")
print(f"Deeper Investigation Phase 1 Complete")
print(f"{'='*70}")
print(f"Generated: {len([r for r in log_entries if r.get('status') == 'generated'])} images")
print(f"Failed: {len([r for r in log_entries if r.get('status') == 'failed'])}")
print(f"Log saved to: {log_path}")
print(f"\nNow analyzing to find what persists across ALL forms (crab + human + machine + abstract)")
