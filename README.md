# Project B
# (Name to be updated, 'B' literally only meant the letter after 'A', referring to the first & other Program A)

## General Overview
Project B is an independent agent architecture designed for low-cost devices (e.g., Raspberry Pi 5). The control plane is implemented in **Rust** (state machines, scheduling, budgets, messaging), while performance‑critical emotional inference and math kernels are implemented in **C++** and exposed to Rust via a safe FFI (Foreign Function Interface) boundary using the `cxx` crate. The system maintains a low‑overhead **base awareness**, escalates effort on **precision‑weighted prediction errors (PWPE (Precision‑Weighted Prediction Error))** when warranted by **Expected Value of Control (EVC (Expected Value of Control))**, globally integrates context when ignition criteria are met, and optionally models **emotional awareness** with a multi‑model **Emotional Awareness Engine (EAE (Emotional Awareness Engine))** that can be disabled via config.

### Design Tenets
- **Rust control plane** for memory safety, predictable latency, and trait‑based “OO” composition.
- **C++ kernels** for SIMD (Single Instruction, Multiple Data)‑friendly math and library access (NEON (ARM Advanced SIMD), Eigen/ONNX Runtime/TFLite if needed).
- **PWPE (Precision‑Weighted Prediction Error)→EVC (Expected Value of Control) gating** for cost‑aware escalation; **adaptive gain** for arousal/scheduling (inverted‑U).
- **Global Synthesis (AEI3 (Awareness Engine Internal State 3 — Responsive Awareness))** for “workspace ignition” when thresholds are met.
- **EAE (Emotional Awareness Engine)** as a mixture‑of‑experts (Appraisal/OCC (Ortony–Clore–Collins)+EMA (Emotion and Adaptation), Dimensional PAD (aka y'all need jesus))/VA, ACT (Affect Control Theory)/BayesACT (Bayesian Affect Control Theory), Constructionist & interoceptive active‑inference, RL/active‑inference) returning a canonical **ESV (Emotional State Vector)** (Emotional State Vector). Toggle via `emotion.enabled`.

## Build
```bash
cargo clean
cargo check
cargo build --release
```
> The build script compiles C++ kernels with NEON-friendly flags when supported.

## Run Demo
```bash
cargo run --release --bin project_b
```
Outputs PWPE, EVC, EVC′, and ESV (if emotion enabled).

## If you’ll build on your dev machine and deploy to Pi (cross-compile):
`rustup target add aarch64-unknown-linux-gnu`
# Install a cross linker/toolchain on your dev box:
# Ubuntu/Debian:
`sudo apt-get install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu`
# macOS (Homebrew): 
`brew install aarch64-linux-gnu-binutils aarch64-linux-gnu-gcc`

## If you build directly on the RPI5 (64-bit OS)
`curl https://sh.rustup.rs -sSf | sh`
`rustup default stable`
`rustup component add rustfmt clippy`

## Toggle Emotion
Edit `config/aei_policies.yaml` → `emotion.enabled: false` to disable the EAE.

## Where to Start
- Rust control plane: `src/`
- C++ kernels: `src/cpp/` (exposed via `cxx` bridge)
- Spec: `docs/Project_B_Technical_Overview.md`

# Acronyms & Key Terms

- **AEI (Awareness Engine Internal)** — Three-state awareness core orchestrating detection, escalation, and synthesis; see **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**, **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)**, **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**. Feeds **IM (Internal Monologue)** and **DEC (Decision Engine)**; logs via **EA (Event Audit)**.

- **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** — Low-cost ambient monitor using **IA (Input Adapters)** and **QR (Quick Recognition)**; computes **PWPE (Precision-Weighted Prediction Error)**; opens **ET (Escalation Ticket)** with **TSIG (Trigger Signature)**; may call **EAE (Emotional Awareness Engine)** for a cheap **PAD (aka y'all need jesus)** pass.

- **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** — Investigation burst with raised **QU (Query Units)**/**EU (Energy Units)** and **SEB (Spatial Exploration Budget)** +5% halo; builds **EH (Event Hypothesis)** for **IM (Internal Monologue)** or escalates to **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**; can use **ACT (Affect Control Theory)** via **EAE (Emotional Awareness Engine)** when social cues appear.

- **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** — Global synthesis with wider temporal/context windows and **Project A (Project A Transform Library)** transforms; outputs **AV (Awareness Vector)** for **IM (Internal Monologue)**/**DEC (Decision Engine)**; writes to **MM (Memory Management)**.

- **IA (Input Adapters)** — Normalizes sensors/streams per config and rate-limits to **QU (Query Units)**/**EU (Energy Units)** budgets; sources frames for **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** and **QR (Quick Recognition)**.

- **QR (Quick Recognition)** — Cheap priors (e.g., novelty/tone hints) injected into **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** to reduce **PWPE (Precision-Weighted Prediction Error)**.

- **IM (Internal Monologue)** — Generates “what/why/next” narratives from **EH (Event Hypothesis)**/**AV (Awareness Vector)**; maintains attention schema and self-model; offers options to **DEC (Decision Engine)**; overlays **ESV (Emotional State Vector)** when **EAE (Emotional Awareness Engine)** is enabled.

- **MM (Memory Management)** — **STM (Short-Term Memory)**/**LTM (Long-Term Memory)** with provenance; stores **AV (Awareness Vector)** and **ESV (Emotional State Vector)** for future retrieval and learning.

- **SA (Situation Assessment)** — Computes task value/risk/constraints feeding **EVC (Expected Value of Control)**/**EVC′ (Expected Value of Control — emotion-modulated)**; can include social urgency from **ACT (Affect Control Theory)** and goal-congruence from **OCC (Ortony–Clore–Collins)**/**EMA (Emotion and Adaptation)**.

- **DEC (Decision Engine)** — Chooses actions using **EVC (Expected Value of Control)**/**EVC′ (Expected Value of Control — emotion-modulated)**; authorizes budgets/escalations; coordinates with **TA (Task Allocation/Planner)**; caps emotion influence.

- **TA (Task Allocation/Planner)** — Compiles **DEC (Decision Engine)** choices into executable plans respecting **AEI (Awareness Engine Internal)** budgets; may include emotion-regulation steps requested by **IM (Internal Monologue)**.

- **EA (Event Audit)** — Structured, append-only logs of hypotheses, narratives, **PWPE (Precision-Weighted Prediction Error)**, **EVC/EVC′ (Expected Value of Control / Expected Value of Control — emotion-modulated)** inputs, budgets, and **ESV (Emotional State Vector)** traces; supports redaction.

- **PWPE (Precision-Weighted Prediction Error)** — Primary anomaly signal in **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**; drives the escalation rule `PWPE ≥ θ1 && EVC′ > 0`.

- **EVC (Expected Value of Control)** / **EVC′ (Expected Value of Control — emotion-modulated)** — Benefit–cost calculus (expected error reduction × task value minus resource/latency costs); **EVC′ (Expected Value of Control — emotion-modulated)** adds bounded **ESV (Emotional State Vector)** effects (arousal/social urgency/goal congruence).

- **QU (Query Units)** / **EU (Energy Units)** — Abstract resource budgets for I/O, retrieval, and compute; burst-raised in **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** and wave-scheduled in **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)**.

- **SEB (Spatial Exploration Budget)** — +5% positional halo around focal item for active sensing; enabled only in **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)**.

