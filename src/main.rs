mod server;

use anyhow::Result as AnyResult;
use anyhow::Context;

#[tokio::main]
async fn main() -> AnyResult<()> {
    server::start_server().await.context("Server Crashed")
}