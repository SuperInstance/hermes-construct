#!/bin/bash
echo "================================================================"
echo " WSL UBUNTU - TRASH/RECLAIMABLE ITEMIZATION (no deletes)"
echo "================================================================"

echo ""
echo "=== 1. APT CACHE (/var/cache/apt) ==="
du -sh /var/cache/apt/archives 2>/dev/null && echo "  ^ downloaded .deb files, regenerable"
echo ""

echo "=== 2. APT AUTOREMOVE CANDIDATES (orphaned packages) ==="
apt-get --dry-run autoremove 2>/dev/null | tail -5
echo ""

echo "=== 3. PIP CACHE ==="
du -sh /root/.cache/pip 2>/dev/null
du -sh /home/*/.cache/pip 2>/dev/null
echo ""

echo "=== 4. NPM CACHE ==="
du -sh /root/.npm 2>/dev/null
du -sh /home/*/.npm 2>/dev/null
echo ""

echo "=== 5. CARGO/RUST CACHE ==="
du -sh /root/.cargo 2>/dev/null
du -sh /home/*/.cargo 2>/dev/null
echo ""

echo "=== 6. GO CACHE ==="
du -sh /root/.cache/go-build 2>/dev/null
du -sh /home/*/.cache/go-build 2>/dev/null
echo ""

echo "=== 7. GRADLE / MAVEN CACHE (if java dev) ==="
du -sh /root/.gradle 2>/dev/null
du -sh /home/*/.gradle 2>/dev/null
du -sh /root/.m2 2>/dev/null
du -sh /home/*/.m2 2>/dev/null
echo ""

echo "=== 8. DOCKER (if installed) ==="
du -sh /var/lib/docker 2>/dev/null
du -sh /root/.docker 2>/dev/null
du -sh /home/*/.docker 2>/dev/null
echo ""

echo "=== 9. /var/log (system logs) ==="
du -sh /var/log 2>/dev/null
ls -lah /var/log 2>/dev/null | head -20
echo ""

echo "=== 10. /tmp (temp files) ==="
du -sh /tmp 2>/dev/null
echo ""

echo "=== 11. /var/tmp ==="
du -sh /var/tmp 2>/dev/null
echo ""

echo "=== 12. /var/cache (all caches) ==="
du -sh /var/cache/* 2>/dev/null | sort -h | tail -20
echo ""

echo "=== 13. OLD KERNELS / HEADERS ==="
ls -la /usr/src 2>/dev/null
echo "  --- linux-headers ---"
dpkg -l 'linux-headers-*' 2>/dev/null | tail -5
echo "  --- linux-image (extras) ---"
dpkg -l 'linux-image-*' 2>/dev/null | tail -5
echo ""

echo "=== 14. JOURNAL (systemd logs) ==="
journalctl --disk-usage 2>/dev/null
echo ""

echo "=== 15. SNAP CACHE ==="
du -sh /var/cache/snapd 2>/dev/null
du -sh /var/lib/snapd 2>/dev/null
ls -la /var/lib/snapd/snaps 2>/dev/null | tail -10
echo ""

echo "=== 16. CRASH DUMPS (core.* in /) ==="
ls -lah /core.* 2>/dev/null | head
echo ""

echo "=== 17. LARGEST TOP-LEVEL DIRS UNDER / ==="
du -shx /* 2>/dev/null | sort -h | tail -25
echo ""

echo "=== 18. LARGEST DIRS UNDER /home ==="
du -shx /home/* 2>/dev/null | sort -h | tail -10
echo ""
if [ -d /home/eileen ]; then
    echo "=== 19. LARGEST DIRS UNDER /home/eileen (top 25) ==="
    du -shx /home/eileen/* 2>/dev/null | sort -h | tail -25
    echo ""
fi

echo "=== 20. LARGEST DIRS UNDER /var (top 15) ==="
du -shx /var/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 21. LARGEST DIRS UNDER /usr (top 15) ==="
du -shx /usr/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 22. OLLAMA / LLM MODELS ==="
du -sh /usr/share/ollama 2>/dev/null
du -sh /root/.ollama 2>/dev/null
du -sh /home/*/.ollama 2>/dev/null
find / -name "*.gguf" 2>/dev/null -exec ls -lah {} \; | head -10
echo ""

echo "=== 23. TRASH DIRECTORIES ==="
ls -la /home/*/.local/share/Trash 2>/dev/null
ls -la /root/.local/share/Trash 2>/dev/null
echo ""

echo "=== 24. PYTHON VENVS / NODE_MODULES BIG ONES ==="
find /home /root -name "node_modules" -type d 2>/dev/null | head -10
find / -path /proc -prune -o -name "site-packages" -type d -print 2>/dev/null | head -10
echo ""

echo "=== 25. ANY .OLD/.BAK FILES ==="
find /home /root /var -name "*.bak" -o -name "*.old" -o -name "*.backup" 2>/dev/null | head -20
echo ""

echo "=== 26. TMPFS / LOGS LARGER THAN 100MB ==="
find /var/log -type f -size +100M 2>/dev/null -exec ls -lh {} \;
echo ""

echo "================================================================"
echo " DONE"
echo "================================================================"
