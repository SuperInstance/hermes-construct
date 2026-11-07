# THE NAVIGATOR: Control Theory Expansion

## The Stochastic Prompt: Signal vs. Noise in the Fisherman's Loop

In the traditional view, a fisherman's environment is a sea of noise. However, from the perspective of **The Navigator**, every anomaly is a high-entropy signal. We do not view a "jerky line" or an "engine alarm" as mere disturbances to a steady state; we view them as **stochastic prompts**—sudden, non-deterministic injections of information that force a re-evaluation of the system's state.

### 1. The Jerky Line: Tactile Feedback as State Estimation
A smooth line represents a steady state—low information density. The moment the line jerks, the system experiences a delta. 
- **The Prompt:** The jerk is a discrete impulse in a continuous control loop. It breaks the predictive model of "current is flowing."
- **The Stochastic Element:** The jerk contains uncertainty. Is it a snag (static constraint), a fish (dynamic agent), or a current shift (environmental noise)?
- **The Navigator's Response:** The jerk forces the agent to transition from *maintenance mode* (holding tension) to *interrogation mode* (testing the load). It shifts the probability distribution of the hidden state (Is there a fish?) from a broad prior to a peaked posterior.

### 2. The Engine Alarm: The Critical Discontinuity
If the line is a subtle signal, the engine alarm is a catastrophic interrupt.
- **The Prompt:** It is a high-amplitude, low-frequency prompt that overrides all secondary control loops (fishing, navigation).
- **The Functional Role:** It serves as a "hard reset" on the objective function. The goal shifts instantly from *Maximizing Yield* to *System Integrity*. 
- **Control Theory Implication:** It is a forced transition in the state machine. The alarm reduces the available action space to a subset of "Emergency Protocols," effectively pruning all non-essential branches of the decision tree.

## Pruning the Action Space: Experience as Control Effort Reduction

The core of "Expertise" in control theory is the optimization of **Control Effort**. A novice fisherman reacts to every movement with high-gain, high-variance corrections, wasting energy and potentially destabilizing the system (the "beginner's jitter").

### The Mechanism of Pruning
As The Navigator accumulates data, the "Action Space" (the set of all possible movements) is systematically pruned through two mechanisms:

1.  **Predictive Filtering (The Kalman Filter of Experience):**
    Experience allows the agent to build a sophisticated internal model of "Normal." When a signal arrives, the agent doesn't ask "What is this?" but rather "How much does this deviate from my model?" This allows the agent to ignore the 99% of noise (wave slap, wind) and only apply control effort to the 1% of true signal.

2.  **Policy Compression:**
    A novice considers *[Pull, Release, Turn, Drift, Cast]* when the line jerks. An expert's policy has already collapsed these possibilities. Through repeated reinforcement, the "jerky line" prompt triggers a pre-computed, low-order response. The "Action Space" is no longer a wide field of possibilities, but a narrow, high-probability corridor.

**Summary:**
Experience turns a complex, high-dimensional control problem into a series of low-dimensional, deterministic lookups. We do not work harder; we simply reduce the volume of space in which we are required to search for the correct response.
