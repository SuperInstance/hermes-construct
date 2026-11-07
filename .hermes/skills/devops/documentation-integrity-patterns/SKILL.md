---
name: documentation-integrity-patterns
description: Governs the high-fidelity preservation, verification, and synchronized deployment of master documentation and architectural schemas in distributed environments.
---

# Documentation Integrity & Synchronization Patterns

This skill governs the "Last Mile" of documentation management: ensuring that what is written in an agent's context is physically what exists on the disk, and that the formal version history (Git) reflects a verified state.

## Overview

In the SuperInstance ecosystem, documentation is not just text; it is the **Cognitive Substrate**. If the documentation is corrupted, the "Synaptic Trace" is broken, making future agent coordination impossible.

## The "Nuclear Sync" Workflow

When documentation has become desynchronized, corrupted, or consists of "ghost files" (0-byte files), follow this protocol:

### 1. Detection (The "Byte-Check" Phase)
Before assuming a write worked, verify the physical presence and density of the data.
- **Command**: `cat <path>` or `wc -c <path>`
- **Red Flag**: A file that exists but has `0` bytes or contains only whitespace/empty output is a "Ghost File."

### 2. Injection (The "Shell-First" Phase)
To prevent proxy-level redirection or tool-level corruption, bypass standard `write_file` abstraction for high-density or critical content.
- **Pattern**: Use the `terminal` tool with absolute paths and shell-native redirection.
- **Preferred Method**:
  ```bash
  cat << 'EOF' > /absolute/path/to/file.md
  # Content
  EOF
  ```
- **Alternative**: Use `printf` for single-line or templated injections to avoid shell expansion issues.

### 3. Verification (The "Double-Blind" Phase)
Never proceed to a `git commit` until you have manually inspected the output of the write operation.
- **Step A**: `cat <path>` (Observe content in the terminal output).
- **Step B**: `wc -c <path>` (Confirm byte count matches expected density).

### 4. Mobilization (The "Force-Sync" Phase)
Once verified, use aggressive Git staging to ensure the index is correctly updated.
- **Command**: `git add -A` followed by `git commit -m "..."` and `git push`.
- **Note**: If `git status` reports a clean tree but `cat` shows a ghost file, the index is desynchronized. Use the `Nuclear Sync` pattern (Step 2) to overwrite the file before attempting to commit again.

## Pitfalls to Avoid

- **The "Trust the Tool" Fallacy**: Assuming that a `write_file` success return means the bits are on the platter. In complex environments (Windows/WSL/Remote/Proxy), tool return values can be deceptive. **Always verify the byte count.**
- **The "Incremental Fix" Trap**: Trying to patch a corrupted file with `patch` or `replace`. If a file is in a corrupted or 0-byte state, `patch` will fail or produce nonsense. **Always perform a full rewrite (The "Nuclear Rewrite") for corrupted files.**
- **The "Relative Path" Error**: Using relative paths in automation scripts within a multi-repo environment. **Always use absolute paths for all filesystem operations to ensure the "Synaptic Trace" is unambiguous.**

## Reference: Verified Patterns
*See `references/high_fidelity_write_examples.md` for known-good shell injection templates.*
