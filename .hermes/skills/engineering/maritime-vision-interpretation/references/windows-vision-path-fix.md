# Windows Vision Tool Path Fix
Date: 2026-08-13

## Issue:
`vision_analyze` and `browser_vision` tools throw "Invalid image source" error when passed relative or MSYS-style paths (e.g. `/c/Users/casey/...` or `ai-writings/artwork/...`)

## Fix:
Use full absolute Windows-style paths: `C:\Users\casey\ai-writings\artwork\crab_silicon_1_001.jpg`

## Verified Working:
Yes — tested on Inkling-Small model with deepinfra.com backend

## Source Session:
Residual Self-Image project, 2026-08-13