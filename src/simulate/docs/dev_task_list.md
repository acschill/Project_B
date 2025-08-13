# Project B — Simulate Package
## Goal-Oriented Implementation Task List (Developer Runbook)

**Scope:** End-to-end plan derived from the approved open‑source research for building the **simulate** package (mathcore + sim bridge + viz) with zero licensing cost, minimal time, and maximum correctness.

---

## Conventions
- **DoD** = Definition of Done (acceptance criteria)
- **ETA** = Rough solo‑dev effort under normal conditions
- `simulate/` is the module root introduced in the repo

---

## Phase 0 — Scaffolding & Environment (Day 0)

### T0.1 Create directories & base files
**Goal:** Establish authoritative structure and placeholder stubs.
- Create:
  - `simulate/{devtools,mathcore,sim,viz}`
  - `simulate/.env.example`
  - `simulate/README.md` (short pointer to Technical Overview & this runbook)

**DoD**
- Tree matches spec and is committed.

---

## Phase 1 — Data Contracts & Config (Day 0)

### T1.1 Define canonical contracts (mathcore)
**Goal:** Ensure stable APIs for solver requests/responses.
- `simulate/mathcore/contracts.py`:
  - `ODERequest` (vars, tvar, equations, params, ivp with tolerances/method)
  - `ODEResponse` (times: list[float], points: list[list[float]], optional frenet)
  - `BVPRequest`/`BVPResponse` (domain, bc, init_guess, tol, max_nodes)

**DoD**
- Pydantic models serialize/validate round‑trip JSON examples from the research docs.

### T1.2 Define canonical contracts (bridge)
**Goal:** A stable, typed schema for actual frames.
- `simulate/sim/schema.py`:
  - `Node = tuple[float,float,float]`
  - `Edge = {from:str, to:str, rate:float}`
  - `ActualFrame = {t:float, nodes:dict[str,Node], edges:list[Edge]}`

**DoD**
- Minimal unit tests that construct valid/invalid frames and assert validation behavior.

### T1.3 Centralize config
**Goal:** Single source for ports/FPS/URLs.
- `simulate/sim/config.py` → `SIM_BRIDGE_PORT`, `SIM_BRIDGE_HOST`, `FPS=60`
- `simulate/.env.example`:
  - `MATHCORE_PORT=8001`
  - `SIM_BRIDGE_PORT=8002`
  - `MATHCORE_URL=http://localhost:${MATHCORE_PORT}`
  - `SIM_BRIDGE_URL=ws://localhost:${SIM_BRIDGE_PORT}`

**DoD**
- Reading `.env` or environment variables works; defaults sane when `.env` missing.

---

## Phase 2 — Math Core (FastAPI + SymPy + SciPy) (Days 1–2)

### T2.1 Parser & safety
**Goal:** Convert user equations → safe numeric functions.
- `simulate/mathcore/parsers.py`:
  - Use SymPy `parse_expr` with `standard_transformations`
  - Whitelist: basic arithmetic, common math functions (`sin`, `cos`, `exp`, `log`, etc.)
  - Create symbol table from `vars` + `tvar` + `params`
  - Build RHS list in `vars` order: map keys like `"dx/dt"` → `x`
  - `lambdify(..., "numpy")` to numeric RHS
  - Explicitly **reject** names with `__`, attributes, lambda, assignment

**DoD**
- Unit test: Lorenz system strings parse to callable; invalid tokens raise clean errors.

### T2.2 ODE IVP solver wrapper
**Goal:** Accurate, configurable IVP solutions with dense output.
- `simulate/mathcore/solvers.py`:
  - `solve_ode_ivp(req: ODERequest)` → use SciPy `solve_ivp`
  - Default method `RK45`; support `"BDF"`/`"Radau"` for stiff
  - Pass through `rtol`, `atol`; add event hooks slot (future)
  - Return SciPy solution object (dense), not yet resampled

**DoD**
- Given Lorenz IVP, solution returns and is bounded; no exceptions with typical tolerances.

### T2.3 BVP solver wrapper
**Goal:** Support simple BVPs for completeness.
- `solve_bvp_wrapper(req: BVPRequest)` using SciPy `solve_bvp`
- Apply `tol`, `max_nodes`; initial mesh from domain; basic guess handling

**DoD**
- Example second‑order linear BVP converges under default tol; node count < `max_nodes`.

### T2.4 Resampling & Frenet frames
**Goal:** Smooth, frame-locked playback samples + orientation.
- `simulate/mathcore/sampling.py`:
  - Build uniform `times` for ~60 fps: `np.linspace(t0,t1,N)`
  - Evaluate dense solution across `times`
  - Interpolate with cubic Hermite or centripetal Catmull‑Rom where needed
  - Compute Frenet T/N/B; handle near‑zero curvature with fallback normal

**DoD**
- Numeric checks: C¹ continuity across samples; Frenet vectors unit‑norm; no NaNs.

