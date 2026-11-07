import subprocess
r = subprocess.run(['du', '-k', '--max-depth=2', '/home/eileen/backups/'], capture_output=True, text=True, timeout=120)
print(f'len: {len(r.stdout)}')
print(f'stderr: {r.stderr[:300]}')
print(r.stdout[:500])