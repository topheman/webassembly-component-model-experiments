use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    pluginlab::run_async().await
}
