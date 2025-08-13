//! Common type re-exports for ProjectB.
//! This keeps `Mode`, `Envelope`, and timestamp aliases in one place.

// Re-export from the shared base types crate (path: src/shared/libs/rust/base_types)
pub use project_b_base_types::{Mode, Envelope, MonoNs, WallMs};

// Convenience aliases (if you don't want to depend on the crate name elsewhere)
pub type ModeKind = Mode;
pub type EnvelopeMsg = Envelope;