
use std::time::Duration;
use crate::model::Mode;
#[derive(Clone, Copy, Debug)]
pub struct ModeThresholds { pub observe_up: f32, pub focus_up: f32, pub react_up: f32 }
impl Default for ModeThresholds { fn default() -> Self { Self { observe_up: 0.25, focus_up: 0.65, react_up: 0.90 } } }
#[derive(Clone, Copy, Debug, Default)]
pub struct Cadence;
impl Cadence {
    pub fn sleep_for(&self, mode: Mode) -> Duration {
        match mode { Mode::Idle => Duration::from_millis(1000), Mode::Observe => Duration::from_millis(100),
                     Mode::Focus => Duration::from_millis(20), Mode::React => Duration::from_millis(5) }
    }
    pub fn should_snapshot(&self, mode: Mode) -> bool {
        match mode { Mode::Idle => false, Mode::Observe => true, Mode::Focus => true, Mode::React => false }
    }
}
#[derive(Clone, Copy, Debug)] pub struct Budgets; impl Default for Budgets { fn default() -> Self { Budgets } }
