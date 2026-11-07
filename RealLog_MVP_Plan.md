# RealLog MVP Plan
## AI-Powered Video Creation Platform

**Version:** 0.1.0
**Last Updated:** 2026-01-01
**Domain:** https://reallog.ai
**Repository:** https://github.com/SuperInstance/RealLog

---

## Executive Summary

### Problem Statement

Content creators face three critical challenges in video production:

1. **Multi-source complexity**: Streamers and YouTubers need to capture from screen, camera, microphone, and system audio simultaneously, then coordinate these sources in editing
2. **Time-intensive editing**: Traditional NLEs (Non-Linear Editors) require 10-30 hours of manual editing per 10-minute video
3. **AI fragmentation**: AI tools exist (auto-captions, silence removal, highlight detection) but are scattered across different apps with poor integration

### Solution

RealLog is a browser-based video editing platform that combines:

- **Multi-source capture** (screen, camera, audio) in a unified timeline
- **AI-assisted editing** (silence removal, highlight detection, auto-captions)
- **Intelligent model routing** (cost-optimized AI processing via SwarmOrchestration)
- **Real-time collaboration** (multi-user editing sessions)

### Target Users

| Segment | Pain Points | RealLog Solution |
|---------|-------------|------------------|
| **YouTubers** (1M+ subs) | Editing bottleneck | AI highlight detection, auto-cuts |
| **Streamers** (Twitch/YouTube) | VOD editing takes days | One-click silence removal, clip extraction |
| **Course Creators** | Multi-source sync issues | Automatic camera/screen alignment |
| **Social Media Teams** | Rapid turnaround needed | Template-based editing, batch export |

### Market Opportunity

- **TAM**: $4.2B (video editing software market)
- **SAM**: $800M (web-based video editing)
- **SOM**: $50M (AI-assisted editing, growing 40% YoY)

Competitors:
- **Descript**: $44M ARR, strong in podcasts but weak in multi-source video
- **CapCut**: Mobile-first, limited desktop features
- **Premiere/DaVinci**: Desktop-only, steep learning curve

**Differentiation**: Browser-based, multi-source capture, AI-assisted editing, cost-optimized processing

---

## Core MVP Features (v0.1.0)

### Feature 1: Multi-Source Video Capture

**Goal**: Capture and synchronize multiple video/audio sources in browser

**User Story**:
> As a streamer, I want to record my screen, camera, and microphone simultaneously so that I can create engaging content with multiple angles.

**Implementation**:

```typescript
// Capture coordinator using MediaRecorder API
class MultiSourceCapture {
  private sources: Map<string, MediaStream> = new Map();
  private recorders: Map<string, MediaRecorder> = new Map();

  async startCapture(config: CaptureConfig) {
    // Screen capture
    if (config.screen) {
      const screenStream = await navigator.mediaDevices.getDisplayMedia({
        video: { cursor: 'always' },
        audio: true
      });
      this.sources.set('screen', screenStream);
    }

    // Camera capture
    if (config.camera) {
      const cameraStream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'user', width: 1280, height: 720 },
        audio: false
      });
      this.sources.set('camera', cameraStream);
    }

    // Microphone capture
    if (config.microphone) {
      const micStream = await navigator.mediaDevices.getUserMedia({
        audio: { echoCancellation: true, noiseSuppression: true }
      });
      this.sources.set('microphone', micStream);
    }

    // Start synchronized recording
    this.startRecording();
  }

  private startRecording() {
    const timestamp = Date.now();

    this.sources.forEach((stream, sourceId) => {
      const recorder = new MediaRecorder(stream, {
        mimeType: 'video/webm;codecs=vp9',
        videoBitsPerSecond: 5000000
      });

      recorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          this.saveChunk(sourceId, timestamp, event.data);
        }
      };

      recorder.start(1000); // Chunk every second
      this.recorders.set(sourceId, recorder);
    });
  }
}
```

**MVP Scope**:
- [x] Screen capture (full screen, window, browser tab)
- [x] Camera capture (user-facing)
- [x] Microphone capture (with noise suppression)
- [x] System audio capture
- [x] Synchronized timestamps across sources
- [ ] Multiple camera angles (v0.2.0)
- [ ] External device input (v0.2.0)

**Technical Constraints**:
- Browser compatibility: Chrome 90+, Edge 90+, Safari 15+
- Storage: IndexedDB for chunks (2GB limit), fallback to FS API
- Duration: Max 2 hours per recording (MVP)

---

### Feature 2: Timeline-Based Editor

**Goal**: Provide intuitive, frame-accurate editing interface

**User Story**:
> As a content creator, I want to drag and drop clips on a timeline so that I can arrange my video visually without manual coding.

**Implementation**:

```typescript
// Timeline using React DnD + Canvas
interface TimelineState {
  tracks: Track[];
  playhead: number; // Frame position
  duration: number; // Total frames
  zoom: number; // Frames per pixel
}

interface Track {
  id: string;
  type: 'video' | 'audio' | 'text';
  clips: Clip[];
  height: number;
  locked: boolean;
}

interface Clip {
  id: string;
  sourceId: string;
  startFrame: number;
  endFrame: number;
  offset: number; // In-point on source
  filters: Filter[];
}

const TimelineEditor: React.FC<TimelineProps> = ({ state, onUpdate }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const renderTimeline = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    const { tracks, playhead, zoom, duration } = state;

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Render tracks
    let trackY = 0;
    tracks.forEach(track => {
      // Draw track background
      ctx.fillStyle = track.locked ? '#2a2a2a' : '#1a1a1a';
      ctx.fillRect(0, trackY, canvas.width, track.height);

      // Draw clips
      track.clips.forEach(clip => {
        const x = clip.startFrame / zoom;
        const width = (clip.endFrame - clip.startFrame) / zoom;

        // Clip thumbnail
        ctx.fillStyle = '#3b82f6';
        ctx.fillRect(x, trackY + 4, width, track.height - 8);

        // Clip name
        ctx.fillStyle = '#fff';
        ctx.font = '12px sans-serif';
        ctx.fillText(clip.id, x + 4, trackY + 20);
      });

      trackY += track.height;
    });

    // Draw playhead
    const playheadX = playhead / zoom;
    ctx.strokeStyle = '#ef4444';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(playheadX, 0);
    ctx.lineTo(playheadX, canvas.height);
    ctx.stroke();

  }, [state]);

  return <canvas ref={canvasRef} width={1920} height={600} />;
};
```

