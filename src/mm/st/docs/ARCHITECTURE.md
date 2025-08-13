# MM-ST (Short-Term Memory) — Architecture Overview

Hot KV for recent context; ring buffers; delta compression; WAL; flush hints to MM-LT.

This scaffold targets Raspberry Pi 5 (64-bit, Linux), with Rust orchestrator and C++ low-latency primitives. It includes:

- Contracts (messages)
- Orchestrator daemon
- Native primitives
- Replay/bench tools (stubs)
- Docs
