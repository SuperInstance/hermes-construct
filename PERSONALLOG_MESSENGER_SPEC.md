# PersonalLog Messenger - Architecture Specification

## Vision

A messenger-style interface for stream-of-thought note-taking, conversation management, and AI interaction. The system grows in intelligence naturally through use.

## Core Concepts

### 1. Conversations as the Primary Unit

Everything is a **Conversation**:
- Personal note-taking conversations (just you)
- AI-assisted conversations (you + one or more AI contacts)
- Reference conversations (transcripts, uploaded content)

### 2. AI Contacts

An **AI Contact** is a saved AI persona/preset:
- System prompt
- Model settings (temperature, response length, etc.)
- Conversation history as context for personality
- Reusable across conversations

### 3. Message Types

| Type | Purpose | Features |
|------|---------|----------|
| **Note** | Stream-of-thought to self | Text, quick capture |
| **Media** | Images, files, audio | Upload, STT for audio |
| **Transcript** | Long-form audio | Full transcription, analysis |
| **AI Message** | Chatbot response | Brief (messenger) or long-form |
| **System** | Context compaction, events | Read-only, informative |

### 4. Response Modes

| Mode | Context | Response Style |
|------|----------|----------------|
| **Messenger** | In-conversation tab | Brief, text-message style |
| **Long-Form** | Separate tab | Full, detailed responses |

## Data Models

### Conversation

```typescript
interface Conversation {
  id: string
  title: string
  type: 'personal' | 'ai-assisted' | 'transcript' | 'reference'
  createdAt: string
  updatedAt: string
  messages: Message[]
  aiContacts: AIAgent[]  // AI agents in this conversation
  settings: ConversationSettings
  metadata: ConversationMetadata
}

interface ConversationSettings {
  responseMode: 'messenger' | 'long-form'
  compactOnLimit: boolean
  compactStrategy: 'summarize' | 'extract-key' | 'user-directed'
  escalationThreshold?: number  // For local->cloud escalation
}

interface ConversationMetadata {
  messageCount: number
  totalTokens: number
  hasMedia: boolean
  tags: string[]
  pinned: boolean
}
```

### Message

```typescript
interface Message {
  id: string
  conversationId: string
  type: MessageType
  author: MessageAuthor
  content: MessageContent
  timestamp: string
  selected: boolean
  replyTo?: string
  metadata: MessageMetadata
}

type MessageType =
  | 'text'
  | 'image'
  | 'file'
  | 'audio'
  | 'transcript'
  | 'system'

type MessageAuthor =
  | 'user'
  | { type: 'ai-contact', contactId: string }
  | { type: 'system' }

interface MessageContent {
  text?: string
  media?: MediaAttachment
  audioTranscript?: {
    text: string
    audioKept: boolean
    duration: number
  }
  compaction?: {
    originalMessageCount: number
    summary: string
    preserved: string[]
  }
}

interface MediaAttachment {
  type: 'image' | 'file' | 'audio'
  url: string
  mimeType: string
  size: number
  thumbnail?: string
}

interface MessageMetadata {
  tokens?: number
  model?: string
  selectedForAI?: boolean
  editHistory?: Edit[]
}

interface Edit {
  timestamp: string
  previous: string
}
```

### AI Contact

```typescript
interface AIAgent {
  id: string
  name: string
  avatar?: string
  createdAt: string
  config: AIConfig
  personality: PersonalityProfile
  capabilities: AgentCapabilities
}

interface AIConfig {
  // Model selection
  provider: 'local' | 'openai' | 'anthropic' | 'google' | 'custom'
  model: string

  // Generation settings
  temperature: number
  maxTokens: number
  responseStyle: 'brief' | 'balanced' | 'detailed'

  // Escalation (for local)
  escalateToCloud?: boolean
  escalationPatience?: number  // Seconds before escalating
  cloudProvider?: string

  // Multi-bot
  arrangement?: 'parallel' | 'series'
  collaborators?: string[]  // Other AI contact IDs
}

interface PersonalityProfile {
  systemPrompt: string
  vibeAttributes: VibeAttribute[]
  contextConversations: string[]  // IDs that shape personality
  responsePatterns: ResponsePattern[]
}

interface VibeAttribute {
  attribute: string
  value: number
  source: 'user-set' | 'learned'
}

interface ResponsePattern {
  trigger: string
  response: string
  confidence: number
}

interface AgentCapabilities {
  canSeeWeb: boolean
  canSeeFiles: boolean
  canHearAudio: boolean
  canGenerateImages: boolean
}
```

