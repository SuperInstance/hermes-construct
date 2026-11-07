# WSL Ubuntu Disk Itemization (no deletes)
**Total inside WSL: 338 GB used / 618 GB free (virtual)**
**VHDX on disk: 342 GB**
**Hidden VHDX metadata: 4 GB**

## CATEGORY BREAKDOWN (sorted by reclaimable size)

| # | Category | Path | Size | Type | Safe to reclaim? |
|---|----------|------|------|------|------------------|
| 1 | **`backups/github-mirror`** | `/home/eileen/backups/github-mirror/` | **50.0 GB** | 4,476 bare git repos (mostly ~140 KB ea., some 100 MB+) | YES — git remote mirrors can be re-cloned |
| 2 | **`models`** | `/home/eileen/models/` | **48.2 GB** | SDXL/SD checkpoints, venv, qwen3-4b | PARTIAL — depending on what's used |
| 3 | **`ollama`** | `/home/eileen/.ollama/` | **41.9 GB** | 20 LLM models (Liquid, deepseek-r1, mistral, llama3.2, granite, llava, qwen2.5/coder/vl, phi3, phi4-mini, qwen3, gemma3, bge-m3, etc.) | YES if unused — pull back via ollama pull |
| 4 | **`huggingface cache`** | `/home/eileen/.cache/huggingface/` | **13.5 GB** | 14 GB in `hub/models--stabilityai--sdxl-turbo` — duplicate of `/home/eileen/models/sdxl-turbo/` (6.5 GB) | YES — duplicate of #5; can re-download |
| 5 | **`sdxl-turbo dup`** | `/home/eileen/models/sdxl-turbo/` | **6.5 GB** | Same model also in HF cache above | PARTIAL — keep one, drop other |
| 6 | **`pip cache`** | `/home/eileen/.cache/pip/` | **2.9 GB** | HTTP cache for pip installs | YES — `pip cache purge` regenerable |
| 7 | **`npm cache`** | `/home/eileen/.npm/_cacache/` | **5.7 GB** | npm download cache (5.7 GB) | YES — `npm cache clean --force` regenerable |
| 8 | **`/var/log/journal`** | `/var/log/journal/` | **828 MB** | systemd rotated logs | YES — `journalctl --vacuum-size=100M` |
| 9 | **`node-gyp`** | `/home/eileen/.cache/node-gyp/` | **66 MB** | C++ build artifacts for native modules | YES — `npm cache clean` regenerable |
| 10 | **`whisper cache`** | `/home/eileen/.cache/whisper/` | **672 MB** | Whisper base/small/tiny.pt models | YES if unused — `pip cache purge` |
| 11 | **`ms-playwright`** | `/home/eileen/.cache/ms-playwright/` | **656 MB** | Playwright browser binaries | YES if unused — `playwright install` regenerable |
| 12 | **`zig cache`** | `/home/eileen/.cache/zig/` | **40 MB** | Zig compiler cache | YES — auto-regenerated |
| 13 | **`wasm-pack`** | `/home/eileen/.cache/.wasm-pack/` | **42 MB** | wasm-pack Rust/Wasm cache | YES — auto-regenerated |
| 14 | **`local/share/opencode`** | `/home/eileen/.local/share/opencode/` | **1.3 GB** | Opencode DB + cache | PARTIAL — DB may be useful |
| 15 | **`local/share/mamba`** | `/home/eileen/.local/share/mamba/` | **1.7 GB** | conda pkgs | YES if unused |
| 16 | **`local/share/pnpm`** | `/home/eileen/.local/share/pnpm/` | **2.7 GB** | pnpm store | PARTIAL — `pnpm store prune` |
| 17 | **`cargo`** | `/home/eileen/.cargo/` | **2.2 GB** | Cargo registry/src | YES — `cargo clean` regenerable |
| 18 | **`rustup`** | `/home/eileen/.rustup/` | **3.1 GB** | 2 toolchains (1.95.0, 1.97.1) | YES — `rustup toolchain uninstall` for unused |
| 19 | **`ollama nm-venv`** | `/home/eileen/models/nm-venv/` | **4.2 GB** | Python venv for nemotron | YES if unused |
| 20 | **`local/lib/python3.14`** | `/home/eileen/.local/lib/python3.14/` | **8.6 GB** | Python 3.14 stdlib (managed by uv) | PARTIAL — depends on what runs |
| 21 | **`local/lib/ollama`** | `/home/eileen/.local/lib/ollama/` | **5.1 GB** | Ollama binaries | NO — needed for ollama |
| 22 | **`local/share/piper-voices`** | `/home/eileen/.local/share/piper-voices/` | **242 MB** | TTS voices | YES if unused |
| 23 | **`core dumps`** | `/core.*` | **~25 MB** | 5 core dump files | YES — never analyzed |