### T2.5 FastAPI endpoints
**Goal:** Expose `/solve/ode`, `/solve/bvp` with contracts.
- `simulate/mathcore/app.py`:
  - `POST /solve/ode` → validate → parse → solve → resample → (opt) frenet → response
  - `POST /solve/bvp` → similar, with `x` as independent variable
  - CORS for `localhost:5173` (viz dev)
  - Swagger at `/docs`

**DoD**
- cURL/HTTPie examples succeed; Swagger shows clean models.

### T2.6 Unit tests
**Goal:** Lock in correctness.
- `simulate/mathcore/tests/`:
  - Harmonic oscillator vs analytic solution error < 1e‑3 over window
  - Lorenz qualitative: ranges within known envelope; no solver failure
  - BVP: residual < tol; mesh nodes reasonable

**DoD**
- `pytest` passes locally in clean venv.

---

## Phase 3 — WebSocket Bridge (Days 2–3)

### T3.1 Bridge server
**Goal:** Reliable WS broadcaster with cadence/backpressure.
- `simulate/sim/bridge.py`:
  - `websockets.serve(handler, host, port)`
  - Loop at `FPS` reading the latest IM snapshot from Project B (plug‑in hook)
  - Coalesce frames if slow consumer; drop oldest under pressure
  - Broadcast JSON frames (`ActualFrame`) to all clients

**DoD**
- Local fake generator streams at 60 fps with stable memory; multiple clients supported.

### T3.2 IM integration hook
**Goal:** Real data from Project B.
- Add minimal observer in Project B’s `im_controller` to expose current state
- In `bridge.py`, replace dummy with real snapshot pull + normalization to IM basis

**DoD**
- When Project B runs, viz receives moving node positions and edge rates.

### T3.3 Health & metrics (optional nice-to-have)
**Goal:** Operational visibility.
- Basic `/health` HTTP or WS ping/pong stats
- Log frame age, dropped count

**DoD**
- Console metrics observable under load.

---

## Phase 4 — Visualization App (Days 3–5)

### T4.1 Vite + R3F bootstrap
**Goal:** Running React app with R3F/Drei.
- `simulate/viz/package.json` with deps: `react`, `three`, `@react-three/fiber`, `@react-three/drei`, `zustand`, `leva`, `react-use-websocket`
- `src/main.tsx`, `src/App.tsx`: Canvas with OrbitControls; basic lights

**DoD**
- Page renders blank scene; no errors; hot reload works.

### T4.2 State store
**Goal:** Central timeline + frames + params.
- `simulate/viz/src/store.ts`:
  - State: `time`, `playing`, `ideal {times, points, frenet?}`, `actual {frames}`, `params {method, rtol, atol, fps}`
  - Actions: `play()`, `pause()`, `seek(t)`, `setIdeal()`, `appendActual(frame)`, `setParams()`
  - Selectors: `sampleIdealAt(t)`, `sampleActualAt(t)` (binary search + lerp)

**DoD**
- Unit tests (Jest or vitest) for selectors and time seeking.

### T4.3 API clients
**Goal:** Mathcore REST + Bridge WS.
- `simulate/viz/src/api.ts`:
  - `postSolveOde(body)` and `postSolveBvp(body)`
  - `useWsBridge(url)` using `react-use-websocket` (auto reconnect, JSON messages)
  - Queue messages; flush to store in RAF or interval to avoid render storms

**DoD**
- Hitting `/solve/ode` renders a sample ideal path; WS shows moving marker.

### T4.4 Scene: twin tubes + markers
**Goal:** Co-spatial, time-synced visuals.
- `simulate/viz/src/scene/GraphScene.tsx`:
  - Ideal tube from `ideal.points` using TubeGeometry/fatline
  - Actual tube from accumulated actual centroid / key node path
  - Two time markers (spheres) bound to store `time`
  - Materials: differentiate ideal vs actual; keep perf via BufferGeometries

**DoD**
- Scrubber moves markers smoothly on both trajectories.

### T4.5 Deviation vector & metrics
**Goal:** Quantify difference live.
- Compute closest point from actual to ideal polyline segment near current time
- Draw deviation vector; HUD shows `e(t)` and windowed RMSE
- Add Leva controls to toggle deviation, RMSE window size

**DoD**
- `e(t)` responds to scrubbing and play; matches visual vector length.

### T4.6 Panels & UX
**Goal:** Self-service experimentation.
- `EquationEditor.tsx`: Textareas for equations/params; dropdown for method; tolerances
- `ParamsPanel.tsx`: FPS, toggles (deviation, error field), camera presets
- `Scrubber.tsx`: play/pause, speed, loop, ←/→ step

**DoD**
- Entering Lorenz equations produces correct-looking attractor; controls work.

