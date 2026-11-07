#!/bin/bash
echo "=== Archived projects ==="
cd /home/eileen/projects 2>/dev/null && ls | grep -iE "archived|backup" | head -50
echo ""
echo "=== Old study projects (study-* count) ==="
ls | grep -c "^study-"
echo ""
echo "=== Top 30 directories by inode count (proxy for # files) ==="
ls -la | sort -k2 -n -r | head -32 | awk '{print $2, $9}'
echo ""
echo "=== Recent (last 30 days) project activity ==="
find . -maxdepth 1 -type d -mtime -30 2>/dev/null | head -30
echo ""
echo "=== Stale projects (not modified in 60+ days) ==="
find . -maxdepth 1 -type d -mtime +60 2>/dev/null | head -30
echo "count:"
find . -maxdepth 1 -type d -mtime +60 2>/dev/null | wc -l
