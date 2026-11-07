import json
with open('C:\\Users\\casey\\hermes-construct\\tmp_repos.json') as f:
    repos = json.load(f)

categorized = {
    'DELETE_ME_ONE_CLICK': [],
    'DELETE_ME_LOW_RISK': [],
    'REVIEW_NEEDED': [],
    'KEEP': []
}

for r in repos:
    name = r.get('name','')
    desc = (r.get('description') or '').strip()
    updated = r.get('updatedAt', '')
    
    if 'auto-created' in desc.lower() or name in ['study-smartcomponent', 'study-weird-roblox-ai']:
        categorized['DELETE_ME_ONE_CLICK'].append(r)
        continue
    
    if not desc:
        categorized['DELETE_ME_LOW_RISK'].append(r)
        continue
    
    if 'robotstudio' in name.lower() or 'abb' in name.lower():
        categorized['DELETE_ME_ONE_CLICK'].append(r)
        continue
    
    if name.startswith('si-'):
        base = name[3:]
        exists = any(r2['name'] == base for r2 in repos)
        if exists:
            categorized['DELETE_ME_LOW_RISK'].append(r)
            continue
    
    dead = [
        'fleet-weather', 'zeroclaw', 'fishinglog-ai', 'fleet-dashboard',
        'system_prompts_leaks', 'snapkit-v2', 'soundcrab', 'chart-room',
        'hermit-crab', 'eigenpy', 'hermit-crab-ecology', 'roam-graph',
        'othismos-reef', 'oracle-relay', 'conformance-service',
        'plato-room-directory', 'emergency-alerts', 'smart-404',
        'domain-landing', 'search-superinstance-ai', 'activelog-app',
        'modurust-lite', 'Mentis', 'papers', 'captain', 'fleet-vessel',
        'ternary-experiment', 'openrooms', 'Spreadsheet-moment',
        'Constraint-Theory', 'forgemaster-shell', 'lau-conservation-experiment',
        'fiedler-universal', 'scummvm-gui-design', 'experiments',
        'mud-terminal', 'player-widget', 'roblox-builder-kit', 'roblox-testkit',
        'GeometryPainterThreeJS', 'nmea-bridge', 'constraints', 'othismos-llm',
        'fleet-embed', 'fleet-ensemble', 'fleet-memory', 'fleet-jepa-midi',
        'fleet-tts', 'hermes-brainstorm',
    ]
    if name.lower() in [d.lower() for d in dead]:
        categorized['DELETE_ME_LOW_RISK'].append(r)
        continue
    
    categorized['REVIEW_NEEDED'].append(r)

for cat, items in categorized.items():
    print(f'\n### {cat} ({len(items)})')
    for r in sorted(items, key=lambda x: x.get('updatedAt', '')):
        name = r['name']
        desc = (r.get('description') or '')[:70]
        print(f'  {name} | {desc}')
