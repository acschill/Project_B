
use anyhow::Result;
use crate::model::WorkingContext;

#[derive(Default)]
pub struct WAL;
#[derive(Default)]
pub struct Snapshot;
#[derive(Default)]
pub struct Kv;
impl WAL { pub fn maybe_flush(&mut self) -> Result<()> { Ok(()) } }
impl Snapshot {
    pub fn write_snapshot(&mut self, _ctx: &WorkingContext) -> Result<()> { Ok(()) }
    pub fn load_latest(&self) -> Result<(WorkingContext, u64)> { Ok((WorkingContext::default(), 0)) }
}
