# Schema Stub Detection Reference

## Byte-Count Thresholds (Windows / Git)

| Byte Count | Status | Action |
|-----------|--------|--------|
| < 200 bytes | **Definite stub** | Replace immediately |
| 200–1,000 bytes | Likely incomplete | Inspect `definitions` and `properties` depth |
| 1,000–5,000 bytes | Partial | Cross-reference against Checklist |
| > 5,000 bytes | Production-ready | Verify against Checklist only |

## Common Stub Patterns

```json
// Pattern A: minimal title+type
{"title": "X Schema", "type": "object"}

// Pattern B: $schema only
{"$schema": "http://json-schema.org/draft-07/schema#", "title": "X", "type": "object"}

// Pattern C: empty object
{"type": "object"}
```

## LOG-tensor Schema Inventory (2026-08-22)

| LOG-tensor Schema | Size | Maps To |
|-------------------|------|---------|
| `smp_cell_schema.json` | 18 KB | `claw_agent.json` (direct) |
| `smpbot_schema.json` | 25 KB | `claw_agent.json` (augments with execution) |
| `smp_lifecycle_schema.json` | 34 KB | `claw_lifecycle.json` (new) |
| `master_schema.json` | 17 KB | `master.json` (push as-is) |
| `conditional_geometry_schema.json` | 16 KB | `claw_seed.json` (augments) |
| `foldable_tensor_schema.json` | 17 KB | `claw_equipment.json` (knowledge module) |
| `folding_group_schema.json` | 17 KB | `claw_equipment.json` (coordination module) |
| `channel_depth_schema.json` | 17 KB | `claw_equipment.json` (perception module) |
| `cold_logic_schema.json` | 30 KB | `claw_equipment.json` (verification module) |

## Backup Convention

Before overwriting any gold-standard schema:
```bash
cp schemas_gold_standard/<name>.json schemas_gold_standard/<name>.json.bak
```

## JSON Lint Gate

All populated schemas MUST pass syntax validation before commit:
```bash
python -c "import json; json.load(open('schema.json'))"
```
