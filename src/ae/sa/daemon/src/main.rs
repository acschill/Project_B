use anyhow::Result;
use sa::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="sa","daemon start");
    // TODO wire controller
    Ok(())
}