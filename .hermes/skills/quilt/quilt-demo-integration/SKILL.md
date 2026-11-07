---
name: quilt-demo-integration
title: Quilt Demo Integration
description: Guide for building a Quilt core demo with a real Cloudflare AI backend, including token and call limits.
category: quilt
---

# Quilt Demo Integration Skill

## Overview
This skill documents how to build a runnable demo for the Quilt core engine that includes:
- A full sheet covering all cell kinds (value, formula, api, program, router, AI, sensor, listener).
- A Node.js runner script that loads the sheet, creates a `QuiltEngine` instance, and executes each cell.
- Integration of a **real Cloudflare AI backend** via a lightweight `SimpleAIEngine` wrapper that enforces per‑process call limits and token caps to prevent abuse.

## Prerequisites
1. Node ≥ 18 (built‑in `fetch`).
2. The Quilt core package built (`npm run build`).
3. Cloudflare account credentials set as environment variables:
   - `CF_ACCOUNT_ID`
   - `CF_AI_TOKEN`
4. Optional: `maxCalls` (default 5) and `max_tokens` (≤ 100) limits.

## Steps
1. **Create the demo sheet** (`demo/demo_sheet.json`).
2. **Add the SimpleAIEngine wrapper** (`demo/simple_ai_engine.js`).
3. **Update the runner** (`demo/run_demo.js`):
   - Import `SimpleAIEngine`.
   - Instantiate `new SimpleAIEngine({maxCalls:5})`.
   - Pass it to `new QuiltEngine({ tracing:true, ai: aiEngine })`.
4. **Run the demo**: `node demo/run_demo.js`.
5. **Verify** that non‑AI cells evaluate correctly and the AI cell either succeeds (if credentials are set) or fails gracefully with a clear warning.

## Pitfalls & Gotchas
- **Missing env vars** – the wrapper logs a warning and AI calls will error. Ensure `CF_ACCOUNT_ID` and `CF_AI_TOKEN` are exported before running.
- **Token limit exceeded** – `SimpleAIEngine` throws `Token request exceeds safe limit (100).` if `max_tokens` > 100.
- **Call limit exceeded** – after `maxCalls` (default 5) further AI calls throw `Demo AI call limit reached.`.
- **TypeScript status literals** – earlier builds failed because `engine.ts` used `'pending'`/`'evaluating'`. They were replaced with `'computing'` to match the `CellStatus` enum.
- **AI config shape** – `ai.ts` required `AICellConfig` fields (`kind`, `ai_kind`, `provider`, `model`, `id`). The patch added `kind: 'ai'` and `ai_kind` mapping.
- **Worker rate‑limit** – the Cloudflare Worker uses a KV namespace (`USAGE`) to count visits per IP and enforce a per‑visitor AI call limit (default 5). If the KV binding is missing or the secret keys are not set, the Worker will return a 429 or fail silently.

## Cloudflare Worker Deployment
### Overview
A full‑stack Worker (`quilt_worker/`) was added to expose the Quilt demo as a public endpoint with built‑in rate limiting and AI token caps.

### Steps
1. **Create KV namespace**
   ```bash
   cd quilt_worker
   wrangler kv namespace create USAGE
   ```
   Copy the generated ID and add it to `wrangler.toml` under the `kv_namespaces` section.
2. **Add secrets**
   ```bash
   wrangler secret put CF_ACCOUNT_ID
   wrangler secret put CF_AI_TOKEN
   ```
3. **Deploy**
   ```bash
   wrangler deploy
   ```
   The CLI prints the Worker URL (e.g. `https://quilt-demo.<subdomain>.workers.dev`).
4. **Test** – curl the URL; you should receive a JSON payload with evaluated cell results. Subsequent requests from the same IP are limited to `VISIT_LIMIT` AI calls per 24 h.

### Pitfalls & Gotchas (Worker)
- **Missing KV binding** – the Worker will error on first request. Ensure the KV ID is present in `wrangler.toml`.
- **Secret not set** – the AI wrapper will warn and return `null` for the AI cell.
- **Rate‑limit too low** – adjust `VISIT_LIMIT` in `src/index.js` if you need a longer or shorter window.
- **CORS** – Workers default to no CORS headers; add them if the demo is called from a browser.

### References
- `references/worker_deployment.md` – full walkthrough of the Worker setup.
- `references/kv_setup.md` – KV namespace creation and usage details.

---
*Extended to capture the Cloudflare Worker demo workflow and rate‑limit strategy.*
- **Missing env vars** – the wrapper logs a warning and AI calls will error. Ensure `CF_ACCOUNT_ID` and `CF_AI_TOKEN` are exported before running.
- **Token limit exceeded** – `SimpleAIEngine` throws `Token request exceeds safe limit (100).` if `max_tokens` > 100.
- **Call limit exceeded** – after `maxCalls` (default 5) further AI calls throw `Demo AI call limit reached.`.
- **TypeScript status literals** – earlier builds failed because `engine.ts` used `'pending'`/`'evaluating'`. They were replaced with `'computing'` to match the `CellStatus` enum.
- **AI config shape** – `ai.ts` required `AICellConfig` fields (`kind`, `ai_kind`, `provider`, `model`, `id`). The patch added `kind: 'ai'` and `ai_kind` mapping.

## References
- `references/simple_ai_engine.md` – full source of the wrapper.
- `references/demo_sheet_structure.md` – description of each cell in the demo sheet.

---
*Created to capture the end‑to‑end demo workflow and the real‑AI integration pattern for future sessions.*