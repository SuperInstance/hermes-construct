# Path Resolution Failure Log: MSYS vs Windows

## Problem Description
In a Windows environment using a bash-like shell (MSYS, Git-Bash) via the `terminal` tool, a path mismatch can occur between the agent's perceived path and the host filesystem's path structure.

## Failure Scenario: The "Ghost Write"
1. **The Request**: `write_file(path='/c/Users/casey/project/file.md')`
2. **The Error**: The tool improperly translates `/c/` to a non-existent `C:\c\` or similar structure, resulting in a file being written outside the designated workspace.
3. **The Consequence**: Subsequent `git` operations fail because the file is not visible to the repository's index.

## Reproduction Recipe
1. Use a POSIX-style path starting with `/c/` in the `write_file` tool.
2. Attempt to `git add` the file immediately after.
3. The error `fatal: pathspec '...' did not match any files` will occur.

## Resolution Pattern
1. **Use Absolute Windows Paths**: Provide `C:\Users\casey\...` to the `write_file` tool instead of `/c/Users/...`.
2. **Verify with `ls`**: Run `ls <path>` inside the `terminal` tool using the absolute path to confirm the file has physically landed in the correct directory.
3. **CD into Root**: For all git-related commands, first `cd` into the absolute path of the project root.

---

## Additional Failure: Python Subprocess Windows Path Escaping (2026-08-18)

### Problem
When passing a Windows path to a Python subprocess via `std::process::Command`, backslashes in the path are interpreted as escape sequences in the Python string literal.

### Example Failure
```rust
let python_script = format!(r#"local_dir = "{}""#, dest.display());
// dest.display() = "C:\Users\casey\AppData\Local\superinstance-embedder\models\sentence-transformers_all-MiniLM-L6-v2"
// Python receives: local_dir = "C:\Users\casey\AppData\Local\superinstance-embedder\models\sentence-transformers_all-MiniLM-L6-v2"
// Error: SyntaxError: (unicode error) 'unicodeescape' codec can't decode bytes in position 2-3: truncated \UXXXXXXXX escape
```

### Root Cause
Windows paths contain `\U` (e.g., `\Users`) which Python interprets as a Unicode escape sequence `\UXXXXXXXX`.

### Fix: Use Raw Strings in Python
```rust
let dest_str = dest.to_string_lossy().replace('\\', "\\\\");
let python_script = format!(r#"local_dir = r"{}""#, dest_str);
// Python receives: local_dir = r"C:\\Users\\casey\\AppData\\Local\\superinstance-embedder\\models\\sentence-transformers_all-MiniLM-L6-v2"
// Works correctly
```

### Key Points
- Use `r"..."` raw string prefix in Python to disable escape sequence interpretation
- Double-escape backslashes in the Rust format string (`\\\\` → `\\` in Python)
- `PathBuf::to_string_lossy()` handles non-UTF8 paths safely
- This applies to ANY Windows path passed to Python subprocesses