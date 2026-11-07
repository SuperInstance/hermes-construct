# Tools to Reinstall - Post-Reformat Reference

> Created: 2026-01-09
> Purpose: Complete list of installed tools for reinstallation after reformat

## Global npm Packages

```bash
# Install all global npm packages
npm install -g @musistudio/claude-code-router@1.0.64
npm install -g @tauri-apps/cli@2.9.6
npm install -g markdown-pdf@11.0.0
npm install -g md-to-pdf@5.2.5
npm install -g npm@11.7.0
npm install -g pnpm@10.26.2
npm install -g typescript@5.9.3
npm install -g vitest@4.0.16
```

### Quick Install Command
```bash
npm install -g @musistudio/claude-code-router @tauri-apps/cli markdown-pdf md-to-pdf npm pnpm typescript vitest
```

## VS Code Extensions

Run `code --list-extensions` after installing VS Code to get current list.
Install extensions with:
```powershell
code --install-extension <extension-id>
```

Common extensions to reinstall (verify with full list):
- Python extensions
- Git extensions
- Theme extensions
- LSP extensions

## Python Packages (pip)

### Full pip freeze output
Save this to `requirements.txt`:
```text
accelerate==1.11.0
aiohappyeyeballs==2.6.1
aiohttp==3.13.2
aiosignal==1.4.0
annotated-types==0.7.0
anyio==4.12.0
attrs==23.2.0
Automat==22.10.0
Babel==2.10.3
bcrypt==3.2.2
bidict==0.23.1
black==25.12.0
blinker==1.9.0
certifi==2023.11.17
chardet==5.2.0
click==8.1.6
cloud-init==25.2
colorama==0.4.6
command-not-found==0.3
configobj==5.0.8
constantly==23.10.4
controlnet_aux==0.0.10
coverage==7.13.1
cryptography==41.0.7
dbus-python==1.3.2
diffusers==0.35.2
distro==1.9.0
distro-info==1.7+build1
docker==5.0.3
docker-compose==1.29.2
dockerpty==0.4.1
docopt==0.6.2
einops==0.8.1
filelock==3.19.1
Flask==3.1.2
Flask-SocketIO==5.5.1
frozenlist==1.8.0
fsspec==2025.9.0
grpcio==1.76.0
grpcio-tools==1.76.0
h11==0.16.0
h2==4.3.0
hf-xet==1.2.0
hpack==4.1.0
httpcore==1.0.9
httplib2==0.20.4
httpx==0.28.1
huggingface-hub==0.36.0
hyperframe==6.1.0
hyperlink==21.0.0
idna==3.6
imageio==2.37.0
importlib_metadata==8.7.0
incremental==22.10.0
iniconfig==2.3.0
itsdangerous==2.2.0
Jinja2==3.1.6
joblib==1.5.3
jsonpatch==1.32
jsonpointer==2.0
jsonschema==4.10.3
launchpadlib==1.11.0
lazr.restfulclient==0.14.6
lazr.uri==1.0.6
lazy_loader==0.4
librt==0.7.7
markdown-it-py==3.0.0
MarkupSafe==2.1.5
maturin==1.9.6
mdurl==0.1.2
mpmath==1.3.0
msgpack==1.1.2
multidict==6.7.0
mypy==1.19.1
mypy_extensions==1.1.0
netifaces==0.11.0
networkx==3.5
numpy==2.2.6
nvidia-cublas-cu12==12.1.3.1
nvidia-cuda-cupti-cu12==12.1.105
nvidia-cuda-nvrtc-cu12==12.1.105
nvidia-cuda-runtime-cu12==12.1.3.1
nvidia-cudnn-cu12==9.1.0.70
nvidia-cufft-cu12==11.0.2.54
nvidia-cufile-cu12==1.13.1.3
nvidia-curand-cu12==10.3.2.106
nvidia-cusolver-cu12==11.4.5.107
nvidia-cusparse-cu12==12.1.0.106
nvidia-cusparselt-cu12==0.7.1
nvidia-nccl-cu12==2.21.5
nvidia-nvjitlink-cu12==12.9.86
nvidia-nvtx-cu12==12.1.105
oauthlib==3.2.2
opencv-python-headless==4.12.0.88
packaging==25.0
pathspec==1.0.3
peft==0.17.1
pillow==11.3.0
pip==25.3
platformdirs==4.5.1
pluggy==1.6.0
portalocker==2.10.1
propcache==0.4.1
protobuf==6.33.2
psutil==7.1.3
pyasn1==0.4.8
pyasn1-modules==0.2.8
pycurl==7.45.3
pydantic==2.12.5
pydantic_core==2.41.5
Pygments==2.17.2
PyGObject==3.48.2
PyHamcrest==2.1.0
PyJWT==2.7.0
pyOpenSSL==23.2.0
pyparsing==3.1.1
pyrsistent==0.20.0
pyserial==3.5
pytest==9.0.2
pytest-asyncio==1.3.0
pytest-cov==7.0.0
python-apt==2.7.7+ubuntu5.1
python-dotenv==1.0.1
python-engineio==4.12.3
python-socketio==5.14.3
pytokens==0.3.0
pytz==2024.1
PyYAML==6.0.1
qdrant-client==1.7.3
regex==2025.10.23
requests==2.31.0
rich==13.7.1
safetensors==0.6.2
scikit-image==0.25.2
scikit-learn==1.8.0
scipy==1.16.3
sentence-transformers==5.2.0
service-identity==24.1.0
setuptools==70.2.0
simple-websocket==1.1.0
six==1.16.0
sympy==1.13.1
systemd-python==235
texttable==1.6.7
threadpoolctl==3.6.0
tifffile==2025.10.16
timm==1.0.21
tokenizers==0.22.1
torch==2.5.1+cu121
torchaudio==2.5.1+cu121
torchvision==0.20.1+cu121
tqdm==4.67.1
transformers==4.57.1
triton==3.1.0
Twisted==24.3.0
typing_extensions==4.15.0
typing-inspection==0.4.2
ubuntu-pro-client==8001
unattended-upgrades==0.1
urllib3==2.0.7
wadllib==1.3.6
websocket-client==1.7.0
websockets==15.0.1
Werkzeug==3.1.3
wheel==0.42.0
wsproto==1.2.0
yarl==1.22.0
zipp==3.23.0
zope.interface==6.1
```

