use anyhow::Result;
use mm_lt::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="mm_lt","daemon start");
    // TODO wire controller
    Ok(())
}