"""Utility module for extracting image metadata (EXIF, timestamp, tags)."""

from pathlib import Path
from PIL import Image
from PIL.ExifTags import TAGS, GPSTAGS
import json

def _get_exif_data(img_path: Path) -> dict:
    """Return a flat dict of EXIF tags (incl. DateTimeOriginal)."""
    with Image.open(img_path) as img:
        raw_exif = img._getexif() or {}
    return {
        TAGS.get(tag_id, tag_id): value
        for tag_id, value in raw_exif.items()
    }

def get_image_metadata(img_path: Path) -> dict:
    """
    Pull out a normalized metadata dict:
        - datetime: ISO‑8601 timestamp (from DateTimeOriginal)
        - tags: list of lower‑cased keywords (from ImageDescription or custom tags)
    """
    exif = _get_exif_data(img_path)

    # 1️⃣ datetime from EXIF
    iso_ts = exif.get("DateTimeOriginal") or exif.get("DateTime")
    if iso_ts:
        from datetime import datetime
        # EXIF format: "YYYY:MM:DD HH:MM:SS"
        dt = datetime.strptime(iso_ts, "%Y:%m:%d %H:%M:%S")
        iso_ts = dt.isoformat()

    # 2️⃣ tags – try a few common fields
    raw_tags = (
        exif.get("ImageDescription") or
        exif.get("UserComment") or
        ""
    )
    # split on commas / spaces, lower‑case, deduplicate
    tags = {t.strip().lower() for t in raw_tags.replace(",", " ").split() if t}

    return {"datetime": iso_ts, "tags": sorted(tags)}