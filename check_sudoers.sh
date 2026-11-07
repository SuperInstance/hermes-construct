#!/bin/bash
echo "=== /etc/sudoers.d/01-resize-nopasswd ==="
cat /etc/sudoers.d/01-resize-nopasswd
echo "=== ls -la ==="
ls -la /etc/sudoers.d/01-resize-nopasswd
echo "=== include lines in /etc/sudoers ==="
grep -E "include|@includedir" /etc/sudoers
echo "=== full /etc/sudoers ==="
cat /etc/sudoers
