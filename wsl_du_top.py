import subprocess, sys
target = sys.argv[1]
out = sys.argv[2] if len(sys.argv) > 2 else '/home/eileen/top_out.txt'
r = subprocess.run(['du', '-k', '--max-depth=2', target], capture_output=True, text=True, timeout=120)
items = []
for l in r.stdout.strip().split('\n'):
    parts = l.split('\t')
    if len(parts) == 2:
        try:
            items.append((int(parts[0]), parts[1]))
        except: pass
items.sort(reverse=True)
with open(out, 'w') as f:
    for kb, p in items[:25]:
        f.write(f'{kb:>10} KB  {p}\n')
print(f'wrote {out}')