**MVP Scope**:
- [x] Multi-track timeline (video, audio, text overlays)
- [x] Drag-and-drop clip arrangement
- [x] Trimming clips (start/end points)
- [x] Playhead scrubbing
- [x] Keyboard shortcuts (space=play, ←→=frame step)
- [x] Undo/redo stack
- [ ] Transitions (v0.2.0)
- [ ] Keyframe animation (v0.3.0)

**Performance Targets**:
- 60fps rendering with 10 tracks
- <100ms latency from scrub to preview update
- Handle 1080p video at 30fps

---

### Feature 3: AI-Assisted Editing Suggestions

**Goal**: Automate repetitive editing tasks with AI

**User Story**:
> As a YouTuber, I want AI to suggest removing silence and highlighting key moments so that I can edit a 1-hour stream into a 10-minute video in minutes instead of hours.

**Architecture**:

```typescript
// AI processing pipeline using SwarmOrchestration
class AIEditingPipeline {
  private swarmClient: SwarmClient;

  async detectSilence(videoId: string): Promise<SilenceSegment[]> {
    // Route to cheapest model for audio analysis
    const task: SwarmTask = {
      type: 'audio_analysis',
      model: 'whisper-tiny', // Cost-optimized
      input: {
        videoId,
        threshold: -40dB, // Silence threshold
        minDuration: 1000 // ms
      }
    };

    const result = await this.swarmClient.execute(task);

    return result.segments.map(seg => ({
      start: seg.start,
      end: seg.end,
      confidence: seg.confidence
    }));
  }

  async detectHighlights(videoId: string): Promise<HighlightSegment[]> {
    // Multi-stage AI pipeline
    // Stage 1: Audio transcription (cheap model)
    const transcript = await this.swarmClient.execute({
      type: 'transcription',
      model: 'whisper-base',
      input: { videoId }
    });

    // Stage 2: Highlight detection (mid-tier model)
    const highlights = await this.swarmClient.execute({
      type: 'highlight_detection',
      model: 'claude-haiku-3.5',
      input: {
        transcript: transcript.text,
        audioFeatures: transcript.features,
        criteria: ['laughter', 'applause', 'energy spikes']
      }
    });

    return highlights.segments;
  }

  async generateCaptions(videoId: string): Promise<Caption[]> {
    const captions = await this.swarmClient.execute({
      type: 'caption_generation',
      model: 'whisper-medium',
      input: { videoId, language: 'en' }
    });

    return captions.segments.map(seg => ({
      start: seg.start,
      end: seg.end,
      text: seg.text,
      confidence: seg.confidence
    }));
  }
}
```

**MVP Scope**:
- [x] Silence detection and removal (audio-based)
- [x] Auto-caption generation (English only)
- [x] Highlight detection (energy + transcription)
- [x] Smart cut suggestions
- [ ] Face detection and tracking (v0.2.0)
- [ ] Scene detection (v0.3.0)
- [ ] Auto-b-roll insertion (v0.3.0)

**Cost Optimization**:
```yaml
# Model routing configuration (from Content Studio patterns)
silence_detection:
  model: whisper-tiny
  cost_per_hour: $0.05
  accuracy: 85%

caption_generation:
  model: whisper-base
  cost_per_hour: $0.15
  accuracy: 92%

highlight_detection:
  model: claude-haiku-3.5
  cost_per_hour: $0.25
  accuracy: 88%

# Tiered processing:
# 1. Quick pass (cheap model): $0.05, 85% accuracy
# 2. Refinement pass (mid model): $0.20, 95% accuracy
# Total: $0.25 per hour of video vs $2.00 all-cloud
```

---

### Feature 4: Export to Common Formats

**Goal**: Export edited video to web-standard formats

**User Story**:
> As a content creator, I want to export my video in multiple formats so that I can upload to YouTube, TikTok, and Instagram.

**Implementation**:

```typescript
// Export pipeline using FFmpeg.wasm
class VideoExporter {
  private ffmpeg: FFmpeg;

  async export(project: Project, config: ExportConfig): Promise<Blob> {
    const { format, resolution, bitrate, codec } = config;

    // Load FFmpeg
    await this.ffmpeg.load();

    // Combine tracks
    const timelineMLT = this.generateMLT(project);
    await this.ffmpeg.writeFile('timeline.mlt', timelineMLT);

    // Export command
    const outputFilename = `output.${format}`;
    await this.ffmpeg.exec([
      '-f', 'mlt',
      '-i', 'timeline.mlt',
      '-c:v', codec,
      '-b:v', bitrate,
      '-s', resolution,
      '-preset', 'ultrafast', // Speed over quality for MVP
      '-movflags', '+faststart', // Web-optimized
      outputFilename
    ]);

    // Read output
    const data = await this.ffmpeg.readFile(outputFilename);
    return new Blob([data], { type: `video/${format}` });
  }

  async exportMultipleFormats(project: Project): Promise<Export[]> {
    // Parallel export for different platforms
    const presets = [
      {
        name: 'youtube-1080p',
        format: 'mp4',
        resolution: '1920x1080',
        bitrate: '8M',
        codec: 'libx264'
      },
      {
        name: 'tiktok-1080x1920',
        format: 'mp4',
        resolution: '1080x1920', // Vertical
        bitrate: '4M',
        codec: 'libx264'
      },
      {
        name: 'instagram-square',
        format: 'mp4',
        resolution: '1080x1080',
        bitrate: '5M',
        codec: 'libx264'
      }
    ];

    return Promise.all(
      presets.map(preset => this.export(project, preset))
    );
  }
}
```

