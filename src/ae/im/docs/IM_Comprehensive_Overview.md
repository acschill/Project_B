
# Project B — Internal Monologue (IM) Comprehensive Architecture

**Role of IM**  
IM is the “conscious” core of Project B. It owns a continuously updated *narrative state* that fuses sensory/AEI outputs, goals, and memory to produce predictions and evaluations. Other subsystems primarily **feed** IM or are **invoked** by IM.

---

## 1. Design Goals & Guarantees

- **Always-on awareness at low cost**: cheap heartbeat with adaptive escalation.
- **Deterministic recovery**: event-sourced state + frequent snapshots for zero-loss restarts.
- **Adaptive workload**: salience-driven mode switching (Idle → Observe → Focus → React) with hysteresis.
- **Tight invariants**: health checks on every heartbeat; deeper diagnostics opportunistically.
- **Strict contracts**: schema-versioned messages, monotonic time, checksums.
- **Observability**: structured logs, metrics, traceable decisions, WAL/snapshot lineage.

---

## 2. Operating Modes & Polling Cadence (minimum refresh if push is quiet)

| Mode   | Hold | IM Tick | AEI1 | AEI2 | AEI3 | MM‑ST | MM‑LT | DEC | EA Log |
|--------|------|---------|------|------|------|-------|-------|-----|--------|
| Idle (S<0.2) | ≥5s | **1 Hz** (1000 ms) | 1 Hz | 0.2 Hz | on‑demand | 1 Hz | 0.1 Hz (10 s) | 0.5 Hz | batch/5 s |
| Observe (0.2–0.6) | ≥3s | **10 Hz** (100 ms) | 10 Hz | 5 Hz | 2 Hz | 10 Hz | /3 s | 2 Hz | /2 s |
| Focus (0.6–0.85) | ≥2s | **50 Hz** (20 ms) | 50 Hz | 20 Hz | 10 Hz | 50 Hz | /1 s | 10 Hz | /1 s |
| React (≥0.85) | ≤0.5s | **200 Hz** (5 ms bursts) | event‑driven + cache | event‑driven + cache | event‑driven + cache | ring‑buffered deltas | flush deferred | fast path | end‑of‑burst |

**Salience S** (recomputed every heartbeat): `S = w1·novelty + w2·uncertainty + w3·risk + w4·deadline_pressure`.  
**Hysteresis thresholds**: Idle→Observe `S↑≥0.25` (back at `S↓≤0.15`); Observe→Focus `S↑≥0.65` (back at `S↓≤0.45`); Focus→React `S↑≥0.90` (back at `S↓≤0.75`).

---

## 3. Runtime Model & Scheduler

- **Core loop**: single authority controller (Rust) orchestrating async tasks via priority queues.
- **Three lanes**: `P0` safety/react, `P1` focus/observe, `P2` idle/maintenance.
- **Within-lane policy**: earliest‑deadline‑first (EDF).
- **Budgets** (per tick window): Idle ≤1 ms/1000 ms; Observe ≤5 ms/100 ms; Focus ≤5 ms/20 ms; React ≤2 ms/5 ms (with spillover to next window).  
- **Backpressure**: shed P2 first, then downshift fidelities, never drop safety checks.
- **Time**: monotonic clock for all scheduling; wall clock retained only in payload metadata.

Recommended stacks: **Rust** (`tokio`, `crossbeam`, `parking_lot`), **C++** (Boost.Asio or Folly, `absl`/`fmt`).

---

## 4. Data Model (Narrative State)

**WorkingContext** *(RAM, backed by MM‑ST)*
- `timeline`: ring buffer (10–60 s) of `Event{ts_mono, wall_ts, src, schema_ver, seq_no, payload_ptr, payload_crc}`
- `scene`: fused snapshot (entities, relations, numeric features, hash)
- `hypotheses`: set of predictions `{horizon, score, rationale_ptr}`
- `goals`: prioritized stack with deadlines
- `confidence`: per‑facet uncertainty scalars
- `mode`: Idle/Observe/Focus/React + salience

**Storage pointers, not blobs**: blobs live in append‑only chunk files or KV store; indexes keep it O(1) to fetch recent context.

---

## 5. Persistence & Recovery

