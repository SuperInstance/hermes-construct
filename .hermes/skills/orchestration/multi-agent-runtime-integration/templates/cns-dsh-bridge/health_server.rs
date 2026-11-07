# Plugin Health Server Template

```rust
//! Health Server — Shared HTTP health endpoints for DSH plugins
//! Provides /health, /metrics, /ready endpoints

use crate::{HealthCheckResult, CircuitState};
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use prometheus::{Encoder, Registry, TextEncoder};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Shared health state for plugins
#[derive(Debug, Clone)]
pub struct PluginHealthState {
    pub plugin_id: String,
    pub version: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub circuit_state: Arc<RwLock<CircuitState>>,
    pub last_health_check: Arc<RwLock<Option<HealthCheckResult>>>,
    pub request_count: Arc<std::sync::atomic::AtomicU64>,
    pub error_count: Arc<std::sync::atomic::AtomicU64>,
}

impl PluginHealthState {
    pub fn new(plugin_id: String, version: String) -> Self {
        Self {
            plugin_id,
            version,
            started_at: chrono::Utc::now(),
            circuit_state: Arc::new(RwLock::new(CircuitState::Closed)),
            last_health_check: Arc::new(RwLock::new(None)),
            request_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub async fn update_circuit_state(&self, state: CircuitState) {
        *self.circuit_state.write().await = state;
    }

    pub async fn update_health_check(&self, result: HealthCheckResult) {
        *self.last_health_check.write().await = Some(result);
    }

    pub fn increment_requests(&self) {
        self.request_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn increment_errors(&self) {
        self.error_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Health check response
#[derive(serde::Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub plugin_id: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub circuit_state: String,
    pub timestamp: String,
    pub checks: HealthChecks,
}

#[derive(serde::Serialize)]
pub struct HealthChecks {
    pub dsh_endpoint: ComponentHealth,
    pub circuit_breaker: ComponentHealth,
    pub event_processing: ComponentHealth,
}

#[derive(serde::Serialize)]
pub struct ComponentHealth {
    pub status: String,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

/// Readiness response
#[derive(serde::Serialize)]
pub struct ReadyResponse {
    pub ready: bool,
    pub plugin_id: String,
    pub reason: Option<String>,
}

/// Start health server
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

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Health server started on {}", bind_addr);
    
    axum::serve(listener, app).await?;
    Ok(())
}

/// GET /health — Full health check
async fn health_handler(
    State((health_state, _)): State<(Arc<PluginHealthState>, Option<Registry>)>,
) -> impl IntoResponse {
    health_state.increment_requests();
    
    let circuit_state = health_state.circuit_state.read().await;
    let last_check = health_state.last_health_check.read().await;
    
    let uptime = chrono::Utc::now().signed_duration_since(health_state.started_at).num_seconds().max(0) as u64;
    
    let dsh_health = if let Some(check) = last_check.as_ref() {
        ComponentHealth {
            status: if check.healthy { "healthy" } else { "unhealthy" }.to_string(),
            latency_ms: check.latency_ms,
            error: check.error.clone(),
        }
    } else {
        ComponentHealth {
            status: "unknown".to_string(),
            latency_ms: None,
            error: Some("No health check performed yet".to_string()),
        }
    };
    
    let circuit_health = ComponentHealth {
        status: if matches!(*circuit_state, CircuitState::Closed) { "healthy" } else { "degraded" }.to_string(),
        latency_ms: None,
        error: if matches!(*circuit_state, CircuitState::Open) { Some("Circuit breaker open".to_string()) } else { None },
    };
    
    let event_health = ComponentHealth {
        status: "healthy".to_string(),
        latency_ms: None,
        error: None,
    };
    
    let overall_healthy = last_check.as_ref().map(|c| c.healthy).unwrap_or(true) && matches!(*circuit_state, CircuitState::Closed);
    
    let response = HealthResponse {
        status: if overall_healthy { "healthy" } else { "degraded" }.to_string(),
        plugin_id: health_state.plugin_id.clone(),
        version: health_state.version.clone(),
        uptime_seconds: uptime,
        circuit_state: format!("{:?}", *circuit_state),
        timestamp: chrono::Utc::now().to_rfc3339(),
        checks: HealthChecks {
            dsh_endpoint: dsh_health,
            circuit_breaker: circuit_health,
            event_processing: event_health,
        },
    };
    
    let status_code = if response.status == "healthy" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status_code, Json(response))
}

/// GET /ready — Readiness probe
async fn ready_handler(
    State((health_state, _)): State<(Arc<PluginHealthState>, Option<Registry>)>,
) -> impl IntoResponse {
    health_state.increment_requests();
    
    let circuit_state = health_state.circuit_state.read().await;
    let last_check = health_state.last_health_check.read().await;
    
    let ready = matches!(*circuit_state, CircuitState::Closed) 
        && last_check.as_ref().map(|c| c.healthy).unwrap_or(true);
    
    let response = ReadyResponse {
        ready,
        plugin_id: health_state.plugin_id.clone(),
        reason: if !ready {
            if matches!(*circuit_state, CircuitState::Open) {
                Some("Circuit breaker open".to_string())
            } else if let Some(check) = last_check.as_ref() {
                if !check.healthy {
                    Some("DSH endpoint unhealthy".to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        },
    };
    
    let status_code = if ready { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status_code, Json(response))
}

/// GET /metrics — Prometheus metrics
async fn metrics_handler(
    State((health_state, registry)): State<(Arc<PluginHealthState>, Option<Registry>)>,
) -> impl IntoResponse {
    health_state.increment_requests();
    
    let circuit_state = health_state.circuit_state.read().await;
    let requests = health_state.request_count.load(std::sync::atomic::Ordering::Relaxed);
    let errors = health_state.error_count.load(std::sync::atomic::Ordering::Relaxed);
    let uptime = chrono::Utc::now().signed_duration_since(health_state.started_at).num_seconds().max(0) as u64;
    
    // Build custom metrics
    let mut metrics = format!(
        "# HELP plugin_requests_total Total requests\n# TYPE plugin_requests_total counter\nplugin_requests_total{{plugin=\"{}\"}} {}\n",
        health_state.plugin_id, requests
    );
    metrics.push_str(&format!(
        "# HELP plugin_errors_total Total errors\n# TYPE plugin_errors_total counter\nplugin_errors_total{{plugin=\"{}\"}} {}\n",
        health_state.plugin_id, errors
    ));
    metrics.push_str(&format!(
        "# HELP plugin_uptime_seconds Uptime in seconds\n# TYPE plugin_uptime_seconds gauge\nplugin_uptime_seconds{{plugin=\"{}\"}} {}\n",
        health_state.plugin_id, uptime
    ));
    metrics.push_str(&format!(
        "# HELP plugin_circuit_state Circuit breaker state (0=closed, 1=open, 2=half-open)\n# TYPE plugin_circuit_state gauge\nplugin_circuit_state{{plugin=\"{}\"}} {}\n",
        health_state.plugin_id, 
        match *circuit_state {
            CircuitState::Closed => 0,
            CircuitState::Open => 1,
            CircuitState::HalfOpen => 2,
        }
    ));
    
    // Add Prometheus registry metrics if available
    if let Some(reg) = registry {
        let encoder = TextEncoder::new();
        let metric_families = reg.gather();
        let mut buffer = Vec::new();
        if encoder.encode(&metric_families, &mut buffer).is_ok() {
            metrics.push_str(&String::from_utf8_lossy(&buffer));
        }
    }
    
    (
        StatusCode::OK,
        [("Content-Type", "text/plain; version=0.0.4")],
        metrics,
    )
}
```