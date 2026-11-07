# Lever Runner Overview

- **File:** `lever_runner.py`
- **Purpose:** Provide a deterministic, token‑lean way to execute a curated set of shell commands (intents) without involving an LLM.
- **Security model:** Only commands listed in `intents.json` are allowed. The script validates the intent name before execution, preventing arbitrary code injection.
- **Execution flow:**
  1. Load `intents.json` from the same directory.
  2. Verify the supplied intent exists.
  3. Substitute any extra CLI arguments into the `{args}` placeholder.
  4. Run the command via `subprocess.run(..., shell=True, capture_output=True)`.
  5. Print stdout; if stderr is present, print it prefixed with `[Lever‑Runner] STDERR:`.
  6. Exit with the subprocess return code.
- **Typical use‑cases:**
  - Pulling latest changes (`git_pull`).
  - Running test suites (`cargo_test`).
  - Syncing Python environments (`uv_sync`).
  - Installing Node dependencies (`npm_install`).
  - Starting the watchdog (`run_watchdog`).
  - Running a linter (`run_linter`).
  - Launching a VS Code devcontainer (`start_devcontainer`).
- **Extensibility:** Add new intents by editing `intents.json`. No code changes required.
- **Pitfalls addressed:**
  - Missing `intents.json` now aborts with a clear error.
  - Command failures are reported via STDERR and exit code.
  - Argument handling is simple; complex quoting must be encoded in the intent definition.