### Custom Editable Packages
```bash
# Install custom packages
pip install git+ssh://git@github.com/SuperInstance/conversation-toolkit.git@ad2c672f88ca63239fe00dda7d3eb5ba023c91cd
pip install git+https://github.com/SuperInstance/embedding-utils.git@e77fba24f6aaf51a1cafaba71b734a9f22b11f72
```

### Quick Install Command
```bash
# Install PyTorch with CUDA support
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121

# Install ML/Data Science packages
pip install numpy scipy scikit-learn pandas pillow

# Install AI/ML packages
pip install transformers diffusers sentence-transformers peft accelerate

# Install web frameworks
pip install Flask Flask-SocketIO

# Install development tools
pip install pytest pytest-cov black mypy

# Install Docker SDK
pip install docker docker-compose
```

## Core Development Tools

### Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Current version: 1.92.0
```

### Go
```bash
# Install Go 1.x
# Download from https://go.dev/dl/
```

### Node.js
```bash
# Install Node.js via nvm or from nodejs.org
# npm version: 11.7.0
```

### Python
```bash
# Python 3.12 installed at: c:\Users\casey\AppData\Local\Programs\Python\Python312\python.exe
```

## Windows Applications

From AppData, these applications were installed:
- VS Code / VS Code Insiders
- Cursor Editor
- Docker Desktop
- GitHub Desktop
- GitHub CLI
- Claude
- ComfyUI
- Adobe products
- Blackmagic Design (DaVinci Resolve)
- NVIDIA software
- Termius (SSH client)
- balenaEtcher
- qBittorrent
- AWS Amplify CLI
- Expo CLI

## CLI Tools

```bash
# GitHub CLI
winget install GitHub.cli

# Docker CLI (comes with Docker Desktop)

# AWS CLI
wingest install Amazon.AWSCLI

# Amplify CLI
npm install -g @aws-amplify/cli

# Expo CLI
npm install -g expo-cli

# Claude CLI (comes with Claude)
```

## WSL

```powershell
# Enable WSL
wsl --install

# Import backed up distributions
wsl --import Ubuntu C:\WSL\Ubuntu C:\Backup\ubuntu.tar
```

## Environment Variables to Restore

Add to `.bashrc`:
```bash
export PATH="$HOME/.local/bin:$PATH"
export PATH=~/.npm-global/bin:$PATH
export GOROOT=/usr/local/go
export GOPATH=$HOME/go
export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
```

## Notes

1. Python packages include NVIDIA CUDA libraries (PyTorch with CUDA 12.1 support)
2. Some packages are Ubuntu-specific (cloud-init, command-not-found, etc.)
3. Editable packages need to be reinstalled with git+ssh:// URLs
