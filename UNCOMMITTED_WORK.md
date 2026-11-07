# Projects with Uncommitted Git Changes

**Generated:** 2026-01-09
**Agent:** Project Assessment Agent (Agent 1)
**Status:** CRITICAL - These projects need commits before migration

---

## Executive Summary

| Project | Uncommitted Files | Status | Priority |
|---------|-------------------|--------|----------|
| PersonalLog | 15+ | Modified + Untracked | CRITICAL |
| eveng1_python_sdk | 40+ | Modified + Untracked | CRITICAL |
| activelog-claude | 6 | Modified + Untracked | CRITICAL |
| ActiveLog-TechnicalRepo | 9 | Modified | HIGH |
| smartCRDT | 20+ | Untracked | HIGH |
| LucidDreamer | 15+ | Untracked docs | HIGH |
| perception-jepa | 4 | Modified + Untracked | HIGH |
| claudeSuperInstance | 6 | Modified + Untracked | MEDIUM |
| distributed-lock | 4 | All untracked | MEDIUM |
| equilibrium-tokens | 3 | Modified | MEDIUM |
| websocket-fabric | 500+ | Build artifacts | LOW |
| streaming-response-handler | 3 | Modified | LOW |
| SwarmOrchestration | 6 | Untracked | LOW |
| circuit-breaker | 1 | Untracked | LOW |

---

## CRITICAL - Immediate Action Required

### PersonalLog
**Path:** `/mnt/c/users/casey/PersonalLog`
**Remote:** https://github.com/SuperInstance/PersonalLog.git

**Modified Files:**
```
M TOOL_ECOSYSTEM_ROADMAP.md
m packages/automatic-type-safe-indexeddb
m packages/hardware-aware-flagging
m packages/hardware-capability-profiler
m packages/privacy-first-analytics
M packages/smartcost/dashboard/index.ts
M packages/toolguardian/README.md
M packages/toolguardian/src/core/ToolGuardian.ts
M packages/toolguardian/src/monitoring/Monitor.ts
M packages/toolguardian/src/retry/RetryManager.ts
M packages/toolguardian/src/types.ts
M packages/toolguardian/src/validation/SchemaValidator.ts
M packages/toolguardian/tsup.config.ts
```

**Untracked Files:**
```
?? .agents/round-4-improvement/
?? .agents/rounds/round-3-briefing.md
?? packages/agentswarm/
?? packages/toolguardian/.gitignore
?? packages/toolguardian/DX.md
?? packages/toolguardian/PERFORMANCE.md
?? packages/toolguardian/docs/
```

**Action:** Commit these changes immediately - this is near-production code.

---

### eveng1_python_sdk
**Path:** /mnt/c/users/casey/eveng1_python_sdk
**Remote:** https://github.com/binarythinktank/eveng1_python_sdk.git

**Modified Files:** (all core files modified)
```
M .gitignore, README.md, __init__.py
M connector/__init__.py, connector/base.py, connector/bluetooth.py
M connector/commands.py, connector/pairing.py
M examples/__init__.py, examples/dashboard.py, examples/even_ai.py
M examples/interactions.py, examples/microphone.py, examples/ppt_teleprompter.py
M examples/send_image.py, examples/send_text.py, examples/simple_connect.py
M requirements.txt, services/__init__.py, services/audio.py
M services/device.py, services/display.py, services/events.py
M services/health.py, services/state.py, services/status.py, services/uart.py
M setup.py, utils/__init__.py, utils/config.py, utils/constants.py, utils/logger.py
```

**Untracked Files:** (extensive new development)
```
?? .claude/, audio_*.raw files, ble_*.py files
?? connect*.py, disconnect.py, force_*.py, gatt_*.py
?? glasses_manager.py, record*.py, rolling_recorder*.py
?? testfile.py, write.py, audio test files
```

**Action:** This appears to be active BLE/audio development. Commit with descriptive message.

---

### activelog-claude
**Path:** /mnt/c/users/casey/activelog-claude
**Remote:** https://github.com/ctothed/activelog-claude.git

**Modified Files:**
```
M docs/main-prompt.txt
M docs/system-prompt.txt
M goals.json
M src/main.py
```

**Untracked Files:**
```
?? activity.log
?? deploy.ps1
?? run.log
?? run.ps1
```

**Action:** Commit prompt and source changes. Logs can be added to .gitignore.

---

## HIGH Priority

### ActiveLog-TechnicalRepo
**Path:** /mnt/c/users/casey/ActiveLog-TechnicalRepo
**Remote:** git@github.com:ctothed/SuperInstance.git

**Modified Files:**
```
M .gitignore
M CONTRIBUTING.md
M INDEX.md
M PROJECTS_OVERVIEW.md
M PROJECT_STRUCTURE.md
M QUICKSTART.md
M README.md
M docs/guides/SETUP.md
```

**Untracked Files:**
```
?? push-to-github.ps1
?? push-to-github.sh
```

**Action:** Documentation updates. Commit and push.

---

### smartCRDT
**Path:** /mnt/c/users/casey/smartCRDT
**Remote:** https://github.com/SuperInstance/SmartCRDT.git

