# Integration Contract Pattern Reference

> **Canonical structure for `INTEGRATION_GUIDE.md` — the frozen Interface Contract between extracted repositories**
> **Derived from:** `claw/SPREADSHEET_MOMENT_INTEGRATION_GUIDE.md` (2026-08-23)
> **Applies to:** All cross-repo integration points in SuperInstance ecosystem

---

## Required Sections (In Order)

### 1. Integration Vision
```markdown
# [Repo A] ↔ [Repo B] Integration Guide

> **[One-line integration vision]**
> **Audience:** [Engineer types on both sides]
> **Status:** [Design/Frozen/Implemented]
> **Contracts:** [Frozen — Do Not Change Without Architect]
```

**Must include:**
- Architecture diagram (ASCII/mermaid) showing data flow
- Clear statement of which contracts are frozen
- Versioning policy for contracts

### 2. Frozen Contracts

#### 2.1 Cell-Level API (Language A → Language B)
```markdown
### Cell-Level API ([Language A] → [Language B])

#### [Operation Name] (e.g., Spawn Agent in Cell)
```typescript
// [Language A] request
interface [Operation]Request {
  field1: Type;     // [constraint]
  field2: Type;     // [constraint]
}

interface [Operation]Response {
  field1: Type;     // [guarantee]
  field2: Type;     // [guarantee]
}
```

**Must include:**
- Request/Response interfaces in both languages
- Field-level constraints and guarantees
- Example usage

#### 2.2 WebSocket Protocol (Real-time)
```markdown
### WebSocket Protocol (Real-time [Entity A] ↔ [Entity B])

#### Connection
```typescript
// WebSocket: ws://[host]:[port]/[path]
// Auth: [Method]
```

#### Message Types
```typescript
type [Entity]WSMessageType =
  | 'MESSAGE_TYPE_1'  // [Description]
  | 'MESSAGE_TYPE_2'  // [Description]
  | 'HEARTBEAT';      // Keepalive
```

#### Message Schemas (One per type)
```json
{
  "type": "MESSAGE_TYPE_1",
  "payload": {
    "field": "value"
  },
  "timestamp": "ISO 8601"
}
```

**Must include:**
- Connection URL and auth method
- Complete message type enum
- JSON schema for each message type
- Direction annotations (A→B or B→A)

#### 2.3 REST API (Management & Inspection)
```markdown
### REST API (Management & Inspection)

```typescript
// Base: http://[host]:[port]

// [Operation]
[METHOD] /[resource]/[id]/[action]
Body: { ... }
Response: { ... }
```

**Must include:**
- Base URL
- All endpoints with method, path, body, response
- Auth requirements per endpoint

### 3. Data Flow Examples

```markdown
## 📊 Data Flow Examples

### Example 1: [Scenario Name]

**User Action:** [What triggers the flow]

```
1. [Step 1: Component A does X]
2. [Step 2: Component B responds Y]
3. [Step 3: Component A processes Z]
4. [Step 4: Result visible to user]
```

### Example 2: [Scenario Name]
...
```

**Must include:**
- 3+ end-to-end scenarios covering happy path, parallel, consensus
- Step-by-step with component interactions
- Visual flow (ASCII arrows)

### 4. Test Matrix

```markdown
## 🧪 Integration Test Matrix

| Test | [Side A] Action | [Side B] Response | Verification |
|------|-----------------|-------------------|--------------|
| **Spawn** | `=FORMULA(...)` in cell | 201 + ID + WS URL | Badge shows Dormant |
| **Periodic** | Wait N ms | MESSAGE_TYPE to cell | Value updates, telemetry streams |
| **Event** | Edit watched cell | STATE: Thinking → Processing | WS receives trigger |
| **Equip** | Click equip in UI | EQUIP message, module loaded | UI updates |
| **Consensus** | Anomaly detected | N votes → threshold → all act | Consensus badge on all |
| **Partition** | Disconnect network | Continues, queues packets | On reconnect: sync |
```

**Must include:**
- Test name, Side A action, Side B response, verification criteria
- Coverage: spawn, trigger types, equip/unequip, social patterns, failure modes
- Minimum 8 test cases

### 5. Security & Auth

```markdown
## 🔐 Security & Auth

### Token Model
- **[Token Type]:** [Lifetime], [Use case], [Rotation]
- **[Token Type]:** [Lifetime], [Use case], [Rotation]

### Permissions Matrix
| Role | Spawn | Equip | Trigger | Consensus | Telemetry | Kill |
|------|-------|-------|---------|-----------|-----------|------|
| [Role 1] | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| [Role 2] | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ |
```

**Must include:**
- All token types with lifetimes and rotation policy
- Permissions matrix covering all roles and operations

### 6. Deployment Topology

```markdown
## 📦 Deployment Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      DEPLOYMENT TOPOLOGY                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐         ┌──────────────┐         ┌────────┐  │
│  │  [Client]    │◄───────►│  [Platform]  │◄───────►│ [Core] │  │
│  │              │  HTTPS  │  (Lang A)    │  WS/REST│ (Lang B)│  │
│  └──────────────┘         └──────────────┘         └────┬───┘  │
│                                                           │      │
│                              ┌──────────────┐             │      │
│                              │  [Gateway]   │◄────────────┘      │
│                              │  (CNS)       │  [Protocol]        │
│                              └──────────────┘                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Local Development:**
- `[Platform]`: `[command]` (port XXXX)
- `[Core]`: `[command]` (port XXXX)
- `[Gateway]`: `[command]` (file-based)

**Production:**
- `[Platform]`: [Target]
- `[Core]`: [Target]
- `[Gateway]`: [Target]
```

**Must include:**
- ASCII topology diagram
- Local dev commands and ports
- Production targets

---

## Contract Freezing Protocol

1. **Architect Review:** Schema Architect reviews and approves
2. **Version Tag:** Contract tagged `contract-v1.0.0` in both repos
3. **Immutable:** Changes require Architect approval + version bump
4. **Breaking Changes:** New version = new contract document, old supported for 1 release cycle
5. **Deprecation:** 30-day notice via CNS broadcast before removal

---

## Quality Gates

Before declaring an Integration Contract complete:
- [ ] All 6 sections present and complete
- [ ] Cell-level API has request/response for every operation
- [ ] WebSocket protocol has complete message schemas (no `any` types)
- [ ] REST API covers all management operations
- [ ] 3+ data flow examples covering happy/parallel/failure paths
- [ ] Test matrix has 8+ cases with verifiable criteria
- [ ] Security model covers all token types and roles
- [ ] Deployment topology matches actual infrastructure
- [ ] Contract version tagged in both repos

---

## Anti-Patterns

| Anti-Pattern | Symptom | Fix |
|--------------|---------|-----|
| **Living Contract** | Contract changes without version bump | Freeze protocol: version tag + Architect approval required |
| **Type Drift** | TS interfaces don't match Rust structs | Shared schema source (JSON Schema) + codegen both sides |
| **Missing Failure Paths** | Test matrix only covers happy path | Mandatory: partition, OOM, timeout, consensus failure |
| **Auth Ambiguity** | "Bearer token" without lifetime/rotation | Explicit token model table with all types |
| **Topology Drift** | Local dev doesn't match production | Contract includes both; CI validates both |