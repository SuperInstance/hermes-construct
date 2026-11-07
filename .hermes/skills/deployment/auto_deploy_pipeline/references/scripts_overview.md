## Overview of New Support Scripts

| Script | Purpose | Location | Executable |
|--------|---------|----------|------------|
| `scripts/tag_images.py` | Scans `images/` for new PNGs and creates timestamped `_tagged/` folders; extracts generation timestamp for tagging. | `~/website/scripts/tag_images.py` | Yes |
| `scripts/generate_tag_index.py` | Generates `tags.html` page that lists each tag folder and thumbnails, enabling easy browsing. | `~/website/scripts/generate_tag_index.py` | Yes |
| `tests/test_auto_deploy.py` | Unit tests verifying that the pipeline generates PNGs and that `auto_deploy.sh` can stage/commit/push changes. | `~/website/tests/test_auto_deploy.py` | N/A (run via pytest) |
| `.github/workflows/docker_hub.yml` | GitHub Actions CI workflow that builds a Docker image on every push to `main` and pushes to Docker Hub. | `~/website/.github/workflows/docker_hub.yml` | N/A |
| `Dockerfile` | Container definition for serving the static site via Nginx in a lightweight image. | `~/website/Dockerfile` | N/A |
| `docker-compose.yml` | Orchestrates the local development stack for testing the site and CI pipeline. | `~/website/docker-compose.yml` | N/A |

All of these scripts are referenced in the auto‑deploy pipeline to ensure reproducibility and traceability.