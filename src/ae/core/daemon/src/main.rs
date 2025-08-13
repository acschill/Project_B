use anyhow::Result;
use ae_core::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="ae_core","daemon start");
    // TODO wire controller
    Ok(())
}