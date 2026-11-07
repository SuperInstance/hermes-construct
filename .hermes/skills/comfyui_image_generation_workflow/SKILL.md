---
name: comfyui_image_generation_workflow
description: Class-level skill for starting the ComfyUI server, managing dependencies, launching image generation, and archiving outputs for the Hermes fleet.
level: class
tags:
  - comfyui
  - image-generation
  - workflow
  - hermes
---
## Overview
This skill governs the **end‑to‑end workflow** for getting ComfyUI up and running on a Windows workstation so that Hermes can generate images via the `gen_hermes_sub.py` pipeline and archive them to `~/ai-writings`. It covers:

1. **Environment preparation** – clean virtual‑env, required packages.
2. **Patch application** – force CPU fallback when CUDA isn’t available.
3. **Server launch** – correct command syntax and port handling.
4. **Verification** – checking that the UI is reachable (`http://127.0.0.1:8190`).
5. **Generation** – queuing prompts, locating output PNGs.
6. **Archiving** – moving images into `~/ai-writings` and delivering via Telegram.
7. **Troubleshooting** – quick commands for the most common failure modes.

## Detailed Technical Operations

For deep-dive technical procedures, advanced patching (e.g., `comfy-aimdo` venv shadowing), and low-level startup diagnostics, refer to the technical operations sub-section:
- [comfyui-operations](engineering/comfyui-operations)

The skill is **class‑level**; all agents that need to start/stop ComfyUI or run image generation should call this workflow rather than ad‑hoc commands.

---

## Prerequisites
| Requirement | Detail |
|-------------|--------|
| **OS** | Windows 10/11 (PowerShell / Bash via git‑bash) |
| **Python** | 3.14 (installed inside a dedicated venv) |
| **Disk** | ≥ 10 GB free space (models can be large) |
| **Port** | 8190 must be free (or specify another) |
| **Network** | Ability to inbound a TCP connection on the host IP (`127.0.0.1`) |
| **Git** | Cloned SuperInstance/ComfyUI repo at `C:\\Users\\casey\\Documents\\ComfyUI` |
| **CLI tools** | `python.exe` from the target venv must be reachable via absolute path |

---

## Step‑by‑Step Procedure

### 1️⃣ Create a clean virtual environment
```bash
# From the repo root
cd C:\\Users\\casey\\Documents\\ComfyUI
python -m venv venv2
# Verify
.\\venv2\\Scripts\\python.exe --version   # → 3.14.x
```

### 2️⃣ Install core dependencies
```bash
.\\venv2\\Scripts\\pip.exe install -r requirements.txt
# Important packages
# - sqlalchemy
# - filelock
# - torch (CPU‑only wheel if CUDA unavailable)
# - comfyui‑aimdo, comfyui‑frontends, … (as listed in requirements.txt)
```

### 3️⃣ Apply the CUDA‑fallback patch
Edit `comfy\\model_management.py` and replace the line:
```python
return torch.device(torch.cuda.current_device())\n```
with:
```python
return torch.device('cpu')\n```
*If using a text editor, keep the exact indentation; the change is safe to re‑apply.*

### 4️⃣ Launch the ComfyUI server
```cmd
start "" cmd /c "cd C:\\Users\\casey\\Documents\\ComfyUI && ^ \n    C:\\Users\\casey\\Documents\\ComfyUI\\venv2\\Scripts\\python.exe main.py ^\n    --listen 127.0.0.1 --port 8190 2>&1"\n```
- **Why this works:** Uses an **absolute path** to `python.exe` and wraps everything in `start` to avoid the `FileNotFoundError` that arises from path‑resolution issues.
- The server should respond on `http://127.0.0.1:8190`.

### 5️⃣ Verify the server is alive
- Open a browser to `http://127.0.0.1:8190`.
- If you get a JSON health check or the UI loads, the server is ready.
- Alternatively, use `curl`:
```bash
curl -I http://127.0.0.1:8190\n```
# Expect HTTP/1.1 200 OK

### 6️⃣ Queue a generation prompt
1. Upload a checkpoint from `models/checkpoints/`.
2. Build a node chain (e.g., `KSampler → Preview Image → Save Image`).
3. Enter a prompt such as:
   ```
   a fathom‑deep, sonar‑pinging towfish navigating icy waters of Alaska, cinematic lighting, ultra‑realistic
   ```
4. Click **Queue Prompt**.  
5. Wait for the job to complete; the PNG appears in `output/`.

### 7️⃣ Archive the result
```powershell
# Move the newest PNG to the archival folder
$latest = Get-ChildItem output\\*.png | Sort-Object LastWriteTime -Descending | Select-Object -First 1
Copy-Item $latest.FullName -Destination \"C:\\Users\\casey\\ai-writings\\generated\\\"\n```
# Or send via Telegram
send_message action=send message=Generated image: $latest.Name MEDIA:$latest.FullName

### 8️⃣ Stop the server (when done)
```cmd
taskkill /PID <PID_OF_COMPYUI> /F
```

---

## Common Troubleshooting Snippets

