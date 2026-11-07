# UI Components - Quick Start Guide

## Quick Installation

Install all components at once:

```bash
npm install @luciddreamer/memory-visualization \
          @luciddreamer/monitoring-dashboard \
          @luciddreamer/agent-grid \
          @luciddreamer/cost-analysis
```

Or install individually:

```bash
# Memory Visualization (9/10 priority ⭐)
npm install @luciddreamer/memory-visualization

# Monitoring Dashboard (8/10)
npm install @luciddreamer/monitoring-dashboard

# Agent Grid (7/10)
npm install @luciddreamer/agent-grid

# Cost Analysis (8/10)
npm install @luciddreamer/cost-analysis
```

## Quick Setup Example

```tsx
import React, { useState } from 'react';
import {
  MemoryVisualization,
  MonitoringDashboard,
  AgentGrid,
  CostAnalysis
} from '@luciddreamer/ui-components';

export default function Dashboard() {
  // Sample data
  const [memories] = useState([
    {
      id: '1',
      content: 'User asked about memory systems',
      timestamp: new Date(),
      importance: 0.9,
      emotionalValence: 0.5,
      accessCount: 5,
      lastAccessed: new Date(),
      type: 'episodic' as const,
    },
  ]);

  const [agents] = useState([
    {
      id: '1',
      name: 'Research Agent',
      status: 'active' as const,
      type: 'Research',
      description: 'Handles research tasks',
      performance: {
        tasksCompleted: 245,
        avgResponseTime: 1200,
        successRate: 94,
      },
    },
  ]);

  return (
    <div style={{ padding: '24px' }}>
      <h1>AI Agent Dashboard</h1>

      {/* Memory Visualization */}
      <section style={{ marginBottom: '32px' }}>
        <h2>Memory System</h2>
        <MemoryVisualization
          memories={memories}
          width={800}
          height={500}
          theme="dark"
        />
      </section>

      {/* Monitoring Dashboard */}
      <section style={{ marginBottom: '32px' }}>
        <h2>System Monitoring</h2>
        <MonitoringDashboard
          metrics={systemMetrics}
          agentMetrics={agents}
          alerts={alerts}
          autoRefresh={true}
        />
      </section>

      {/* Agent Grid */}
      <section style={{ marginBottom: '32px' }}>
        <h2>Agent Management</h2>
        <AgentGrid
          agents={agents}
          selectable={true}
          onAgentClick={(agent) => console.log('Selected:', agent)}
        />
      </section>

      {/* Cost Analysis */}
      <section style={{ marginBottom: '32px' }}>
        <h2>Cost Analysis</h2>
        <CostAnalysis
          costData={costBreakdown}
          forecast={costForecast}
          theme="dark"
        />
      </section>
    </div>
  );
}
```

## Component Props Quick Reference

### MemoryVisualization

```tsx
<MemoryVisualization
  memories={MemoryItem[]}           // Required: Memory data
  onMemoryClick={(memory) => {}}    // Optional: Click handler
  onNodeHover={(node) => {}}        // Optional: Hover handler
  width={800}                        // Optional: Width (px)
  height={600}                       // Optional: Height (px)
  theme="dark"                       // Optional: 'dark' | 'light'
/>
```

### MonitoringDashboard

```tsx
<MonitoringDashboard
  metrics={SystemMetrics[]}          // Optional: System metrics
  agentMetrics={AgentMetrics[]}      // Optional: Agent data
  alerts={Alert[]}                   // Optional: Alert list
  refreshRate={5000}                 // Optional: Refresh interval (ms)
  autoRefresh={true}                 // Optional: Auto-refresh
  theme="dark"                       // Optional: 'dark' | 'light'
  onAlertClick={(alert) => {}}       // Optional: Alert handler
  onAgentClick={(agent) => {}}       // Optional: Agent handler
/>
```

### AgentGrid

```tsx
<AgentGrid
  agents={Agent[]}                   // Required: Agent data
  onAgentClick={(agent) => {}}       // Optional: Click handler
  onAgentSelect={(agent) => {}}      // Optional: Selection handler
  selectedAgentId="1"                // Optional: Selected agent ID
  columns="auto"                     // Optional: number | 'auto'
  spacing="medium"                   // Optional: 'small' | 'medium' | 'large'
  showStatus={true}                  // Optional: Show status
  showPerformance={true}             // Optional: Show metrics
  selectable={false}                 // Optional: Enable selection
  theme="dark"                       // Optional: 'dark' | 'light'
/>
```

