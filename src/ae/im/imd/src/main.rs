
use anyhow::Result;
use im_core::{ImController, logging};
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    let (tx_out, mut rx_out) = unbounded_channel();
    let (_tx_in, rx_in) = unbounded_channel();
    let wal = im_core::storage::WAL::default();
    let snapshot = im_core::storage::Snapshot::default();
    let mut ctl = ImController::new(rx_in, tx_out, wal, snapshot);
    tokio::spawn(async move {
        while let Some(msg) = rx_out.recv().await {
            tracing::info!(target="imd", "OUT {:?}", msg);
        }
    });
    ctl.run().await?;
    Ok(())
}
