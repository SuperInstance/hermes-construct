This reference documents the extended deployment workflow, including tag generation and CI integration.

## New Steps Added

1. **Tag Images**: `python scripts/tag_images.py` – Walks `images/` and creates `_tagged` subfolders.
2. **Generate Tag Index**: `python scripts/generate_tag_index.py` – Outputs `tags.html` listing tags and thumbnails.
3. **CI / Docker Hub**: Updated `.github/workflows/docker_hub.yml` now builds and pushes an image on each `main` merge.
4. **Unit Tests**: Added `tests/test_auto_deploy.py` to verify PNG generation and commit flow.

These steps are automatically invoked by the updated `auto_deploy.sh` script.