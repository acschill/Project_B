use anyhow::Result;
use mm_st::logging;
#[tokio::main(flavor="multi_thread")]
async fn main() -> Result<()> {
    logging::init();
    tracing::info!(target="mm_st","daemon start");
    // TODO wire controller
    Ok(())
}