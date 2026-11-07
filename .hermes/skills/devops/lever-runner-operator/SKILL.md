---
name: lever-runner-operator
description: Token‑lean operator that runs only pre‑approved shell intents.
summary: Simple CLI wrapper that validates intent names against a whitelist JSON and executes the associated command.
---

## Overview
`lever_runner.py` provides a deterministic, low‑token way to run common development commands without invoking an LLM. The whitelist lives in `intents.json` and maps intent names to exact shell strings.

### Usage
```bash
python lever_runner.py <intent> [args...]
```
- The script loads `intents.json` from the same directory.
- If the intent is not present, it prints an error and exits with code 1.
- Arguments are interpolated into the command via `{args}` placeholder.
- The command runs via `subprocess.run(..., shell=True, capture_output=True)` and prints stdout and any stderr.

## Common Intents (as of now)
- `git_pull` → `git pull`
- `cargo_test` → `cargo test --workspace`
- `uv_sync` → `uv sync`
- `npm_install` → `npm install`
- `run_watchdog` → `python3 watchdog_vectorization.py`
- `run_linter` → `uv run flake8 .`
- `start_devcontainer` → `code . --folder-uri "vscode-remote://devcontainer+${PWD}"`

## Pitfalls & Work‑arounds
- **Missing intents.json** – Ensure the file is present alongside `lever_runner.py`. The script will abort with a clear message if not found.
- **Command failures** – The runner prints STDERR but still returns the subprocess exit code. Caller should check the exit status.
- **Argument injection** – Only simple space‑separated arguments are supported; complex quoting must be handled in the intent definition.
- **Security** – Because the whitelist is static, no arbitrary code can be executed, satisfying the token‑lean safety requirement.

## Extending the Whitelist
Add a new entry to `intents.json`:
```json
"my_intent": "my_command --option {args}"
```
Then run `python lever_runner.py my_intent arg1 arg2`.

## References
- `references/lever_runner_overview.md` – design rationale and security considerations.
- `references/intents_json_example.md` – full current whitelist.
