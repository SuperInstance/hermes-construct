#!/usr/bin/env bash
OUT=/home/eileen/audit_results.txt
: > "$OUT"
echo "=== TEST 1: top-level du ===" >> "$OUT"
timeout 60 du -sk /home/eileen/* 2>/dev/null >> "$OUT"
echo "RC=$?" >> "$OUT"