mod env;
mod server;

use anyhow::{
    Context,
    Result as AnyResult,
};

use log::info;

use env::ENV;

#[tokio::main]
async fn main() -> AnyResult<()> {
    env_logger::builder()
        .parse_default_env()
        .target(env_logger::Target::Stdout)
        .init();

    info!("Starting Server With Env {:#?}", *ENV);

    info!("Starting Server on Port {}", ENV.port0);
    server::start_server().await.context("Server Crashed")
}
