wsl -d Ubuntu -- bash -c "
echo '=== mounts ==='
mount | grep -v cgroup
echo ''
echo '=== block size of sda (the ext4.vhdx) ==='
tune2fs -l /dev/sda 2>&1 | grep -E 'Block count|Block size|Filesystem'
echo ''
echo '=== block size of sdd (current /) ==='
tune2fs -l /dev/sdd 2>&1 | grep -E 'Block count|Block size|Filesystem'
echo ''
echo '=== df all ==='
df -h
echo ''
echo '=== du top-level ==='
sudo du -shx /* 2>/dev/null | sort -h | tail -20
" 2>&1