**MVP Scope**:
- [x] MP4 export (H.264 codec)
- [x] Resolution presets (1080p, 720p, 480p)
- [x] Platform presets (YouTube, TikTok, Instagram)
- [x] Progress indicator
- [x] Thumbnail generation
- [ ] HDR export (v0.3.0)
- [ ] ProRes export (v0.3.0)

**Performance Targets**:
- Export speed: 1 minute of video in 2 minutes (browser)
- Max file size: 2GB (IndexedDB limit)
- Fallback: Download-as-you-export for large files

---

## Technical Architecture

### Frontend Stack

```yaml
Framework: Next.js 15
  - App Router for routing
  - Server Components for data fetching
  - Edge Functions for lightweight processing

UI: React 19
  - Client Components for interactive editor
  - Suspense for loading states
  - Server Actions for mutations

Styling: Tailwind CSS 4
  - Custom design tokens
  - Dark mode by default
  - Responsive breakpoints

State Management: Zustand + React Query
  - Zustand: Editor state (timeline, selection, undo/redo)
  - React Query: Server state (projects, exports, AI jobs)

Video Processing:
  - FFmpeg.wasm: Browser-based encoding
  - WebCodecs API: Hardware acceleration
  - Web Audio API: Audio visualization
  - MediaRecorder: Capture
```

### Backend Services

```yaml
API: Next.js Route Handlers
  - /api/projects: CRUD operations
  - /api/capture: WebSocket for live capture
  - /api/export: Polling for export progress
  - /api/ai: Proxy to SwarmOrchestration

Database: Supabase (PostgreSQL)
  - projects: Project metadata
  - clips: Clip references and edits
  - exports: Export job status
  - users: Authentication and preferences

Storage: Supabase Storage
  - videos: Original source files
  - thumbnails: Generated thumbnails
  - exports: Exported video files

Real-time: Supabase Realtime
  - Collaboration cursors
  - Live editing sessions
  - Export progress updates
```

### AI Integration

```yaml
Orchestration: SwarmOrchestration (SuperInstance)
  - Task routing based on complexity
  - Cost optimization via multi-model routing
  - Fault tolerance and retry logic

Models (Content Studio patterns):
  Silence Detection:
    Primary: whisper-tiny (local, $0)
    Fallback: whisper-base (Replicate, $0.05/hr)

  Caption Generation:
    Primary: whisper-base (Replicate, $0.15/hr)
    Fallback: whisper-medium (Replicate, $0.30/hr)

  Highlight Detection:
    Primary: claude-haiku-3.5 (Anthropic, $0.25/hr)
    Fallback: gpt-4o-mini (OpenAI, $0.20/hr)

  Smart Cuts:
    Primary: claude-sonnet-3.5 (Anthropic, $2.00/hr)
    Fallback: llama-3.1-70b (Replicate, $0.65/hr)

Cost Model:
  Target: $0.50 per hour of video processing
  Actual (projected): $0.35/hr with intelligent routing
  Savings: 82% vs all-cloud ($2.00/hr)
```

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Browser (Client)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  Capture     │  │   Timeline   │  │   Preview    │     │
│  │  (MediaRecorder)│  (Canvas)    │  │  (Video)     │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
│         │                  │                  │             │
│         └──────────────────┴──────────────────┘             │
│                            │                                │
└────────────────────────────┼────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                  Next.js API Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │  /api/capture│  │ /api/export  │  │   /api/ai    │     │
│  │  (WebSocket) │  │  (Polling)   │  │   (Proxy)    │     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘     │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌─────────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  Supabase DB    │  │ Supabase    │  │ SwarmOrchestration  │ │
│  (PostgreSQL)   │  │ Storage     │  │  (AI Pipeline)      │ │
│                 │  │ (S3-compatible)│                     │ │
└─────────────────┘  └─────────────┘  └─────────────────────┘ │
                                                        │
                                                        ▼
                                            ┌──────────────────────┐
                                            │   Model Providers    │
                                            │  - Anthropic (Claude)│
                                            │  - OpenAI (GPT-4)    │
                                            │  - Replicate (Llama) │
                                            └──────────────────────┘
