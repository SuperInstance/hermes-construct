# Projects to Migrate to GitHub

**Generated:** 2026-01-09
**Agent:** Project Assessment Agent (Agent 1)
**Status:** Complete - Ready for Repo Creation Agent

---

## Executive Summary

| Priority | Count | Need GitHub Repo | Notes |
|----------|-------|------------------|-------|
| **CRITICAL** | 7 | 7 | Active development, uncommitted work |
| **HIGH** | 25 | 25 | Good projects, have code, need backup |
| **MEDIUM** | 10 | 10 | Organized but inactive |
| **LOW** | 8 | 8 | Early stage, minimal content |
| **TOTAL** | **50** | **50** | Projects needing migration |

---

## CRITICAL Priority (Immediate Action Required)

These projects have **uncommitted git changes** and/or represent active development work:

| Project | Local Path | Has Git | Has README | Notes | Uncommitted Changes |
|---------|------------|---------|------------|-------|---------------------|
| PersonalLog | /mnt/c/users/casey/PersonalLog | YES | YES | Near production, active development | Modified files in packages/toolguardian, new packages |
| ActiveLog-TechnicalRepo | /mnt/c/users/casey/ActiveLog-TechnicalRepo | YES | YES | Technical documentation repo | Modified docs, push scripts |
| activelog-claude | /mnt/c/users/casey/activelog-claude | YES | YES | Active development | Modified docs, new scripts |
| claudeSuperInstance | /mnt/c/users/casey/claudeSuperInstance | YES | YES | Container with sub-projects | Modified CLAUDE.md, new research folders |
| eveng1_python_sdk | /mnt/c/users/casey/eveng1_python_sdk | YES | YES | EVE-NG Python SDK | Many modified files, new test files |
| perception-jepa | /mnt/c/users/casey/perception-jepa | YES | NO | JEPA perception training | Modified Go files, new training code |
| LucidDreamer | /mnt/c/users/casey/LucidDreamer | YES | NO | Cognitive engine | Many new untracked docs |
| distributed-lock | /mnt/c/users/casey/distributed-lock | YES | YES | Rust distributed lock | All files untracked (new repo) |
| smartCRDT | /mnt/c/users/casey/smartCRDT | YES | NO | SmartCRDT monorepo | Many new untracked files |

**Action Required:** Run `git status` and commit changes before migration for these projects.

---

## HIGH Priority (Value Projects)

These projects have substantial code and should be preserved:

| Project | Local Path | Has Git | Has README | Notes |
|---------|------------|---------|------------|-------|
| activelog-backend | /mnt/c/users/casey/activelog-backend | NO | YES | 34 files, active backend |
| api-gateway | /mnt/c/users/casey/api-gateway | YES | YES | Infrastructure service |
| audio-pipeline | /mnt/c/users/casey/audio-pipeline | YES | YES | Audio processing |
| bandit-learner | /mnt/c/users/casey/bandit-learner | YES | YES | ML project |
| cache-layer | /mnt/c/users/casey/cache-layer | YES | YES | Caching service |
| cache-layer-optimizer | /mnt/c/users/casey/cache-layer-optimizer | YES | YES | Cache optimization |
| circuit-breaker | /mnt/c/users/casey/circuit-breaker | YES | YES | Circuit breaker pattern |
| cluster-orchestrator | /mnt/c/users/casey/cluster-orchestrator | YES | YES | Cluster orchestration |
| deployment-automator | /mnt/c/users/casey/deployment-automator | YES | YES | Deployment automation |
| distributed-tracing | /mnt/c/users/casey/distributed-tracing | YES | YES | Distributed tracing |
| embeddings-engine | /mnt/c/users/casey/embeddings-engine | YES | YES | Embedding generation |
| frozen-model-rl | /mnt/c/users/casey/frozen-model-rl | YES | YES | Reinforcement learning |
| gpu-accelerator | /mnt/c/users/casey/gpu-accelerator | YES | YES | GPU acceleration |
| inference-optimizer | /mnt/c/users/casey/inference-optimizer | YES | YES | AI inference optimization |
| log-aggregator | /mnt/c/users/casey/log-aggregator | YES | YES | Log aggregation |
| monitoring-system | /mnt/c/users/casey/monitoring-system | YES | YES | Monitoring infrastructure |
| protocol-adapters | /mnt/c/users/casey/protocol-adapters | YES | YES | Protocol adapters |
| rag-indexer | /mnt/c/users/casey/rag-indexer | YES | YES | RAG indexing |
| rate-limiter | /mnt/c/users/casey/rate-limiter | YES | YES | Rate limiting |
| realtime-core | /mnt/c/users/casey/realtime-core | YES | YES | Realtime infrastructure |
| semantic-store | /mnt/c/users/casey/semantic-store | YES | YES | Semantic storage |
| streaming-response-handler | /mnt/c/users/casey/streaming-response-handler | YES | YES | Streaming responses | Modified files |
| task-queue | /mnt/c/users/casey/task-queue | YES | YES | Task queue |
| timeseries-db | /mnt/c/users/casey/timeseries-db | YES | YES | Time series database |
| vector-navigator | /mnt/c/users/casey/vector-navigator | YES | YES | Vector navigation |
| webrtc-stream | /mnt/c/users/casey/webrtc-stream | YES | YES | WebRTC streaming |

