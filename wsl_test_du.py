import subprocess
r = subprocess.run(['du', '-sk', '/home/eileen/projects/EXOCORTEX'], capture_output=True, text=True, timeout=30)
print('RC:', r.returncode)
print('STDOUT:', repr(r.stdout))
print('STDERR:', repr(r.stderr))