import json
import random

def generate_chaos_log(filename="social_events.log"):
    # Mapping: "vibe_check" -> "sociability_index", "chaos_factor" -> "entropy", "logic_stream" -> "coherence"
    valid_tokens = ["vibe_check", "chaos_factor", "logic_stream"]
    
    with open(filename, "w") as f:
        # 1. 30 Valid (mapped) entries
        for _ in range(30):
            token = random.choice(valid_tokens)
            entry = {
                token: round(random.uniform(0, 1), 2),
                "timestamp": "2026-08-12T12:00:00Z"
            }
            f.write(json.dumps(entry) + "\n")

        # 2. 40 Invalid (unmapped tokens)
        for _ in range(40):
            entry = {
                f"unmapped_token_{random.randint(100, 999)}": round(random.uniform(0, 1), 2),
                "timestamp": "2026-08-12T12:00:00Z"
            }
            f.write(json.dumps(entry) + "\n")

        # 3. 30 Malformed (missing fields, corrupted JSON)
        for _ in range(30):
            choice = random.randint(0, 2)
            if choice == 0:
                # Missing field (e.g., value) - In this context, a key-value pair with a non-null value
                # but let's just make it a dict with only timestamp
                f.write(json.dumps({"timestamp": "2026-08-12T12:00:00Z"}) + "\n")
            elif choice == 1:
                # Corrupted JSON string
                f.write("CORRUPTED_JSON_DATA_!!!_#$\n")
            else:
                # Empty/Garbage
                f.write("{}\n")
    
    print(f"Successfully generated {filename} with 100 entries.")

if __name__ == "__main__":
    generate_chaos_log()
