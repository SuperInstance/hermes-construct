# Resilient DSH Client Template

```rust
//! CNS↔DSH Bridge — DSH Client (Resilient)
//! Communicates with DeepSeek Harness event bus with retry, circuit breaker, health checks

use crate::types::{DSHEvent, DSHMetadata, DSHPriority, PluginManifest, HealthCheckResult, CircuitState};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Retry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub exponential_base: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            exponential_base: 2.0,
        }
    }
}

/// Circuit Breaker State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit Breaker
#[derive(Debug)]
pub struct CircuitBreaker {
    failure_threshold: u32,
    success_threshold: u32,
    timeout: Duration,
    failure_count: Arc<Mutex<u32>>,
    success_count: Arc<Mutex<u32>>,
    state: Arc<RwLock<CircuitState>>,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, success_threshold: u32, timeout: Duration) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            timeout,
            failure_count: Arc::new(Mutex::new(0)),
            success_count: Arc::new(Mutex::new(0)),
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            last_failure_time: Arc::new(Mutex::new(None)),
        }
    }
    
    pub async fn can_execute(&self) -> bool {
        let mut state = self.state.write().await;
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout elapsed
                if let Some(last_failure) = *self.last_failure_time.lock().await {
                    if last_failure.elapsed() >= self.timeout {
                        *state = CircuitState::HalfOpen;
                        *self.success_count.lock().await = 0;
                        info!("Circuit breaker: Open -> HalfOpen");
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }
    
    pub async fn record_success(&self) {
        let state = self.state.read().await;
        if *state == CircuitState::HalfOpen {
            let mut success_count = self.success_count.lock().await;
            *success_count += 1;
            if *success_count >= self.success_threshold {
                drop(state);
                *self.state.write().await = CircuitState::Closed;
                *self.failure_count.lock().await = 0;
                info!("Circuit breaker: HalfOpen -> Closed");
            }
        } else if *state == CircuitState::Closed {
            *self.failure_count.lock().await = 0;
        }
    }
    
    pub async fn record_failure(&self) {
        let mut state = self.state.write().await;
        match *state {
            CircuitState::Closed => {
                let mut failure_count = self.failure_count.lock().await;
                *failure_count += 1;
                *self.last_failure_time.lock().await = Some(Instant::now());
                if *failure_count >= self.failure_threshold {
                    *state = CircuitState::Open;
                    warn!("Circuit breaker: Closed -> Open (threshold reached)");
                }
            }
            CircuitState::HalfOpen => {
                *state = CircuitState::Open;
                *self.last_failure_time.lock().await = Some(Instant::now());
                warn!("Circuit breaker: HalfOpen -> Open (failure in half-open)");
            }
            CircuitState::Open => {
                *self.last_failure_time.lock().await = Some(Instant::now());
            }
        }
    }
    
    pub async fn state(&self) -> CircuitState {
        *self.state.read().await
    }
}

/// DSH Client
pub struct DSHClient {
    plugin_id: String,
    endpoint: String,
    client: Client,
    retry_config: RetryConfig,
    circuit_breaker: CircuitBreaker,
    local_event_queue: Arc<Mutex<Vec<DSHEvent>>>,
}

impl DSHClient {
    pub async fn new(plugin_id: String, endpoint: String) -> Result<Self> {
        Self::with_retry_config(plugin_id, endpoint, RetryConfig::default()).await
    }
    
    pub async fn with_retry_config(
        plugin_id: String,
        endpoint: String,
        retry_config: RetryConfig,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        let circuit_breaker = CircuitBreaker::new(5, 3, Duration::from_secs(30));
        
        Ok(Self {
            plugin_id,
            endpoint,
            client,
            retry_config,
            circuit_breaker,
            local_event_queue: Arc::new(Mutex::new(Vec::new())),
        })
    }
    
    async fn execute_with_resilience<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T>> + Send,
    {
        let mut delay = self.retry_config.base_delay;
        for attempt in 0..self.retry_config.max_attempts {
            if !self.circuit_breaker.can_execute().await {
                return Err(anyhow::anyhow!("Circuit breaker open"));
            }
            
            match operation().await {
                Ok(result) => {
                    self.circuit_breaker.record_success().await;
                    return Ok(result);
                }
                Err(e) if attempt < self.retry_config.max_attempts - 1 => {
                    self.circuit_breaker.record_failure().await;
                    warn!("DSH request failed (attempt {}/{}): {}", attempt + 1, self.retry_config.max_attempts, e);
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
    
    pub async fn register_plugin(&self, manifest: PluginManifest) -> Result<()> {
        self.execute_with_resilience(|| async {
            let response = self.client
                .post(&format!("{}/plugins/register", self.endpoint))
                .json(&manifest)
                .send()
                .await?;
            
            if !response.status().is_success() {
                let err = response.text().await.unwrap_or_default();
                return Err(anyhow::anyhow!("Registration failed: {}", err));
            }
            Ok(())
        }).await
    }
    
    pub async fn emit(&self, event: DSHEvent) -> Result<()> {
        self.execute_with_resilience(|| async {
            let response = self.client
                .post(&format!("{}/events", self.endpoint))
                .json(&event)
                .send()
                .await?;
            
            if !response.status().is_success() {
                let err = response.text().await.unwrap_or_default();
                return Err(anyhow::anyhow!("Emit failed: {}", err));
            }
            Ok(())
        }).await
    }
    
    pub async fn poll(&self) -> Result<Vec<DSHEvent>> {
        self.execute_with_resilience(|| async {
            let response = self.client
                .get(&format!("{}/events/poll", self.endpoint))
                .query(&[("plugin_id", &self.plugin_id)])
                .send()
                .await?;
            
            if !response.status().is_success() {
                let err = response.text().await.unwrap_or_default();
                return Err(anyhow::anyhow!("Poll failed: {}", err));
            }
            
            let events: Vec<DSHEvent> = response.json().await?;
            Ok(events)
        }).await
    }
    
    pub async fn health_check(&self) -> HealthCheckResult {
        let start = Instant::now();
        let circuit_state = self.circuit_breaker.state().await;
        
        let result = self.client
            .get(&format!("{}/health", self.endpoint))
            .send()
            .await;
        
        let latency_ms = start.elapsed().as_millis() as u64;
        
        match result {
            Ok(response) if response.status().is_success() => {
                self.circuit_breaker.record_success().await;
                HealthCheckResult {
                    healthy: true,
                    endpoint: self.endpoint.clone(),
                    circuit_state: format!("{:?}", circuit_state),
                    latency_ms: Some(latency_ms),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    error: None,
                }
            }
            Ok(response) => {
                let status = response.status();
                let err = response.text().await.unwrap_or_default();
                self.circuit_breaker.record_failure().await;
                HealthCheckResult {
                    healthy: false,
                    endpoint: self.endpoint.clone(),
                    circuit_state: format!("{:?}", circuit_state),
                    latency_ms: Some(latency_ms),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    error: Some(format!("HTTP {}: {}", status, err)),
                }
            }
            Err(e) => {
                self.circuit_breaker.record_failure().await;
                HealthCheckResult {
                    healthy: false,
                    endpoint: self.endpoint.clone(),
                    circuit_state: format!("{:?}", circuit_state),
                    latency_ms: Some(latency_ms),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    error: Some(e.to_string()),
                }
            }
        }
    }
    
    pub async fn circuit_state(&self) -> CircuitState {
        self.circuit_breaker.state().await
    }
    
    pub async fn inject_event(&self, event: DSHEvent) {
        self.local_event_queue.lock().await.push(event);
    }
    
    pub async fn poll_local(&self) -> Vec<DSHEvent> {
        let mut queue = self.local_event_queue.lock().await;
        let events = queue.drain(..).collect();
        events
    }
}
```