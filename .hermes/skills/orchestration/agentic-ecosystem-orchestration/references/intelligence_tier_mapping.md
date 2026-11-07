# Intelligence Tier Mapping & Onboarding

This reference document defines the intelligence hierarchy and the formal handshake required for new agents to join the SuperInstance ecosystem.

## Intelligence Tier Matrix

| Tier | Intelligence Profile | Typical Providers | Scaling Strategy |
| :--- | :--- | :--- | :--- |
| **Tier 1: Workhorse** | High Intelligence / Balanced Cost | `DeepSeek`, `Groq` | Default for standard agentic loops. |
| **Tier 2: Specialist** | Ultra-High Intelligence | `Anthropic`, `Minimax`, `Z.ai`, `Kimi` | Escalation for complex reasoning. |
| **Tier 3: Utility** | Broad Intelligence / General Purpose | `OpenAI`, `Grok`, `Perplexity` | Fallback for general knowledge. |
| **Tier 4: Edge/Local** | Low Latency / Deterministic | `Ollama`, `Docker-Local` | Real-time monitoring and privacy. |

## Onboarding Handshake (The Persona Setup)

When a new agent is provisioned, it must be initialized with an `agent_profile.json` that follows the Tiered Intelligence model. This allows the orchestrator to perform "Intelligence-Aware Routing."

### Example Profile Template
```json
{
  "version": "1.0",
  "intelligence_profile": {
    "tier_mapping": {
      "reasoning": "Tier 2",
      "monitoring": "Tier 4"
    },
    "provider_preferences": [
      { "tier": "Tier 1", "preferred": "deepseek-chat" },
      { "tier": "Tier 2", "preferred": "claude-3-5-sonnet" }
    ]
  }
}
```
