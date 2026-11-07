from typing import Dict, Any, List, Optional
import logging

# Setup minimal logging
logging.basicConfig(level=logging.INFO, format='%(levelname)s: %(message)s')
logger = logging.getLogger("SocialTelemetryBridgeV2")

class SocialTelemetryBridgeV2:
    """
    A schema-resilient bridge for mapping social signals to telemetry parameters.
    Implements defensive updates and fuzzy mapping with Rust-like safety philosophy.
    """

    def __init__(self, parameter_schema: Dict[str, Any]):
        """
        Initialize with a valid schema of parameters the engine accepts.
        :param parameter_schema: A dict where keys are valid telemetry parameter names.
        """
        # Defensive copy of the schema to ensure internal state integrity
        self._schema = dict(parameter_schema)
        # Fuzzy mapping: Social Token -> Parameter Name
        self._fuzzy_map: Dict[str, str] = {}
        logger.info(f"Initialized Bridge with {len(self._schema)} valid parameters.")

    def set_fuzzy_mapping(self, mapping: Dict[str, str]):
        """
        Establish the links between social tokens and engine parameters.
        Example: {"vibe_check": "sociability_index", "noise_level": "entropy"}
        """
        for token, param in mapping.items():
            if param in self._schema:
                self._fuzzy_map[token] = param
            else:
                logger.warning(f"Fuzzy mapping ignored: Target parameter '{param}' not in schema.")
        logger.info(f"Fuzzy mapping configured with {len(self._fuzzy_map)} active links.")

    def process_social_signal(self, signal_payload: Dict[str, Any]) -> Dict[str, Any]:
        """
        Processes an incoming signal payload and returns an update dict for the engine.
        Ensures that unknown tokens or missing parameters do not crash the bridge.
        """
        updates = {}

        # 1. Iterate through signal payload (Defensive approach)
        for token, value in signal_payload.items():
            # 2. Attempt to resolve token through fuzzy mapping
            # This is the "Fuzzy Mapping Logic"
            target_param = self._fuzzy_map.get(token)

            if target_param:
                # 3. Verify target parameter exists in schema (Defensive Update Pattern)
                if target_param in self._schema:
                    # We use the schema to ensure the value type matches if needed
                    # For this simulation, we just ensure the key is valid.
                    updates[target_param] = value
                    logger.debug(f"Mapped '{token}' -> '{target_param}': {value}")
                else:
                    # This case is mostly handled by set_fuzzy_mapping logic, but added for depth
                    logger.error(f"Schema mismatch: '{target_param}' resolved from '{token}' but missing in schema.")
            else:
                # Handle unknown tokens gracefully (part of the survival requirement)
                logger.debug(f"Skipping unknown social token: '{token}'")
        
        return updates

def run_verification_test():
    print("--- Starting SocialTelemetryBridgeV2 Verification ---")
    
    # Define a valid engine schema
    engine_schema = {
        "sociability_index": 0.0,
        "entropy": 0.0,
        "coherence": 1.0,
        "aggression": 0.0
    }

    bridge = SocialTelemetryBridgeV2(engine_schema)

    # Configure fuzzy mapping
    bridge.set_fuzzy_mapping({
        "vibe_check": "sociability_index",
        "chaos_factor": "entropy",
        "logic_stream": "coherence"
    })

    # Test Case 1: Standard valid payload
    valid_payload = {
        "vibe_check": 0.85,
        "chaos_factor": 0.2,
        "logic_stream": 0.95
    }
    print(f"\nTest 1 (Valid Payload): {valid_payload}")
    updates_1 = bridge.process_social_signal(valid_payload)
    print(f"Resulting Updates: {updates_1}")
    assert updates_1 == {"sociability_index": 0.85, "entropy": 0.2, "coherence": 0.95}

    # Test Case 2: Payload with "Unknown" social tokens (The survival test)
    unknown_payload = {
        "vibe_check": 0.7,
        "mystery_token_xyz": 999,         # Should be ignored
        "random_noise_v3": "unexpected",  # Should be ignored
        "chaos_factor": 0.4
    }
    print(f"\nTest 2 (Payload with Unknown Tokens): {unknown_payload}")
    updates_2 = bridge.process_social_signal(unknown_payload)
    print(f"Resulting Updates: {updates_2}")
    assert updates_2 == {"sociability_index": 0.7, "entropy": 0.4}
    assert "mystery_token_xyz" not in updates_2

    # Test Case 3: Payload mapping to non-existent schema parameter (via mapping)
    # We'll simulate this by forcing a bad mapping if allowed, 
    # or just verifying the current implementation prevents it.
    bridge.set_fuzzy_mapping({"glitch": "non_existent_param"}) 
    glitch_payload = {"glitch": 1.0}
    print(f"\nTest 3 (Mapping to non-existent schema param): {glitch_payload}")
    updates_3 = bridge.process_social_signal(glitch_payload)
    print(f"Resulting Updates: {updates_3}")
    assert updates_3 == {}

    print("\n--- ALL TESTS PASSED SUCCESSFULLY ---")

if __name__ == "__main__":
    run_verification_test()
