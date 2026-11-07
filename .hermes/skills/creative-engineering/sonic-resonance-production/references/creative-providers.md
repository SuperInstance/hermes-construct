# User's Creative Generation Toolkit

## Available Providers (2026-08-18)

This user has multiple options for creative generation:

### 1. MMX CLI (MiniMax)
- Primary tool used for: `mmx music generate`, `mmx speech synthesize`, `mmx image generate`
- Integrated CLI with `mmx` command
- **Credit limits observed**: Music and speech generation hit limits faster than text
- Voices: `English_expressive_narrator` and others (see `mmx speech voices`)

### 2. Cloudflare Workers AI
- **Image models available** via REST API:
  - `@cf/black-forest-labs/flux-1-schnell` — ✅ Works with simple JSON API
  - `@cf/black-forest-labs/flux-2-dev` — Needs multipart format
  - `@cf/black-forest-labs/flux-2-klein-4b` / `@cf/black-forest-labs/flux-2-klein-9b` — Needs multipart format
  - `@cf/bytedance/stable-diffusion-xl-lightning` — Fast SDXL
  - `@cf/lykon/dreamshaper-8-lcm` — Photorealism
  - `@cf/leonardo/phoenix-1.0` — Great prompt adherence
  - `@cf/leonardo/lucid-origin` — Adaptable/prompt-responsive

- **LLM models** also available (not tested this session)
- **Auth**: OAuth token stored in wrangler config
- See `cloudflare/references/workers-ai-image-gen.md` for working code

### 3. DeepInfra (api.deepinfra.com)
- User mentioned availability
- Models: FLUX.1-dev, FLUX.1-schnell, SD3, etc.
- **API key location**: Unknown (check env vars: `DEEPINFRA_API_KEY`?)

### 4. Local ComfyUI
- **Location**: `C:\Users\casey\Documents\ComfyUI\`
- **21 checkpoints** available (memory-confirmed)
- **GPU**: RTX 4050 (local inference)
- Best option for: Unlimited generation, highest quality, no API limits

## Provider Selection Strategy

| Scenario | Recommended Provider |
|----------|----------------------|
| Quick iteration, simple images | Cloudflare FLUX.1-schnell |
| High quality, no time pressure | Local ComfyUI |
| Music / speech production | MMX (when credits available) |
| Backup when MMX credits low | Cloudflare + Local ComfyUI |

## Proven Workflow from "Sound of the Abyss" (2026-08-18)

1. **Deploy subagent** for blueprint creation using `delegate_task`
   - Thematic pillar alignment
   - Multi-agent divergence for creative blueprints
   - Refinement into track dossier

2. **Generate with MMX** while credits last
   - `mmx music generate --prompt "..."`
   - `mmx speech synthesize --text "..." --voice "..."`
   - `mmx image generate --prompt "..."`

3. **Fall back to Cloudflare** when MMX credits exhaust
   - Use FLUX.1-schnell for cover art
   - See `cloudflare/references/workers-ai-image-gen.md` for working Python code

4. **Save complete dossiers** for future regeneration
   - Concepts, lyrics, structure, prompts
   - Can be re-generated when credits refresh
