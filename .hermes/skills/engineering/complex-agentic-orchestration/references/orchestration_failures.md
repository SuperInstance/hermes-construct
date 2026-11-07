# Orchestration Failures & Remediation

This reference archive documents observed failures in subagent orchestration and the specific technical patterns used to resolve them.

## 1. The "Blind Subagent" Problem (ModuleNotFoundError)
**Observation:**
Subagents dispatched via `delegate_task` frequently fail when trying to `import` local project modules (e.g., `agent`, `claw`, `superinstance`), despite the modules being present in the host's `venv`.

**Root Cause:**
The subagent's isolated shell environment does not inherit the specific `site-packages` path of the parent worker's virtual environment. It defaults to the system Python path.

**Remediation Pattern (The "Sight" Injection):**
Instead of relying on implicit environment inheritance, always explicitly define the `PYTHONPATH` within the subagent's command string.

*   **❌ Failed Approach:** 
    `python -c "import agent"`
*   **❌ Brittle Approach:** 
    `export PYTHONPATH=$PYTHONPATH:/c/path/to/venv/Lib/site-packages && python -c "import agent"` (Fails if `$PYTHONPATH` is initially empty or malformed).
*   **✅ Robust Approach (Direct Assignment):**
    `export PYTHONPATH=/c/path/to/venv/Lib/site-packages && python -c "import agent"`

## 2. The "Silent Handover" Problem (Filesystem Desync)
**Observation:**
An agent creates a file (e.g., `manifest.md`) and reports success. A subsequent agent (the "Architect") is tasked with reading that file but returns `File Not Found`.

**Root Cause:**
In many multi-agent orchestration layers, the file written by a subagent is stored in a temporary or context-specific directory that is not automatically synchronized with the parent agent's primary workspace or the global filesystem.

**Remediation Pattern:**
1.  **Explicit Pathing:** Always use absolute paths for critical artifacts.
2.  **Manifest Confirmation:** For high-stakes handovers, the producing agent should verify the file exists using a `ls` or `stat` command before reporting success.
3.  **Manual Injection (Hermes Override):** If a handover fails, the parent agent should intercept the file content using `read_file` and inject it directly into the next agent's `context` prompt.
