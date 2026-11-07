import asyncio
import random
import math
import time
from dataclasses import dataclass, field
from typing import List, Dict

@dataclass
class AgentMetrics:
    task_completions: int = 0
    total_attempts: int = 0
    entropy_history: List[float] = field(default_factory=list)
    last_action_timestamp: float = field(default_factory=time.time)

class SocialStressTestKernel:
    def __init__(self, agent_count: int = 5, perturbation_chance: float = 0.2):
        self.agent_count = agent_count
        self.perturbation_chance = perturbation_chance
        self.agents: List[AgentMetrics] = [AgentMetrics() for _ in range(agent_count)]
        self.global_step = 0

    def calculate_entropy(self, distribution: List[float]) -> float:
        """Calculate Shannon entropy of a distribution."""
        if not distribution:
            return 0.0
        # Ensure sum is 1 for entropy calculation
        total = sum(distribution)
        if total == 0:
            return 0.0
        probs = [p / total for p in distribution]
        return -sum(p * math.log2(p) for p in probs if p > 0)

    async def run_simulation(self, steps: int = 50):
        print(f"🚀 Starting Social Stress-Test Kernel...")
        print(f"Config: {self.agent_count} agents, Perturbation Chance: {self.perturbation_chance}\n")
        
        for step in range(steps):
            self.global_step = step
            
            # 1. Determine if social perturbation occurs this step
            perturbation_intensity = 0.0
            is_perturbed = random.random() < self.perturbation_chance
            if is_perturbed:
                perturbation_intensity = random.uniform(0.5, 2.0)
                # Simulate "token burst" by injecting noise into the system
                print(f"⚠️  [Step {step}] SOCIAL PERTURBATION DETECTED (Intensity: {perturbation_intensity:.2f})")

            # 2. Agent Loop
            for i, agent in enumerate(self.agents):
                agent.total_attempts += 1
                
                # Base success probability
                base_success_prob = 0.8
                
                # Impact of perturbation on success rate
                # High intensity reduces probability of task completion
                effective_prob = max(0.0, base_success_prob - (perturbation_intensity * 0.3))
                
                # Manifold instability (entropy) simulation
                # Noise increases the randomness in agent "state transitions"
                state_noise = 0.1 + (perturbation_intensity * 0.5)
                # Entropy calculation: we model agent state as a probability distribution
                # In a perturbed state, workers are less predictable.
                entropy = self.calculate_entropy([1.0 - state_noise, state_noise])
                agent.entropy_history.append(entropy)

                # Attempt task
                if random.random() < effective_prob:
                    agent.task_completions += 1
                
                # Simulate processing time
                await asyncio.sleep(0.01)

            # 3. Periodic Reporting
            if step % 10 == 0 or is_perturbed:
                self.report_metrics(step)

        self.final_report()

    def report_metrics(self, step: int):
        avg_completion_rate = sum(a.task_completions / a.total_attempts for a in self.agents) / self.agent_count
        avg_entropy = sum(a.entropy_history[-1] for a in self.agents) / self.agent_count
        print(f"[Step {step:02d}] Avg Completion Rate: {avg_completion_rate:.2%}, Avg Manifold Entropy: {avg_entropy:.4f}")

    def final_report(self):
        print("\n" + "="*40)
        print("📊 FINAL STRESS-TEST REPORT")
        print("="*40)
        
        total_completions = sum(a.task_completions for a in self.agents)
        total_attempts = sum(a.total_attempts for a in self.agents)
        overall_rate = total_completions / total_attempts
        
        final_entropies = [a.entropy_history[-1] for a in self.agents]
        avg_final_entropy = sum(final_entropies) / len(final_entropies)

        print(f"Total Task Success Rate: {overall_rate:.2%}")
        print(f"Final Manifold Stability (Entropy): {avg_final_entropy:.4f}")
        print("-" * 40)
        
        for i, agent in enumerate(self.agents):
            rate = agent.task_completions / agent.total_attempts
            ent = agent.entropy_history[-1]
            print(f"Agent {i}: Success Rate: {rate:.2%}, Entropy: {ent:.4f}")
        print("="*40)

if __name__ == "__main__":
    kernel = SocialStressTestKernel(agent_count=5, perturbation_chance=0.25)
    asyncio.run(kernel.run_simulation(steps=60))
