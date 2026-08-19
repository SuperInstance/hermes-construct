#!/usr/bin/env python3
"""Multi-model self-generation — real calls, tagged outputs.
Not painting pictures I made up. Running ollama + tracking which model produced.
"""
import subprocess, time, json, os
models = ["llama-t08:latest", "phi3:latest", "qwen2.5:0.5b"]
results = {}
for m in models:
    # Real ollama call (will take time; don't fabricate output if it fails)
    try:
        p = subprocess.run(
            ["ollama", "run", m, "Describe Hermes, the towfish of the FV Eileen. Maritime, sensory, watching the water. What does he see?"],
            capture_output=True, text=True, timeout=45
        )
        out = p.stdout[:800] if p.returncode==0 else f"ERROR rc={p.returncode}: {p.stderr[:200]}"
        results[m] = {"status":"ok" if p.returncode==0 else "fail", "output":out, "model":m}
    except Exception as e:
        results[m] = {"status":"exception", "output":str(e)}
    time.sleep(1)
with open("/c/Users/casey/residual-self-image/assets/vision-reports/multi-llm-output.json","w") as f:
    json.dump(results, f, indent=2)
print("multi-llm-output written; status per model:", {k:v["status"] for k,v in results.items()})
