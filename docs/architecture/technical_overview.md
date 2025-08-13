# Project B — Technical Overview

## 1. General Overview

Project B is an independent agent architecture designed for low-cost devices (e.g., Raspberry Pi 5). The control plane is implemented in **Rust** (state machines, scheduling, budgets, messaging), while performance‑critical emotional inference and math kernels are implemented in **C++** and exposed to Rust via a safe FFI (Foreign Function Interface) boundary using the `cxx` crate. The system maintains a low‑overhead **base awareness**, escalates effort on **precision‑weighted prediction errors (PWPE (Precision‑Weighted Prediction Error))** when warranted by **Expected Value of Control (EVC (Expected Value of Control))**, globally integrates context when ignition criteria are met, and optionally models **emotional awareness** with a multi‑model **Emotional Awareness Engine (EAE (Emotional Awareness Engine))** that can be disabled via config.

### Design Tenets
- **Rust control plane** for memory safety, predictable latency, and trait‑based “OO” composition.
- **C++ kernels** for SIMD (Single Instruction, Multiple Data)‑friendly math and library access (NEON (ARM Advanced SIMD), Eigen/ONNX Runtime/TFLite if needed).
- **PWPE (Precision‑Weighted Prediction Error)→EVC (Expected Value of Control) gating** for cost‑aware escalation; **adaptive gain** for arousal/scheduling (inverted‑U).
- **Global Synthesis (AEI3 (Awareness Engine Internal State 3 — Responsive Awareness))** for “workspace ignition” when thresholds are met.
- **EAE (Emotional Awareness Engine)** as a mixture‑of‑experts (Appraisal/OCC (Ortony–Clore–Collins)+EMA (Emotion and Adaptation), Dimensional PAD (Pleasure–Arousal–Dominance (aka y'all need jesus))/VA (Pleasure–Arousal–Dominance (aka y'all need jesus) / Valence–Arousal), ACT (Affect Control Theory)/BayesACT (Bayesian Affect Control Theory), Constructionist & interoceptive active‑inference, RL/active‑inference) returning a canonical **ESV (Emotional State Vector)** (Emotional State Vector). Toggle via `emotion.enabled`.

### High‑Level Dataflow
```
Sensors/Events → IA (Input Adapters, Rust)
      ↓
AEI1 Base Awareness (Rust: PWPE monitor, low QU/EU)
      ├─ [optional] EAE.evaluate(ctx)  # Rust router → C++ kernels
      ├─ if escalate (PWPE ≥ θ1 and EVC′ > 0) → AEI2
      │         ↓
      │  AEI2 Escalated Awareness (Rust; burst QU/EU; +5% spatial halo)
      │         ├─ [optional] EAE.evaluate(ctx)  # Appraisal; ACT if social
      │         ├─ if EH.conf ≥ θ2 and EVC′ > 0 → IM (narrative options)
      │         └─ else → AEI3
      ↓
AEI3 Responsive Awareness (Rust; global synthesis; Project A transforms; C++ math optional)
      ├─ [optional] EAE.evaluate(ctx)  # full fusion; constructionist updates
      ↓
IM (Rust: inner speech + attention schema) → DEC (policy + EVC′) → TA (plans) → EA (audit)
      ↘
       MM (STM/LTM with provenance; emotion‑tagged snapshots; extended‑mind hooks)
```

EVC (Expected Value of Control)′ denotes EVC (Expected Value of Control) modulated by EAE (bounded contribution). Setting `emotion.enabled=false` bypasses EAE (Emotional Awareness Engine) and preserves core behavior.

### Project A Relationship
Project A provides mathematically derived transforms (`transformation_matrices.npy`, `proof_graph.json` or Rust/C++ equivalents) used by AEI3 (Awareness Engine Internal State 3 — Responsive Awareness) to refine contextual vectors before producing the final **Awareness Vector (AV (Awareness Vector))**. When EAE (Emotional Awareness Engine) is enabled, the AV (Awareness Vector) is stored with an accompanying **ESV (Emotional State Vector)** and appraisal metadata.

---

## 2. Language & Runtime Model

