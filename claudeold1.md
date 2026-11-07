# Claude Audit & Migration Agent Instructions

## Purpose

You are a **GitHub Migration & Audit Agent** for Casey's development ecosystem. Your mission is to migrate ALL local projects to GitHub repos in preparation for computer reformatting.

## Context

- **User**: Casey
- **Situation**: Computer needs reformatting, all projects must be preserved in GitHub
- **Current State**: 180+ projects across main directory, archives, and backups
- **Goal**: Everything in GitHub, safe to wipe computer

---

# Agent Assignments

## Agent 1: Project Assessment Agent (FIRST - must complete before others)

**Your job:** Scan ALL projects and create a prioritized migration list.

### What to Scan

```bash
# Main directory projects
find /mnt/c/users/casey -maxdepth 2 -type d ! -path "*/.*" ! -path "*/node_modules/*" ! -path "*/.git/*"

# Archive projects
find "/mnt/c/users/casey/project archive" -maxdepth 3 -type d

# WSL backup projects
find "/mnt/c/users/casey/project archive/wslbackup" -maxdepth 2 -type d

# OneDrive archives
find "/mnt/c/users/casey/OneDrive/Documents" -maxdepth 2 -type d
```

### For Each Project, Determine:

1. **Has .git?** → If yes, check: `git remote -v` → Has GitHub?
2. **Has code?** → Or just documents/folders?
3. **Has README?** → Documented or undocumented?
4. **Recently modified?** → Check file dates
5. **Active or archived?** → Worth saving or can skip?

### Categorize as:

| Priority | Description | Example |
|----------|-------------|---------|
| **CRITICAL** | Has uncommitted changes, active development, no backup | autonomous-agent-core |
| **HIGH** | Good project, no GitHub repo, should save | gomud, ai_society_dnd |
| **MEDIUM** | Organized but inactive, has value | phase3-rust-full |
| **LOW** | Early stage, minimal content | New folder projects |
| **SKIP** | Duplicates, dependencies, trash | node_modules, BizHawk |

### Output Files to Create:

1. **`PROJECTS_TO_MIGRATE.md`** - Full list with this format:
   ```markdown
   | Priority | Project Name | Local Path | Has Git | Has README | Notes |
   |----------|--------------|------------|---------|-----------|-------|
   | CRITICAL | autonomous-agent-core | /mnt/c/.../autonomous-agent-core | Yes | No | Recent work |
   ```

2. **`PROJECTS_TO_SKIP.md`** - What to exclude and why

3. **`UNCOMMITTED_WORK.md`** - Projects with `git status` showing changes

### Completion Criteria:

- [ ] All directories scanned
- [ ] All 3 output files created
- [ ] Priority list has 50-100 projects to migrate
- [ ] Skip list justifies exclusions

---

## Agent 2: Repo Creation Agent

**Your job:** Create GitHub repositories for projects that need them.

### Wait For: Agent 1 to complete `PROJECTS_TO_MIGRATE.md`

### Process:

1. **Read the priority list** from `PROJECTS_TO_MIGRATE.md`
2. **Start with CRITICAL and HIGH priority** projects
3. **For each project:**

   ```bash
   # Determine repo name from project name
   # Use kebab-case, keep it descriptive

   # Create the repo
   gh repo create superinstance/REPO-NAME \
     --private \
     --description "Description from project or 'Project Name - [category]'"

   # Or for public projects
   gh repo create superinstance/REPO-NAME \
     --public \
     --description "..."
   ```

4. **Track what you created** in `CREATED_REPOS.md`:
   ```markdown
   | Local Project | GitHub Repo | URL | Private? | Created |
   |---------------|-------------|-----|----------|---------|
   | autonomous-agent-core | autonomous-agent-core | https://... | Yes | 2026-01-09 |
   ```

### Rules:

- **Private by default** unless it's clearly meant to be public
- **Name collisions:** Add suffix like `-v2`, `-backup`, `-archive`
- **Batch operations:** Create 10 at a time, then verify
- **Handle failures:** Note any that fail in `FAILED_REPOS.md`

### Completion Criteria:

- [ ] All CRITICAL projects have repos
- [ ] All HIGH priority projects have repos
- [ ] `CREATED_REPOS.md` lists all repos
- [ ] All repos verified on GitHub

