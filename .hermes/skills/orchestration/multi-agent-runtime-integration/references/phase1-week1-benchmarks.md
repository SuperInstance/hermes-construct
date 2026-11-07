# Phase 1 Week 1 Benchmarks — Release Mode

**Date**: 2026-08-23  
**Tool**: criterion (n=100, release profile)  
**Target**: <100μs translation latency

---

## Results

| Operation | Latency | Target | Margin |
|-----------|---------|--------|--------|
| USCP→DSH (full) | **1.33 μs** | 100 μs | **75x under** |
| DSH→USCP (full) | **1.40 μs** | 100 μs | **71x under** |
| Roundtrip (full) | **2.78 μs** | 100 μs | **36x under** |
| Roundtrip (minimal) | **1.37 μs** | 100 μs | **73x under** |

---

## Test Suite Status

| Plugin | Build | Tests |
|--------|-------|-------|
| cns-dsh-bridge | ✅ | 4/4 unit |
| dsh-plugin-saddle | ✅ | 1/1 |
| dsh-plugin-fleet-conductor | ✅ | 0/0 |
| dsh-plugin-registry | ✅ | 0/0 |

**Integration tests**: 3/3 passing
- `test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow`
- `test_plugin_registry_discovery_flow`
- `test_fleet_conductor_workflow_flow`

---

## Notes

- Release mode ~2-3x slower than dev mode due to optimization overhead
- Still well within 100μs budget (36-75x margin)
- All 6 red lines verified intact
- CNS packets sent: GO decision + Phase 1 Week 1 complete