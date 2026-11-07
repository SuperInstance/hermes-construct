# Metadata Extraction Utilities

These functions extract timestamps and tags from generated PNGs using PIL's EXIF support.

```python
from pathlib import Path
from PIL import Image
from PIL.ExifTags import TAGS, GPSTAGS
import json

def _get_exif_data(img_path: Path) -> dict:
    with Image.open(img_path) as img:
        raw_exif = img._getexif() or {}
    return {
        TAGS.get(tag_id, tag_id): value
        for tag_id, value in raw_exif.items()
    }

def get_image_metadata(img_path: Path) -> dict:
    exif = _get_exif_data(img_path)
    iso_ts = exif.get("DateTimeOriginal") or exif.get("DateTime")
    if iso_ts:
        from datetime import datetime
        dt = datetime.strptime(iso_ts, "%Y:%m:%d %H:%M:%S")
        iso_ts = dt.isoformat()
    raw_tags = exif.get("ImageDescription") or exif.get("UserComment") or ""
    tags = {t.strip().lower() for t in raw_tags.replace(",", " ").split() if t}
    return {"datetime": iso_ts, "tags": sorted(tags)}
```