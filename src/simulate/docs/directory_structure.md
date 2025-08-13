# Project B — Simulate Package
## Directory Structure

```plaintext
simulate/
├── devtools/
│   ├── run_mathcore.sh         # Launch FastAPI solver service
│   ├── run_bridge.sh           # Launch sim-to-viz WebSocket bridge
│   ├── run_viz.sh              # Launch R3F development server
│   └── stop_all.sh             # Kill all simulate services
│
├── mathcore/
│   ├── __init__.py
│   ├── app.py                  # FastAPI entrypoint with /solve endpoints
│   ├── contracts.py            # Pydantic models for request/response validation
│   ├── parsers.py              # SymPy-based equation parser and safety enforcement
│   ├── solvers.py              # SciPy solve_ivp and solve_bvp wrapper logic
│   ├── sampling.py             # Resampling to uniform FPS; Frenet frame computation
│   ├── tests/
│   │   ├── __init__.py
│   │   ├── test_ivp.py         # Unit tests for IVP solver
│   │   ├── test_bvp.py         # Unit tests for BVP solver
│   │   ├── test_parser.py      # Unit tests for equation parsing
│   │   └── test_sampling.py    # Unit tests for resampling & Frenet frames
│   └── __pycache__/            # (ignored) Python cache
│
├── sim/
│   ├── __init__.py
│   ├── bridge.py               # WS server to stream actual IM frames
│   ├── schema.py               # Pydantic models for actual frames
│   ├── config.py               # Interface config (ports, FPS)
│   └── __pycache__/            # (ignored) Python cache
│
├── viz/
│   ├── package.json            # NPM dependencies
│   ├── vite.config.ts          # Vite config
│   └── src/
│       ├── main.tsx            # React app bootstrap
│       ├── App.tsx             # Root component, provider setup
│       ├── api.ts              # HTTP + WS clients (mathcore & sim bridge)
│       ├── store.ts            # Zustand store: time, ideal, actual frames
│       ├── scene/
│       │   ├── GraphScene.tsx   # Combined 3D scene: actual, ideal, deviation
│       │   ├── IdealLayer.tsx   # Ideal path: tubes + Frenet glyphs
│       │   └── ErrorField.tsx   # Optional 3D error volume rendering
│       └── ui/
│           ├── Scrubber.tsx      # Time control
│           ├── EquationEditor.tsx# User inputs DEs/BCs
│           └── ParamsPanel.tsx   # Solver settings (method, tolerances)
│
├── .env.example                # Optional config template
└── README.md                   # Brief overview and pointers to docs
```
