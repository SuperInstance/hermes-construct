# GitHub Migration & Computer Cleanup Plan

**Goal:** Get ALL projects into GitHub repos, then safely reformat computer
**Date:** 2026-01-09
**Status:** PLANNING → EXECUTION

---

## Project Inventory Summary

| Category | Count | Need GitHub |
|----------|-------|-------------|
| Active Local Projects | 62 | ~20 need repos |
| Archived Projects | 120+ | ~50 worth saving |
| Already on GitHub | 62 | 0 |
| **Total to Process** | **~180** | **~70 new repos needed** |

---

## Phase 1: Assessment & Categorization (1 agent)

**Agent 1: Project Assessment Agent**

Identify which projects need GitHub repos and categorize by priority:

### Priority Levels

| Priority | Criteria | Count |
|----------|----------|-------|
| **CRITICAL** | Active development, uncommitted work | ~10 |
| **HIGH** | Recent work, documented,有价值 | ~20 |
| **MEDIUM** | Organized but inactive | ~30 |
| **LOW** | Early stage, minimal content | ~50 |
| **SKIP** | Duplicates, node_modules, trash | ~70 |

### Output Required

- `PROJECTS_TO_MIGRATE.md` - Full list with priorities
- `PROJECTS_TO_SKIP.md` - What to exclude (with reasons)
- `UNCOMMITTED_WORK.md` - Projects with git changes to preserve

---

## Phase 2: GitHub Repo Creation (1 agent)

**Agent 2: Repo Creation Agent**

Create GitHub repositories for all projects that need them.

### Repo Creation Rules

1. **Naming Convention:**
   - Use kebab-case for repo names
   - Prefix with category if helpful (e.g., `ai-`, `game-`, `tool-`)
   - Keep names descriptive but concise

2. **Repo Settings:**
   - Private by default (unless public makes sense)
   - Add description from project's README or purpose
   - Initialize with README if none exists
   - Add `.gitignore` appropriate for the language
   - Choose appropriate license (MIT for most)

3. **Batch Creation:**
   - Use GitHub CLI: `gh repo create`
   - Create repos in batches of 10
   - Track created repos in spreadsheet

### Output Required

- `CREATED_REPOS.md` - List of all repos created with URLs
- `FAILED_REPOS.md` - Any that failed and why

---

## Phase 3: Local Repo Setup (1 agent)

**Agent 3: Git Initialization Agent**

Initialize git repos for local projects and add remotes.

### Tasks

1. **For projects without .git:**
   ```bash
   cd /path/to/project
   git init
   git add .
   git commit -m "Initial commit - Migration from local development"
   ```

2. **Create README.md if missing:**
   ```markdown
   # Project Name

   ## Description
   [Brief description of project purpose]

   ## Status
   - [ ] Planning
   - [ ] In Progress
   - [ ] Archived

   ## Tech Stack
   - [List technologies]

   ## Next Steps
   - [ ] Document architecture
   - [ ] Add tests
   - [ ] Deploy
   ```

3. **Add appropriate .gitignore:**
   - Node.js: node_modules, .env, dist, build
   - Python: __pycache__, venv, .env, *.pyc
   - Rust: target, Cargo.lock
   - Go: bin, pkg, *.exe

4. **Add GitHub remote:**
   ```bash
   git remote add origin https://github.com/SuperInstance/repo-name.git
   ```

### Output Required

- `INITIALIZED_REPOS.md` - List of projects initialized
- `README_CREATED.md` - Projects with new READMEs
- `GITIGNORE_ADDED.md` - Projects with .gitignore

---

## Phase 4: Push to GitHub (1 agent)

**Agent 4: Push & Sync Agent**

Push all local repos to GitHub and verify.

### Tasks

1. **Push all repos:**
   ```bash
   git push -u origin main
   # Or if master is default:
   git push -u origin master
   ```

2. **Handle branch naming:**
   - Rename master to main if needed
   - Set main as default branch

3. **Verify pushes:**
   - Check repo exists on GitHub
   - Verify file count matches
   - Check latest commit appears

4. **Create protection rules (for important repos):**
   - Require pull request for main
   - Require status checks

### Output Required

- `PUSHED_REPOS.md` - Successfully pushed repos
- `PUSH_FAILED.md` - Failed pushes with errors
- `VERIFICATION.md` - Verification report

---

## Phase 5: Documentation & Organization (1 agent)

**Agent 5: Documentation Agent**

Create proper documentation for all projects.

### Tasks

