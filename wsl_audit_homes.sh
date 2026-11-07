#!/usr/bin/env bash
OUT=/tmp/audit.out
echo "=== Top-level /home/eileen items (KB) ===" > "$OUT"
du -sk /home/eileen/* 2>/dev/null | /usr/bin/sort -n | /usr/bin/tail -n 40 >> "$OUT"
echo "" >> "$OUT"
echo "=== Hidden dirs in /home/eileen (KB) ===" >> "$OUT"
du -sk /home/eileen/.[!.]* 2>/dev/null | /usr/bin/sort -n | /usr/bin/tail -n 40 >> "$OUT"
echo "" >> "$OUT"
echo "=== /var/log (KB) ===" >> "$OUT"
du -sk /var/log/* 2>/dev/null | /usr/bin/sort -n | /usr/bin/tail -n 20 >> "$OUT"
echo "" >> "$OUT"
echo "=== /var/cache (KB) ===" >> "$OUT"
du -sk /var/cache/* 2>/dev/null | /usr/bin/sort -n | /usr/bin/tail -n 20 >> "$OUT"
echo "DONE" >> "$OUT"