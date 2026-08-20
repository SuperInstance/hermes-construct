"""Generate a simple tag index page for browsing images by tag."""

import os
from pathlib import Path

IMG_DIR = Path(__file__).parents[0] / "images"
TAG_DIR = IMG_DIR / "_tagged"

def _tag_folders():
    """Yield (tag_name, list_of_image_paths) for each tag folder."""
    for tag_folder in TAG_DIR.iterdir():
        if tag_folder.is_dir():
            tag_name = tag_folder.name
            images = list(tag_folder.glob("*.png"))
            yield tag_name, images

def build_html():
    """Construct a minimal HTML page that lists tags and thumbnails."""
    lines = [
        "<!DOCTYPE html>",
        "<html lang=\"en\">",
        "<head>",
        "  <meta charset=\"UTF-8\">",
        "  <title>Image Tags</title>",
        "  <link rel=\"stylesheet\" href=\"css/style.css\">",
        "</head>",
        "<body>",
        "  <nav>",
        "    <ul>",
        "      <li><a href=\"index.html\">Home</a></li>",
        "      <li><a href=\"feedback.html\">Feedback</a></li>",
        "    </ul>",
        "  </nav>",
        "  <main>",
        "    <h1>Image Tags</h1>",
        "    <div id=\"tag-list\">"
    ]

    for tag_name, images in _tag_folders():
        lines.append(f'      <div class="tag-section">')
        lines.append(f'        <h2>{tag_name}</h2>')
        for img_path in images[:8]:  # limit to 8 thumbnails per tag
            # Relative URL from this HTML file
            rel_path = f"../{img_path}"
            lines.append(f'        <figure><img src="{rel_path}" alt="{img_path.name}"></figure>')
        lines.append('      </div>')

    lines.extend([
        "    </div>",
        "  </main>",
        "  <script src=\"js/lightbox.js\"></script>"
    ])

    lines.append("</body>")
    lines.append("</html>")
    return "\n".join(lines)

if __name__ == "__main__":
    html = build_html()
    # Output to this repo root as tags.html
    out_path = Path(__file__).parents[0] / "tags.html"
    out_path.write_text(html, encoding="utf-8")
    print(f"Tag index written to {out_path}")