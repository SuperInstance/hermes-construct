# Worker Deployment Guide

This document records the exact steps used to turn the Quilt demo into a Cloudflare Worker.

## 1. Prepare the repo
```bash
cd /c/Users/casey/quilt_repo/quilt_worker
# Ensure the compiled core is present (copied from packages/core/dist)
``` 

## 2. Create KV namespace
```bash
wrangler kv namespace create USAGE
```
Copy the generated ID and edit `wrangler.toml`:
```toml
[kv_namespaces]
binding = "USAGE"
id = "<YOUR_KV_ID>"
```

## 3. Add AI secrets
```bash
wrangler secret put CF_ACCOUNT_ID   # paste your Cloudflare account ID
wrangler secret put CF_AI_TOKEN    # paste your Workers AI token
```

## 4. Deploy
```bash
wrangler deploy
```
The CLI will output a URL like:
```
https://quilt-demo.<subdomain>.workers.dev
```
Visit that URL to see the JSON result.

## 5. Verify rate‑limit
- First request from an IP counts as 1 AI call.
- Subsequent requests from the same IP are allowed up to `VISIT_LIMIT` (default 5) within a 24‑hour window.
- After the limit is hit the Worker returns **429** with the message `Rate limit exceeded. Try again later.`.

## 6. Optional tweaks
- Adjust `VISIT_LIMIT` in `src/index.js` if you need a different quota.
- Add CORS headers if the demo will be called from a browser:
```js
return new Response(JSON.stringify(results), {
  headers: {
    "Content-Type": "application/json",
    "Access-Control-Allow-Origin": "*",
  },
});
```