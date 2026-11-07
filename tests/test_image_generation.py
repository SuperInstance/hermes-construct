import pytest
import sys
import os

# Ensure scripts are importable
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'scripts')))

import tag_images
import generate_tag_index

def test_modules_load():
    assert hasattr(tag_images, 'tag_new_images')
    assert hasattr(generate_tag_index, 'build_tag_index')