---

## MEDIUM Priority (Organized but Inactive)

| Project | Local Path | Has Git | Has README | Notes |
|---------|------------|---------|------------|-------|
| agent-grid | /mnt/c/users/casey/agent-grid | YES | YES | Agent grid system |
| SwarmOrchestration | /mnt/c/users/casey/SwarmOrchestration | YES | YES | Has remote, untracked files |
| equilibrium-tokens | /mnt/c/users/casey/equilibrium-tokens | YES | YES | Token economics | Modified files |
| websocket-fabric | /mnt/c/users/casey/websocket-fabric | YES | NO | Has remote, but build artifacts uncommitted |

---

## LOW Priority (Early Stage)

| Project | Local Path | Has Git | Has README | Notes |
|---------|------------|---------|------------|-------|
| autocoder | /mnt/c/users/casey/autocoder | YES | YES | Has remote |
| api-doc-generator | /mnt/c/users/casey/api-doc-generator | YES | YES | Has remote |
| businesslog-app | /mnt/c/users/casey/businesslog-app | YES | YES | Has remote |
| caching-service | /mnt/c/users/casey/caching-service | YES | YES | Has remote |
| config-manager | /mnt/c/users/casey/config-manager | YES | YES | Has remote |
| health | /mnt/c/users/casey/health | YES | YES | Has remote |
| luciddreamer-os | /mnt/c/users/casey/luciddreamer-os | YES | YES | Has remote |
| multibot | /mnt/c/users/casey/multibot | YES | YES | Has remote |
| project1 | /mnt/c/users/casey/project1 | YES | YES | Has remote |
| secret-manager | /mnt/c/users/casey/secret-manager | YES | YES | Has remote |
| workflow-engineer | /mnt/c/users/casey/workflow-engineer | YES | YES | Has remote |
| autogen | /mnt/c/users/casey/autogen | YES | YES | Has remote |

---

## Already on GitHub (No Migration Needed)

These projects already have GitHub remotes configured:

