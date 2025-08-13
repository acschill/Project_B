
use crate::model::{WorkingContext, Event, SceneDelta, Uncertainty, Hypothesis, Candidate};
pub fn update_scene(_ctx: &mut WorkingContext, _events: &[Event]) -> SceneDelta { SceneDelta::default() }
pub fn estimate_salience(_ctx: &WorkingContext) -> f32 { 0.0 }
pub fn predict(_ctx: &WorkingContext, _horizon_ms: u64) -> Vec<Hypothesis> { Vec::new() }
pub fn evaluate(_ctx: &WorkingContext, _ev: &Event) -> Vec<Candidate> { Vec::new() }
pub fn select_action(_cands: &[Candidate]) -> super::interfaces::Outbound { super::interfaces::Outbound::Nop }
pub fn compute_uncertainty(_ctx: &WorkingContext) -> Uncertainty { Uncertainty::default() }
