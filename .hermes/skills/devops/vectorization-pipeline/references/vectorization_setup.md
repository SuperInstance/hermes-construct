# Vectorization Setup Details

- **File:** `vectorization_setup.py`
- **Purpose:** Given a file path via `WATCHED_PATH`, compute a deterministic embedding and upsert it to Qdrant.
- **Steps:**
  1. Validate `WATCHED_PATH` is set; skip if missing.
  2. If the path contains `.hermes-tmp`, log and return (temporary files are ignored).
  3. Read the file bytes.
  4. Compute SHA‑256 hash of the content.
  5. Convert the first 32 hex characters of the hash into a UUID (`uuid.UUID(hash[:32])`). This yields a valid Qdrant point ID.
  6. Generate a 32‑dim embedding by normalising each byte of the hash (`byte/255.0`).
  7. Infer the file type (`code`, `audio`, `graph`, or `other`).
  8. Build the upsert payload and `PUT` to `/collections/<COLLECTION>/points?wait=true`.
  9. Log success or any HTTP errors.
- **Design notes:**
  - The placeholder embedding is deterministic, enabling reproducible vectors without an external model.
  - The UUID approach avoids Qdrant's restriction to unsigned integers or UUID strings.
  - Timestamp uses UTC via `datetime.datetime.utcnow().isoformat()` (future work: switch to timezone‑aware `datetime.now(datetime.UTC)`).
- **Pitfalls addressed:**
  - Previously attempted to upsert temporary files, causing invalid ID errors.
  - Fixed indentation error that prevented execution after the first change.
  - Added explicit skip for `.hermes-tmp` files.