| Project | Local Path | GitHub URL |
|---------|------------|------------|
| ActiveLog-TechnicalRepo | /mnt/c/users/casey/ActiveLog-TechnicalRepo | git@github.com:ctothed/SuperInstance.git |
| LucidDreamer | /mnt/c/users/casey/LucidDreamer | https://github.com/SuperInstance/CognitiveEngine.git |
| MakerLog | /mnt/c/users/casey/MakerLog | https://github.com/SuperInstance/MakerLog.git |
| Murmur | /mnt/c/users/casey/Murmur | https://github.com/SuperInstance/Murmur.git |
| PersonalLog | /mnt/c/users/casey/PersonalLog | https://github.com/SuperInstance/PersonalLog.git |
| SuperInstanceCommunity | /mnt/c/users/casey/SuperInstanceCommunity | https://github.com/SuperInstance/SuperInstanceCommunity.git |
| SuperInstanceExamples | /mnt/c/users/casey/SuperInstanceExamples | https://github.com/SuperInstance/SuperInstanceExamples.git |
| SwarmOrchestration | /mnt/c/users/casey/SwarmOrchestration | https://github.com/SuperInstance/LucidDreamer.git |
| activelog-claude | /mnt/c/users/casey/activelog-claude | https://github.com/ctothed/activelog-claude.git |
| ai-token-counter | /mnt/c/users/casey/ai-token-counter | git@github.com:SuperInstance/ai-token-counter.git |
| api-doc-generator | /mnt/c/users/casey/api-doc-generator | git@github.com:SuperInstance/api-doc-generator.git |
| autocoder | /mnt/c/users/casey/autocoder | git@github.com:SuperInstance/autocoder.git |
| autogen | /mnt/c/users/casey/autogen | git@github.com:SuperInstance/autogen.git |
| businesslog-app | /mnt/c/users/casey/businesslog-app | git@github.com:SuperInstance/businesslog-app.git |
| caching-service | /mnt/c/users/casey/caching-service | git@github.com:SuperInstance/caching-service.git |
| character-agent-integration | /mnt/c/users/casey/character-agent-integration | git@github.com:SuperInstance/character-agent-integration.git |
| character-library | /mnt/c/users/casey/character-library | git@github.com:SuperInstance/character-library.git |
| character-skill-trees | /mnt/c/users/casey/character-skill-trees | git@github.com:SuperInstance/character-skill-trees.git |
| claudeSuperInstance | /mnt/c/users/casey/claudeSuperInstance | https://github.com/SuperInstance/quicunnel.git |
| config-manager | /mnt/c/users/casey/config-manager | git@github.com:SuperInstance/config-manager.git |
| conversation-toolkit | /mnt/c/users/casey/conversation-toolkit | git@github.com:SuperInstance/conversation-toolkit.git |
| cost-analysis | /mnt/c/users/casey/cost-analysis | https://github.com/SuperInstance/cost-analysis.git |
| ec2mud | /mnt/c/users/casey/ec2mud | https://github.com/SuperInstance/ec2mud.git |
| embedding-utils | /mnt/c/users/casey/embedding-utils | https://github.com/SuperInstance/embedding-utils.git |
| equilibrium-tokens | /mnt/c/users/casey/equilibrium-tokens | https://github.com/SuperInstance/equilibrium-tokens.git |
| eveng1_python_sdk | /mnt/c/users/casey/eveng1_python_sdk | https://github.com/binarythinktank/eveng1_python_sdk.git |
| health-monitoring-service | /mnt/c/users/casey/health-monitoring-service | git@github.com:SuperInstance/health-monitoring-service.git |
| health | /mnt/c/users/casey/health | git@github.com:SuperInstance/health.git |
| hierarchical-memory | /mnt/c/users/casey/hierarchical-memory | git@github.com:SuperInstance/hierarchical-memory.git |
| llm-cost-calculator | /mnt/c/users/casey/llm-cost-calculator | git@github.com:SuperInstance/llm-cost-calculator.git |
| local-model-manager | /mnt/c/users/casey/local-model-manager | git@github.com:SuperInstance/local-model-manager.git |
| luciddreamer-os | /mnt/c/users/casey/luciddreamer-os | git@github.com:SuperInstance/luciddreamer-os.git |
| memory-visualization | /mnt/c/users/casey/memory-visualization | git@github.com:SuperInstance/agent-grid.git |
| model-switching-strategy | /mnt/c/users/casey/model-switching-strategy | git@github.com:SuperInstance/model-switching-strategy.git |
| monitoring-dashboard | /mnt/c/users/casey/monitoring-dashboard | https://github.com/SuperInstance/monitoring-dashboard.git |
| multi-provider-router | /mnt/c/users/casey/multi-provider-router | https://github.com/SuperInstance/multi-provider-router.git |
| multibot | /mnt/c/users/casey/multibot | git@github.com:SuperInstance/multibot.git |
| perception-jepa | /mnt/c/users/casey/perception-jepa | https://github.com/SuperInstance/project-JEPA.git |
| project1 | /mnt/c/users/casey/project1 | git@github.com:SuperInstance/project1.git |
| provider-abstraction-layer | /mnt/c/users/casey/provider-abstraction-layer | git@github.com:SuperInstance/provider-abstraction-layer.git |
| rate-limiting-service | /mnt/c/users/casey/rate-limiting-service | git@github.com:SuperInstance/rate-limiting-service.git |
| secret-manager | /mnt/c/users/casey/secret-manager | git@github.com:SuperInstance/secret-manager.git |
| smartCRDT | /mnt/c/users/casey/smartCRDT | https://github.com/SuperInstance/SmartCRDT.git |
| streaming-response-handler | /mnt/c/users/casey/streaming-response-handler | git@github.com:SuperInstance/streaming-response-handler.git |
| superinstance-docs | /mnt/c/users/casey/superinstance-docs | https://github.com/SuperInstance/SuperInstanceDocs.git |
| superinstance-ui | /mnt/c/users/casey/superinstance-ui | https://github.com/SuperInstance/UI.git |
| websocket-fabric | /mnt/c/users/casey/websocket-fabric | git@github.com:SuperInstance/websocket-fabric.git |
| workflow-engineer | /mnt/c/users/casey/workflow-engineer | git@github.com:SuperInstance/workflow-engineer.git |

---

## Migration Order Recommendation

1. **First:** CRITICAL projects (commit uncommitted changes first)
2. **Second:** HIGH priority projects (initialize git where needed, create repos)
3. **Third:** MEDIUM priority projects
4. **Last:** LOW priority projects

---

## Notes for Repo Creation Agent

- Use `SuperInstance` organization for most projects
- Private repos by default
- Create appropriate `.gitignore` based on language detected
- Initialize with README if missing
- Suggested naming: use kebab-case matching folder names
