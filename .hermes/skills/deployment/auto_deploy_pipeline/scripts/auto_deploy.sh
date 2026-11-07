#!/usr/bin/env bash
# ~/.hermes/scripts/auto_deploy.sh
# –––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––
# 1️⃣ Run the Hermes image generator
# 2️⃣ Copy any newly-created PNGs into the website assets
# 3️⃣ Git add / commit / push any changes
# –––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––

set -euo pipefail

# ---------- 1️⃣ Generate new images ----------
echo "🔧 Running image‑generation script..."
python "$(dirname "$0")/../website/gen_hermes_sub.py" >> "$HOME/auto_deploy.log" 2>&1

# ---------- 2️⃣ Copy new assets ----------
if ls "$HOME/website/output"/*.png 1>/dev/null 2>&1; then
  TS=$(date +%Y%m%d_%H%M%S)
  for PNG in "$HOME/website/output"/*.png; do
    cp "$PNG" "$HOME/website/images/Hermes_Sub_${TS}_$(basename "$PNG")"
  done
  echo "✅ Copied new images with timestamp ${TS}"
else
  echo "⚠️ No new PNGs – nothing to copy"
fi

# ---------- 3️⃣ Commit & push ----------
cd "$HOME/website"
if git diff --cached --quiet; then
  echo "📦 No image changes to commit"
else
  git add images
  git commit -m "🤖 Auto‑generated images $(date +%Y-%m-%d)"
  echo "🚀 Created commit for new images"
  git push origin main || echo "❌ Git push failed – resolve any conflicts manually"
fi

echo "✅ auto_deploy.sh finished"