### Global Settings

```typescript
interface GlobalSettings {
  defaultAI: {
    provider: AIConfig['provider']
    model: string
    temperature: number
    responseStyle: AIConfig['responseStyle']
  }
  escalation: {
    enabled: boolean
    defaultPatience: number
    cloudProvider: string
  }
  privacy: {
    localOnly: boolean
    dataRetention: number  // days
  }
  compaction: {
    autoCompact: boolean
    threshold: number  // tokens
    defaultStrategy: ConversationSettings['compactStrategy']
  }
}
```

## UI Architecture

### Main Views

```
┌─────────────────────────────────────────────────────────────┐
│ PERSONALLOG MESSENGER                                      │
├─────────────┬───────────────────────────────────────────────┤
│             │                                               │
│ CONVERSATION│  CHAT AREA                                    │
│ LIST        │                                               │
│             │  ┌─────────────────────────────────────────┐ │
│ 🔍 Search   │  │ User: Thinking about the architecture  │ │
│             │  │    for this new messenger system...     │ │
│ ┌─────────┐ │  ├─────────────────────────────────────────┤ │
│ │Project X│ │  │ 📎 Selected (3)    [Send to AI] [New]  │ │
│ ├─────────┤ │  ├─────────────────────────────────────────┤ │
│ │Notes    │ │  │ Alex AI: I like the messenger concept!  │ │
│ │to Self  │ │  │    A few thoughts...                   │ │
│ ├─────────┤ │  ├─────────────────────────────────────────┤ │
│ │Alex AI  │ │  │ User: Upload: screenshot.png            │ │
│ └─────────┘ │  ├─────────────────────────────────────────┤ │
│             │  │ [Type a message...]          [📎][🎤]   │ │
│ [+ New]     │  └─────────────────────────────────────────┘ │
│             │                                               │
│ AI CONTACTS │  [Toggle: Brief Mode | Long-Form View]       │
│             │  [⚙️ Advanced]                               │
│ ┌─────────┐ │                                               │
│ │Alex     │ │                                               │
│ │Research │ │                                               │
│ │Helper   │ │                                               │
│ └─────────┘ │                                               │
│             │                                               │
│ [+ Create]  │                                               │
└─────────────┴───────────────────────────────────────────────┘
```

### Views Breakdown

1. **Conversation List** (Left sidebar)
   - Search conversations
   - Recent conversations
   - Pinned conversations
   - Tags/folders

2. **AI Contacts** (Left sidebar, below conversations)
   - Saved AI personas
   - Quick-add to conversation

3. **Chat Area** (Main)
   - Message bubbles
   - Message selection (multi-select)
   - Media display
   - Typing indicators

4. **Advanced Options** (Modal/drawer)
   - Per-conversation AI settings
   - Compaction controls
   - Model selection
   - Vibe-fine-tuning

5. **Long-Form View** (Separate tab)
   - Full-width reading
   - Detailed responses
   - Document-style layout

## Key Interactions

### Message Selection → AI Chat

```
1. User taps messages (multiple)
2. Toolbar appears: [Send to AI in Chat] [Start New Chat]
3. If "Send to AI in Chat":
   - Selected messages become context
   - User can add prompt
   - AI response appears in conversation
4. If "Start New Chat":
   - New conversation created
   - Selected messages copied
   - User selects/creates AI contact
```

### Vibe-Fine-Tuning

```
1. User: "Be more enthusiastic"
2. System:
   - Detects personality request
   - Shows "Vibe Update" panel
   - Displays what changed:
     • enthusiasm: 0.3 → 0.7
     • useEmojis: false → true
   - User confirms or adjusts
3. AI contact updated with new vibe
```

### Context Compaction

```
1. When token limit approached:
   - System shows "Conversation is getting long"
   - Options: [Review & Compact] [Auto-Compact] [Start New]
2. If "Review & Compact":
   - User selects what to prioritize
   - System generates summary
   - User confirms
3. Result: Compact message replaces older messages
   - Originals archived (not deleted)
```

### Model Agnostic Routing

