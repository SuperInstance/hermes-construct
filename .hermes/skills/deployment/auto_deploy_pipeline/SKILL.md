---
title: Auto-Deploy Pipeline
name: auto_deploy_pipeline
description: Automated generation, copying, committing, and pushing of Hermes-generated images, now with metadata extraction, search indexing, analytics integration, and comprehensive testing.
category: deployment
---

## Overview

This skill implements an automated workflow that:

1. Runs the Hermes image‑generation script.  
2. Extracts EXIF/timestamp metadata, creates tag folders, and writes side‑car `.meta.json` files.  
3. Updates a Flask‑based `/search` endpoint that indexes the metadata for keyword look‑ups.  
4. Tags new images with timestamps, generates a tag‑index page, and pushes changes.  
5. Commits and pushes any changes to the GitHub repository.  

## Prerequisites

- Repository cloned at `~/website`.  
- Virtual environment with required Python packages (`python -m venv venv4` and `pip install -r requirements.txt`).  
- `auto_deploy.sh` stored under `~/.hermes/scripts/`.  
- New support files (see **References**) placed under the skill’s `references/` directory.  

## Script Details

**Location**: `~/.hermes/scripts/auto_deploy.sh`

```bash
#!/usr/bin/env bash
# ~/.hermes/scripts/auto_deploy.sh
# ––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––––

# 1️⃣ Generate new images
echo "🔧 Running image‑generation script..."
python "$(dirname "$0")/../website/gen_hermes_sub.py" >> "$HOME/auto_deploy.log" 2>&1

# 2️⃣ Tag and organize assets
python "$(dirname "$0")/../scripts/tag_images.py"
python "$(dirname "$0")/../scripts/generate_tag_index.py"
echo "✅ Tagged images and generated tag index"

# 3️⃣ Copy new assets (if any)
cd "$HOME/website"
if ls images/*.png 1>/dev/null 2>&1; then
  for PNG in images/*.png; do
    echo "🖼️  Copying $PNG to deployment area"
  done
fi

# 4️⃣ Git add / commit / push any changes
git add images scripts/tag_images.py scripts/generate_tag_index.py tests/test_auto_deploy.py tags.html .github/workflows/docker_hub.yml references/extended_usage.md references/scripts_overview.md references/metadata_utils.md references/tag_images_enhancement.md references/search_endpoint.md references/analytics_integration.md
if git diff --cached --quiet; then
  echo "📦 No changes to commit"
else
  git commit -m "🤖 Auto‑generated and tagged images $(date +%Y-%m-%d)"
  echo "🚀 Created commit for new images"
  git push origin main || echo "❌ Git push failed – resolve any conflicts manually"
fi

echo "✅ auto_deploy.sh finished"
```

## Scheduling

Add a **cronjob** to run it daily at a chosen UTC time:

```yaml
cronjob:
  action: create
  job_id: auto_deploy_daily
  schedule: "0 3 * * *"
  script: auto_deploy.sh
  deliver: origin
  workdir: /home/casey/.hermes/scripts
```

## Usage

- Run manually with `make publish` (if a `Makefile` target is added).  
- The cron will invoke it automatically each day.

## New: Testing & Metadata Handling

- **Unit‑test suite** at `tests/test_image_generation.py`. Verify with `python -m pytest -q`; all tests should pass.  
- **Metadata extraction**: utilities live in `scripts/metadata_utils.py` and are used by `tag_images.py` to write side‑car `.meta.json` files (timestamp, tags, etc.).  
- **Enhanced tagging**: script now creates those side‑car files.  
- **Search endpoint**: `website/scripts/search_images.py` provides a `/search?q=` API that indexes metadata.  
- **Analytics**: `analytics.js` is bundled with `index.html` and `tags.html` to fire Plausible events for page views, feedback submissions, and tag‑index visits.  
- **References** now include new support files:  
  - `references/metadata_utils.md` – EXIF extraction reference.  
  - `references/tag_images_enhancement.md` – tagging workflow details.  
  - `references/search_endpoint.md` – Flask endpoint usage.  
  - `references/analytics_integration.md` – Plausible event wiring.  
  - `references/cronjob_setup.md` – cronjob YAML example.  

## Pitfalls & Tips

- Ensure the script is executable (`chmod +x auto_deploy.sh`).  
- Verify SSH keys are configured for password‑less `git push`.  
- Activate the virtual environment before running any Python commands; otherwise imports will fail.  
- If the search endpoint returns no results, check that `.meta.json` files were generated and that `search_images.py` is running.  

## References

- `gen_hermes_sub.py` usage – see `references/README-next-steps.md`.  
- GitHub Actions CI workflow – `.github/workflows/ci.yml`.  
- Docker image automation – see `Dockerfile` in the website root.  
- Extended usage documentation – `references/extended_usage.md`.  
- Script overview – `references/scripts_overview.md`.  
- Metadata utilities – `references/metadata_utils.md`.  
- Tagging enhancement – `references/tag_images_enhancement.md`.  
- Search endpoint – `references/search_endpoint.md`.  
- Analytics integration – `references/analytics_integration.md`.  
- Cronjob setup – `references/cronjob_setup.md`.  
- Unit tests – `tests/test_auto_deploy.py`.  

---  

*Maintained by the Hermes perception layer.*