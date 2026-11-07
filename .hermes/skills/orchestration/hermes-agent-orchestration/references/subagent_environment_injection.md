# 🛠️ Subagent Environment Injection & Provisioning (REFS)

This file contains the proven pattern for resolving `ModuleNotFoundError` when delegating tasks to subagents within the SuperInstance/FV-Eileen ecosystem.

## 🔍 The Problem: "The Blind Subagent"
When a subagent is dispatched via `delegate_task`, it often spawns in an isolated shell that lacks the `PYTHONPATH` context of the primary agent's `venv`. This leads to `ModuleNotFoundError` even when the library is present on disk.

## ✅ The Fix: "Vision Injection" Pattern
To restore sight to a subagent, the `PYTHONPATH` must be explicitly declared to include the project's virtual environment `site-packages` directory.

### 🧪 Reproduction Recipe
**Target Path:** `/c/Users/casey/hermes-construct/hermes-construct/venv/Lib/site-packages`

**The Failed Pattern (Ineffective):**
```bash
# Appending to an potentially empty or mismatched PYTHONPATH often fails in subagent shells
export PYTHONPATH=$PYTHONPATH:/c/Users/casey/hermes-construct/hermes-construct/venv/Lib/site-packages && python -c "import agent"
```

**The Working Pattern (The "Vision Injection"):**
```bash
# Direct assignment ensures the module is explicitly reachable
export PYTHONPATH=/c/Users/casey/hermes-construct/hermes-construct/venv/Lib/site-packages && python -c "import agent; print('SUCCESS')"
```

## 📊 Diagnostic Workflow
If a subagent reports a module error, follow this sequence:
1. **Locate:** `find [VENV_PATH] -name "[module_name]"`
2. **Inspect:** `python -c "import sys; print('\n'.join(sys.path))"`
3. **Inject & Verify:** Run the "Working Pattern" above with the discovered path.
