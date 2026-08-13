import numpy as np
from scipy.stats import entropy

class ResonanceKernel:
    """
    A minimal Resonance Engine capable of performing a 'Pruned Projection'.
    """

    def apply_mask(self, signal: np.ndarray, mask: np.ndarray) -> np.ndarray:
        """
        Applies a mask (representing experience/filtering) to a signal.
        The mask should be the same shape as the signal.
        """
        if signal.shape != mask.shape:
            raise ValueError(f"Shape mismatch: Signal {signal.shape} != Mask {mask.shape}")
        
        # The projection: element-wise multiplication represents the 'pruning' 
        # of information based on the mask.
        return signal * mask

    def calculate_entropy(self, vector: np.ndarray) -> float:
        """
        Calculates the Shannon entropy of the vector (normalized probabilities).
        """
        # Ensure the vector is treated as a probability distribution
        # We take the absolute values and normalize to sum to 1
        abs_vector = np.abs(vector)
        if np.sum(abs_vector) == 0:
            return 0.0
        probs = abs_vector / np.sum(abs_vector)
        return entropy(probs)

    def process(self, signal: np.ndarray, mask: np.ndarray) -> np.ndarray:
        """
        Performs the full resonance calculation.
        """
        resonance = self.apply_mask(signal, mask)
        return resonance

def run_verification():
    kernel = ResonanceKernel()

    # 1. Define a Signal (high entropy, random-ish)
    # We use a larger dimension to make entropy differences more meaningful
    dim = 100
    np.random.seed(42)
    signal = np.random.uniform(0, 1, dim)
    
    initial_entropy = kernel.calculate_entropy(signal)
    print(f"Initial Signal Entropy: {initial_entropy:.4f}")

    # 2. Define a Mask (the 'experience')
    # A mask that prunes certain frequencies or dimensions.
    # For simplicity, we'll use a mask that zeroes out half the signal 
    # and amplifies another part, effectively 'focusing' the signal.
    mask = np.zeros(dim)
    mask[25:75] = 1.0  # Focus on the middle 50%
    
    # 3. Apply Pruned Projection
    resonance = kernel.process(signal, mask)
    
    # 4. Verify Results
    resonance_entropy = kernel.calculate_entropy(resonance)
    print(f"Resonance Entropy:     {resonance_entropy:.4f}")

    # Verification: Output should have lower entropy than the input
    assert resonance_entropy < initial_entropy, \
        f"Verification Failed: Resonance entropy ({resonance_entropy}) should be lower than initial ({initial_entropy})"
    
    print("Verification SUCCESS: Resonance achieved (Entropy reduced).")
    return True

if __name__ == "__main__":
    try:
        run_verification()
    except Exception as e:
        print(f"Verification FAILED: {e}")
        exit(1)
