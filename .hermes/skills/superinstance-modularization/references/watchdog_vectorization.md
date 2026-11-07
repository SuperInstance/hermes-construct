# Watchdog Vectorization Guide

## Purpose
Continuously monitor the repository for file changes and trigger the vectorization pipeline.

## Implementation (`watchdog_vectorization.py`)
```python
import sys
import time
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

class VectorHandler(FileSystemEventHandler):
    def on_modified(self, event):
        if not event.is_directory:
            print(f"[watchdog] Modified: {event.src_path}")
            # Call the vectorization entry point
            import subprocess
            subprocess.run([sys.executable, "vectorization_setup.py", event.src_path])

if __name__ == "__main__":
    path = "."  # repo root
    observer = Observer()
    observer.schedule(VectorHandler(), path, recursive=True)
    observer.start()
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
    observer.join()
```

## Common Issues & Fixes
- **`ModuleNotFoundError: No module named 'watchdog'`** – install with `python -m pip install watchdog`.
- **Path handling on Windows** – use POSIX paths (`/c/Users/...`) when invoking from the terminal tool.
- **Background process silence** – always launch with `notify_on_complete=True` so Hermes receives a completion notification.
- **Recursive loops** – ensure the handler ignores changes to generated files (e.g., skip `*.pyc`).

## Verification Steps
1. Start the script in background:
   ```bash
   hermes terminal "python watchdog_vectorization.py" background=true notify_on_complete=true
   ```
2. Modify any source file (e.g., `touch src/lib.rs`).
3. Observe the console output from the background process indicating the file path and that `vectorization_setup.py` was invoked.

---
*Generated from session 2026‑08‑17*