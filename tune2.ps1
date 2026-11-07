wsl -d Ubuntu -- bash -c "sudo tune2fs -l /dev/sda 2>&1; echo ===END SDA===; sudo tune2fs -l /dev/sdd 2>&1" 2>&1
