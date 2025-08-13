
use anyhow::Result;
pub struct WalRec;
#[derive(Default)]
pub struct WalWriter;
impl WalWriter {
    pub fn append(&mut self, _batch: &[WalRec]) -> Result<u64> { Ok(0) }
    pub fn fsync_deadline(&mut self) -> Result<()> { Ok(()) }
}
