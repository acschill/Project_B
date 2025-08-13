use anyhow::Result;
use ta::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="ta","daemon start");
    // TODO wire controller
    Ok(())
}