use anyhow::Result;
use spoti_dump::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
