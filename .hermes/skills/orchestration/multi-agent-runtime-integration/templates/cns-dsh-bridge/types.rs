//! CNS↔DSH Bridge — Core Types Template
//! Copy to new bridge crate and customize

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::Display;

/// USCP v3 Packet — Canonical CNS packet format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCPv3Packet {
    pub header: USCPHeader,
    pub body: USCPBody,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCPHeader {
    pub version: String,
    pub packet_id: String,
    pub source_id: String,
    pub destination_id: String,
    pub timestamp: String,
    pub intent: USCPIntent,
    pub priority: USCPPriority,
    pub correlation_id: Option<String>,
    pub ttl: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USCPBody {
    pub payload: serde_json::Value,
    pub extensions: Option<USCPExtensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub key_id: String,
    pub signature: String,
}

/// USCPIntent — All fleet intents (exhaustive for bidirectional mapping)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum USCPIntent {
    FleetPulse,
    SensoryData,
    ExecutePlan,
    DelegateTask,
    ConsensusPropose,
    ConsensusVote,
    ConsensusResult,
    AgentRegister,
    AgentUnregister,
    EquipmentEquip,
    EquipmentUnequip,
    SeedTrain,
    SeedDeploy,
    #[serde(other)]
    Custom(String),
}

/// USCPPriority — Priority levels preserved end-to-end
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum USCPPriority {
    Critical,
    High,
    Normal,
    Low,
}

/// USCPExtensions — 7 Telemetry Dimensions (SuperInstance Resonance Protocol)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct USCPExtensions {
    pub gamma_eta: Option<GammaEta>,      // Coherence γ & Plasticity η
    pub thermal: Option<Thermal>,         // Temperature T, delta, p_melt
    pub creative: Option<Creative>,       // Creative delta, zone
    pub melt: Option<Melt>,               // Melt depth, trigger
    pub molt: Option<Molt>,               // State transition from→to, catalyst
    pub uncertainty: Option<Uncertainty>, // Value, source
    pub anomaly: Option<Anomaly>,         // Score, description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaEta {
    pub gamma: f64,  // Coherence [0,1]
    pub eta: f64,    // Plasticity [0,1]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thermal {
    pub T: f64,         // Temperature
    pub delta: f64,     // Rate of change
    pub p_melt: f64,    // Melt probability
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creative {
    pub delta: f64,              // Creative pressure
    pub zone: CreativeZone,      // Crystallized, Creative, Melt
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "PascalCase")]
pub enum CreativeZone {
    Crystallized,
    Creative,
    Melt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Melt {
    pub depth: f64,
    pub trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Molt {
    pub from_state: String,
    pub to_state: String,
    pub catalyst: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uncertainty {
    pub value: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub score: f64,
    pub description: String,
}

/// DSH Event — DeepSeek Harness event format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSHEvent {
    pub event_id: String,
    pub event_type: String,
    pub source_plugin: String,
    pub target_plugin: Option<String>,
    pub timestamp: String,
    pub payload: serde_json::Value,
    pub metadata: DSHMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSHMetadata {
    pub priority: DSHPriority,
    pub correlation_id: Option<String>,
    pub extensions: Option<DSHExtensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DSHPriority {
    Critical,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DSHExtensions {
    pub gamma_eta: Option<GammaEta>,
    pub thermal: Option<Thermal>,
    pub creative: Option<Creative>,
    pub melt: Option<Melt>,
    pub molt: Option<Molt>,
    pub uncertainty: Option<Uncertainty>,
    pub anomaly: Option<Anomaly>,
}

/// Plugin Manifest — DSH plugin registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub config_schema: serde_json::Value,
}

/// Bridge Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub agent_id: String,
    pub cns_inbox: String,
    pub cns_outbox: String,
    pub dsh_endpoint: String,
    pub poll_interval_ms: u64,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            agent_id: "hermes-cns".into(),
            cns_inbox: dirs::home_dir()
                .unwrap()
                .join(".hermes/cns_inbox")
                .to_string_lossy()
                .into(),
            cns_outbox: dirs::home_dir()
                .unwrap()
                .join(".hermes/cns_outbox")
                .to_string_lossy()
                .into(),
            dsh_endpoint: "http://localhost:8080".into(),
            poll_interval_ms: 1000,
        }
    }
}