- **EAE (Emotional Awareness Engine)** — Mixture-of-experts router calling C++ kernels for **PAD (aka y'all need jesus)**, **Appraisal (OCC/EMA)**, **ACT (Affect Control Theory/BayesACT)**, **Constructionist/Interoceptive Active-Inference**, and **RL/Active-Inference**; returns canonical **ESV (Emotional State Vector)**; toggled via config.

- **ESV (Emotional State Vector)** — Canonical emotional state fused from **EAE (Emotional Awareness Engine)** sub‑models and consumed by **AEI (Awareness Engine Internal)**, **IM (Internal Monologue)**, **DEC (Decision Engine)**, and **MM (Memory Management)**.

- **ACT (Affect Control Theory)** / **BayesACT (Bayesian Affect Control Theory)** — Social meaning/deflection models invoked by **EAE (Emotional Awareness Engine)** during human interaction; contribute to **ESV (Emotional State Vector)** and **SA (Situation Assessment)**.

- **OCC (Ortony–Clore–Collins)** / **EMA (Emotion and Adaptation)** — Appraisal theories used by **EAE (Emotional Awareness Engine)** to derive discrete appraisals and **goal_congruence**, mapped to **PAD (aka y'all need jesus)**.

- **PAD (aka y'all need jesus)** / **VA (Valence–Arousal)** — Dimensional emotion spaces; lingua franca for **EAE (Emotional Awareness Engine)** fusion and smoothing; cheap baseline in **AEI1 (Awareness Engine Internal State 1 — Base Awareness)**.

- **AV (Awareness Vector)** — Final, globally integrated representation produced by **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** (with **Project A (Project A Transform Library)** transforms); narrated by **IM (Internal Monologue)**; stored in **MM (Memory Management)**.

- **CV (Contextual Vector)** — Intermediate enriched representation from **AEI2 (Awareness Engine Internal State 2 — Escalated Awareness)** refined by **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** on the path to **AV (Awareness Vector)**.

- **ET (Escalation Ticket)** — Handoff capsule with **TSIG (Trigger Signature)**, budgets, partial metrics, snapshots, and optional **ESV (Emotional State Vector)**; passed from **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** to **AEI2/AEI3 (Awareness Engine Internal States 2/3 — Escalated/Responsive Awareness)**.

- **TSIG (Trigger Signature)** — Compact fingerprint of the stimulus that caused escalation; emitted by **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** and carried in **ET (Escalation Ticket)**.

- **PFB (Pre-Feature Buffer)** — Normalized, low-cost features produced by **AEI1 (Awareness Engine Internal State 1 — Base Awareness)** prior to context enrichment.

- **STM (Short-Term Memory)** / **LTM (Long-Term Memory)** — Subsystems of **MM (Memory Management)** for fast context and durable storage with provenance.

- **FFI (Foreign Function Interface)** — Safe Rust↔C++ boundary (`cxx::bridge`) used to call **EAE (Emotional Awareness Engine)** kernels and optional math routines.

- **SIMD (Single Instruction, Multiple Data)** / **NEON (ARM Advanced SIMD)** / **AArch64 (64-bit ARM architecture)** — Hardware/vectorization context for performance on Raspberry Pi 5–class devices; relevant to C++ kernels and Rust `std::simd`.

- **Project A (Project A Transform Library)** — External artifact set providing topological/relational transforms used by **AEI3 (Awareness Engine Internal State 3 — Responsive Awareness)** during global synthesis.