```

---

## Data Model

### Database Schema (PostgreSQL)

```sql
-- Projects
CREATE TABLE projects (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES auth.users(id),
  name VARCHAR(255) NOT NULL,
  thumbnail_url TEXT,
  duration INT, -- Total frames
  fps INT DEFAULT 30,
  resolution JSONB, -- { width: 1920, height: 1080 }
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Sources (captured media)
CREATE TABLE sources (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  type VARCHAR(20) NOT NULL, -- 'screen', 'camera', 'microphone', 'audio'
  storage_path TEXT NOT NULL, -- Supabase Storage path
  duration INT, -- Frames
  metadata JSONB, -- { codec, bitrate, resolution }
  created_at TIMESTAMP DEFAULT NOW()
);

-- Clips (timeline edits)
CREATE TABLE clips (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  source_id UUID REFERENCES sources(id),
  track_index INT NOT NULL,
  start_frame INT NOT NULL,
  end_frame INT NOT NULL,
  offset_frame INT DEFAULT 0, -- In-point on source
  filters JSONB, -- { brightness: 1.0, contrast: 1.0 }
  created_at TIMESTAMP DEFAULT NOW()
);

-- Text Overlays
CREATE TABLE text_overlays (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  track_index INT NOT NULL,
  start_frame INT NOT NULL,
  end_frame INT NOT NULL,
  content TEXT NOT NULL,
  style JSONB, -- { font, size, color, position }
  created_at TIMESTAMP DEFAULT NOW()
);

-- AI Jobs (background processing)
CREATE TABLE ai_jobs (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  type VARCHAR(50) NOT NULL, -- 'silence_detection', 'highlights', 'captions'
  status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
  model VARCHAR(100),
  cost_usd DECIMAL(10, 4),
  result JSONB, -- { segments: [], metadata: {} }
  error TEXT,
  created_at TIMESTAMP DEFAULT NOW(),
  completed_at TIMESTAMP
);

-- Exports
CREATE TABLE exports (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  format VARCHAR(10) NOT NULL, -- 'mp4', 'webm'
  resolution VARCHAR(20), -- '1920x1080'
  bitrate VARCHAR(20), -- '8M'
  status VARCHAR(20) DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
  progress INT DEFAULT 0, -- 0-100
  storage_path TEXT,
  file_size BIGINT,
  created_at TIMESTAMP DEFAULT NOW(),
  completed_at TIMESTAMP
);

-- Edit History (undo/redo)
CREATE TABLE edit_history (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
  action VARCHAR(100) NOT NULL, -- 'create_clip', 'delete_clip', 'trim_clip'
  before_state JSONB,
  after_state JSONB,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_projects_user_id ON projects(user_id);
CREATE INDEX idx_sources_project_id ON sources(project_id);
CREATE INDEX idx_clips_project_id ON clips(project_id);
CREATE INDEX idx_ai_jobs_project_id ON ai_jobs(project_id);
CREATE INDEX idx_ai_jobs_status ON ai_jobs(status);
CREATE INDEX idx_exports_project_id ON exports(project_id);
CREATE INDEX idx_edit_history_project_id ON edit_history(project_id);
```

### State Management (Zustand)

```typescript
// stores/editor.ts
interface EditorState {
  // Current project
  projectId: string | null;

  // Timeline state
  tracks: Track[];
  playhead: number;
  zoom: number;
  selection: string[]; // Selected clip IDs

  // Playback state
  isPlaying: boolean;
  playbackSpeed: number;

  // History
  past: EditorState[];
  future: EditorState[];

  // Actions
  setProjectId: (id: string) => void;
  addTrack: (track: Track) => void;
  removeTrack: (trackId: string) => void;
  addClip: (clip: Clip, trackIndex: number) => void;
  removeClip: (clipId: string) => void;
  trimClip: (clipId: string, start: number, end: number) => void;
  moveClip: (clipId: string, trackIndex: number, offset: number) => void;
  setPlayhead: (frame: number) => void;
  setZoom: (level: number) => void;
  setSelection: (clipIds: string[]) => void;

  // Undo/redo
  undo: () => void;
  redo: () => void;

  // Playback
  togglePlayback: () => void;
  setPlaybackSpeed: (speed: number) => void;
}

export const useEditorStore = create<EditorState>()(
  persist(
    (set, get) => ({
      // Initial state
      projectId: null,
      tracks: [],
      playhead: 0,
      zoom: 1,
      selection: [],
      isPlaying: false,
      playbackSpeed: 1.0,
      past: [],
      future: [],

      // Actions
      setProjectId: (id) => set({ projectId: id }),

      addClip: (clip, trackIndex) => set((state) => {
        const tracks = [...state.tracks];
        tracks[trackIndex].clips.push(clip);

        // Add to history
        const past = [...state.past, state];
        const future = [];

        return { tracks, past, future };
      }),

      undo: () => set((state) => {
        if (state.past.length === 0) return state;

        const previous = state.past[state.past.length - 1];
        const newPast = state.past.slice(0, state.past.length - 1);
        const newFuture = [state, ...state.future];

        return {
          ...previous,
          past: newPast,
          future: newFuture
        };
      }),

      // ... other actions
    }),
    {
      name: 'reallog-editor',
      partialize: (state) => ({
        projectId: state.projectId,
        tracks: state.tracks,
        zoom: state.zoom
      })
    }
  )
);
```

---

## User Stories

### Story 1: YouTuber Creates Highlight Reel

**Actor**: Sarah, gaming YouTuber with 500K subscribers

**Goal**: Create 10-minute highlight video from 4-hour stream

**Workflow**:

1. **Capture** (2 minutes)
   - Sarah opens RealLog
   - Clicks "New Project" → selects "Stream Capture"
   - Chooses sources: Screen (game), Camera (face), Microphone
   - Starts OBS-style recording in browser
   - Plays game for 4 hours (automatic capture)

2. **AI Processing** (automatic, background)
   - RealLog detects silence ($0.10 for 4 hours)
   - Generates captions ($0.60 for 4 hours)
   - Detects highlights ($1.00 for 4 hours)
   - Total AI cost: $1.70

3. **Edit** (30 minutes)
   - Sarah opens "Highlight Suggestions" panel
   - Sees 20 suggested clips ranked by excitement score
   - Drags top 10 clips to timeline
   - Uses "Auto-Remove Silence" button
   - Adds intro/outro templates
   - Review captions (98% accuracy)

4. **Export** (20 minutes for 10-minute video)
   - Selects "YouTube 1080p" preset
   - Clicks export
   - Does other work while FFmpeg.wasm processes
   - Downloads MP4 when complete

**Time Savings**: 10 hours (manual editing) → 1 hour (AI-assisted) = **90% reduction**

---

### Story 2: Course Creator Records Tutorial

**Actor**: James, programming instructor

**Goal**: Create 20-minute tutorial with screen recording and face cam

**Workflow**:

1. **Capture** (1 minute setup)
   - Selects "Tutorial Recording" mode
   - Captures: Code editor (screen), Face (camera), Mic
   - Records demo in real-time

2. **Sync Sources** (automatic)
   - RealLog aligns all sources by timestamp
   - James sees 3-track timeline:
     - Track 1: Screen (code)
     - Track 2: Camera (face)
     - Track 3: Audio (voiceover)

3. **Edit** (15 minutes)
   - Trims mistake at 12:30 (razor tool)
   - Adds "Section 1" text overlay (0:00 - 5:00)
   - Inserts pause at 8:15 (zoom in on code)
   - Auto-generates captions for accessibility

4. **Export** (10 minutes)
   - Selects "YouTube 1080p" + "Podcast Audio" (dual export)
   - Gets MP4 video + MP3 audio for podcast version

**Outcome**: Professional tutorial in 30 minutes vs 4 hours (traditional)

---

### Story 3: Social Media Manager Repurposes Content

**Actor**: Maria, social media manager for tech startup

**Goal**: Create 15 TikToks from 1-hour webinar

**Workflow**:

1. **Import** (5 minutes)
   - Uploads existing webinar video
   - RealLog generates transcript ($0.15)

2. **AI Clip Detection** (10 minutes, $0.50)
   - Uses "Smart Clipper" feature
   - Detects 30 potential clips (15-60 seconds each)
   - Ranks by "shareability" score

3. **Batch Edit** (30 minutes)
   - Maria selects top 15 clips
   - Applies "TikTok vertical" preset to all
   - Adds auto-captions (styled for mobile)
   - Inserts brand intro/outro to all

4. **Batch Export** (parallel, 30 minutes)
   - Exports all 15 clips simultaneously
   - Gets 15 MP4 files ready for upload

**Time Savings**: 15 hours (manual) → 1.25 hours (AI) = **92% reduction**

---

## Differentiation Strategy

### Why RealLog Over Competitors?

| Feature | RealLog | Descript | CapCut | Premiere/DaVinci |
|---------|---------|----------|---------|------------------|
| **Browser-based** | ✅ Yes | ✅ Yes | ❌ Mobile only | ❌ Desktop only |
| **Multi-source capture** | ✅ 4+ sources | ⚠️ 2 max | ⚠️ 2 max | ✅ Unlimited |
| **AI silence removal** | ✅ One-click | ✅ Yes | ❌ No | ❌ Manual |
| **AI highlights** | ✅ Auto-detect | ⚠️ Manual only | ❌ No | ❌ Manual |
| **Auto-captions** | ✅ 98% accuracy | ✅ 95% | ✅ 90% | ❌ Manual |
| **Real-time collab** | ✅ Multi-user | ⚠️ Single user | ❌ No | ⚠ia Plugin |
| **Cost optimization** | ✅ $0.50/hr | ❌ $2.00/hr | ❌ $1.50/hr | N/A |
| **Learning curve** | ⚠️ Medium | ⚠️ Medium | ✅ Easy | ❌ Steep |
| **Export formats** | ✅ 10+ presets | ⚠️ 4 presets | ⚠️ 3 presets | ✅ Custom |

### Key Differentiators

1. **Cost-Optimized AI Processing**
   - Intelligent model routing from Content Studio patterns
   - 82% cheaper than Descript ($0.50/hr vs $2.00/hr)
   - Transparent cost breakdown before processing

2. **Multi-Source Capture in Browser**
   - No software download required
   - Screen + Camera + Mic + System Audio simultaneously
   - Automatic timestamp synchronization

3. **AI-First Workflow**
   - Highlights suggested, not just manual clip selection
   - One-click silence removal across all tracks
   - Smart cut suggestions based on transcription

4. **Real-Time Collaboration**
   - Multiple editors on same project
   - Live cursor tracking
   - Conflict resolution (last edit wins)

5. **Developer-Friendly**
   - API-first architecture
   - Webhook support for export notifications
   - Embeddable player component

---

## Development Roadmap

### Phase 0: Foundation (Week 1-2) - Complete

**Goals**: Project setup, infrastructure ready

- [x] Next.js 15 project with TypeScript
- [x] Supabase setup (database + storage)
- [x] Authentication with Supabase Auth
- [x] Tailwind CSS design system
- [x] CI/CD with GitHub Actions
- [x] Deployment to Vercel

**Deliverables**:
- https://reallog.ai redirects to Vercel app
- "Sign in with GitHub" working
- Blank project created on signup

---

### Phase 1: MVP Core (Week 3-6) - In Progress

**Goals**: Basic capture and editing working

#### Sprint 1: Capture System (Week 3)
- [ ] MediaRecorder integration for screen/camera/mic
- [ ] Multi-source coordinator class
- [ ] Chunked recording to IndexedDB
- [ ] Upload to Supabase Storage
- [ ] Basic timeline display of sources

**Success Criteria**: User can record 5-minute video from 3 sources and see on timeline

#### Sprint 2: Timeline Editor (Week 4)
- [ ] Canvas-based timeline renderer
- [ ] Drag-and-drop clips between tracks
- [ ] Trimming clips with mouse
- [ ] Playhead scrubbing
- [ ] Video preview panel
- [ ] Undo/redo stack

**Success Criteria**: User can rearrange clips and preview playback

#### Sprint 3: AI Integration (Week 5)
- [ ] SwarmOrchestration client setup
- [ ] Silence detection job queue
- [ ] Caption generation pipeline
- [ ] Progress polling UI
- [ ] Cost estimation before processing

**Success Criteria**: User can click "Remove Silence" and see suggestions

#### Sprint 4: Export (Week 6)
- [ ] FFmpeg.wasm integration
- [ ] MP4 export (H.264)
- [ ] Progress indicator
- [ ] Thumbnail generation
- [ ] Download to device

**Success Criteria**: User can export 5-minute video in 10 minutes

---

### Phase 2: Beta Features (Week 7-10)

**Goals**: Polish and advanced features

#### Sprint 5: AI Enhancements (Week 7-8)
- [ ] Highlight detection (energy + transcription)
- [ ] Smart cut suggestions
- [ ] Speaker detection (multi-person videos)
- [ ] Auto-b-roll suggestions
- [ ] Caption styling options

#### Sprint 6: Collaboration (Week 9)
- [ ] Multi-user editing sessions
- [ ] Live cursor tracking
- [ ] Conflict resolution
- [ ] Comment system on timeline
- [ ] Version history

#### Sprint 7: Platform Presets (Week 10)
- [ ] TikTok vertical export
- [ ] Instagram square export
- [ ] YouTube shorts export
- [ ] Custom resolutions
- [ ] Batch export for multiple formats

**Beta Launch Goal**: 10 beta users testing for 2 weeks

---

### Phase 3: Public Launch (Week 11-14)

**Goals**: Production-ready, scalable

#### Sprint 8: Performance (Week 11)
- [ ] WebCodecs API for hardware acceleration
- [ ] Lazy loading FFmpeg.wasm
- [ ] Virtualized timeline (large projects)
- [ ] Export queue management
- [ ] CDN caching for assets

#### Sprint 9: Polish (Week 12)
- [ ] Keyboard shortcuts documentation
- [ ] Onboarding tutorial
- [ ] Error handling improvements
- [ ] Loading states
- [ ] Accessibility audit

#### Sprint 10: Marketing (Week 13-14)
- [ ] Landing page with demo video
- [ ] Documentation site
- [ ] Example project templates
- [ ] Social media launch campaign
- [ ] Product Hunt launch

**Public Launch**: v0.1.0 live on https://reallog.ai

---

### Phase 4: Post-Launch (Month 4-6)

**Goals**: Feedback-driven improvements

- [ ] Transitions between clips
- [ ] Keyframe animation
- [ ] Audio ducking
- [ ] Color grading filters
- [ ] Plugin architecture
- [ ] API access for developers

**Target**: 100 active users by end of Month 6

---

## Monetization Strategy

### MVP Pricing (v0.1.0 - v0.3.0)

**Phase 1: Free Beta** (First 3 months)
- Unlimited projects
- 10 hours of AI processing per month
- 720p export max
- Community support

**Goal**: Get 100 users, collect feedback, refine features

---

### Launch Pricing (v0.4.0+)

#### Free Tier
- 3 projects max
- 1 hour AI processing/month
- 720p export with watermark
- Community Discord support

**Target**: Students, hobbyists

---

#### Pro Tier - $29/month
- Unlimited projects
- 10 hours AI processing/month
- 1080p export no watermark
- Email support
- Platform presets (YouTube, TikTok, Instagram)
- Collaboration (2 users max)

**Target**: YouTubers with 10K-100K subs, streamers

**Value Proposition**: $29/month saves 20 hours editing = **$1.45/hour saved**

---

#### Team Tier - $99/month
- Everything in Pro
- 50 hours AI processing/month
- 4K export
- Unlimited collaboration
- Priority support
- Custom branding on player
- API access

**Target**: Production companies, agencies

**Value Proposition**: $99/month for team of 5 = **$20/user/month**

---

#### Enterprise - Custom
- Everything in Team
- Unlimited AI processing
- Dedicated infrastructure
- SLA guarantees
- Onboarding & training
- Custom integrations

**Target**: Large media companies, educational institutions

---

### Revenue Projections

**Year 1** (Conservative):
- Free users: 1,000
- Pro users: 50 (5% conversion) × $29 × 12 = $17,400
- Team users: 10 (1% conversion) × $99 × 12 = $11,880
- **Total ARR**: $29,280

**Year 2** (Moderate growth):
- Free users: 10,000
- Pro users: 500 (5% conversion) × $29 × 12 = $174,000
- Team users: 100 (1% conversion) × $99 × 12 = $118,800
- **Total ARR**: $292,800

**Year 3** (Aggressive growth):
- Free users: 100,000
- Pro users: 5,000 (5% conversion) × $29 × 12 = $1,740,000
- Team users: 1,000 (1% conversion) × $99 × 12 = $1,188,000
- **Total ARR**: $2.9M

---

### Cost Structure

**Per-User Costs** (Monthly):
- AI processing: $5 average (Pro), $25 (Team)
- Storage: $2 average (Supabase)
- Bandwidth: $1 average (Vercel)
- Support: $5 average (Pro), $20 (Team)
- **Total**: $13 (Pro), $53 (Team)

**Margins**:
- Pro: $29 - $13 = $16 (55% margin)
- Team: $99 - $53 = $46 (46% margin)

**Break-Even**: 50 Pro users or 15 Team users covers infrastructure costs (~$800/month)

---

## Success Metrics

### MVP Success Criteria (v0.1.0)

**Product Metrics**:
- [ ] 100 active users (created ≥1 project)
- [ ] 50% week-1 retention
- [ ] 10% conversion rate (free → Pro)
- [ ] Average session: 30+ minutes
- [ ] 20 projects exported per week

**Technical Metrics**:
- [ ] 99% uptime (Vercel SLA)
- [ ] <2s page load time
- [ ] <100ms timeline scrub latency
- [ ] Export success rate: 95%+
- [ ] AI job success rate: 90%+

**User Feedback**:
- [ ] NPS score: 40+
- [ ] Average rating: 4.0+ stars
- [ ] "Would recommend": 70%+

---

### Post-MVP Targets (v0.2.0 - v0.3.0)

**Month 6**:
- 1,000 active users
- 50 paying customers
- $2,000 MRR
- 70% week-1 retention

**Month 12**:
- 10,000 active users
- 500 paying customers
- $15,000 MRR
- 80% week-1 retention

---

## Risks & Mitigation

### Technical Risks

**Risk 1: Browser-based export too slow**
- **Probability**: High (60%)
- **Impact**: High (users won't wait 20 minutes for 10-minute video)
- **Mitigation**:
  - Offer cloud-based export as add-on ($0.10/minute)
  - Implement progressive download (watch while exporting)
  - Use WebCodecs API for hardware acceleration

**Risk 2: FFmpeg.wasm memory issues**
- **Probability**: Medium (40%)
- **Impact**: High (crashes browser on large projects)
- **Mitigation**:
  - Enforce 2GB project size limit
  - Chunked export for large videos
  - Fallback to cloud FFmpeg

**Risk 3: AI processing costs explode**
- **Probability**: Medium (30%)
- **Impact**: Medium (negative margins on Pro tier)
- **Mitigation**:
  - Strict per-user limits (10 hours/month)
  - Cache results (deduplication)
  - Pass costs to users after quota exceeded

---

### Market Risks

**Risk 4: Descript launches multi-source capture**
- **Probability**: Medium (40%)
- **Impact**: High (loses main differentiation)
- **Mitigation**:
  - Focus on cost optimization (82% cheaper)
  - Build developer ecosystem (API, plugins)
  - Target underserved niches (course creators, social media teams)

**Risk 5: Users won't trust browser-based video editing**
- **Probability**: Low (20%)
- **Impact**: High (slow adoption)
- **Mitigation**:
  - Emphasize privacy (client-side processing)
  - Show "works offline" badge
  - Highlight cloud export optional

---

### Business Risks

**Risk 6: Can't reach $29/month price point**
- **Probability**: Medium (35%)
- **Impact**: High (need 2-3x users for same revenue)
- **Mitigation**:
  - Start with free beta to prove value
  - A/B test pricing ($19, $29, $49)
  - Offer annual discount ($290/year = $24/month)

**Risk 7: Infrastructure scales faster than revenue**
- **Probability**: High (60%)
- **Impact**: Medium (cash flow issues)
- **Mitigation**:
  - Enforce storage limits (10GB free, 100GB Pro)
  - Use CDN caching aggressively
  - Implement usage-based pricing for overages

---

## Appendix: Code Samples

### A1: Capture Component

```typescript
// app/components/CaptureControl.tsx
'use client';

import { useState } from 'react';

export function CaptureControl() {
  const [isRecording, setIsRecording] = useState(false);
  const [sources, setSources] = useState({
    screen: false,
    camera: false,
    microphone: false
  });

  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getDisplayMedia({
        video: { cursor: 'always' },
        audio: true
      });

      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: 'video/webm;codecs=vp9',
        videoBitsPerSecond: 5000000
      });

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          // Upload to Supabase
          uploadChunk(event.data);
        }
      };

      mediaRecorder.start(1000); // Chunk every second
      setIsRecording(true);

    } catch (error) {
      console.error('Error starting capture:', error);
    }
  };

  const stopRecording = () => {
    // Stop all streams
    setIsRecording(false);
  };

  return (
    <div className="flex gap-4">
      <button
        onClick={isRecording ? stopRecording : startRecording}
        className={`px-6 py-3 rounded-lg font-medium ${
          isRecording
            ? 'bg-red-500 text-white'
            : 'bg-blue-500 text-white'
        }`}
      >
        {isRecording ? 'Stop Recording' : 'Start Recording'}
      </button>

      <div className="flex gap-2">
        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={sources.screen}
            onChange={(e) => setSources({ ...sources, screen: e.target.checked })}
          />
          Screen
        </label>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={sources.camera}
            onChange={(e) => setSources({ ...sources, camera: e.target.checked })}
          />
          Camera
        </label>

        <label className="flex items-center gap-2">
          <input
            type="checkbox"
            checked={sources.microphone}
            onChange={(e) => setSources({ ...sources, microphone: e.target.checked })}
          />
          Microphone
        </label>
      </div>
    </div>
  );
}
```

---

### A2: AI Pipeline Integration

```typescript
// app/lib/ai/pipeline.ts
import { SwarmClient } from '@superinstance/swarm-orchestration';

