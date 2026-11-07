# DMLog MVP Plan
## AI-Powered Tabletop RPG Campaign Management

**Domain:** https://dmlog.ai
**Repository:** https://github.com/SuperInstance/DMLog
**Version:** 0.1.0
**Last Updated:** 2026-01-01
**Status:** Planning Phase

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Problem Statement](#2-problem-statement)
3. [Target Audience](#3-target-audience)
4. [Core MVP Features v0.1.0](#4-core-mvp-features-v010)
5. [Technical Architecture](#5-technical-architecture)
6. [Data Model](#6-data-model)
7. [User Stories](#7-user-stories)
8. [Differentiation](#8-differentiation)
9. [Development Roadmap](#9-development-roadmap)
10. [Monetization Strategy](#10-monetization-strategy)
11. [Success Metrics](#11-success-metrics)
12. [Next Steps](#12-next-steps)

---

## 1. Executive Summary

### What DMLog Solves

**DMLog** is an AI-powered campaign management platform for tabletop role-playing games (TTRPGs) that combines intelligent organization, creative content generation, and session management into one seamless experience. It's the "forever DM's assistant" that never forgets details, generates content on demand, and adapts to any game system.

**Core Value Proposition:**
- **Intelligent Campaign Organization:** Unified hub for campaigns, characters, locations, items, and lore
- **AI Content Generation:** Generate NPCs, encounters, locations, and story hooks on demand
- **Session Transcription & Review:** Record game sessions and search through them later
- **System Agnostic:** Works with D&D, Pathfinder, Call of Cthulhu, Shadowrun, Vampire: The Masquerade, and homebrew systems
- **Adaptive Tone Matching:** AI adapts to your campaign's tone, style, and world rules
- **Knowledge Retrieval:** RAG-powered search across your entire campaign history

### Target Market Size

- **Global TTRPG Market:** $15B annually (2025), growing 12% CAGR
- **D&D 5E Players:** 50+ million worldwide
- **Virtual Tabletop Users:** 15+ million (Roll20, Foundry, Fantasy Grounds)
- **Addressable Market:** Dungeon Masters and campaign organizers (~20% of players)
- **Initial Focus:** English-speaking D&D 5E and Pathfinder communities

### Business Model

Freemium SaaS with AI-powered content generation:
- **Free Tier:** Basic campaign management, manual note-taking, community content
- **Pro Tier ($9/mo):** AI content generation, session transcription, advanced search
- **Campaign Tier ($19/mo):** Multi-campaign, player collaboration, voice synthesis
- **Enterprise:** Licensing for game stores, streaming groups, and content creators

---

## 2. Problem Statement

### The Current Pain Points

#### 1. **Campaign Organization is Fragmented**

- **Scattered Notes:** DMs use disparate tools (Google Docs, Notion, physical notebooks, wikis)
- **Lost Details:** Critical NPCs, plot hooks, and world lore get forgotten across sessions
- **No Unified View:** Can't quickly see relationships between characters, locations, and items
- **System Constraints:** VTTs like Roll20 focus on tactical combat, not campaign management
- **Search is Impossible:** Finding that one NPC from 10 sessions ago takes hours

#### 2. **Content Creation is Time-Consuming**

- **NPC Generation:** Creating memorable NPCs with personalities, backstories, and voices takes hours
- **Encounter Design:** Balancing encounters requires deep system knowledge and math
- **World-Building Fatigue:** Fleshing out locations, lore, and history is overwhelming
- **Improvisation Pressure:** DMs freeze when players go off the rails
- **Consistency Challenges:** Remembering plot threads, character motivations, and world rules

#### 3. **Session Review is Non-Existent**

- **No Record:** Most sessions are lost to time with only hazy memories
- **Transcription is Manual:** Taking notes while running a game is distracting
- **No Replayability:** Can't easily revisit epic moments or funny quotes
- **Campaign Continuity:** New players struggle to catch up on past sessions
- **Content Mining:** Can't easily extract NPCs or locations from past sessions for reuse

#### 4. **Multi-System Support is Weak**

- **System-Specific Tools:** Most tools focus on one game (usually D&D 5E)
- **Homebrew Nightmares:** Hard to adapt tools to custom rules or homebrew content
- **Cross-System Campaigns:** Playing in different systems requires learning new tools
- **Rules Reference:** Looking up rules across multiple systems is cumbersome

### The Opportunity

The TTRPG renaissance is accelerating:
- **Critical Role Effect:** Millions of new DMs entering the hobby
- **D&D 5E Dominance:** But players are exploring other systems (Pathfinder 2E, Call of Cthulhu)
- **Streaming Culture:** Actual play podcasts and streams are mainstream entertainment
- **Digital Adoption:** COVID accelerated VTT adoption from 20% to 80% of players
- **AI Acceptance:** DMs are enthusiastic about AI assistants (see DungeonAI, RPGGO)

**Existing solutions are narrow, system-specific, and lack AI.** DMLog will be the first truly AI-native, system-agnostic campaign management platform.

---

## 3. Target Audience

| Segment | Size | Pain Points | DMLog Value |
|---------|------|-------------|-------------|
| **Forever DMs** | Large | Burnout from constant prep, improvisation fatigue | AI generates content, handles lore recall, reduces prep time 80% |
| **New DMs** | Growing | Overwhelmed by rules, world-building, NPCs | Templates, generators, and AI coaching build confidence |
| **Streamers/Podcasters** | Medium | Need searchable archives, content mining | Transcription, clip extraction, searchable session library |
| **Game Store Organized Play** | Small | Managing multiple campaigns, tracking players | Multi-campaign dashboards, player progress tracking |
| **Solo Players** | Niche | Need AI DM, world simulation | AI dungeon master, reactive world simulation |
| **System Hoppers** | Medium | Learning new systems, adapting campaigns | System-agnostic tools, rules reference, homebrew support |

### Primary Persona: "The Burned-Out Forever DM"

- **Age:** 25-40
- **Experience:** 5+ years DMing
- **Pain Points:** Running 2-3 weekly campaigns, exhausted by prep, forgetting details
- **Goals:** Reduce prep time, stay organized, impress players with quality content
- **Tech Comfort:** High (uses VTTs, Discord, digital tools)
- **Willingness to Pay:** $10-20/mo for time-saving tools

### Secondary Persona: "The Aspiring Streamer"

- **Age:** 18-30
- **Experience:** 1-3 years DMing
- **Pain Points:** Content creation, building audience, searchable archives
- **Goals:** Produce high-quality streams, grow audience, monetize content
- **Tech Comfort:** Very high (OBS, streaming tools, social media)
- **Willingness to Pay:** $20-30/mo for content creation tools

---

## 4. Core MVP Features v0.1.0

### Feature 1: Campaign Organization Hub

**Goal:** Unified dashboard for all campaign elements

#### Capabilities

- **Campaign Dashboard**
  - Campaign overview (name, system, setting, party level, session count)
  - Quick access to recent sessions, NPCs, locations
  - Campaign timeline (visual session history)
  - Party overview (PCs with quick stat blocks)

- **Character Management**
  - **PC Tracking:** Import from VTT or manual entry, track levels, items, notes
  - **NPC Library:** Organized by location, faction, relationship, CR/rank
  - **Relationships:** Visual graph of connections between characters
  - **Quick Reference:** Stat blocks, personality notes, voice descriptions, secrets

- **Location Catalog**
  - Hierarchical organization (World → Region → City → District → Building)
  - Tags and categories (urban, wilderness, dungeon, planar)
  - Connected NPCs, items, plot hooks
  - Rich descriptions with images and custom fields

- **Item Database**
  - Equipment, magic items, artifacts, treasures
  - Organized by rarity, type, location, owner
  - Custom fields for homebrew items
  - Import from SRD or manual entry

- **Lore & Plot Hooks**
  - Timeline of events (past, present, future)
  - Plot hook tracker (active, resolved, abandoned)
  - Faction relationships and conflicts
  - World history and cosmology

#### Technical Approach

```
┌─────────────────────────────────────────────────────────────┐
│                     DMLog Web App                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ Campaign    │  │ Session     │  │ Content     │         │
│  │ Dashboard   │  │ Library     │  │ Generators  │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                  │
│         └────────────────┼────────────────┘                  │
│                          ▼                                   │
│              ┌───────────────────────┐                       │
│              │   Unified Data Store  │                       │
│              │   (PostgreSQL)        │                       │
│              └───────────┬───────────┘                       │
│                          │                                   │
│         ┌────────────────┼────────────────┐                 │
│         ▼                ▼                ▼                 │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ Campaigns  │  │ Sessions   │  │ Entities   │            │
│  │ (CRUD)     │  │ (Time-     │  │ (NPCs,     │            │
│  │            │  │  stamped)  │  │  Locations)│            │
│  └────────────┘  └────────────┘  └────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

#### API Endpoints

```
GET    /api/campaigns                    # List campaigns
POST   /api/campaigns                    # Create campaign
GET    /api/campaigns/:id                # Get campaign details
PUT    /api/campaigns/:id                # Update campaign
DELETE /api/campaigns/:id                # Delete campaign

GET    /api/campaigns/:id/characters     # List NPCs/PCs
POST   /api/campaigns/:id/characters     # Create character
GET    /api/campaigns/:id/locations      # List locations
POST   /api/campaigns/:id/locations      # Create location
GET    /api/campaigns/:id/items          # List items
POST   /api/campaigns/:id/items          # Create item

GET    /api/search?q=query               # Full-text search
```

---

### Feature 2: Session Logging & Notes

**Goal:** Record sessions with structured notes and manual transcription

#### Capabilities

- **Session Creation**
  - Quick-start session with date, location, attendees
  - Session template (one-shot, episodic, arc-based)
  - Pre-loaded NPCs, locations, items from campaign

- **Note-Taking Interface**
  - Rich text editor with markdown support
  - Quick-insert NPCs, locations, items (autocomplete)
  - Timestamped notes for reference
  - Tags and categories (combat, roleplay, exploration, puzzle)

- **Session Summary**
  - Auto-generate summary from notes using AI (v0.2.0)
  - Track XP awarded, loot found, NPCs met
  - Highlight "moments" (epic wins, funny quotes, deaths)
  - Export to PDF, markdown, or VTT format

- **Session Timeline**
  - Visual timeline of campaign sessions
  - Filter by location, NPCs, plot hooks
  - Track campaign arcs and threads

#### Technical Approach

```
┌─────────────────────────────────────────────────────────────┐
│                     Session Editor                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Rich Text Editor                                     │    │
│  │ [Insert NPC] [Insert Location] [Insert Item]         │    │
│  │                                                      │    │
│  │ # Session 12: The Dragon's Lair                     │    │
│  │                                                      │    │
│  │ The party arrived at the mountain pass...           │    │
│  │                                                      │    │
│  │ [Timestamp: 8:05 PM]                                │    │
│  │ Met guard captain @Thorin Ironshield                │    │
│  │                                                      │    │
│  │ [Timestamp: 8:30 PM]                                │    │
│  │ Combat encounter with 3 goblins...                  │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
│  ┌───────────────┐  ┌───────────────┐                       │
│  │ AI Summary    │  │ Export        │                       │
│  │ (v0.2.0)      │  │ Options       │                       │
│  └───────────────┘  └───────────────┘                       │
└─────────────────────────────────────────────────────────────┘
```

#### API Endpoints

```
GET    /api/campaigns/:id/sessions       # List sessions
POST   /api/campaigns/:id/sessions       # Create session
GET    /api/sessions/:id                 # Get session details
PUT    /api/sessions/:id                 # Update session
DELETE /api/sessions/:id                 # Delete session
POST   /api/sessions/:id/summary         # Generate AI summary (v0.2.0)
```

---

### Feature 3: AI NPC Generator

**Goal:** Generate fully fleshed NPCs on demand

#### Capabilities

- **Quick Generation**
  - Input: Name (optional), race, class, role, level
  - Output: Full stat block, appearance, personality, backstory, voice notes
  - System-specific templates (D&D 5E, Pathfinder 2E, etc.)

- **Personality Engine**
  - Randomized traits (ideals, bonds, flaws, mannerisms)
  - Voice and speech patterns
  - Relationships and secrets
  - Motivations and goals

- **Adaptive Generation**
  - Learn from campaign tone (dark, heroic, humorous)
  - Match world-building (fitting cosmology, factions, history)
  - Consistency checks (doesn't contradict established lore)

- **Visual Integration**
  - AI-generated portrait prompts (for Midjourney, DALL-E)
  - Style matching (cartoon, realistic, oil painting, sketch)
  - Token export for VTTs

#### Technical Approach

```
┌─────────────────────────────────────────────────────────────┐
│                   AI NPC Generator                           │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  User Input:                                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Race: Elf    Class: Wizard    Level: 5              │    │
│  │ Role: Shopkeeper   Tone: Mysterious                │    │
│  │ Location: Arcane District                         │    │
│  │ [Generate]                                         │    │
│  └─────────────────────────────────────────────────────┘    │
│                          │                                   │
│                          ▼                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ LucidDreamer API (Port 4000)                         │    │
│  │ - Generate personality traits                         │    │
│  │ - Create backstory                                  │    │
│  │ - Match campaign tone                               │    │
│  └──────────────────────┬──────────────────────────────┘    │
│                          │                                   │
│                          ▼                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Output:                                             │    │
│  │                                                     │    │
│  │ Name: Sylara Moonwhisper                            │    │
│  │ Race: High Elf    Class: Wizard (School of Div.)   │    │
│  │                                                     │    │
│  │ Personality:                                        │    │
│  │ - Speaks in riddles and metaphors                   │    │
│  │ - Obsessed with predicting the future               │    │
│  │ - Secret: Sold her soul to a devil for knowledge   │    │
│  │                                                     │    │
│  │ Voice: Soft, ethereal, pauses frequently           │    │
│  │                                                     │    │
│  │ [Save to Campaign] [Generate Image] [Edit]         │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

#### API Endpoints

```
POST   /api/generate/npc                  # Generate NPC
POST   /api/generate/npc/:id/voice         # Generate voice description
POST   /api/generate/npc/:id/portrait      # Generate portrait prompt
POST   /api/generate/npc/batch             # Generate multiple NPCs
```

#### Generation Flow

1. User inputs parameters (or leaves blank for random)
2. DMLog builds generation prompt with campaign context
3. Calls LucidDreamer API with:
   - Campaign tone, world rules, existing NPCs
   - User parameters
   - System-specific constraints
4. LucidDreamer generates:
   - Personality traits
   - Backstory
   - Relationships to existing entities
   - Secrets and plot hooks
5. DMLog formats output as NPC entity
6. User can edit, regenerate, or save to campaign

---

### Feature 4: Encounter Builder

**Goal:** Quick, balanced encounter generation

#### Capabilities

- **Party-Based Balancing**
  - Input party level, size, composition
  - Calculate XP budget (per DMG guidelines)
  - Suggest monster count and CR
  - Adjust for difficulty (easy, medium, hard, deadly)

- **Encounter Templates**
  - Combat (arena, dungeon, wilderness, urban)
  - Social (negotiation, intrigue, investigation)
  - Puzzle (riddles, traps, skill challenges)
  - Mixed (combat + social + puzzle)

- **Loot Generator**
  - Roll treasure hoards per DMG
  - Individual monster loot
  - Magic item distribution
  - Custom loot tables

- **Tactical Suggestions**
  - Terrain advantages
  - Monster tactics (smart, brute, pack tactics)
  - Environmental hazards
  - Map suggestions (gridless, 5ft grid, hex)

#### Technical Approach

```
┌─────────────────────────────────────────────────────────────┐
│                   Encounter Builder                          │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Party Setup:                                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Level: 5    Party Size: 4    Difficulty: Hard       │    │
│  │ [Calculate XP Budget] → 5,000 XP                   │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
│  Encounter Type:                                             │
│  ◉ Combat    ○ Social    ○ Puzzle    ○ Mixed              │
│                                                               │
│  Environment:                                                │
│  ○ Dungeon  ◉ Wilderness  ○ Urban  ○ Planar               │
│                                                               │
│  [Generate Encounter]                                         │
│                          │                                   │
│                          ▼                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Suggested Encounter:                                │    │
│  │                                                     │    │
│  │ 1 Young Red Dragon (CR 10)                          │    │
│  │ 4 Goblin Bosses (CR 2)                              │    │
│  │ 8 Goblins (CR 1/4)                                  │    │
│  │                                                     │    │
│  │ Total XP: 5,800 (Hard encounter)                    │    │
│  │                                                     │    │
│  │ Terrain: Rocky outcropping with elevation changes   │    │
│  │ Tactics: Dragon flies, goblins harass with bows     │    │
│  │ Loot: 1,200 gp, Potion of Flying, Dragon Scale Mail │    │
│  │                                                     │    │
│  │ [Regenerate] [Save to Session] [Print Stat Blocks] │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

#### API Endpoints

```
POST   /api/generate/encounter           # Generate encounter
POST   /api/generate/encounter/loot      # Generate loot
GET    /api/encounters/templates         # List encounter templates
```

---

### Feature 5: Campaign Knowledge Search (RAG)

**Goal:** Search entire campaign history with semantic understanding

#### Capabilities

- **Full-Text Search**
  - Search NPCs, locations, items, sessions, lore
  - Filter by entity type, date, tags
  - Fuzzy matching for typos

- **Semantic Search (v0.2.0)**
  - "Find all NPCs who hate dragons"
  - "Where did we find the magic sword?"
  - "What happened in the last session with the thieves' guild?"
  - Uses embeddings for concept matching

- **Relationship Queries**
  - Show all connections between entities
  - Visual graph of NPC relationships
  - Faction influence maps

#### Technical Approach

```
┌─────────────────────────────────────────────────────────────┐
│                    Campaign Search                            │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Search: [Find NPCs who know about the dragon cult ]        │
│           │                                                   │
│           ▼                                                   │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ Results:                                            │    │
│  │                                                     │    │
│  │ 1. Thorin Ironshield (Guard Captain)                │    │
│  │    - Has intelligence on cult hideout               │    │
│  │    - Last seen: Session 8                          │    │
│  │    - Relationship: Ally (if paid)                  │    │
│  │    [View Details] [Add to Session]                 │    │
│  │                                                     │    │
│  │ 2. Sylara Moonwhisper (Arcane Shopkeeper)           │    │
│  │    - Divination cult activities                     │    │
│  │    - Secret: Former cult member                     │    │
│  │    [View Details] [Add to Session]                 │    │
│  │                                                     │    │
│  │ 3. Vex (Street Urchin)                              │    │
│  │    - Saw cultists meeting in sewers                 │    │
│  │    - Will trade info for gold                       │    │
│  │    [View Details] [Add to Session]                 │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
│  Filters: [NPCs] [Locations] [Sessions] [Lore]               │
└─────────────────────────────────────────────────────────────┘
```

#### API Endpoints

```
GET    /api/search?q=query               # Full-text search
POST   /api/search/semantic              # Semantic search (v0.2.0)
GET    /api/campaigns/:id/graph          # Entity relationship graph
```

---

## 5. Technical Architecture

### Technology Stack

#### Frontend (DMLog Web App)

| Technology | Version | Purpose |
|------------|---------|---------|
| **Next.js** | 15 | React framework, SSR, API routes |
| **React** | 19 | UI framework |
| **TypeScript** | 5 | Type safety |
| **Tailwind CSS** | 4 | Styling |
| **shadcn/ui** | Latest | Component library |
| **Zustand** | Latest | State management |
| **React Query** | Latest | Server state, caching |
| **Monaco Editor** | Latest | Rich text editing |

#### Backend (SuperInstance Integration)

| Service | Port | Purpose |
|---------|------|---------|
| **DMLog API** | 3006 | Main application API (Next.js API routes) |
| **SwarmOrchestration** | 3000 | Multi-agent coordination (for complex generation tasks) |
| **LucidDreamer** | 4000 | Cognitive AI engine (NPC generation, insights) |
| **PostgreSQL** | 5432 | Primary database |
| **Redis** | 6379 | Caching, sessions |
| **MinIO/S3** | 9000 | Asset storage (images, audio) |

#### SuperInstance Packages

```json
{
  "dependencies": {
    "@superinstance/async": "^1.0.0",           // Retry, timeout, batching
    "@superinstance/chat": "^1.0.0",            // LLM abstraction (OpenAI, Anthropic)
    "@superinstance/memory": "^1.0.0",          // Memory system for campaign context
    "@superinstance/rag": "^1.0.0",             // Retrieval-augmented generation
    "@superinstance/storage": "^1.0.0",         // Storage abstraction (PostgreSQL, S3)
    "@superinstance/validate": "^1.0.0",        // Zod validation schemas
    "@superinstance/events": "^1.0.0",          // Event system
    "@superinstance/logger": "^1.0.0",          // Structured logging
    "@superinstance/config": "^1.0.0",          // Config management
    "@superinstance/tts": "^1.0.0",             // Text-to-speech (v0.3.0 for NPC voices)
    "@superinstance/asr": "^1.0.0"              // Speech recognition (v0.2.0 for transcription)
  }
}
```

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         DMLog.ai                                │
│                        (Next.js 15)                             │
│                        Port: 3006                               │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Frontend (React 19)                   │  │
│  │                                                          │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │  │
│  │  │ Campaign │  │ Session  │  │ Content  │  │ Search  │ │  │
│  │  │ Dashboard│  │ Editor   │  │ Generators│ │         │ │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │  │
│  └────────────────────────┬─────────────────────────────────┘  │
│                           │                                     │
│  ┌────────────────────────┴─────────────────────────────────┐  │
│  │                    API Routes (Next.js)                  │  │
│  │                                                          │  │
│  │  /api/campaigns/*    /api/sessions/*                    │  │
│  │  /api/characters/*   /api/generate/*                    │  │
│  │  /api/search/*       /api/auth/*                         │  │
│  └────────────────────────┬─────────────────────────────────┘  │
└───────────────────────────┼────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│ PostgreSQL    │   │ LucidDreamer  │   │ Swarm         │
│ (Campaign     │   │ (AI Content  │   │ Orchestration │
│  Data)        │   │  Generation) │   │ (Complex      │
│               │   │   Port: 4000  │   │  Tasks)       │
│ Port: 5432    │   │               │   │ Port: 3000    │
└───────────────┘   └───────────────┘   └───────────────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            ▼
                  ┌───────────────┐
                  │ Redis         │
                  │ (Cache,       │
                  │  Sessions)    │
                  │ Port: 6379    │
                  └───────────────┘
```

### Database Schema

```sql
-- Core Tables
CREATE TABLE campaigns (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id),
  name VARCHAR(255) NOT NULL,
  system VARCHAR(50), -- 'dnd5e', 'pathfinder2e', 'coc', etc.
  setting TEXT,
  tone VARCHAR(50), -- 'dark', 'heroic', 'humorous', etc.
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
  title VARCHAR(255),
  session_number INTEGER,
  date DATE,
  notes TEXT,
  summary TEXT, -- AI-generated summary
  xp_awarded INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE characters (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  type VARCHAR(20) NOT NULL CHECK (type IN ('pc', 'npc')),
  race VARCHAR(100),
  class VARCHAR(100),
  level INTEGER DEFAULT 1,
  description TEXT,
  personality TEXT,
  backstory TEXT,
  voice_notes TEXT,
  image_url TEXT,
  stats JSONB, -- System-specific stat block
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE locations (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
  parent_id UUID REFERENCES locations(id), -- Hierarchical
  name VARCHAR(255) NOT NULL,
  type VARCHAR(50), -- 'city', 'dungeon', 'wilderness', etc.
  description TEXT,
  image_url TEXT,
  metadata JSONB,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  type VARCHAR(50), -- 'weapon', 'armor', 'potion', 'treasure', etc.
  rarity VARCHAR(20),
  description TEXT,
  stats JSONB,
  owner_id UUID REFERENCES characters(id),
  location_id UUID REFERENCES locations(id),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE relationships (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
  source_type VARCHAR(50) NOT NULL, -- 'character', 'location', etc.
  source_id UUID NOT NULL,
  target_type VARCHAR(50) NOT NULL,
  target_id UUID NOT NULL,
  relationship_type VARCHAR(50), -- 'ally', 'enemy', 'family', etc.
  description TEXT,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Search Optimization
CREATE INDEX campaigns_user_id_idx ON campaigns(user_id);
CREATE INDEX sessions_campaign_id_idx ON sessions(campaign_id);
CREATE INDEX characters_campaign_id_idx ON characters(campaign_id);
CREATE INDEX locations_campaign_id_idx ON locations(campaign_id);
CREATE INDEX items_campaign_id_idx ON items(campaign_id);

-- Full-Text Search
CREATE INDEX characters_name_fts ON characters USING gin(to_tsvector('english', name));
CREATE INDEX locations_name_fts ON locations USING gin(to_tsvector('english', name));
CREATE INDEX sessions_notes_fts ON sessions USING gin(to_tsvector('english', notes));

-- RAG: Vector embeddings (v0.2.0)
-- ALTER TABLE characters ADD COLUMN embedding vector(1536);
-- CREATE INDEX characters_embedding_idx ON characters USING ivfflat(embedding vector_cosine_ops);
```

### AI Integration

#### LucidDreamer Integration

```typescript
// services/npc-generator.ts

import { Chat } from '@superinstance/chat';

export class NPCGenerator {
  private chat: Chat;

  constructor() {
    this.chat = new Chat({
      provider: 'anthropic', // or 'openai'
      apiKey: process.env.ANTHROPIC_API_KEY,
      model: 'claude-3-5-sonnet-20241022'
    });
  }

  async generateNPC(params: {
    race?: string;
    class?: string;
    level?: number;
    role?: string;
    campaignContext: {
      tone: string;
      system: string;
      existingNPCs: string[];
      worldRules: string;
    };
  }) {
    const prompt = this.buildPrompt(params);

    const response = await this.chat.complete({
      messages: [
        {
          role: 'system',
          content: `You are an expert TTRPG content generator. Create NPCs that are vivid, memorable, and consistent with the campaign's tone and rules.`
        },
        {
          role: 'user',
          content: prompt
        }
      ],
      temperature: 0.8,
      maxTokens: 2000
    });

    return this.parseNPCResponse(response.content);
  }

  private buildPrompt(params: any): string {
    return `
Generate a ${params.race || 'random'} ${params.class || 'random'} of level ${params.level || 1}.
Role: ${params.role || 'random'}

Campaign Context:
- System: ${params.campaignContext.system}
- Tone: ${params.campaignContext.tone}
- World Rules: ${params.campaignContext.worldRules}

Avoid duplicating these existing NPCs: ${params.campaignContext.existingNPCs.join(', ')}

Provide:
1. Name
2. Appearance
3. Personality traits (3-5)
4. Backstory (2-3 sentences)
5. Voice description
6. Secret or plot hook
7. Stat block (in ${params.campaignContext.system} format)

Respond in JSON format.
`;
  }

  private parseNPCResponse(content: string) {
    // Parse JSON response
    return JSON.parse(content);
  }
}
```

#### SwarmOrchestration Integration (v0.2.0)

For complex multi-step tasks like:
- Generating an entire dungeon (rooms, NPCs, loot, encounters)
- Creating a quest chain (multiple connected encounters)
- Simulating campaign consequences (if players do X, then Y happens)

```typescript
// services/complex-generator.ts

import { SwarmClient } from '@superinstance/swarm-client';

export class ComplexGenerator {
  private swarm: SwarmClient;

  constructor() {
    this.swarm = new SwarmClient({
      baseURL: 'http://localhost:3000'
    });
  }

  async generateDungeon(params: {
    level: number;
    theme: string;
    roomCount: number;
    campaignContext: any;
  }) {
    const task = {
      type: 'generate_dungeon',
      params,
      subtasks: [
        { type: 'generate_rooms', params: { count: params.roomCount } },
        { type: 'populate_encounters', params: { level: params.level } },
        { type: 'generate_loot', params: { tier: Math.ceil(params.level / 5) } },
        { type: 'add_npc_interactions', params: { theme: params.theme } }
      ]
    };

    const result = await this.swarm.executeTask(task);
    return result;
  }
}
```

---

## 6. Data Model

### Core Entities

#### Campaign

```typescript
interface Campaign {
  id: string;
  userId: string;
  name: string;
  system: TTRPGSystem; // 'dnd5e' | 'pathfinder2e' | 'coc' | 'shadowrun' | 'custom'
  setting: string;
  tone: CampaignTone; // 'dark' | 'heroic' | 'humorous' | 'mystery' | 'epic'
  description?: string;
  imageUrl?: string;
  createdAt: Date;
  updatedAt: Date;

  // Relations (loaded separately)
  sessions?: Session[];
  characters?: Character[];
  locations?: Location[];
  items?: Item[];
}
```

#### Session

```typescript
interface Session {
  id: string;
  campaignId: string;
  title: string;
  sessionNumber: number;
  date: Date;
  notes: string; // Rich text/Markdown
  summary?: string; // AI-generated
  xpAwarded: number;
  lootFound?: Item[];
  charactersPresent?: Character[]; // NPCs met, PCs present
  moments?: SessionMoment[]; // Highlighted moments
  tags: string[];
  createdAt: Date;
  updatedAt: Date;
}

interface SessionMoment {
  type: 'epic' | 'funny' | 'death' | 'critical' | 'roleplay';
  timestamp: string; // HH:MM timestamp
  description: string;
  characters?: string[]; // Character IDs involved
}
```

#### Character

```typescript
interface Character {
  id: string;
  campaignId: string;
  name: string;
  type: 'pc' | 'npc';
  race?: string;
  class?: string;
  level: number;
  description: string;
  appearance?: string;
  personality?: string; // Traits, ideals, bonds, flaws
  backstory?: string;
  voiceNotes?: string; // Voice description for roleplay
  secrets?: string[]; // Hidden information from players
  stats?: StatBlock; // System-specific stat block
  imageUrl?: string;
  tokenUrl?: string; // VTT token
  relationships?: Relationship[];
  createdAt: Date;
  updatedAt: Date;
}

interface StatBlock {
  system: string; // 'dnd5e', etc.
  format: 'compact' | 'full';
  data: any; // System-specific stats
}

interface Relationship {
  targetId: string;
  targetType: 'character' | 'location' | 'faction';
  type: 'ally' | 'enemy' | 'family' | 'romantic' | 'mentor' | 'rival';
  strength: number; // 1-5
  description: string;
}
```

#### Location

```typescript
interface Location {
  id: string;
  campaignId: string;
  parentId?: string; // For hierarchical locations
  name: string;
  type: LocationType;
  description: string;
  atmosphere?: string;
  image?: string;
  map?: string; // VTT map URL
  npcs?: string[]; // NPC IDs present here
  items?: string[]; // Item IDs found here
  secrets?: string[];
  metadata?: {
    size?: string; // 'hamlet' | 'village' | 'city' | 'metropolis'
    population?: number;
    government?: string;
    economy?: string;
    customs?: string[];
  };
  createdAt: Date;
  updatedAt: Date;
}

type LocationType =
  | 'world'
  | 'region'
  | 'city'
  | 'district'
  | 'building'
  | 'dungeon'
  | 'wilderness'
  | 'plane';
```

#### Item

```typescript
interface Item {
  id: string;
  campaignId: string;
  name: string;
  type: ItemType;
  rarity?: 'common' | 'uncommon' | 'rare' | 'very-rare' | 'legendary';
  description: string;
  stats?: any; // System-specific stats
  owner?: string; // Character ID
  location?: string; // Location ID
  history?: string[]; // Ownership history
  imageUrl?: string;
  createdAt: Date;
  updatedAt: Date;
}

type ItemType =
  | 'weapon'
  | 'armor'
  | 'potion'
  | 'scroll'
  | 'wand'
  | 'ring'
  | 'wondrous-item'
  | 'treasure'
  | 'tool'
  | 'vehicle';
```

### System-Specific Extensions

#### D&D 5E Character

```typescript
interface DND5ECharacter extends Character {
  stats: {
    system: 'dnd5e';
    abilityScores: {
      strength: number;
      dexterity: number;
      constitution: number;
      intelligence: number;
      wisdom: number;
      charisma: number;
    };
    proficiencyBonus: number;
    savingThrows: string[];
    skills: string[];
    hp: number;
    ac: number;
    speed: number;
    attacks?: Attack[];
    features?: string[];
  };
}

interface Attack {
  name: string;
  type: 'melee' | 'ranged';
  toHit: string;
    damage: string;
    damageType: string;
  }
}
```

#### Pathfinder 2E Character

```typescript
interface PF2ECharacter extends Character {
  stats: {
    system: 'pathfinder2e';
    abilityScores: {
      strength: number;
      dexterity: number;
      constitution: number;
      intelligence: number;
      wisdom: number;
      charisma: number;
    };
    proficiency: number;
    hp: number;
    ac: number;
    speed: number;
    strikes?: Strike[];
    feats?: string[];
  };
}
```

---

## 7. User Stories

### Epic 1: Campaign Preparation

#### US-1.1: As a DM, I want to create a campaign with basic settings

**Acceptance Criteria:**
- Can create campaign with name, system, setting, tone
- Campaign appears in dashboard
- Can add description and cover image
- Can select from predefined templates (Fantasy, Sci-Fi, Horror, Modern)

#### US-1.2: As a DM, I want to generate NPCs on demand

**Acceptance Criteria:**
- Can input race, class, level, role
- Generate button creates full NPC with stats, personality, backstory
- NPC is automatically added to campaign
- Can regenerate if unsatisfied
- Can edit before saving

#### US-1.3: As a DM, I want to build balanced encounters quickly

**Acceptance Criteria:**
- Input party level and size
- Select difficulty (easy/medium/hard/deadly)
- Generate returns appropriate monsters
- XP budget is calculated correctly
- Can adjust monster selection
- Encounter shows loot and tactical suggestions

### Epic 2: Session Management

#### US-2.1: As a DM, I want to take structured notes during sessions

**Acceptance Criteria:**
- Quick-create session with date and title
- Rich text editor with markdown support
- Quick-insert NPCs, locations, items from campaign
- Timestamp notes for reference
- Can tag sections (combat, roleplay, exploration)
- Notes auto-save

#### US-2.2: As a DM, I want AI to summarize my session notes

**Acceptance Criteria:** (v0.2.0)
- Generate summary button creates concise session summary
- Summary highlights: XP awarded, loot found, NPCs met, plot hooks
- Can edit summary before saving
- Summary is searchable

#### US-2.3: As a DM, I want to search my campaign history

**Acceptance Criteria:**
- Search bar in top navigation
- Search returns NPCs, locations, items, sessions, notes
- Can filter by entity type
- Click result to view details
- Can add directly to current session

### Epic 3: Player Collaboration

#### US-3.1: As a DM, I want to share my campaign with players

**Acceptance Criteria:** (v0.2.0)
- Can invite players by email
- Players have read-only access to campaign
- Can choose what players see (hide secrets, spoilers)
- Players can view their character sheets

#### US-3.2: As a player, I want to track my character

**Acceptance Criteria:** (v0.3.0)
- Can view my character sheet
- Can see my inventory and stats
- Can add private notes (DM can't see)
- DM updates reflect in real-time

### Epic 4: Content Generation

#### US-4.1: As a DM, I want AI to generate location details

**Acceptance Criteria:**
- Input location type (city, dungeon, tavern)
- Generate returns name, description, NPCs, plot hooks
- Matches campaign tone and setting
- Can regenerate or edit

#### US-4.2: As a DM, I want AI to generate quest hooks

**Acceptance Criteria:**
- Input desired quest type (investigation, combat, social)
- Generate returns quest premise, antagonist, stakes, reward
- Connected to existing NPCs and locations
- Can accept or regenerate

### Epic 5: Session Recording

#### US-5.1: As a DM, I want to record and transcribe sessions

**Acceptance Criteria:** (v0.2.0)
- Can start/stop recording during session
- Audio uploads to cloud storage
- Transcription starts automatically after upload
- Transcription appears in session notes
- Can edit transcription

#### US-5.2: As a DM, I want to extract NPCs from session transcripts

**Acceptance Criteria:** (v0.3.0)
- Can highlight NPC names in transcript
- AI extracts NPC details from context
- Creates NPC entry with name, personality, quotes
- Can save to campaign or discard

---

## 8. Differentiation

### Competitors

| Product | Strengths | Weaknesses |
|---------|-----------|------------|
| **Roll20** | VTT, combat, large user base | No campaign management, no AI |
| **Foundry VTT** | Moddable, one-time purchase | Complex setup, no campaign tools |
| **DnDBeyond** | Official D&D content, character sheets | Subscription-heavy, no AI, walled garden |
| **Notion** | Flexible, good docs | Not TTRPG-specific, no AI generation |
| **Obsidian Portal** | Campaign wiki, public sharing | Dated UI, no AI, limited organization |
| **KoboldPlus** | AI NPCs, VTT integration | New, limited features, subscription |
| **DungeonAI** | AI dungeon master | Solo play only, no campaign management |
| **RPGGO** | VTT + AI | Chinese-focused, limited Western systems |

### DMLog's Unique Advantages

#### 1. AI-Native Campaign Management

> **"The only platform that combines deep organization with creative AI"**

- Competitors are either **VTTs** (Roll20, Foundry) or **wikis** (Obsidian Portal)
- DMLog is the first **AI-native campaign manager**
- Not just a VTT add-on—a standalone platform for DM creativity

#### 2. System Agnostic

> **"One tool for every game system"**

- Most competitors focus on D&D 5E
- DMLog supports: D&D 5E, Pathfinder 2E, Call of Cthulhu, Shadowrun, Vampire: The Masquerade, homebrew
- AI adapts to system rules, tone, and mechanics

#### 3. Session Transcription & Review

> **"Never lose a session again"**

- No competitor offers transcription
- DMLog records, transcribes, and indexes sessions
- Search across your entire campaign history
- Extract NPCs, locations, items from past sessions

#### 4. Adaptive Tone Matching

> **"AI that learns your campaign's voice"**

- Competitors use generic AI prompts
- DMLog learns from your campaign: tone, style, world rules, existing NPCs
- Generated content feels consistent and personalized

#### 5. Separation of Concerns

> **"Campaign management without VTT lock-in"**

- VTTs (Roll20, Foundry) focus on tactical combat
- DMLog focuses on **preparation, organization, creativity**
- Use DMLog alongside any VTT (or offline games)
- Export data to VTT formats

#### 6. Freemium with Fair AI Limits

> **"Powerful AI without breaking the bank"**

- Free tier: Basic campaign management, community content
- Pro tier ($9/mo): Unlimited AI generation, transcription
- No hidden costs, no per-token fees

### Why Choose DMLog Over Competitors?

| Use Case | Best Tool | Why |
|----------|-----------|-----|
| **Online tactical combat** | Roll20/Foundry | Purpose-built VTT |
| **Offline game prep** | DMLog | Campaign management, AI generation |
| **Session recording** | DMLog | Only platform with transcription |
| **Multi-system campaigns** | DMLog | System-agnostic tools |
| **Official character sheets** | DnDBeyond | Licensed content, integrations |
| **Campaign wiki** | DMLog | Better organization than Obsidian Portal |
| **Solo RPG with AI DM** | DungeonAI | Purpose-built solo experience |
| **AI content generation** | DMLog | More sophisticated than KoboldPlus |

**DMLog is not a VTT—it's a companion to VTTs.** DMLog helps you prepare, run, and review campaigns. Use it with Roll20, Foundry, or at a physical table.

---

## 9. Development Roadmap

### v0.1.0 - MVP (3 months)

**Goal:** Prove core value with AI campaign management

#### Features

- ✅ Campaign CRUD (create, read, update, delete)
- ✅ Character management (PCs and NPCs)
- ✅ Location catalog (hierarchical)
- ✅ Item database
- ✅ Session logging with rich text notes
- ✅ AI NPC generator (basic)
- ✅ Encounter builder (XP budgeting)
- ✅ Full-text search
- ✅ User authentication (email/password)
- ✅ Campaign sharing (read-only links)

#### Tech Stack

- Frontend: Next.js 15, React 19, TypeScript 5, Tailwind CSS 4
- Backend: Next.js API routes, PostgreSQL
- AI: LucidDreamer integration (NPC generation)
- Auth: NextAuth.js
- Deployment: Vercel (frontend), Railway (backend)

#### Success Metrics

- 1,000 beta users
- 50 campaigns created
- 10,000 NPCs generated
- 70%+ user retention after 1 week
- Average 3+ sessions logged per campaign

---

### v0.2.0 - Transcription & Search (2 months)

**Goal:** Add session recording and semantic search

#### Features

- Audio recording (in-browser, upload later)
- Automatic transcription (ASR integration)
- AI session summaries
- Semantic search (RAG with embeddings)
- Player collaboration (invite by email)
- Campaign templates
- Export to VTT formats (Roll20, Foundry)

#### Tech Additions

- ASR: @superinstance/asr (OpenAI Whisper)
- RAG: @superinstance/rag with vector embeddings (pgvector)
- Real-time: WebRTC for audio
- File storage: MinIO/S3

#### Success Metrics

- 5,000 users
- 1,000 sessions transcribed
- 500+ searches per day
- 40% DAU/MAU ratio

---

### v0.3.0 - Voice & VTT Integration (2 months)

**Goal:** Add NPC voice synthesis and deep VTT integration

#### Features

- NPC voice synthesis (TTS integration)
- Voice persona matching (text → voice characteristics)
- Roll20 integration (import/export)
- Foundry VTT module (bi-directional sync)
- Mobile app (iOS/Android) for players
- Character sheet integration (DnDBeyond import)
- Dice roller (in-app)

#### Tech Additions

- TTS: @superinstance/tts (ElevenLabs, OpenAI)
- Mobile: React Native or Progressive Web App
- Roll20 API: Character import/export
- Foundry: Module development

#### Success Metrics

- 10,000 users
- 5,000 NPCs with voices
- 1,000 VTT integrations
- 50% paid conversion rate

---

### v0.4.0 - Advanced AI (2 months)

**Goal:** Leverage SwarmOrchestration for complex generation

#### Features

- Dungeon generator (entire floor plans with encounters)
- Quest chain generator (multi-session arcs)
- Campaign consequence simulation (if players do X...)
- Relationship graph visualization
- Faction dynamics simulation
- World-building assistant (create entire regions)
- Image generation integration (Midjourney, DALL-E prompts)

#### Tech Additions

- SwarmOrchestration: Multi-agent coordination
- LucidDreamer: Advanced abstraction
- Knowledge graphs: Network analysis
- Image generation: Prompt engineering

#### Success Metrics

- 20,000 users
- 1,000 generated dungeons
- 500 quest chains
- 30% MRR growth

---

### v0.5.0 - Solo Mode & Marketplace (3 months)

**Goal:** Enable solo play and community content

#### Features

- AI Dungeon Master mode (solo play)
- Scenario marketplace (publish/sell campaigns)
- Community content (free templates, NPCs, locations)
- Subscription tools (monetize your campaigns)
- Streaming integration (OBS overlays, chat commands)
- Discord bot (session reminders, dice rolling)

#### Tech Additions

- Payment processing: Stripe
- Content delivery: CDN
- Discord bot: Discord.js
- OBS: WebSocket plugin

#### Success Metrics

- 50,000 users
- 1,000 marketplace items
- 500 creators earning money
- $10k MRR

---

### Future Roadmap (v1.0+)

#### v1.0.0 - Platform Maturity

- Multi-language support (Spanish, French, German)
- Advanced analytics (campaign insights, player engagement)
- Campaign migration (import from other tools)
- API for third-party integrations
- White-label licensing (for game stores)

#### v1.5.0 - Virtual Tabletop

- Optional built-in VTT (lightweight, focused on roleplay)
- 2D map tools (with Fog of War)
- Token movement and initiative tracking
- Integrated dice roller with animations

#### v2.0.0 - Metaverse

- 3D virtual tabletop (WebXR)
- Spatial audio for voice chat
- VR support (Meta Quest)
- AR tabletop (project on physical table)

---

## 10. Monetization Strategy

### Pricing Model

| Tier | Price | Features |
|------|-------|----------|
| **Free** | $0 | - 1 active campaign<br>- 5 AI generations/month<br>- Manual session notes<br>- Full-text search<br>- Community content |
| **Pro DM** | $9/mo or $90/yr | - Unlimited campaigns<br>- Unlimited AI generation<br>- Session transcription (10 hrs/mo)<br>- Semantic search<br>- NPC voice synthesis (5 hrs/mo)<br>- Export to VTTs<br>- Priority support |
| **Campaign Studio** | $19/mo or $190/yr | - Everything in Pro<br>- Unlimited transcription<br>- Unlimited voice synthesis<br>- Player collaboration (5 players)<br>- Dungeon generator<br>- Quest chain generator<br>- Image generation prompts<br>- Custom branding (streamers) |
| **Game Store** | $49/mo | - Everything in Studio<br>- White-label branding<br>- 50 player seats<br>- Organized play management<br>- Analytics dashboard<br>- API access |

### Revenue Projections

| Milestone | Users | Paying % | MRR |
|-----------|-------|----------|-----|
| Launch (v0.1.0) | 1,000 | 5% | $450 |
| v0.2.0 | 5,000 | 8% | $3,600 |
| v0.3.0 | 10,000 | 10% | $9,000 |
| v0.4.0 | 20,000 | 12% | $21,600 |
| v0.5.0 | 50,000 | 15% | $67,500 |
| v1.0.0 | 100,000 | 15% | $135,000 |

### Revenue Streams

1. **Subscription Fees** (Primary)
   - 70% Pro DM, 20% Campaign Studio, 10% Game Store

2. **Marketplace** (v0.5.0+)
   - 30% commission on community content sales
   - Estimated: $5k/mo at launch, $20k/mo at scale

3. **Enterprise Licensing** (v1.0.0+)
   - White-label for game stores, streaming studios
   - $500-2,000/yr per license

4. **API Usage** (v1.0.0+)
   - Pay-per-call for third-party integrations
   - Usage-based pricing

5. **Affiliate Program**
   - 20% commission for referrals to DnDBeyond, Roll20, etc.
   - Drive traffic to VTTs and content platforms

### Cost Analysis

| Expense | Monthly Cost |
|---------|--------------|
| Infrastructure (Vercel, Railway, PostgreSQL) | $500 |
| AI API Costs (OpenAI, Anthropic) | $2,000 |
| Storage (S3, CDN) | $200 |
| Support (1 FT at v0.3.0) | $4,000 |
| Marketing (ads, content creators) | $3,000 |
| **Total (v0.3.0)** | **$9,700** |
| **Total (v1.0.0, scaled)** | **$25,000** |

### Break-Even Analysis

- **Break-even at v0.3.0:** ~200 Pro DM subscribers
- **Break-even at v1.0.0:** ~500 Pro DM subscribers

### Unit Economics

- **CAC (Customer Acquisition Cost):** $15 (content marketing, SEO)
- **LTV (Lifetime Value):** $108 (Pro DM, 12-month avg)
- **LTV:CAC Ratio:** 7.2x (healthy)

---

## 11. Success Metrics

### North Star Metric

**Weekly Active Campaigns** – Number of campaigns with at least one session logged or AI generation per week.

### Key Performance Indicators (KPIs)

#### Acquisition

| Metric | Target (v0.1.0) | Target (v1.0.0) |
|--------|-----------------|-----------------|
| Signups | 1,000/month | 10,000/month |
| Traffic | 5,000 visits/mo | 100,000 visits/mo |
| Conversion (Free → Paid) | 5% | 15% |

#### Engagement

| Metric | Target (v0.1.0) | Target (v1.0.0) |
|--------|-----------------|-----------------|
| DAU/MAU | 20% | 40% |
| Sessions per campaign | 3+ | 10+ |
| AI generations per user | 5/week | 20/week |
| Time in app | 20 min/session | 45 min/session |

#### Retention

| Metric | Target (v0.1.0) | Target (v1.0.0) |
|--------|-----------------|-----------------|
| Day 7 retention | 40% | 60% |
| Day 30 retention | 20% | 40% |
| Day 90 retention | 10% | 25% |

#### Quality

| Metric | Target (v0.1.0) | Target (v1.0.0) |
|--------|-----------------|-----------------|
| NPS (Net Promoter Score) | 40+ | 60+ |
| Bug report rate | <5% users | <2% users |
| API error rate | <1% | <0.1% |
| Uptime | 99.5% | 99.9% |

#### Revenue

| Metric | Target (v0.3.0) | Target (v1.0.0) |
|--------|-----------------|-----------------|
| MRR | $9,000 | $135,000 |
| ARPU (Avg Revenue Per User) | $1.80 | $2.70 |
| Churn rate | 8%/mo | 5%/mo |

### Success Milestones

- **MVP Success (v0.1.0):** 1,000 users, 70% retention, positive NPS
- **Product-Market Fit (v0.3.0):** 10,000 users, $10k MRR, organic growth >50%
- **Scale (v1.0.0):** 100,000 users, $100k MRR, marketplace revenue >$20k/mo

---

## 12. Next Steps

### Immediate Actions (Week 1-2)

1. **Repository Setup**
   - Create GitHub repo: https://github.com/SuperInstance/DMLog
   - Initialize Next.js 15, TypeScript 5, Tailwind CSS 4
   - Set up PostgreSQL database (Railway or Supabase)
   - Configure environment variables

2. **Core Architecture**
   - Install SuperInstance packages (@superinstance/async, chat, memory, rag, etc.)
   - Set up LucidDreamer connection (localhost:4000)
   - Create database schema (migrations)
   - Implement authentication (NextAuth.js)

3. **MVP Feature Prioritization**
   - Campaign CRUD (foundational)
   - Character management (high value)
   - Session logging (core workflow)
   - AI NPC generator (wow factor)
   - Encounter builder (differentiator)

### Development Phase 1 (Week 3-8: MVP)

**Sprint 1 (Week 3-4): Foundation**
- Database schema and migrations
- Authentication and user management
- Campaign CRUD API
- Basic UI framework (shadcn/ui)

**Sprint 2 (Week 5-6): Core Features**
- Character management (PCs and NPCs)
- Location catalog (hierarchical)
- Item database
- Session logging (rich text notes)

**Sprint 3 (Week 7-8): AI Integration**
- LucidDreamer integration
- AI NPC generator
- Encounter builder (XP budgeting)
- Full-text search

**Sprint 4 (Week 9-10): Polish & Launch**
- Beta testing with 50 DMs
- Bug fixes and performance tuning
- Documentation and onboarding
- Public launch announcement

### Phase 2 Planning (Week 11+)

- **Voice Transcription:** Integrate @superinstance/asr
- **Semantic Search:** Add RAG with vector embeddings
- **Player Collaboration:** Real-time features, invite system
- **VTT Integration:** Roll20 and Foundry connectors

### Go-to-Market Strategy

#### Pre-Launch (Week 8-10)

1. **Content Marketing**
   - Blog series: "The Future of TTRPG Campaign Management"
   - YouTube demos: NPC generation, encounter building
   - Reddit engagement (r/DMAcademy, r/dndnext)

2. **Influencer Outreach**
   - Partner with TTRPG content creators (Dungeon Dudes, Bob World Builder)
   - Sponsor podcasts (The Dungeoncast, Dragon Talk)
   - Provide early access to streamers

3. **Community Building**
   - Discord server for beta testers
   - Feature voting board (feedback prioritization)
   - Content creator program (exclusive features)

#### Launch (Week 10)

1. **Product Hunt Launch**
   - Prepare demo video, screenshots
   - Schedule launch day
   - Engage with comments

2. **Social Media Push**
   - Twitter threads (AI-generated NPCs examples)
   - TikTok/Reels (quick generation demos)
   - LinkedIn (B2B game store angle)

3. **Direct Outreach**
   - Email DMs from mailing list
   - Cold outreach to game stores
   - Offer free Pro trials

#### Post-Launch (Week 11+)

1. **Retention Focus**
   - Onboarding emails (5-day campaign creation course)
   - In-app prompts (generate your first NPC)
   - Weekly feature highlights

2. **Growth Loop**
   - Referral program (invite 3 friends = 1 month Pro)
   - User-generated content showcase
   - Social proof (testimonials, case studies)

3. **Monetization Optimization**
   - Feature gating (AI limits trigger upgrades)
   - Annual discount offers
   - Bundle with SuperInstance products

---

## Appendix

### A. Technical Decisions

#### Why Next.js 15?

- Server components for performance
- API routes (no separate backend needed for MVP)
- Built-in image optimization
- Great developer experience
- Easy deployment to Vercel

#### Why PostgreSQL?

- Relational data (campaigns, characters, sessions)
- Full-text search built-in
- JSON support for flexible stats
- Vector extension (pgvector) for RAG
- Strong ecosystem (Prisma, Drizzle)

#### Why LucidDreamer for AI?

- Purpose-built for creative generation
- 5-level abstraction (richer NPCs)
- Tone matching (learns campaign voice)
- System-specific knowledge (D&D, Pathfinder, etc.)
- Already integrated into SuperInstance ecosystem

#### Why Not Roll20/Foundry Integration First?

- High effort, low differentiation
- Locks us to specific VTTs
- DMs already use VTTs—we want to be the prep tool, not combat tracker
- VTT integration comes later (v0.3.0)

### B. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| **AI quality is poor** | High | Extensive prompt engineering, user feedback loops, fine-tuning |
| **VTTs add AI features** | Medium | Move fast, establish user base, focus on organization (not just AI) |
| **Copyright issues with SRD** | Medium | Use Open Gaming License, user-generated content focus |
| **High API costs** | High | Free tier limits, caching, cheaper model options (Claude Haiku) |
| **User churn** | Medium | Onboarding flows, feature discovery, community building |
| **Data privacy concerns** | Low | Clear privacy policy, data export, GDPR compliance |

### C. Open Questions

1. **Homebrew System Support**
   - How much flexibility to provide for custom systems?
   - MVP: User-defined tags and fields
   - v0.4.0: Custom stat blocks, system templates

2. **Monetization Timing**
   - When to start charging?
   - Decision: Charge from day 1 (freemium), but generous free tier

3. **VTT Integration Scope**
   - Deep integration (bi-directional sync) or simple import/export?
   - MVP: Import/export
   - v0.3.0: Deep integration with Roll20, Foundry

4. **Mobile Strategy**
   - Native apps or responsive PWA?
   - MVP: Responsive web app
   - v0.3.0: PWA with offline support
   - v1.0.0: Native iOS/Android (if demand justifies)

### D. References & Inspiration

- **Obsidian Portal:** Campaign wiki concept
- **Notion:** Flexible database and rich text
- **KoboldPlus:** AI NPC generation
- **DungeonAI:** Solo play with AI DM
- **Roll20/Foundry:** VTT standards to match
- **Roam Research:** Bidirectional linking for relationships
- **Midjourney:** AI-generated portrait prompts

---

## Conclusion

DMLog is positioned to be the **first AI-native, system-agnostic campaign management platform** for tabletop RPGs. By combining deep organization with creative AI, we solve real DM pain points: campaign fragmentation, content creation fatigue, and session amnesia.

The MVP (v0.1.0) focuses on core value: campaign organization, AI NPC generation, encounter building, and session logging. Subsequent releases add transcription, semantic search, VTT integration, and advanced AI generation.

With a freemium model targeting $9-19/mo subscriptions, DMLog can reach $135k MRR within 18 months by capturing 15% of 100,000 users. The market opportunity is large ($15B TTRPG industry, 50M D&D players), and no competitor offers our combination of features.

**The time is now.** TTRPGs are mainstream, AI is accepted, and DMs are hungry for tools that reduce prep time and spark creativity. DMLog will be the platform that transforms campaign management from a chore into a joy.

---

**Status:** Ready for development
**Next Action:** Initialize repository, set up infrastructure, begin Sprint 1

*Last Updated: 2026-01-01*
*Author: SuperInstance Team*
