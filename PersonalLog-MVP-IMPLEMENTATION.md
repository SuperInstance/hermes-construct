# PersonalLog MVP - Implementation Plan

**Date:** January 2, 2026
**Status:** Active Development
**Goal:** Create a polished personal journaling/notes application as the MVP for SuperInstance

---

## Executive Summary

PersonalLog is a personal knowledge management application that serves as the MVP for the SuperInstance ecosystem. It combines journaling, note-taking, task management, and a self-populated discussion forum powered by knowledge graphs.

---

## MVP Features

### Core Features (Must Have)

| Feature | Description | Package |
|---------|-------------|---------|
| **Daily Journal** | Write daily entries with mood tracking | Custom |
| **Notes System** | Hierarchical notes with rich text editing | Custom |
| **Task Management** | Create, organize, complete tasks | Custom |
| **Knowledge Graph** | Visual knowledge connections | @superinstance/knowledge-tensor |
| **Discussion Forum** | Self-populated forum from knowledge graph | @superinstance/knowledge-tensor |
| **AI Chat Assistant** | Ask questions about your data | @superinstance/chat |
| **Content Analysis** | Auto-tag and analyze content | @superinstance/content-analyzer |
| **Memory System** | Long-term semantic memory | @superinstance/memory |
| **Search** | Full-text and semantic search | @superinstance/rag |

### Secondary Features (Nice to Have)

| Feature | Description | Priority |
|---------|-------------|----------|
| Dark Mode | Full dark mode support | P1 |
| Mobile Responsive | Works on phones/tablets | P1 |
| Export | Markdown, PDF export | P2 |
| Sync | Cross-device sync | P2 |
| Reminders | Task and journal reminders | P3 |

### Postponed Features (Not for MVP)

| Feature | Reason |
|---------|--------|
| TTS/ASR | Not essential for personal log |
| Agent Factory | Will add later |
| Personalization | Will add later |
| Audio Workspace | Separate app |

---

## Architecture

### Frontend: PersonalLog App

```
personallog/
├── src/
│   ├── app/
│   │   ├── (main)/              # Main app layout
│   │   │   ├── page.tsx         # Dashboard
│   │   │   ├── journal/          # Daily journal
│   │   │   │   └── page.tsx
│   │   │   ├── notes/            # Notes system
│   │   │   │   └── page.tsx
│   │   │   ├── tasks/            # Task management
│   │   │   │   └── page.tsx
│   │   │   ├── forum/            # Discussion forum
│   │   │   │   └── page.tsx
│   │   │   ├── knowledge/        # Knowledge graph visualizer
│   │   │   │   └── page.tsx
│   │   │   └── chat/             # AI assistant
│   │   │       └── page.tsx
│   │   ├── api/                  # Backend API routes
│   │   │   ├── entries/          # Journal/notes CRUD
│   │   │   ├── tasks/            # Task CRUD
│   │   │   ├── forum/            # Forum posts/replies
│   │   │   ├── knowledge/        # Knowledge graph operations
│   │   │   ├── chat/             # AI chat endpoint
│   │   │   ├── search/           # Search functionality
│   │   │   └── analytics/        # Content analysis
│   │   └── layout.tsx
│   ├── components/
│   │   ├── editor/               # Rich text editor
│   │   ├── forum/                # Forum components
│   │   ├── knowledge-graph/      # Graph visualization
│   │   ├── task-list/            # Task management
│   │   └── chat/                 # Chat interface
│   ├── lib/
│   │   ├── backend/              # Backend client
│   │   ├── knowledge-tensor.ts   # Knowledge graph wrapper
│   │   └── storage.ts            # Local storage wrapper
│   └── types/
│       ├── entries.ts
│       ├── tasks.ts
│       ├── forum.ts
│       └── knowledge.ts
```

### Backend: Extensible Module System

