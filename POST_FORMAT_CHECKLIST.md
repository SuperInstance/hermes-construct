# Post-Format Checklist

> Created: 2026-01-09
> Purpose: Step-by-step verification after computer reformat

## Phase 1: Base System Setup

### Windows Configuration
- [ ] Install Windows updates
- [ ] Configure display settings
- [ ] Set power management preferences
- [ ] Configure Windows security/firewall
- [ ] Install antivirus if needed

### WSL Installation
- [ ] Enable WSL feature: `wsl --install`
- [ ] Install Ubuntu distribution
- [ ] Import WSL backup (if available): `wsl --import Ubuntu C:\WSL\Ubuntu C:\Backup\ubuntu.tar`
- [ ] Verify WSL works: Open Ubuntu terminal

## Phase 2: Development Environment

### Git & GitHub Setup
- [ ] Install Git for Windows
- [ ] Configure Git:
  ```bash
  git config --global user.name "Claude (SuperInstance)"
  git config --global user.email "claude@superinstance.com"
  git config --global credential.helper store
  ```
- [ ] Restore SSH keys: Copy `id_ed25519` and `id_ed25519.pub` to `~/.ssh/`
- [ ] Set correct permissions: `chmod 600 ~/.ssh/id_ed25519`
- [ ] Install GitHub CLI: `winget install GitHub.cli`
- [ ] Authenticate GitHub CLI: `gh auth login`
- [ ] Verify SSH connection: `ssh -T git@github.com`

### Node.js & npm
- [ ] Install Node.js (LTS version)
- [ ] Verify installation: `node --version` and `npm --version`
- [ ] Configure npm global path: `npm config set prefix ~/.npm-global`
- [ ] Install global npm packages:
  ```bash
  npm install -g @musistudio/claude-code-router @tauri-apps/cli markdown-pdf md-to-pdf npm pnpm typescript vitest
  ```
- [ ] Verify global packages: `npm list -g --depth=0`

### Python Environment
- [ ] Install Python 3.12 from python.org
- [ ] Add Python to PATH
- [ ] Verify installation: `python --version`
- [ ] Install pip packages from requirements.txt:
  ```bash
  pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
  pip install -r requirements.txt
  ```
- [ ] Install editable packages:
  ```bash
  pip install git+ssh://git@github.com/SuperInstance/conversation-toolkit.git
  pip install git+https://github.com/SuperInstance/embedding-utils.git
  ```

### Rust & Cargo
- [ ] Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Verify installation: `rustc --version` and `cargo --version`
- [ ] Restore `.cargo/` directory if available

### Go
- [ ] Install Go from go.dev
- [ ] Set environment variables in `.bashrc`:
  ```bash
  export GOROOT=/usr/local/go
  export GOPATH=$HOME/go
  export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
  ```

## Phase 3: Applications

### Code Editors
- [ ] Install VS Code: Download from code.visualstudio.com
- [ ] Install VS Code extensions
- [ ] Restore VS Code settings
- [ ] Install Cursor (if used)

### Docker
- [ ] Install Docker Desktop for Windows
- [ ] Restore Docker configuration from backup
- [ ] Verify installation: `docker --version`
- [ ] Verify containers: `docker ps -a`

### Communication & Tools
- [ ] Install GitHub Desktop
- [ ] Install Termius (or preferred SSH client)
- [ ] Install Claude desktop app

## Phase 4: Configuration Restoration

### SSH Keys
- [ ] Copy `id_ed25519` to `~/.ssh/`
- [ ] Copy `id_ed25519.pub` to `~/.ssh/`
- [ ] Set permissions: `chmod 600 ~/.ssh/id_ed25519`
- [ ] Test SSH: `ssh -T git@github.com`

### AWS Credentials
- [ ] Copy `.aws/config` to `~/.aws/`
- [ ] Copy `.aws/credentials` to `~/.aws/`

### Bash Configuration
- [ ] Copy `.bashrc` from backup
- [ ] IMPORTANT: Update API keys with new values
- [ ] Source bashrc: `source ~/.bashrc`
- [ ] Verify PATH includes all necessary directories

### Git Configuration
- [ ] Copy `.gitconfig` or reconfigure
- [ ] Verify: `git config --global --list`

## Phase 5: Repository Restoration

### Clone All GitHub Repositories
```bash
# Get list of all repos
gh repo list --limit 200 > repos.txt

# Clone all repos (requires script)
while IFS=$'\t' read -r name _ _; do
    gh repo clone "$name"
done < repos.txt
```

Or use `gh repo clone` for specific repos:
```bash
gh repo clone SuperInstance/activelog-claude
gh repo clone SuperInstance/agent-grid
gh repo clone SuperInstance/api-gateway
# ... repeat for all needed repos
```

### Verify Repositories
- [ ] Count repositories: Should have ~75+ projects
- [ ] Test `git status` in a few repos
- [ ] Verify `git pull` works

### Restore Project Dependencies
- [ ] Run `npm install` in Node.js projects
- [ ] Run `pip install -r requirements.txt` in Python projects
- [ ] Run `cargo build` in Rust projects

## Phase 6: Docker & Services

### Restore Qdrant Container
```bash
docker run -d --name ec2mud_qdrant \
  -p 6333-6334:6333-6334 \
  qdrant/qdrant:v1.7.4
```

### Verify Services
- [ ] Check Docker containers running: `docker ps`
- [ ] Test Qdrant connection on port 6333

## Phase 7: Final Verification

### Development Workflow Test
- [ ] Create a test git repo
- [ ] Push to GitHub
- [ ] Clone back
- [ ] Delete test repo

### Build Test
- [ ] Build a simple Node.js project
- [ ] Build a simple Python script
- [ ] Verify all tools work

### Network & SSH
- [ ] SSH to a remote server
- [ ] Git push/pull to GitHub
- [ ] Access all needed services

## Phase 8: Clean Up

### Remove Installation Files
- [ ] Delete downloaded installers
- [ ] Empty recycle bin

### Backup Cleanup
- [ ] Move backup to external storage
- [ ] Verify all important data is safe

## Troubleshooting Notes

### Common Issues
1. **SSH permissions**: Always `chmod 600` for private keys
2. **WSL path issues**: Use `/mnt/c/` to access Windows files from WSL
3. **Docker WSL integration**: Enable in Docker Desktop settings
4. **npm permissions**: May need to configure npm global directory
5. **PATH issues**: Add tools to PATH in `.bashrc`

### Recovery Commands
```bash
# Reset WSL if needed
wsl --shutdown
wsl --unregister Ubuntu
wsl --import Ubuntu C:\WSL\Ubuntu C:\Backup\ubuntu.tar

# Reset Docker
# Use Docker Desktop "Reset to factory defaults" if needed

# Reinstall npm globals
npm list -g --depth=0  # Check what's missing
```

## Completion Checklist

- [ ] All 75+ GitHub repositories cloned
- [ ] All global npm packages installed
- [ ] All Python packages installed
- [ ] SSH keys working
- [ ] Git push/pull working
- [ ] Docker containers running
- [ ] VS Code configured with extensions
- [ ] API keys configured (use new values)
- [ ] Bash aliases and PATH configured
- [ ] Qdrant/vector database running (if needed)
