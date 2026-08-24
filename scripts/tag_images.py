"""Tag images based on metadata / timestamps and move them into tag folders."""

import os
import shutil
from pathlib import Path
from datetime import datetime

# Root folder that holds generated PNGs
IMG_DIR = Path(__file__).parents[0] / "images"
# Folder where tag subfolders will be created
TAG_DIR = IMG_DIR / "_tagged"
TAG_DIR.mkdir(exist_ok=True)

def _make_tag(img_path: Path) -> str:
    """
    Derive a tag from the image's modification timestamp.
    Format: YYYY-MM-DD_HH
    """
    ts = datetime.fromtimestamp(img_path.stat().st_mtime)
    return ts.strftime("%Y-%m-%d_%H")

def tag_images() -> None:
    """Walk the image directory and copy each PNG into a tag folder."""
    for img_path in IMG_DIR.glob("*.png"):
        if not img_path.is_file():
            continue
        tag = _make_tag(img_path)
        tag_folder = TAG_DIR / tag
        tag_folder.mkdir(exist_ok=True)
        # Copy instead of move so the original stays in the gallery
        shutil.copy(img_path, tag_folder / img_path.name)
        print(f"Tagged {img_path.name} → {tag}")

if __name__ == "__main__":
    tag_images()