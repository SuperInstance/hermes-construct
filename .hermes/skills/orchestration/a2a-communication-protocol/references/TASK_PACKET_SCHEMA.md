# Task Packet Schema Reference
**Audience:** Hermes, subagent runners, CNS monitor
**Source of truth:** `fleet/docs/delegation/TASK_PACKET_TEMPLATE.md`
**Load when:** Dispatching a subagent task via CNS, or reading a subagent response.

## Dispatch Packet (Outbound)

```json
{
  "header": {
    "packet_id": "task-pack-<uuid>",
    "version": "1.0",
    "created_at": "<ISO timestamp>",
    "dispatcher": "hermes",
    "intent": "REQUEST_REASONING | EXECUTE_PLAN | SENSORY_DATA",
    "priority": "P0 | P1 | P2 | P3",
    "deadline": "<ISO timestamp | null>"
  },
  "body": {
    "task_id": "<unique task id>",
    "goal": "<one-line description>",
    "context": "<narrative context>",
    "repo": "claw | constrainttheory | spreadsheet-moment | dodecet-encoder",
    "module": "claw-core | claw-api | claw-schema | agent-core | cudaclaw-bridge",
    "acceptance_criteria": ["<verifiable criteria>"],
    "constraints": {
      "schema_file": "<relative path>",
      "tests_must_pass": true,
      "max_loc_per_module": 500,
      "no_legacy_imports": true,
      "origin_tracking_required": true
    },
    "artifacts_to_create": ["<file path>"],
    "toolsets": ["terminal", "file", "web"]
  },
  "signature": {
    "dispatcher_id": "hermes-default",
    "schema_hash": "<SHA-256>",
    "trace_chain": "<parent task id>"
  }
}
```

## Response Packet (Inbound)

```json
{
  "task_id": "task-id",
  "status": "COMPLETED | FAILED | BLOCKED",
  "summary": "<2-3 sentence summary>",
  "artifacts": [{ "path": "relative/path", "action": "created | modified | deleted" }],
  "verification": {
    "tests_passed": true,
    "benchmark_results": { "bench_name": value_ms },
    "schema_valid": true
  },
  "blocker": "<error string | null>",
  "next_recommended_task": "<optional>"
}
```

## Intent Reference

| Intent | Meaning | Expected Action |
|--------|---------|-----------------|
| REQUEST_REASONING | Analyze/design | Return analysis doc |
| EXECUTE_PLAN | Implement | Return code + test output |
| SENSORY_DATA | Inspect filesystem/repo | Return structured observation |

## Priority Reference

| Priority | Deadline Behavior | Escalation |
|----------|-----------------|------------|
| P0 | Immediate. Blocking other work. | Notify Hermes on failure |
| P1 | Within current sprint | Notify on missed deadline |
| P2 | Within 1 week | Weekly check-in |
| P3 | Backlog | No notification |

## Logging Convention

| Direction | Location |
|-----------|----------|
| Dispatch (outbound) | `fleet/docs/delegation/task_log.jsonl` |
| Response (inbound) | `~/.hermes/cns_inbox/` |