### CostAnalysis

```tsx
<CostAnalysis
  costData={CostBreakdown}           // Required: Cost data
  forecast={CostForecast}            // Optional: Forecast data
  comparison={CostComparison}        // Optional: Provider comparison
  onProviderClick={(provider) => {}} // Optional: Provider handler
  theme="dark"                       // Optional: 'dark' | 'light'
/>
```

## Data Structure Examples

### Memory Item

```typescript
interface MemoryItem {
  id: string;
  content: string;
  timestamp: Date;
  importance: number;              // 0-1
  emotionalValence: number;        // -1 to 1
  accessCount: number;
  lastAccessed: Date;
  type: 'working' | 'episodic' | 'semantic' | 'procedural';
}
```

### Agent

```typescript
interface Agent {
  id: string;
  name: string;
  status: 'active' | 'idle' | 'busy' | 'error' | 'offline';
  type?: string;
  description?: string;
  capabilities?: string[];
  currentTask?: string;
  performance?: {
    tasksCompleted: number;
    avgResponseTime: number;
    successRate: number;
  };
  lastActive?: Date;
}
```

### System Metrics

```typescript
interface SystemMetrics {
  timestamp: Date;
  cpu: { usage: number; load: number[] };
  memory: { used: number; total: number; percentage: number };
  gpu?: { usage: number; memory: { used: number; total: number } };
  network: { requestsPerSecond: number; bytesPerSecond: number };
  api: { responseTime: number; errorRate: number; requestsPerMinute: number };
}
```

### Cost Breakdown

```typescript
interface CostBreakdown {
  period: { start: Date; end: Date };
  totalCost: number;
  totalTokens: number;
  totalRequests: number;
  providers: ProviderCost[];
  costOverTime: CostDataPoint[];
}
```

## Common Patterns

### With Real-time Data

```tsx
import { useState, useEffect } from 'react';

function RealTimeDashboard() {
  const [metrics, setMetrics] = useState([]);

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:8080/metrics');
    ws.onmessage = (e) => {
      const update = JSON.parse(e.data);
      setMetrics((prev) => [...prev.slice(-100), update]);
    };
    return () => ws.close();
  }, []);

  return <MonitoringDashboard metrics={metrics} autoRefresh={false} />;
}
```

### With Data Fetching

```tsx
function DataLoader() {
  const [data, setData] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      const res = await fetch('/api/agents');
      const json = await res.json();
      setData(json);
    };
    fetchData();
    const interval = setInterval(fetchData, 5000);
    return () => clearInterval(interval);
  }, []);

  return data ? <AgentGrid agents={data} /> : <div>Loading...</div>;
}
```

### Custom Styling

```tsx
// All components accept a theme prop
<MemoryVisualization theme="light" />
<AgentGrid theme="dark" />

// You can also wrap components and apply custom styles
<div style={{ fontFamily: 'CustomFont', backgroundColor: '#000' }}>
  <MonitoringDashboard theme="dark" />
</div>
```

## Troubleshooting

### Import Errors

```bash
# Make sure React is installed
npm install react react-dom

# For TypeScript projects
npm install --save-dev @types/react @types/react-dom
```

### Build Issues

```bash
# Clear node_modules and reinstall
rm -rf node_modules package-lock.json
npm install

# Rebuild components
npm run build
```

### Theme Not Applying

```tsx
// Make sure parent container allows theme propagation
<div style={{ colorScheme: 'dark' }}>
  <MemoryVisualization theme="dark" />
</div>
```

## Performance Tips

1. **Memoize data** to prevent unnecessary re-renders
2. **Limit history** in time-series data (keep last 100-1000 points)
3. **Use autoRefresh** sparingly (5-10 second intervals)
4. **Virtualize** large lists (1000+ items)

## Resources

- **Documentation**: See each component's README.md
- **Examples**: Check /examples directory in each package
- **Type Definitions**: See src/types/index.ts in each package
- **Storybook**: Run `npm run storybook` in any component directory

## Support

For issues or questions:
- GitHub: https://github.com/luciddreamer/ui-components
- Documentation: See individual package READMEs
- Examples: See component-specific examples

---

**Ready to build!** 🚀

All components are production-ready and fully typed. Start building your AI agent dashboard today!
