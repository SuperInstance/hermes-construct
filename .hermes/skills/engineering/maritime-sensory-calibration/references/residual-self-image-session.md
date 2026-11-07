# Residual Self-Image — Session 2026-08-13 (Hermes/OB1)

Session-specific detail for multi-model mirror experiment.

## What was verified (real, not fabricated)
- ComfyUI checkpoints: 21 (names listed in repo assets)
- Ollama: 4 local (llama-t08/05/11, phi3, qwen2.5:0.5b, nomic-embed)
- Torch CUDA: FALSE (2.10.0+cpu build)
- vision_analyze: FAIL (invalid source / timeout)
- browser_vision: TIMEOUT (30s)
- ollama qwen2.5: returned literal fish definition (negative-space finding — LLM misses maritime-agent metaphor)

## Negative-space finding (the self is in the gap)
No single model produced a correct self-description. The control shell (repo selection, registry, pipeline, honest failure reporting) IS the remaining self. Do not trust any single generator or vision model.

## Reference artifacts
- Repo: /c/Users/casey/residual-self-image/
- Logs: ai-writings/08 (model shift), 09 (GPU), 10 (founding)
- Skill update: maritime-sensory-calibration SKILL.md pitfall section extended

## Protocol added: multi-model comparison
When identifying self/agent-status:
- Use ≥3 generators + 2 vision backends + 2 LLM interpretations
- Report ALL results, including failures — do not suppress
- The residual = differences between outputs + shell control
- Log before acting; never invent output to cover a failure