- **Event‑sourced**: append‑only **WAL** for all IM events and decisions.
- **Snapshots**: periodic, CRC‑checked “IM snapshot” with WorkingContext (without blobs), stream heads, RNG seeds, schema versions.
- **Fsync cadence**: Idle 1 s; Observe 250 ms; Focus/React 50 ms.
- **Snapshot cadence**: Idle 20 s; Observe 5 s; Focus 5 s; React end‑of‑burst.
- **Stores**: SQLite (WAL mode) or RocksDB; blobs are memory‑mapped chunk files.  
- **Cold start**: load latest snapshot (tens of ms), replay WAL tail, reopen ring buffers, start Idle tick immediately.  
- **Determinism**: version pins + RNG seed persisted enable reproducible replays.

---

## 6. Error Detection & Health

- **Heartbeats** (each IM tick): jitter check, queue depth, stale sources, seq gaps, CRC verification of recent WAL segment.
- **Cross‑sensor checks**: disagreement > kσ lowers confidence and marks partial.
- **Prediction monitors**: running MAE/Drift alarms.
- **Backpressure monitors**: latency histograms gating AEI3/DEC work.
- **Watchdogs**: crash loops trigger supervised restart with reason captured in WAL.

---

## 7. Interfaces (Schema‑Versioned Contracts)

Inbound → IM:
- `AEI1Update`, `AEI2Update`, `AEI3Update`, `SAEvent`, `MMQueryResult`, `TAPlanUpdate`, `DECOutcome`

Outbound ← IM:
- `IMDecision`, `IMRequest{component, query, deadline}`, `IMModeChange`, `IMSnapshotHint`, `IMHealth`

All messages: `(seq_no, ts_mono, schema_ver, source_id, checksum)`; serialized with Protobuf or FlatBuffers; versioned loaders.

---

## 8. Constant Base Awareness (IM‑Lite)

Runs every heartbeat, sub‑millisecond cost:
- Update `salience`, run invariants, emit minimal `IMHealth`, persist compressed deltas to MM‑ST, keep **scene hash** hot.  
- Heavy work (embeddings, long‑horizon predictions) gated to Observe/Focus or explicit triggers.

---

## 9. Important Files & Primary Functions

> Dual‑language plan: **Rust** orchestrates the controller, scheduling, persistence, and contracts. **C++** hosts ultra‑low‑latency primitives (ring buffers, CRC/codec, timers) used via FFI.

### 9.1 Rust crate: `im_core`

- `src/lib.rs`  
  - Exports crate API, version stamps, feature flags.

- `src/controller.rs`  
  - `struct ImController` — owns queues, budgets, mode machine.  
  - `fn run(&mut self)` — main loop (tokio task set).  
  - `fn tick(&mut self, now: MonoInstant)` — heartbeat; IM‑Lite, salience, scheduling.  
  - `fn on_event(&mut self, ev: Inbound)` — updates WorkingContext; may change mode.  
  - `fn persist(&mut self)` — WAL batching, snapshot triggers.  
  - `fn apply_backpressure(&mut self)` — degrade noncritical tasks.  
  - `fn health_checks(&mut self)` — jitter, staleness, seq gaps, CRC.

- `src/analysis.rs`  
  - `fn update_scene(ctx: &mut WorkingContext, events: &[Event]) -> SceneDelta`  
  - `fn estimate_salience(ctx: &WorkingContext) -> f32`  
  - `fn predict(ctx: &WorkingContext, horizon_ms: u64) -> Vec<Hypothesis>`  
  - `fn evaluate(ctx: &WorkingContext, ev: &Event) -> Vec<Candidate>`  
  - `fn select_action(cands: &[Candidate]) -> IMDecision`  
  - `fn compute_uncertainty(ctx: &WorkingContext) -> Uncertainty`

- `src/model.rs`  
  - Types: `WorkingContext, Event, Scene, Hypothesis, Candidate, Goal, Uncertainty, Mode`  
  - Ring buffer headers (Rust view of the C++ ring).

- `src/scheduler.rs`  
  - `struct PriQueue` (P0/P1/P2); EDF heap per lane.  
  - `fn schedule(&mut self, job: Job)`; `fn poll_ready(&mut self, budget: Budget) -> Vec<Job>`

- `src/storage/mod.rs`  
  - `WAL`, `Snapshot`, `Kv` traits.