## CONSUMER BREAKDOWN (no obvious bloat, but documented)

- **`/home/eileen/projects/ai-writings/`** = 8.5 GB total
  - `.git/` = 5.4 GB (large git history — possibly bloated with audio)
  - `music/` = 1.5 GB
  - `radio-theater/`, `fleet-radio/`, `site/`, `podcasts/`, `radio/` = 1.7 GB combined
  - This is **legitimate content** (writing portfolio + audio archives) — not trash
- **`/home/eileen/projects/ACE-Step-1.5/`** = 10.8 GB
  - Code + trained model assets; legitimate ML project
- **`/home/eileen/projects/SuperInstance-papers/`** = 917 MB — paper repo
- **`/home/eileen/projects/OpenRoom/`** = 1.3 GB

## TOP RECLAIM OPPORTUNITIES (sorted by `cleaned easily` × `no data loss`)

1. **`/home/eileen/backups/github-mirror/`** = 50 GB — bare git mirror of 4,476 repos. Most can be deleted; the live projects are in `/home/eileen/projects/`. **Reclaim: 30-50 GB if aggressive**
2. **`/home/eileen/.ollama/`** = 42 GB — 20 models, only need a few. **Reclaim: 25-35 GB** by removing unused
3. **`/home/eileen/.cache/huggingface/hub/models--stabilityai--sdxl-turbo`** = 14 GB — duplicate of `/home/eileen/models/sdxl-turbo/`. **Reclaim: 14 GB**
4. **Python venv `/home/eileen/models/nm-venv/`** = 4.2 GB if nemotron unused
5. **`/home/eileen/.rustup/`** = 3.1 GB — keep one toolchain. **Reclaim: 1.5 GB**
6. **`/home/eileen/.cargo/`** = 2.2 GB. **Reclaim: 1-2 GB** via `cargo clean`
7. **`/home/eileen/.local/share/pnpm/`** = 2.7 GB. **Reclaim: 1-2 GB** via `pnpm store prune`
8. **`/home/eileen/.npm/_cacache/`** = 5.7 GB. **Reclaim: 5 GB** via `npm cache clean`
9. **`/home/eileen/.cache/pip/`** = 2.9 GB. **Reclaim: 2.9 GB** via `pip cache purge`
10. **`/var/log/journal/`** = 828 MB. **Reclaim: 700 MB** via `journalctl --vacuum-size=100M`
11. **`/home/eileen/.cache/whisper/`** = 672 MB. **Reclaim: 672 MB** if unused
12. **`/home/eileen/.cache/ms-playwright/`** = 656 MB. **Reclaim: 656 MB** if unused
13. **`/home/eileen/.local/share/mamba/`** = 1.7 GB. **Reclaim: 1-2 GB** via `conda clean --all`
14. **`/home/eileen/models/checkpoints/`** = 22 GB (mostly SDXL/SD checkpoints including 4.9 GB partial downloads). **Reclaim: 5 GB** by removing partial downloads

## NOT TRASH — KEEP
- `/home/eileen/projects/` (411 project dirs, including ai-writings, ACE-Step, SuperInstance-papers)
- `/home/eileen/models/checkpoints/` (mostly complete model files)
- `/home/eileen/.local/lib/python3.14/` (managed by uv; if Python projects are active, keep)
- `/home/eileen/.local/lib/ollama/` (needed to run ollama)
- `/home/eileen/micromamba-envs/` (1.4 GB — used by active Python envs)
- `/home/eileen/micromamba/` (3.2 GB — needed by micromamba)

## ESTIMATED MAX RECLAIM (no data loss): ~100-130 GB
- Best wins: github-mirror (50 GB) + ollama unused models (30 GB) + HF duplicate (14 GB) + caches (15 GB)

## ESTIMATED CONSERVATIVE RECLAIM (low risk): ~60-80 GB
- HF duplicate (14 GB) + caches (~15 GB) + unused ollama models (~15 GB) + git mirror partial (20-30 GB)

## NEXT STEPS (waiting for your direction)
- Phase 1 (immediate, safe): clean caches (pip, npm, zig, wasm-pack, node-gyp) → ~10 GB freed
- Phase 2 (low risk): remove HF cache duplicate of sdxl-turbo → 14 GB
- Phase 3 (medium): inventory ollama models, identify unused ones → 15-30 GB
- Phase 4 (higher risk): inventory github-mirror, identify dead repos → 30-50 GB
- Phase 5 (depends on usage): rustup, cargo, pnpm cleanup → 5-10 GB
- Phase 6 (verify only): ai-writings/.git cleanup (gc + repack) → possibly 1-3 GB

Nothing has been deleted. Awaiting your call on what to proceed with.