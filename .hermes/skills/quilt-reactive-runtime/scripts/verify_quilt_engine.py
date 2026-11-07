import json, asyncio
from pathlib import Path
# Assuming the WASM binding is available as `quilt_wasm` with class `QuiltEngine`
from quilt_wasm import QuiltEngine

async def main():
    engine = QuiltEngine()
    sheet_path = Path(__file__).parent.parent / 'templates' / 'quilt_sheet_example.json'
    sheet = json.loads(sheet_path.read_text())
    engine.loadSheet(sheet)
    # Get static value
    val = await engine.get('static_value')
    print('static_value =', val)
    # Compute formula
    calc = await engine.get('calc')
    print('calc =', calc)
    # Push sensor data
    await engine.push('sensor_temp', 23.5)
    # Verify listener fired (no direct output, but we can check value)
    temp = await engine.get('sensor_temp')
    print('sensor_temp =', temp)

if __name__ == '__main__':
    asyncio.run(main())
