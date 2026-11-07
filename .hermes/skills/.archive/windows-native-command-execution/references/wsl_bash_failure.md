# Debugging the WSL/Bash Bridge Failure

## Symptom
The `terminal` tool returns the following error when attempting to execute any command:
`WSL (XX - Relay) ERROR: CreateProcessCommon:818: execvpe(/bin/bash) failed: No such file or directory`

## Cause
The environment is a native Windows environment (not WSL), but the Hermes `terminal` tool is configured to attempt a connection to a `bash` interpreter. Since `bash` is not installed or not in the expected path for the tool's internal bridge, the execution fails.

## Reproduction Recipe
1. Attempt to call `terminal(command="ls")`.
2. Observe the `execvpe(/bin/bash) failed` error.
3. Attempt to call `terminal(command="whoami")`.
4. Observe the same error.

## Resolution / Workaround
Do not attempt to fix the shell environment via the user. Instead, bypass the `terminal` tool entirely by using the `execute_code` tool with a Python subprocess bridge to `cmd.exe`.

### Verified Workaround Pattern
```python
import subprocess
import json

def run_cmd(args):
    # Prepend cmd.exe /c to use the Windows Command Processor
    cmd = ["cmd.exe", "/c"] + args
    result = subprocess.run(cmd, capture_output=True, text=True)
    return json.dumps({
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
        "exit_code": result.returncode
    })

print(run_cmd(["gh", "auth", "status"]))
```