const swarm = new SwarmClient({
  endpoint: process.env.SWARM_ENDPOINT,
  apiKey: process.env.SWARM_API_KEY
});

export async function detectSilence(
  videoId: string,
  options: { threshold: number; minDuration: number }
): Promise<SilenceSegment[]> {

  const task = await swarm.submit({
    type: 'silence_detection',
    model: 'whisper-tiny',
    input: {
      videoId,
      threshold: options.threshold, // -40dB
      minDuration: options.minDuration // 1000ms
    }
  });

  // Poll for completion
  let result = await task.get();
  while (result.status === 'processing') {
    await new Promise(resolve => setTimeout(resolve, 1000));
    result = await task.get();
  }

  if (result.status === 'failed') {
    throw new Error(result.error);
  }

  return result.output.segments;
}

export async function generateCaptions(
  videoId: string,
  language: string = 'en'
): Promise<Caption[]> {

  const task = await swarm.submit({
    type: 'caption_generation',
    model: 'whisper-base',
    input: {
      videoId,
      language
    }
  });

  const result = await task.await();

  return result.segments.map(seg => ({
    start: seg.start,
    end: seg.end,
    text: seg.text,
    confidence: seg.confidence
  }));
}

export async function detectHighlights(
  videoId: string,
  transcript: string
): Promise<HighlightSegment[]> {

  // Two-stage pipeline
  // Stage 1: Analyze audio features (cheap)
  const audioFeatures = await swarm.submit({
    type: 'audio_analysis',
    model: 'whisper-tiny',
    input: { videoId }
  });

  // Stage 2: Detect highlights using transcript + features (mid-tier)
  const highlights = await swarm.submit({
    type: 'highlight_detection',
    model: 'claude-haiku-3.5',
    input: {
      transcript,
      audioFeatures: await audioFeatures,
      criteria: ['laughter', 'applause', 'energy_spikes', 'sentiment_changes']
    }
  });

  return (await highlights).segments;
}
```

---

### A3: Export Pipeline

```typescript
// app/lib/export/ffmpeg.ts
import { FFmpeg } from '@ffmpeg/ffmpeg';
import { fetchFile, toBlobURL } from '@ffmpeg/util';

