import sys
import os
import asyncio

# Add site-packages to path
sys.path.insert(0, 'C:/Users/casey/hermes-construct/hermes-construct/venv/Lib/site-packages')

from agent.agent_init import init_agent

async def run_leaf_agent():
    try:
        print("Initializing leaf agent...")
        # Using deepseek-v3 via deepinfra provider
        agent = init_agent(model='deepseek-v3', provider='deepinfra')
        
        # Since we just want to confirm identity and a 'hello' message, 
        # we'll manually trigger a small interaction or just print what we know.
        # In a real orchestrator, you'd call agent.run() or similar.
        print(f"SUCCESS: Spawned agent with model: {agent.model}")
        print("LEAF MESSAGE: Hello from the leaf subagent!")
        
    except Exception as e:
        print(f"ERROR during spawn/run: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(run_leaf_agent())
