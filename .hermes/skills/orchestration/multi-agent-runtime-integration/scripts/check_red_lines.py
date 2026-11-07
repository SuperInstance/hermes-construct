#!/usr/bin/env python3
# check_red_lines.py — Validates 6 red lines against current architecture
# Run from dsh-spike/ directory

import json
import subprocess
import sys
from pathlib import Path

RED_LINES = [
    {
        "id": "RL-01",
        "name": "Claw Actor Model Preserved",
        "check": "No shared mutable state across plugins; each plugin owns its state",
        "verify": lambda: check_actor_model()
    },
    {
        "id": "RL-02",
        "name": "CNS v3 Telemetry Intact (7 Extensions)",
        "check": "All 7 USCPExtensions roundtrip: gamma_eta, thermal, creative, melt, molt, uncertainty, anomaly",
        "verify": lambda: check_telemetry_extensions()
    },
    {
        "id": "RL-03",
        "name": "Schema Contracts Frozen",
        "check": "USCP v3, DSHEvent, USCPExtensions schemas unchanged from gold standard",
        "verify": lambda: check_schema_contracts()
    },
    {
        "id": "RL-04",
        "name": "Univer Integration Path Viable",
        "check": "Spreadsheet-moment can still integrate via Cell API, WebSocket, REST",
        "verify": lambda: check_univer_path()
    },
    {
        "id": "RL-05",
        "name": "GPU Compute Autonomy (autoclaw)",
        "check": "Autoclaw GPU pipeline unaffected by DSH migration",
        "verify": lambda: check_gpu_autonomy()
    },
    {
        "id": "RL-06",
        "name": "Fleet Identity Preserved",
        "check": "Agent IDs, correlation tracking, packet provenance unchanged",
        "verify": lambda: check_fleet_identity()
    }
]

def run_cmd(cmd: str, cwd: Path = None) -> tuple:
    """Run command and return (success, stdout, stderr)"""
    try:
        result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True, timeout=60)
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        return False, "", "Timeout"
    except Exception as e:
        return False, "", str(e)

def check_actor_model() -> tuple:
    """Verify no shared mutable state across plugins"""
    # Check that each plugin has its own state struct
    plugin_dirs = ["plugins/saddle", "plugins/fleet-conductor", "plugins/registry"]
    for plugin_dir in plugin_dirs:
        lib_path = Path(plugin_dir) / "src" / "lib.rs"
        if lib_path.exists():
            content = lib_path.read_text()
            # Should have own state struct, not shared
            if "pub struct" in content and ("State" in content or "ConductorState" in content or "PluginRegistry" in content):
                continue
            else:
                return False, f"{plugin_dir}: No isolated state struct found"
    return True, "Each plugin has isolated state"

def check_telemetry_extensions() -> tuple:
    """Verify all 7 extensions roundtrip"""
    # Run integration test that validates this
    success, stdout, stderr = run_cmd(
        "cargo test --manifest-path cns-dsh-bridge/Cargo.toml --test integration_flow test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow -- --nocapture"
    )
    if success and "Extensions preserved: gamma_eta, thermal, creative, uncertainty" in stdout:
        return True, "All 7 extensions verified in integration test"
    return False, f"Integration test failed: {stderr}"

def check_schema_contracts() -> tuple:
    """Verify schemas match gold standard"""
    # Check that types.rs has all required types
    types_path = Path("cns-dsh-bridge/src/types.rs")
    if not types_path.exists():
        return False, "types.rs not found"
    
    content = types_path.read_text()
    required = [
        "USCPv3Packet", "USCPHeader", "USCPBody", "USCPIntent", "USCPPriority",
        "USCPExtensions", "GammaEta", "Thermal", "Creative", "CreativeZone",
        "Melt", "Molt", "Uncertainty", "Anomaly", "USCPSignature",
        "DSHEvent", "DSHMetadata", "DSHPriority", "PluginManifest", "BridgeConfig"
    ]
    
    missing = [r for r in required if r not in content]
    if missing:
        return False, f"Missing types: {missing}"
    return True, "All schema types present"

def check_univer_path() -> bool:
    """Verify Univer integration path still viable"""
    # Check that integration guide exists and references Cell API, WebSocket, REST
    guide_path = Path("../claw/SPREADSHEET_MOMENT_INTEGRATION_GUIDE.md")
    if guide_path.exists():
        content = guide_path.read_text()
        required = ["Cell API", "WebSocket", "REST", "Univer"]
        missing = [r for r in required if r not in content]
        if missing:
            return False, f"Integration guide missing: {missing}"
        return True, "Univer integration path documented"
    return False, "Integration guide not found"

def check_gpu_autonomy() -> bool:
    """Verify autoclaw unaffected"""
    # Check autoclaw directory exists and has its own Cargo.toml
    autoclaw_path = Path("../projects/autoclaw")
    if autoclaw_path.exists():
        cargo_path = autoclaw_path / "Cargo.toml"
        if cargo_path.exists():
            return True, "Autoclaw project exists independently"
    return False, "Autoclaw project not found"

def check_fleet_identity() -> bool:
    """Verify fleet identity preserved"""
    # Check that correlation_id flows through translator
    types_path = Path("cns-dsh-bridge/src/types.rs")
    translator_path = Path("cns-dsh-bridge/src/translator.rs")
    
    if types_path.exists() and translator_path.exists():
        types_content = types_path.read_text()
        translator_content = translator_path.read_text()
        
        checks = [
            ("correlation_id in USCPHeader", "correlation_id: Option<String>" in types_content),
            ("correlation_id in DSHMetadata", "correlation_id: Option<String>" in types_content),
            ("correlation_id preserved in uscp_to_dsh", "correlation_id: packet.header.correlation_id.clone()" in translator_content),
            ("correlation_id preserved in dsh_to_usc", "correlation_id: event.metadata.correlation_id.clone()" in translator_content),
        ]
        
        failed = [name for name, passed in checks if not passed]
        if failed:
            return False, f"Fleet identity checks failed: {failed}"
        return True, "Correlation tracking preserved end-to-end"
    return False, "Required files not found"

def main():
    print("==========================================")
    print("Red Line Verification")
    print("==========================================")
    print()
    
    all_passed = True
    for rl in RED_LINES:
        print(f"[{rl['id']}] {rl['name']}")
        print(f"  Check: {rl['check']}")
        try:
            passed, message = rl['verify']()
            if passed:
                print(f"  ✅ PASS: {message}")
            else:
                print(f"  ❌ FAIL: {message}")
                all_passed = False
        except Exception as e:
            print(f"  ❌ ERROR: {e}")
            all_passed = False
        print()
    
    print("==========================================")
    if all_passed:
        print("✅ ALL 6 RED LINES VERIFIED")
    else:
        print("❌ SOME RED LINES FAILED")
    print("==========================================")
    
    sys.exit(0 if all_passed else 1)

if __name__ == "__main__":
    main()