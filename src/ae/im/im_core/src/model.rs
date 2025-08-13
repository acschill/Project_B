
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode { Idle, Observe, Focus, React }
impl Default for Mode { fn default() -> Self { Mode::Idle } }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub ts_mono_ns: u64,
    pub wall_ts_ms: u64,
    pub src: u32,
    pub schema_ver: u16,
    pub seq_no: u64,
    pub payload_ptr: u64,
    pub payload_crc: u32,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Scene { pub hash: u64 }
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SceneDelta { pub changed: bool }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis { pub horizon_ms: u64, pub score: f32 }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Candidate { pub score: f32 }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Goal { pub priority: i32, pub deadline_ms: u64 }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Uncertainty { pub sensor: f32, pub model: f32, pub memory: f32 }

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WorkingContext {
    pub mode: Mode,
    pub scene: Scene,
    pub goals: Vec<Goal>,
    pub salience: f32,
}
impl WorkingContext {
    pub fn apply_salience(&mut self, s: f32, th: &crate::config::ModeThresholds) {
        self.salience = s;
        let new_mode = if s >= th.react_up { Mode::React }
            else if s >= th.focus_up { Mode::Focus }
            else if s >= th.observe_up { Mode::Observe }
            else { Mode::Idle };
        self.mode = new_mode;
    }
}
