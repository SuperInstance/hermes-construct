#!/usr/bin/env bash
set -e
cd /home/eileen
ls projects/ > p.txt
echo "p.txt has $(wc -l < p.txt) lines"
python3 /mnt/c/Users/casey/wsl_proj_audit.py
echo "Exit: $?"
ls -la projects_sizes.txt