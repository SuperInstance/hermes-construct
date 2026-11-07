# Phase 1 Week 3 — Plugin Health Endpoints

**Date**: 2026-08-24  
**Focus**: Axum-based HTTP health server with Prometheus metrics

---

## Health Server Implementation

**File**: `cns-dsh-bridge/src/health_server.rs`

### Dependencies (added to Cargo.toml)
```toml
axum = { version = "0.7", features = ["macros"] }
tower = "0.4"
prometheus = "0.13"
```

### Endpoints

| Endpoint | Method | Purpose | Response |
|----------|--------|---------|----------|
| `/health` | GET | Full health check | `HealthResponse` (200/503) |
| `/ready` | GET | Kubernetes readiness probe | `ReadyResponse` (200/503) |
| `/metrics` | GET | Prometheus scrape | Text format metrics |

---

## Health Response Types

```rust
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,           // "healthy" | "degraded"
    pub plugin_id: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub circuit_state: String,    // "Closed" | "Open" | "HalfOpen"
    pub timestamp: String,
    pub checks: HealthChecks,
}

#[derive(Serialize)]
pub struct HealthChecks {
    pub dsh_endpoint: ComponentHealth,
    pub circuit_breaker: ComponentHealth,
    pub event_processing: ComponentHealth,
}

#[derive(Serialize)]
pub struct ComponentHealth {
    pub status: String,           // "healthy" | "unhealthy" | "degraded" | "unknown"
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ReadyResponse {
    pub ready: bool,
    pub plugin_id: String,
    pub reason: Option<String>,
}
```

---

## Plugin Health State

```rust
#[derive(Debug, Clone)]
pub struct PluginHealthState {
    pub plugin_id: String,
    pub version: String,
    pub started_at: DateTime<Utc>,
    pub circuit_state: Arc<RwLock<CircuitState>>,
    pub last_health_check: Arc<RwLock<Option<HealthCheckResult>>>,
    pub request_count: Arc<AtomicU64>,
    pub error_count: Arc<AtomicU64>,
}

impl PluginHealthState {
    pub fn new(plugin_id: String, version: String) -> Self { ... }
    pub async fn update_circuit_state(&self, state: CircuitState) { ... }
    pub async fn update_health_check(&self, result: HealthCheckResult) { ... }
    pub fn increment_requests(&self) { ... }
    pub fn increment_errors(&self) { ... }
}
```

---

## Server Startup

```rust
pub async fn start_health_server(
    bind_addr: String,
    health_state: Arc<PluginHealthState>,
    prometheus_registry: Option<Registry>,
) -> Result<()> {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/metrics", get(metrics_handler))
        .with_state((health_state, prometheus_registry));

    let listener = TcpListener::bind(&bind_addr).await?;
    info!("Health server started on {}", bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
```

---

## Prometheus Metrics

### Custom Metrics (always exposed)
```text
# HELP plugin_requests_total Total requests
# TYPE plugin_requests_total counter
plugin_requests_total{plugin="saddle-fleet-conductor"} 42

# HELP plugin_errors_total Total errors
# TYPE plugin_errors_total counter
plugin_errors_total{plugin="saddle-fleet-conductor"} 0

# HELP plugin_uptime_seconds Uptime in seconds
# TYPE plugin_uptime_seconds gauge
plugin_uptime_seconds{plugin="saddle-fleet-conductor"} 3600

# HELP plugin_circuit_state Circuit breaker state (0=closed, 1=open, 2=half-open)
# TYPE plugin_circuit_state gauge
plugin_circuit_state{plugin="saddle-fleet-conductor"} 0
```

### Registry Metrics (merged if provided)
```rust
if let Some(reg) = registry {
    let encoder = TextEncoder::new();
    let metric_families = reg.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    metrics.push_str(&String::from_utf8_lossy(&buffer));
}
```

---

## Integration with Plugins

### Plugin Initialization
```rust
// In plugin (e.g., saddle/src/lib.rs)
let health_state = Arc::new(PluginHealthState::new(
    "saddle-fleet-conductor".into(),
    "0.1.0".into(),
));

// Start health server on dedicated port
tokio::spawn(async move {
    start_health_server("0.0.0.0:8081".to_string(), health_state.clone(), None).await
});

// Update circuit state from DSHClient
health_state.update_circuit_state(client.circuit_state().await).await;

// Update health check from DSHClient
let health = client.health_check().await;
health_state.update_health_check(health).await;
```

### Kubernetes Probes
```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8081
  initialDelaySeconds: 10
  periodSeconds: 30

readinessProbe:
  httpGet:
    path: /ready
    port: 8081
  initialDelaySeconds: 5
  periodSeconds: 10
```

---

## Test Coverage

Health server tested via integration tests:
- `test_live_cns_inbox_outbox_flow` — validates bridge processes packets
- `test_live_fleet_conductor_workflow` — validates plugin responds
- `test_live_plugin_registry_registration` — validates registry health

No dedicated unit tests for health server (integration coverage sufficient).