- `src/storage/wal.rs`  
  - `fn append(batch: &[WalRec]) -> WalSegId` ; `fn fsync_deadline(deadline: MonoInstant)`
- `src/storage/snapshot.rs`  
  - `fn write_snapshot(ctx: &WorkingContext) -> SnapId` ; `fn load_latest() -> (WorkingContext, WalPos)`
- `src/storage/kv.rs`  
  - `fn put_blob(ptr: BlobPtr, data: &[u8])` ; `fn get_blob(ptr: BlobPtr) -> Mmap`

- `src/interfaces/messages.rs`  
  - Protobuf wrapper types and version adapters.

- `src/health.rs`  
  - Invariant library (jitter, seq gaps, staleness, drift).

- `src/config.rs`  
  - Mode thresholds, budgets, cadence, store paths.

- `src/logging.rs`  
  - Structured logging + metrics emitters.

- `src/ffi/cxxbridge.rs`  
  - Safe wrappers to C++ ring buffer, CRC, and timers.

### 9.2 C++ library: `im_native`

- `include/im_native/fast_ring_buffer.hpp` / `src/fast_ring_buffer.cpp`  
  - Lock‑free MPMC ring with single‑cacheline headers; monotonic timestamps.

- `include/im_native/crc32c.hpp` / `src/crc32c.cpp`  
  - Hardware‑accelerated CRC32C (SSE4.2/ARMv8 fallbacks).

- `include/im_native/event_codec.hpp` / `src/event_codec.cpp`  
  - Varint codecs; Protobuf/FlatBuffers marshalling helpers.

- `include/im_native/time_monotonic.hpp` / `src/time_monotonic.cpp`  
  - Steady clock helpers, jitter measurement.

- `include/im_native/prio_queue.hpp` / `src/prio_queue.cpp`  
  - EDF heap with 3‑lane priorities.

- `include/im_native/watchdog.hpp` / `src/watchdog.cpp`  
  - Thread watchdog; crash reason capture hooks.

- `include/im_native/metrics_exporter.hpp` / `src/metrics_exporter.cpp`  
  - Prometheus/OpenMetrics exporters.

---

## 10. Observability & Audit

- **Events**: every decision stamped with inputs, mode, salience, uncertainty, RNG seed.
- **Metrics**: tick jitter, queue depths, fsync latency, snapshot size/time, predict MAE.
- **Tracing**: spans for React bursts and Focus windows; decision timelines.
- **Replay tools**: deterministic “black box recorder” to reproduce any decision path.

---

## 11. Security & Integrity

- Signed snapshots (Ed25519) and WAL segments (MAC) if threat model requires.
- Schema migrations are additive; readers tolerate forward fields.
- All external inputs validated and version‑checked before entering WorkingContext.

---

## 12. Goal‑Oriented Task List (Single Developer, Sequential)

1. Initialize repositories and CI
1a. Create `im_core` (Rust) and `im_native` (C++) repos; set license, codeowners, rust‑toolchain, clang‑format.  
1b. Wire CI (lint, fmt, build, unit tests) and sanitizer jobs (asan/tsan for C++; miri for Rust).  
1c. Add pre‑commit hooks (format, lint, simple schema checks).

2. Define message schemas and versioning
2a. Author Protobuf/FlatBuffers for all inbound/outbound contracts with `(seq_no, ts_mono, schema_ver, source_id, checksum)`.  
2b. Generate Rust/C++ bindings; add golden test vectors.  
2c. Implement version adapters in `interfaces/messages.rs`.

3. Build native primitives (C++)
3a. Implement `fast_ring_buffer` with tests for wraparound, MPMC contention.  
3b. Implement `crc32c`, `time_monotonic`, and `prio_queue` (EDF) with microbenchmarks.  
3c. Provide a stable C ABI (or `cxx::bridge`) for Rust FFI; fuzz the codecs.

4. Storage layer (Rust)
4a. Implement `WAL` with batched appends and fsync deadlines; include CRC and segment indexes.  
4b. Implement `Snapshot` writer/loader with schema version stamps; CRC32C; mmap blobs.  
4c. Implement `Kv` blob store (chunked, append‑only) with compaction; fuzz file headers.

