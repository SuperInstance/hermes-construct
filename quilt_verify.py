"""
Quilt Engine sanity‑check script.
Loads a minimal sheet covering all cell kinds, exercises get/set/push,
and prints results. Designed for the Python WASM binding of @quilt/core.
"""
import asyncio
import json

# Import the WASM‑bound QuiltEngine – the exact import may differ depending on the binding.
# Here we assume the binding exposes a class named `QuiltEngine` in a module `quilt`.
try:
    from quilt import QuiltEngine  # type: ignore
except ImportError:
    raise RuntimeError("QuiltEngine binding not found. Ensure the @quilt/core WASM package is installed and importable as 'quilt'.")

# Minimal sheet definition covering every cell kind.
SHEET = {
    "cells": [
        {"id": "static_val", "kind": "value", "value": 42},
        {"id": "calc", "kind": "formula", "expr": "static_val * 2"},
        {"id": "api_call", "kind": "api", "url": "https://httpbin.org/get"},
        {
            "id": "prog",
            "kind": "program",
            "code": "return await runtime.get('calc');"
        },
        {
            "id": "router",
            "kind": "router",
            "rules": [
                {"when": "true", "route": "prog"}
            ]
        },
        {
            "id": "ai_cell",
            "kind": "ai",
            "ai_kind": "ai.llm",
            "provider": "zai",
            "model": "glm-4.5",
            "prompt": "Answer yes or no: is 2+2=4?"
        },
        {"id": "sensor_input", "kind": "sensor"},
        {
            "id": "listener",
            "kind": "listener",
            "listens": "static_val",
            "fn": "(newVal, oldVal) => console.log('static_val changed from', oldVal, 'to', newVal)"
        }
    ]
}

async def main():
    engine = QuiltEngine()
    # The binding may expose `load_sheet` or `loadSheet`; try both.
    if hasattr(engine, "load_sheet"):
        engine.load_sheet(SHEET)  # type: ignore
    elif hasattr(engine, "loadSheet"):
        engine.loadSheet(SHEET)  # type: ignore
    else:
        raise RuntimeError("Engine does not expose a load method.")

    # 1. Basic reads
    static = await engine.get('static_val')
    calc = await engine.get('calc')
    print('static_val =', static)
    print('calc (static_val * 2) =', calc)

    # 2. Mutate and propagate
    await engine.set('static_val', 100)
    new_calc = await engine.get('calc')
    print('After set static_val=100, calc =', new_calc)

    # 3. Program cell (calls calc)
    prog_res = await engine.get('prog')
    print('Program cell result (should be 200):', prog_res)

    # 4. Router cell (delegates to prog)
    router_res = await engine.get('router')
    print('Router cell result (delegated to prog):', router_res)

    # 5. AI cell – will error if no provider configured.
    try:
        ai_res = await engine.get('ai_cell')
        print('AI cell response:', ai_res)
    except Exception as e:
        print('AI cell error (expected without provider):', e)

    # 6. Push to sensor (listener will log via console.log in the WASM runtime)
    await engine.push('sensor_input', {'temp': 12})
    # No direct Python output; just ensure no crash.

if __name__ == '__main__':
    asyncio.run(main())
