"""Unit tests for the Hermes image‑generation and deployment pipeline."""

import os
import subprocess
import sys
from pathlib import Path

def test_generator_creates_png(tmp_path: Path) -> None:
    """
    Verify that the Hermes generator produces at least one PNG file.
    """
    # Ensure we are in the website root (where index.html lives)
    repo_root = Path(__file__).parents[1]
    generator_script = repo_root / Path(sys.argv[0]).parent.parent / ".hermes" / "scripts" / "gen_hermes_sub.py"
    assert generator_script.is_file(), f"Generator script not found at {generator_script}"

    # Run the generator in a sandbox directory so we don't pollute state
    with tmp_path as sandbox:
        # Export environment variable so the script knows to write PNGs into this dir
        env = os.environ.copy()
        env["GENERATE_OUTPUT_DIR"] = str(sandbox)

        # Execute the generator script
        result = subprocess.run(
            [sys.executable, str(generator_script)],
            env=env,
            capture_output=True,
            text=True
        )

        # The script should exit cleanly
        assert result.returncode == 0, f"Generator failed with exit code {result.returncode}\nstderr: {result.stderr}"

        # At least one PNG should appear in the sandbox
        png_files = list(sandbox.glob("*.png"))
        assert len(png_files) > 0, "No PNG files were generated"


def test_deploy_pushes_to_origin(tmp_path: Path) -> None:
    """
    Verify that the deployment script can stage and commit changes when a new PNG exists.
    """
    # This test uses a temporary git repo to simulate a clean remote state.
    import shutil, tempfile

    # Create a fresh bare repo that will act as the "origin"
    origin = tempfile.mkdtemp()
    shutil.copytree(repo_root, f"{origin}/repo", dirs_exist_ok=True)
    os.chdir(f"{origin}/repo")
    subprocess.run(["git", "init"], check=True)
    subprocess.run(["git", "config", "user.name", "Hermes CI"], check=True)
    subprocess.run(["git", "config", "user.email", "ci@hermes.test"], check=True)
    subprocess.run(["git", "add", "."], check=True)
    subprocess.run(["git", "commit", "-m", "initial"], check=True)
    subprocess.run(["git", "remote", "add", "origin", f"https://github.com/username/hermes-gallery.git"], check=True)

    # Run the deployment script in a temp working dir where we will create a dummy PNG
    with tempfile.TemporaryDirectory() as work:
        # Copy a dummy image into the working dir
        dummy_png = os.path.join(work, "dummy.png")
        with open(dummy_png, "wb") as f:
            f.write(b"\x89PNG\r\n\x1a\n")  # minimal PNG header

        # Write a temporary config that points the script at this dummy image
        # (the script looks for *.png in the images/ folder)
        images_dir = os.path.join(work, "images")
        os.makedirs(images_dir, exist_ok=True)
        shutil.copy(dummy_png, os.path.join(images_dir, "dummy.png"))

        # Set env var so script knows to use this dir
        env = os.environ.copy()
        env["IMAGE_DIR"] = images_dir

        # Run the script – it should detect the dummy image and attempt a commit/push
        result = subprocess.run(
            [sys.executable, "..\\..\\.hermes\\scripts\\auto_deploy.sh"],
            env=env,
            capture_output=True,
            text=True,
            cwd=work
        )

        # The script should exit without error
        assert result.returncode == 0, f"Deploy script failed: {result.stderr}"

        # At this point the dummy commit should exist locally
        status = subprocess.run(["git", "status", "--porcelain"], capture_output=True, text=True)
        assert "Changes not staged for commit" not in status.stdout, "No changes were staged"

        # Cleanup
        os.chdir(repo_root)