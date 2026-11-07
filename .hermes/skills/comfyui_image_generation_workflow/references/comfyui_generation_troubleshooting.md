# Hermes Fleet Image Generation – Recent Trouble‑Shooting Log (2026‑09‑29)

## 1️⃣ Checkpoint‑Related Errors
**Symptom**: `RuntimeError: Runtime validation failed: ckpt_name not in []`  
**Root Cause**: The workflow tried to load `chilloutmix_NiPrunedFp32Fix.safetensors` but no checkpoint existed in `Models/checkpoints/`.  
**Fix**:
- Download the checkpoint from the public link:  
  ```bash
  curl -L -o "Models/checkpoints/chilloutmix_NiPrunedFp32Fix.safetensors" "https://huggingface.co/latent-consistency/chilloutmix/resolve/main/chilloutmix_NiPrunedFp32Fix.safetensors"
  ```
- Verify the file exists and the name matches exactly (including case).

## 2️⃣ Database Lock
**Symptom**: `Failed to initialize database. Could not acquire lock on database 'comfyui.db'`.  
**Root Cause**: A previous Python/ComfyUI process crashed and left the SQLite lock file behind.  
**Fix**:
```bash
# Kill any lingering python processes
taskkill /IM python.exe /F

# Delete the stale lock file
rm -f user/comfyui.db

# Restart ComfyUI (see workflow steps)
```

## 3️⃣ Port Conflict
**Symptom**: `Port 8190 is already in use`.  
**Root Cause**: Another instance of ComfyUI or a reverse‑proxy is bound to the same port.  
**Fix**:
- Choose a free port (e.g., `--port 8200`) and update `SERVER` in `gen_hermes_sub.py` accordingly, **or**
- Stop the competing process (`taskkill /IM <process_name>.exe /F`).

## 4️⃣ CUDA‑Fallback Mishap
**Symptom**: `AssertionError: Torch not compiled with CUDA enabled`.  
**Root Cause**: The fallback patch in `comfy/model_management.py` was missing or reverted.  
**Fix**: Edit `comfy/model_management.py` and replace:
```python
return torch.device(torch.cuda.current_device())
```
with:
```python
return torch.device('cpu')
```
Then restart the server.

## 5️⃣ COMFY‑AIMDO Initialization Failure
**Symptom**: `RuntimeError: comfy-aimdo is not initialized`.  
**Root Cause**: An installed `comfy-aimdo` wheel shadowed the local mock module.  
**Fix**:
```bash
venv2\Scripts\pip.exe uninstall comfy-aimdo
# Ensure the local `comfy_aimdo` folder is used (not the pip package)
```
Then restart the server.

## 6️⃣ Successful Generation Flow (Post‑Fix)
After applying the above fixes, re‑run the generation script:

```bash
python /c/Users/casey/Documents/ComfyUI/gen_hermes_sub.py
```

The script will:
1. POST a pre‑baked workflow to `http://127.0.0.1:8190/prompt`.  
2. Receive a JSON response containing a base‑64 PNG.  
3. Save the PNG to `output/`.  
4. Archive the file to `~/ai-writings` (or send via `MEDIA:` to Telegram).

**Log Example** (saved to `output/Hermes_Sub-001.png`):
```
[INFO] Generation completed – image stored at output/Hermes_Sub-001.png
```

---

**Tip:** Keep this markdown file version‑controlled (e.g., commit to the fleet repo) so future agents can diff against it when debugging similar issues.