let ffmpegInstance: FFmpeg | null = null;

async function getFFmpeg(): Promise<FFmpeg> {
  if (ffmpegInstance) return ffmpegInstance;

  const ffmpeg = new FFmpeg();

  const baseURL = 'https://unpkg.com/@ffmpeg/core@0.12.6/dist/umd';
  await ffmpeg.load({
    coreURL: await toBlobURL(`${baseURL}/ffmpeg-core.js`, 'text/javascript'),
    wasmURL: await toBlobURL(`${baseURL}/ffmpeg-core.wasm`, 'application/wasm')
  });

  ffmpegInstance = ffmpeg;
  return ffmpeg;
}

export async function exportVideo(
  project: Project,
  config: ExportConfig,
  onProgress: (progress: number) => void
): Promise<Blob> {

  const ffmpeg = await getFFmpeg();

  // Generate MLT timeline file
  const mltContent = generateMLT(project);
  await ffmpeg.writeFile('timeline.mlt', mltContent);

  // Setup progress callback
  ffmpeg.on('progress', ({ progress }) => {
    onProgress(Math.round(progress * 100));
  });

  // Execute export
  const outputFilename = `output.${config.format}`;
  await ffmpeg.exec([
    '-f', 'mlt',
    '-i', 'timeline.mlt',
    '-c:v', config.codec,
    '-b:v', config.bitrate,
    '-s', config.resolution,
    '-preset', 'ultrafast',
    '-movflags', '+faststart',
    outputFilename
  ]);

  // Read output
  const data = await ffmpeg.readFile(outputFilename);
  return new Blob([data], { type: `video/${config.format}` });
}

