#!/bin/bash
# Verify that critical repos in a GitHub org are still alive
# Usage: ./verify-fleet.sh "repo1" "repo2" ...
# Returns exit code 0 if all alive, 1 if any missing

missing=0
alive=0

for repo in "$@"; do
  if gh repo view "SuperInstance/$repo" &> /dev/null; then
    echo "✅ $repo"
    ((alive++))
  else
    echo "❌ $repo MISSING"
    ((missing++))
  fi
done

echo ""
echo "Alive: $alive / Missing: $missing"
[ $missing -eq 0 ] && exit 0 || exit 1