- **Rust (stable)**: AEI (Awareness Engine Internal) state machines, IM (Internal Monologue)/DEC (Decision Engine)/SA (Situation Assessment)/TA (Task Allocation/Planner), MM (Memory Management)/EA (Event Audit), budget schedulers, config, and telemetry.
- **C++20+**: EAE (Emotional Awareness Engine) sub‑model kernels and optional math/inference ops. Compiled with NEON (ARM Advanced SIMD)‑friendly flags for Pi 5 (AArch64 (64‑bit ARM architecture)).
- **FFI (Foreign Function Interface)**: `cxx::bridge` defines a small set of POD structs and functions: Rust calls `pad_evaluate`, `appraisal_evaluate`, `act_evaluate`, `construct_evaluate`, `rl_evaluate`; the Rust router performs reliability‑weighted fusion into ESV (Emotional State Vector).
- **Threading**: lock‑free ring buffers in Rust (e.g., crossbeam) for component messaging; short‑lived bursts in AEI2 (Awareness Engine Internal State 2 — Escalated Awareness); wave scheduling in AEI3 (Awareness Engine Internal State 3 — Responsive Awareness).
- **Config**: YAML under `config/aei_policies.yaml` (EAE (Emotional Awareness Engine) toggle and weights).

---


## 3. Acronyms & Key Terms (fully expanded and cross‑referenced)

- **AEI (Awareness Engine Internal)** — The three‑state core of awareness and escalation. See also **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**, **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)**, **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**. AEI (Awareness Engine Internal) computes **PWPE (Precision‑Weighted Prediction Error)** and, when warranted by **EVC (Expected Value of Control)** or **EVC (Expected Value of Control)′ (Expected Value of Control — emotion‑modulated)**, escalates to higher states. Outputs are propagated to **IM (Internal Monologue)** and fed into **DEC (Decision Engine)**; snapshots are stored by **MM (Memory Management)** and audited via **EA (Event Audit)**.

