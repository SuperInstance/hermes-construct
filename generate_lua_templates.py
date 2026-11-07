import json
import os
import glob

def generate_lua_template(data):
    """
    Generates a Lua template based on the Scrap JSON structure.
    """
    name = data.get("item", {}).get("name", "unknown_item")
    archetype = data.get("archetype", "unknown_archetype")
    properties = data.get("item", {}).get("properties", [])
    rarity = data.get("item", {}).get("rarity", "common")
    mass = data.get("item", {}).get("mass_kg", 0.0)
    origin = data.get("metadata", {}).get("origin_sector", "Unknown")
    integrity = data.get("metadata", {}).get("integrity", 0.0)

    lua_template = f"""-- Manifestation for {name}
local Manifestation = {{
    Archetype = "{archetype}",
    ItemName = "{name}",
    Properties = {{
        {", ".join([f'"{p}"' for p in properties]) if properties else ""}
    }},
    Rarity = "{rarity}",
    MassKg = {mass},
    Metadata = {{
        OriginSector = "{origin}",
        Integrity = {integrity},
    }}
}}

return Manifestation
"""
    return lua_template

def process_scraps(input_dir, output_dir):
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    json_files = glob.glob(os.path.join(input_dir, "scrap_*.json"))
    print(f"Found {len(json_files)} scrap files.")

    for file_path in json_files:
        try:
            with open(file_path, 'r') as f:
                # The files have a leading '1|' which might be custom protocol. 
                # We'll strip it if it's present.
                content = f.read().strip()
                if content.startswith("1|"):
                    content = content[2:]
                
                data = json.loads(content)
            
            lua_content = generate_lua_template(data)
            base_name = os.path.basename(file_path).replace(".json", ".lua")
            output_path = os.path.join(output_dir, base_name)
            
            with open(output_path, 'w') as f_out:
                f_out.write(lua_content)
            print(f"Generated: {output_path}")
            
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

if __name__ == "__main__":
    INPUT_DIR = "sim_data/scrap_stream"
    OUTPUT_DIR = "sim_data/architect_output"
    process_scraps(INPUT_DIR, OUTPUT_DIR)
