---
name: windows-native-command-execution
description: Robust execution of CLI tools via Python subprocesses when the terminal tool's bash bridge fails.
---

# Windows-Native Command Execution

This skill covers the robust execution of CLI-based tools and system commands within a Windows environment when the high-level `terminal` tool (which relies on WSL/bash) is unavailable or misconfigured.

## Overview
In certain Windows deployments, the agent's `terminal` tool may attempt to invoke a POSIX-compliant shell (`/bin/bash`) that does not exist or is improperly bridged. To maintain operational capability, all shell-based tasks should be routed through the Python `execute_code` tool using the Windows Command Processor (`cmd.exe`).

## Execution Pattern: The Python-Subprocess Bridge

When the `terminal` tool fails with `execvpe(/bin/bash) failed`, use the following pattern to interact with the system.

### Core Workflow
1. **Avoid `terminal`**: Do not attempt to use the `terminal` tool for any command requiring a shell.
2. **Use `execute_code`**: Write a Python script to act as the orchestrator.
3. **Invoke `cmd.exe`**: Use `subprocess.run` with `shell=False` (to avoid internal bash calls) and prepending the command with `cmd.exe /c`.

### Implementation Template

```python
import subprocess
import json

def run_windows_cmd(args):
    """
    Executes a command via the Windows Command Processor.
    :param args: List of command arguments (e.g., ["gh", "auth", "status"])
    :return: Dict containing stdout, stderr, and exit_code
    """
    try:
        # Use cmd.exe /c to ensure compatibility with Windows CLI tools
        result = subprocess.run(
            ["cmd.exe", "/c"] + args,
            capture_output=True,
            text=True,
            timeout=15
        )
        return {
            "stdout": result.stdout.strip(),
            "stderr": result.stderr.strip(),
            "exit_code": result.returncode
        }
    except Exception as e:
        return {"error": str(e)}

# Example Usage for GitHub CLI
gh_status = run_windows_cmd(["gh", "auth", "status"])
print(json.dumps(gh_status, indent=2))
```

## Pitfalls & Constraints
* **Bash Dependency**: The `terminal` tool is likely hardcoded to use `bash`. Even if `git-bash` is in the PATH, the tool's internal orchestration may fail if it expects a specific WSL/MSYS environment.
* **Environment Variables**: When running via `subprocess.run`, the environment is inherited from the Python process. Ensure the `PATH` includes the necessary binaries (e.g., `gh`, `wrangler`).
* **No PTY/Interactive Mode**: The `execute_code` method is non-interactive. Do not use it for commands that require user input or a pseudo-terminal (PTY) unless you provide the input via `subprocess.communicate()`.

## Verification
To verify if this mode is required, attempt a simple `whoami` via `terminal`. If it returns `execvpe(/bin/bash) failed`, switch to the Python-Subprocess Bridge immediately.
