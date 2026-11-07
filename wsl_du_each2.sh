#!/usr/bin/env bash
exec > /home/eileen/du_proj.log 2>&1
echo "starting"
while IFS= read -r p; do
    [ -z "$p" ] && continue
    timeout 20 du -sk "/home/eileen/projects/$p" 2>/dev/null || echo "0	/home/eileen/projects/$p [timeout]"
done < /home/eileen/p.txt
echo "done"