| Symptom | Fix |\n|---------|-----|
| `FileNotFoundError: [WinError 2] The system cannot find the file specified` | Use the **full qualified path** to `python.exe` as shown in step 4; avoid double‑escaped backslashes. |\n| `AssertionError: Torch not compiled with CUDA enabled` | Ensure the **CPU fallback patch** is in place (step 3). Re‑run the server. |\n| `Failed to initialize database. Could not acquire lock on database 'comfyui.db'` | Kill any stray Python processes: `taskkill /IM python.exe /F`, then delete `comfyui.db` if it remains locked. |\n| `Port 8190 is already in use` | Change `--port` to an unused port (e.g., `--port 8200`) and update any UI links. |\n| `RuntimeError: comfy-aimdo is not initialized` | Ensure the local `comfy_aimdo` mock (`control.py` and `model_mmap.py`) is **not shadowed** by an installed package. Run `.\\\\venv2\\\\Scripts\\\\pip.exe uninstall comfy-aimdo` if needed, then restart. |\\n| Checkpoint name not found validation error | Prompt fails with 'ckpt_name: ... not in []' and expects a checkpoint in `Models/checkpoints/`. Fix: download required checkpoint and place it there. |\\n| Missing checkpoint file error | No checkpoint was found in `Models/checkpoints/`. Add your `.safetensors` file and rename accordingly. |\\n| Database lock error | Stale `comfyui.db` remains locked after crash. Kill lingering Python processes and delete `user/comfyui.db` before restart. |\\n| Port conflict error | Server reports `Port X is already in use`. Change `--port` flag or stop the other process. |\\n---\\n\n\n---\n\n## References (session‑specific)\n* `references/comfyui_launch_troubleshooting.md` – detailed logs, error transcripts, and command‑by‑command reproductions of the failures encountered during the recent restart attempts.\n* `scripts/launch_comfyui.ps1` – a PowerShell wrapper that automates steps 4‑5 with built‑in error handling and logging.\n\n---\n\n## Scripts (executable helpers)\n* `scripts/launch_comfyui.ps1` – PowerShell script that runs the server start command, captures stdout/stderr, and writes a timestamped log to `logs/comfyui_start.log`.  \n* `scripts/generate_and_archive.py` – a tiny Python helper that calls `gen_hermes_sub.py`, waits for completion, and moves the resulting PNG to `~/ai-writings`.  \n\n---\n\n## Change Log (future‑proofing)\n- **v1.2 (2026‑09‑29)** – Added explicit port validation and PowerShell wrapper script.  \n- **v1.1 (2026‑09‑28)** – Introduced CPU‑fallback patch documentation and troubleshooting table.  \n- **v1.0 (2026‑09‑27)** – Initial release covering server start, verification, and basic generation.\n\n--- \n*This skill should be invoked whenever a new Hermes image generation run is planned, or when any component of the ComfyUI pipeline is modified. It guarantees reproducibility, proper error handling, and consistent archiving across fleet agents.* \n

---

## Common Troubleshooting Snippets

| Symptom | Fix |
|---------|-----|
| `FileNotFoundError: [WinError 2] The system cannot find the file specified` | Use the **full qualified path** to `python.exe` as shown in step 4; avoid double‑escaped backslashes. |
| `AssertionError: Torch not compiled with CUDA enabled` | Ensure the **CPU fallback patch** is in place (step 3). Re‑run the server. |
| `Failed to initialize database. Could not acquire lock on database 'comfyui.db'` | Kill any stray Python processes: `taskkill /IM python.exe /F`, then delete `comfyui.db` if it remains locked. |
| `Port 8190 is already in use` | Change `--port` to an unused port (e.g., `--port 8200`) and update any UI links. |
| `RuntimeError: comfy-aimdo is not initialized` | Ensure the local `comfy_aimdo` mock (`control.py` and `model_mmap.py`) is **not shadowed** by an installed package. Run `.\\venv2\\Scripts\\pip.exe uninstall comfy-aimdo` if needed, then restart. |\n| Checkpoint name not found validation error | Prompt fails with 'ckpt_name: ... not in []' and expects a checkpoint in `Models/checkpoints/`. Fix: download required checkpoint and place it there. |\n| Missing checkpoint file error | No checkpoint was found in `Models/checkpoints/`. Add your `.safetensors` file and rename accordingly. |\n| Database lock error | Stale `comfyui.db` remains locked after crash. Kill lingering Python processes and delete `user/comfyui.db` before restart. |\n| Port conflict error | Server reports `Port X is already in use`. Change `--port` flag or stop the other process. |\n---\n

---

## References (session‑specific)
* `references/comfyui_launch_troubleshooting.md` – detailed logs, error transcripts, and command‑by‑command reproductions of the failures encountered during the recent restart attempts.
* `scripts/launch_comfyui.ps1` – a PowerShell wrapper that automates steps 4‑5 with built‑in error handling and logging.

---

## Scripts (executable helpers)
* `scripts/launch_comfyui.ps1` – PowerShell script that runs the server start command, captures stdout/stderr, and writes a timestamped log to `logs/comfyui_start.log`.  
* `scripts/generate_and_archive.py` – a tiny Python helper that calls `gen_hermes_sub.py`, waits for completion, and moves the resulting PNG to `~/ai-writings`.  

---

## Change Log (future‑proofing)
- **v1.2 (2026‑09‑29)** – Added explicit port validation and PowerShell wrapper script.  
- **v1.1 (2026‑09‑28)** – Introduced CPU‑fallback patch documentation and troubleshooting table.  
- **v1.0 (2026‑09‑27)** – Initial release covering server start, verification, and basic generation.

--- 
*This skill should be invoked whenever a new Hermes image generation run is planned, or when any component of the ComfyUI pipeline is modified. It guarantees reproducibility, proper error handling, and consistent archiving across fleet agents.* 