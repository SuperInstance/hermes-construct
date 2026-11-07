# Importing Extractors from Hyphenated Packages

Python module names cannot contain hyphens, so a repository like `musician-soul` cannot be imported with a normal `import musician-soul`. The workaround used in `run_extractions.py` is:

```python
import importlib.util

def load_extractor(module_path, func_name):
    spec = importlib.util.spec_from_file_location("temp_mod", module_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return getattr(mod, func_name)

musician_soul_path = repo_root / "musician-soul" / "extractor.py"
extract_audio_prompts = load_extractor(musician_soul_path, "extract_audio_prompts")
```

This pattern:
1. Builds a `Path` to the `extractor.py` file inside the hyphenated directory.
2. Creates a temporary module spec and loads the file.
3. Retrieves the desired function by name.

It works for any hyphenated repo and avoids the need to rename directories or add `__init__.py` files.
