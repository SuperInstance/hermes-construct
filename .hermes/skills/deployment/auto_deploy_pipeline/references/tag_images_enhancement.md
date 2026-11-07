# Tag Images Enhancement

The `tag_images.py` script now:

1. Generates a timestamp-based tag folder (YYYY-MM-DD_HH).  
2. Copies the PNG into that folder.  
3. Writes a side‑car `.meta.json` file containing:
   - `datetime` – ISO‑8601 timestamp from EXIF.
   - `tags` – list of lower‑cased keywords extracted from EXIF tags.

Example output:

```json
{
  "datetime": "2026-08-24T15:12:00.123456",
  "tags": ["alaska", "submarine", "hermes"]
}
```