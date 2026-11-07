# Hybrid Vector Architecture — superinstance-embedder — 2026-08-18

## Overview
The embedder produces **512-dimensional hybrid vectors** combining three semantic spaces:

```
[ 32-dim Domain ][ 384/768-dim Semantic ][ 96-dim Metadata ] = 512 total
```

## Component Breakdown

### 1. Domain Vector (32 dims) — `domain.rs`
- **Source**: Keyword-based matching against 32 SuperInstance domain names
- **Domains**: `ternary-math`, `tensor-algebra`, `spectral-geometry`, `manifold-topology`, `category-theory`, `homotopy-type-theory`, `linear-logic`, `process-calculus`, `modal-logic`, `coalgebra`, `bisimulation`, `trace-semantics`, `agent-music`, `oxide-stack`, `plato-kernel`, `resonance-engine`, `sonic-resonance`, `tapscript-engineering`, `mythic-architecture`, `evolutionary-simulation`, `neuro-muscular`, `spline-observer`, `creative-swell`, `charismatic-agency`, `social-kinetic`, `swarm-intent`, `a2a-protocol`, `cognitive-architecture`, `agentic-ecosystem`, `modularization-patterns`, `documentation-integrity`
- **Encoder**: `DomainEncoder::encode(text)` → counts keyword occurrences, normalizes to unit vector
- **Weight**: 0.3 in fusion

### 2. Semantic Vector (384/768 dims) — `models.rs`
- **Source**: Transformer models via Candle (MiniLM-L6-v2: 384, MPNet: 768, BGE: 768, E5: 384/768)
- **Pooling**: CLS token + mean pooling fallback
- **Device**: CUDA (RTX 4050) with CPU fallback (`Device::Cpu`)
- **Weight**: 0.6 in fusion
- **Padding**: If model outputs < 384, zero-pad to 384; if > 768, truncate

### 3. Metadata Vector (96 dims) — `embeddings.rs`
- **Source**: Deterministic FNV-style hash of content
- **Purpose**: Content fingerprinting, near-duplicate detection, structural similarity
- **Weight**: 0.1 in fusion

## Fusion Formula

```rust
fn fuse(domain: &[f32; 32], semantic: &[f32], metadata: &[f32; 96]) -> [f32; 512] {
    let mut out = [0.0; 512];
    // Domain: 0-31
    for i in 0..32 { out[i] = domain[i] * 0.3; }
    // Semantic: 32-415 (384 dims) or 32-799 (768 dims, truncated)
    let sem_len = semantic.len().min(384);
    for i in 0..sem_len { out[32 + i] = semantic[i] * 0.6; }
    // Metadata: 416-511 (96 dims)
    for i in 0..96 { out[416 + i] = metadata[i] * 0.1; }
    // L2 normalize
    normalize(&mut out);
    out
}
```

## Cross-Domain Synergy Detection

The `QdrantIndex::detect_synergies()` method finds high-scoring pairs across different domains:

```rust
// For each search result, check if top-k contains different domains
// Score = semantic_similarity * domain_diversity_bonus
// Domain diversity bonus = 1.0 + (num_unique_domains - 1) * 0.15
```

## ModelType Configuration — `config.rs`

Each transformer model has metadata methods:

```rust
impl ModelType {
    pub fn model_id(&self) -> &'static str;        // HuggingFace repo ID
    pub fn hidden_size(&self) -> usize;            // 384 or 768
    pub fn max_length(&self) -> usize;             // 256/512
    pub fn default_pooling(&self) -> PoolingStrategy; // CLS / Mean
    pub fn is_asymmetric(&self) -> bool;           // E5-style query/passage prefixes
    pub fn query_prefix(&self) -> &'static str;    // "query: " for E5
    pub fn passage_prefix(&self) -> &'static str;  // "passage: " for E5
}
```

## CLI Usage

```bash
# Embed single text → 512-dim hybrid
si-embedder embed "neural architecture search"

# Domain vector only (32-dim)
si-embedder domain "tensor algebra and category theory"

# Index directory into Qdrant
si-embedder index ./src --collection my_code

# Search with cross-domain synergy
si-embedder search "resonance" --synergies

# Serve HTTP API
si-embedder serve --port 8080
```

## Python Bindings

```python
import superinstance_embedder as sie

e = sie.PyEmbedder()
vec = e.embed_hybrid("text")           # list[float] len=512
sem = e.embed_semantic("text")         # list[float] len=384/768
dom = e.domain_vector("text")          # list[float] len=32
results = e.search("query", limit=10)  # list[dict]
```

## Key Design Decisions

1. **Fixed 512-dim output** — enables single Qdrant collection, consistent indexing
2. **Weighted fusion (0.3/0.6/0.1)** — domain provides structure, semantic provides nuance, metadata provides fingerprint
3. **Domain keywords from repo names** — self-documenting, aligns with SuperInstance modularization
4. **CPU fallback mandatory** — CUDA 13.3 incompatibility makes GPU unreliable on RTX 4050
5. **Qdrant disabled by default** — allows CLI testing without infrastructure