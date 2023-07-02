mod env;
mod server;

use anyhow::{
    Context,
    Result as AnyResult,
};

use env::ENV;

#[tokio::main]
async fn main() -> AnyResult<()> {
    env_logger::builder()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    log::info!("Starting Server With Env {:#?}", *ENV);

    log::info!("Starting Server on Port {}", ENV.port0);
    server::start_server().await.context("Server Crashed")
}
