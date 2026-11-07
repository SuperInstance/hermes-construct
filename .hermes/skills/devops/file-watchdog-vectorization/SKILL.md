---
name: file-watchdog-vectorization
title: File Watchdog Vectorization
description: Reusable pattern for monitoring a directory and triggering a vectorization (or custom) script on file changes.
category: devops
version: 1.0
---

# Overview
A reusable pattern for monitoring a repository (or any directory) for file changes and triggering a vectorization (or any custom) script. The implementation uses the **watchdog** Python library, **pathlib** for reliable path handling, and **subprocess** to invoke a target script.

## Prerequisites
- Python 3.13+ (or any recent version) with `watchdog` installed (`pip install watchdog`).
- A script to be executed on change, e.g. `vectorization_setup.py`.
- The watchdog script should reside in the root of the repository it watches.

## Steps
1. **Create the watchdog script** (`watchdog_vectorization.py`):
   ```python
   #!/usr/bin/env python3
   import time, subprocess, sys
   from pathlib import Path
   from watchdog.observers import Observer
   from watchdog.events import FileSystemEventHandler

   class ChangeHandler(FileSystemEventHandler):
       def __init__(self, root: Path):
           self.root = root
           self.debounce = 1.0  # seconds
           self.last = 0.0

       def on_any_event(self, event):
           if event.is_directory:
               return
           now = time.time()
           if now - self.last < self.debounce:
               return
           self.last = now
           print(f"Change detected: {event.src_path}. Running vectorization...")
           subprocess.run([sys.executable, "vectorization_setup.py"], cwd=self.root)

   if __name__ == "__main__":
       # Resolve the directory containing this script
       root = Path(__file__).resolve().parent
       handler = ChangeHandler(root)
       observer = Observer()
       observer.schedule(handler, path=root, recursive=True)
       observer.start()
       print(f"Watchdog started on {root}. Press Ctrl+C to stop.")
       try:
           while True:
               time.sleep(1)
       except KeyboardInterrupt:
           observer.stop()
       observer.join()
   ```
2. **Create the target script** (`vectorization_setup.py`). It can be a placeholder that logs the trigger, or a full embedding pipeline.
3. **Run the watchdog** in the background via Hermes:
   ```bash
   python3 watchdog_vectorization.py &
   ```
   Use the `terminal` tool with `background=true` and `notify_on_complete=true` for reliable monitoring.
4. **Verify** by touching or editing any file under the watched directory; the watchdog should print a change line and invoke the target script.

## Pitfalls & Gotchas
- **Indentation errors**: Ensure the `root = Path(__file__).resolve().parent` line is indented inside the `if __name__ == "__main__":` block. A stray dedent caused an `IndentationError` in an earlier iteration.
- **Path handling**: Using `os.path.abspath(os.path.dirname(__file__))` can be brittle on Windows. `pathlib.Path` provides a cross‑platform, reliable solution.
- **Debounce timing**: Without a debounce, rapid file saves can spawn many subprocesses. The 1‑second debounce mitigates this.
- **Missing target script**: The watchdog will error if `vectorization_setup.py` does not exist. Ensure the script is present in the same directory or adjust the `cwd` argument.
- **Background process management**: When started via Hermes, set `notify_on_complete=true` so you receive a completion alert. Use `process(action='kill', session_id=…)` to stop it cleanly.

## References
- Session‑specific notes are stored in `references/watchdog_vectorization.md`.
- Official watchdog docs: https://python‑watchdog.readthedocs.io/

---
*This skill captures the pattern used in the recent session where the watchdog script was corrected to use `pathlib` and proper indentation, and the background process was managed via Hermes tooling.*