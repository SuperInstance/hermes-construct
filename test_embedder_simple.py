"""
Simple Embedder + Qdrant Verification
"""
import superinstance_embedder as sie
from qdrant_client import QdrantClient
from qdrant_client.models import Distance, VectorParams, PointStruct

print("="*60)
print("Embedder + Qdrant Verification")
print("="*60)

# Init
config = sie.PyEmbedderConfig(
    model_enabled=True,
    model_type="mini-lm",
    use_cuda=False,
    qdrant_enabled=False,
)
embedder = sie.PyEmbedder(config)
client = QdrantClient(url="http://localhost:6333", check_compatibility=False)

# Test embedding
test_phrases = [
    "hermes cybernetic hermit crab captain",
    "platos shell underwater habitat porthole",
    "sound of the abyss dark ambient music",
]

print("\n--- Embedding Test ---")
for phrase in test_phrases:
    vec = embedder.embed(phrase)
    print(f"   '{phrase[:40]}...' -> {len(vec)}-dim vector")

# Collection
collection = "verify-test"
print(f"\n--- Setting up collection '{collection}' ---")
try:
    client.delete_collection(collection)
except:
    pass
client.create_collection(
    collection_name=collection,
    vectors_config=VectorParams(size=512, distance=Distance.COSINE),
)
print("   Collection created")

# Documents
docs = [
    {
        "id_int": 1,
        "id_str": "hermes",
        "text": "Hermes OB1 is a cybernetic hermit crab captain with one organic eye and one glowing blue cybernetic sensor eye. He lives in Plato's Shell and is the towfish operator, always watching through the porthole.",
    },
    {
        "id_int": 2,
        "id_str": "plato",
        "text": "Plato's Shell is a circular bronze copper underwater habitat with portholes looking out into the deep blue ocean. It is the environment, the cave, the bounded perceived reality where holographic displays of attention layers and tensor equations float around.",
    },
    {
        "id_int": 3,
        "id_str": "abyss",
        "text": "Sound of the Abyss is dark ambient underwater music with deep bass drones and sonar pings echoing. Sound travels five times faster in water. The ocean remembers in echoes forever.",
    },
]

# Upsert
print("\n--- Indexing Documents ---")
points = []
for d in docs:
    vec = embedder.embed(d["text"])
    points.append(PointStruct(
        id=d["id_int"],
        vector=vec,
        payload={"id_str": d["id_str"]}
    ))
    print(f"   Indexed: {d['id_str']}")

client.upsert(collection_name=collection, points=points)
print("   Upsert complete")

# Query
print("\n--- Running Queries ---")
test_queries = [
    ("cybernetic hermit crab captain", "hermes"),
    ("underwater habitat porthole cave", "plato"),
    ("dark ambient music sonar echoes", "abyss"),
    ("towfish operator watching porthole", "hermes"),
]

passed = 0
for query, expected in test_queries:
    query_vec = embedder.embed(query)
    
    # Use query() method
    results = client.query(
        collection_name=collection,
        query=query_vec,
        limit=3
    )
    
    first_id_str = results[0].payload.get("id_str") if results else None
    is_correct = first_id_str == expected
    
    if is_correct:
        passed += 1
        marker = "✅"
    else:
        marker = "❌"
    
    print(f"\n{marker} Query: '{query}'")
    print(f"   Expected: {expected}, Got: {first_id_str}")
    for i, r in enumerate(results):
        print(f"      {i+1}. {r.payload.get('id_str')} (score: {r.score:.4f})")

# Summary
print("\n" + "="*60)
print(f"Results: {passed}/{len(test_queries)} passed")
if passed == len(test_queries):
    print("🎉 ALL TESTS PASSED!")
    print("   Embedder + Qdrant working correctly.")
else:
    print(f"⚠️ {len(test_queries)-passed} failed")
print("="*60)
