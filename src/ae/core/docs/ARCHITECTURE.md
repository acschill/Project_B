# AE-Core (with AEI1/2/3) — Architecture Overview

AE-Core normalizes inputs and hosts AEI1–3.
- AEI1: ingest & preprocess
- AEI2: context enrichment via MM-ST
- AEI3: fusion + Project A transforms

This scaffold targets Raspberry Pi 5 (64-bit, Linux), with Rust orchestrator and C++ low-latency primitives. It includes:

- Contracts (messages)
- Orchestrator daemon
- Native primitives
- Replay/bench tools (stubs)
- Docs