**Untracked Files:**
```
?? .agent-system/
?? .local-archive/
?? ADAPTER_DOCUMENTATION_REPORT.md
?? CARTRIDGE_FORMAT_IMPLEMENTATION_SUMMARY.md
?? CHANGELOG.md
?? CLAUDE.md (and backups)
?? COMPLETE_API_REFERENCE.md
?? EDGE_CASE_DOCUMENTATION_REPORT.md
?? INVARIANT_DOCUMENTATION_REPORT.md
?? JSDOC_DOCUMENTATION_REPORT.md
?? LICENSE
?? PAPER_16_Cartridge_Format_Specification.md
?? PRIVACY_CLASSIFIER_IMPLEMENTATION_REPORT.md
?? RATIONALE_DOCS_REPORT.md
?? SYNC_TASK_SUMMARY.md
?? audit_workspace/
?? build-verification-output.log
?? demo/
?? deps-fix.cjs
?? packages/
?? scripts/
```

**Action:** Major additions including packages and documentation. Needs full review before commit.

---

### LucidDreamer
**Path:** /mnt/c/users/casey/LucidDreamer
**Remote:** https://github.com/SuperInstance/CognitiveEngine.git

**Untracked Documentation Files:**
```
?? AGENT_COMPILATION_SUMMARY.md
?? AGENT_STATUS.md
?? CI_CD_QUICK_REFERENCE.md
?? CI_CD_SETUP_SUMMARY.md
?? CLAUDE.md
?? CLAUDE_TOOL_MAKER.md
?? EXTRACTION_COMPLETE.md
?? FINAL_STATUS.md
?? NIGHTLY_STATUS_REPORT.md
?? PHASE2_CICD_SUMMARY.md
?? PRELIMINARY_TOOL_INVENTORY.md
?? PYPUBLISHING_GUIDE.md
?? ROUND_1_STATUS.md
?? ROUND_2_3_STATUS.md
?? TOOL_EXTRACTION_PROGRESS.md
?? TOOL_INVENTORY.md
?? TOOL_LIBRARY_GUIDE.md
?? TOOL_LIBRARY_MASTER_PLAN.md
?? docs/
?? examples/
?? extracted-tools/
```

**Action:** Documentation and tools added. Commit with message about tool extraction work.

---

### perception-jepa
**Path:** /mnt/c/users/casey/perception-jepa
**Remote:** https://github.com/SuperInstance/project-JEPA.git

**Modified Files:**
```
M internal/sensors/energy.go
M internal/sensors/predictor.go
```

**Untracked Files:**
```
?? SIMPLE_TRAINING_GUIDE.md
?? TRAINING_SUCCESS_REPORT.md
?? cmd/train/
?? train-on-real-gpu.sh
```

**Action:** Training implementation added. Commit these changes.

---

## MEDIUM Priority

### claudeSuperInstance
**Path:** /mnt/c/users/casey/claudeSuperInstance
**Remote:** https://github.com/SuperInstance/quicunnel.git (multipart repo)

**Modified Files:**
```
M CLAUDE.md
m hwscan
```

**Untracked Files:**
```
?? repos/
?? research/progress-rs/
?? research/prompt-rs/
?? research/tui-rs/
```

**Action:** Container project with research subfolders. Commit updates.

---

### distributed-lock
**Path:** /mnt/c/users/casey/distributed-lock

**Untracked Files (All files):**
```
?? .gitignore
?? Cargo.toml
?? README.md
?? src/
```

**Action:** New Rust project - all files untracked. Initialize properly.

---

### equilibrium-tokens
**Path:** /mnt/c/users/casey/equilibrium-tokens
**Remote:** https://github.com/SuperInstance/equilibrium-tokens.git

**Modified Files:**
```
M .claude/completion_status.toml
M .claude/work_log.md
M architecture.md
```

**Action:** Minor changes. Commit and push.

---

## LOW Priority

### websocket-fabric
**Path:** /mnt/c/users/casey/websocket-fabric
**Remote:** git@github.com:SuperInstance/websocket-fabric.git

**Untracked Files:** (500+ build artifacts)
```
?? target/ (entire Rust build directory with dependencies)
```

**Action:** Add `target/` to .gitignore if not already. Do not commit build artifacts.

---

### streaming-response-handler
**Path:** /mnt/c/users/casey/streaming-response-handler
**Remote:** git@github.com:SuperInstance/streaming-response-handler.git

**Modified Files:**
```
M streaming_handler/__init__.py
M tests/test_handler.py
```

**Untracked Files:**
```
?? streaming_handler/retry.py
```

**Action:** Minor changes. Add new retry module and commit.

---

### SwarmOrchestration
**Path:** /mnt/c/users/casey/SwarmOrchestration
**Remote:** https://github.com/SuperInstance/LucidDreamer.git

**Untracked Files:**
```
?? .env.example
?? .gitignore
?? CONTRIBUTING.md
?? LICENSE
?? docker-compose.yml
?? package.json
?? src/
?? tsconfig.json
?? tsup.config.ts
```

**Action:** New project files added. This appears to be reorganized as separate project.

---

### circuit-breaker
**Path:** /mnt/c/users/casey/circuit-breaker

**Untracked Files:**
```
?? pkg/circuitbreaker/.claude/
```

**Action:** Claude Code workspace folder. Add to .gitignore.

---

## Recommended Commands

For each project, run:

```bash
# Review changes
cd /path/to/project
git status
git diff

# Add appropriate files
git add .

# Commit with descriptive message
git commit -m "WIP: [describe changes]"

# For projects with build artifacts, add to .gitignore first:
echo "target/" >> .gitignore
echo "*.pyc" >> .gitignore
```

---

## After Committing

1. Verify commits were created: `git log -1`
2. Check for anything remaining: `git status`
3. Push to remote if exists: `git push`
4. If no remote, proceed to migration step