```
┌──────────────┐
│ User Message │
└──────┬───────┘
       │
       ▼
┌─────────────────────────────┐
│ Conversation AI Config      │
│ • Provider: local           │
│ • Escalation: enabled       │
│ • Patience: 30s             │
└──────────┬──────────────────┘
           │
           ▼
┌─────────────────────────────┐
│ LOCAL MODEL ATTEMPT         │
│ • Timeout: 30s              │
│ • If complete → done        │
│ • If timeout/poor → escalate│
└──────────┬──────────────────┘
           │
           ▼ (if escalation)
┌─────────────────────────────┐
│ CLOUD MODEL                 │
│ • Provider: user selection  │
│ • Fallback response         │
└─────────────────────────────┘
```

### Parallel vs Series Multi-Bot

```
PARALLEL:
User → [Bot A] ──┐
      └──> [Bot B] ─├──> Display both responses
      └──> [Bot C] ──┘

SERIES:
User → [Bot A] → [Bot B sees A's response] → [Bot C sees both]
                     ↓
                  Combined response
```

## Implementation Phases

### Phase 1: Core Messenger (Foundation)
- [ ] Conversation data model and storage
- [ ] Message rendering (text only)
- [ ] Message selection
- [ ] Basic AI integration (single model)

### Phase 2: AI Contact System
- [ ] AI contact creation and management
- [ ] Per-conversation AI settings
- [ ] Vibe-fine-tuning UI
- [ ] Personality saving/loading

### Phase 3: Media Support
- [ ] Image/file upload
- [ ] Audio recording with STT
- [ ] Long-form transcription
- [ ] Media gallery

### Phase 4: Advanced Features
- [ ] Context compaction
- [ ] Long-form view mode
- [ ] Multi-bot (parallel/series)
- [ ] Model-agnostic routing

### Phase 5: Polish
- [ ] Animations and transitions
- [ ] Keyboard shortcuts
- [ ] Accessibility
- [ ] Performance optimization

## Technical Stack

- **Frontend**: Next.js 15, React 19, TypeScript
- **Styling**: Tailwind CSS 4
- **Storage**: IndexedDB (local), optional cloud sync
- **AI**: Model-agnostic adapter pattern
- **STT**: Web Speech API (local) + optional cloud
- **State**: React Context + hooks

## File Structure

```
PersonalLog/
├── src/
│   ├── app/
│   │   ├── (messenger)/
│   │   │   ├── layout.tsx
│   │   │   ├── page.tsx           # Main messenger view
│   │   │   └── conversation/
│   │   │       └── [id]/page.tsx
│   │   ├── (longform)/
│   │   │   ├── layout.tsx
│   │   │   └── conversation/
│   │   │       └── [id]/page.tsx
│   │   └── api/
│   │       └── ai/
│   │           ├── chat/route.ts
│   │           ├── transcribe/route.ts
│   │           └── compact/route.ts
│   ├── components/
│   │   ├── messenger/
│   │   │   ├── ConversationList.tsx
│   │   │   ├── ChatArea.tsx
│   │   │   ├── MessageBubble.tsx
│   │   │   ├── MessageSelector.tsx
│   │   │   ├── AIAvatar.tsx
│   │   │   └── TypingIndicator.tsx
│   │   ├── ai-contacts/
│   │   │   ├── AIContactCard.tsx
│   │   │   ├── AIContactCreator.tsx
│   │   │   ├── VibeFineTuner.tsx
│   │   │   └── AdvancedOptions.tsx
│   │   ├── media/
│   │   │   ├── AudioRecorder.tsx
│   │   │   ├── ImageUploader.tsx
│   │   │   ├── TranscriptView.tsx
│   │   │   └── MediaGallery.tsx
│   │   └── compaction/
│   │       ├── CompactionDialog.tsx
│   │       └── SummaryPreview.tsx
│   ├── lib/
│   │   ├── storage/
│   │   │   ├── conversation-store.ts
│   │   │   ├── ai-contact-store.ts
│   │   │   └── message-store.ts
│   │   ├── ai/
│   │   │   ├── model-adapter.ts
│   │   │   ├── local-provider.ts
│   │   │   ├── cloud-providers.ts
│   │   │   └── escalation-handler.ts
│   │   └── stt/
│   │       ├── web-speech.ts
│   │       └── transcriber.ts
│   └── types/
│       ├── conversation.ts
│       ├── ai-contact.ts
│       └── message.ts
```

## Next Steps

1. Create core data models and storage
2. Build messenger UI components
3. Implement basic AI integration
4. Add AI Contact system
5. Implement media support
6. Add advanced features
