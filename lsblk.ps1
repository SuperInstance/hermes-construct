wsl -d Ubuntu -- bash -c "lsblk -o NAME,SIZE,MOUNTPOINTS,FSTYPE,FSVER,LABEL,UUID 2>&1" 2>&1
