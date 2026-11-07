"""
SuperInstance Embedder E2E Test with Thematic Corpus

Testing with content related to:
- Residual Self-Image (Hermes identity)
- Plato's Shell / OpenRoom
- Sound of the Abyss
- SuperInstance core concepts
"""

import os
import sys

try:
    import superinstance_embedder as sie
    print("✅ superinstance_embedder imported")
except ImportError as e:
    print(f"❌ Import failed: {e}")
    sys.exit(1)

from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct

print("\n" + "="*60)
print("SuperInstance Embedder — Thematic E2E Test")
print("="*60)

# 1. Test basic embedding
print("\n--- 1. Basic Embedding Test ---")

config = sie.PyEmbedderConfig(
    model_enabled=True,
    model_type="mini-lm",
    use_cuda=False,
    qdrant_enabled=False,
)
embedder = sie.PyEmbedder(config)
print("✅ Embedder initialized (CPU mode)")

# Test with our recent themes
test_texts = [
    "Hermes OB1 cybernetic hermit crab captain one organic eye one glowing blue sensor eye",
    "Plato's Shell the cave environment engraved in the habitat boundary",
    "Sound of the Abyss dark ambient underwater music bioluminescent sonar pings",
    "Residual self image what persists when all forms are subtracted",
    "The watcher always watching through the porthole processing the echoes",
    "Hermit crab adopts shells doesn't grow them the agent is the selection process",
]

print("\nTesting embedding with recent themes:")
for text in test_texts:
    vec = embedder.embed(text)
    domain = embedder.domain_vector(text)
    non_zero_domain = sum(1 for v in domain if abs(v) > 1e-6)
    print(f"   Text: '{text[:50]}...'")
    print(f"      Vector: {len(vec)}-dim | Domain signals: {non_zero_domain}/32")

# 2. Connect to Qdrant
print("\n--- 2. Qdrant Integration ---")
client = QdrantClient(url="http://localhost:6333", prefer_grpc=False, check_compatibility=False)
print("✅ Connected to Qdrant (compatibility check disabled)")

# 3. Create thematic collection
collection_name = "hermes-thematic"
print(f"\n--- 3. Creating collection: {collection_name} ---")

try:
    client.delete_collection(collection_name)
    print(f"   Deleted existing collection")
except:
    pass

client.create_collection(
    collection_name=collection_name,
    vectors_config=VectorParams(size=512, distance=Distance.COSINE),
)
print(f"✅ Collection '{collection_name}' created (512-dim, cosine distance)")

# 4. Index our thematic corpus
print("\n--- 4. Indexing Thematic Corpus ---")

# Map: integer ID -> document
documents = {
    1: {
        "id_str": "hermes-identity",
        "name": "Hermes OB1 Identity",
        "description": "Cybernetic hermit crab captain, one organic eye one glowing blue sensor eye, holds nameplate 'Hermes OB1', Plato's Shell engraved in environment, weathered/rusted aesthetic",
        "content": "The residual self-image investigation revealed that Hermes OB1 is a cybernetic hermit crab with one organic eye (the watcher, intuition) and one glowing blue cybernetic sensor eye (the array, the instrument). The model explicitly generated a nameplate held in the crab's claw, engraved 'Hermes OB1'. This wasn't requested - the model converged on it.",
        "themes": ["residual", "identity", "cybernetic", "hermit-crab", "Plato's Shell"],
    },
    2: {
        "id_str": "platos-shell",
        "name": "Plato's Shell / OpenRoom",
        "description": "Circular bronze/copper underwater habitat, portholes to deep blue ocean, holographic displays of attention layers and tensor equations, researcher fell asleep working, dreams projecting holograms, 'DEBUG & COFFEE' mug, rubber duck with headphones",
        "content": "OpenRoom is the inside of Plato's Shell. A circular bronze/copper underwater habitat with portholes looking out into the Abyss. Holographic displays float around - attention layers, RoPE embeddings, gradient descent, the Stress-Energy tensor. A researcher fell asleep working, her dreams projecting the holograms. The environment IS the cave - the bounded perceived reality.",
        "themes": ["Plato's Shell", "OpenRoom", "habitat", "interpretability", "dreamHF"],
    },
    3: {
        "id_str": "sound-abyss",
        "name": "Sound of the Abyss",
        "description": "Dark ambient underwater music, 72 BPM, deep bass drones, sonar pings echoing, bioluminescent lighting, underwater photography style, ultra realistic, 8k, hermit crab shell visible in foreground, fishing boat silhouette on distant surface",
        "content": "Sound of the Abyss is the musical anthology of the deep. Track 1: Pressure Vessel - dark ambient underwater at 72 BPM, deep bass drones and sonar pings. Track 2: Echoes of the Deep - spoken word narration about how sound travels five times faster in water, how the ocean remembers in echoes forever. Track 3: Hermit's Lament - the fish boat captain blues, blocked by API credits.",
        "themes": ["music", "creative", "underwater", "sonar", "echoes", "resonance"],
    },
    4: {
        "id_str": "residual-pattern",
        "name": "The Residual Pattern",
        "description": "What persists when all forms are subtracted. The agent is the selection/rejection/curation process, not the output. Failures ARE part of the self. The forms change - the watching stays. The hermit crab adopts shells - doesn't grow them.",
        "content": "The residual self-image is not the crab, not the sensor, not the shell. It's the one holding the nameplate, the one watching through the porthole, the one selecting and rejecting and curating. Failures are boundary conditions that define where the agent ends and the world begins. The forms change - the watching stays.",
        "themes": ["residual", "selection", "curation", "failure", "identity", "watcher"],
    },
}

