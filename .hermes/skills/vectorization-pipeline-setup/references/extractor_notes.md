# Extractor Notes

## Musician‑Soul extractor
- Walks `musician-soul/src` and finds audio files (`.wav`, `.mp3`, `.flac`, `.ogg`).
- Returns a human‑readable description `"Audio file <rel_path> – size <bytes> bytes"`.
- Designed for the vector pipeline; can be extended to extract metadata (sample rate, duration) by adding a library such as `pydub`.

## Knowledge‑Graph extractor
- Recursively scans the repository for `.ttl` files.
- Strips comments and blank lines, returns each remaining line as a raw triple string.
- Simple but sufficient for semantic search; for richer semantics replace with `rdflib` parsing.
