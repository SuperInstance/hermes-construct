//! Integration Test Template: CNS → DSH → Plugin → DSH → CNS
//! Copy to cns-dsh-bridge/tests/integration_flow.rs and customize

use cns_dsh_bridge::{
    CNSDSHTranslator, BridgeConfig, DSHEvent, DSHMetadata, DSHPriority,
    USCPv3Packet, USCPHeader, USCPBody, USCPIntent, USCPPriority, USCPExtensions,
    GammaEta, Thermal, Creative, CreativeZone, Uncertainty,
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[tokio::test]
async fn test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow() -> Result<()> {
    let translator = Arc::new(Mutex::new(CNSDSHTranslator::new("test-bridge".to_string())));
    
    // Step 1: Create CNS USCP v3 packet with all 7 extensions
    let cns_packet = USCPv3Packet {
        header: USCPHeader {
            version: "3.0".to_string(),
            packet_id: Uuid::new_v4().to_string(),
            source_id: "test-source".to_string(),
            destination_id: "test-target".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            intent: USCPIntent::FleetPulse,
            priority: USCPPriority::Normal,
            correlation_id: Some("test-flow-001".to_string()),
            ttl: 64,
        },
        body: USCPBody {
            payload: serde_json::json!({
                "type": "fleet_pulse",
                "agent_count": 5,
                "avg_gamma": 0.85,
                "avg_eta": 0.12,
                "avg_thermal": 1.0,
                "avg_creative": 0.45
            }),
            extensions: Some(USCPExtensions {
                gamma_eta: Some(GammaEta { gamma: 0.85, eta: 0.12 }),
                thermal: Some(Thermal { T: 1.0, delta: 0.0, p_melt: 0.0 }),
                creative: Some(Creative { delta: 0.45, zone: CreativeZone::Creative }),
                melt: None,
                molt: None,
                uncertainty: Some(Uncertainty { value: 0.05, source: "test".to_string() }),
                anomaly: None,
            }),
        },
        signature: None,
    };
    
    // Step 2: CNS → DSH translation
    let dsh_event = {
        let t = translator.lock().await;
        t.uscp_to_dsh(&cns_packet)?
    };
    
    // Verify DSH event structure
    assert_eq!(dsh_event.event_type, "fleet.telemetry.pulse");
    assert!(dsh_event.metadata.uscp_extensions.is_some());
    assert_eq!(dsh_event.metadata.priority, DSHPriority::Normal);
    assert_eq!(dsh_event.metadata.correlation_id, Some("test-flow-001".to_string()));
    
    // Step 3: Simulate plugin processing (echo pattern - same event_type)
    let plugin_response = simulate_plugin(&dsh_event).await?;
    
    // Step 4: Plugin → DSH response (same event_type for roundtrip)
    assert_eq!(plugin_response.event_type, "fleet.telemetry.pulse");
    assert!(plugin_response.metadata.correlation_id.is_some());
    
    // Step 5: DSH → CNS translation
    let response_packet = {
        let t = translator.lock().await;
        t.dsh_to_usc(&plugin_response)?
    };
    
    // Verify response packet
    assert_eq!(response_packet.header.intent, USCPIntent::FleetPulse);
    assert_eq!(response_packet.header.correlation_id, Some("test-flow-001".to_string()));
    assert!(response_packet.body.extensions.is_some());
    
    // Step 6: Verify all 7 extensions preserved
    let extensions = response_packet.body.extensions.unwrap();
    assert!(extensions.gamma_eta.is_some());
    assert!(extensions.thermal.is_some());
    assert!(extensions.creative.is_some());
    assert!(extensions.uncertainty.is_some());
    
    println!("✅ Full CNS→DSH→Plugin→DSH→CNS flow validated");
    Ok(())
}

async fn simulate_plugin(event: &DSHEvent) -> Result<DSHEvent> {
    Ok(DSHEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: event.event_type.clone(),  // Echo pattern for roundtrip
        source_plugin: "test-plugin".to_string(),
        target_plugin: Some(event.source_plugin.clone()),
        timestamp: chrono::Utc::now().to_rfc3339(),
        payload: event.payload.clone(),
        metadata: DSHMetadata {
            priority: event.metadata.priority.clone(),
            correlation_id: event.metadata.correlation_id.clone(),
            uscp_extensions: event.metadata.uscp_extensions.clone(),
            tags: vec!["test".to_string(), "processed".to_string()],
            ttl: 64,
        },
    })
}

