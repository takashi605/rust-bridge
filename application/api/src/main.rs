#![recursion_limit = "512"]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    api::run_server().await
}
