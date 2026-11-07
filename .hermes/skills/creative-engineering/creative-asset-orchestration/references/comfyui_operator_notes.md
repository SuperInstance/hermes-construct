# ComfyUI Operator Reference — Headless API & Local Checkpoint Surgery

## Setup (once)
```ps
cd C:\Users\casey\Documents\ComfyUI
.venv\Scripts\python.exe -m pip install -r requirements.txt
```

## The `comfy_aimdo` mock (MUST fix before first run)
The repo expects three local files next to `main.py`. All three must exist or the server crashes or checkpoint loads fail.

- `comfy_aimdo/__init__.py` — mock classes: `HostBuffer`, `VRAMBuffer`, `ModelVBAR`, `ModelMMAP`, etc.
- `comfy_aimdo/control.py` — add **`lib = None`** at top level; without this, `model_mmap.py` raises `AttributeError: module 'comfy_aimdo.control' has no attribute 'lib'`
- `comfy_aimdo/model_mmap.py` — no-op `ModelMMAP` with truthy `self.state`; avoids `comfy-aimdo is not initialized` crash

After patching, clear `__pycache__` and restart. Do NOT patch `comfy/utils.py`.

## Port / launch
Default GUI log shows startup on 8000. API responds on the same port. If 8000 is taken, 8188 is the fallback.
```bash
cd C:\Users\casey\Documents\ComfyUI
.venv\Scripts\python.exe main.py --listen 127.0.0.1 --port 8188
```
Health check:
```bash
curl -s http://127.0.0.1:8188/system_stats
```

## Checkpoint placement
`mklink` is not available in git-bash. Use:
```bash
cd C:/Users/casey/Documents/ComfyUI/models/checkpoints
ln -s "C:/Users/casey/Downloads/chilloutmix_NiPrunedFp32Fix.safetensors" chilloutmix_NiPrunedFp32Fix.safetensors
```

## Headless generation script
Save as `gen_<name>.py`. Submit with `ckpt_name` only (no full path).
Typical node IDs: CheckpointLoaderSimple(4) → EmptyLatentImage(5) → CLIPTextEncode pos(6) → CLIPTextEncode neg(7) → KSampler(3) → VAEDecode(8) → SaveImage(9). Seed is at node 3.

Poll for completion:
```python
import time, urllib.request, json
pid = ...
for i in range(N):
    time.sleep(15)
    with urllib.request.urlopen(f"http://127.0.0.1:8188/history/{pid}") as r:
        h = json.load(r)[pid]
    st = h.get("status",{}).get("status_str","?")
    if st in ("success","error"): break
```
Always verify `"status_str":"success"` and extract `outputs[i]["images"][0]["filename"]` before claiming completion.

## VRAM notes (RTX 4050 Laptop ~6.4 GB)
Use `--cpu-vae` + `--force-fp32` to keep VAE off GPU and avoid half-precision segfaults. UNet runs on CUDA; first run per model is slow (model off disk).

## Gotchas
- `comfy_aimdo` is a local source overlay, NOT a pip package. Do not pip-install it.
- `pip install -r requirements.txt` upgrades `comfy_aimdo` wheel into `.venv`, but `main.py` prefers the local `comfy_aimdo/` package next to itself. The three-file mock must be in the repo tree, not only in `.venv`.
- `comfyui-frontend-package` not installed causes hard exit; reinstall requirements.txt.
- Orphan `python.exe` / `pythonw.exe` instances (seen in Task Manager with high Virtual Memory Size) will hold port 8188; kill them before relaunch.