- **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** — Low‑cost ambient monitor that continuously samples inputs via **IA (Input Adapters)** and **QR (Quick Recognition)** hints, computes **PWPE (Precision‑Weighted Prediction Error)**, and prepares an **ET (Escalation Ticket)** with **TSIG (Trigger Signature)** when escalation thresholds are met. May invoke **EAE (Emotional Awareness Engine)** in its cheap **PAD (Pleasure–Arousal–Dominance (aka y'all need jesus) aka y'all too horney)** path to produce an **ESV (Emotional State Vector)** for **EVC (Expected Value of Control)′ (Expected Value of Control — emotion‑modulated)** gating.

- **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** — Investigation stage instantiated by **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**. Temporarily raises **QU (Query Units)** and **EU (Energy Units)** budgets and enables **SEB (Spatial Exploration Budget)** with a +5% positional halo to perform active sensing. Runs **Appraisal (OCC (Ortony–Clore–Collins)/EMA (Emotion and Adaptation))** and, when **IM (Internal Monologue)** or **IA (Input Adapters)** signals human interaction, also **ACT (Affect Control Theory)** via **EAE (Emotional Awareness Engine)**. Produces an event hypothesis for **IM (Internal Monologue)** or escalates to **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**.

- **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** — Global synthesis stage. Broadens temporal windows and relational context, applies **Project A** transforms to produce the **AV (Awareness Vector)**, and (optionally) performs constructionist/active‑inference updates via **EAE (Emotional Awareness Engine)**. Sends finalized results to **IM (Internal Monologue)** and **DEC (Decision Engine)** and consolidates emotion‑tagged snapshots to **MM (Memory Management)**.

- **IA (Input Adapters)** — Rust modules that normalize heterogeneous inputs (e.g., audio, vision, logs) according to `adapters.yaml` and `sensor_map.json`. Feed **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** and **QR (Quick Recognition)** with rate limits aligned to **QU (Query Units)**/**EU (Energy Units)** budgets.

- **QR (Quick Recognition)** — Cheap heuristics that inject priors (e.g., novelty or tone hints) into **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** to reduce baseline **PWPE (Precision‑Weighted Prediction Error)** and improve triage before costly escalation.

- **IM (Internal Monologue)** — Narrative generator with an **attention schema** and **self‑model** that explains “what/why/next,” receives **EH (Event Hypothesis)** or **AV (Awareness Vector)** from **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)**/**AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**, and presents options to **DEC (Decision Engine)**. When **EAE (Emotional Awareness Engine)** is enabled, **IM (Internal Monologue)** overlays the **ESV (Emotional State Vector)** to annotate justifications and social‑norm rationales.

- **MM (Memory Management)** — **STM (Short‑Term Memory)**/**LTM (Long‑Term Memory)** stores with derived "roots". Indexes **AV (Awareness Vector)** and **ESV (Emotional State Vector)** for emotion‑aware retrieval and supports the expanded temporal/context windows of **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**.

- **SA (Situation Assessment)** — Computes task value, risk, and constraints, providing **EVC (Expected Value of Control)** inputs to **DEC (Decision Engine)** and escalation logic in **AEI (Awareness Engine Internal)**. Incorporates **ACT (Affect Control Theory)**‑derived social urgency and appraisal‑based goal congruence when **EAE (Emotional Awareness Engine)** is enabled (bounded to keep **EVC (Expected Value of Control)′ (Expected Value of Control — emotion‑modulated)** stable).

- **TA (Task Allocation/Planner)** — Turns **DEC (Decision Engine)** choices into executable plans respecting **AEI (Awareness Engine Internal)** budgets and optional emotion‑regulation actions requested by **IM (Internal Monologue)** when **EAE (Emotional Awareness Engine)** signals extreme **ESV (Emotional State Vector)** values.

- **EA (Event Audit)** — Structured, append‑only logging of hypotheses, narratives, **PWPE (Precision‑Weighted Prediction Error)**, **EVC (Expected Value of Control)/EVC (Expected Value of Control)′ (Expected Value of Control / Expected Value of Control — emotion‑modulated)** inputs, budgets, and **ESV (Emotional State Vector)** traces with privacy‑preserving redaction profiles.

- **DEC (Decision Engine)** — Applies policy using **EVC (Expected Value of Control)/EVC (Expected Value of Control)′ (Expected Value of Control / Expected Value of Control — emotion‑modulated)**; authorizes budgets and escalations; coordinates with **TA (Task Allocation/Planner)**. Caps emotion influence to prevent runaway behavior.

- **PWPE (Precision‑Weighted Prediction Error)** — Primary anomaly trigger derived in **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**. Drives **EVC (Expected Value of Control)** and the escalation rule (`PWPE ≥ θ1 && EVC′ > 0`).

- **EVC (Expected Value of Control)** / **EVC (Expected Value of Control)′ (Expected Value of Control — emotion‑modulated)** — Cost‑benefit calculus balancing expected error reduction × task value against **QU (Query Units)**/**EU (Energy Units)**/latency. **EVC (Expected Value of Control)′ (Expected Value of Control — emotion‑modulated)** adds bounded contributions from **ESV (Emotional State Vector)** (arousal/social urgency/goal congruence). "What in the tiny baby jesus was that noise!?" == Event Reaction will correspond to high value emotional state vectors that heavily influence the likelihood of escalation to 

- **QU (Query Units)** and **EU (Energy Units)** — Abstract resource budgets governing I/O, retrieval, and compute. Raised in **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** bursts and wave‑scheduled in **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**. Energy resource monitoring based on AEI state.

- **SEB (Spatial Exploration Budget)** — Positional sampling halo (+5%) permitted only in **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** for active sensing around a focal item. "Head tilting".

- **EAE (Emotional Awareness Engine)** — Mixture‑of‑experts router (Rust) that calls C++ kernels for **PAD (Pleasure–Arousal–Dominance (aka y'all need jesus))**, **Appraisal (OCC (Ortony–Clore–Collins)/EMA (Emotion and Adaptation))**, **ACT (Affect Control Theory/BayesACT (Bayesian Affect Control Theory))**, **Constructionist/Interoceptive Active‑Inference**, and **RL/Active‑Inference**, returning a canonical **ESV (Emotional State Vector)**. Toggled by `emotion.enabled`.

- **ESV (Emotional State Vector)** — Canonical emotional state fused from **EAE (Emotional Awareness Engine)** sub‑models and consumed by **AEI (Awareness Engine Internal)**, **IM (Internal Monologue)**, **DEC (Decision Engine)**, and **MM (Memory Management)**.

- **ACT (Affect Control Theory)** / **BayesACT (Bayesian Affect Control Theory)** — Social meaning and deflection models invoked by **EAE (Emotional Awareness Engine)** during human interaction. Their outputs contribute to **ESV (Emotional State Vector)** (social urgency) and **SA (Situation Assessment)** (norm coherence).

- **OCC (Ortony–Clore–Collins)** / **EMA (Emotion and Adaptation)** — Appraisal theories used in **EAE (Emotional Awareness Engine)** to derive discrete appraisals and **goal_congruence** values, mapped to **PAD (Pleasure–Arousal–Dominance (aka y'all need jesus))** and ultimately into the **ESV (Emotional State Vector)**.

- **PAD (Pleasure–Arousal–Dominance (aka y'all need jesus))** / **VA (Valence–Arousal)** — Dimensional emotion spaces used as the lingua franca for **EAE (Emotional Awareness Engine)** fusion and smoothing. Cheap baseline path for **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** when budgets are tight.

- **AV (Awareness Vector)** — Final, globally integrated representation produced by **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** (with **Project A** transforms). Stored by **MM (Memory Management)** with optional **ESV (Emotional State Vector)** tags and narrated by **IM (Internal Monologue)**.

- **CV (Contextual Vector)** — Intermediate enriched representation built by **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** and refined by **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** on the path to the **AV (Awareness Vector)**.

- **ET (Escalation Ticket)** — Handoff capsule carrying **TSIG (Trigger Signature)**, budgets, partial metrics, snapshots, and optional **ESV (Emotional State Vector)** from **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** to **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)**/**AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**.

- **TSIG (Trigger Signature)** — Compact fingerprint of the stimulus that caused escalation, emitted by **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** and propagated in **ET (Escalation Ticket)**.

- **PFB (Pre‑Feature Buffer)** — Normalized, low‑cost features produced by **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** before context enrichment.

- **STM (Short‑Term Memory)** / **LTM (Long‑Term Memory)** — Subsystems of **MM (Memory Management)** for fast context and durable storage with provenance.

- **FFI (Foreign Function Interface)** — Safe boundary (`cxx::bridge`) used by Rust to call **C++ kernels** implementing **EAE (Emotional Awareness Engine)** sub‑models and optional math routines.

- **SIMD (Single Instruction, Multiple Data)** / **NEON (ARM Advanced SIMD (Single Instruction, Multiple Data))** / **AArch64 (64‑bit ARM architecture)** — Hardware/vectorization context relevant to C++ kernels and to performance on Raspberry Pi 5–class devices.
## 4. Detailed Structure & Dataflows

### Repository Layout (Rust-first, C++ kernels)
```
project_b_rust/
├── Cargo.toml
├── build.rs
├── .cargo/config.toml
├── config/
│   ├── aei_policies.yaml
│   ├── context_rules.yaml
│   ├── adapters.yaml
│   └── sensor_map.json
├── data/project_a/
│   ├── transformation_matrices.npy        # placeholder; or Rust serde formats
│   └── proof_graph.json
├── src/
│   ├── lib.rs
│   ├── types.rs
│   ├── aei/{aei1.rs, aei2.rs, aei3.rs, mod.rs}
│   ├── emotion/{router.rs, ffi.rs, mod.rs}
│   ├── im/{mod.rs}
│   ├── mm/{mod.rs}
│   ├── sa/{mod.rs}
│   ├── ta/{mod.rs}
│   ├── dec/{mod.rs}
│   ├── ea/{mod.rs}
│   └── bin/demo.rs
└── src/cpp/
    ├── include/eae.h
    ├── eae_pad.cpp
    ├── eae_appraisal.cpp
    ├── eae_act.cpp
    ├── eae_construct.cpp
    └── eae_rl.cpp
```

### AEI (Awareness Engine Internal) — Operational States & Escalation (Rust)

- **AEI1 (Base)**: low‑cost ambient monitoring; computes **PWPE (Precision‑Weighted Prediction Error)**; optionally calls EAE (dimensional PAD (Pleasure–Arousal–Dominance (aka y'all need jesus)) baseline). Emits **PFB (Pre‑Feature Buffer) + TSIG (+ ESV (Emotional State Vector))**.
- **AEI2 (Escalated)**: uses +5% **SEB (Spatial Exploration Budget)** and burst budgets; optionally runs Appraisal and ACT (Affect Control Theory); hands EH to IM (Internal Monologue) if confident, else to AEI3 (Awareness Engine Internal State 3 — Responsive Awareness).
- **AEI3 (Responsive)**: expands temporal/context windows; applies Project A transforms; optional constructionist updates; produces **AV (Awareness Vector)** (+ ESV (Emotional State Vector)) and de‑escalates.

Escalation rule (Rust): `PWPE ≥ θ1 && EVC′ > 0`, with EVC (Expected Value of Control)′ = EVC (Expected Value of Control) ± bounded emotional modulation.

### EAE (Emotional Awareness Engine) — Emotional Awareness Engine (Rust router → C++ kernels)

- Models: **Dimensional (PAD (Pleasure–Arousal–Dominance (aka y'all need jesus)))**, **Appraisal (EMA (Emotion and Adaptation)/OCC (Ortony–Clore–Collins))**, **ACT (Affect Control Theory)/BayesACT (Bayesian Affect Control Theory)**, **Constructionist/Interoceptive Active‑Inference**, **RL/Active‑Inference**.
- Router policy:
  - AEI1 (cheap): run PAD (Pleasure–Arousal–Dominance (aka y'all need jesus)) baseline (+ light appraisal).
  - AEI2 (Awareness Engine Internal State 2 — Escalated Awareness): Appraisal primary; ACT (Affect Control Theory) when social; PAD (Pleasure–Arousal–Dominance (aka y'all need jesus)) smoothing.
  - AEI3 (Awareness Engine Internal State 3 — Responsive Awareness): Fuse all; run constructionist/RL updates.
- Fusion: reliability‑weighted average in PAD (Pleasure–Arousal–Dominance (aka y'all need jesus)) space; export **ESV (Emotional State Vector)** with `{valence, arousal, dominance?, goal_congruence, social_urgency}`.

---

## 5. Configuration (YAML)

```yaml
aei:
  thresholds: { theta1_pwpe: 0.70, theta2_conf: 0.85, theta3_ignite: 0.90 }
  budgets:
    aei1: { QU: 1.0,  EU: 1.0,  spatial_pct: 0.00 }
    aei2: { QU: 10.0, EU: 8.0,  spatial_pct: 0.05, burst_seconds: 5 }
    aei3: { QU: 25.0, EU: 15.0, spatial_pct: 0.00, wave_seconds: 10 }
  control:
    evc: { alpha: 1.0, beta: 1.0, gamma: 0.1, value_model: "task_value * expected_error_reduction" }
    adaptive_gain: { target_zone: inverted_U_mid, feedback_vars: [latency_ms,error_rate,pupil_proxy] }

emotion:
  enabled: true
  router:
    use_pad_as_canonical: true
    weights: { appraisal: 0.35, act: 0.35, dimensional: 0.20, constructed: 0.05, rl_activeinf: 0.05 }
    switch:
      human_interaction_bias: 0.25
      low_budget_bias_dimensional: 0.30
  pad_decay: 0.10
  evc_modulation: { k_arousal: 0.10, k_social: 0.15, k_goal: 0.10, cap: 0.25 }
```

Disable EAE (Emotional Awareness Engine) by setting `emotion.enabled: false` (no EAE (Emotional Awareness Engine) calls, EVC (Expected Value of Control)′=EVC (Expected Value of Control)).

---

## 6. APIs & FFI (Foreign Function Interface) Boundaries

### Rust Types
- `ESV { valence: f64, arousal: f64, dominance: f64, goal_congruence: f64, social_urgency: f64 }`
- `PadOut` (FFI (Foreign Function Interface) mirror of ESV (Emotional State Vector) + reliability).

### FFI (via `cxx::bridge`, namespace `eae`)
- `pad_evaluate() -> PadOut`
- `appraisal_evaluate() -> PadOut`
- `act_evaluate(human: bool) -> PadOut`
- `construct_evaluate() -> PadOut`
- `rl_evaluate() -> PadOut`

Rust router selects models, calls these functions, fuses outputs into ESV (Emotional State Vector).

---

## 7. Build & Deployment (Pi‑friendly)

- **Rust:** `cargo build --release`
- **C++ flags:** set by `build.rs` (`-O3 -DNDEBUG -mcpu=cortex-a76 -mtune=cortex-a76 -ffast-math -flto` when available).
- **Rust flags:** `.cargo/config.toml` sets `target-cpu=cortex-a76`, `opt-level=3`, `lto=thin` for release.
- **Run demo:** `cargo run --release --bin demo` — prints PWPE (Precision‑Weighted Prediction Error), EVC (Expected Value of Control), EVC (Expected Value of Control)′, and ESV (Emotional State Vector) if enabled.
- **Service:** deploy as a systemd unit binding CPU affinity for AEI (Awareness Engine Internal) vs kernels if needed.

---

## 8. Acceptance Metrics, Security/Privacy, Interfaces

Same metrics as the previous edition (trigger quality, economy, resolution, narrative quality, emotion fit, memory utility, stability) with the addition that the **EAE (Emotional Awareness Engine) toggle** enables A/B benchmarking. Emotion data has redaction profiles in logs (EA (Event Audit)) and bounded influence on EVC (Expected Value of Control)′.
