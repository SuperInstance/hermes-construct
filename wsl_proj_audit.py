import subprocess, os, sys

LOG = '/home/eileen/projects_sizes.txt'

with open('/home/eileen/p.txt') as f:
    projects = [l.strip() for l in f if l.strip()]

with open(LOG, 'w') as f:
    f.write(f'Total projects: {len(projects)}\n\n')

sizes = []
for i, p in enumerate(projects):
    path = os.path.join('/home/eileen/projects', p)
    try:
        r = subprocess.run(['du', '-sk', path], capture_output=True, text=True, timeout=30)
        if r.returncode == 0 and r.stdout.strip():
            kb = int(r.stdout.split()[0])
            sizes.append((kb, path))
        else:
            sizes.append((0, path))
    except subprocess.TimeoutExpired:
        sizes.append((-1, path))
    except Exception as e:
        sizes.append((-2, path))
    if i % 25 == 0:
        with open(LOG, 'a') as f:
            f.write(f'progress: {i}/{len(projects)}\n')

sizes.sort(reverse=True)
with open(LOG, 'a') as f:
    f.write(f'\n=== TOP 50 BY SIZE ===\n')
    for kb, path in sizes[:50]:
        marker = ''
        if kb == -1: marker = ' [TIMEOUT]'
        elif kb == -2: marker = ' [ERR]'
        f.write(f'{kb:>10} KB{marker}  {path}\n')
    f.write(f'\n=== TOTAL OF NON-ZERO ===\n')
    total = sum(k for k,_ in sizes if k > 0)
    f.write(f'{total} KB = {total/1024/1024:.2f} GB\n')
print('done')