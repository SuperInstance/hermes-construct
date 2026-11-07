# Phase 1 Week 3 — Resilience Implementation

**Date**: 2026-08-24  
**Focus**: DSH Client resilience (retry, circuit breaker, health checks)

---

## Retry with Exponential Backoff

**File**: `cns-dsh-bridge/src/dsh_client.rs`

### RetryConfig
```rust
pub struct RetryConfig {
    pub max_attempts: u32,           // default: 3
    pub base_delay: Duration,        // default: 100ms
    pub max_delay: Duration,         // default: 10s
    pub exponential_base: f64,       // default: 2.0
}
```

### Implementation
```rust
async fn execute_with_resilience<F, Fut, T>(&self, operation: F) -> Result<T>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = Result<T>> + Send,
{
    let mut delay = self.retry_config.base_delay;
    for attempt in 0..self.retry_config.max_attempts {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < self.retry_config.max_attempts - 1 => {
                self.circuit_breaker.record_failure().await;
                sleep(delay).await;
                delay = std::cmp::min(
                    Duration::from_millis((delay.as_millis() as f64 * self.retry_config.exponential_base) as u64),
                    self.retry_config.max_delay,
                );
            }
            Err(e) => {
                self.circuit_breaker.record_failure().await;
                return Err(e);
            }
        }
    }
    unreachable!()
}
```

### Usage
```rust
let client = DSHClient::with_retry_config(
    "plugin-id".to_string(),
    "http://dsh:8080".to_string(),
    RetryConfig { max_attempts: 5, base_delay: Duration::from_millis(50), ... }
).await?;
```

---

## Circuit Breaker

**File**: `cns-dsh-bridge/src/dsh_client.rs`

### States
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,     // Normal operation
    Open,       // Failing fast
    HalfOpen,   // Testing recovery
}
```

### Configuration
```rust
pub struct CircuitBreaker {
    failure_threshold: u32,    // failures before Open (default: 5)
    success_threshold: u32,    // successes in HalfOpen before Closed (default: 3)
    timeout: Duration,         // Open → HalfOpen delay (default: 30s)
}
```

### State Transitions
```
Closed --(failures >= threshold)--> Open
Open --(timeout elapsed)--> HalfOpen
HalfOpen --(successes >= threshold)--> Closed
HalfOpen --(any failure)--> Open
```

### Methods
```rust
pub async fn can_execute(&self) -> bool {
    match self.state().await {
        CircuitState::Closed => true,
        CircuitState::Open => false,
        CircuitState::HalfOpen => true,  // Allow test requests
    }
}

pub async fn record_success(&self) { ... }
pub async fn record_failure(&self) { ... }
pub async fn state(&self) -> CircuitState { ... }
```

---

## Health Checks

**File**: `cns-dsh-bridge/src/dsh_client.rs`

### HealthCheckResult
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub healthy: bool,
    pub endpoint: String,
    pub circuit_state: String,
    pub latency_ms: Option<u64>,
    pub timestamp: String,
    pub error: Option<String>,
}
```

### Implementation
```rust
pub async fn health_check(&self) -> HealthCheckResult {
    let start = Instant::now();
    let circuit_state = self.circuit_breaker.state().await;
    
    let result = self.client.get(&format!("{}/health", self.endpoint)).send().await;
    let latency_ms = start.elapsed().as_millis() as u64;
    
    match result {
        Ok(response) if response.status().is_success() => {
            self.circuit_breaker.record_success().await;
            HealthCheckResult { healthy: true, latency_ms: Some(latency_ms), ... }
        }
        Ok(response) => {
            let err = response.text().await.unwrap_or_default();
            let status = response.status();
            self.circuit_breaker.record_failure().await;
            HealthCheckResult { healthy: false, latency_ms: Some(latency_ms), error: Some(format!("HTTP {}: {}", status, err)), ... }
        }
        Err(e) => {
            self.circuit_breaker.record_failure().await;
            HealthCheckResult { healthy: false, latency_ms: Some(latency_ms), error: Some(e.to_string()), ... }
        }
    }
}
```

---

## Resilience Tests

**File**: `cns-dsh-bridge/tests/resilience_tests.rs` (9 tests, all passing)

| Test | Description |
|------|-------------|
| `test_circuit_breaker_closed_by_default` | CircuitBreaker starts Closed |
| `test_circuit_breaker_opens_after_threshold` | Opens after failure threshold |
| `test_circuit_breaker_half_open_after_timeout` | Transitions to HalfOpen after timeout |
| `test_circuit_breaker_closes_after_success_threshold` | Closes after success threshold in HalfOpen |
| `test_circuit_breaker_reopens_on_half_open_failure` | Reopens on failure in HalfOpen |
| `test_retry_config_defaults` | Default RetryConfig values |
| `test_dsh_client_creation` | DSHClient creates with Closed circuit |
| `test_dsh_client_with_custom_retry` | Custom RetryConfig works |
| `test_health_check_result_serialization` | HealthCheckResult serializes to JSON |

---

## Test Results Summary

**Total: 20 tests passing**
- Unit (translator): 4
- Integration (CNS↔DSH↔Plugin): 3
- Live CNS (real inbox/outbox): 3
- Resilience (circuit breaker, retry, health): 9
- Saddle plugin: 1