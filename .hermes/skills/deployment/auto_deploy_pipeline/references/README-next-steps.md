# Future Enhancements for Hermes Image Gallery

## 1. Search & Tagging
- Extract metadata (EXIF, timestamps, image hash) from generated PNGs.
- Store metadata in a lightweight SQLite DB.
- Build a client‑side search UI that filters thumbnails by tag or keyword.

## 2. Analytics Integration
- Add Plausible (or Umami) analytics script to track page views, referral sources, and user interactions.
- Create a simple dashboard showing:
  - Daily unique visitors
  - Most‑viewed images
  - Geographic distribution

## 3. Scheduled Generation & Auto‑Push
- Enable a nightly cron job (03:00 UTC) that runs `gen_hermes_sub.py`.
- Auto‑copy new PNGs into `website/images/`, commit, and push to `main`.
- Tag each commit with the generation timestamp for easy rollback.

## 3.1 Docker Automation
- Extend the pipeline script to build a Docker image after each push.
- Push the image to Docker Hub or GHCR and deploy via a container orchestrator.

## 4. Feedback Loop
- Add a “Leave Feedback” form on each image page.
- Store submissions in a separate DB table and receive weekly summary emails.

## 5. Documentation Hub
- Convert `README-next-steps.md` into a live documentation site using MkDocs or Docusaurus.
- Include API reference snippets for `gen_hermes_sub.py` and the auto‑deploy script.

## 6. CI/CD Pipeline
- Add GitHub Actions workflow:
  - Lint Dockerfile
  - Run unit tests on the generation script
  - Build and push Docker image on each merge
- Enable badge display in the repo README.

These items build on the current auto‑deploy pipeline and will turn the static gallery into a full‑featured, community‑driven showcase of Hermes‑generated art.