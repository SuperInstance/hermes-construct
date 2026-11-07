import json
import math
import random
import numpy as np

def generate_social_tokens(count=50):
    """Generates high-entropy social tokens (tension, joy, conflict)."""
    themes = [
        "betrayal", "euphoria", "scandal", "whisper", "clash", 
        "laughter", "stolen_glance", "resentment", "triumph", "mockery",
        "revelation", "suspicion", "ecstasy", "envy", "conspiracy",
        "jubilation", "hostility", "intimacy", "shame", "revolt"
    ]
    tokens = []
    for _ in range(count):
        theme = random.choice(themes)
        entropy_modifier = hex(random.randint(0, 0xFFFF))[2:]
        tokens.append(f"{theme}_{entropy_modifier}")
    return tokens

def calculate_entropy(tokens):
    """Calculates the Shannon entropy of the token list."""
    if not tokens:
        return 0
    counts = {}
    for t in tokens:
        counts[t] = counts.get(t, 0) + 1
    
    total = len(tokens)
    entropy = 0
    for count in counts.values():
        p = count / total
        entropy -= p * math.log2(p)
    return entropy

def generate_social_vector(tokens, dimensions=100):
    """
    Converts tokens into a 100-dimension vector.
    Uses a deterministic hash-based approach for semantic frequency simulation.
    """
    vector = np.zeros(dimensions)
    for token in tokens:
        # Create a pseudo-semantic hash for the token to distribute across dimensions
        hash_val = hash(token)
        for i in range(dimensions):
            # Use a combination of token hash and dimension index to simulate semantic weight
            weight = (hash(f"{token}_{i}") % 1000) / 1000.0
            vector[i] += weight
            
    # Normalize the vector
    if np.linalg.norm(vector) > 0:
        vector = vector / np.linalg.norm(vector)
    
    return vector.tolist()

def main():
    # 1. Generate Tokens
    tokens = generate_social_tokens(50)
    
    # 2. Calculate Entropy
    entropy = calculate_entropy(tokens)
    
    # 3. Generate Vector
    vector = generate_social_vector(tokens, 100)
    
    # 4. Prepare Payload
    payload = {
        "event_type": "Gossip Burst",
        "location": "The Tap",
        "metadata": {
            "token_count": len(tokens),
            "initial_entropy": round(entropy, 4),
            "vector_dimensions": len(vector)
        },
        "social_vector": vector,
        "tokens_sample": tokens[:5]  # Include a sample for verification
    }
    
    # 5. Write to File
    output_path = "social_signal_event.json"
    with open(output_path, "w") as f:
        json.dump(payload, f, indent=4)
    
    print(f"Successfully produced {output_path}")
    print(f"Entropy: {entropy}")

if __name__ == "__main__":
    main()