#[tokio::test]
async fn test_plugin_registry_discovery_flow() -> Result<()> {
    let translator = Arc::new(Mutex::new(CNSDSHTranslator::new("test-registry".to_string())));
    
    // Registry registration event (maps to AgentRegister)
    let register_event = DSHEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "fleet.lifecycle.register".to_string(),  // Must match reverse_intent_map
        source_plugin: "test-plugin".to_string(),
        target_plugin: Some("registry".to_string()),
        timestamp: chrono::Utc::now().to_rfc3339(),
        payload: serde_json::json!({ "plugin_id": "test-plugin" }),
        metadata: DSHMetadata {
            priority: DSHPriority::Normal,
            correlation_id: Some("registry-test-001".to_string()),
            uscp_extensions: None,
            tags: vec!["registry".to_string(), "register".to_string()],
            ttl: 64,
        },
    };
    
    // Registry response (same event_type for roundtrip)
    let registered_response = DSHEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "fleet.lifecycle.register".to_string(),
        source_plugin: "registry".to_string(),
        target_plugin: Some(register_event.source_plugin.clone()),
        timestamp: chrono::Utc::now().to_rfc3339(),
        payload: serde_json::json!({ "plugin_id": "test-plugin" }),
        metadata: DSHMetadata {
            priority: DSHPriority::Normal,
            correlation_id: register_event.metadata.correlation_id.clone(),
            uscp_extensions: None,
            tags: vec!["registry".to_string(), "registered".to_string()],
            ttl: 64,
        },
    };
    
    // Verify roundtrip
    let cns_packet = {
        let t = translator.lock().await;
        t.dsh_to_usc(&registered_response)?
    };
    
    assert_eq!(cns_packet.header.intent, USCPIntent::AgentRegister);
    assert_eq!(cns_packet.header.correlation_id, Some("registry-test-001".to_string()));
    
    println!("✅ Plugin registry discovery flow validated");
    Ok(())
}

#[tokio::test]
async fn test_fleet_conductor_workflow_flow() -> Result<()> {
    let translator = Arc::new(Mutex::new(CNSDSHTranslator::new("test-conductor".to_string())));
    
    // Workflow creation via CNS
    let cns_packet = USCPv3Packet {
        header: USCPHeader {
            version: "3.0".to_string(),
            packet_id: Uuid::new_v4().to_string(),
            source_id: "test-commander".to_string(),
            destination_id: "fleet-conductor".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            intent: USCPIntent::ExecutePlan,
            priority: USCPPriority::High,
            correlation_id: Some("workflow-001".to_string()),
            ttl: 64,
        },
        body: USCPBody {
            payload: serde_json::json!({
                "workflow_id": "wf-001",
                "name": "Test Pipeline",
                "steps": [
                    { "step_id": "s1", "capability": "data.ingest" },
                    { "step_id": "s2", "capability": "data.transform" }
                ]
            }),
            extensions: None,
        },
        signature: None,
    };
    
    // CNS → DSH
    let dsh_event = {
        let t = translator.lock().await;
        t.uscp_to_dsh(&cns_packet)?
    };
    
    assert_eq!(dsh_event.event_type, "fleet.command.execute");
    assert_eq!(dsh_event.metadata.priority, DSHPriority::High);
    
    // Fleet Conductor response (same event_type)
    let workflow_response = DSHEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "fleet.command.execute".to_string(),
        source_plugin: "fleet-conductor".to_string(),
        target_plugin: Some(dsh_event.source_plugin.clone()),
        timestamp: chrono::Utc::now().to_rfc3339(),
        payload: serde_json::json!({ "workflow_id": "wf-001", "status": "created" }),
        metadata: DSHMetadata {
            priority: DSHPriority::High,
            correlation_id: dsh_event.metadata.correlation_id.clone(),
            uscp_extensions: None,
            tags: vec!["workflow".to_string(), "created".to_string()],
            ttl: 64,
        },
    };
    
    // DSH → CNS
    let response_packet = {
        let t = translator.lock().await;
        t.dsh_to_usc(&workflow_response)?
    };
    
    assert_eq!(response_packet.header.intent, USCPIntent::ExecutePlan);
    assert_eq!(response_packet.header.correlation_id, Some("workflow-001".to_string()));
    
    println!("✅ Fleet Conductor workflow flow validated");
    Ok(())
}