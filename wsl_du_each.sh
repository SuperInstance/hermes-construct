#!/usr/bin/env bash
LABEL="$1"; shift
TARGET="$1"
python3 - "$LABEL" "$TARGET" <<'PY'
import subprocess, sys, os
label, target = sys.argv[1], sys.argv[2]
entries = sorted(os.listdir(target))
with open('/home/eileen/audit_results.txt', 'a') as f:
    f.write(f'=== {label} ({len(entries)} items) ===\n')
    for entry in entries:
        item = os.path.join(target, entry)
        try:
            r = subprocess.run(['du', '-sk', item], capture_output=True, text=True, timeout=30)
            if r.returncode == 0:
                f.write(r.stdout)
            else:
                f.write(f'TIMEOUT {item}\n')
        except subprocess.TimeoutExpired:
            f.write(f'TIMEOUT {item}\n')
    f.write('---END---\n')
PY