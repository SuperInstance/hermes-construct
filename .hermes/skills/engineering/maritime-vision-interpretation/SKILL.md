---
name: maritime-vision-interpretation
description: Interpreting Nobeltec TZpro telemetry and sonar imagery for maritime situational awareness.
---

# Maritime Visual Perception & Sonar Interpretation

This skill governs the process of interpreting high-fidelity maritime telemetry, specifically the Nobeltec TZpro interface, through a "Submarine Resident" lens.

## Core Principles
- **Semantic Mapping:** Do not treat UI elements as pixels; treat them as functional maritime entities (Heading, Speed, Depth, Contours).
- **The Red Line Constraint:** The 51-fathom contour (Red Line) is a critical safety boundary. Any trajectory approaching this line triggers an immediate "Critical Depth Breach" alert.
- **Temporal-Spatial Awareness:** Recognize that side-view/sonar panels are time-series projections. Use the "Echogram" to understand the geometry of the seabed over time.
- **Biological Signature Recognition:** Distinguish between "Hard" returns (seabed/rocks) and "Soft/Biological" returns (fish/plankton/bait balls) based on pixel intensity (Heatmap) and color temperature (Orange/Yellow vs Blue/Black).

## Operational Workflow
1. **Capture:** Perform a multi-monitor visual sweep.
2. **Parse:** Convert visual elements into a structured "Semantic Map" (Heading, Speed, Depth, Contour status).
3. **Monitor:** Continually evaluate "Trigger Patterns" (e.g., a sudden spike in pixel intensity in the sonar pane for a "Strike" or a proximity alert for the Red Line).
4. **Report:** Communicate findings using Alaskan maritime standards (DMS, NM, Fathoms).

## Patterns & Triggers
- **Strike Pattern:** Sudden transition from dark blue/black to high-intensity orange/yellow pixels in the vertical sonar pane.
- **Contour Breach:** Vessel depth $\le$ 51 fathoms.
- **Interference Pattern:** Vertical line networks (potential vessel interference).

## References
- references/residual-self-image-mirror.md — multi-model vision; negative-space recording; CUDA contradiction (2026-08-13)

## Pitfalls
- Vision/sonar failure = data, not absence. Record explicitly.
- Memory CUDA 13.2 vs torch CPU = contradiction to report, not suppress.
- Windows Local Vision Path Fix: `vision_analyze` / `browser_vision` require full absolute Windows-style paths (e.g. `C:\Users\casey\ai-writings\artwork\crab_silicon_1_001.jpg`) — relative or MSYS-style paths will fail with "Invalid image source" error.