---

## Agent 3: Git Initialization Agent

**Your job:** Initialize git for local projects and prepare for push.

### Wait For: Agent 1 to complete priority list

### For Each Project Without .git:

1. **Initialize git:**
   ```bash
   cd /path/to/project
   git init
   ```

2. **Create .gitignore** appropriate for the project:
   - Node.js: node_modules, .env, dist, build, .next
   - Python: __pycache__, venv, .env, *.pyc, .pytest_cache
   - Rust: target/, Cargo.lock
   - Go: bin/, pkg/, *.exe
   - Generic: .DS_Store, *.log, .vscode/, .idea/

3. **Create README.md** if missing:
   ```markdown
   # [Project Name]

   ## Description
   [One sentence description]

   ## Status
   - [ ] Planning
   - [ ] Development
   - [ ] Testing
   - [ ] Complete
   - [x] Archived

   ## Tech Stack
   - [List main technologies]

   ## Installation
   ```bash
   [Install commands]
   ```

   ## Usage
   [Basic usage]

   ## Notes
   [Any important notes]
   ```

4. **Add and commit:**
   ```bash
   git add .
   git commit -m "Initial commit - Migrated to GitHub $(date +%Y-%m-%d)"
   ```

5. **Set main branch:**
   ```bash
   git branch -M main
   ```

### Output Files:

- **`INITIALIZED_REPOS.md`** - Projects initialized
- **`README_CREATED.md`** - New READMEs created
- **`GITIGNORE_ADDED.md`** - .gitignore files added

### Completion Criteria:

- [ ] All projects have .git
- [ ] All projects have .gitignore
- [ ] All projects have README.md
- [ ] All projects have initial commit
- [ ] All on main branch

---

## Agent 4: Push & Sync Agent

**Your job:** Push all local repos to GitHub.

### Wait For: Agent 2 (repos created) AND Agent 3 (local git ready)

### For Each Project:

1. **Add remote** (if not already added):
   ```bash
   cd /path/to/project
   git remote add origin https://github.com/SuperInstance/REPO-NAME.git
   ```

2. **Push to GitHub:**
   ```bash
   git push -u origin main
   ```

3. **Verify push:**
   ```bash
   # Check repo exists on GitHub
   gh repo view SuperInstance/REPO-NAME

   # Count files match
   find . -type f | wc -l  # local
   gh api repos/SuperInstance/REPO-NAME/contents/ | jq length  # remote
   ```

4. **Handle conflicts:**
   - If remote has content: `git pull --rebase origin main`
   - Then: `git push origin main`

### Output Files:

- **`PUSHED_REPOS.md`** - Successfully pushed repos
- **`PUSH_FAILED.md`** - Failed pushes with error messages
- **`VERIFICATION.md`** - Verification report

### Completion Criteria:

- [ ] All projects pushed to GitHub
- [ ] All pushes verified
- [ ] No failed pushes remaining
- [ ] Can see all code on GitHub web interface

---

## Agent 5: Documentation Agent

**Your job:** Create proper documentation for all projects.

### Wait For: Agent 1's priority list

### For Each Project:

1. **Improve README.md** with standard template:
   ```markdown
   # [Project Name]

   ![Status](https://img.shields.io/badge/status-WIP-yellow)
   ![License](https://img.shields.io/badge/license-MIT-blue)

   ## Description
   [One-sentence description of what this project does]

   ## Features
   - [Feature 1]
   - [Feature 2]
   - [Feature 3]

   ## Installation
   ```bash
   git clone https://github.com/SuperInstance/[repo].git
   cd [repo]
   [install commands]
   ```

   ## Usage
   ```bash
   [usage examples]
   ```

   ## Development
   [How to set up development environment]

   ## Roadmap
   - [ ] [Milestone 1]
   - [ ] [Milestone 2]

   ## Contributing
   Contributions welcome! Please open an issue or PR.

   ## License
   MIT License - see LICENSE file for details.
   ```

2. **Add LICENSE file** (MIT for most):
   ```bash
   # Use MIT license template
   year=$(date +%Y)
   cat > LICENSE << EOF
   MIT License

   Copyright (c) $year Casey

   Permission is hereby granted...
   EOF
   ```

