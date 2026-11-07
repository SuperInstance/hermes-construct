#!/bin/bash
# Multi-repo audit: classify each local repo as SYNCED/BEHIND/AHEAD/DIRTY
# Usage: multi_repo_audit.sh /c/Users/casey <path2> <path3> ...
# All paths are printed to stdout for piping into other commands.
#
# Output format: STATUS|AHEAD|BEHIND|DIRTY_FILES|REMOTE_URL|BRANCH|REPO_NAME
# where STATUS is one of: SYNCED, BEHIND, AHEAD, DIRTY_SYNC
#
# Filters out repos that fail fetch (no network, no remote, etc.)

for d in "$@"; do
  repo=$(basename "$d")
  
  # Get remote and branch
  remote=$(git -C "$d" remote get-url origin 2>/dev/null) || continue
  branch=$(git -C "$d" branch --show-current 2>/dev/null) || continue
  
  # Fetch remote state
  git -C "$d" fetch origin --quiet 2>/dev/null || continue
  
  # Determine upstream name
  upstream="${branch}"
  
  # Count ahead/behind
  ahead=$(git -C "$d" rev-list --count "${upstream}..HEAD" 2>/dev/null || echo "0")
  behind=$(git -C "$d" rev-list --count "HEAD..${upstream}" 2>/dev/null || echo "0")
  dirty=$(git -C "$d" status --porcelain 2>/dev/null | wc -l)
  
  # Classify
  if [ "$ahead" -gt 0 ] 2>/dev/null; then
    status="AHEAD"
  elif [ "$behind" -gt 0 ] 2>/dev/null; then
    status="BEHIND"
  elif [ "$dirty" -gt 0 ] 2>/dev/null; then
    status="DIRTY_SYNC"
  else
    status="SYNCED"
  fi
  
  echo "${status}|${ahead}|${behind}|${dirty}|${remote}|${branch}|${repo}"
done
