# GitHub Sync & Disk Cleanup 2026-08-20

## Session Summary
User requested "get everything on GitHub" then "clean up harddrive space of things we aren't using and have stored on github". Approach: verify GitHub backup before deletion, use parallel background terminals with notify_on_complete, create local-backup branches for diverged repos.

## Actions Performed
- Identified diverged repos: casting-call, cudaclaw, conservation-enforcer-rs, polln/claw, DMLog
- Created safety branches `local-backup-2026-08-20` and pushed to GitHub for diverged work-trees
- Fast-forward pulls for synced repos: activelog-ai, claudetz, hermes-avatar, pincher, plato-portal, VaaS, etc.
- Removed duplicate/sandbox temp clones
- Deleted non-git clutter: intelligence_hub/, research_lab/, ZenflowProjects/, .openclaw-autoclaw/, .pyenv/, Downloads/ComfyUI/, elevenlabs-mcp/
- Deleted verified synced repos: activelog-ai, boat-agent, hermes-avatar, perception-cascade, provenance-log, trinity-agent, tzpro-agent, vessel-agent, vessel-quest, hermes-roblox-construct, hermit-crab
- Pruned _si_research tiny clones and larger synced clones after verifying zero ahead/behind
- Backed up untracked changes before deletion: flux-runtime untracked tarball, SuperInstance open-mind patch

## Key Techniques
- Parallel terminal jobs with notify_on_complete for bulk fetches/pushes/deletes
- Safety branch creation for diverged repos: `git -C path branch -f local-backup-YYYY-MM-DD && git push -u origin local-backup-YYYY-MM-DD:refs/heads/local-backup-YYYY-MM-DD`
- Pre-deletion dirty check: `git status --porcelain`, `git rev-list --count origin/main..HEAD`, `git rev-list --count HEAD..origin/main`
- Untracked backup pattern: `tar -czf backup.tar.gz -C path <untracked paths>`
- Modified file backup: `git diff file > backup.patch`

## Pitfalls Encountered
- DMLog backup failed: `fatal: not a valid object name: 'main'` — unborn HEAD. Verify branch exists before backup.
- flux-runtime had untracked files, SuperInstance had modified file. Backed up before deletion.

## Core Keep Set
polln, claw, ai-writings, SuperInstance-papers, hermes-nerve-center, hermes-construct
