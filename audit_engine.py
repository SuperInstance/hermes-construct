import json
import numpy as np
import time

def simulate_propagation(social_vector, initial_entropy, decay_factor=0.85, steps=10):
    """
    Simulates the propagation of a social vector through a memory system.
    As the vector 'propagates', its norm (influence) decays.
    """
    propagation_log = []
    current_vector = np.array(social_vector)
    
    # Simulate "Decay Rate"
    # In a real system, this would be dot-product similarity with memory retrieval 
    # and the resulting weights on the decision manifold.
    
    for step in range(steps):
        # Simulate "Semantic Drift" - the vector becomes less 'pure' or more 'diffuse'
        noise = np.random.normal(0, 0.01, current_vector.shape)
        current_vector = current_vector * decay_factor + noise
        
        # Normalize to keep it a unit vector, but the 'strength' of influence 
        # is measured by how aligned it is with the original "truth" or its persistence.
        # Here we'll use a 'manifestation_strength' metric.
        manifestation_strength = np.linalg.norm(current_vector) / (step + 1) 
        
        # For this simulation, we'll track "influence" as a scalar that decays.
        # We simulate the effect of the vector on a 'Decision Manifold' 
        # by calculating its projected weight.
        influence_on_manifold = manifestation_strength * (initial_entropy / 5.0)
        
        propagation_log.append({
            "step": step,
            "timestamp_offset_sec": step * 3600, # simulating time passage in hours
            "influence_magnitude": float(influence_on_manifold),
            "semantic_drift": float(np.linalg.norm(noise))
        })
        
    return propagation_log

def main():
    # Load the actual event
    with open("social_signal_event.json", "r") as f:
        event = json.load(f)
    
    social_vector = event["social_vector"]
    initial_entropy = event["metadata"]["initial_entropy"]
    
    print(f"Starting propagation audit for event: {event['event_type']}")
    print(f"Location: {event['location']}")
    
    # Perform Audit
    results = simulate_propagation(social_vector, initial_entropy)
    
    # Output results to a file for the report
    with open("propagation_results.json", "w") as f:
        json.dump(results, f, indent=4)
    
    print("Audit complete. Results written to propagation_results.json")

if __name__ == "__main__":
    main()
