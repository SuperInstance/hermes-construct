import subprocess, os, sys
print('step1', flush=True)
with open('/home/eileen/p.txt') as f:
    projects = [l.strip() for l in f if l.strip()]
print(f'step2: {len(projects)} projects', flush=True)
sizes = []
for i, p in enumerate(projects[:5]):
    path = os.path.join('/home/eileen/projects', p)
    print(f'step3-{i}: {path}', flush=True)
    try:
        r = subprocess.run(['du', '-sk', path], capture_output=True, text=True, timeout=30)
        print(f'  rc={r.returncode} stdout={r.stdout[:50]!r}', flush=True)
        if r.returncode == 0 and r.stdout.strip():
            kb = int(r.stdout.split()[0])
            sizes.append((kb, path))
    except subprocess.TimeoutExpired:
        print(f'  TIMEOUT', flush=True)
    except Exception as e:
        print(f'  ERR: {e}', flush=True)
print(f'collected {len(sizes)} sizes', flush=True)