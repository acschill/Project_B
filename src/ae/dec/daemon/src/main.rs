use anyhow::Result;
use dec::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="dec","daemon start");
    // TODO wire controller
    Ok(())
}