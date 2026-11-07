# Backup Manifest

> Created: 2026-01-09
> Purpose: Document all critical data to backup before computer reformat

## Critical Security Keys

### SSH Keys
- **Location (Windows)**: `/mnt/c/users/casey/.ssh/`
- **Location (WSL)**: `~/.ssh/` (`/home/casey/.ssh/`)
- **Files to backup**:
  - `id_ed25519` - Private SSH key
  - `id_ed25519.pub` - Public SSH key
  - `known_hosts` - Known hosts file

### GPG Keys
- **Location**: Not found - GPG keys may not be set up or in a different location
- **Action Required**: Check if GPG keys exist with `gpg --list-keys`

## Application Configurations

### Git Configuration
- **Location**: `~/.gitconfig` or `/mnt/c/users/casey/.gitconfig`
- **Content**:
  ```ini
  [credential]
      helper = store
  [user]
      email = claude@superinstance.com
      name = Claude (SuperInstance)
  ```

### GitHub CLI
- **Location**: `~/.config/gh/hosts.yml`
- **Authenticated Account**: SuperInstance
- **Token scopes**: admin:public_key, gist, read:org, repo
- **Git protocol**: SSH

### VS Code Settings
- **Location (Windows)**: `/mnt/c/users/casey/AppData/Roaming/Code/User/settings.json`
- **Location (WSL)**: `~/.vscode-server/data/Machine/settings.json`
- **Current Settings**:
  ```json
  {
      "security.allowedUNCHosts": ["wsl.localhost"],
      "editor.inlineSuggest.syntaxHighlightingEnabled": true,
      "python.defaultInterpreterPath": "c:\\Users\\casey\\AppData\\Local\\Programs\\Python\\Python312\\python.exe",
      "Codegeex.EnableExtension": false
  }
  ```

### AWS Credentials
- **Location**: `/mnt/c/users/casey/.aws/`
- **Files**: `config`, `credentials`

### Docker Configuration
- **Location**: `/mnt/c/users/casey/AppData/Roaming/Docker/`
- **Files to backup**:
  - `config.json`
  - `daemon.json`
  - `contexts/` directory

### Cargo/Rust
- **Location**: `/mnt/c/users/casey/.cargo/`
- **Files to backup**:
  - `.global-cache`
  - `bin/` directory
  - `registry/` directory
- **Rust version**: 1.92.0 (ded5c06cf 2025-12-08)

### Bash Configuration
- **Location**: `~/.bashrc`
- **Important exports**:
  - PATH includes: `~/.local/bin`, `~/.npm-global/bin`, Go binaries
  - API Keys (move to secure location after reformat):
    - ANTHROPIC_API_KEY
    - ZAI_API_KEY (xAI)
    - DEEPSEEK_API_KEY
    - GROK_API_KEY

## WSL Distributions to Export

Export WSL distributions before reformatting:
```powershell
# From Windows PowerShell/Command Prompt
wsl --export Ubuntu C:\Backup\ubuntu.tar
wsl --export Ubuntu-22.04 C:\Backup\ubuntu-22.04.tar
# List all distributions first with: wsl --list --verbose
```

## Application Data Locations

### Important Config Directories (Windows AppData)
- `/mnt/c/users/casey/AppData/Roaming/Claude/` - Claude AI settings
- `/mnt/c/users/casey/AppData/Roaming/Cursor/` - Cursor Editor settings
- `/mnt/c/users/casey/AppData/Roaming/GitHub Desktop/` - GitHub Desktop repos
- `/mnt/c/users/casey/AppData/Roaming/GitHub CLI/` - GitHub CLI config
- `/mnt/c/users/casey/AppData/Roaming/npm/` - Global npm packages
- `/mnt/c/users/casey/AppData/Roaming/Docker Desktop/` - Docker Desktop settings

### Important Config Directories (Home)
- `~/.config/gh/` - GitHub CLI
- `~/.config/git/` - Git config
- `~/.expo/` - Expo/React Native
- `~/.amplify/` - AWS Amplify
- `~/.cloudflared/` - Cloudflare tunnel
- `~/.claude/` - Claude Code settings
- `~/.claude-profiles/` - Claude profiles

## Docker Containers

Currently running containers:
- `ec2mud_qdrant` - Qdrant vector database (v1.7.4) on ports 6333-6334
- `openhands-runtime-*` - OpenHands AI runtime containers (2 stopped)

Action: Export any necessary container data volumes before reformatting

## GitHub Repositories

Total GitHub repositories: 83 repos under account `SuperInstance`

Use `gh repo list --limit 200` to get full list, or backup from:
https://github.com/SuperInstance?tab=repositories

## Project Directories with Git (75 local repos)

All projects in `/mnt/c/users/casey/` with `.git` folder should be cloned from GitHub after reformat.

Key projects include:
- ActiveLog-TechnicalRepo
- agent-grid
- api-gateway
- claudeSuperInstance
- cost-analysis
- LucidDreamer
- luciddreamer-os
- MakerLog
- Murmur
- PersonalLog
- PlayerLoG.AI
- podcastz
- smartCRDT
- SuperInstanceCommunity
- SuperInstanceExamples
- SwarmOrchestration
- workflow-engineer
- (and 60+ others)

## Backup Checklist

- [ ] Copy SSH keys to external storage
- [ ] Copy GPG keys to external storage (if exists)
- [ ] Export VS Code settings
- [ ] Export Cursor Editor settings
- [ ] Copy `.gitconfig`
- [ ] Copy `.bashrc` (review and sanitize API keys first)
- [ ] Copy AWS credentials
- [ ] Copy Docker configuration
- [ ] Export WSL distributions
- [ ] Document GitHub repositories list
- [ ] Backup npm global packages list
- [ ] Backup pip packages list
- [ ] Backup Cargo packages (if any installed globally)
- [ ] Copy Claude Code settings
- [ ] Copy Expo/Amplify configs
