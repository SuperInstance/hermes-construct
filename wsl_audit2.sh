#!/bin/bash
# Skip /proc /sys /dev /run which cause du to hang
SCAN_DIRS="/bin /boot /etc /home /lib /lib32 /lib64 /libx32 /media /mnt /opt /root /sbin /srv /usr /var"

echo "=== 17. LARGEST TOP-LEVEL DIRS UNDER / ==="
echo $SCAN_DIRS | tr ' ' '\n' | while read d; do
    [ -d "$d" ] && du -shx "$d" 2>/dev/null
done | sort -h | tail -25
echo ""

echo "=== 18. LARGEST DIRS UNDER /home ==="
du -shx /home/* 2>/dev/null | sort -h
echo ""

echo "=== 19. LARGEST DIRS UNDER /home/eileen (top 30) ==="
du -shx /home/eileen/* 2>/dev/null | sort -h | tail -30
echo ""

echo "=== 20. LARGEST DIRS UNDER /var ==="
du -shx /var/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 21. LARGEST DIRS UNDER /usr ==="
du -shx /usr/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 22. OLLAMA / LLM MODELS ==="
du -sh /usr/share/ollama 2>/dev/null
du -sh /root/.ollama 2>/dev/null
du -sh /home/*/.ollama 2>/dev/null
find /home /root /usr/share -name "*.gguf" 2>/dev/null -exec ls -lah {} \; | head -10
echo ""

echo "=== 23. TRASH DIRECTORIES ==="
ls -la /home/*/.local/share/Trash 2>/dev/null
ls -la /root/.local/share/Trash 2>/dev/null
echo ""

echo "=== 24. node_modules and site-packages ==="
echo "node_modules:"
find /home /root -name "node_modules" -type d 2>/dev/null | head -20 | while read d; do
    sz=$(du -sh "$d" 2>/dev/null | cut -f1)
    echo "  $sz  $d"
done
echo ""

echo "=== 25. .OLD/.BAK files ==="
find /home /root /var -name "*.bak" -o -name "*.old" -o -name "*.backup" 2>/dev/null | head -20
echo ""

echo "=== 26. LOGS >100MB ==="
find /var/log -type f -size +100M 2>/dev/null -exec ls -lh {} \;
echo ""

echo "=== 27. venv directories ==="
find /home /root -name ".venv" -o -name "venv" -type d 2>/dev/null | head -10 | while read d; do
    sz=$(du -sh "$d" 2>/dev/null | cut -f1)
    echo "  $sz  $d"
done
echo ""

echo "=== 28. /var/lib breakdown ==="
du -shx /var/lib/* 2>/dev/null | sort -h | tail -20
echo ""

echo "=== 29. journal breakdown ==="
ls -la /var/log/journal 2>/dev/null
echo ""

echo "=== 30. /home/eileen/.cache breakdown ==="
du -shx /home/eileen/.cache/* 2>/dev/null | sort -h | tail -20
echo ""

echo "=== 31. /root/.cache breakdown ==="
du -shx /root/.cache/* 2>/dev/null | sort -h | tail -20
echo ""

echo "=== 32. SNAP packages ==="
snap list --all 2>/dev/null
echo ""

echo "=== 33. Large individual files (>500MB) ==="
find /home /root /var /opt /usr -type f -size +500M 2>/dev/null | while read f; do
    sz=$(ls -lah "$f" 2>/dev/null | awk '{print $5}')
    echo "  $sz  $f"
done | head -30
echo ""

echo "=== 34. conda/anaconda locations ==="
ls -la /opt/conda 2>/dev/null | head -5
ls -la /opt/anaconda3 2>/dev/null | head -5
echo ""

echo "=== 35. /opt breakdown ==="
du -shx /opt/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 36. /home/eileen/.local/share breakdown ==="
du -shx /home/eileen/.local/share/* 2>/dev/null | sort -h | tail -15
echo ""

echo "=== 37. /usr/local breakdown ==="
du -shx /usr/local/* 2>/dev/null | sort -h | tail -10
echo ""

echo "=== DONE ==="