```
backend/
├── modules/                      # Module interface
│   ├── base.ts                   # Base module class
│   ├── registry.ts               # Module registry
│   └── types.ts                  # Module types
├── services/
│   ├── entry-service.ts          # Journal/notes service
│   ├── task-service.ts           # Task service
│   ├── forum-service.ts          # Forum service
│   ├── knowledge-service.ts      # Knowledge graph service
│   ├── chat-service.ts           # AI chat service
│   └── search-service.ts         # Search service
├── storage/
│   ├── local-storage.ts          # IndexedDB wrapper
│   └── file-storage.ts           # File system wrapper
└── api/
    ├── routes.ts                 # API route definitions
    └── handlers/                 # Request handlers
```

---

## Data Models

### Entry (Journal/Notes)

```typescript
interface Entry {
  id: string
  type: 'journal' | 'note'
  title: string
  content: string
  tags: string[]
  mood?: 'great' | 'good' | 'okay' | 'bad' | 'terrible'
  parentId?: string  // For hierarchical notes
  createdAt: Date
  updatedAt: Date
  metadata: {
    wordCount: number
    readTime: number
    aiSummary?: string
    aiTags?: string[]
    connections?: string[]  // Related entry IDs
  }
}
```

### Task

```typescript
interface Task {
  id: string
  title: string
  description?: string
  status: 'todo' | 'in-progress' | 'done'
  priority: 'low' | 'medium' | 'high'
  dueDate?: Date
  parentId?: string  // For subtasks
  createdAt: Date
  completedAt?: Date
  tags: string[]
  attachments?: string[]
}
```

### Forum Post

```typescript
interface ForumPost {
  id: string
  title: string
  content: string
  author: 'user' | 'ai'
  category: string
  tags: string[]
  createdAt: Date
  updatedAt: Date
  replyTo?: string  // For nested replies
  metadata: {
    views: number
    likes: number
    aiGenerated: boolean
    relatedTopics: string[]  // Knowledge graph connections
  }
}
```

### Knowledge Node

```typescript
interface KnowledgeNode {
  id: string
  type: 'concept' | 'topic' | 'theme' | 'entity' | 'entry' | 'task'
  label: string
  data: {
    entryId?: string
    taskId?: string
    excerpt?: string
  }
  metadata: {
    confidence: number
    source: 'user' | 'ai'
    createdAt: Date
    lastAccessed: Date
  }
}
```

---

## Self-Populated Discussion Forum

### How It Works

1. **Content Analysis**: When user creates entries/tasks, AI analyzes and extracts:
   - Key topics and concepts
   - Questions posed by user
   - Insights and patterns
   - Action items

2. **Knowledge Graph Update**: Extracted entities are added to knowledge tensor:
   - Nodes: topics, concepts, entities
   - Edges: relationships between them
   - Strengthening: reinforces with each mention

3. **Forum Auto-Population**: System generates forum posts when:
   - New topic emerges (3+ mentions)
   - Question is unanswered
   - Pattern is detected
   - User asks for exploration

4. **AI Responses**: AI generates responses to:
   - Open questions
   - Topic requests
   - Pattern exploration requests
   - "Tell me more about X" queries

### Forum Categories

| Category | Auto-Created When | Purpose |
|----------|-------------------|---------|
| **Topics** | 3+ entries mention concept | Explore themes |
| **Questions** | User poses question | Q&A accumulation |
| **Insights** | AI detects pattern | Pattern visualization |
| **Actions** | Task created | Task tracking |
| **Connections** | 2+ topics related | Cross-topic discussion |

---

## API Design

### Entries API

```typescript
// GET /api/entries
// List all entries (filterable)
{
  type?: 'journal' | 'note'
  tag?: string
  dateFrom?: Date
  dateTo?: Date
  search?: string
}

// POST /api/entries
// Create new entry
{
  type: 'journal' | 'note'
  title: string
  content: string
  tags?: string[]
  mood?: string
  parentId?: string
}

// GET /api/entries/[id]
// Get single entry with connections

// PUT /api/entries/[id]
// Update entry

// DELETE /api/entries/[id]
// Delete entry
```

