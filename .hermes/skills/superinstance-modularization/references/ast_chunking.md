# AST Chunking Reference

## Goal
Break source files into logical, semantically meaningful chunks for embedding.

## Approach (`ast_chunking.py`)
- Parse Python files with the built‑in `ast` module.
- Walk the AST, collecting nodes of type `FunctionDef`, `ClassDef`, and top‑level statements.
- For each node, extract the source slice using `ast.get_source_segment`.
- Emit a JSON line per chunk:
  ```json
  {"path": "src/module.py", "type": "function", "name": "my_func", "code": "def my_func(...): ..."}
  ```
- Optionally compute a hash of the chunk for deduplication.

## Sample Code Snippet
```python
import ast, pathlib, json

def chunk_file(path):
    src = pathlib.Path(path).read_text()
    tree = ast.parse(src)
    chunks = []
    for node in ast.iter_child_nodes(tree):
        if isinstance(node, (ast.FunctionDef, ast.ClassDef)):
            code = ast.get_source_segment(src, node)
            chunks.append({
                "path": str(path),
                "type": type(node).__name__.lower(),
                "name": node.name,
                "code": code,
            })
    return chunks

if __name__ == "__main__":
    import sys
    for p in sys.argv[1:]:
        for c in chunk_file(p):
            print(json.dumps(c))
```

## Pitfalls
- **Indentation loss** – `ast.get_source_segment` preserves original formatting; avoid `ast.unparse` which may reformat.
- **Non‑Python files** – guard with file extension check before parsing.
- **Large files** – process line‑by‑line or stream to avoid memory blow‑up.

## Integration Points
- Called by `watchdog_vectorization.py` when a `.py` file changes.
- Output can be piped directly to the embedding service (e.g., Qdrant loader).

---
*Generated from session 2026‑08‑17*