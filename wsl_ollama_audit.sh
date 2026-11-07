#!/bin/bash
echo "=== All ollama models installed ==="
find /home/eileen/.ollama/models/manifests -type f 2>/dev/null | sort
echo ""
echo "=== Total ollama manifest count ==="
find /home/eileen/.ollama/models/manifests -type f 2>/dev/null | wc -l
echo ""
echo "=== When was each model last used? ==="
for m in $(find /home/eileen/.ollama/models/manifests -type f 2>/dev/null); do
    name=$(echo "$m" | sed 's|.*/library/||;s|.*/||')
    age=$(stat -c %Y "$m" 2>/dev/null)
    if [ -n "$age" ]; then
        echo "$age  $name"
    fi
done | sort -n | awk '{cmd="date -d @"$1" +%Y-%m-%d"; cmd | getline d; close(cmd); print d, $0}' | head -50