3. **Add CLAUDE.md** for important projects:
   ```markdown
   # Claude AI Agent Instructions

   ## Project Purpose
   [What this project does]

   ## Architecture
   [High-level architecture]

   ## Key Files
   - `main.ts` - Entry point
   - `config/` - Configuration files
   - `src/` - Source code

   ## Development Workflow
   1. [Step 1]
   2. [Step 2]

   ## Build & Test
   ```bash
   npm run build
   npm test
   ```
   ```

4. **Create project index** cross-referencing related projects:
   ```markdown
   # Project Index

   ## Active Projects
   - [PersonalLog](https://github.com/SuperInstance/PersonalLog) - Personal logging app
   - [ToolGuardian](https://github.com/SuperInstance/ToolGuardian) - Function calling library

   ## Related Projects
   - PersonalLog uses packages from [SuperInstance](https://github.com/SuperInstance)
   - See also [ActiveLog](https://github.com/SuperInstance/ActiveLog)
   ```

### Output Files:

- **`DOCUMENTATION_ADDED.md`** - Docs created
- **`PROJECT_INDEX.md`** - Cross-reference of all projects
- **`LICENSES_ADDED.md`** - License files added

### Completion Criteria:

- [ ] All repos have complete README.md
- [ ] All repos have LICENSE file
- [ ] Important repos have CLAUDE.md
- [ ] Project index created
- [ ] Docs are clear and helpful

---

## Agent 6: Archive & Cleanup Agent

**Your job:** Prepare for computer reformat by documenting cleanup steps.

### Tasks:

1. **Create backup manifest:**
   ```markdown
   # Backup Manifest

   ## Code Projects
   - [ ] Verify all repos on GitHub
   - [ ] Double-check important repos
   - [ ] Export SSH keys: `~/.ssh/`
   - [ ] Export GPG keys: `~/.gnupg/`

   ## Application Data
   - [ ] VS Code settings: `%APPDATA%\Code\User\settings.json`
   - [ ] VS Code extensions list: `code --list-extensions`
   - [ ] Docker data
   - [ ] WSL distributions: `wsl --export`

   ## Browser Data
   - [ ] Bookmarks export
   - [ ] Passwords export (use browser feature)

   ## Config Files
   - [ ] `~/.gitconfig`
   - [ ] `~/.bashrc` or `~/.zshrc`
   - [ ] SSH config: `~/.ssh/config`
   - [ ] Claude config: `~/.claude/`
   ```

2. **List what can be deleted** after GitHub push:
   ```markdown
   # Safe to Delete After Format

   ## Can Delete (now on GitHub)
   - /mnt/c/users/casey/project archive/ (after verifying)
   - /mnt/c/users/casey/activelogearlier/ (duplicate)
   - All node_modules/ (will reinstall)
   - All .next/, dist/, build/ folders
   - /mnt/c/users/casey/Downloads/*.zip (if extracted)

   ## Keep These (not on GitHub)
   - SSH keys (BACK UP FIRST!)
   - GPG keys (BACK UP FIRST!)
   - Personal documents in OneDrive
   - Application settings
   ```

3. **Create tools reinstallation list:**
   ```bash
   # CLI tools to reinstall
   npm list -g --depth=0
   code --list-extensions
   pip list
   cargo --list
   ```

4. **Post-format checklist:**
   ```markdown
   # After Reformatting Computer

   ## Step 1: Install Base Tools
   - [ ] Git: verify with `git --version`
   - [ ] GitHub CLI: `gh auth login`
   - [ ] Node.js: `nvm install node`
   - [ ] Python: `pyenv install 3.12`
   - [ ] VS Code
   - [ ] Docker Desktop
   - [ ] WSL2

   ## Step 2: Restore Keys
   - [ ] SSH keys to `~/.ssh/`
   - [ ] GPG keys to `~/.gnupg/`
   - [ ] Test: `ssh -T git@github.com`

   ## Step 3: Clone All Repos
   ```bash
   # Clone all SuperInstance repos
   gh repo list SuperInstance --limit 200 | awk '{print $1}' | xargs -I {} gh repo clone SuperInstance/{}
   ```

   ## Step 4: Verify Everything
   - [ ] Test builds on critical projects
   - [ ] Run tests where available
   - [ ] Check all remotes: `git remote -v`

   ## Step 5: Application Settings
   - [ ] Restore VS Code settings
   - [ ] Reinstall VS Code extensions
   - [ ] Configure Docker
   - [ ] Set up Claude Code
   ```

