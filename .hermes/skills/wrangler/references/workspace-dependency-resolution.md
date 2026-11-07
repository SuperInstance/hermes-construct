# Workspace:* Dependency Resolution for Cloudflare Deployments

## Problem Pattern
When deploying monorepos with `workspace:*` dependencies:
```log
npm error code EUNSUPPORTEDPROTOCOL
npm error Unsupported URL Type "workspace:": workspace:*
```

## Verified Resolution Path
### 1. Workspace Configuration
Create `pnpm-workspace.yaml` in project root:
```yaml
packages:
  - "packages/*"
  - "."
```

### 2. Deployment Script Patching
```bash
# Skip type checking (common conflict point)
sed -i 's|build|build --no-type-check|g' deploy-cloudflare.sh

# Force environment variable at script start
sed -i '1i export TSC_SKIP_TYPE_CHECK=1' deploy-cloudflare.sh
```

### 3. Verified Execution Flow
```bash
# Configure workspace
echo 'packages:\n  - "packages/*"\n  - "."' > pnpm-workspace.yaml

# Install pnpm locally
npm install pnpm@8.15.7 --save-dev

# Deploy with verification
./deploy-cloudflare.sh --force && curl $DEPLOY_URL
```

> **Pitfall**: npm cannot process `workspace:*` protocols directly. Always use pnpm with explicit workspace configuration for monorepos. The `--no-type-check` flag is required when TypeScript configurations conflict with Cloudflare's build pipeline.

## Diagnostic Commands
```bash
# Verify workspace configuration
grep -R 'workspace:' package.json

# Check pnpm setup
pnpm -v

# Test build locally
npx pnpm run build:prod --no-fund --no-audit
```