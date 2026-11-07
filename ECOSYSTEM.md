# SuperInstance Ecosystem Documentation

Complete documentation index for the SuperInstance modular toolkit ecosystem.

---

## Getting Started

### New to SuperInstance?

1. Read the [Onboarding Guide](./ONBOARDING.md)
2. Explore the [Package Catalog](#package-catalog)
3. Try the [Examples](https://github.com/SuperInstance/SuperInstanceExamples)
4. Join the [Community](https://github.com/SuperInstance/SuperInstanceCommunity)

### Quick Install

```bash
# Install any package
pnpm add @superinstance/package-name

# Or use with npm
npm install @superinstance/package-name
```

---

## Package Catalog

### Foundation Primitives

Core utilities that other packages depend on.

| Package | Version | Description | Links |
|---------|---------|-------------|-------|
| `@superinstance/async` | 1.0.0 | Async utilities: retry, timeout, throttle, batch, queue | [README](packages/async/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/async) |
| `@superinstance/validate` | 1.0.0 | Schema validation with Zod | [README](packages/validate/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/validate) |
| `@superinstance/cache` | 1.0.0 | High-performance caching with LRU and TTL | [README](packages/cache/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/cache) |
| `@superinstance/events` | 1.0.0 | Type-safe event emitter primitives | [README](packages/events/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/events) |
| `@superinstance/config` | 1.0.0 | Configuration management with Zod validation | [README](packages/config/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/config) |
| `@superinstance/storage` | 1.0.0 | Unified storage abstraction (memory, IndexedDB, SQLite, PostgreSQL) | [README](packages/storage/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/storage) |
| `@superinstance/provider-base` | 1.0.0 | Base classes for LLM/TTS/ASR providers | [README](packages/provider-base/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/provider-base) |
| `@superinstance/logger` | 1.0.0 | Production-safe structured logging | [README](packages/logger/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/logger) |

### Feature Packages

Higher-level modules built on foundation primitives.

| Package | Version | Description | Links |
|---------|---------|-------------|-------|
| `@superinstance/chat` | 1.0.0 | Type-safe LLM chat abstraction (OpenAI, Anthropic, Z-AI) | [README](packages/chat/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/chat) |
| `@superinstance/tts` | 1.0.0 | Text-to-speech synthesis (OpenAI, ElevenLabs, Z-AI) | [README](packages/tts/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/tts) |
| `@superinstance/asr` | 1.0.0 | Speech recognition and transcription | [README](packages/asr/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/asr) |
| `@superinstance/memory` | 1.0.0 | Four-layer memory system with semantic search | [README](packages/memory/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/memory) |
| `@superinstance/dreamer` | 1.0.0 | Cognitive intelligence with 5-level abstraction | [README](packages/dreamer/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/dreamer) |
| `@superinstance/rag` | 1.0.0 | Retrieval-augmented generation toolkit | [README](packages/rag/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/rag) |
| `@superinstance/knowledge-tensor` | 1.0.0 | Multi-dimensional knowledge operations | [README](packages/knowledge-tensor/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/knowledge-tensor) |
| `@superinstance/abstraction` | 1.0.0 | Pattern detection and insight generation | [README](packages/abstraction/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/abstraction) |
| `@superinstance/content-analyzer` | 1.0.0 | Multi-modal content analysis (text, image, URL) | [README](packages/content-analyzer/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/content-analyzer) |
| `@superinstance/personalization` | 1.0.0 | User preference learning and adaptation | [README](packages/personalization/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/personalization) |
| `@superinstance/event-runtime` | 1.0.0 | Real-time WebSocket event infrastructure | [README](packages/event-runtime/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/event-runtime) |
| `@superinstance/audio-workspace` | 1.0.0 | Audio editing UI with timeline and waveform | [README](packages/audio-workspace/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/audio-workspace) |
| `@superinstance/agent-factory` | 1.0.0 | Create specialized AI agents through conversation | [README](packages/agent-factory/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/agent-factory) |
| `@superinstance/script-editor` | 1.0.0 | Monaco-based script editing component | [README](packages/script-editor/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/script-editor) |
| `@superinstance/error-handler` | 1.0.0 | Retry logic, circuit breaker, error aggregation | [README](packages/error-handler/README.md) \| [GitHub](https://github.com/SuperInstance/SuperInstance/tree/main/packages/error-handler) |

---

## Applications

| Application | Port | Description | GitHub |
|-------------|------|-------------|--------|
| **SuperInstanceCore** | 3001 | Module dashboard and manager | [github.com/SuperInstance/SuperInstanceCore](https://github.com/SuperInstance/SuperInstanceCore) |
| **PersonalLog** | 3002 | Personal everyday use app | [github.com/SuperInstance/PersonalLog](https://github.com/SuperInstance/PersonalLog) |
| **MakerLog** | 3003 | Creator-focused tools | [github.com/SuperInstance/MakerLog](https://github.com/SuperInstance/MakerLog) |
| **Murmur** | 3004 | TensorDB wiki & bulletin board | [github.com/SuperInstance/Murmur](https://github.com/SuperInstance/Murmur) |
| **ec2mud** | 3005 | Multi-user dungeon game | [github.com/SuperInstance/ec2mud](https://github.com/SuperInstance/ec2mud) |

---

## Backend Services

| Service | Port | Description | GitHub |
|---------|------|-------------|--------|
| **SwarmOrchestration** | 3000 | Multi-agent coordination | [github.com/SuperInstance/SwarmOrchestration](https://github.com/SuperInstance/SwarmOrchestration) |
| **LucidDreamer** | 4000 | Cognitive AI engine | [github.com/SuperInstance/luciddreamer](https://github.com/SuperInstance/luciddreamer) |

---

## Shared Libraries

| Library | Version | Description | GitHub |
|---------|---------|-------------|--------|
| **@superinstance/ui** | 1.0.0 | React component library | [github.com/SuperInstance/UI](https://github.com/SuperInstance/UI) |

---

## Documentation

| Resource | Description | URL |
|----------|-------------|-----|
| **Docs Site** | Full documentation | [github.com/SuperInstance/SuperInstanceDocs](https://github.com/SuperInstance/SuperInstanceDocs) |
| **Examples** | Code examples and templates | [github.com/SuperInstance/SuperInstanceExamples](https://github.com/SuperInstance/SuperInstanceExamples) |
| **Community** | Contributing guidelines | [github.com/SuperInstance/SuperInstanceCommunity](https://github.com/SuperInstance/SuperInstanceCommunity) |

---

## Architecture

### Technology Stack

- **Runtime:** Node.js 18+
- **Language:** TypeScript 5
- **Frontend:** React 19, Next.js 15
- **Styling:** Tailwind CSS 4
- **Package Manager:** pnpm 8+
- **Build:** tsup, Turborepo

### Design Principles

1. **Independence First** - Every package works standalone
2. **Interfaces Over Implementation** - Define contracts, not details
3. **Zero Overhead** - Don't pay for what you don't use
4. **Type Safety** - TypeScript everywhere
5. **Developer Experience** - Make it joyful to use

---

## Versioning

All packages follow [Semantic Versioning 2.0.0](https://semver.org/).

- **MAJOR** - Incompatible API changes
- **MINOR** - Backwards-compatible functionality
- **PATCH** - Backwards-compatible bug fixes

Current release: **v1.0.0**

---

## Contributing

We welcome contributions! Please see:

1. [Contributing Guide](https://github.com/SuperInstance/SuperInstanceCommunity/blob/main/CONTRIBUTING.md)
2. [Code of Conduct](https://github.com/SuperInstance/SuperInstanceCommunity/blob/main/CODE_OF_CONDUCT.md)
3. [Onboarding Guide](./ONBOARDING.md)

---

## License

All packages and applications are released under the **MIT License**.

See [LICENSE](https://github.com/SuperInstance/SuperInstance/blob/main/LICENSE) for details.

---

## Support

- **Issues:** https://github.com/SuperInstance/SuperInstanceCore/issues
- **Discussions:** https://github.com/orgs/SuperInstance/discussions
- **Email:** hello@superinstance.dev

---

**SuperInstance** - Modular toolkit ecosystem for intelligent applications.