5. Core data model
5a. Define `WorkingContext`, `Event`, `Scene`, `Hypothesis`, `Goal`, `Uncertainty`, `Mode` in `model.rs`.  
5b. Implement ring‑buffer views that map onto C++ storage; include property tests.  
5c. Add scene hashing and equality for cheap change detection.

6. Scheduler and controller shell
6a. Implement `PriQueue` (3 lanes) + EDF within lanes; budget accounting.  
6b. Implement `ImController::run`, `tick`, `apply_backpressure`, and basic mode machine.  
6c. Wire monotonic clock and jitter measurement; export metrics.

7. IM‑Lite heartbeat
7a. Implement minimal path: invariants, salience estimate, scene hash, IMHealth emit.  
7b. Ensure ≤ 0.5 ms/tick cost in Idle on target hardware (profiling baseline).  
7c. Persist compressed deltas to MM‑ST on a 1 Hz cadence.

8. Salience & mode switching
8a. Implement `estimate_salience` with weights and hysteresis thresholds.  
8b. Add hold timers and state transition logs; unit tests for flapping prevention.  
8c. Gate AEI/DEC/TA work by mode and budgets.

9. Analysis functions
9a. Implement `update_scene` (fusion of recent events → scene delta).  
9b. Implement `predict` (short‑horizon hypotheses) and `evaluate` (candidates).  
9c. Implement `select_action` and emit `IMDecision` with rationale pointers.

10. Persistence integration
10a. Wire `persist` batching for WAL with mode‑dependent fsync deadlines.  
10b. Implement snapshot cadence and triggers (mode boundaries or time).  
10c. Implement cold‑start path: load snapshot, replay WAL tail, resume Idle tick.

11. Health and error handling
11a. Implement invariants: jitter, staleness, seq gaps, CRC mismatches, drift.  
11b. Implement `watchdog` hooks and supervised restart logic.  
11c. Add backpressure rules: shed P2 tasks, widen flush intervals, suspend AEI3 if needed.

12. Observability
12a. Structured logging with context IDs and decision traces.  
12b. Metrics exporters; dashboards for tick jitter, queue depth, fsync latency.  
12c. Replay tool to reproduce decisions from WAL + snapshot.

13. Integration with AEI/MM/DEC/TA
13a. Implement inbound adapters for AEI updates and SA events; outbound requests to MM/DEC/TA.  
13b. Enforce per‑source staleness thresholds; cache latest good values.  
13c. Validate schemas and back‑compat via integration tests with recorded fixtures.

14. React burst path
14a. Implement high‑priority fast path (≤ 0.5 s) with cached reads only.  
14b. Prove budgets under stress; add end‑of‑burst snapshot.  
14c. Ensure rationale capture for audit even when heavy tasks are deferred.

15. Failure/recovery drills
15a. Kill/restart tests mid‑Focus and mid‑fsync; verify zero‑loss recovery.  
15b. Snapshot corruption simulation → selective replay; document RTO/RPO.  
15c. Clock skew simulation; wall clock changes do not affect scheduling.

16. Performance & scaling
16a. Microbenchmarks for ring, scheduler, WAL/snapshot, and analysis functions.  
16b. Tune budgets and cadences; document target HW specs and throughput.  
16c. Add feature flags for `AEI3 on‑demand` and DEC pruning strategies.

17. Packaging & service
17a. Build `imd` (IM daemon) with config loader and health endpoint.  
17b. Provide systemd/service manifests; graceful shutdown (final snapshot).  
17c. Versioned release artifacts; changelog and upgrade notes.

18. Documentation
18a. API reference for messages; recovery and replay manual.  
18b. Design rationale, mode thresholds, and tuning guide.  
18c. Architecture diagrams and sequence examples.

19. Tag v0.1 and handoff
19a. Freeze schemas; publish artifacts and dashboards.  
19b. Draft integration plan with the rest of Project B (AEI/MM/DEC/TA).  
19c. Collect perf/bug feedback for v0.2 roadmap.

---

## 13. Full Project Structure (IM Module)

