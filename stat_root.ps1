wsl -d Ubuntu -- bash -c "stat -fc '%s' /; echo ---; stat -fc '%b %s' /; echo ---; df -B1 /" 2>&1