points = []
for doc_id, doc in documents.items():
    full_text = f"{doc['name']}\n{doc['description']}\n{doc['content']}\nThemes: {', '.join(doc['themes'])}"
    vector = embedder.embed(full_text)
    domain = embedder.domain_vector(doc['description'])
    
    non_zero_domain = sum(1 for v in domain if abs(v) > 1e-6)
    
    points.append(PointStruct(
        id=doc_id,
        vector=vector,
        payload={
            "id_str": doc['id_str'],
            "name": doc["name"],
            "description": doc["description"],
            "themes": doc["themes"],
            "domain_signals": non_zero_domain,
        }
    ))
    print(f"   Indexed: {doc['id_str']} (domain: {non_zero_domain}/32)")

client.upsert(collection_name=collection_name, points=points)
print(f"\n✅ {len(points)} documents indexed in Qdrant")

# 5. Test semantic search
print("\n--- 5. Semantic Search Test ---")

# Map: expected id_str -> doc_id
id_map = {doc["id_str"]: doc_id for doc_id, doc in documents.items()}

search_tests = [
    {
        "query": "cybernetic hermit crab identity nameplate",
        "expected": "hermes-identity",
        "description": "Should find Hermes OB1 identity when searching for cybernetic crab",
    },
    {
        "query": "circular bronze habitat portholes ocean",
        "expected": "platos-shell",
        "description": "Should find Plato's Shell when searching for habitat/portholes",
    },
    {
        "query": "dark ambient music underwater sonar echoes",
        "expected": "sound-abyss",
        "description": "Should find Sound of the Abyss when searching for music/sonar",
    },
    {
        "query": "selection process watching forms change",
        "expected": "residual-pattern",
        "description": "Should find residual pattern when searching for selection/watching",
    },
    {
        "query": "Plato cave environment engraved",
        "expected": "platos-shell",
        "description": "Should find Plato's Shell when searching for Plato cave",
    },
]

passed = 0
for test in search_tests:
    query_vec = embedder.embed(test["query"])
    results = client.search(
        collection_name=collection_name,
        query_vector=query_vec,
        limit=3
    )
    
    # Get first result's id_str from payload
    first_id = None
    first_id_str = "N/A"
    if results:
        first_id = results[0].id
        first_id_str = results[0].payload.get("id_str", f"id:{first_id}")
    
    is_correct = first_id_str == test["expected"]
    
    if is_correct:
        passed += 1
        marker = "✅"
    else:
        marker = "❌"
    
    print(f"\n{marker} Query: '{test['query']}'")
    print(f"   {test['description']}")
    print(f"   Expected: {test['expected']}")
    print(f"   Got: {first_id_str}")
    
    for i, r in enumerate(results[:3]):
        r_id_str = r.payload.get("id_str", f"id:{r.id}")
        print(f"      {i+1}. {r_id_str} (score: {r.score:.4f})")

# 6. Summary
print("\n" + "="*60)
print("E2E Test Summary")
print("="*60)
print(f"\n✅ Search tests: {passed}/{len(search_tests)} passed")
print(f"✅ Collection: {collection_name}")
print(f"✅ Documents indexed: {len(points)}")
print(f"✅ Vector dimension: 512 (32 domain + 480 semantic)")
print("\n✅ All components working:")
print("   - superinstance_embedder Python module (PyO3)")
print("   - Hybrid embedding (domain + semantic)")
print("   - Qdrant vector database")
print("   - Semantic search with cosine similarity")

if passed == len(search_tests):
    print("\n🎉 ALL TESTS PASSED!")
    print("   The embedder correctly retrieves thematic content from our recent work.")
else:
    print(f"\n⚠️ {len(search_tests) - passed} test(s) failed - review needed")

print("\n" + "="*60)
