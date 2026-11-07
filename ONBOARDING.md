# SuperInstance Developer Onboarding Guide

Welcome to the SuperInstance ecosystem! This guide will help you get started as a contributor.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Development Environment](#development-environment)
3. [Repository Overview](#repository-overview)
4. [Common Workflows](#common-workflows)
5. [Coding Standards](#coding-standards)
6. [Testing](#testing)
7. [Submitting Changes](#submitting-changes)

---

## Quick Start

### Prerequisites

- **Node.js** 18 or higher
- **pnpm** 8 or higher (recommended package manager)
- **Git** for version control
- **Docker** (for backend services)

### Install pnpm

```bash
npm install -g pnpm
```

### Clone a Repository

```bash
# Clone any SuperInstance repository
git clone https://github.com/SuperInstance/[REPO_NAME].git
cd [REPO_NAME]

# Install dependencies
pnpm install

# Start development server
pnpm dev
```

---

## Development Environment

### Recommended Tools

| Tool | Purpose | Download |
|------|---------|----------|
| VS Code | Code editor | code.visualstudio.com |
| pnpm | Package manager | pnpm.io |
| Docker | Container runtime | docker.com |
| Postman | API testing | postman.com |

### VS Code Extensions

```json
{
  "recommendations": [
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "bradlc.vscode-tailwindcss",
    "ms-vscode.vscode-typescript-next"
  ]
}
```

---

## Repository Overview

### Package Monorepo

**URL:** https://github.com/SuperInstance/SuperInstance

Contains 23+ packages organized by purpose:

**Foundation Primitives:**
- `@superinstance/async` - Async utilities
- `@superinstance/validate` - Schema validation
- `@superinstance/cache` - Caching layer
- `@superinstance/events` - Event system
- `@superinstance/config` - Configuration
- `@superinstance/storage` - Storage abstraction
- `@superinstance/provider-base` - LLM provider base
- `@superinstance/logger` - Logging utilities

**Feature Packages:**
- `@superinstance/chat` - Multi-LLM chat
- `@superinstance/tts` - Text-to-speech
- `@superinstance/asr` - Speech recognition
- `@superinstance/memory` - Memory system
- `@superinstance/rag` - RAG pipeline
- `@superinstance/knowledge-tensor` - Knowledge graph
- `@superinstance/abstraction` - Pattern detection
- `@superinstance/content-analyzer` - Content analysis
- `@superinstance/personalization` - User preferences
- `@superinstance/event-runtime` - WebSocket events
- `@superinstance/audio-workspace` - Audio editing
- `@superinstance/agent-factory` - Agent creation
- `@superinstance/script-editor` - Script editor
- `@superinstance/error-handler` - Error handling

### Applications

| Repo | Port | Purpose |
|------|------|---------|
| SuperInstanceCore | 3001 | Module dashboard |
| PersonalLog | 3002 | Personal app |
| MakerLog | 3003 | Creator tools |
| Murmur | 3004 | Wiki |
| ec2mud | 3005 | Game server |

### Backend Services

| Repo | Port | Purpose |
|------|------|---------|
| SwarmOrchestration | 3000 | Multi-agent coordination |
| LucidDreamer | 4000 | Cognitive AI |

---

## Common Workflows

### Running an App Locally

```bash
git clone https://github.com/SuperInstance/PersonalLog.git
cd PersonalLog
pnpm install
pnpm dev
# Open http://localhost:3002
```

### Running Backend Services

```bash
# SwarmOrchestration (requires Redis)
cd SwarmOrchestration
docker-compose up -d redis
pnpm install
pnpm dev

# LucidDreamer (requires PostgreSQL)
cd LucidDreamer
docker-compose up -d postgres
pnpm install
pnpm dev
```

### Building Packages

```bash
cd packages
pnpm build

# Build specific package
pnpm --filter @superinstance/chat build
```

### Type Checking

```bash
pnpm type-check
```

### Linting

```bash
pnpm lint
```

---

## Coding Standards

### TypeScript

- Use strict TypeScript
- Prefer explicit types over `any`
- Add JSDoc for exports
- Use interfaces for public APIs

### Naming Conventions

- Packages: `@superinstance/kebab-case`
- Files: `kebab-case.ts`
- Components: `PascalCase.tsx`
- Variables: `camelCase`
- Constants: `UPPER_SNAKE_CASE`
- Types: `PascalCase`

### Commit Messages

Follow conventional commits:

```
type(scope): description

feat(chat): add OpenAI provider support
fix(async): resolve timeout issue
docs(readme): update installation instructions
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

---

## Testing

### Unit Tests

```bash
pnpm test
```

### Test Coverage

```bash
pnpm test:coverage
```

### Writing Tests

```typescript
import { describe, it, expect } from 'vitest';

describe('MyFunction', () => {
  it('should do something', () => {
    const result = myFunction('input');
    expect(result).toBe('expected');
  });
});
```

---

## Submitting Changes

### 1. Fork and Branch

```bash
git checkout -b feature/my-feature
```

### 2. Make Changes

- Write code
- Add tests
- Update docs

### 3. Commit

```bash
git add .
git commit -m "feat: add my new feature"
```

### 4. Push and Create PR

```bash
git push origin feature/my-feature
# Create PR on GitHub
```

### PR Checklist

- [ ] Tests pass
- [ ] Linting passes
- [ ] Documentation updated
- [ ] Commit messages follow conventions
- [ ] PR description explains the "why"

---

## Getting Help

- **Documentation:** https://github.com/SuperInstance/SuperInstanceDocs
- **Issues:** https://github.com/SuperInstance/SuperInstanceCore/issues
- **Discussions:** https://github.com/orgs/SuperInstance/discussions
- **Email:** hello@superinstance.dev

---

## Next Steps

1. Star your favorite repositories
2. Join our GitHub Discussions
3. Explore the codebase
4. Find a good first issue to work on
5. Submit your first PR!

Welcome aboard!
