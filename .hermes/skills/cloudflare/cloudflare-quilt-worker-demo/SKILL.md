---
name: cloudflare-quilt-worker-demo
description: End‑to‑end workflow for building a Cloudflare Worker that runs the Quilt reactive engine with AI integration and per‑visitor rate limiting.
summary: Guides creation, configuration, deployment, and verification of a Quilt demo worker, including KV namespace setup, secret handling, and common pitfalls.
---

## Prerequisites
- Wrangler installed (`npm i -g @cloudflare/wrangler`) and authenticated (`wrangler login`).
- Cloudflare account ID and AI token available.
- Quilt core compiled (`packages/core/dist`) and demo sheet (`demo/demo_sheet.json`).
- **Windows users**: run commands in PowerShell or Command Prompt; the CLI commands are the same, but ensure you are using a compatible shell.

## Step‑by‑step
1. **Create project folder**
   ```bash
   mkdir quilt_worker && cd quilt_worker
   ```
2. **Add source files**
   - `src/index.js` – entry point that loads the sheet, creates `SimpleAIEngine`, enforces KV rate‑limit, and returns JSON.
   - `src/simple_ai_engine.js` – thin wrapper around Cloudflare AI, uses `import { URLSearchParams } from 'node:url';`, caps `max_tokens` to 100 and limits calls per request.
   - `demo/demo_sheet.json` – static Quilt sheet used for the demo.
3. **Create `wrangler.toml`** (use the template in `templates/wrangler.toml.template`). Replace `<KV_NAMESPACE_ID>` with the ID from step 4.
   - Add Node.js compatibility flags and set compatibility_date to 2024-09-23 or later:
     ```toml
     compatibility_flags = ["nodejs_compat"]
     compatibility_date = "2024-09-23"
     ```
   - If your `wrangler.toml` defines multiple environments (e.g., `[env.production]`), remember to specify the target environment with `--env=""` for subsequent wrangler commands to avoid warnings.
4. **Create KV namespace**
   ```bash
   wrangler kv namespace create USAGE
   ```
   Copy the generated ID into the `wrangler.toml` file.
5. **Add AI secrets** (use `--env=""` if you have multiple environments)
   ```bash
   wrangler secret put CF_ACCOUNT_ID --env=""   # paste your 32‑char account ID
   wrangler secret put CF_AI_TOKEN --env=""    # paste your AI token
   ```
6. **Deploy** (use `--env=""` if you have multiple environments)
   ```bash
   wrangler deploy --env=""
   ```
   The CLI prints the worker URL.
7. **Verify**
   ```bash
   curl https://<worker-subdomain>.workers.dev
   ```
   You should receive a JSON payload with evaluated cells. After `VISIT_LIMIT` (default 5) calls from the same IP you’ll get a `429` response.

## Pitfalls & Gotchas
- **KV config syntax** – older Wrangler versions need the `[[kv_namespaces]]` table format. Using the newer array syntax may be ignored, causing the "Unexpected fields found in triggers field: kv_namespaces" warning.
- **Missing secrets** – if `CF_ACCOUNT_ID` or `CF_AI_TOKEN` are not set, the AI wrapper logs a warning and all AI calls fail.
- **Rate‑limit logic** – the KV counter is stored with a TTL of 86400 s (24 h). Ensure `expirationTtl` is passed when incrementing.
- **Token cap** – the wrapper enforces `max_tokens <= 100`. Requests exceeding this throw an error.
- **URL import error** – If you see `Could not resolve "url"` when using `import { URLSearchParams } from 'url';`, change the import to `import { URLSearchParams } from 'node:url';` and ensure `compatibility_date` is set to 2024-09-23 or later in `wrangler.toml`.
- **Wrangler version** – newer Wrangler (≥ 4.125.0) improves KV handling. Upgrade if you see warnings.
- **Multiple environments warning** – if your `wrangler.toml` defines more than one environment (e.g., `[env.production]`), wrangler commands will warn about needing to specify `--env`. Use `--env=""` to target the top‑level environment, or specify a specific environment like `--env=production`.

## References
- `references/kv_namespace_id.md` – the actual KV namespace ID generated for this project.
- `templates/wrangler.toml.template` – ready‑to‑copy `wrangler.toml` with placeholder ID.

---
*Skill maintained by Hermes (OB1). Updated 2026‑08‑21.*