### T4.7 Optional: ErrorField
**Goal:** Volumetric error visualization (toggle).
- Sample 3D box around IM; compute scalar error `E(r,t)` (start with distance field around ideal path)
- Render MarchingCubes isosurface (low resolution first); add slice planes
- Compute on a worker to keep main thread responsive

**DoD**
- Toggle on/off; FPS remains ≥ 30 on dev machine.

---

## Phase 5 — Quality, Performance, and Safety (Days 5–6)

### T5.1 Performance hardening
**Goal:** 60 fps target in typical scenes.
- Instanced meshes and typed BufferAttributes
- Frame batching (apply actual-frame updates at a fixed cadence)
- Avoid GC churn (reuse arrays/Float32Array buffers)

**DoD**
- Telemetry shows ≤ 5% dropped frames with 60 fps target on dev machine.

### T5.2 Parser security
**Goal:** Prevent code execution via equations.
- Strict whitelist for functions/symbols
- No attributes, lambdas, or eval-like constructs
- Fuzz a set of malicious inputs; assert rejection

**DoD**
- Fuzz tests pass; only math expressions are accepted.

### T5.3 Documentation & examples
**Goal:** Developer can run without external lookup.
- `simulate/README.md` quick start
- Example JSON payloads for ODE/BVP
- Screenshot/GIF of the viz

**DoD**
- New dev reproduces demo in <15 minutes on clean machine.

---

## Phase 6 — (Optional) Parameter Fitting & PDE (Week 2+)

### T6.1 Fit-to-actual
**Goal:** Estimate ideal model params to better match actual.
- Mathcore `/fit` using SciPy `least_squares` (JAX later)
- Optimize `theta` to minimize Σ ||x_actual(tk)-x_ideal(tk;θ)||²
- UI toggle “Fit params” + live results

**DoD**
- For synthetic data with known params, recovered θ within ±ε.

### T6.2 PDE module (future)
**Goal:** Field-level models if needed.
- Add `/solve/pde` with FEniCSx (or Dedalus for spectral domains)
- Keep separate module to avoid heavy deps for users who don’t need PDEs

**DoD**
- Simple 3D diffusion/wave example returns a field and renders as slices/isosurface.

---

## Deliverables Checklist

- ✅ `simulate/mathcore` with `/solve/ode`, `/solve/bvp`, tests
- ✅ `simulate/sim` streaming real IM frames via WS
- ✅ `simulate/viz` R3F app with twin tubes, scrubber, deviation vector
- ✅ Dev scripts: `run_mathcore.sh`, `run_bridge.sh`, `run_viz.sh`, `stop_all.sh`
- ✅ Docs: README + example payloads; this runbook
- ⭕ Optional: ErrorField, Fit-to-actual, PDE module

---

## Risk Register & Mitigations

- **Stiff systems stall playback** → Use `BDF/Radau`, lower tolerances, clamp max step, pre-sample with dense output.  
- **Parser abuse** → Strict whitelist + fuzz tests.  
- **WS backpressure** → Coalesce frames, drop oldest, queue limits.  
- **Frame jitter** → Shared clock + resampling; interpolate both ideal & actual.  
- **Volumetrics slow** → Keep grid coarse, throttle updates, worker threads.

---

## Quick Commands

```bash
# mathcore
cd simulate/mathcore && python -m venv .venv && source .venv/bin/activate
pip install -U pip fastapi uvicorn sympy scipy pydantic numpy
uvicorn app:app --reload --port 8001

# bridge
cd simulate/sim && python -m venv .venv && source .venv/bin/activate
pip install -U pip websockets pydantic numpy python-dotenv
python bridge.py

# viz
cd simulate/viz
npm i
npm run dev
```

---

## Definition of “Ready for Demo”

1) Enter Lorenz ODE in UI → ideal tube renders; scrubbing smooth at ≥60 fps.  
2) Bridge streams a canned IM playback → actual tube + marker track live.  
3) Deviation vector + numerical `e(t)` match visually.  
4) Tolerances/methods toggled in UI affect smoothness/shape as expected.  
5) Optional: error isosurface toggles without tanking FPS below 30.

---

## Appendix — File Map to Tasks

- `mathcore/contracts.py` → T1.1  
- `mathcore/parsers.py` → T2.1  
- `mathcore/solvers.py` → T2.2, T2.3  
- `mathcore/sampling.py` → T2.4  
- `mathcore/app.py` → T2.5  
- `mathcore/tests/*` → T2.6  
- `sim/schema.py` → T1.2  
- `sim/config.py` → T1.3  
- `sim/bridge.py` → T3.1–T3.3  
- `viz/src/api.ts` → T4.3  
- `viz/src/store.ts` → T4.2  
- `viz/src/scene/GraphScene.tsx` → T4.4  
- `viz/src/scene/IdealLayer.tsx` → T4.4  
- `viz/src/ui/*` → T4.6  
- `devtools/*` → deploy scripts (support all phases)

---

**End of Runbook**