### Tasks API

```typescript
// GET /api/tasks
// List all tasks (filterable)

// POST /api/tasks
// Create new task

// PUT /api/tasks/[id]/complete
// Mark task complete

// PUT /api/tasks/[id]
// Update task

// DELETE /api/tasks/[id]
// Delete task
```

### Forum API

```typescript
// GET /api/forum/posts
// List forum posts

// POST /api/forum/posts
// Create new post (user or auto-generated)

// POST /api/forum/posts/[id]/reply
// Reply to post

// GET /api/forum/categories
// List all categories

// POST /api/forum/auto-populate
// Trigger auto-population from knowledge graph
```

### Knowledge API

```typescript
// GET /api/knowledge/graph
// Get knowledge graph data

// GET /api/knowledge/related/[id]
// Get related nodes

// POST /api/knowledge/analyze
// Trigger content analysis

// GET /api/knowledge/stats
// Get knowledge graph statistics
```

### Chat API

```typescript
// POST /api/chat/message
// Send message to AI assistant

// GET /api/chat/history
// Get chat history

// DELETE /api/chat/history
// Clear chat history
```

---

## Implementation Phases

### Phase 1: Foundation (Week 1)

**Goal:** Set up project structure and basic CRUD

- [ ] Create PersonalLog app structure
- [ ] Set up database schema
- [ ] Implement Entry CRUD
- [ ] Implement Task CRUD
- [ ] Basic UI with routing

### Phase 2: Knowledge Integration (Week 2)

**Goal:** Integrate knowledge-tensor and AI

- [ ] Integrate @superinstance/knowledge-tensor
- [ ] Content analysis on entry creation
- [ ] Auto-tagging and connections
- [ ] Knowledge graph visualization
- [ ] AI chat assistant

### Phase 3: Forum (Week 3)

**Goal:** Self-populated discussion forum

- [ ] Forum data models
- [ ] Forum CRUD API
- [ ] Auto-population logic
- [ ] Forum UI components
- [ ] AI-generated responses

### Phase 4: Polish (Week 4)

**Goal:** Production-ready MVP

- [ ] Dark mode
- [ ] Mobile responsiveness
- [ ] Performance optimization
- [ ] Error handling
- [ ] Testing

---

## BusinessLog Variations

### What Changes

| Aspect | PersonalLog | BusinessLog |
|--------|-------------|-------------|
| **Theme** | Casual, warm | Professional, clean |
| **Color Palette** | Warm tones, gradients | Cool blues, grays |
| **Terminology** | Journal, notes | Logs, reports |
| **Templates** | Personal growth | Business metrics |
| **Features** | Mood tracking | Project tracking |
| **AI Tone** | Friendly, personal | Professional, concise |

### Shared Backend

Both apps use the same backend - only the frontend presentation differs:

```
backend/                         # Shared
├── PersonalLog/                 # Personal frontend
│   ├── src/
│   │   ├── themes/
│   │   │   └── personal.ts     # Warm, casual theme
│   │   └── components/
│   └── package.json
│
└── BusinessLog/                 # Business frontend
    ├── src/
    │   ├── themes/
    │   │   └── professional.ts  # Clean, business theme
    │   └── components/
    └── package.json
```

---

## Success Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Features Complete | 100% | 0% |
| Backend APIs | 9 | 0 |
| UI Components | 20+ | 0 |
| Knowledge Nodes | Auto-generated | N/A |
| Forum Auto-Populated | Working | N/A |

---

## Next Steps

1. **Create backend API structure** - Extensible module system
2. **Implement Entry service** - Journal/notes CRUD
3. **Integrate knowledge-tensor** - Knowledge graph
4. **Build forum auto-population** - Discussion system
5. **Create PersonalLog frontend** - MVP app
6. **Create BusinessLog theme** - Professional variant

---

*Generated with [Claude Code](https://claude.com/claude-code)*
