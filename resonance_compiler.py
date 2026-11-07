import json
import re
import sys
from pathlib import Path

class ResonanceCompiler:
    def __init__(self, mapping_path: str):
        with open(mapping_path, 'r') as f:
            self.mapping = json.load(f)

    def parse_tap_file(self, tap_path: str) -> dict:
        """Parses a .tap file for resonance metadata."""
        with open(tap_path, 'r') as f:
            content = f.read()

        # Look for the [resonance: ...] pattern
        metadata_match = re.search(r'\[resonance:\s*(.*?)\]', content)
        if not metadata_match:
            raise ValueError(f"No resonance metadata found in {tap_path}")

        metadata_str = metadata_match.group(1)
        # Parse key=value pairs
        metadata = {}
        for part in metadata_str.split(','):
            if '=' in part:
                k, v = part.split('=')
                metadata[k.strip().lower()] = v.strip().lower()
        
        return metadata

    def compile(self, tap_path: str, output_path: str):
        """Maps resonance metadata to kinetic parameters."""
        resonance_data = self.parse_tap_file(tap_path)
        kinetic_payload = {}

        for key, value in resonance_data.items():
            if key in self.mapping:
                # If the key exists in our mapping (e.g., 'tension'), 
                # map it to the corresponding sub-values using the intensity level provided.
                if value in self.mapping[key]:
                    kinetic_payload.update(self.mapping[key][value])
                else:
                    print(f"Warning: Intensity level '{value}' not found for '{key}' in mapping.")
            else:
                print(f"Warning: Resonance parameter '{key}' not defined in mapping.")

        # Add metadata from the tap file to the payload
        kinetic_payload['source_tap'] = Path(tap_path).name
        
        with open(output_path, 'w') as f:
            json.dump(kinetic_payload, f, indent=2)
        
        return kinetic_payload

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python resonance_compiler.py <input.tap> <output.json>")
        sys.exit(1)

    input_tap = sys.argv[1]
    output_json = sys.argv[2]
    mapping_file = "social_kinetic_mapping.json"

    compiler = ResonanceCompiler(mapping_file)
    try:
        result = compiler.compile(input_tap, output_json)
        print(f"Compilation successful. Payload: {result}")
    except Exception as e:
        print(f"Compilation failed: {e}")
        sys.exit(1)