```
project_b/
└── im/
    ├── im_core/                         # Rust crate (controller, scheduler, persistence, contracts)
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── controller.rs            # ImController::run, tick, on_event, persist, health_checks, backpressure
    │       ├── analysis.rs              # update_scene, estimate_salience, predict, evaluate, select_action
    │       ├── model.rs                 # WorkingContext, Event, Scene, Hypothesis, Candidate, Goal, Uncertainty, Mode
    │       ├── scheduler.rs             # PriQueue (P0,P1,P2), EDF, budgets
    │       ├── storage/
    │       │   ├── mod.rs               # traits: WAL, Snapshot, Kv
    │       │   ├── wal.rs               # append, fsync_deadline, segment index
    │       │   ├── snapshot.rs          # write_snapshot, load_latest, verify_crc
    │       │   └── kv.rs                # blob put/get (mmap), compaction
    │       ├── interfaces/
    │       │   └── messages.rs          # Protobuf/FlatBuffers adapters, versioning
    │       ├── health.rs                # invariants and monitors
    │       ├── config.rs                # thresholds, cadences, paths
    │       ├── logging.rs               # structured logs + metrics
    │       └── ffi/
    │           └── cxxbridge.rs         # safe wrappers to im_native primitives
    ├── im_native/                       # C++ library (low‑latency primitives)
    │   ├── CMakeLists.txt
    │   ├── include/im_native/
    │   │   ├── fast_ring_buffer.hpp     # lock‑free MPMC ring
    │   │   ├── crc32c.hpp               # hardware CRC32C
    │   │   ├── event_codec.hpp          # codecs + PB/FB marshal
    │   │   ├── time_monotonic.hpp       # steady clock helpers
    │   │   ├── prio_queue.hpp           # EDF heap, 3 lanes
    │   │   ├── watchdog.hpp             # restart hooks
    │   │   └── metrics_exporter.hpp     # Prometheus exporter
    │   └── src/
    │       ├── fast_ring_buffer.cpp
    │       ├── crc32c.cpp
    │       ├── event_codec.cpp
    │       ├── time_monotonic.cpp
    │       ├── prio_queue.cpp
    │       ├── watchdog.cpp
    │       └── metrics_exporter.cpp
    ├── messages/                        # IDL and generated code
    │   ├── im.proto                     # or .fbs
    │   └── generated/                   # rust/ & cpp/ outputs
    ├── imd/                             # IM daemon
    │   ├── Cargo.toml
    │   └── src/main.rs                  # loads config, starts ImController, health endpoint
    ├── tools/
    │   ├── replay/                      # deterministic replay CLI
    │   └── bench/                       # microbenchmarks
    └── docs/
        ├── IM_architecture.md
        ├── IM_contracts.md
        └── IM_tuning.md
```

---

## 14. Directory/File/Function Key

- **im_core**: Rust crate with business logic and orchestration.  
  - `controller.rs` — scheduler, mode machine, IO; **main loop**.  
  - `analysis.rs` — scene fusion, salience, prediction/evaluation; **decision engine**.  
  - `model.rs` — strongly typed state; **safety & clarity**.  
  - `scheduler.rs` — priorities + EDF; **latency control**.  
  - `storage/*` — WAL/snapshot/KV; **durability & recovery**.  
  - `interfaces/messages.rs` — schema adapters; **compatibility**.  
  - `health.rs` — invariants; **fault detection**.  
  - `config.rs` — cadences/budgets; **tuning**.  
  - `logging.rs` — logs/metrics; **observability**.  
  - `ffi/cxxbridge.rs` — safe FFI; **Rust↔C++ bridge**.

- **im_native**: C++ primitives for ultra‑low‑latency operations.  
  - `fast_ring_buffer` — zero‑copy event queues.  
  - `crc32c` — integrity checks.  
  - `event_codec` — compact serialization.  
  - `time_monotonic` — stable timing & jitter tools.  
  - `prio_queue` — EDF heap implementation.  
  - `watchdog` — crash supervision.  
  - `metrics_exporter` — Prometheus output.

- **messages**: IDL definitions and generated bindings.  
- **imd**: runnable daemon hosting IM.  
- **tools/replay**: deterministic decision replay.  
- **tools/bench**: performance baselines.  
- **docs**: reference and tuning materials.

---

### Notes on Integration with Project A & Other Project B Modules

- AEI3 imports Project A artifacts (e.g., `transformation_matrices.npy`) via MM; IM reads them through MM‑ST pointers.  
- IM never blocks on heavy AEI/DEC work during React; it uses cached last‑good values.  
- All cross‑module traffic flows through schema‑versioned messages with explicit staleness rules.

---

**End of document.**
