wsl -d Ubuntu -- bash -c "
echo '=== mounts ==='
mount | grep -v cgroup | grep -v proc
echo ''
echo '=== sda info ==='
tune2fs -l /dev/sda 2>&1 | grep -E 'Block count|Block size|Filesystem'
echo ''
echo '=== sdd info ==='
tune2fs -l /dev/sdd 2>&1 | grep -E 'Block count|Block size|Filesystem'
echo ''
echo '=== df ==='
df -h
" 2>&1
