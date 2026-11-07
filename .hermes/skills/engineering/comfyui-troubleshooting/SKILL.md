---
name: comfyui-troubleshooting
title: ComfyUI Advanced Troubleshooting
description: Additional pitfalls discovered during recent image generation runs (2026-08-19).
---
## Overview
This skill captures specific pitfalls and fixes discovered during recent ComfyUI image generation runs. It supplements the existing `comfyui-operations` skill.

## New Pitfalls & Fixes

### Shadowed `comfy_aimdo` package
The venv still loads the installed `comfy-aimdo` from `.venv\\Lib\\site-packages\\` even after local patches. Run:

```bash
pip uninstall comfy-aimdo -y
```

to force import of the local folder. After uninstall, flush `__pycache__` and restart. Verify import path points to `...\\ComfyUI\\comfy_aimdo\\__init__.py`.

### Stale `__pycache__`
Re‑introduces the TypeError. Delete all `__pycache__` and `*.pyc` before each restart.

### TIME_WAIT after abort
Wait ~60 s; TIME_WAIT clears any existing WAIT entries automatically. Re‑poll with `netstat -ano | grep 8188`. Still no `LISTENING`? Re‑run foreground with log redirect to capture errors.

### dtype mismatch (`Half` vs `Float`)
Still requires `--force-fp32` flag on server launch; it is the only reliable fix for this checkpoint.

---  
These additions are intended to be merged into the main `comfyui-operations` skill in a future edit pass, but the standalone skill provides a quick reference for future runs.