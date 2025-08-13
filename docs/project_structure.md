ProjectB/
├── Cargo.toml                 # Workspace root (lists all member crates)
├── CMakeLists.txt             # Top-level for native C++ libs
├── README.md
├── docs/
│   ├── Project_B_Technical_Guide.md
│   ├── IM/
│   │   └── ProjectB_IM_Comprehensive_Overview.md
│   ├── AE-Core/
│   │   └── ARCHITECTURE.md
│   ├── MM-ST/
│   ├── MM-LT/
│   ├── SA/
│   ├── TA/
│   ├── DEC/
│   └── EA/
└── src/
    ├── ae/
    │   ├── core/                          # AE-Core (hosts AEI1/2/3 logic)
    │   │   ├── Cargo.toml                 # crate: ae_core
    │   │   ├── src/
    │   │   │   ├── lib.rs
    │   │   │   ├── core.rs                # ingress/router/normalization
    │   │   │   ├── aei1.rs                # Initial Awareness
    │   │   │   ├── aei2.rs                # Intermediate Awareness
    │   │   │   └── aei3.rs                # Final Awareness (uses Project A artifacts via MM)
    │   │   ├── ae_native/                 # C++ low-latency primitives for AE
    │   │   │   ├── CMakeLists.txt
    │   │   │   ├── include/ae_native/*.hpp
    │   │   │   └── src/*.cpp
    │   │   ├── messages/
    │   │   │   ├── ae_core.proto
    │   │   │   └── generated/
    │   │   ├── daemon/
    │   │   │   ├── Cargo.toml
    │   │   │   └── src/main.rs
    │   │   ├── tools/
    │   │   │   ├── replay/
    │   │   │   └── bench/
    │   │   └── docs/
    │   │       ├── ARCHITECTURE.md
    │   │       └── README.md
    │   ├── im/                            # Internal Monologue (IM)
    │   │   ├── im_core/                   # Rust orchestrator for IM
    │   │   │   ├── Cargo.toml
    │   │   │   └── src/ (controller.rs, analysis.rs, model.rs, scheduler.rs, storage/*, interfaces/*, etc.)
    │   │   ├── im_native/                 # C++ primitives for IM
    │   │   │   ├── CMakeLists.txt
    │   │   │   ├── include/im_native/*.hpp
    │   │   │   └── src/*.cpp
    │   │   ├── messages/
    │   │   │   ├── im.proto
    │   │   │   └── generated/
    │   │   ├── imd/                       # IM daemon
    │   │   │   ├── Cargo.toml
    │   │   │   └── src/main.rs
    │   │   ├── tools/
    │   │   │   ├── replay/
    │   │   │   └── bench/
    │   │   └── docs/
    │   │       ├── IM_Comprehensive_Overview.md
    │   │       └── README.md
    │   ├── sa/                            # State Analyzer
    │   │   ├── sa/                        # Rust crate (name: sa)
    │   │   │   ├── Cargo.toml
    │   │   │   └── src/ (analysis.rs, logging.rs, etc.)
    │   │   ├── sa_native/                 # C++ primitives
    │   │   ├── messages/ (sa.proto, generated/)
    │   │   ├── daemon/ (Cargo.toml, src/main.rs)
    │   │   └── docs/
    │   ├── ta/                            # Task Planner
    │   │   ├── ta/                        # Rust crate (planner.rs, logging.rs)
    │   │   ├── ta_native/
    │   │   ├── messages/
    │   │   ├── daemon/
    │   │   └── docs/
    │   ├── dec/                           # Decision Engine
    │   │   ├── dec/                       # Rust crate (engine.rs, logging.rs)
    │   │   ├── dec_native/
    │   │   ├── messages/
    │   │   ├── daemon/
    │   │   └── docs/
    │   └── ea/                            # Event Audit / Experience Archive
    │       ├── ea/                        # Rust crate (archive.rs, logging.rs)
    │       ├── ea_native/
    │       ├── messages/
    │       ├── daemon/
    │       └── docs/
    ├── mm/
    │   ├── st/                            # Short-Term Memory
    │   │   ├── mm_st/                     # Rust crate
    │   │   ├── mm_native/                 # C++ primitives for ST
    │   │   ├── messages/
    │   │   ├── daemon/
    │   │   └── docs/
    │   └── lt/                            # Long-Term Memory
    │       ├── mm_lt/                     # Rust crate
    │       ├── mm_native/                 # C++ primitives for LT
    │       ├── messages/
    │       ├── daemon/
    │       └── docs/
    ├── shared/
    │   ├── contracts/                     # (Optional) Shared IDL if you centralize base types
    │   │   └── base.proto                 # Envelope + common fields (modules can import)
    │   └── libs/                          # (Optional) cross-cutting helpers
    └── tools/
        ├── replay/                        # Global replay utilities (module-specific ones live in each module)
        └── bench/                         # Global microbench harness

