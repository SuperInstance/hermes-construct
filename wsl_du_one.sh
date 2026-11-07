#!/usr/bin/env bash
LABEL="$1"; shift
DEPTH="$1"; shift
TARGET="$*"
python3 - "$LABEL" "$DEPTH" "$TARGET" <<'PY'
import subprocess, sys
label, depth, target = sys.argv[1], int(sys.argv[2]), sys.argv[3]
with open('/home/eileen/audit_results.txt', 'a') as f:
    f.write(f'=== {label} (depth={depth}) ===\n')
    cmd = ['du', '-k', f'--max-depth={depth}', target]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
    f.write(result.stdout)
    if result.stderr:
        f.write('STDERR: ' + result.stderr[:500] + '\n')
    f.write('---END---\n')
PY