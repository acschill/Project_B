#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Monotonic nanoseconds since boot.
pub type MonoNs = u64;
/// Milliseconds since UNIX epoch (wall clock; for metadata only).
pub type WallMs = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode { Idle, Observe, Focus, React }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub seq_no: u64,
    pub ts_mono_ns: MonoNs,
    pub schema_ver: u32,
    pub source_id: u32,
    /// Opaque payload (e.g., protobuf message bytes).
    pub payload: Vec<u8>,
    pub checksum: u32,
}

impl Envelope {
    #[must_use]
    pub fn new(seq_no: u64, ts_mono_ns: MonoNs, schema_ver: u32, source_id: u32, payload: Vec<u8>, checksum: u32) -> Self {
        Self { seq_no, ts_mono_ns, schema_ver, source_id, payload, checksum }
    }
}