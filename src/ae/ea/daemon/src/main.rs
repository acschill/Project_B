use anyhow::Result;
use ea::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="ea","daemon start");
    // TODO wire controller
    Ok(())
}