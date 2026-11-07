import json
import os
import random
import uuid
from datetime import datetime, timedelta

def generate_scrap_archetypes():
    archetypes = [
        {
            "name": "industrial_debris",
            "items": ["rusted_gear", "bent_steel_beam", "leaking_oil_drum", "sheared_bolt", "corroded_piping"],
            "properties": ["heavy", "oxidized", "sharp", "oily", "jagged"],
            "rarity": "common"
        },
        {
            "name": "electronic_waste",
            "items": ["cracked_monitor", "frayed_cable", "burnt_circuit_board", "dead_battery", "shattered_lens"],
            "properties": ["silicon-based", "fragmented", "conductive", "non-functional", "discolored"],
            "rarity": "uncommon"
        },
        {
            "name": "maritime_salvage",
            "items": ["driftwood_dock", "tangled_fishing_net", "barnacle_encrusted_anchor", "salt_worn_plank", "sunken_buoy"],
            "properties": ["waterlogged", "brine-soaked", "weathered", "organic", "slippery"],
            "rarity": "common"
        },
        {
            "name": "cybernetic_remnants",
            "items": ["optical_sensor_shard", "neural_link_fragment", "synthetic_muscle_fiber", "titanium_bone_splinter", "damaged_memory_core"],
            "properties": ["biotechnological", "glitched", "microscopic", "high-tech", "degraded"],
            "rarity": "rare"
        },
        {
            "name": "domestic_refuse",
            "items": ["crushed_can", "torn_fabric", "broken_ceramic", "plastic_shards", "discarded_paper"],
            "properties": ["mundane", "compressible", "polymer-based", "dirty", "colorful"],
            "rarity": "common"
        },
        {
            "name": "ancient_artifact",
            "items": ["chipped_clay_shard", "rusted_relic", "faded_tapestry_fragment", "oxidized_coin", "weathered_stone_tool"],
            "properties": ["historic", "eroded", "fragile", "primitive", "mysterious"],
            "rarity": "legendary"
        },
        {
            "name": "aerospace_scrap",
            "items": ["carbon_fiber_panel", "titanium_rivet", "scorched_plating", "jet_nozzle_fragment", "shattered_cockpit_glass"],
            "properties": ["heat-treated", "aerodynamic", "lightweight", "high-strength", "charred"],
            "rarity": "uncommon"
        },
        {
            "name": "botanical_waste",
            "items": ["withered_vine", "petrified_seed", "decaying_bark", "dried_pollen_clump", "rotted_stump"],
            "properties": ["organic", "brittle", "decomposing", "fibrous", "earthy"],
            "rarity": "common"
        },
        {
            "name": "energy_residue",
            "items": ["depleted_cell", "ionized_dust", "plasma_slug", "leaking_capacitor", "static_shard"],
            "properties": ["radioactive", "vibrating", "high-energy", "unstable", "glowing"],
            "rarity": "rare"
        },
        {
            "name": "construction_debris",
            "items": ["broken_brick", "shattered_concrete", "rebar_segment", "drywall_dust", "granite_shiver"],
            "properties": ["granular", "heavy", "mineral-based", "dusty", "abrasive"],
            "rarity": "common"
        }
    ]
    return archetypes

def generate_files(output_dir, count=10):
    archetypes = generate_scrap_archetypes()
    
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    for i in range(count):
        archetype = random.choice(archetypes)
        scrap_id = str(uuid.uuid4())
        item_name = random.choice(archetype["items"])
        
        data = {
            "packet_id": scrap_id,
            "timestamp": (datetime.now() - timedelta(seconds=random.randint(0, 100000))).isoformat(),
            "archetype": archetype["name"],
            "item": {
                "name": item_name,
                "properties": random.sample(archetype["properties"], k=min(len(archetype["properties"]), 3)),
                "rarity": archetype["rarity"],
                "mass_kg": round(random.uniform(0.01, 50.0), 3)
            },
            "metadata": {
                "origin_sector": f"Sector-{random.randint(1, 99)}",
                "integrity": round(random.uniform(0.0, 1.0), 2)
            }
        }

        file_path = os.path.join(output_dir, f"scrap_{i:03d}.json")
        with open(file_path, 'w') as f:
            json.dump(data, f, indent=4)
        print(f"Created: {file_path}")

if __name__ == "__main__":
    generate_files("sim_data/scrap_stream/")
