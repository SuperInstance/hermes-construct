wsl -d Ubuntu -- bash -c "tune2fs -l /dev/sda 2>&1 | head -25; echo ===END SDA===; tune2fs -l /dev/sdd 2>&1 | head -25; echo ===END SDD===" 2>&1
