# ComfyUI Troubleshooting Reference (Hermes Fleet)

## 1. Database lock error
- **Symptom**: `Failed to initialize database. Could not acquire lock on database 'comfyui.db'`.
- **Cause**: A previous ComfyUI process left `comfyui.db` locked.
- **Fix**: Kill all Python processes and delete the lock file.
  ```bash
  taskkill /IM python.exe /F
  del C:\Users\casey\Documents\ComfyUI\comfyui.db
  ```

## 2. Port conflict
- **Symptom**: `Port 8190 is already in use`.
- **Cause**: Another application bound to the same port.
- **Fix**: Either stop the other process or change the port:
  ```cmd
  --port 8200   # pick any free port > 1024
  ```

## 3. CUDA / Torch assertion error
- **Symptom**: `AssertionError: Torch not compiled with CUDA enabled`.
- **Cause**: CUDA is unavailable but the code attempts to use it.
- **Patch**: Ensure `comfy\model_management.py` contains:
  ```python
  return torch.device('cpu')
  ```
- After patch, restart the server.

## 4. comfy-aimdo initialization failure
- **Symptom**: `RuntimeError: comfy-aimdo is not initialized`.
- **Cause**: The local `comfy_aimdo` mock directory is being shadowed by an installed package.
- **Fix**:
  1. Remove the installed package (if present):
     ```bash
     .\venv2\Scripts\pip.exe uninstall comfy-aimdo
     ```
  2. Verify that `control.py` contains `lib = None` and `model_mmap.py` does not raise on import.
  3. Restart ComfyUI.

## 5. FileNotFoundError for python.exe
- **Symptom**: `FileNotFoundError: [WinError 2] The system cannot find the file specified`.
- **Cause**: Relative path or double‑escaped backslashes cause path resolution to fail.
- **Fix**: Use the **absolute** path to the interpreter:
  ```cmd
  start "" cmd /c "cd C:\Users\casey\Documents\ComfyUI && C:\Users\casey\Documents\ComfyUI\venv2\Scripts\python.exe main.py --listen 127.0.0.1 --port 8190"
  ```

## 6. General verification checklist
1. `.\venv2\Scripts\python.exe --version` → should show `3.14.x`.
2. `.\venv2\Scripts\pip.exe list` → should include `torch`, `sqlalchemy`, `filelock`.
3. Open `http://127.0.0.1:8190` → UI loads.
4. Upload a checkpoint, run a simple node chain, verify PNG appears in `output/`.

### Quick copy‑paste launch command (PowerShell)
```powershell
Start-Process -FilePath "powershell.exe" -ArgumentList "-Command", `
"cd C:\Users\casey\Documents\ComfyUI; Start-Process -FilePath \"C:\Users\casey\Documents\ComfyUI\venv2\Scripts\python.exe\" -ArgumentList \"main.py --listen 127.0.0.1 --port 8190\" -NoNewWindow"
```

---  
*Keep this file updated whenever a new failure mode is discovered. It lives in `references/` of the `comfyui_image_generation_workflow` skill, so any agent that loads the skill can instantly retrieve the latest fixes.* 