# Project B — Simulate Package Research & Recommended Approach

This document contains the **full open-source research** and **recommendations** for implementing the **Project B simulate package**. It is based on peer-reviewed, academic, and authoritative sources, prioritizing **open-source solutions**, minimizing cost and time.

---

## 1) Math Back-End (Solvers, Parsing, Contracts)

### Findings
- **IVP ODEs:** Use SciPy’s `solve_ivp` (modern, supports RK45/Radau/BDF, dense output, events, and accuracy controls).
- **BVP ODEs:** Use SciPy’s `solve_bvp` (two-point BCs, unknown params, tolerances, max nodes).
- **Symbolic → Numeric:** Use SymPy for parsing equations and compiling to NumPy callables via `lambdify` (with strict whitelist).
- **Sampling:** Resample solver output to uniform FPS (e.g., 60 Hz) using cubic Hermite or centripetal Catmull-Rom interpolation.
- **Curve Frames:** Compute Frenet–Serret T/N/B frames with curvature fallback.

### Conclusion
**FastAPI + Pydantic + SymPy + SciPy** is the lowest-friction, fully OSS stack for parsing and solving user-defined ODE/BVPs with stability and strong documentation.

---

## 2) Real-Time “Actual” Stream (Bridge)

### Findings
- **Protocol:** WebSockets for bidirectional, low-latency streaming (Python `websockets` for server).
- **Client Integration:** Use `react-use-websocket` in React for reconnects, heartbeats, and JSON handling.
- **Alternative:** FastAPI WebSockets possible for unified service, but a thin bridge is cleaner.

### Conclusion
Keep a **thin WebSocket bridge** that normalizes IM state into `{t, nodes, edges}` and streams at fixed cadence.

---

## 3) 3D Visualization (Ideal + Actual in Same Scene)

### Findings
- **Engine:** Three.js with React Three Fiber (R3F) for declarative React integration; Drei for helpers.
- **Geometry:** TubeGeometry extruded along curves for ideal and actual paths.
- **State Mgmt:** Zustand for store, Leva for UI controls.
- **Volumetric Error:** Optional Marching Cubes or slice planes for error visualization.

### Conclusion
**R3F + Drei + Zustand + Leva** on top of **Three.js** gives rapid iteration, OSS support, and minimal boilerplate.

---

## 4) Error Metrics & Comparison Math

### Findings
- **Deviation:** Compute shortest distance from actual point to ideal polyline segment (projection onto segment).
- **Time Alignment:** Resample both trajectories to shared clock for error calculation.
- **Orientation Diagnostics:** Frenet frames enable tangential vs. normal error breakdown.

---

## 5) PDE Track (Future-Proofing)

### Findings
- **FEniCSx:** FEM, strong academic heritage, Python interface, heavier dependencies.
- **Dedalus:** Spectral methods, peer-reviewed, MPI-parallel, great for smooth domains.

### Conclusion
Skip PDEs in v1 to save cost/time; design mathcore so PDE solvers can be plugged in later.

---

## 6) Why This Stack Minimizes Time & Cost

- Mature OSS components with strong documentation (SciPy, SymPy, FastAPI, Three.js, R3F).
- Direct SymPy → lambdify → SciPy flow removes need for custom DSL.
- WebSocket streaming is well-supported; `react-use-websocket` reduces boilerplate.
- R3F ecosystem enables rapid prototyping with fewer lines of code.

---

## Proposed v1 Scope

1. **mathcore (FastAPI)** — `/solve/ode`, `/solve/bvp`:
   - SymPy-parsed equations → lambdified RHS → SciPy solvers → dense output → Hermite/Catmull-Rom resample → (optional) Frenet frames.  

2. **sim/bridge (WS): Publish {t, nodes, edges}**:
   - Publish {t, nodes, edges} frames @ 60 Hz. Handle Backpressure: drop/coalesce old frames; JSON (or binary later).   

3. **viz (R3F)**:
   - One scene, two tubes (ideal/actual) + time marker + deviation vector; scrubber; params panel; optional marching-cubes error field.

---

## Considerations

- Combining REST + WS in FastAPI is possible but separate bridge offers clearer separation.
- For heavy streaming, consider binary WS messages later (Float32Array).

---
