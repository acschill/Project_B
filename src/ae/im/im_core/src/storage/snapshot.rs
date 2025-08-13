
use anyhow::Result;
use crate::model::WorkingContext;
#[derive(Default)]
pub struct SnapWriter;
impl SnapWriter {
    pub fn write_snapshot(&mut self, _ctx: &WorkingContext) -> Result<u64> { Ok(0) }
    pub fn load_latest(&self) -> Result<(WorkingContext, u64)> { Ok((WorkingContext::default(), 0)) }
}