1. **Standard README.md Template:**
   ```markdown
   # [Project Name]

   ![Status](https://img.shields.io/badge/status-WIP-yellow)
   ![License](https://img.shields.io/badge/license-MIT-blue)

   ## Description
   [One-sentence description]

   ## Features
   - [Feature 1]
   - [Feature 2]

   ## Installation
   ```bash
   git clone https://github.com/SuperInstance/[repo].git
   cd [repo]
   [Install commands]
   ```

   ## Usage
   [Basic usage examples]

   ## Development
   [Setup for development]

   ## Roadmap
   - [ ] Milestone 1
   - [ ] Milestone 2

   ## Contributing
   [Contribution guidelines]

   ## License
   MIT License - see LICENSE file
   ```

2. **Add CLAUDE.md to important repos:**
   - Instructions for AI agents working on the project
   - Architecture overview
   - Key files and their purposes
   - Development workflow

3. **Create project-level index:**
   - Group related projects
   - Document dependencies between projects
   - Note which are production-ready vs experimental

### Output Required

- `DOCUMENTATION_ADDED.md` - Documentation created
- `PROJECT_INDEX.md` - Cross-reference of all projects
- `TEMPLATE_README.md` - README template for reuse

---

## Phase 6: Archive & Cleanup Prep (1 agent)

**Agent 6: Archive & Cleanup Agent**

Prepare for computer reformat by organizing archives and cleanup lists.

### Tasks

1. **Create backup manifest:**
   - List all important data locations
   - Note any non-code folders to preserve
   - Document application settings to export

2. **Organize archives:**
   - Consolidate duplicate projects
   - Mark what can be deleted after GitHub push
   - Create "DELETE_AFTER_FORMAT" list

3. **Generate cleanup checklist:**
   ```markdown
   # Pre-Format Checklist

   ## Code Projects
   - [ ] All repos pushed to GitHub
   - [ ] Verification complete
   - [ ] Local backups made (optional)

   ## Data to Preserve
   - [ ] SSH keys backed up
   - [ ] GPG keys backed up
   - [ ] Application configs documented
   - [ ] Browser exports (bookmarks, passwords)

   ## After Format
   - [ ] Clone all repos from GitHub
   - [ ] Restore SSH keys
   - [ ] Reinstall tools from list
   - [ ] Verify everything works
   ```

4. **Create tools reinstallation list:**
   - List all installed CLI tools
   - Document VS Code extensions
   - Note Python packages, npm globals, etc.

### Output Required

- `BACKUP_MANIFEST.md` - What to backup before format
- `DELETE_AFTER_FORMAT.md` - Safe to delete
- `TOOLS_TO_REINSTALL.md` - Software reinstallation list
- `POST_FORMAT_CHECKLIST.md` - After format steps

---

## Parallel Execution Strategy

### Agent Assignments

| Agent | Focus | Estimated Projects |
|-------|-------|-------------------|
| 1 | Assessment & Categorization | 180 projects |
| 2 | GitHub Repo Creation | ~70 new repos |
| 3 | Git Initialization | ~50 projects |
| 4 | Push to GitHub | ~120 projects |
| 5 | Documentation | ~100 projects |
| 6 | Archive & Cleanup | All projects |

### Coordination

- **Agent 1 goes first** - produces priority list for others
- **Agents 2-6 work in parallel** once Agent 1 completes
- **Use shared files** for coordination:
  - `PROJECTS_TO_MIGRATE.md` (input for agents 2-6)
  - `CREATED_REPOS.md` (output from agent 2, input for agent 4)
  - `PROCESSED_REPOS.md` (tracking all agents)

---

## Success Criteria

- [ ] All active projects have GitHub repos
- [ ] All code is pushed and verified
- [ ] All projects have at least basic README
- [ ] Archive projects documented or deleted
- [ ] Backup manifest ready
- [ ] Safe to reformat computer

---

## Timeline Estimate

| Phase | Duration | Dependencies |
|-------|----------|--------------|
| Phase 1: Assessment | 30 min | None |
| Phases 2-6 (parallel) | 2-4 hours | Phase 1 complete |
| **Total** | **3-5 hours** | |

---

## Commands Reference

### GitHub CLI (Batch Operations)

```bash
# Create private repo with description
gh repo create repo-name --private --description "Description"

# Create public repo
gh repo create repo-name --public

# Create with README
gh repo create repo-name --public --readme.md

# List all repos
gh repo list --limit 200

# Clone all repos (for post-format)
gh repo list --json name --jq '.[].name' | xargs -I {} gh repo clone SuperInstance/{}
```

### Git Operations

```bash
# Init and push new repo
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/SuperInstance/repo.git
git push -u origin main

# Add .gitignore
# Use: https://gitignore.io/api/node,python,rust,go

# Create initial README
echo "# Project Name" > README.md
```

---

**Status:** Ready to spawn agents
**Next:** Spawn 6 parallel agents