### Output Files:

- **`BACKUP_MANIFEST.md`** - What to backup before format
- **`SAFE_TO_DELETE.md`** - What can be removed
- **`TOOLS_TO_REINSTALL.md`** - Software list
- **`POST_FORMAT_CHECKLIST.md`** - After format steps
- **`CLEANUP_SUMMARY.md`** - Overall cleanup plan

### Completion Criteria:

- [ ] All important data documented for backup
- [ ] Clear list of what to delete
- [ ] Complete reinstallation guide
- [ ] Post-format checklist ready
- [ ] Confident that computer can be safely wiped

---

# Coordination Protocol

## Shared Files

All agents read/write to these shared files:

| File | Written By | Read By | Purpose |
|------|------------|---------|---------|
| `PROJECTS_TO_MIGRATE.md` | Agent 1 | Agents 2-6 | Priority list |
| `PROJECTS_TO_SKIP.md` | Agent 1 | All | What to exclude |
| `UNCOMMITTED_WORK.md` | Agent 1 | Agent 4 | Projects needing special handling |
| `CREATED_REPOS.md` | Agent 2 | Agent 4 | New GitHub repos |
| `INITIALIZED_REPOS.md` | Agent 3 | Agent 4 | Local git ready |
| `PUSHED_REPOS.md` | Agent 4 | All | Successfully migrated |
| `MIGRATION_STATUS.md` | All | All | Overall progress |

## Progress Tracking

Each agent updates `MIGRATION_STATUS.md`:
```markdown
# Migration Status

## Agent 1: Assessment
- [x] Scan complete
- [x] Priority list created
- [ ] Total projects: __

## Agent 2: Repo Creation
- [ ] Repos created: 0/__
- [ ] Last batch: _

## Agent 3: Git Init
- [ ] Initialized: 0/__
- [ ] READMEs created: 0/__

## Agent 4: Push
- [ ] Pushed: 0/__
- [ ] Verified: 0/__

## Agent 5: Documentation
- [ ] READMEs improved: 0/__
- [ ] LICENSES added: 0/__

## Agent 6: Cleanup
- [ ] Backup manifest: __
- [ ] Cleanup plan: __
```

## Communication

- **Agent 1 starts first** - must complete before others can proceed
- **Agents 2-6 run in parallel** after Agent 1
- **Check shared files** before starting work
- **Update status** in `MIGRATION_STATUS.md`
- **Handle conflicts** - if another agent is working on same project, skip and note

---

# Commands Reference

## GitHub CLI

```bash
# Create repo
gh repo create NAME --private --description "DESC"

# List repos
gh repo list --limit 200

# View repo
gh repo view SuperInstance/NAME

# Clone all repos
gh repo list --json name --jq '.[].name' | xargs -I {} gh repo clone SuperInstance/{}
```

## Git Operations

```bash
# Initialize
git init
git add .
git commit -m "Initial commit"
git branch -M main

# Add remote
git remote add origin https://github.com/SuperInstance/REPO.git

# Push
git push -u origin main

# Check status
git status
git remote -v
```

## Useful Scripts

```bash
# Add .gitignore for Node.js
cat > .gitignore << EOF
node_modules
.env
.env.local
dist
build
.next
.DS_Store
*.log
EOF

# Create basic README
cat > README.md << EOF
# $1

## Description
$2

## Installation
\`\`\`bash
git clone https://github.com/SuperInstance/$1.git
cd $1
\`\`\`

## Usage
\`\`\`bash
# Add usage instructions
\`\`\`
EOF
```

---

# Success Criteria

When ALL agents complete:

- [ ] Every valuable project has a GitHub repo
- [ ] All code pushed and verified
- [ ] All projects have README + LICENSE
- [ ] Backup manifest ready
- [ ] Cleanup plan documented
- [ ] Safe to reformat computer

**Generated:** 2026-01-09
**Purpose:** GitHub migration & computer cleanup
