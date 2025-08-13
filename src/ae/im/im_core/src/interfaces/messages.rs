
use serde::{Serialize, Deserialize};
use crate::model::Event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Inbound {
    AEIUpdate(AEIUpdate),
    SAEvent(SAEvent),
    MMQueryResult(()),
    TAPlanUpdate(()),
    DECOutcome(()),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Outbound {
    IMDecision(IMDecision),
    IMRequest(()),
    IMModeChange(IMModeChange),
    IMSnapshotHint(()),
    Health(IMHealth),
    Nop,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AEIUpdate { pub event: Event }
impl AEIUpdate { pub fn into_event(self) -> Event { self.event } }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SAEvent {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IMDecision { pub rationale_ptr: u64 }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IMModeChange { pub to: crate::model::Mode }
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IMHealth { pub salience: f32, pub mode: crate::model::Mode }
impl From<&crate::model::WorkingContext> for IMHealth {
    fn from(ctx: &crate::model::WorkingContext) -> Self { Self { salience: ctx.salience, mode: ctx.mode } }
}
