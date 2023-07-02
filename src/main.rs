mod server;

use anyhow::{
    Context,
    Result as AnyResult,
};

#[tokio::main]
async fn main() -> AnyResult<()> {
    env_logger::builder()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    log::info!("Starting Server");
    server::start_server().await.context("Server Crashed")
}
