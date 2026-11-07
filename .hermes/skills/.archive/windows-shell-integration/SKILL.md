---
name: windows-shell-integration
description: Workarounds for executing commands on a Windows host where the terminal shell bridge is misconfigured or failing to spawn.
---

# SKILL: windows-shell-integration

This skill provides patterns for working with an agent deployed on a Windows host where the `terminal` tool is configured to use a CMD/Bash bridge (MSYS/WSL) which may encounter `execvpe` or path-resolution errors.

## 🎯 Purpose
To provide reliable execution patterns for commands and scripts in a Windows environment where the primary shell bridge might be unstable or misconfigured.

## 🛠️ Patterns & Workarounds

### 1. The "Python Bypass" Pattern
When the `terminal` tool returns `execvpe(/bin/bash) failed` or similar shell-spawning errors, do not attempt to retry the terminal command. Instead, use `execute_code` to run the command via a Python `subprocess` call.

**Why this works:** `execute_code` runs in a native Python environment that can call `cmd.exe` directly without needing to spawn a `/bin/bash` shell.

**Implementation Template:**
```python
import subprocess

def run_windows_cmd(args_list):
    # Always prefix with 'cmd.exe /c' to ensure direct Windows execution
    try:
        result = subprocess.run(["cmd.exe", "/c"] + args_list, capture_output=True, text=True, timeout=30)
        return result.stdout.strip()
    except Exception as e:
        return str(e)

# Example: gh auth status
print(run_windows_cmd(["gh", "auth", "status"]))
```

### 2. Path Handling & Environment Fixes
Always use absolute paths for file operations to avoid ambiguity between the shell's CWD and the Python environment's CWD. Use `os.path.join` or raw strings (`r"C:\\..."`) to avoid backslash escaping issues.

**The "Shell Re-injection" Fix:**
When encountering systematic `execvpe(/bin/bash) failed` errors, the root cause is often the absence of Git/MSYS `bin` or `usr/bin` folders in the System PATH. 

**Fix Pattern (PowerShell):**
If heavy use of the `terminal` tool is required, use a script to permanently inject Git's binary paths into the System Environment Variables:
```powershell
# Locate Git and inject bin/usr/bin to System Path
$gitPath = (Get-Command git.exe -ErrorAction SilentlyContinue).Source
$gitDir = Split-Path (Split-Path $gitPath -Parent) -Parent
$binPaths = @("$gitDir\bin", "$gitDir\usr\bin")

foreach ($path in $binPaths) {
    $oldPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($oldPath -notlike "*$path*") {
        [Environment]::SetEnvironmentVariable("Path", "$path;$oldPath", "Machine")
    }
}
```
*Note: This requires a session restart to take effect.*

### 3. Avoiding Shell-Specific Built-ins
Avoid using POSIX-style tools (`ls`, `grep`, `cat`, `rm`) in the `terminal` tool if the environment error indicates a missing bash. Instead:
- Use `os.listdir` or `dir` (via `cmd.exe`)
- Use `findstr` instead of `grep`
- Use `type` or `read_file` instead of `cat`

## ⚠️ Pitfalls
- **Do NOT** continue retrying the `terminal` tool if it fails with a shell-spawning error. This is a systemic environment issue, not a transient tool failure.
- **Do NOT** assume `/bin/bash` or `/bin/sh` is available.
- **DO** prefer `execute_code` as the primary driver for all command-line operations in this environment.