function generateMLT(project: Project): string {
  return `
<?xml version="1.0" encoding="utf-8"?>
<mlt>
  <profile id="square"
    width="1920" height="1080"
    progressive="1"
    sample_aspect_num="1" sample_aspect_den="1"
    display_aspect_num="16" display_aspect_den="9"
    frame_rate_num="30" frame_rate_den="1"
  />

  ${project.tracks.map((track, i) => `
    <playlist id="track${i}">
      ${track.clips.map(clip => `
        <entry producer="clip${clip.id}" in="${clip.offset}" out="${clip.end - clip.start}" />
      `).join('')}
    </playlist>

    <producer id="clip${clip.id}">
      <property name="resource">${clip.sourceUrl}</property>
    </producer>
  `).join('')}

  <tractor id="tractor0">
    ${project.tracks.map((_, i) => `
      <track producer="track${i}" />
    `).join('')}
  </tractor>
</mlt>
  `.trim();
}
```

---

## Next Steps

### Immediate Actions (This Week)

1. **Set up repository**
   ```bash
   git clone https://github.com/SuperInstance/RealLog.git
   cd RealLog
   npm install
   ```

2. **Configure Supabase**
   - Create project at supabase.com
   - Run database schema migration
   - Set up storage buckets
   - Configure auth (GitHub OAuth)

3. **Deploy to Vercel**
   ```bash
   vercel link
   vercel env pull .env.local
   vercel prod
   ```

4. **Schedule Sprint 1 kickoff**
   - Week 3: Capture system
   - Assign developer(s)
   - Set up daily standups
   - Create GitHub project board

### First Month Goals

- [ ] Working multi-source capture (screen + camera + mic)
- [ ] Basic timeline display
- [ ] First 5-minute test video exported
- [ ] 5 beta users testing

### First Quarter Goals

- [ ] Public MVP launch (v0.1.0)
- [ ] 100 active users
- [ ] 20 projects exported per week
- [ ] NPS score: 40+

---

**Let's build RealLog and make video editing 10x faster with AI.**

---

*Document Version: 1.0*
*Last Updated: 2026-01-01*
*Author: SuperInstance Team*
*License: MIT (open source, closed source for AI processing infrastructure)
