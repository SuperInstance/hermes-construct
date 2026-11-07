#!/bin/bash
# Batch archive repos in a GitHub org with full logging
# Usage: ./batch-archive.sh "repo1" "repo2" "repo3"
# Creates archive-log-<date>.txt with results

LOG="archive-log-$(date +%Y%m%d-%H%M%S).txt"
success=0
already=0
notfound=0

echo "=== Archive Run $(date) ===" > "$LOG"

for repo in "$@"; do
  echo "→ Archiving $repo..." | tee -a "$LOG"
  result=$(gh repo archive --yes "SuperInstance/$repo" 2>&1)
  rc=$?
  
  if [ $rc -eq 0 ] && echo "$result" | grep -qi "already"; then
    echo "  ✅ Already archived: $repo" | tee -a "$LOG"
    ((already++))
  elif [ $rc -eq 0 ]; then
    echo "  ✅ Archived: $repo" | tee -a "$LOG"
    ((success++))
  else
    echo "  ⏭️  Not found/was already deleted: $repo" | tee -a "$LOG"
    echo "    Detail: $result" >> "$LOG"
    ((notfound++))
  fi
done

echo "" >> "$LOG"
echo "=== Summary ===" >> "$LOG"
echo "  Archived: $success" >> "$LOG"
echo "  Already archived: $already" >> "$LOG"
echo "  Not found (pre-deleted): $notfound" >> "$LOG"
echo "  Total handled: $((success + already + notfound))" >> "$LOG"

echo ""
echo "Archive complete. Results in $LOG"
echo "  Archived: $success"
echo "  Already archived: $already"
echo "  Not found: $notfound"
